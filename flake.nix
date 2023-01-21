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
        buildInputs = with pkgs; [ nodejs nodePackages.npm ];
      };

      defaultPackage = pkgs.buildNpmPackage {
        pname = "lhf-pt-website";
        version = "0.0.1";

        src = ./.;
        npmDepsHash = "sha256-5Py0Qaam168qsqwFe/zVFXJxxHXJ33gTYzeRT0x5WBI=";

        installPhase = ''
          mkdir -p $out
          npx ncc build build/ -m -o $out
          cp -r build/client $out

          # Make entrypoint executable
          sed '1 i\#!${pkgs.nodejs}/bin/node' -i $out/index.js
          chmod +x $out/index.js
        '';
      };
    });
}
