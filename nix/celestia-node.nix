{
    nixpkgs
,   celestia-node-src
}:
let
    celestia-node = nixpkgs.buildGo122Module {
        name = "celestia-node";
        version = "0.14.1";
        src = celestia-node-src;
        vendorHash = "sha256-UxJNlcIn6z5OYkaA2OyI2EOVDdl+dhypi8IUCyr1X+U=";
        doCheck = false;
    };
in
celestia-node