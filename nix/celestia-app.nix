{
    nixpkgs
,   celestia-app-src
}:
let
    celestia-app = nixpkgs.buildGo122Module {
        name = "celestia-app";
        version = "2.0.0";
        src = celestia-app-src;
        vendorHash = "sha256-0O+ODTtCfDgahizZDYFEdgIYwi4w3lY6sPZp0Q7h4FY=";
        doCheck = false;
        excludedPackages = [
            "./test/interchain"
            "./test/ledger"
            "./test/testground"
        ];
    };
in
celestia-app