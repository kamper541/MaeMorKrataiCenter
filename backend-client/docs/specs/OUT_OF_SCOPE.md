# Out of Scope Register

This document captures items, secondary features, and unaddressed edge cases deferred during specification discussions with **Spectre** to avoid context bloat, scope creep, and token degradation.

It serves as seed context for future Spectre spec sessions or for graduating into candidate Proto-Specs in `TODO.md`.

---

## Deferred Items Register

### [2026-11-15] Deferred from Spec: SPEC-001 (Backend Infrastructure Scaffold)
- **Item**: First Live Domain Slice — Public Cards Catalog, Daily Readings & R2 Card Art Serving (`SPEC-002`)
  - **Reason for Deferral**: Deliberately split from `SPEC-001` so the foundational Rust Worker + D1 + R2 + Security baseline is verified cleanly first.
  - **Captured Context**: Public read endpoints (`GET /v1/cards`, `GET /v1/daily/...`) backed by shared D1 + `PUBLIC_ASSETS` R2 CDN, Turnstile-protected draw routes, canonical relationship-status enum keys, and removal of internal `source_*` leaks.
  - **Recommended Action**: Graduate immediately as **`SPEC-002`** after `SPEC-001` completes.

- **Item**: AI LLM Chat, RAG Database & Agentic Tool Calling
  - **Reason for Deferral**: Non-core to infrastructure scaffold; requires separate RAG/vector store, prompt guardrails, and spend caps design.
  - **Captured Context**: Future LLM chat integration + RAG database + agentic tool-calling layer for customer reading interaction and evidence-grounded rewrites. Zero public LLM endpoints in `SPEC-001`.
  - **Recommended Action**: Logged as Proto-Spec `TODO-002` in `TODO.md`.

- **Item**: Customer Accounts, Login, Saved History & Cloudflare Access / OAuth
  - **Reason for Deferral**: Phase 1 customer surface has no accounts (`backend-client` handles public connections under `D04` defenses).
  - **Captured Context**: Reusable JWKS JWT verification module is scaffolded cleanly for future authenticated customer/session phases.
  - **Recommended Action**: Logged as Proto-Spec `TODO-006` in `TODO.md`.

- **Item**: 3+1 Spread Session Engine, Server Shuffle/Lock & Payments/Credit Ledger
  - **Reason for Deferral**: Subsequent roadmap milestones (`docs/context/07-roadmap.md`).
  - **Recommended Action**: Logged as Proto-Specs `TODO-003` and `TODO-004` in `TODO.md`.
