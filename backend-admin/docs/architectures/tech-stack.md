# Tech Stack & Environment Conventions

## 1. Core Runtime & Languages
- **Primary Language**: Rust (`edition = "2021"`, `rust-version = "1.85"`)
- **Compilation Target**: `wasm32-unknown-unknown` (Cloudflare Workers) and host native target (CLI & `cargo test`)
- **Worker Runtime / CLI**: Cloudflare `workerd` via `wrangler` (`^4.0.0`) and `worker-build`
- **Package Manager**: Cargo (Rust workspace: `.`, `tools/create-user`) + npm (`package.json` for Wrangler scripts)

## 2. Key Frameworks & Libraries
- **Worker & HTTP**: `worker` (`0.8.5`, features: `http`, `d1`, `axum`), `axum` (`0.8`, `default-features = false`, features: `json`, `macros`, `query`), `http` (`1.3`), `tower-service` (`0.3`)
- **Cryptography**:
  - `p256` (`0.13`, features: `ecdsa`, `pkcs8`) — `ES256` JWT signing & verification
  - `aes-gcm` (`0.10.3`) — AES-256-GCM envelope encryption for JWKS private keys
  - `rsa` (`0.9.10`, features: `sha2`) — `RS256` signature verification only (Zero Trust readiness)
  - `pbkdf2` (`0.12`, features: `hmac`) — host PBKDF2-HMAC-SHA256 (`100,000` iterations); `web-sys` `SubtleCrypto` on `wasm32`
  - `hmac` (`0.12`), `sha2` (`0.10`), `subtle` (`2.6`), `zeroize` (`1.8`)
- **Serialization & Utilities**: `serde` (`1`), `serde_json` (`1`), `base64` (`0.22`), `uuid` (`1`, features: `v4`, `js`, `serde`), `async-trait` (`0.1.88`), `getrandom` (`0.2`, features: `js`)
- **Testing Framework**: `cargo test --workspace` with `tower` (`0.5`, features: `util`) and `InMemoryPorts`

## 3. Standard Scripts & Commands
- **Check (`wasm32`)**: `make check` (`cargo check --target wasm32-unknown-unknown`)
- **Test Suite**: `make test` (`cargo test --workspace`)
- **Release Build**: `make build` (`worker-build --release`)
- **Dev Server**: `make dev` (`npx wrangler dev`)
- **Apply Local Migrations**: `make migrate-local` (`npx wrangler d1 migrations apply DB --local`)
- **Provision User**: `make create-user EMAIL=<email> ROLE=<owner|admin> ENV=<dev|staging|prod>`
