{
  description = "Nix development dependencies for ibc-rs";

  inputs = {
    nixpkgs.url = github:nixos/nixpkgs/nixpkgs-unstable;
    flake-utils.url = github:numtide/flake-utils;
    cosmos-nix.url = github:informalsystems/cosmos.nix;
    sovereign-nix.url = github:informalsystems/sov-rollup-starter;
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
      sovereign-nix = inputs.sovereign-nix.packages.${system};
    in {
      packages = {
        inherit
          (cosmos-nix)
          ibc-go-v7-simapp
          ;

        inherit
          (sovereign-nix)
          gaia
          celestia-app
          celestia-node
        ;
      };
    });
}
