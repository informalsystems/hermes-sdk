{
  description = "Nix development dependencies for ibc-rs";

  inputs = {
    nixpkgs.url = github:nixos/nixpkgs/nixpkgs-unstable;
    rust-overlay.url = github:oxalica/rust-overlay;
    flake-utils.url = github:numtide/flake-utils;

    cosmos-nix.url = github:informalsystems/cosmos.nix;

    cosmwasm-ibc-src = {
      url = github:informalsystems/cosmwasm-ibc;
      flake = false;
    };

    celestia-app-src = {
        flake = false;
        url = github:celestiaorg/celestia-app/v1.13.0;
    };

    celestia-node-src = {
        flake = false;
        url = github:celestiaorg/celestia-node/v0.14.1;
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

      tendermint-wasm-client = import ./nix/tendermint-wasm-client {
        inherit nixpkgs;
        inherit (inputs) cosmwasm-ibc-src;
      };

      celestia-app = import ./nix/celestia-app.nix {
        inherit nixpkgs;
        inherit (inputs) celestia-app-src;
      };

      celestia-node = import ./nix/celestia-node.nix {
        inherit nixpkgs;
        inherit (inputs) celestia-node-src;
      };
    in {
      packages = {
        inherit tendermint-wasm-client celestia-app celestia-node;

        gaia = cosmos-nix.gaia18;

        inherit
          (nixpkgs)
          protobuf
          cargo-nextest
        ;

        inherit
          (cosmos-nix)
          ibc-go-v7-simapp
          ibc-go-v8-simapp
          ibc-go-v7-wasm-simapp
          ibc-go-v8-wasm-simapp
          gaia18
          gaia14
          osmosis
        ;
      };
    });
}
