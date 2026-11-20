---
id: SPEC-001
title: "Client Backend Infrastructure Scaffold (Rust + Cloudflare Workers + D1 + R2)"
status: APPROVED
spectre_audit: PASSED
created_at: "2026-11-15"
updated_at: "2026-11-15"
parent_spec: ""
sub_specs: []
---

# Specification: Client Backend Infrastructure Scaffold (`backend-client`)

## 1. Intent & High-Level Overview
Establish the production-grade foundational infrastructure scaffold for `backend-client` in **Rust** on **Cloudflare Workers (`wasm32-unknown-unknown`)**, binding to the shared **Cloudflare D1** database (read-only / published-data posture in Phase 1) and the public **Cloudflare R2** bucket (`PUBLIC_ASSETS`), with **hardened public-endpoint security** for account-less Phase 1 customer traffic (`frontend-client`).

This scaffold prepares clean modular boundaries for `SPEC-002` (Cards Catalog & Daily Reading APIs) and future specs (`TODO-002`+ AI LLM Chat / RAG / Agentic Tools, `TODO-003` 3+1 Reading Engine, `TODO-004` Payments/Credits, `TODO-005` Customer Accounts) without exposing unauthenticated AI execution or private storage in Phase 1.

## 2. Research & Spikes (Linked)
- [Research: Rust Cloudflare Workers, D1, R2, Auth & Rate Limiting Stack (2026)](../researches/2026-11-15-rust-cloudflare-workers-stack.md)

## 3. Key Decisions (Linked)
- [D01: Rust Cloudflare Worker Stack, Axum Router & Shared D1 Migrations Ownership](../decisions/D01.md)
- [D02: Shared Cloudflare D1 & R2 Topology with Environment Isolation](../decisions/D02.md)
- [D03: Day-1 Admin Auth, Role Hierarchy, JWT/JWKS Rotation & Refresh Sessions](../decisions/D03.md)
- [D04: Public `backend-client` Security Baseline (Phase 1 Account-less Access)](../decisions/D04.md)

## 4. Behavioral Requirements (EARS Syntax)
- **UBIQUITOUS**: THE SYSTEM SHALL compile to `wasm32-unknown-unknown` using the `worker` crate (`http`, `d1`) bridged to minimal `axum` (`D01`), with isolated Wrangler environment configurations for `dev` (local), `staging`, and `prod` (`D02`).
- **UBIQUITOUS**: THE SYSTEM SHALL bind only to the shared environment-specific D1 database (`DB`) and the public R2 bucket (`PUBLIC_ASSETS`), and SHALL NOT bind `PRIVATE_STORAGE` (`D02`).
- **UBIQUITOUS**: THE SYSTEM SHALL NOT author or run independent D1 schema migrations; canonical schema migrations belong to `backend-admin/migrations/` (`D01`).
- **UBIQUITOUS**: THE SYSTEM SHALL enforce on every HTTP route: strict environment-scoped CORS allowlists (never `*` with credentials), Cloudflare `[[ratelimits]]` throttling, request body size limits, security response headers (`X-Content-Type-Options: nosniff`, `Referrer-Policy`, `Cache-Control: no-store` on dynamic/error responses), and a non-leaking JSON error envelope (`request_id`, `code`, `message`) (`D04`).
- **UBIQUITOUS**: THE SYSTEM SHALL expose zero public LLM/AI chat endpoints and zero customer login/signup routes in `SPEC-001` (`D03`, `D04`).
- **UBIQUITOUS**: THE SYSTEM SHALL include reusable, unit-tested scaffolding modules for Cloudflare Turnstile verification (`siteverify` guard) and standard JWKS JWT verification (`ES256`/`RS256`) so `SPEC-002+` and future authenticated phases plug in without refactoring core middleware (`D03`, `D04`).
- **EVENT-DRIVEN**: WHEN a client requests `GET /health`, THE SYSTEM SHALL verify shared D1 connectivity (`SELECT 1`) and `PUBLIC_ASSETS` R2 binding availability without leaking internal errors, returning HTTP `200` when healthy or HTTP `503` when degraded.
- **EVENT-DRIVEN**: WHEN a browser sends a CORS preflight (`OPTIONS`) or request from an allowlisted origin for the active environment, THE SYSTEM SHALL return explicit `Access-Control-Allow-Origin` and allowed methods/headers; WHEN the origin is not allowlisted, THE SYSTEM SHALL omit CORS allow headers and reject unauthorized cross-origin mutations.
- **UNWANTED BEHAVIOR**: IF client traffic from an IP/key exceeds the configured `[[ratelimits]]` threshold, THE SYSTEM SHALL reject the request with HTTP `429` using the standard JSON error envelope.
- **UNWANTED BEHAVIOR**: IF a request exceeds the configured maximum payload size or sends malformed JSON/headers, THE SYSTEM SHALL reject it with HTTP `413` or `400` via the sanitized JSON error envelope.
- **UNWANTED BEHAVIOR**: IF a Turnstile-guarded endpoint receives a missing, invalid, or upstream-rejected Turnstile token (or fails closed when verification is unavailable), THE SYSTEM SHALL reject the request with HTTP `403` (or `503` on upstream verification outage, never silently bypassing verification).
- **UNWANTED BEHAVIOR**: IF an internal panic, D1 failure, or R2 failure occurs, THE SYSTEM SHALL log structured diagnostics (never logging secrets, tokens, or PII) and return HTTP `500`/`503` using the sanitized error envelope.

## 5. Acceptance Criteria
- [ ] **Scenario 1 (Build, Isolation & Zero-Stubs Verification)**: GIVEN `backend-client` Rust workspace and Wrangler config for `dev`, `staging`, and `prod` WHEN `cargo check --target wasm32-unknown-unknown` and `cargo test` run THEN compilation and unit tests succeed with zero `tokio`/`mio` OS errors, zero `PRIVATE_STORAGE` bindings, and zero `// TODO` placeholders.
- [ ] **Scenario 2 (Health Check & Shared D1/Public R2 Readiness)**: GIVEN shared local D1 initialized from `../backend-admin/migrations/` and `PUBLIC_ASSETS` R2 bound WHEN `GET /health` is called THEN HTTP `200` is returned confirming `d1: ok` and `public_r2: ok` with zero internal config leakage.
- [ ] **Scenario 3 (Strict CORS Allowlist Enforcement)**: GIVEN an allowlisted origin and a disallowed origin in `staging` WHEN each sends a request THEN the allowlisted origin receives matching `Access-Control-Allow-Origin` headers and the disallowed origin never receives wildcard `*` or `Access-Control-Allow-Credentials: true`.
- [ ] **Scenario 4 (Rate Limiting & Sanitized Error Envelope)**: GIVEN requests exceeding `[[ratelimits]]` threshold or triggering a malformed-request / simulated D1 error WHEN handled by the Worker THEN the response returns HTTP `429` / `400` / `503` matching the `{ request_id, code, message }` envelope with zero SQL or stack-trace text.
- [ ] **Scenario 5 (Turnstile & JWKS Reusable Guard Verification)**: GIVEN the Turnstile verification guard and JWKS JWT verifier modules WHEN exercised by unit/integration tests with valid, invalid, expired, and upstream-error fixtures THEN valid tokens pass, invalid/expired tokens fail closed (`401`/`403`), and upstream Turnstile errors fail closed (`503`).

## 6. Scope Boundaries
- **In Scope**:
  - `backend-client` Rust Worker crate (`axum` + `worker`), modular architecture (`routes`, `middleware`, `security`, `db`, `storage`, `error`, `config`).
  - Wrangler config for `dev` (local), `staging`, `prod` sharing D1 (`DB`) and public R2 (`PUBLIC_ASSETS`) only.
  - Global security middleware (CORS allowlist, `[[ratelimits]]`, body size limits, security headers, sanitized error envelope).
  - Reusable `TurnstileGuard` and `JwksVerifier` scaffolding modules (unit-tested, ready for `SPEC-002+`).
  - OpenAPI 3.1 contract skeleton in `../docs/api/client/openapi.yaml`.
  - Unit tests, integration tests, and `docs/architectures/` documentation.
- **Out of Scope**:
  - Public Cards Catalog & Daily Reading APIs (`SPEC-002` — see [OUT_OF_SCOPE.md](./OUT_OF_SCOPE.md)).
  - AI LLM Chat, RAG database & Agentic Tools (`TODO-002`).
  - 3+1 Reading Engine & Server Deck Lock (`TODO-003`).
  - Orders, Credits Ledger & Payment Webhooks (`TODO-004`).
  - Customer Accounts & Saved History (`TODO-005`).

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
