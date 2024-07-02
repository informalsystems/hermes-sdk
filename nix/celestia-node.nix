{
    nixpkgs
,   celestia-node-src
}:
let
    celestia-node = nixpkgs.buildGo121Module {
        name = "celestia-node";
        version = "0.12.0";
        src = celestia-node-src;
        vendorHash = "sha256-iAyXhSV3Up8mb1kyQYxFDC1RUyYMFwG2E+OgG/wUe88=";
        doCheck = false;
    };
in
celestia-node