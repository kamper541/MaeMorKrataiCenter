# Plan 00: Overview & Task Manifest

- **Spec**: [SPEC-001](../../specs/SPEC-001-infra-scaffold.md)
- **Created**: 2026-11-16
- **Status**: IN_PROGRESS

## 1. Objective & Architecture Impact
Build the `backend-client` Rust Worker scaffold targeting `wasm32-unknown-unknown` with `worker` (`0.8.5`, `http`, `d1`, `axum`) and `axum` (`0.8`, `default-features = false`). The service binds strictly to the shared D1 database (`DB`) and the public R2 bucket (`PUBLIC_ASSETS`), never `PRIVATE_STORAGE`, and never owns D1 migrations (`backend-admin/migrations/` is canonical). It configures isolated `dev` / `staging` / `prod` Wrangler environments, global security middleware (CORS allowlist, `[[ratelimits]]`, body size limits, security headers, sanitized error envelope), and reusable `TurnstileGuard` and `JwksVerifier` modules ready for `SPEC-002+` and `TODO-005`.

## 2. Battleground Summary
- **State Bridging Debate**: [docs/battlegrounds/SPEC-001-infra-scaffold/axum-worker-state-bridging/debate.md](../../battlegrounds/SPEC-001-infra-scaffold/axum-worker-state-bridging/debate.md)
  - **Winner (Option B)**: Trait-backed `ClientPorts` (`HealthPort`, `RateLimitPort`) with `SendWrapper<worker::Env>` isolated strictly inside `WorkerClientAdapter` (binding `DB`, `PUBLIC_ASSETS`, and `PUBLIC_RATE_LIMITER` only), paired with `MemoryClientStore` for host `cargo test` coverage.
- **Turnstile & JWKS Guard Debate**: [docs/battlegrounds/SPEC-001-infra-scaffold/turnstile-and-jwks-guard-design/debate.md](../../battlegrounds/SPEC-001-infra-scaffold/turnstile-and-jwks-guard-design/debate.md)
  - **Winner (Option B)**: Trait-based `TurnstileTransport` and `JwksSource` backing composable Axum extractors/verifiers (`TurnstileGuard`, `JwksVerifier`), failing closed (`403`/`503` for Turnstile, `401` for JWT/JWKS) and supporting `ES256` and `RS256`.

## 3. Task Manifest
> **Execution Policy**: Each task below MUST be executed by a dedicated, fresh **Executor / Coder** agent. Before execution, each agent must read this `00_OVERVIEW.md` file first. Strictly 1 agent per 1 task.
- [ ] `01_RUST_CRATE_AND_WRANGLER.md` — Configure Cargo crate, `.cargo/config.toml`, `wrangler.toml` (`dev`/`staging`/`prod`, zero `PRIVATE_STORAGE`, zero `migrations_dir`), `.dev.vars.example`, `package.json`, `Makefile`, `src/config.rs`, and `src/error.rs`.
- [ ] `02_PORTS_AND_WORKER_ADAPTERS.md` — Define `ClientPorts` traits (`HealthPort`, `RateLimitPort`), `WorkerClientAdapter` (`DB` + `PUBLIC_ASSETS` + `PUBLIC_RATE_LIMITER`), and `MemoryClientStore`.
- [ ] `03_SECURITY_MIDDLEWARE_AND_GUARDS.md` — Implement request ID, strict CORS allowlist, security headers, body size cap, rate-limit middleware, `TurnstileGuard`, and `JwksVerifier` (`ES256`/`RS256`).
- [ ] `04_ROUTES_TESTS_AND_OPENAPI.md` — Implement `GET /health`, `build_router`, `#[event(fetch)]` Worker entrypoint, `tests/spec_001_scenarios.rs`, and `../docs/api/client/openapi.yaml`.
- [ ] `05_DOCUMENTATION.md` — Write permanent feature documentation to `docs/knowledges/features/infra-scaffold-and-security.md` and update `docs/architectures/`.
