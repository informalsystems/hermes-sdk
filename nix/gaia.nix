{
    nixpkgs
,   gaia-src
}:
let
    gaia = nixpkgs.buildGoModule {
        name = "gaia";
        version = "14.1.0";
        src = gaia-src;
        vendorHash = "sha256-zKeVgrvINTuIU2EI7HyzYR3gnQyQ2qTAMiOHmC0ln/o=";
        doCheck = false;
    };
in
gaia