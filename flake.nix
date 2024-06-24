{
  description = "Nix development dependencies for ibc-rs";

  inputs = {
    nixpkgs.url = github:nixos/nixpkgs/nixpkgs-unstable;
    flake-utils.url = github:numtide/flake-utils;
    cosmos-nix.url = github:informalsystems/cosmos.nix;
    cosmos-nix-wasm.url = github:informalsystems/cosmos.nix/jonathan/ibc-go-wasm;
    sovereign-nix.url = github:informalsystems/sov-rollup-starter/ibc-rollup;
    sovereign-ibc-nix.url = github:informalsystems/sovereign-ibc;
    rust-overlay.url = github:oxalica/rust-overlay;

    ibc-rs-src = {
      url = github:cosmos/ibc-rs;
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
      cosmos-nix-wasm = inputs.cosmos-nix-wasm.packages.${system};
      sovereign-nix = inputs.sovereign-nix.packages.${system};
      sovereign-ibc-nix = inputs.sovereign-ibc-nix.packages.${system};

      tendermint-wasm-client = import ./nix/tendermint-wasm-client {
        inherit nixpkgs;
        inherit (inputs) ibc-rs-src;
      };

    in {
      packages = {
        inherit tendermint-wasm-client;

        inherit
          (cosmos-nix)
          ibc-go-v7-simapp
          ibc-go-v8-simapp
        ;

        inherit
          (cosmos-nix-wasm)
          ibc-go-v7-wasm-simapp
        ;

        inherit
          (nixpkgs)
          protobuf
          cargo-nextest
        ;

        sovereign-rollup = sovereign-nix.rollup;
      };
    });
}
