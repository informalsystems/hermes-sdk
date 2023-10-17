{
    src,
    nixpkgs,
}:
let
    rust-bin = nixpkgs.rust-bin.stable.latest.default;
    # rust-bin = nixpkgs.rust-bin.fromRustupToolchainFile ( src + "/rust-toolchain.toml");

    # rust-bin = nixpkgs.rust-bin.stable.latest.default.override {
    #     extensions = [ "rust-src" ];
    #     targets = [ "wasm32-unknown-unknown" ];
    # };

    risc0 = nixpkgs.rustPlatform.buildRustPackage {
        inherit src;

        buildAndTestSubdir = "risc0/cargo-risczero";

        # cargoLock = {
        #     lockFileContents = builtins.readFile ./Cargo.lock;
        #     outputHashes = {
        #         "ark-secret-scalar-0.0.2" = "sha256-Nbf77KSsAjDKiFIP5kgzl23fRB+68x1EirNuZlS7jeM=";
        #         "common-0.1.0" = "sha256-3OKBPpk0exdlV0N9rJRVIncSrkwdI8bkYL2QNsJl+sY=";
        #         "fflonk-0.1.0" = "sha256-MNvlePHQdY8DiOq6w7Hc1pgn7G58GDTeghCKHJdUy7E=";
        #         "frame-support-4.0.0-dev" = "sha256-RETvqOatRGFy59IZMDbX4uqiINVOylYjZtQCffFkET4=";
        #     };
        # };

        # postPatch = ''
        #     ln -s ${./Cargo.lock} Cargo.lock
        # '';

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

        # cargoBuildHook = false;
        # cargoSetupPostPatchHook = false;

        # buildPhase = ''
        #     cargo build -p cargo-risczero
        # '';
    };
in
risc0