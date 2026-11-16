# Tech Stack & Environment Conventions

## 1. Core Runtime & Languages
- **Primary Language**: Rust (`edition = "2021"`, `rust-version = "1.85"`)
- **Compilation Target**: `wasm32-unknown-unknown` (Cloudflare Workers)
- **Runtime**: Cloudflare Workers (`compatibility_date = "2026-11-01"`)
- **CLI / Package Manager**: Cargo (`cargo`) + npm (`wrangler ^4.0.0` in `package.json`)

## 2. Key Frameworks & Libraries
- **Worker Runtime & HTTP Bridge**: `worker = "0.8.5"` (`features = ["http", "d1", "axum"]`), `tower-service = "0.3"`, `http = "1.3"`
- **Web / API Router**: `axum = "0.8"` (`default-features = false`, `features = ["json", "macros", "query"]`)
- **Cryptography (Verify-Only)**: `p256 = "0.13"` (`ecdsa`), `rsa = "0.9.10"` (`sha2`), `sha2 = "0.10"` — used by `JwksVerifier` for `ES256` / `RS256` JWT verification
- **Serialization & Utilities**: `serde = "1"`, `serde_json = "1"`, `base64 = "0.22"`, `bytes = "1"`, `uuid = "1"` (`v4`, `js`, `serde`), `async-trait = "0.1.88"`, `getrandom = "0.2"` (`js`)
- **Testing Framework**: Built-in `cargo test` with `tower = "0.5"` (`util`), `rand_core = "0.6"`, `signature = "2.2"`, and in-memory `MemoryClientStore`

## 3. Standard Scripts & Commands
Run from `backend-client/`:
- **Target Check**: `make check` (`cargo check --target wasm32-unknown-unknown`)
- **Test Suite**: `make test` (`cargo test`)
- **Release Build**: `make build` (`worker-build --release`)
- **Local Dev Server**: `make dev` (`npx wrangler dev`)
- **Deploy**: `npm run deploy:staging` / `npm run deploy:prod`
