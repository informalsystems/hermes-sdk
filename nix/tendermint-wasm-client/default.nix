{
  nixpkgs
, ibc-rs-src
}:
let
  rust-bin = nixpkgs.rust-bin.fromRustupToolchainFile ./wasm-rust-toolchain.toml;

  ibc-rs-src-with-lockfile = nixpkgs.stdenv.mkDerivation {
    name = "ibc-rs-src";
    dontUnpack = true;
    dontBuild = true;

    installPhase = ''
        mkdir -p $out
        cp ${./Cargo.lock} $out/Cargo.lock

        cp -r ${ibc-rs-src}/. $out/
        ls -la $out
    '';
  };

  tendermint-wasm-client = nixpkgs.rustPlatform.buildRustPackage {
    name = "ibc-client-tendermint-cw";
    src = ibc-rs-src;

    cargoLock = {
      lockFile = ./Cargo.lock;
    };

    nativeBuildInputs = [
      rust-bin
    ];

    doCheck = false;

    patchPhase = ''
        cp ${./Cargo.lock} ./Cargo.lock
    '';

    buildPhase = ''
      RUSTFLAGS='-C link-arg=-s' cargo build -p ibc-client-tendermint-cw --target wasm32-unknown-unknown --release --lib --locked
    '';

    installPhase = ''
        mkdir -p $out
        cp -r target/wasm32-unknown-unknown/release/ibc_client_tendermint_cw.wasm $out/
    '';
  };
in
tendermint-wasm-client