# Out of Scope Register

This document captures items, secondary features, and unaddressed edge cases deferred during specification discussions with **Spectre** to avoid context bloat, scope creep, and token degradation.

It serves as seed context for future Spectre spec sessions or for graduating into candidate Proto-Specs in `TODO.md`.

---

## Deferred Items Register

### [2026-11-15] Deferred from Spec: SPEC-001 (Backend Infrastructure Scaffold)
- **Item**: First Live Domain Slice — Cards Catalog, Daily Readings & Admin Card CRUD (`SPEC-002`)
  - **Reason for Deferral**: Deliberately split from `SPEC-001` so the foundational Rust Worker + D1 + R2 + Auth + Security scaffold is verified cleanly first.
  - **Captured Context**: Migrate `daily-cards.json` (78 cards), `reading-cards.json` (78 cards), and `v3/manifest.json` (74 cards) into D1 tables, publish `v3/*.webp` into `PUBLIC_ASSETS` R2, and expose admin card update endpoints (e.g. updating card descriptions) with RBAC.
  - **Recommended Action**: Graduate immediately as **`SPEC-002`** after `SPEC-001` completes.
  - **Status (2026-11-21)**: `GRADUATED` into [docs/specs/SPEC-002-master-data-to-backend.md](../../specs/SPEC-002-master-data-to-backend.md) (data only; R2 card art and server-side daily draw stay deferred).

- **Item**: AI LLM Chat, RAG Database & Agentic Tool Calling
  - **Reason for Deferral**: Non-core to infrastructure scaffold; requires separate RAG/vector store, prompt guardrails, and spend caps design.
  - **Captured Context**: Future LLM chat integration + RAG database + agentic tool-calling layer for reading assistance and content workflows.
  - **Recommended Action**: Logged as Proto-Spec `TODO-002` in `TODO.md`.

- **Item**: Cloudflare Access Zero Trust Migration
  - **Reason for Deferral**: Phase 1 launches quickly with Email + Password (`users`, `sessions`, `jwks_keys`) for ~3 admin/owner users; Zero Trust cutover happens later.
  - **Captured Context**: `D03` ensures JWKS-compatible JWT verification and `users` role mapping so switching issuer/JWKS to Cloudflare Access requires minimal changes.
  - **Recommended Action**: Logged as Proto-Spec `TODO-006` in `TODO.md`.

- **Item**: Self-Service Admin/User Registration, Public Customer Accounts & Argon2id Upgrade
  - **Reason for Deferral**: Phase 1 has only ~3 admin/owner accounts (provisioned via `make create-user`), no customer login on `frontend-client`, and deploys on Cloudflare Workers Free tier (10 ms CPU cap prevents Argon2id).
  - **Captured Context**: `users.role` enum already defines `owner > admin > user` in schema, and `users.password_hash` uses a versioned hash prefix so upgrading from WebCrypto PBKDF2-100k+pepper to Argon2id on Paid tier (or migrating to Cloudflare Access Zero Trust) requires no breaking schema rewrite.
  - **Recommended Action**: Logged as `TODO-007` (Argon2id upgrade if Paid tier precedes Zero Trust) and `TODO-006` (Cloudflare Access Zero Trust) in `TODO.md`.

- **Item**: Reading Engine (3+1 Spread & `needs_owner` Escalations), Payments/Credit Ledger, Social Center & Content Studio Pipeline
  - **Reason for Deferral**: Full domain features belonging to subsequent roadmap milestones (`docs/context/07-roadmap.md`).
  - **Recommended Action**: Logged as Proto-Specs `TODO-003`, `TODO-004`, and `TODO-005` in `TODO.md`.
