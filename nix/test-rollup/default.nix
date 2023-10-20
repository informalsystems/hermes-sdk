{
    src,
    nixpkgs,
}:
let
    rust-bin = nixpkgs.rust-bin.stable.latest.default;

    test-rollup = nixpkgs.rustPlatform.buildRustPackage {
        inherit src;

        buildAndTestSubdir = "examples/demo-rollup";

        cargoSha256 = "sha256-ETTJ7DmpxxRc55CeEpuqVd0gussf9vzXZC9Hn0g79YE=";

        nativeBuildInputs = [
            rust-bin
            nixpkgs.pkg-config
            nixpkgs.gcc
            nixpkgs.rustup
        ];

        PKG_CONFIG_PATH = "${nixpkgs.openssl.dev}/lib/pkgconfig";

        name = "test-rollup";
        version = "v0.2.0";
        doCheck = false;
        cargoCheckCommand = "true";
    };
in
test-rollup