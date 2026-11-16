# Feature: Infrastructure Scaffold & Day-1 Admin Authentication (`SPEC-001`)

- **Status**: Implemented (Pending Verifier Sign-off)
- **Spec**: [SPEC-001-infra-scaffold.md](../../specs/SPEC-001-infra-scaffold.md)
- **Plan**: [00_OVERVIEW.md](../../plans/SPEC-001-infra-scaffold/00_OVERVIEW.md)
- **OpenAPI Contract**: [docs/api/admin/openapi.yaml](../../../../docs/api/admin/openapi.yaml)

---

## 1. Overview & Architecture

`backend-admin` is a Rust Cloudflare Worker targeting `wasm32-unknown-unknown`, built with `worker` (`0.8.5`, features `http`, `d1`, `axum`) and minimal `axum` (`0.8`, `default-features = false`, features `json`, `macros`, `query`).

### `AppPorts` Isolation (`src/ports.rs`, `src/adapters/`)
To keep Axum handlers `Send + Sync` and unit-testable on the host without `workerd` FFI, `worker::Env` (`!Send`/`!Sync`) is isolated strictly inside `WorkerEnvAdapter` (`src/adapters/worker_env.rs`) via `SendWrapper`. Handlers and middleware depend only on `AppPorts`:
- `AuthRepo`: D1-backed user, session, JWKS key, and audit event persistence (with atomic D1 `batch` refresh-session rotation).
- `StorageProbe`: R2 bucket availability probes for `PUBLIC_ASSETS` and `PRIVATE_STORAGE`.
- `RateLimitPort`: Auth endpoint rate limiting via Cloudflare `AUTH_RATE_LIMITER` (`[[ratelimits]]`).
- `SecretStore`: Access to `PASSWORD_PEPPER` and `KEY_ENCRYPTION_KEY`.
- `Clock`: Timestamp provider (`now_secs`).

For host `cargo test` execution, `InMemoryPorts` (`src/adapters/memory.rs`) implements the same `AppPorts` traits in memory.

### Bindings & Environments (`wrangler.toml`)
`backend-admin` owns the canonical shared D1 schema migrations (`migrations/0001_init_auth.sql`) and binds:
- **D1 Database**: `DB`
- **R2 Buckets**: `PUBLIC_ASSETS` and `PRIVATE_STORAGE`
- **Rate Limiter**: `AUTH_RATE_LIMITER` (`10` requests per `60` seconds)

Three isolated Wrangler environments are configured: `dev` (`maemorkratai-backend-admin-dev`), `staging` (`maemorkratai-backend-admin-staging`), and `prod` (`maemorkratai-backend-admin-prod`), each with distinct `database_name`/`database_id`, R2 bucket names, `ALLOWED_ORIGINS`, `JWT_ISSUER`, and secrets.

---

## 2. How to Use / API Specs

### Standard Commands (`Makefile`)
Run from `backend-admin/`:
- `make check`: `cargo check --target wasm32-unknown-unknown`
- `make test`: `cargo test --workspace`
- `make build`: `worker-build --release`
- `make dev`: `npx wrangler dev`
- `make migrate-local`: `npx wrangler d1 migrations apply DB --local`
- `make migrate-staging`: `npx wrangler d1 migrations apply DB --env staging --remote`
- `make migrate-prod`: `npx wrangler d1 migrations apply DB --env prod --remote`
- `make create-user EMAIL=<email> ROLE=<owner|admin> ENV=<dev|staging|prod>`: Provisions an admin/owner user via `tools/create-user` (reads password and pepper securely via prompt/stdin, never CLI flags).

### Required Secrets
- Local development: copy `.dev.vars.example` to `.dev.vars` (never commit `.dev.vars`) and set `PASSWORD_PEPPER` and `KEY_ENCRYPTION_KEY` (base64-encoded 32-byte AES-256 key).
- Remote environments: configure `PASSWORD_PEPPER` and `KEY_ENCRYPTION_KEY` via `npx wrangler secret put <NAME> --env <staging|prod>`.

### HTTP Endpoints
Full request/response schemas are defined in [docs/api/admin/openapi.yaml](../../../../docs/api/admin/openapi.yaml):
- `GET /health` — Health check probing D1 (`DB`) and R2 (`PUBLIC_ASSETS`, `PRIVATE_STORAGE`).
- `GET /.well-known/jwks.json` — Serves `active` and `retiring` public JWKs.
- `POST /v1/auth/login` — Authenticates email + password, issues short-lived `ES256` access JWT (`900s` TTL) and sets `HttpOnly` refresh cookie (`mmk_admin_rt`, `604800s` TTL).
- `POST /v1/auth/refresh` — Single-use refresh token rotation (requires `Origin` allowlist + `X-CSRF-Protection: 1`).
- `POST /v1/auth/logout` — Revokes current refresh session and clears cookie (requires `Origin` allowlist + `X-CSRF-Protection: 1`).
- `GET /v1/me` — Returns authenticated user identity and role (`owner | admin | user`).
- `POST /v1/admin/auth/keys/rotate` — `owner`-only endpoint to generate a new `ES256` signing key and transition the previous active key to `retiring`.

---

## 3. Security & Operational Invariants

- **Zero HTTP Signup / Bootstrap Endpoints**: There is no public or secret-gated HTTP user-creation endpoint. Initial `owner`/`admin` accounts are provisioned strictly out-of-band via `make create-user` (`tools/create-user`), reading secrets via terminal prompt or stdin.
- **Peppered PBKDF2-HMAC-SHA256 (`100,000` iterations)**: Passwords are hashed using `SubtleCrypto` PBKDF2-HMAC-SHA256 (`100,000` iterations) on `wasm32` and pure-Rust `pbkdf2` (`100,000` iterations) on host, combined with an HMAC-SHA256 server-side `PASSWORD_PEPPER` and verified with `subtle::ConstantTimeEq`.
- **AES-256-GCM Key Envelope Encryption**: `ES256` private keys stored in `jwks_keys.encrypted_private_key` are encrypted with `AES-256-GCM` (`KEY_ENCRYPTION_KEY`) bound to AAD `kid|alg`.
- **Refresh Token Single-Use Rotation & Replay Revocation**: Refresh tokens are opaque CSPRNG values stored only as SHA-256 hashes in `sessions`. Presenting an already-rotated/revoked token triggers replay detection and revokes the entire `family_id`. Refresh and logout routes enforce strict `Origin` allowlist validation and `X-CSRF-Protection: 1`.
- **Deny-by-Default RBAC**: Role hierarchy is `owner > admin > user`. Protected routes require a valid `ES256`/`RS256` bearer JWT and explicit role authorization (`owner` for key rotation).

---

## 4. Linked Decisions

- [D01: Rust Cloudflare Worker Stack, Axum Router & Shared D1 Migrations Ownership](../../decisions/D01.md)
- [D02: Shared Cloudflare D1 & R2 Topology with Environment Isolation](../../decisions/D02.md)
- [D03: Day-1 Admin Auth, Role Hierarchy, JWT/JWKS Rotation & Refresh Sessions](../../decisions/D03.md)
- [D04: Public `backend-client` Security Baseline (Phase 1 Account-less Access)](../../decisions/D04.md)
