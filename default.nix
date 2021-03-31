{ rustPlatform, lib }:
rustPlatform.buildRustPackage rec {
  pname = "cryptopals-rs";
  version = "0.1.0";

  src = ./.;

  cargoSha256 = "1xjmvvp5d81ny5dn3w3m5l7fqirkgprvwc7nkjn2f47ba1a8hgyi";
}
