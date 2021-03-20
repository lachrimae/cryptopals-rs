pkg:
	nix-build ./release.nix

shell:
	nix-shell

build:
	cargo build

run:
	cargo run
