.PHONY: check test build dev

check:
	cargo check --target wasm32-unknown-unknown

test:
	cargo test

build:
	worker-build --release

dev:
	npx wrangler dev
