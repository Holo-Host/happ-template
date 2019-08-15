{ pkgs ? import ./pkgs.nix {} }: with pkgs;

{
  happ-example = buildDNA {
    name = "happ-example";
    src = gitignoreSource ./.;
  };
}
