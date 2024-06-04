{
  description = "Nix development dependencies for ibc-rs";

  inputs = {
    nixpkgs.url = github:nixos/nixpkgs/nixpkgs-unstable;
    flake-utils.url = github:numtide/flake-utils;
    cosmos-nix.url = github:informalsystems/cosmos.nix;
    cosmos-nix-wasm.url = github:informalsystems/cosmos.nix/jonathan/ibc-go-wasm;
    sovereign-nix.url = github:informalsystems/sov-rollup-starter/ibc-rollup;
    sovereign-ibc-nix.url = github:informalsystems/sovereign-ibc;

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
      };

      cosmos-nix = inputs.cosmos-nix.packages.${system};
      cosmos-nix-wasm = inputs.cosmos-nix-wasm.packages.${system};
      sovereign-nix = inputs.sovereign-nix.packages.${system};
      sovereign-ibc-nix = inputs.sovereign-ibc-nix.packages.${system};

      rust-bin = nixpkgs.rust-bin.fromRustupToolchainFile ./nix/wasm-rust-toolchain.toml;

      tendermint-wasm-client = nixpkgs.rustPlatform.buildRustPackage {
        name = "ibc-client-tendermint-cw";
        src = inputs.ibc-rs-src;

        cargoLock = {
          lockFile = ./nix/Cargo.lock;
        };

        nativeBuildInputs = [
          rust-bin
        ];

        doCheck = false;

        buildPhase = ''
            RUSTFLAGS='-C link-arg=-s' cargo build -p ibc-client-tendermint-cw --target wasm32-unknown-unknown --release --lib --locked
        '';

        installPhase = ''
            mkdir -p $out
            cp -r target/wasm32-unknown-unknown/release $out/
        '';
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
          (sovereign-nix)
          gaia
          celestia-app
          celestia-node
        ;

        inherit
          (sovereign-ibc-nix)
          sov-celestia-cw
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
