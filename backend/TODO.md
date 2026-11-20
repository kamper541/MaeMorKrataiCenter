# Backend TODO (`TODO.md`)

> Merged from the former `backend-client` and `backend-admin` repos. Each part below keeps its own IDs (TODO IDs may collide across parts); docs live in `docs/client/` and `docs/admin/`.


---

<!-- ===== client ===== -->

# Project Roadmap & Proto-Spec Incubator (`TODO.md`)

> **The Proto-Spec Concept**:
> In `gb-sdlc`, task files strictly require full code and forbid in-code `// TODO: implement later` placeholders.
> However, during planning, execution, and code review, developers and agents frequently uncover insights, edge cases, refactorings, or architectural enhancements that are **worth not doing right now, but valuable to add later**.
>
> Rather than letting these ideas get lost or polluting active tasks, capture them here as **Proto-Specs** (Candidate Specs).
> When the project is ready for its next cycle, **Spectre** reviews this file and graduates high-value proto-specs into formal specifications (`docs/specs/<spec-name>.md`).

---

## 1. Proto-Spec Register

| ID | Title | Priority | Status | Discovered During | Graduated Spec |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `TODO-001` | Initial Feature Enhancements | Low | PROPOSED | Project Initialization | - |
| `TODO-002` | AI-Native LLM Chat, RAG Database & Agentic Tools | High | PROPOSED | SPEC-001 Scoping | - |
| `TODO-003` | 3+1 Reading Engine, Server Shuffle/Lock & Escalation Flow | High | PROPOSED | SPEC-001 Scoping | - |
| `TODO-004` | Customer Orders, Credits Ledger & Payment Webhooks | Medium | PROPOSED | SPEC-001 Scoping | - |
| `TODO-005` | Customer Accounts, Auth & Saved Reading History | Medium | PROPOSED | SPEC-001 Scoping | - |
| `TODO-006` | Infra Hardening: Clippy Cast Cleanup, HTTPS URL Guard & Structured Audit Logs | Low | PROPOSED | SPEC-001 Security Audit | - |

---

## 2. Active Proto-Specs

### [TODO-001] Initial Feature Enhancements
- **Status**: `PROPOSED`
- **Discovered On**: 2026-10-31
- **Discovered By / During**: Project Scaffolding
- **Priority**: `P3 (Low)`
- **Why Not Now?**: Scaffolding is complete; core features should be specified first before secondary enhancements.
- **Context & Problem Statement**:
  Placeholder for future improvements, optimizations, or refactoring ideas discovered while developing features.
- **Proto-Spec Sketch (Suggested Approach)**:
  - Document ideas, edge cases, or optimizations here.
- **Graduation / Promotion Criteria**:
  - When the team identifies this enhancement as valuable for an upcoming milestone, Spectre will promote it to a spec in `docs/specs/`.

### [TODO-002] AI-Native LLM Chat, RAG Database & Agentic Tools
- **Status**: `PROPOSED`
- **Discovered On**: 2026-11-15
- **Discovered By / During**: SPEC-001 Scoping
- **Priority**: `P1 (High)`
- **Why Not Now?**: Deferred from `SPEC-001` infrastructure scaffold; zero public LLM endpoints in Phase 1.
- **Context & Problem Statement**:
  Future integration of LLM chat, RAG retrieval database, and agentic tool-calling workflows for tarot reading assistance—with strict evidence verification and spend/rate caps.
- **Proto-Spec Sketch (Suggested Approach)**:
  - Cloudflare AI Gateway + RAG index + typed tool contracts + output evidence verifier + fallback to `needs_owner`.
- **Graduation / Promotion Criteria**:
  - After `SPEC-002` (Cards/Daily D1+R2 slice) and approved knowledge releases exist.

### [TODO-003] 3+1 Reading Engine, Server Shuffle/Lock & Escalation Flow
- **Status**: `PROPOSED`
- **Discovered On**: 2026-11-15
- **Discovered By / During**: SPEC-001 Scoping (from `docs/context/05` & `handover-frontend-decoupling.md`)
- **Priority**: `P1 (High)`
- **Why Not Now?**: Requires `SPEC-001` infra and `SPEC-002` card catalog in D1 first.
- **Context & Problem Statement**:
  Replace `frontend-client/src/lib/reading.ts` client-side shuffle and demo `createReading()` with server-authoritative 3+1 card draw locking (4th expansion card hidden from baseline) and Engine A/B reading pipeline.
- **Proto-Spec Sketch (Suggested Approach)**:
  - Session state machine in D1 + deterministic lookup + Turnstile/auth protection.
- **Graduation / Promotion Criteria**:
  - After `SPEC-002` is deployed.

### [TODO-004] Customer Orders, Credits Ledger & Payment Webhooks
- **Status**: `PROPOSED`
- **Discovered On**: 2026-11-15
- **Discovered By / During**: SPEC-001 Scoping (from `docs/context/04`)
- **Priority**: `P2 (Medium)`
- **Why Not Now?**: Phase 1 has no customer accounts or payments.
- **Context & Problem Statement**:
  Credit packs in integer satang, signature-verified payment webhooks, idempotent order processing, and atomic D1 credit reservation/commit/release.
- **Proto-Spec Sketch (Suggested Approach)**:
  - D1 `batch()` conditional ledger updates / Durable Objects per user account.
- **Graduation / Promotion Criteria**:
  - Paired with customer accounts (`TODO-005`) before paid beta.

### [TODO-005] Customer Accounts, Auth & Saved Reading History
- **Status**: `PROPOSED`
- **Discovered On**: 2026-11-15
- **Discovered By / During**: SPEC-001 Scoping
- **Priority**: `P2 (Medium)`
- **Why Not Now?**: Phase 1 is explicitly account-less (`D04` public-connection model).
- **Context & Problem Statement**:
  Enable customer login, saved reading history, and async `needs_owner` resolution notifications using the `user` role already defined in `D03`.
- **Proto-Spec Sketch (Suggested Approach)**:
  - Leverage `D03` JWKS verification scaffold + `users` (`role = 'user'`) schema.
- **Graduation / Promotion Criteria**:
  - Required before paid beta (`TODO-004`).

### [TODO-006] Infra Hardening: Clippy Cast Cleanup, HTTPS URL Guard & Structured Audit Logs
- **Status**: `PROPOSED`
- **Discovered On**: 2026-11-16
- **Discovered By / During**: Verifier & Security Auditor (`SPEC-001-infra-scaffold`)
- **Priority**: `P3 (Low)`
- **Why Not Now?**: Non-blocking observations from `SPEC-001` verification; all core acceptance criteria and OWASP A01–A10 checks passed.
- **Context & Problem Statement**:
  Minor code hygiene and defense-in-depth enhancements identified during `SPEC-001` audit.
- **Proto-Spec Sketch (Suggested Approach)**:
  - Remove unnecessary `as u64` cast in `src/adapters/worker_env.rs:141` (`clippy::unnecessary_cast`).
  - Enforce `https://` scheme validation on `TURNSTILE_VERIFY_URL` and `JWKS_URL` in `WorkerClientAdapter::load_config` for `staging` and `prod`.
  - Emit structured, PII/token-redacted security audit logs for `401`/`403`/`429`/`503` responses.
- **Graduation / Promotion Criteria**:
  - Fold into `SPEC-002` or a dedicated hardening pass.

---

## 3. Graduated & Archived Proto-Specs

*(No proto-specs graduated yet.)*


---

<!-- ===== admin ===== -->

# Project Roadmap & Proto-Spec Incubator (`TODO.md`)

> **The Proto-Spec Concept**:
> In `gb-sdlc`, task files strictly require full code and forbid in-code `// TODO: implement later` placeholders.
> However, during planning, execution, and code review, developers and agents frequently uncover insights, edge cases, refactorings, or architectural enhancements that are **worth not doing right now, but valuable to add later**.
>
> Rather than letting these ideas get lost or polluting active tasks, capture them here as **Proto-Specs** (Candidate Specs).
> When the project is ready for its next cycle, **Spectre** reviews this file and graduates high-value proto-specs into formal specifications (`docs/specs/<spec-name>.md`).

---

## 1. Proto-Spec Register

| ID | Title | Priority | Status | Discovered During | Graduated Spec |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `TODO-001` | Initial Feature Enhancements | Low | PROPOSED | Project Initialization | - |
| `TODO-002` | AI-Native LLM Chat, RAG Database & Agentic Tools | High | PROPOSED | SPEC-001 Scoping | - |
| `TODO-003` | Knowledge Release Curation, Reading Escalations & A/B Eval Harness | High | PROPOSED | SPEC-001 Scoping | - |
| `TODO-004` | Orders, Credit Ledger & Payment Reconciliation Admin | Medium | PROPOSED | SPEC-001 Scoping | - |
| `TODO-005` | Content Studio Job Queue (Mac Pull Worker) & Social Center | Medium | PROPOSED | SPEC-001 Scoping | - |
| `TODO-006` | Cloudflare Access Zero Trust Migration & Key Rotation Automation | Medium | PROPOSED | SPEC-001 Auth Design | - |
| `TODO-007` | Argon2id Password Hashing Upgrade (Workers Paid Tier) | Low | PROPOSED | SPEC-001 Spectre Audit | - |
| `TODO-008` | Scheduled D1 Session & Retiring-JWK Cleanup Cron | Low | PROPOSED | SPEC-001 Planning | - |
| `TODO-009` | Code Quality & Auth Hardening Follow-Ups (Clippy, Timing Parity, `cargo-audit`) | Low | PROPOSED | SPEC-001 Security Audit | - |

---

## 2. Active Proto-Specs

### [TODO-001] Initial Feature Enhancements
- **Status**: `PROPOSED`
- **Discovered On**: 2026-10-31
- **Discovered By / During**: Project Scaffolding
- **Priority**: `P3 (Low)`
- **Why Not Now?**: Scaffolding is complete; core features should be specified first before secondary enhancements.
- **Context & Problem Statement**:
  Placeholder for future improvements, optimizations, or refactoring ideas discovered while developing features.
- **Proto-Spec Sketch (Suggested Approach)**:
  - Document ideas, edge cases, or optimizations here.
- **Graduation / Promotion Criteria**:
  - When the team identifies this enhancement as valuable for an upcoming milestone, Spectre will promote it to a spec in `docs/specs/`.

### [TODO-002] AI-Native LLM Chat, RAG Database & Agentic Tools
- **Status**: `PROPOSED`
- **Discovered On**: 2026-11-15
- **Discovered By / During**: SPEC-001 Scoping
- **Priority**: `P1 (High)`
- **Why Not Now?**: Deferred from `SPEC-001` infrastructure scaffold so core Workers, D1, R2, and Auth are built and audited first.
- **Context & Problem Statement**:
  The backend must support future LLM chat integration, a RAG knowledge base, and agentic AI tool-calling workflows (e.g., knowledge proposal extraction, competitor clip analysis, evidence-grounded reading assistance).
- **Proto-Spec Sketch (Suggested Approach)**:
  - Define Cloudflare Vectorize / RAG retrieval index + Cloudflare AI Gateway adapter, MCP/typed tool schemas, prompt-injection defense, and per-job/per-day budget caps.
- **Graduation / Promotion Criteria**:
  - After `SPEC-002` (Cards/Daily D1+R2 slice) is live and knowledge curation begins.

### [TODO-003] Knowledge Release Curation, Reading Escalations & A/B Eval Harness
- **Status**: `PROPOSED`
- **Discovered On**: 2026-11-15
- **Discovered By / During**: SPEC-001 Scoping (from `docs/context/05` & `06`)
- **Priority**: `P1 (High)`
- **Why Not Now?**: Depends on `SPEC-001` scaffold and `SPEC-002` card schema.
- **Context & Problem Statement**:
  Reader needs to curate original vs. edited knowledge, maintain strict `development` vs. `holdout` splits, publish immutable `knowledge_releases`, resolve `needs_owner` reading escalations, and run blind A/B engine evaluations.
- **Proto-Spec Sketch (Suggested Approach)**:
  - D1 tables (`knowledge_entries`, `knowledge_releases`, `approval_events`, `eval_runs`) and R2 private audio/transcript storage.
- **Graduation / Promotion Criteria**:
  - Next milestone after `SPEC-002`.

### [TODO-004] Orders, Credit Ledger & Payment Reconciliation Admin
- **Status**: `PROPOSED`
- **Discovered On**: 2026-11-15
- **Discovered By / During**: SPEC-001 Scoping (from `docs/context/04`)
- **Priority**: `P2 (Medium)`
- **Why Not Now?**: Phase 1 has no customer accounts or paid checkout yet.
- **Context & Problem Statement**:
  Admin visibility into satang-denominated orders, provider payment events, credit ledger adjustments, refunds, and stuck-reservation reconciliation.
- **Proto-Spec Sketch (Suggested Approach)**:
  - Admin read/reconcile endpoints backed by D1 transactional ledger tables.
- **Graduation / Promotion Criteria**:
  - Paired with `backend-client` paid beta milestone.

### [TODO-005] Content Studio Job Queue (Mac Pull Worker) & Social Center
- **Status**: `PROPOSED`
- **Discovered On**: 2026-11-15
- **Discovered By / During**: SPEC-001 Scoping (from `docs/context/06` & `social-center.md`)
- **Priority**: `P2 (Medium)`
- **Why Not Now?**: Requires `SPEC-001` R2 private bucket and admin auth foundation first.
- **Context & Problem Statement**:
  Durable job state machine for Blender/ffmpeg video pipelines (pulled securely by owner's Mac via scoped token) plus Social Center Meta OAuth/analytics and competitor research.
- **Proto-Spec Sketch (Suggested Approach)**:
  - Cloudflare Queues/D1 lease-based jobs + R2 artifact manifests + encrypted Meta OAuth token storage.
- **Graduation / Promotion Criteria**:
  - When Content Studio or Social Center UI work begins in `frontend-admin/`.

### [TODO-006] Cloudflare Access Zero Trust Migration & Key Rotation Automation
- **Status**: `PROPOSED`
- **Discovered On**: 2026-11-15
- **Discovered By / During**: SPEC-001 Auth Design (`D03`)
- **Priority**: `P2 (Medium)`
- **Why Not Now?**: Phase 1 launches with Email + Password (`users`, `sessions`, `jwks_keys`) for ~3 admin/owner users.
- **Context & Problem Statement**:
  Replace Phase 1 email+password login with Cloudflare Access Zero Trust JWT verification and/or automate scheduled JWKS key rotation.
- **Proto-Spec Sketch (Suggested Approach)**:
  - Switch JWKS verifier config to Cloudflare Access `certs` URL + `Cf-Access-Jwt-Assertion` header validation mapped to `users.email`/`role`.
- **Graduation / Promotion Criteria**:
  - When Cloudflare Access Zero Trust is provisioned for admin team.

### [TODO-007] Argon2id Password Hashing Upgrade (Workers Paid Tier)
- **Status**: `PROPOSED`
- **Discovered On**: 2026-11-15
- **Discovered By / During**: SPEC-001 Spectre Audit (`D03`)
- **Priority**: `P3 (Low)`
- **Why Not Now?**: Phase 1 deploys on Cloudflare Workers Free tier (10 ms CPU limit), which cannot execute memory-hard Argon2id; WebCrypto PBKDF2-HMAC-SHA256 (`100,000` iterations) + Wrangler Secret `PASSWORD_PEPPER` fits Free tier.
- **Context & Problem Statement**:
  If the project upgrades to Workers Paid plan before migrating admin auth to Cloudflare Access Zero Trust (`TODO-006`), upgrade password hashes lazily on login from `$pbkdf2-sha256-peppered$...` to tuned Argon2id (`16 MiB`, `t=2`, `p=1`).
- **Proto-Spec Sketch (Suggested Approach)**:
  - Add Argon2id verifier/hasher behind versioned hash prefix dispatch.
- **Graduation / Promotion Criteria**:
  - Only needed if Paid Workers plan is active while email+password auth is still primary.

### [TODO-008] Scheduled D1 Session & Retiring-JWK Cleanup Cron
- **Status**: `PROPOSED`
- **Discovered On**: 2026-11-16
- **Discovered By / During**: SPEC-001 Planning
- **Priority**: `P3 (Low)`
- **Why Not Now?**: `SPEC-001` focuses on core auth, rotation, and replay revocation; expired rows remain inert via `expires_at` checks.
- **Context & Problem Statement**:
  Expired rows in `sessions` and old `retiring` keys in `jwks_keys` accumulate in D1 over time.
- **Proto-Spec Sketch (Suggested Approach)**:
  - Add a Cloudflare Worker `[triggers]` `crons` handler (`#[event(scheduled)]`) deleting expired `sessions` beyond retention window and transitioning expired `retiring` keys to `revoked`.
- **Graduation / Promotion Criteria**:
  - When production auth traffic grows or before `TODO-006` cutover.

### [TODO-009] Code Quality & Auth Hardening Follow-Ups (Clippy, Timing Parity, `cargo-audit`)
- **Status**: `PROPOSED`
- **Discovered On**: 2026-11-16
- **Discovered By / During**: SPEC-001 Verifier & Security Audit
- **Priority**: `P3 (Low)`
- **Why Not Now?**: Non-blocking items discovered during final verification of `SPEC-001`; all 15 tests and `wasm32-unknown-unknown` check pass.
- **Context & Problem Statement**:
  1. `cargo clippy --workspace --all-targets -- -D warnings` reports 4 minor lints (`dead_code` on `DbSessionRow.token_hash` in `src/adapters/worker_env.rs:122`, `clippy::unnecessary_cast` at `src/adapters/worker_env.rs:514`, `clippy::cloned_ref_to_slice_refs` at `src/auth/jwt.rs:287`, and `clippy::let_unit_value` at `src/auth/password.rs:156`).
  2. `handle_login` in `src/routes/auth.rs` returns early when an email is not found before running PBKDF2, creating a minor timing difference between unknown emails and wrong passwords.
  3. `cargo-audit` is not yet wired into CI to scan `Cargo.lock` for advisories.
- **Proto-Spec Sketch (Suggested Approach)**:
  - Clean up the 4 Clippy lints and enforce `cargo clippy --workspace --all-targets -- -D warnings` + `cargo audit` in CI.
  - Execute a dummy PBKDF2 verification on unknown email in `handle_login` to equalize login response timing.
- **Graduation / Promotion Criteria**:
  - Bundle into the next maintenance or `SPEC-002` prep PR.

---

## 3. Graduated & Archived Proto-Specs

*(No proto-specs graduated yet.)*



---

<!-- ===== deployment ===== -->

### [TODO-007] Cloudflare Ingestion Pipeline (needs spec)
- **Status**: `PROPOSED`
- **Discovered On**: 2026-11-20
- **Discovered By / During**: Cloudflare deployment prep (worktree `backend-cloudflare-deploy`)
- **Priority**: `P2 (Medium)`
- **Why Not Now?**: No ingestion pipeline exists in the repo and its shape is an architecture decision (Spectre zero-assumption). Scope is not yet defined.
- **Context & Problem Statement**:
  Deployment prep was requested "with ingestion". Candidates: R2 `PRIVATE_STORAGE` upload event -> Queue -> consumer Worker; or RAG/knowledge-release ingestion (see TODO-002).
- **Graduation / Promotion Criteria**:
  Developer confirms ingestion scope; Spectre drafts a spec and logs the decision.
