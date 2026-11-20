# Feature: Infrastructure Scaffold & Security Baseline (`SPEC-001`)

- **Spec**: [SPEC-001-infra-scaffold.md](../../specs/SPEC-001-infra-scaffold.md)
- **Plan**: [00_OVERVIEW.md](../../plans/SPEC-001-infra-scaffold/00_OVERVIEW.md)
- **OpenAPI Contract**: [openapi.yaml](../../../../docs/api/client/openapi.yaml)
- **Linked Decisions**: [D01.md](../../decisions/D01.md), [D02.md](../../decisions/D02.md), [D03.md](../../decisions/D03.md), [D04.md](../../decisions/D04.md)

---

## 1. Overview & Architecture

`backend-client` is a Rust Cloudflare Worker compiled to `wasm32-unknown-unknown` using `worker` (`0.8.5`, features `http`, `d1`, `axum`) bridged to `axum` (`0.8`, `default-features = false`).

### Port & Adapter Isolation (`ClientPorts`)
To keep Axum handlers and middleware testable on the host (`cargo test`) without `wasm32` bindings, all Cloudflare Worker bindings are abstracted behind traits in `src/ports.rs`:
- `HealthPort`: checks shared D1 (`DB`) and public R2 (`PUBLIC_ASSETS`) readiness.
- `RateLimitPort`: enforces per-key rate limits via `PUBLIC_RATE_LIMITER`.
- `SecretProvider`: retrieves runtime secrets (e.g., `TURNSTILE_SECRET_KEY`).
- `Clock`: provides current epoch seconds for JWT claim validation (`exp`, `nbf`, `iat`).

Two adapters implement `ClientPorts`:
- `WorkerClientAdapter` (`src/adapters/worker_env.rs`): wraps `!Send`/`!Sync` `worker::Env` inside `SendWrapper` and binds strictly to `DB`, `PUBLIC_ASSETS`, and `PUBLIC_RATE_LIMITER`.
- `MemoryClientStore` (`src/adapters/memory.rs`): in-memory adapter used by unit and integration tests (`tests/spec_001_scenarios.rs`).

### Storage & Migration Ownership
- **Shared D1 (`DB`)**: Binds to the shared Cloudflare D1 database per environment. Canonical SQL migrations live exclusively in `../backend-admin/migrations/`; `backend-client` has zero `migrations_dir` in `wrangler.toml`.
- **Public R2 (`PUBLIC_ASSETS`)**: Binds exclusively to `PUBLIC_ASSETS`. `PRIVATE_STORAGE` is never bound (`D02`).
- **Wrangler Environments**: Isolated `dev`, `staging`, and `prod` environments in `wrangler.toml`, each with distinct `database_id`, R2 bucket names, `PUBLIC_RATE_LIMITER` namespace IDs, `ALLOWED_ORIGINS`, `JWKS_URL`, `JWT_ISSUER`, and `JWT_AUDIENCE`.

---

## 2. How to Use & API Specs

### Standard Commands (`Makefile` & `package.json`)
Run from `backend-client/`:
- `make check` — `cargo check --target wasm32-unknown-unknown`
- `make test` — `cargo test` (host unit + `SPEC-001` integration scenarios)
- `make build` — `worker-build --release`
- `make dev` — `npx wrangler dev`
- `npm run deploy:staging` / `npm run deploy:prod` — deploy to `staging` or `prod` via Wrangler

### Secrets & Local Configuration
- Copy `.dev.vars.example` to `.dev.vars` (gitignored) for local `wrangler dev`.
- Set `TURNSTILE_SECRET_KEY` via `.dev.vars` locally (Cloudflare's dummy always-pass key `1x0000000000000000000000000000000AA` in `.dev.vars.example`) and via `npx wrangler secret put TURNSTILE_SECRET_KEY --env <staging|prod>` in deployed environments.
- `backend-client` must never bind `KEY_ENCRYPTION_KEY` or `PASSWORD_PEPPER` (`D03`).

### API Contract
- Defined in `../docs/api/client/openapi.yaml` (OpenAPI `3.1.0`).
- `GET /health` — verifies `DB` and `PUBLIC_ASSETS` readiness, returning `200` (`{ status, d1, public_r2 }`), `429` (`RATE_LIMITED`), or `503` (`SERVICE_UNAVAILABLE`).

---

## 3. Guidelines & Security Invariants

All routes registered in `build_router(ports)` (`src/lib.rs`) inherit `security_pipeline_middleware` (`src/middleware/security.rs`) and uniform error handling (`src/error.rs`):

1. **Strict Environment CORS Allowlist**:
   - `ALLOWED_ORIGINS` is explicitly configured per environment (`dev`, `staging`, `prod`).
   - Wildcard `*` is never used; disallowed origins on `OPTIONS` preflight or standard requests do not receive `Access-Control-Allow-Origin` (`D04`).
2. **Global Rate Limiting (`[[ratelimits]]`)**:
   - Every request passes through `PUBLIC_RATE_LIMITER` keyed by `cf-connecting-ip` + route. Breaches fail closed with `429 Too Many Requests` (`RATE_LIMITED`).
3. **Body Size Caps, Security Headers & Sanitized Error Envelope**:
   - Requests exceeding `MAX_BODY_BYTES` (default `32768`) are rejected with `413 Payload Too Large` (`PAYLOAD_TOO_LARGE`).
   - Responses include `X-Request-Id`, `X-Content-Type-Options: nosniff`, `Referrer-Policy: strict-origin-when-cross-origin`, and `Cache-Control: no-store`.
   - Errors serialize as `{ request_id, code, message }` without internal SQL, stack traces, or binding details.
4. **`TurnstileGuard` (`src/security/turnstile.rs`)**:
   - Fail-closed verification via `TurnstileTransport` (`WorkerTurnstileTransport` in production): missing/invalid tokens return `403 Forbidden`; missing secret config or upstream siteverify failures return `503 Service Unavailable`.
5. **`JwksVerifier` (`src/security/jwks.rs`)**:
   - Verifies JWTs against JWKS (`JwksSource`) supporting `ES256` (`p256`) and `RS256` (`rsa`) only.
   - Explicitly rejects `alg: none` and algorithm confusion, validates `iss`, `aud`, `exp`, and `nbf`, and fails closed with `401 Unauthorized`.

---

## 4. Linked Decisions
- [D01.md](../../decisions/D01.md) — Rust Cloudflare Worker stack (`worker` + `axum`) and canonical `backend-admin/migrations/` D1 ownership.
- [D02.md](../../decisions/D02.md) — Shared D1 & R2 topology (`PUBLIC_ASSETS` only for `backend-client`) and `dev`/`staging`/`prod` environment isolation.
- [D03.md](../../decisions/D03.md) — Day-1 auth isolation (`backend-client` public in Phase 1, JWKS verifier prepared for future routes).
- [D04.md](../../decisions/D04.md) — Public `backend-client` defense-in-depth security baseline.
