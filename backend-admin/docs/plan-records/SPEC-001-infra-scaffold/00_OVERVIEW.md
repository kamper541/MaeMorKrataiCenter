# Plan 00: Overview & Task Manifest

- **Spec**: [SPEC-001](../../specs/SPEC-001-infra-scaffold.md)
- **Created**: 2026-11-16
- **Status**: IN_PROGRESS

## 1. Objective & Architecture Impact
Build the `backend-admin` Rust Worker scaffold targeting `wasm32-unknown-unknown` with `worker` (`0.8.5`, `http`, `d1`, `axum`) and `axum` (`0.8`, `default-features = false`). The service owns canonical shared D1 migrations (`migrations/0001_init_auth.sql`), binds D1 (`DB`) and both R2 buckets (`PUBLIC_ASSETS`, `PRIVATE_STORAGE`), isolates `dev` / `staging` / `prod` Wrangler environments, provides local `make create-user` provisioning, and enforces Day-1 admin authentication, deny-by-default RBAC (`owner > admin > user`), AES-256-GCM envelope-encrypted `ES256` JWKS key rotation, and refresh-token replay revocation.

## 2. Battleground Summary
- **State Bridging Debate**: [docs/battlegrounds/SPEC-001-infra-scaffold/axum-worker-state-bridging/debate.md](../../battlegrounds/SPEC-001-infra-scaffold/axum-worker-state-bridging/debate.md)
  - **Winner (Option B)**: Trait-backed `AppPorts` (`AuthRepo`, `StorageProbe`, `RateLimitPort`, `SecretStore`, `Clock`) with `SendWrapper<worker::Env>` isolated strictly inside `WorkerEnvAdapter`, paired with `InMemoryPorts` for host `cargo test` coverage.
- **Crypto & Hashing Debate**: [docs/battlegrounds/SPEC-001-infra-scaffold/crypto-jwt-jwks-and-pbkdf2/debate.md](../../battlegrounds/SPEC-001-infra-scaffold/crypto-jwt-jwks-and-pbkdf2/debate.md)
  - **Winner (Option B)**: Hybrid `SubtleCrypto` PBKDF2-HMAC-SHA256 (`100,000` iterations) under `#[cfg(target_arch = "wasm32")]` and pure-Rust `pbkdf2` under `#[cfg(not(target_arch = "wasm32"))]` with HMAC-SHA256 `PASSWORD_PEPPER` and `subtle::ConstantTimeEq`, paired with `p256` (`ES256`), `aes-gcm` (`0.10.3`), `rsa` (verify-only `RS256`), and `sha2`.

## 3. Task Manifest
> **Execution Policy**: Each task below MUST be executed by a dedicated, fresh **Executor / Coder** agent. Before execution, each agent must read this `00_OVERVIEW.md` file first. Strictly 1 agent per 1 task.
- [x] `01_RUST_WORKSPACE_AND_WRANGLER.md` — Configure Cargo workspace (`backend-admin` + `tools/create-user`), `.cargo/config.toml`, `wrangler.toml` (`dev`/`staging`/`prod`), `.dev.vars.example`, `package.json`, `src/config.rs`, and `src/error.rs`.
- [x] `02_D1_MIGRATIONS_AND_CREATE_USER_CLI.md` — Create canonical `migrations/0001_init_auth.sql`, `tools/create-user` CLI binary (secure tty/stdin password + peppered PBKDF2-100k), and `Makefile`.
- [x] `03_CRYPTO_PASSWORD_ENVELOPE_AND_JWT.md` — Implement hybrid `SubtleCrypto`/host PBKDF2-100k password hashing, AES-256-GCM key envelope encryption (`kid|alg` AAD), and `ES256`/`RS256` JWT + JWK primitives.
- [x] `04_PORTS_AND_WORKER_ADAPTERS.md` — Define `AppPorts` traits, `WorkerEnvAdapter` (parameterized D1 queries + atomic `batch` rotation, R2 probe, rate limiter), and `InMemoryPorts` test adapter.
- [x] `05_MIDDLEWARE_AUTH_AND_ROUTES.md` — Implement request ID, strict CORS, security headers, body size cap, rate limit guard, deny-by-default RBAC extractors, all `SPEC-001` endpoints, and `src/lib.rs` Worker entrypoint.
- [x] `06_TESTS_AND_OPENAPI_CONTRACT.md` — Add `tests/spec_001_scenarios.rs` covering `SPEC-001` Scenarios 1 to 6 and author `../docs/api/admin/openapi.yaml`.
- [x] `07_DOCUMENTATION.md` — Write permanent feature documentation to `docs/knowledges/features/infra-scaffold-and-auth.md` and update `docs/architectures/`.
