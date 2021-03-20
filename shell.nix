with import ./.nix/nixpkgs.nix;
mkShell {
  name = "embedded-shell";
  buildInputs = [
    gnumake
    rustc
    cargo
  ];
}
