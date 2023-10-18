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
        ];

        buildPhase = ''
            cargo-risczero cargo-risczero install-toolchain
        '';
    };
in
risc0-toolchain