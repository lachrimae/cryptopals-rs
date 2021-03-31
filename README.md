I started this repository to work through the Cryptopals challenges in Rust.
I wanted to learn how to write in Rust, and I wanted to learn how to crack sloppy crypto.

# How to use
```bash
# access useful tools
nix-shell

# build as a Nix derivation
nix-build ./package.nix

# download various ciphertexts from the cryptopals website
make get-data

# run the code for the ninth challenge
cargo run -- --set 9
# assign `set' to a garbage string to run all challenges sequentially
cargo run -- --set asdf

# run additional tests
cargo test
```
