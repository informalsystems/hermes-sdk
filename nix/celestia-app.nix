{
    nixpkgs
,   celestia-app-src
}:
let
    celestia-app = nixpkgs.buildGo122Module {
        name = "celestia-app";
        version = "1.13.0";
        src = celestia-app-src;
        vendorHash = "sha256-HCVb7hmTVaZnO9dfyXHT8RaaS/FyPyH2cdyXRgv4gkE=";
        doCheck = false;
        excludedPackages = [
            "./test/interchain"
            "./test/ledger"
            "./test/testground"
        ];
    };
in
celestia-app