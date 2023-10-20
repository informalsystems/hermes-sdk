{
    src,
    nixpkgs,
    cargo-risczero
}:
let
    rust-bin = nixpkgs.rust-bin.stable.latest.default;

    risc0-toolchain = nixpkgs.stdenv.mkDerivation {
        inherit src;

        name = "risc0-toolchain";

        buildInputs = [
            cargo-risczero
            rust-bin
            nixpkgs.git
            nixpkgs.python3
        ];

        CI = "1";

        configurePhase = "true";

        buildPhase = ''
            export RISC0_BUILD_DIR=$pwd
            mkdir -p $out/rust
            mv * $out/rust/
            mv $out/rust ./
            ls -la
            cargo-risczero risczero build-toolchain
        '';
    };
in
risc0-toolchain