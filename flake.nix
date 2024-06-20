{
  description = "lhf.pt website";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.05";
    utils.url = "github:numtide/flake-utils";

    picocss = {
      url = "github:picocss/pico/v2.0.6";
      flake = false;
    };

    fontawesome = {
      url = "github:FortAwesome/Font-Awesome/6.5.2";
      flake = false;
    };
  };

  outputs = {...} @ inputs:
    inputs.utils.lib.eachDefaultSystem (system: let
      pkgs = import inputs.nixpkgs {inherit system;};

      vercelCfg = builtins.toJSON {
        version = 3;
        routes = [
          {handle = "filesystem";}
          {
            src = "/(.*)";
            status = 404;
            dest = "/404.html";
          }
        ];
      };

      web = {prod}:
        pkgs.stdenv.mkDerivation {
          name = "lhf.pt-web";
          src = ./web;
          patchPhase = ''
            cp -r ${inputs.picocss}/scss sass/_pico

            cp -r ${inputs.fontawesome}/scss sass/_fontawesome
            cp -r ${inputs.fontawesome}/webfonts static/webfonts
          '';

          installPhase = ''
            mkdir $out

            cat > $out/config.json << EOF
            ${vercelCfg}
            EOF

            ${pkgs.zola}/bin/zola build -o $out/static ${
              if prod
              then ""
              else "-u=https://preview.lhf.pt"
            }
          '';
        };

      deploy = {
        prod,
        ci ? true,
      }: let
        app = pkgs.writeScriptBin "deploy" (''
            #!${pkgs.stdenv.shell}

            ${pkgs.nodePackages.vercel}/bin/vercel pull --yes --environment ${
              if prod
              then "production"
              else "development"
            } ${pkgs.lib.optionalString ci "--token=$VERCEL_TOKEN"}

            rm -rf .vercel/output
            mkdir -p .vercel/output
            cp -r ${web {inherit prod;}}/* .vercel/output
            chmod -R +w .vercel/output

            url=$(${pkgs.nodePackages.vercel}/bin/vercel deploy --prebuilt ${
              if prod
              then "--prod"
              else ""
            } ${pkgs.lib.optionalString ci "--token=$VERCEL_TOKEN"})
          ''
          + (
            pkgs.lib.optionalString (! prod) ''
              ${pkgs.nodePackages.vercel}/bin/vercel alias --yes $url preview.lhf.pt  ${pkgs.lib.optionalString ci "--token=$VERCEL_TOKEN"}
            ''
          ));
      in {
        type = "app";
        program = "${app}/bin/deploy";
      };
    in {
      devShell = pkgs.mkShell {
        shellHook = ''
          unlink web/sass/_pico
          ln -s ${inputs.picocss}/scss web/sass/_pico

          unlink web/sass/_fontawesome
          ln -s ${inputs.fontawesome}/scss web/sass/_fontawesome

          unlink web/static/webfonts
          ln -s ${inputs.fontawesome}/webfonts web/static/webfonts
        '';

        buildInputs = with pkgs; [zola nodePackages.vercel];
      };

      defaultPackage = web {prod = true;};

      apps.deploy-prod = deploy {prod = true;};
      apps.deploy-dev = deploy {prod = false;};
      apps.deploy-not-ci = deploy {
        prod = false;
        ci = false;
      };

      formatter = pkgs.alejandra;
    });
}
