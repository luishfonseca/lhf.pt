{
  description = "lhf.pt website";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-22.11";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { ... } @ inputs: inputs.utils.lib.eachDefaultSystem (system:
    let
      pkgs = import inputs.nixpkgs { inherit system; };
    in
    {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [ nodejs yarn ];
      };
      defaultPackage = pkgs.mkYarnPackage rec {
        pname = "lhf-pt-website";
        version = "0.0.1";

        src = ./.;
        packageJson = "${src}/package.json";
        yarnLock = "${src}/yarn.lock";

        buildPhase = ''
          yarn --offline build
        '';

        installPhase = ''
          mkdir -p $out
          echo "{ \"type\": \"module\" }" > $out/package.json
          cp -rt $out deps/${pname}/build/* node_modules
        '';

        postFixup = ''
          sed '1 i\#!${pkgs.nodejs}/bin/node' -i $out/index.js
          chmod +x $out/index.js
        '';

        distPhase = "true";
      };
    });
}
