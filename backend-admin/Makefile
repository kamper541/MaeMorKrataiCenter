.PHONY: check test build dev migrate-local migrate-staging migrate-prod create-user

check:
	cargo check --target wasm32-unknown-unknown

test:
	cargo test --workspace

build:
	worker-build --release

dev:
	npx wrangler dev

migrate-local:
	npx wrangler d1 migrations apply DB --local

migrate-staging:
	npx wrangler d1 migrations apply DB --env staging --remote

migrate-prod:
	npx wrangler d1 migrations apply DB --env prod --remote

# Usage: make create-user EMAIL=owner@maemorkratai.com ROLE=owner ENV=dev
create-user:
	cargo run --manifest-path tools/create-user/Cargo.toml -- --email=$(EMAIL) --role=$(ROLE) --env=$(or $(ENV),dev)
