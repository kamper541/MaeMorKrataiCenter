.PHONY: check test build-client build-admin dev-client dev-admin migrate-local migrate-staging migrate-prod create-user

check:
	cargo check --workspace --exclude create-user --target wasm32-unknown-unknown

test:
	cargo test --workspace

build-client:
	cd client && worker-build --release

build-admin:
	cd admin && worker-build --release

dev-client:
	cd client && npx wrangler dev

dev-admin:
	cd admin && npx wrangler dev

# Migrations are shared (../migrations) and applied through the admin worker's D1 binding.
migrate-local:
	cd admin && npx wrangler d1 migrations apply DB --local

migrate-staging:
	cd admin && npx wrangler d1 migrations apply DB --env staging --remote

migrate-prod:
	cd admin && npx wrangler d1 migrations apply DB --env prod --remote

# Usage: make create-user EMAIL=owner@maemorkratai.com ROLE=owner ENV=dev
create-user:
	cargo run --manifest-path tools/create-user/Cargo.toml -- --email=$(EMAIL) --role=$(ROLE) --env=$(or $(ENV),dev)
