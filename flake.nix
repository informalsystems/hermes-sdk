{
  description = "Nix development dependencies for ibc-rs";

  inputs = {
    nixpkgs.url = github:nixos/nixpkgs/nixpkgs-unstable;
    rust-overlay.url = github:oxalica/rust-overlay;
    flake-utils.url = github:numtide/flake-utils;

    cosmos-nix.url = github:informalsystems/cosmos.nix/soareas/gaia18;
    cosmos-nix-wasm.url = github:informalsystems/cosmos.nix/jonathan/ibc-go-wasm;

    ibc-rs-src = {
      url = github:cosmos/ibc-rs;
      flake = false;
    };

    gaia-src = {
        flake = false;
        url = github:cosmos/gaia/v18.1.0;
    };

    celestia-app-src = {
        flake = false;
        url = github:celestiaorg/celestia-app/v2.0.0;
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
      cosmos-nix-wasm = inputs.cosmos-nix-wasm.packages.${system};
      sovereign-nix = inputs.sovereign-nix.packages.${system};
      sovereign-ibc-nix = inputs.sovereign-ibc-nix.packages.${system};

      tendermint-wasm-client = import ./nix/tendermint-wasm-client {
        inherit nixpkgs;
        inherit (inputs) ibc-rs-src;
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
          gaia18
        ;

        inherit
          (cosmos-nix-wasm)
          ibc-go-v7-wasm-simapp
        ;
      };
    });
}
