let
  pkgs = import ./.nix/nixpkgs.nix;
in
  pkgs.callPackage ./default.nix {}
