---
id: RES-001
topic: "Rust Cloudflare Workers, D1, R2, Auth & Rate Limiting Stack (2026)"
status: COMPLETED
created_at: "2026-11-15"
investigator: "Researcher"
target_spec: "docs/specs/SPEC-001-infra-scaffold.md"
---

# Research Spike: Rust Cloudflare Workers, D1, R2, Auth & Rate Limiting Stack (2026)

## 1. Executive Summary & Recommendation for Spectre
For `backend-client` and `backend-admin` running on **Cloudflare Workers (`wasm32-unknown-unknown`)** with **D1** and **R2**, the official `worker` crate (`cloudflare/workers-rs`, with `d1` and `http` features) paired with `axum` (stripped of Tokio OS/socket features like `mio`) or `worker::Router`, plus Cloudflare native `[[ratelimits]]` bindings and `wrangler d1 migrations`, provides a mature, production-ready foundation.

Crucially, live investigation uncovered two **Cloudflare Workers platform traps** affecting **Authentication & Password Hashing**:
1. **CPU & Memory Limits for Password Hashing**: Cloudflare Workers Free tier enforces a **10 ms CPU limit** per request, and all Workers share a **128 MB isolate memory cap** (JS + Wasm heap). Pure-Rust Argon2id at standard OWASP settings (~19 MB+, ~50–100+ ms) will **hard-fail on Free tier** and requires Paid tier + conservative memory tuning (e.g., 16 MiB) or alternative hashing. Meanwhile, Cloudflare `workerd` WebCrypto (`crypto.subtle.deriveBits` PBKDF2) enforces a **hard 100,000-iteration ceiling** in production (`NotSupportedError` above 100k—and `wrangler dev` does NOT catch it locally!).
2. **JWT / JWKS Key Rotation on Wasm**: Standard native crypto crates using `ring` or missing entropy fail on `wasm32-unknown-unknown`. Delegating ES256/EdDSA JWT signing to WebCrypto (`SubtleCrypto` / `worker_jwt`) or pure-Rust `p256`/`ed25519-dalek`/`jsonwebtoken` with `getrandom` `"js"`/`"wasm_js"` enabled avoids OS-syscall traps and keeps key rotation + JWKS publishing (`/.well-known/jwks.json`) fast at the edge.

Recommended path for Spectre & the developer:
- **HTTP & Routing**: `worker` crate (`http` + `d1` features) + `axum` (`default-features = false`, `json`) via `tower-service` / `EnvWrapper`, OR lean `worker::Router` (present both as a clear Phase 4 trade-off).
- **Rate Limiting**: Cloudflare native `[[ratelimits]]` Worker binding (`env.rate_limiter(...)`) + Turnstile verification on public state-changing routes.
- **Password Hashing Strategy**: Explicitly choose between (a) Cloudflare Workers **Paid tier** + tuned Argon2id in Rust (`16 MiB`, versioned hash format), or (b) Cloudflare WebCrypto PBKDF2-SHA256 capped at `100,000` iterations + strong pepper/secret, knowing the eventual migration path to **Cloudflare Access Zero Trust**.

## 2. Research Intent & Live Web Search Queries
- **Investigation Objective**: Verify 2026 Rust `worker` crate capabilities for D1, R2, Axum/HTTP routing, native rate limiting, Wrangler D1 multi-environment migrations (`local`/`staging`/`prod`), JWT/JWKS on `wasm32-unknown-unknown`, and password hashing constraints on Cloudflare Workers.
- **Live Web Search Queries**:
  - `"workers-rs" OR "worker" crate cloudflare workers rust D1 R2 2025 2026`
  - `rust cloudflare workers axum router wasm32-unknown-unknown 2025 2026`
  - `rust password hashing argon2 pbkdf2 cloudflare workers wasm32 cpu limit`
  - `rust jwt jwks ed25519 p256 wasm32-unknown-unknown cloudflare workers`
  - `crates.io "worker" cloudflare workers latest version 2025 2026 d1 r2`
  - `cloudflare workers rate limiting binding rust worker crate OR wrangler.toml`
  - `cloudflare workers pbkdf2 subtlecrypto 100000 iteration limit 2025 2026`
  - `cloudflare/workers-rs releases worker crate 0.5 0.6 0.7 d1 r2`
  - `cloudflare d1 migrations wrangler environments local staging production`

## 3. Current State of the Art & Modern Best Practices (2026)
- **`worker` (`cloudflare/workers-rs`) with `http` + `d1` + R2**: Modern releases (`0.5`–`0.7+`) integrate the standard `http` crate (`features = ["http", "d1"]`), typed `D1Database` prepared statements, and `R2Bucket` streaming/put/get bindings, plus `wasm-bindgen` panic recovery.
- **D1 Multi-Environment Migrations**: `wrangler.toml` / `wrangler.jsonc` `[env.staging]` and `[env.production]` isolate `database_id` values per environment; `wrangler d1 migrations apply DB --local` / `--env staging` / `--env production` tracks applied SQL files in `d1_migrations`.
- **Native Rate Limiting Binding**: `[[ratelimits]]` (`period = 10` or `60` seconds) exposes `env.rate_limiter(...)` directly inside Rust Workers without external Redis.
- **Anti-Patterns / Traps to Avoid**:
  - Pulling full `tokio` or `mio` (causes `wasm32-unknown-unknown` compile failure).
  - Forgetting `getrandom` `"js"` / `"wasm_js"` feature (runtime entropy panic).
  - Using WebCrypto PBKDF2 > 100,000 iterations (passes locally in `wrangler dev`, crashes in prod `workerd` with `NotSupportedError`).
  - Attempting Argon2id on Cloudflare Workers Free tier (10 ms CPU limit hard-fails).
  - Sharing `database_id` across `staging` and `production` environments.

## 4. Options & Technology Comparison Matrix
| Option / Component | Security Posture (OWASP) | Maintenance & License | Technical Debt & Simplicity | Status |
| :--- | :--- | :--- | :--- | :--- |
| **`worker` (`http` + `d1`) + `axum` (minimal)** | Strong (Tower middleware, strict typing) | Active (`cloudflare/workers-rs` Apache-2.0/MIT + `tokio-rs/axum` MIT) | Clean modular routers/extractors; requires `Send`/`Sync` `EnvWrapper` for JS bindings | Candidate for Decision |
| **`worker::Router` (native `workers-rs`)** | Strong (minimal surface, zero Axum/Tokio deps) | Active (`cloudflare/workers-rs`) | Smallest Wasm binary & zero `EnvWrapper` friction, but fewer middleware ergonomics | Candidate for Decision |
| **JWT/JWKS via WebCrypto (`ES256`) / `jsonwebtoken` + `getrandom` (`js`)** | Strong (asymmetric keys, clean JWKS rotation) | Active | Fast edge verification; caches JWKS in isolate/KV; clean bridge to Cloudflare Access | Recommended |
| **Password Hash: Paid Tier Tuned Argon2id vs WebCrypto PBKDF2 (100k + HMAC pepper)** | Argon2id > PBKDF2-100k | Active RustCrypto / Native V8 WebCrypto | Argon2id requires Paid Workers tier (`>10ms` CPU); PBKDF2-100k runs on Free tier too | Needs Human Decision |

## 5. Security & Risk Analysis
- **Wasm Isolation & Panic Handling**: Configure `panic = "abort"` carefully with `worker` panic recovery and structured 500 JSON errors (never leak stack traces or SQL errors to clients).
- **Thread-Safety & Binding Leakage**: `worker::Env` is `!Send`/`!Sync` in single-threaded V8 isolates; keep request-scoped bindings cleanly isolated so no request state bleeds across requests in the same isolate.
- **Secret Management**: Signing keys (for temporary private keys prior to CF Access migration), password peppers, and Turnstile secrets must live in Wrangler Secrets (`wrangler secret put`), never in `wrangler.toml` or Git.
- **Key Rotation & Refresh Tokens**: Store active + previous public keys in JWKS (`kid`) with overlap window so rotating signing keys does not immediately invalidate in-flight short-lived access tokens; store **hashed** refresh tokens (`SHA-256`) in D1 `sessions` with revocation support (`revoked_at`, `replaced_by`).

## 6. References & Verified URLs
- `cloudflare/workers-rs` (`worker` crate, `http` feature, D1, R2, rate_limiter docs): https://github.com/cloudflare/workers-rs & https://docs.rs/worker
- Cloudflare Workers CPU / Memory Limits & `workerd` PBKDF2 100k iteration ceiling: Cloudflare Workers Limits docs & `cloudflare/workerd` WebCrypto PBKDF2 issues
- Cloudflare D1 Wrangler Environments & Migrations: Cloudflare D1 Migrations & Wrangler Configuration docs
- Cloudflare Workers Rate Limiting Binding (`[[ratelimits]]`): Cloudflare Workers Rate Limiting API docs
