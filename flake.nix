{
  description = "lhf.pt website";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.05";
    utils.url = "github:numtide/flake-utils";

    picocss = {
      url = "github:picocss/pico/v2.0.6";
      flake = false;
    };
  };

  outputs = { ... } @ inputs: inputs.utils.lib.eachDefaultSystem (system:
    let
      pkgs = import inputs.nixpkgs { inherit system; };

      vercelCfg = builtins.toJSON {
        version = 3;
        routes = [
          { handle = "filesystem"; }
          { src = "/(.*)"; status = 404; dest = "/404.html"; }
        ];
      };

      web = { prod }: pkgs.stdenv.mkDerivation {
        name = "lhf.pt-web";

        src = ./web;

        patchPhase = ''
          cp -r ${inputs.picocss}/scss sass/_pico
        '';

        installPhase = ''
          mkdir $out

          cat > $out/config.json << EOF
          ${vercelCfg}
          EOF

          ${pkgs.zola}/bin/zola build -o $out/static ${if prod then "" else "-u=https://preview.lhf.pt"}
        '';
      };

      deploy = { prod, ci ? true }: pkgs.writeScript "deploy" (''
        ${pkgs.nodePackages.vercel}/bin/vercel pull --yes --environment ${if prod then "production" else "development"} ${if ci then "--token=$VERCEL_TOKEN" else ""}

        rm -rf .vercel/output
        mkdir -p .vercel/output
        cp -r ${web { inherit prod; }}/* .vercel/output
        chmod -R +w .vercel/output

        url=$(${pkgs.nodePackages.vercel}/bin/vercel deploy --prebuilt ${if prod then "--prod" else ""} ${if ci then "--token=$VERCEL_TOKEN" else ""})
      '' + (if prod then "" else ''
        ${pkgs.nodePackages.vercel}/bin/vercel alias --yes $url preview.lhf.pt ${if ci then "--token=$VERCEL_TOKEN" else ""}
      ''));

      mkApp = run: { type = "app"; program = "${run}"; };
    in
    {
      devShell = pkgs.mkShell {
        shellHook = ''
          unlink web/sass/_pico
          ln -s ${inputs.picocss}/scss web/sass/_pico
        '';

        buildInputs = with pkgs; [ zola nodePackages.vercel ];
      };

      defaultPackage = web { prod = true; };

      apps.deploy-prod = mkApp (deploy { prod = true; });
      apps.deploy-dev = mkApp (deploy { prod = false; });
      apps.deploy-not-ci = mkApp (deploy { prod = false; ci = false; });
    });
}
