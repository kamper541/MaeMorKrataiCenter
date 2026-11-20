---
id: SPEC-001
title: "Admin Backend Infrastructure Scaffold (Rust + Cloudflare Workers + D1 + R2 + Auth)"
status: APPROVED
spectre_audit: PASSED
created_at: "2026-11-15"
updated_at: "2026-11-15"
parent_spec: ""
sub_specs: []
---

# Specification: Admin Backend Infrastructure Scaffold (`backend-admin`)

## 1. Intent & High-Level Overview
Establish the production-grade foundational infrastructure scaffold for `backend-admin` in **Rust** on **Cloudflare Workers (`wasm32-unknown-unknown`)**, owning shared **Cloudflare D1** migrations, binding both public and private **Cloudflare R2** buckets, and providing Day-1 **authentication, role-based authorization (`owner > admin > user`), and JWT/JWKS key-rotation primitives** compatible with the **Cloudflare Workers Free tier** and cross-origin frontend hosting.

This scaffold prepares clean modular boundaries for future domain specs (`SPEC-002` Cards & Daily CRUD, and `TODO-002`+ AI/RAG/Agentic tools, Knowledge Curation, Content Studio, and Cloudflare Access Zero Trust migration) without prematurely implementing domain business logic.

## 2. Research & Spikes (Linked)
- [Research: Rust Cloudflare Workers, D1, R2, Auth & Rate Limiting Stack (2026)](../researches/2026-11-15-rust-cloudflare-workers-stack.md)

## 3. Key Decisions (Linked)
- [D01: Rust Cloudflare Worker Stack, Axum Router & Shared D1 Migrations Ownership](../decisions/D01.md)
- [D02: Shared Cloudflare D1 & R2 Topology with Environment Isolation](../decisions/D02.md)
- [D03: Day-1 Admin Auth, Role Hierarchy, JWT/JWKS Rotation & Refresh Sessions](../decisions/D03.md)
- [D04: Public `backend-client` Security Baseline](../decisions/D04.md)

## 4. Behavioral Requirements (EARS Syntax)
- **UBIQUITOUS**: THE SYSTEM SHALL compile to `wasm32-unknown-unknown` using the `worker` crate (`http`, `d1`) bridged to minimal `axum` (`D01`), with isolated Wrangler environment configurations for `dev` (local), `staging`, and `prod` (`D02`).
- **UBIQUITOUS**: THE SYSTEM SHALL own canonical shared D1 SQL migrations under `backend-admin/migrations/`, defining at minimum `users` (`role` enum `owner | admin | user`), `sessions`, `jwks_keys`, and `audit_events` (`D01`, `D03`).
- **UBIQUITOUS**: THE SYSTEM SHALL enforce deny-by-default authorization across all `backend-admin` endpoints, permitting unauthenticated access strictly to `GET /health`, `GET /.well-known/jwks.json`, `POST /v1/auth/login`, `POST /v1/auth/refresh`, and `POST /v1/auth/logout` (and exposing zero HTTP signup or bootstrap endpoints).
- **UBIQUITOUS**: THE SYSTEM SHALL provide a local `make create-user` CLI provisioning workflow (`--role=owner|admin|user`, `--email`, `--env=dev|staging|prod`) that reads password input securely via interactive prompt/stdin (never command-line arguments), computes a versioned WebCrypto/PBKDF2-HMAC-SHA256 (`100,000` iterations) + HMAC-SHA256 `PASSWORD_PEPPER` hash, and writes to the target D1 database via `wrangler d1 execute`.
- **UBIQUITOUS**: THE SYSTEM SHALL enforce strict CORS allowlists per environment (with `Access-Control-Allow-Credentials: true` only for explicitly allowlisted origins, never `*`), security response headers (`X-Content-Type-Options: nosniff`, `Referrer-Policy`, `Cache-Control: no-store` on auth/dynamic responses), request body size limits, and a uniform non-leaking JSON error envelope (`request_id`, `code`, `message`).
- **EVENT-DRIVEN**: WHEN a client requests `GET /health`, THE SYSTEM SHALL verify D1 connectivity (`SELECT 1`) and R2 bucket binding availability (`PUBLIC_ASSETS` and `PRIVATE_STORAGE`) without leaking internal errors, returning HTTP `200` when healthy or HTTP `503` when degraded.
- **EVENT-DRIVEN**: WHEN an active `owner` or `admin` submits valid credentials to `POST /v1/auth/login`, THE SYSTEM SHALL verify the versioned PBKDF2-100k + `PASSWORD_PEPPER` password hash via constant-time comparison, issue a short-lived `ES256` access JWT (with `kid`, `iss`, `aud`, `sub`, `role`, `jti`, `exp`), create a SHA-256-hashed refresh token session row in D1, and set the refresh cookie (`HttpOnly; Path=/v1/auth; SameSite=None; Secure` in `staging`/`prod`, `HttpOnly; Path=/v1/auth; SameSite=Lax` in `dev`).
- **EVENT-DRIVEN**: WHEN a client submits a valid refresh cookie to `POST /v1/auth/refresh` (or `POST /v1/auth/logout`) from an allowlisted `Origin` with the required CSRF custom header, THE SYSTEM SHALL atomically rotate (or revoke on logout) the refresh token in D1 (`sessions`), issuing a new token pair in the same `family_id`.
- **EVENT-DRIVEN**: WHEN an authorized `owner` invokes `POST /v1/admin/auth/keys/rotate`, THE SYSTEM SHALL generate a new `ES256` keypair, encrypt the private key with AES-256-GCM using the Wrangler Secret `KEY_ENCRYPTION_KEY` and AAD (`kid || alg`), store it in D1 `jwks_keys`, transition the prior active key to `retiring`, and record an `audit_events` entry.
- **EVENT-DRIVEN**: WHEN a client queries `GET /.well-known/jwks.json`, THE SYSTEM SHALL return all `active` and `retiring` public JWKs (never private key material) with appropriate cache headers.
- **EVENT-DRIVEN**: WHEN any protected route is called, THE SYSTEM SHALL validate the bearer JWT against JWKS (`ES256` and `RS256` supported for future Cloudflare Access cutover), confirm the caller's role satisfies the endpoint's minimum privilege level (`owner > admin > user`), and reject `user`-role tokens on admin routes with HTTP `403`.
- **STATE-DRIVEN**: WHILE a user account has `is_active = false`, THE SYSTEM SHALL deny login, refresh, and protected route access with HTTP `403` and revoke any active refresh requests for that user.
- **UNWANTED BEHAVIOR**: IF an invalid password, unknown email, expired/invalid JWT, or missing token is presented, THE SYSTEM SHALL return an undifferentiated HTTP `401` (never revealing whether an email exists) and enforce Cloudflare `[[ratelimits]]` on auth endpoints (`429` on breach).
- **UNWANTED BEHAVIOR**: IF a request to `POST /v1/auth/refresh` or `POST /v1/auth/logout` lacks an allowlisted `Origin` or required CSRF custom header, THE SYSTEM SHALL reject it with HTTP `403`.
- **UNWANTED BEHAVIOR**: IF a previously rotated or revoked refresh token is replayed at `POST /v1/auth/refresh`, THE SYSTEM SHALL treat it as token theft, immediately revoke all sessions in that `family_id`, clear the refresh cookie, record an `audit_events` security alert, and return HTTP `401`.
- **UNWANTED BEHAVIOR**: IF an internal panic, D1 failure, R2 failure, or `KEY_ENCRYPTION_KEY` decryption error occurs, THE SYSTEM SHALL log structured diagnostics (never logging passwords, refresh tokens, JWTs, or secrets) and return HTTP `500`/`503` using the sanitized error envelope.

## 5. Acceptance Criteria
- [ ] **Scenario 1 (Environment & Build Verification)**: GIVEN `backend-admin` Rust workspace and Wrangler config for `dev`, `staging`, and `prod` WHEN `cargo check --target wasm32-unknown-unknown` and `cargo test` run THEN compilation and unit tests succeed with zero `tokio`/`mio` OS socket errors and zero `// TODO` placeholders.
- [ ] **Scenario 2 (D1 Migrations, CLI Provisioning & Health Check)**: GIVEN local D1 migrations applied via `wrangler d1 migrations apply DB --local`, a user provisioned via `make create-user`, and `PUBLIC_ASSETS` + `PRIVATE_STORAGE` bindings configured WHEN `GET /health` is called THEN HTTP `200` is returned confirming `d1: ok` and `r2: ok` without exposing binding secrets, and the provisioned user row contains only the salted+peppered PBKDF2-100k hash.
- [ ] **Scenario 3 (Deny-by-Default RBAC & Role Hierarchy)**: GIVEN an active `user`-role JWT, an active `admin`-role JWT, and an active `owner`-role JWT WHEN each calls an `admin`-level route (`GET /v1/me`) and an `owner`-only route (`POST /v1/admin/auth/keys/rotate`) THEN `user` receives `403` on admin/owner routes, `admin` succeeds on `admin` routes and receives `403` on `owner`-only routes, and `owner` succeeds on both.
- [ ] **Scenario 4 (Login, Cross-Origin Cookie, Refresh Rotation & Replay Detection)**: GIVEN an active `admin` user on an allowlisted cross-origin client WHEN they login, refresh once with `token_1` + CSRF header (receiving `token_2`), and then replay `token_1` THEN the login sets `HttpOnly; Path=/v1/auth; SameSite=None; Secure` (or `SameSite=Lax` in `dev`), the first refresh returns `200` with a new access JWT and `token_2`, and the replay of `token_1` returns `401` and revokes the entire `family_id` (`token_2` is rejected thereafter).
- [ ] **Scenario 5 (Envelope-Encrypted JWKS Key Rotation Continuity)**: GIVEN an `ES256` JWT signed by `kid_1` WHEN `owner` rotates signing keys to `kid_2` THEN D1 stores only AES-256-GCM ciphertext for `kid_2`'s private key (bound with `kid || alg` AAD), `GET /.well-known/jwks.json` publishes both `kid_1` and `kid_2` public keys, tokens signed by `kid_1` verify until expiry, newly minted tokens carry `kid_2`, and an `audit_events` row is written.
- [ ] **Scenario 6 (Auth Rate Limit, CSRF Guard & Error Sanitization)**: GIVEN repeated failed login attempts exceeding `[[ratelimits]]` OR a `POST /v1/auth/refresh` lacking the CSRF header / allowlisted `Origin` WHEN processed by the Worker THEN the Worker returns `429` or `403` with the `{ request_id, code, message }` envelope and zero credential/stack leakage.

## 6. Scope Boundaries
- **In Scope**:
  - `backend-admin` Rust Worker crate (`axum` + `worker`), modular architecture (`routes`, `middleware`, `auth`, `db`, `storage`, `error`, `config`).
  - Wrangler config for `dev` (local), `staging`, `prod` with shared D1 (`DB`) and R2 (`PUBLIC_ASSETS`, `PRIVATE_STORAGE`).
  - Canonical D1 SQL migrations (`users`, `sessions`, `jwks_keys`, `audit_events`).
  - `Makefile` + `create-user` local provisioning script (secure password prompt/stdin, PBKDF2-100k + `PASSWORD_PEPPER`).
  - Auth & RBAC endpoints (`/health`, `/.well-known/jwks.json`, `/v1/auth/login`, `/v1/auth/refresh`, `/v1/auth/logout`, `/v1/admin/auth/keys/rotate`, `/v1/me`).
  - OpenAPI 3.1 contract skeleton in `../docs/api/admin/openapi.yaml`.
  - Unit tests, integration tests, and `docs/architectures/` documentation.
- **Out of Scope**:
  - Domain data CRUD (`SPEC-002`: Cards & Daily Reading D1/R2 migration — see [OUT_OF_SCOPE.md](./OUT_OF_SCOPE.md)).
  - Argon2id password hashing upgrade (`TODO-007` — deferred until Workers Paid tier).
  - AI LLM Chat, RAG database & Agentic Tools (`TODO-002`).
  - Self-service user signup, Knowledge Curation/Eval (`TODO-003`), Orders/Reconciliation (`TODO-004`), Content Studio & Social Center (`TODO-005`), Cloudflare Access Cutover (`TODO-006`).

## 7. Spectre Auditor Review & Sign-Off Checklist
- **Auditor**: Spectre (`.gb-sdlc/agents/spectre.md`)
- **Spectre Audit Verdict**: PASSED
- [x] **Zero Assumptions Verified**: No unconfirmed architectures, database models, or unspoken requirements. All human choices captured in `docs/decisions/`.
- [x] **Interrogation Audit**: Failure modes, network drops, timeouts, invalid payloads, concurrency, and permission checks probed and resolved.
- [x] **EARS Behavioral Syntax**: Requirements rigorously structured (Ubiquitous, Event-driven, State-driven, Unwanted behavior).
- [x] **Acceptance Testability**: Concrete GIVEN-WHEN-THEN scenarios; zero hand-waving or ambiguity.
- [x] **Scope Watchdog & Split-Spec Check**: Scope is razor-sharp and single-focus; excess complexity deferred to `OUT_OF_SCOPE.md` or `TODO.md`.
- [x] **Security Pre-Check**: Priority #1 Security and OWASP considerations validated.
- **Auditor Sign-Off**: Spectre (`PASSED` on 2026-11-15)
- **Human Sign-Off**: Owner (`APPROVED` on 2026-11-15)
