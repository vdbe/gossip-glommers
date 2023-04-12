{
  description = "Description for the project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    my-packages = {
      url = "github:vdbe/nix-configuration/dev?dir=packages";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    nci = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        parts.follows = "parts";
      };
    };
  };

  outputs = inputs@{ parts, nixpkgs, my-packages, nci, ... }:
    parts.lib.mkFlake { inherit inputs; }
      {
        imports = [
          nci.flakeModule
        ];

        debug = true;
        systems = [ "x86_64-linux" ]; # "aarch64-darwin" ];
        perSystem = { config, self', inputs', pkgs, system, ... }:
          let
            crateName = "gossip-glomers";
            crateOutputs = config.nci.outputs.${crateName};

            mypkgs = my-packages.packages."${system}";
          in
          {
            nci = {
              projects.${crateName}.relPath = "";
              crates.${crateName} = {
                export = true;
              };
            };

            packages = {
              default = self'.packages.gossip-glomers;
              gossip-glomers = crateOutputs.packages.release;

              inherit (mypkgs) maelstrom;
            };

            devShells.default = config.nci.outputs.${crateName}.devShell.overrideAttrs (old: {
              packages = (old.packages or [ ]) ++ [ self'.packages.maelstrom ];
            });
          };
        flake = { };
      };
}
