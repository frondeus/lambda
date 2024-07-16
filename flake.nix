{
  description = "Lambda lang";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-utils.url = "github:numtide/flake-utils";
    # Building rust
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };
  };

  outputs = inputs@{ flake-parts, ... }: flake-parts.lib.mkFlake { inherit inputs; } {
    systems = [ "x86_64-linux" "aarch64-darwin" ];
    perSystem = { config, pkgs, system, self', ... }: let 
      inherit (pkgs) lib;
      craneLib = (inputs.crane.mkLib pkgs).overrideToolchain
        # (inputs.fenix.packages.${system}.fromManifestFile ./rust-toolchain.toml).minimalToolchain;
        inputs.fenix.packages.${system}.stable.toolchain ;
      src = craneLib.cleanCargoSource (craneLib.path ./.);
      commonArgs = {
        inherit src;
        inherit (craneLib.crateNameFromCargoToml { cargoToml = ./Cargo.toml; }) pname version;
        strictDeps = true;
        buildInputs = [
        ] ++ lib.optionals pkgs.stdenv.isDarwin [ 
          pkgs.darwin.apple_sdk.frameworks.SystemConfiguration 
          pkgs.libiconv
        ];
      };
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      individualCrateArgs = commonArgs // {
        inherit cargoArtifacts;
        doCheck = false;
      };
      fileSetForCrate = crate: lib.fileset.toSource {
        root = ./.;
        fileset = lib.fileset.unions [
          ./Cargo.toml
          ./Cargo.lock
          ./tree-sitter-lambda
          ./.
        ];
      };
      lambda = craneLib.buildPackage (individualCrateArgs // {
        pname = "lambda";
        cargoExtraArgs = "-p lambda";
        src = fileSetForCrate ./.;
      });
    in 
    {
      # Building packages
      packages."lambda" = lambda;
      apps."lambda" = inputs.flake-utils.lib.mkApp {
        name = "lambda";
        drv = lambda;
      };
      checks = { 
        inherit lambda;
      };
      devShells.default = craneLib.devShell {
        checks = self'.checks;
        packages = with pkgs; [  
          cargo-watch
          # rust-analyzer-nightly
          tree-sitter
          colordiff
        ];
      };
    };
  };
}
