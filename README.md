I started this repository to work through the Cryptopals challenges in Rust.
I wanted to learn how to write in Rust, and I wanted to learn how to crack sloppy crypto.

# How to use
Run the following to set up the repo:
```bash
nix-shell
make get-data

# run the code for the ninth challenge
cargo run -- --set 9
# assign `set' to a garbage string to run all challenges sequentially
cargo run -- --set asdf

# run additional tests
cargo test
```
