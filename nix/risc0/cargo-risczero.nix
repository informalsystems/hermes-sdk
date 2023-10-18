{
    src,
    nixpkgs,
}:
let
    rust-bin = nixpkgs.rust-bin.stable.latest.default;

    cargo-risczero = nixpkgs.rustPlatform.buildRustPackage {
        inherit src;

        buildAndTestSubdir = "risc0/cargo-risczero";

        cargoSha256 = "sha256-ETTJ7DmpxxRc55CeEpuqVd0gu9Hf9vzXZC9Hn0g79YE=";

        nativeBuildInputs = [
            rust-bin
            nixpkgs.pkg-config
            nixpkgs.gcc
            nixpkgs.rustup
        ];

        PKG_CONFIG_PATH = "${nixpkgs.openssl.dev}/lib/pkgconfig";

        name = "risc0";
        version = "v0.18.0";
        doCheck = false;
        cargoCheckCommand = "true";
    };
in
cargo-risczero