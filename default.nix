{ pkgs ? import ./pkgs.nix, shell ? false }:

with pkgs;

let
  inherit (darwin.apple_sdk.frameworks) CoreServices Security;
in

{
  happ-example = buildDNA {
    inherit shell;

    name = "happ-example";
    src = gitignoreSource ./.;

    nativeBuildInputs = []
    ++ lib.optionals stdenv.isDarwin [ CoreServices ];
  };
}
