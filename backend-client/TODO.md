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

---

## 3. Graduated & Archived Proto-Specs

*(No proto-specs graduated yet.)*
