{
  description = "Description for the project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    my-packages = {
      url = "github:vdbe/nix-configuration/dev?dir=packages";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    devshell.url = "github:numtide/devshell";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs@{ flake-parts, nixpkgs, my-packages, rust-overlay, ... }:
    let
      pname = "gossip-glomers";
      version = "0.1.0";

    in
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devshell.flakeModule
      ];

      systems = [ "x86_64-linux" "aarch64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, ... }:
        let
          mypkgs = my-packages.packages."${system}";

          rustVersion = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);
          rustPlatform = pkgs.makeRustPlatform {
            cargo = rustVersion;
            rustc = rustVersion;
          };

          myRustBuild = rustPlatform.buildRustPackage {
            inherit pname version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };
        in
        {
          _module.args.pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
            config.allowUnfree = true;
          };

          packages = rec {
            default = gossip-glomers;
            inherit (mypkgs) maelstrom;
            gossip-glomers = myRustBuild;
          };

          devshells.default = {
            packages = [
              #mypkgs.maelstrom
              self'.packages.maelstrom
            ];
          };
        };
      flake = { };
    };
}
