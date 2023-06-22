{
  description = "lhf.pt website";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-23.05";

    utils.url = "github:numtide/flake-utils";

    nix-filter.url = "github:numtide/nix-filter";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "utils";
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "utils";
      inputs.rust-overlay.follows = "rust-overlay";
    };
  };

  outputs = { ... } @ inputs: inputs.utils.lib.eachDefaultSystem (system:
    let
      pkgs = import inputs.nixpkgs { inherit system; overlays = [ inputs.rust-overlay.overlays.default ]; };

      rust = pkgs.rust-bin.stable.latest.default.override {
        extensions = [ "rust-src" "rust-analyzer" ];
        targets = [ "wasm32-unknown-unknown" ];
      };

      crane = (inputs.crane.mkLib pkgs).overrideToolchain rust;

      bindgen = crane.buildPackage {
        src = pkgs.fetchCrate {
          pname = "wasm-bindgen-cli";
          version = "0.2.87";
          sha256 = "sha256-0u9bl+FkXEK2b54n7/l9JOCtKo+pb42GF9E1EnAUQa0=";
        };

        doCheck = false;

        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = [ pkgs.openssl ];
      };

      commonArgs = {
        doCheck = false;
        cargoExtraArgs = "--target wasm32-unknown-unknown";
      };

      deps = { prod }: crane.buildDepsOnly (commonArgs // {
        CARGO_PROFILE = if prod then "release" else "dev";
        src = inputs.nix-filter.lib.filter {
          root = ./.;
          include = [ "Cargo.toml" "Cargo.lock" ];
        };
      });

      buildWasm = { prod }: crane.buildPackage (commonArgs // {
        CARGO_PROFILE = if prod then "release" else "dev";
        cargoArtifacts = deps { inherit prod; };
        src = inputs.nix-filter.lib.filter {
          root = ./.;
          include = [ "Cargo.toml" "Cargo.lock" "src" ];
        };
      });

      vercelCfg = builtins.toJSON {
        version = 3;
        routes = [
          { src = "/[^.]+"; dest = "/"; }
        ];
      };

      dist = { prod }:
        let wasm = buildWasm { inherit prod; }; in
        pkgs.stdenv.mkDerivation {
          pname = "${wasm.pname}-dist";
          version = wasm.version;

          src = inputs.nix-filter.lib.filter {
            root = ./.;
            include = [
              "index.html"
              "LICENSE"
              "README.md"
              "content"
            ];
          };

          installPhase = ''
            mkdir -p $out/static

            cat > $out/config.json << EOF
            ${vercelCfg}
            EOF

            cp README.md $out/
            cp LICENSE $out/
            cp index.html $out/static
            cp -r content $out/static
            ${pkgs.tree}/bin/tree $out/static/content/posts -J > $out/static/content/posts.json

            ${bindgen}/bin/wasm-bindgen --target web --no-typescript --out-dir $out/static ${wasm}/lib/lhf_pt.wasm

          '' + (if prod then ''
            ${pkgs.binaryen}/bin/wasm-opt -Oz $out/static/lhf_pt_bg.wasm -o $out/static/lhf_pt_bg.wasm.opt
            mv $out/static/lhf_pt_bg.wasm.opt $out/static/lhf_pt_bg.wasm
          '' else "");
        };

      deploy = { prod, ci ? true }: pkgs.writeScript "deploy" ''
        ${pkgs.nodePackages.vercel}/bin/vercel pull --yes --environment ${if prod then "production" else "development"} ${if ci then "--token=$VERCEL_TOKEN" else ""}

        rm -rf .vercel/output
        mkdir -p .vercel/output
        cp -r ${dist { prod = true; /* Always compile release for Vercel */ }}/* .vercel/output
        chmod -R +w .vercel/output

        ${pkgs.nodePackages.vercel}/bin/vercel deploy --prebuilt ${if prod then "--prod" else ""} ${if ci then "--token=$VERCEL_TOKEN" else ""}
      '';

      serve = { prod, port }: pkgs.writeScript "serve" ''
        ${pkgs.nodePackages.serve}/bin/serve ${dist { inherit prod; }}/static -l ${toString port} -s
      '';

      mkApp = run: {
        type = "app";
        program = "${run}";
      };

    in
    rec {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [ rust twiggy xsel ];
      };

      packages.lhf-pt-wasm-prod = buildWasm { prod = true; };
      packages.lhf-pt-wasm-dev = buildWasm { prod = false; };

      packages.lhf-pt-dist-prod = dist { prod = true; };
      packages.lhf-pt-dist-dev = dist { prod = false; };

      apps.deploy-prod = mkApp (deploy { prod = true; });
      apps.deploy-dev = mkApp (deploy { prod = true; });
      apps.deploy-not-ci = mkApp (deploy { prod = false; ci = false; });

      apps.serve-prod = mkApp (serve { prod = true; port = 5001; });
      apps.serve-dev = mkApp (serve { prod = false; port = 5002; });
    });
}
