# Battleground Debate Record

- **Plan**: `SPEC-001-infra-scaffold`
- **Task**: `axum-worker-state-bridging`
- **Date**: 2026-11-16
- **Status**: RESOLVED

## 1. The Conflict / Decision Point
How should `backend-admin` bridge single-threaded `worker::Env` bindings (`D1Database`, `Bucket`, `RateLimiter`, Wrangler Secrets) into `axum::Router` while satisfying `Send + Sync` handler bounds, least-privilege access control, and host `cargo test` execution?

## 2. Competing Options

### Option A: Direct `SendWrapper<worker::Env>` in Axum Handlers
- **Advocate**: Contender Agent A
- **Security (#1)**: Every route handler receives raw `worker::Env` via Axum `State` or `Extension`. Unauthenticated handlers such as `GET /health` and `GET /.well-known/jwks.json` gain direct access to `env.secret("PASSWORD_PEPPER")`, `env.secret("KEY_ENCRYPTION_KEY")`, `PRIVATE_STORAGE`, and raw SQL execution. This weakens OWASP A01 least-privilege isolation and makes error sanitization harder to enforce uniformly.
- **Technical Debt (#2)**: Handlers, middleware, and auth logic couple directly to `worker::Env` and `#[worker::send]`. Host `cargo test` cannot construct `worker::Env`, leaving RBAC, CSRF, and refresh-replay handlers untested outside `wrangler dev`.
- **Speed (#3)**: Saves roughly 150 lines of trait definitions during initial scaffold setup, at the cost of slower debugging in `wrangler dev`.

### Option B: Trait-Backed `AppPorts` with `WorkerEnvAdapter` and In-Memory Test Ports
- **Advocate**: Contender Agent B
- **Security (#1)**: `AppPorts` splits capabilities into narrow traits (`AuthRepo`, `StorageProbe`, `RateLimitPort`, `SecretStore`, `Clock`). `SendWrapper<worker::Env>` remains encapsulated inside `WorkerEnvAdapter`, keeping Wrangler secrets and R2 handles away from routes that do not need them. Parameterized D1 queries live in one repository adapter, and typed errors map to the sanitized `{ request_id, code, message }` JSON envelope before serialization. Full Axum integration tests run in `cargo test` via `tower::ServiceExt::oneshot`, verifying deny-by-default RBAC, replay revocation, CSRF checks, and error sanitization on every run.
- **Technical Debt (#2)**: Route handlers and auth services contain zero `worker` crate imports. Swapping JWT verification for Cloudflare Access in `TODO-006` only changes the JWKS verifier adapter.
- **Speed (#3)**: Requires ~200 lines of port traits and adapter glue upfront, then delivers millisecond host test execution for all auth scenarios.

## 3. Outcome & Winning Rationale
- **Winner**: Option B
- **Justification**: Option B wins on Priority 1 Security and Priority 2 Technical Debt. Isolating `SendWrapper<worker::Env>` inside `WorkerEnvAdapter` enforces least-privilege bindings across modules and enables deterministic host `cargo test` verification of all `SPEC-001` security scenarios.
