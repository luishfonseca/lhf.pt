{
  description = "lhf.pt website";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-22.11";
    unstable.url = "nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { ... } @ inputs: inputs.utils.lib.eachDefaultSystem (system:
    let
      pkgs = import inputs.nixpkgs { inherit system; };
      pkgs-unstable = import inputs.unstable { inherit system; };

      inherit (pkgs-unstable) prisma-engines;
    in
    {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [ nodejs nodePackages.npm openssl pscale mysql ];
        PRISMA_MIGRATION_ENGINE_BINARY="${prisma-engines}/bin/migration-engine";
        PRISMA_QUERY_ENGINE_BINARY="${prisma-engines}/bin/query-engine";
        PRISMA_QUERY_ENGINE_LIBRARY="${prisma-engines}/lib/libquery_engine.node";
        PRISMA_INTROSPECTION_ENGINE_BINARY="${prisma-engines}/bin/introspection-engine";
        PRISMA_FMT_BINARY="${prisma-engines}/bin/prisma-fmt";
      };
    });
}
