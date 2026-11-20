# Backend Handoff (`HANDOFF.md`)

> Merged from the former `backend-client` and `backend-admin` repos. Each part below keeps its own IDs (TODO IDs may collide across parts); docs live in `docs/client/` and `docs/admin/`.

## Current State (both services) — 2026-11-21

- **Project Status**: `PLANNING`
- **Active Spec**: [docs/specs/SPEC-002-master-data-to-backend.md](docs/specs/SPEC-002-master-data-to-backend.md). One spec for both services. `status: APPROVED`, `spectre_audit: PASSED` (3 audit rounds), `complexity: complex`.
- **New shared locations** (from SPEC-002 on): `docs/specs/`, `docs/decisions/` (D05–D10), `docs/researches/`, `docs/knowledges/actors-and-roles.md`. D01–D04 stay in `docs/admin|client/decisions/`.
- **API contracts** v0.2.0 (need frontend review in a PR): `../docs/api/client/openapi.yaml` (public read-only GETs, 10-min cache) and `../docs/api/admin/openapi.yaml` (admin CRUD, 36 operations).
- **New proto-specs**: `TODO-010` pagination, `TODO-011` cache invalidation (client section of `TODO.md`).
- **Immediate Next Steps**:
  1. Open the contract PR for frontend review; notify frontend of English keys (D08), nullable love-status texts and uncategorized lists (D09, D10).
  2. Run `/gb-plan` for SPEC-002 (Opus planners, `complexity: complex`).

---

<!-- ===== client ===== -->

# Project Handoff & Session State (`HANDOFF.md`)

> **Role & Purpose**: This document preserves cross-session continuity between human developers and autonomous AI agents.
> **Rule for Incoming Agents**: Always read this file at the start of a session to understand current status, what has been completed, in-flight work, and immediate next steps. Update this file before concluding your session.

---

## 1. Executive Summary & Progress

- **Project Status**: `IDLE (Verified)`
- **Last Updated**: 2026-11-16
- **Active Session Agent**: `Verifier & Security Auditor (SPEC-001 Verified -> Handing over)`
- **Progress Snapshot**: `SPEC-001` (`SPEC-001-infra-scaffold`) verified and archived to `docs/plan-records/SPEC-001-infra-scaffold/`. All 5 acceptance scenarios, `D01`–`D04`, `cargo test` (8/8), `cargo check --target wasm32-unknown-unknown`, and OWASP Top 10 (A01–A10) security audit passed.

### Milestone Checklist
- [x] Milestone 1: Initial Core Specification (`SPEC-001` Approved)
- [x] Milestone 2: Planning & Battleground (`docs/plans/SPEC-001-infra-scaffold/`)
- [x] Milestone 3: Autonomous Agent Factory Implementation & Verification

---

## 2. What We Have in Current (Current State)

- **Working Components**:
  - `gb-sdlc Engine`: Settled guardrails, agent presets, workflows, and templates installed in `.gb-sdlc/`.
  - `Battleground Records`:
    - [docs/battlegrounds/SPEC-001-infra-scaffold/axum-worker-state-bridging/debate.md](docs/client/battlegrounds/SPEC-001-infra-scaffold/axum-worker-state-bridging/debate.md) (Option B: `ClientPorts` + `WorkerClientAdapter` + `MemoryClientStore`)
    - [docs/battlegrounds/SPEC-001-infra-scaffold/turnstile-and-jwks-guard-design/debate.md](docs/client/battlegrounds/SPEC-001-infra-scaffold/turnstile-and-jwks-guard-design/debate.md) (Option B: Trait-based `TurnstileTransport` + `JwksSource` with `TurnstileGuard` + `JwksVerifier`)
  - `Archived Plan & Audit`: [docs/plan-records/SPEC-001-infra-scaffold/00_OVERVIEW.md](docs/client/plan-records/SPEC-001-infra-scaffold/00_OVERVIEW.md) | [security-audit.md](docs/client/plan-records/SPEC-001-infra-scaffold/security-audit.md)
  - `Task 01 (Rust Crate, Wrangler, Config & Error Envelope)`: `Cargo.toml`, `.cargo/config.toml`, `wrangler.toml`, `package.json`, `.dev.vars.example`, `Makefile`, `.gitignore`, `src/config.rs`, `src/error.rs`, and temporary `src/lib.rs` stub.
  - `Task 02 (Ports & Worker Adapters)`: `src/ports.rs` (`HealthPort`, `RateLimitPort`, `SecretProvider`, `Clock`, `ClientPorts`), `src/adapters/mod.rs`, `src/adapters/worker_env.rs` (`WorkerClientAdapter` binding `DB`, `PUBLIC_ASSETS`, `PUBLIC_RATE_LIMITER` only), `src/adapters/memory.rs` (`MemoryClientStore`), and updated `src/lib.rs`.
  - `Task 03 (Security Middleware & Guards)`: `src/middleware/mod.rs`, `src/middleware/security.rs` (`RequestId`, `security_pipeline_middleware`, `ValidatedJson`), `src/security/mod.rs`, `src/security/turnstile.rs` (`TurnstileTransport`, `WorkerTurnstileTransport`, `TurnstileGuard`, `VerifiedTurnstile`), `src/security/jwks.rs` (`JwksSource`, `WorkerJwksSource`, `JwksVerifier`, `verify_jwt_against_keys` for `ES256`/`RS256`, `VerifiedJwtClaims`), and updated `src/lib.rs`.
  - `Task 04 (Health Route, Worker Entrypoint, SPEC-001 Tests & OpenAPI)`: `src/routes/mod.rs`, `src/routes/health.rs` (`get_health`, `fallback_not_found`), `src/lib.rs` (`build_router`, `#[event(fetch)]`), `tests/spec_001_scenarios.rs` (Scenarios 1-5), and `../docs/api/client/openapi.yaml` (OpenAPI 3.1 contract).
  - `Task 05 (Documentation & Architecture Update)`: `docs/knowledges/features/infra-scaffold-and-security.md`, `docs/architectures/tech-stack.md`, `docs/architectures/database.md`, and `docs/architectures/cloud-infrastructure.md`.
- **Architecture & Tech Stack**:
  - See [docs/architectures/tech-stack.md](docs/client/architectures/tech-stack.md), [docs/architectures/database.md](docs/client/architectures/database.md), and [docs/architectures/cloud-infrastructure.md](docs/client/architectures/cloud-infrastructure.md)
- **Verification Status**:
  - Tests: `cargo test` PASSED (3 unit + 5 integration = 8/8); `cargo check --target wasm32-unknown-unknown` PASSED; `cargo clippy --all-targets -- -D warnings` flagged 1 non-blocking lint (`clippy::unnecessary_cast` at `src/adapters/worker_env.rs:141`, logged as `TODO-006`).
  - Security Audit: `PASSED` (OWASP A01–A10 verified; see [security-audit.md](docs/client/plan-records/SPEC-001-infra-scaffold/security-audit.md))

---

## 3. Active In-Flight Work

- **Active Spec**: None (`SPEC-001` completed & verified)
- **Active Plan**: None (Archived to `docs/plan-records/SPEC-001-infra-scaffold/`)
- **Current Task In-Flight**: None
- **Active Battleground**: None

---

## 4. Leftovers, Blockers & Next Steps

### Immediate Next Steps (For Next Agent / Developer)
Ready for next feature spec session with Spectre:
1. [ ] Graduate `SPEC-002` (Public Cards Catalog & Daily Reading APIs backed by D1 + R2) from [docs/specs/OUT_OF_SCOPE.md](docs/client/specs/OUT_OF_SCOPE.md).

### Leftovers & Technical Nuances
- **Leftover Tasks**: Non-blocking hardening items logged in [TODO.md](./TODO.md) (`TODO-006`).
- **Known Blockers / Warnings**: Ensure `wrangler.toml` shares the exact Cloudflare D1 `database_id` provisioned for `backend-admin` before deploying `--env staging` or `--env prod`.

---

## 5. Proto-Specs & Deferred Ideas

- For improvements or architectural insights discovered during execution that are worth noting for future specs, see [TODO.md](./TODO.md).

---

## 6. Context & Reference Pointers

- **Master Instructions**: [GEMINI.md](./GEMINI.md) | [AGENTS.md](./AGENTS.md)
- **Architectures**: [docs/architectures/](docs/client/architectures/)
- **Decisions Log**: [docs/decisions/](docs/client/decisions/)
- **Researches & Spikes**: [docs/researches/](docs/client/researches/)
- **Completed Plan Records**: [docs/plan-records/](docs/client/plan-records/) (includes [SPEC-001-infra-scaffold](docs/client/plan-records/SPEC-001-infra-scaffold/00_OVERVIEW.md))


---

<!-- ===== admin ===== -->

# Project Handoff & Session State (`HANDOFF.md`)

> **Role & Purpose**: This document preserves cross-session continuity between human developers and autonomous AI agents.
> **Rule for Incoming Agents**: Always read this file at the start of a session to understand current status, what has been completed, in-flight work, and immediate next steps. Update this file before concluding your session.

---

## 1. Executive Summary & Progress

- **Project Status**: `IDLE (Verified)`
- **Last Updated**: 2026-11-16
- **Active Session Agent**: `Verifier & Security Auditor (SPEC-001-infra-scaffold PASSED)`
- **Progress Snapshot**: `SPEC-001` (`SPEC-001-infra-scaffold`) verified and archived to [docs/plan-records/SPEC-001-infra-scaffold/](docs/admin/plan-records/SPEC-001-infra-scaffold/). All 15 workspace tests pass, `wasm32-unknown-unknown` compiles cleanly, and the OWASP Top 10 security audit passed.

### Milestone Checklist
- [x] Milestone 1: Initial Core Specification (`SPEC-001` Approved)
- [x] Milestone 2: Planning & Battleground (`docs/plan-records/SPEC-001-infra-scaffold/`)
- [x] Milestone 3: Autonomous Agent Factory Implementation & Verification

---

## 2. What We Have in Current (Current State)

- **Working Components**:
  - `gb-sdlc Engine`: Settled guardrails, agent presets, workflows, and templates installed in `.gb-sdlc/`.
  - `Battleground Records`:
    - [docs/battlegrounds/SPEC-001-infra-scaffold/axum-worker-state-bridging/debate.md](docs/admin/battlegrounds/SPEC-001-infra-scaffold/axum-worker-state-bridging/debate.md) (Option B: `AppPorts` + `WorkerEnvAdapter` + `MemoryStore`)
    - [docs/battlegrounds/SPEC-001-infra-scaffold/crypto-jwt-jwks-and-pbkdf2/debate.md](docs/admin/battlegrounds/SPEC-001-infra-scaffold/crypto-jwt-jwks-and-pbkdf2/debate.md) (Option B: Hybrid `SubtleCrypto` PBKDF2-100k on `wasm32` + pure-Rust `pbkdf2` on host, `p256`, `aes-gcm`, `rsa` verify-only)
  - `Execution Plan Record`: [docs/plan-records/SPEC-001-infra-scaffold/00_OVERVIEW.md](docs/admin/plan-records/SPEC-001-infra-scaffold/00_OVERVIEW.md)
  - `Security Audit`: [docs/plan-records/SPEC-001-infra-scaffold/security-audit.md](docs/admin/plan-records/SPEC-001-infra-scaffold/security-audit.md) (`PASSED` — OWASP Top 10 A01–A10)
  - `Implemented Foundation (SPEC-001)`: Rust/Axum/Cloudflare Worker scaffold (`dev`/`staging`/`prod`), canonical D1 migrations (`users`, `sessions`, `jwks_keys`, `audit_events`), `make create-user` CLI, PBKDF2-100k + `PASSWORD_PEPPER`, AES-256-GCM envelope-encrypted `ES256`/`RS256` JWKS & key rotation, refresh token rotation with replay detection, deny-by-default RBAC (`owner > admin > user`), CORS/CSRF/rate-limit/error-envelope middleware, OpenAPI 3.1 contract (`../docs/api/admin/openapi.yaml`), and `docs/architectures/` + `docs/knowledges/features/` docs.
- **Architecture & Tech Stack**:
  - See [docs/architectures/tech-stack.md](docs/admin/architectures/tech-stack.md), [docs/architectures/database.md](docs/admin/architectures/database.md), and [docs/architectures/cloud-infrastructure.md](docs/admin/architectures/cloud-infrastructure.md)
- **Verification Status**:
  - Tests: `PASSED` — `cargo test --workspace` (15/15: 7 lib + 5 `spec_001_scenarios` + 3 `create-user`), `cargo check --target wasm32-unknown-unknown` passed; `cargo clippy --workspace --all-targets -- -D warnings` noted 4 non-blocking lints (logged to `TODO-009`).
  - Security Audit: `PASSED` — OWASP Top 10 (A01–A10) audit passed ([security-audit.md](docs/admin/plan-records/SPEC-001-infra-scaffold/security-audit.md)).

---

## 3. Active In-Flight Work

- **Active Spec**: None (`SPEC-001-infra-scaffold` verified and archived)
- **Active Plan**: None
- **Current Task In-Flight**: None
- **Active Battleground**: None

---

## 4. Leftovers, Blockers & Next Steps

### Immediate Next Steps (For Next Agent / Developer)
- **Immediate Next Steps**: Ready for next feature spec session with Spectre (e.g., graduate `SPEC-002` from [docs/specs/OUT_OF_SCOPE.md](docs/admin/specs/OUT_OF_SCOPE.md)).
- [x] Spawn **Verifier** (`.gb-sdlc/agents/verifier.md`) to verify Scenarios 1-6 and complete the OWASP Top 10 security audit.
- [ ] Graduate `SPEC-002` from [docs/specs/OUT_OF_SCOPE.md](docs/admin/specs/OUT_OF_SCOPE.md).

### Leftovers & Technical Nuances
- **Known Blockers / Warnings**: Ensure `wrangler.toml` placeholder D1 `database_id`s are replaced with provisioned Cloudflare D1 IDs before remote `--env staging` / `--env prod` deployment (`--local` works out of the box).
- **Non-Blocking Follow-Ups**: See `TODO-009` in [TODO.md](./TODO.md) for 4 minor Clippy lints and login timing parity hardening.

---

## 5. Proto-Specs & Deferred Ideas

- For improvements or architectural insights discovered during planning/execution that are worth noting for future specs, see [TODO.md](./TODO.md).

---

## 6. Context & Reference Pointers

- **Master Instructions**: [GEMINI.md](./GEMINI.md) | [AGENTS.md](./AGENTS.md)
- **Architectures**: [docs/architectures/](docs/admin/architectures/)
- **Decisions Log**: [docs/decisions/](docs/admin/decisions/)
- **Researches & Spikes**: [docs/researches/](docs/admin/researches/)
- **Completed Plan Records**: [docs/plan-records/](docs/admin/plan-records/) ([SPEC-001-infra-scaffold](docs/admin/plan-records/SPEC-001-infra-scaffold/00_OVERVIEW.md) | [security-audit.md](docs/admin/plan-records/SPEC-001-infra-scaffold/security-audit.md))



---

<!-- ===== deployment prep (2026-11-20, worktree backend-cloudflare-deploy) ===== -->

## Cloudflare Deployment Prep

- **Done**: `.github/workflows/backend-ci.yml`, `.github/workflows/backend-deploy.yml` (manual only), `docs/deployment.md` (command guideline), and Makefile targets `cf-whoami`, `cf-bootstrap`, `cf-secret`, `cf-dry-run`, `deploy-*` (Makefile is the only command entry point; no loose scripts). Cloudflare plugin installed for Claude Code; wrangler logged in locally.
- **NOT done / nothing deployed**: no D1/R2 created, `database_id`s in both `wrangler.toml` are placeholders, no secrets set, no routes/domains, GitHub secrets not configured.
- **Open**: ingestion scope undecided (`TODO-007`). Rustfmt and clippy fail on existing code (`TODO-006`), so they are advisory in CI.
- **Next**: follow `docs/deployment.md` one-time setup, then trigger `backend-deploy` for staging.
