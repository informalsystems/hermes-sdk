{
  nixpkgs
, cosmwasm-ibc-src
}:
let
  rust-bin = nixpkgs.rust-bin.fromRustupToolchainFile ./wasm-rust-toolchain.toml;

  tendermint-wasm-client = nixpkgs.rustPlatform.buildRustPackage {
    name = "ibc-client-tendermint-cw";
    src = cosmwasm-ibc-src;

    cargoLock = {
      lockFile = cosmwasm-ibc-src + "/Cargo.lock";
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
        cp -r target/wasm32-unknown-unknown/release/ibc_client_tendermint_cw.wasm $out/
    '';
  };
in
tendermint-wasm-client