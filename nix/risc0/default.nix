{
    src,
    nixpkgs,
}:
let
    risc0 = nixpkgs.rustPlatform.buildRustPackage {
        inherit src;

        cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
                "ark-secret-scalar-0.0.2" = "sha256-Nbf77KSsAjDKiFIP5kgzl23fRB+68x1EirNuZlS7jeM=";
                "common-0.1.0" = "sha256-3OKBPpk0exdlV0N9rJRVIncSrkwdI8bkYL2QNsJl+sY=";
                "fflonk-0.1.0" = "sha256-MNvlePHQdY8DiOq6w7Hc1pgn7G58GDTeghCKHJdUy7E=";
                "frame-support-4.0.0-dev" = "sha256-RETvqOatRGFy59IZMDbX4uqiINVOylYjZtQCffFkET4=";
            };
        };

        postPatch = ''
            ln -s ${./Cargo.lock} Cargo.lock
        '';

        nativeBuildInputs = [
            nixpkgs.rust-bin.stable.latest.default
        ];

        pname = "risc0";
        version = "v0.18.0";
        doCheck = false;
        cargoCheckCommand = "true";
    };
in
risc0