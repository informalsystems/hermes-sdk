{
  description = "Nix development dependencies for ibc-rs";

  inputs = {
    nixpkgs.url = github:nixos/nixpkgs/nixpkgs-unstable;
    flake-utils.url = github:numtide/flake-utils;
    cosmos-nix.url = github:informalsystems/cosmos.nix;
    rust-overlay.url = github:oxalica/rust-overlay;

    risc0-src = {
      url = github:informalsystems/risc0/v0.18.0-with-lock;
      flake = false;
    };

    risc0-rust-src = {
      url = github:risc0/rust/risc0;
      flake = false;
    };

    test-rollup-src = {
      url = github:informalsystems/sovereign-sdk/farhad/demo-rollup-with-ibc;
      flake = false;
    };
  };

  outputs = inputs: let
    utils = inputs.flake-utils.lib;
  in
    utils.eachSystem
    [
      "aarch64-linux"
      "aarch64-darwin"
      "x86_64-darwin"
      "x86_64-linux"
    ]
    (system: let
      nixpkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [
          inputs.rust-overlay.overlays.default
        ];
      };

      cosmos-nix = inputs.cosmos-nix.packages.${system};


      cargo-risczero = import ./nix/risc0/cargo-risczero.nix {
        inherit nixpkgs;
        src = inputs.risc0-src;
      };

      risc0-toolchain = import ./nix/risc0/toolchain.nix {
        inherit nixpkgs cargo-risczero;
        src = inputs.risc0-rust-src;
      };

      test-rollup = import ./nix/test-rollup {
        inherit nixpkgs;
        src = inputs.test-rollup-src;
      };

    in {
      packages = {
        inherit
          (cosmos-nix)
          gaia11
          ibc-go-v7-simapp
          ;

        inherit cargo-risczero risc0-toolchain test-rollup;
      };
    });
}
