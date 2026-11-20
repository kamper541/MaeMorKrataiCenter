# Task 05: Documentation & Architecture Update

- **Plan Overview**: [00_OVERVIEW.md](./00_OVERVIEW.md)
- **Target Knowledge File**: `docs/knowledges/features/infra-scaffold-and-security.md`
- **Executor / Coder Rule**:
  1. ALWAYS read [00_OVERVIEW.md](./00_OVERVIEW.md) first before executing.
  2. Execute ONLY this documentation task as a fresh agent. When complete, hand off to Verifier.

## 1. Objective
Produce permanent documentation for the `backend-client` infrastructure scaffold and security middleware, and update the architecture records under `docs/architectures/`.

## 2. Feature Documentation Content
Write to `backend-client/docs/knowledges/features/infra-scaffold-and-security.md`:
- **Overview**: `worker` + `axum` Worker architecture, `ClientPorts` isolation, shared D1 (`DB`) + `PUBLIC_ASSETS` R2 binding (zero `PRIVATE_STORAGE`, zero independent migrations), and `dev`/`staging`/`prod` Wrangler environments.
- **How to Use / API Specs**: `Makefile` commands (`check`, `test`, `build`, `dev`), Wrangler secrets (`TURNSTILE_SECRET_KEY`), and `../docs/api/client/openapi.yaml`.
- **Guidelines & Invariants**:
  - Strict environment CORS allowlist (never `*` with credentials).
  - Global `[[ratelimits]]` throttling (`PUBLIC_RATE_LIMITER`), body size caps (`413`), security headers (`nosniff`, `Referrer-Policy`, `Cache-Control: no-store`), and `{ request_id, code, message }` error envelope.
  - `TurnstileGuard` fail-closed semantics (`403` missing/invalid, `503` upstream unavailable).
  - `JwksVerifier` (`ES256`/`RS256`, rejecting `alg: none` and algorithm confusion, failing closed with `401`).
- **Linked Decisions**: `D01.md`, `D02.md`, `D03.md`, `D04.md`.

## 3. Architecture Update
Update `backend-client/docs/architectures/tech-stack.md` and create `backend-client/docs/architectures/database.md` and `backend-client/docs/architectures/cloud-infrastructure.md` documenting:
- Rust `1.85+` / `wasm32-unknown-unknown`, `worker 0.8.5`, `axum 0.8`, `p256 0.13`, `rsa 0.9.10` (verify-only).
- Shared D1 schema ownership policy (`backend-admin/migrations/` is canonical; `backend-client` has zero `migrations_dir`).
- Least-privilege R2 (`PUBLIC_ASSETS` only) and `[[ratelimits]]` (`PUBLIC_RATE_LIMITER`) per environment (`dev`, `staging`, `prod`).
