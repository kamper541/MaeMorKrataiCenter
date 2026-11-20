# Task 07: Documentation & Architecture Update

- **Plan Overview**: [00_OVERVIEW.md](./00_OVERVIEW.md)
- **Target Knowledge File**: `docs/knowledges/features/infra-scaffold-and-auth.md`
- **Executor / Coder Rule**:
  1. ALWAYS read [00_OVERVIEW.md](./00_OVERVIEW.md) first before executing.
  2. Execute ONLY this documentation task as a fresh agent. When complete, hand off to Verifier.

## 1. Objective
Produce permanent documentation for the `backend-admin` infrastructure scaffold and Day-1 authentication system, and populate the architecture records under `docs/architectures/`.

## 2. Feature Documentation Content
Write to `backend-admin/docs/knowledges/features/infra-scaffold-and-auth.md`:
- **Overview**: `worker` + `axum` Worker architecture, `AppPorts` isolation, D1 (`DB`) + R2 (`PUBLIC_ASSETS`, `PRIVATE_STORAGE`) bindings, and `dev`/`staging`/`prod` Wrangler environments.
- **How to Use / API Specs**: `Makefile` commands (`migrate-local`, `create-user`, `check`, `test`, `build`), Wrangler secrets (`PASSWORD_PEPPER`, `KEY_ENCRYPTION_KEY`), and links to `../docs/api/admin/openapi.yaml`.
- **Guidelines & Invariants**:
  - Zero HTTP signup/bootstrap endpoints; user provisioning strictly via `make create-user` reading secrets via prompt/stdin.
  - PBKDF2-HMAC-SHA256 (`100,000` iterations) + HMAC-SHA256 `PASSWORD_PEPPER` (`SubtleCrypto` on `wasm32`, pure-Rust `pbkdf2` on host).
  - AES-256-GCM envelope encryption for `ES256` private keys bound to `kid|alg` AAD.
  - Refresh token single-use rotation with `family_id` replay revocation and `Origin` + `X-CSRF-Protection: 1` guard.
  - Deny-by-default RBAC (`owner > admin > user`).
- **Linked Decisions**: `D01.md`, `D02.md`, `D03.md`, `D04.md`.

## 3. Architecture Update
Update `backend-admin/docs/architectures/tech-stack.md` and create `backend-admin/docs/architectures/database.md` and `backend-admin/docs/architectures/cloud-infrastructure.md` documenting:
- Rust `1.85+` / `wasm32-unknown-unknown`, `worker 0.8.5`, `axum 0.8`, `p256 0.13`, `aes-gcm 0.10.3`, `rsa 0.9.10` (verify-only), `pbkdf2 0.12`.
- Canonical D1 migrations ownership (`backend-admin/migrations/0001_init_auth.sql`) and tables (`users`, `sessions`, `jwks_keys`, `audit_events`).
- Cloudflare Workers Free tier (`10 ms` CPU cap), D1, R2 (`PUBLIC_ASSETS` + `PRIVATE_STORAGE`), and `[[ratelimits]]` (`AUTH_RATE_LIMITER`) per environment (`dev`, `staging`, `prod`).
