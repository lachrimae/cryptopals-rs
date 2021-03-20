{ rustPlatform, lib }:
rustPlatform.buildRustPackage rec {
  pname = "cryptopals-rs";
  version = "0.1.0";

  src = ./.;

  cargoSha256 = "165mks0drd33v640rqb2w738y8i7mla62w1d29hhy72l2sbzcnny";
}
