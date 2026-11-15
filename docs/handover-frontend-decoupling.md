# Handover · Frontend-to-Backend Decoupling & Cloudflare Rust Architecture Bridge

- **Created**: 2026-11-15
- **Purpose**: Pre-spec decoupling audit & handover document for **Spec 01 (AI-Native & Agentic-Native Rust Backend Scaffold on Cloudflare Workers + D1 + R2)**.
- **Scope**: Audits `frontend-client/` (and the planned scope of `frontend-admin/`) to identify every dataset, feature, state transition, and media asset that should move to `backend-client`, `backend-admin`, Cloudflare D1, and Cloudflare R2—while preserving pure UI/animation concerns in the static Next.js frontend.

---

## 1. Executive Summary

`frontend-client/` is currently a **100% static Next.js export** (`output: 'export'`, Next.js `16.4.2`, React `19.2`, Three.js `0.186.1`). All card catalogs, Thai tarot interpretations, source provenance, random shuffling, session state, and reading generation run directly in the browser using bundled JSON files (`daily-cards.json` at `401.9 KB` and `reading-cards.json` at `281.7 KB`) and static assets in `public/` (`~19.3 MB` total).

`frontend-admin/` is currently empty (only `README.md`), while legacy admin/data workflows exist outside this monorepo (Workshop on Supabase; Social Center and Blender/ffmpeg video pipeline on local Mac).

To make the product **production-grade, secure, scalable, and AI/agentic-native**, we must decouple:
1. **Content, provenance, and unapproved drafts** out of browser bundles into **Cloudflare D1** and **backend-admin** knowledge release workflows.
2. **Card draw integrity, 3+1 spread locking, paid credit ledger, reading history, and AI-native/agentic reading generation** into **backend-client** (Cloudflare Workers + D1).
3. **Heavy binary card artwork, scene media, reader audio recordings, and Content Studio video artifacts** into **Cloudflare R2** (separated into public CDN assets vs. private signed-access assets).
4. **Pure UI, WebGL/GSAP/CSS animations, accessibility, and interactive card table UX** kept strictly in `frontend-client/`.

---

## 2. Current Frontend Audit (`frontend-client/` & `frontend-admin/`)

### 2.1 Bundled Static Data & Metadata (Currently in `frontend-client/src/lib/` & `public/`)

| File | Size / Count | Contents Today | Consumers | Issues / Risks in PoC |
|---|---|---|---|---|
| `src/lib/daily-cards.json` | `401.9 KB` (78 cards) | Daily reading text (`theme`, `overview`, `work`, `money`, `love_by_status`, `rabbit_message`, `colors`, `affirmation`), `missing_fields`, `variants`, plus internal provenance (`source.url`, `raw_file`, `message_index`, `message_id`), `approved_for_web: false`, `content_status`. | `src/app/DailyReading.tsx` | **Bloats client bundle** (~402 KB raw JSON), **leaks internal draft/provenance metadata** to end users, and cannot be updated without redeploying frontend. 22 Major Arcana have `complicated: null`. |
| `src/lib/reading-cards.json` | `281.7 KB` (78 cards) | Card `id`, `name`, `keyword`, `art`, short `meanings` (6 love statuses), `details` (long love text), `details_status` (`"claude_draft_published_pending_owner_review"`), `review_status` (`"owner-provided-chat-unreviewed"`), `source_message`, `source_line`, `source_url`. | `src/app/page.tsx`, `src/lib/reading.ts` | **Exposes unreviewed AI/draft labels & ChatGPT source links** directly to client. Couples card catalog (needed for UI gallery) with unapproved reading interpretations. 4 cards have `art: null`. |
| `src/lib/cards.json` | `4.8 KB` (5 cards) | Legacy 5-card pilot subset (`the-fool`, `three-of-wands`, `the-lovers`, `the-hanged-man`, `queen-of-swords`). | **Unused** in `src/` | Legacy PoC artifact; safe to remove from frontend once catalog comes from backend. |
| `public/cards/v3/manifest.json` | `31.6 KB` (74 cards) | Image dimensions (`width`, `height`, `renderedWidth`, `renderedHeight`), `sha256`, `sourceSha256`, `bytes`, `art` URLs. | `src/lib/card-art.ts` | Couples asset versioning to frontend builds; should be served via card catalog API or R2 asset manifest. |
| `public/cards/deck.js` | `8 KB` (78 cards) | Global `window.__mmktDeck` array mapping `id`, `name`, `art` path for the SVG room iframe. | `public/scene/index.html`, `public/scene/controller.js` | Hardcodes `/cards/v3/...` local paths; must be updated or dynamically fed if card art moves to R2 CDN. |

### 2.2 Client-Side Features & Business Logic (`frontend-client/src/app/`)

| Feature / Route | Current Implementation | What Happens Today | Gaps for Production |
|---|---|---|---|
| **Daily Reading (`/`)** (`page.tsx`, `DailyReading.tsx`) | Client-side WebCrypto draw (`randomIndex`), static lookup `getDailyCard(id)`, hardcoded `source` ChatGPT link, 78-card gallery (`#collection`) + detail modal. | User draws 1 random card in browser; all 78 cards' daily content and provenance links are already downloaded in JS. | Need API-backed approved knowledge release (`knowledge_version`), optional server draw/telemetry, lightweight gallery catalog (on-demand detail fetch), and removal of raw ChatGPT source links from public UI. |
| **3-Card Spread (`/reading`)** (`ReadingExperience.tsx`, `reading.ts`) | Client-side `categories`, `situations`, `suggestions`, `positions`; `shuffleDeck()` in browser; `sessionReducer` state machine; fake `locked → reading` unlock button; `createReading()` demo stub (`status: 'demo'`). | User picks category/situation/question, shuffles/picks 3 cards in browser, clicks "ปลดล็อกคำอ่านฉบับเต็ม" (no payment/auth), sees old love meanings or placeholder text for work/money/life. | **Critical security & business gaps**: (1) Shuffled deck & card IDs live in browser memory before reveal; (2) No 4th expansion card (`ใบขยาย`); (3) No auth, credit reservation/commit, or reading history; (4) No real reading engine / AI-native agentic pipeline; (5) Hardcoded topics/situations/suggestions. |
| **Card & Scene Media (`public/`)** | `public/cards/` (`15 MB` incl. legacy `.jpeg`/`.svg`/`.webp` + `v3/` `2.4 MB`), `public/card-scenes/` (`2.9 MB` unused), `public/scene/` (`1.4 MB` incl. `idle-loop.mp4`, `draw-shuffle.mp4`, `character.jpg`, `gsap.min.js`). | Served directly from Vercel static hosting alongside Next.js `out/`. | Legacy unused assets (`public/card-scenes/`, pilot `.jpeg`/`.svg`) bloat deploy; future 78+ card high-res art, missing 4 cards, TTS audio, and Content Studio videos belong in **R2**. |
| **Admin & Content Studio (`frontend-admin/`)** | Empty (`README.md` only). Specs in `docs/context/06-admin-and-content-studio.md` and `docs/reference/social-center.md`. | Workshop (Supabase) and Social Center (localhost FastAPI + SQLite) + Blender/ffmpeg run outside this repo. | Needs `backend-admin` APIs for knowledge curation/approval/releases, reading escalations (`needs_owner`), orders/credits/reconciliation, competitor clip research, Social Center OAuth/sync, and Content Studio durable job orchestration. |

### 2.3 Code-Level Inconsistencies Discovered During Audit

1. **Relationship Status Label Drift**:
   - `/` (`DailyReading.tsx`): `['คนโสด', 'คนแอบชอบ', 'คนคุย', 'ความสัมพันธ์ซับซ้อน', 'คนมีแฟน', 'คนรักเก่า']`
   - `/reading` (`src/lib/reading.ts`): `['คนโสด', 'แอบชอบ', 'คุยกัน', 'ซับซ้อน', 'มีแฟน', 'เลิกกัน']`
   - Notice `คนรักเก่า` vs `เลิกกัน` (flagged in `docs/context/glossary.md` as semantically different!). **Backend D1 schema must use canonical enum keys** (`single`, `crush`, `talking`, `complicated`, `partnered`, `ex_partner`) and serve canonical display labels + topic availability from the API.
2. **Missing Content & Art**:
   - Missing art (`art: null`) in `reading-cards.json` and `deck.js` for 4 cards: `six-of-wands`, `nine-of-wands`, `page-of-pentacles`, `king-of-pentacles`.
   - Missing `complicated` daily love reading (`null`) for all 22 Major Arcana cards in `daily-cards.json`.
   - Note: Missing reader knowledge/art must **never** be silently invented by AI (`docs/context/01-overview.md`, `08-open-decisions.md` ADR-08); backend must represent missing fields explicitly (`null` / `incomplete`) until reader-approved.
3. **Legacy Deadweight in `frontend-client/`**:
   - `src/lib/cards.json` and `public/card-scenes/*.html` (`2.9 MB`) are not referenced by any active route.
   - `public/cards/*.jpeg` (`~9.5 MB`) and `public/cards/*.svg` (`~2.9 MB`) are old pilot files (note: `card-back.svg` IS actively used by `CardReveal.tsx` and `public/scene/controller.js`, while `card-back.webp` is used by `DeckScene.tsx` and `CardVisual.tsx`).
4. **Static Export Constraint (`next.config.ts`)**:
   - `output: 'export'` means `frontend-client` has no Next.js server/API routes. All backend calls will go cross-origin (or via shared domain routing) to Cloudflare Workers (`backend-client`), requiring clean CORS, cookie/token auth strategy, and strict OpenAPI contracts in `docs/api/client/`.

---

## 3. Decoupling Matrix: What Moves Where

### 3.1 Move to `backend-client` (Cloudflare Worker + D1)

| Feature / Data | Current Location | Target in `backend-client` | Notes & Rationale |
|---|---|---|---|
| **Card Catalog & Aspect Ratios** | `reading-cards.json`, `v3/manifest.json` | `GET /v1/cards`, `GET /v1/cards/:id` (D1 `cards` + `card_assets`) | Serves `id`, `name`, `arcana`, `art_url` (R2 CDN), `width`, `height`, `theme`/`keyword`. Strips internal `source_*` and unreviewed statuses. |
| **Daily Reading Content & Draw** | `daily-cards.json`, `page.tsx` (`randomIndex`) | `GET /v1/daily/cards/:id` or `POST /v1/daily/draw` (D1 `daily_readings` + `knowledge_releases`) | Moves 402 KB JSON off client; ensures only `published`/`approved` release version is served; allows daily draw logging/analytics without blocking anonymous free access. |
| **Reading Taxonomy & Prompts** | `reading.ts` (`categories`, `situations`, `suggestions`, `positions`) | `GET /v1/reading/config` (D1 / KV-cached config) | Enables reader/admin to enable topics (e.g., `health`, `life`, `work`, `money`, `love`), canonical situation keys/labels, and prompt suggestions without frontend redeploy. |
| **3+1 Reading Session & Card Draw Lock** | `reading.ts` (`shuffleDeck`, `sessionReducer`, `UNLOCK`) | `POST /v1/readings/sessions`, `POST /v1/readings/sessions/:id/pick`, `POST /v1/readings/sessions/:id/unlock`, `POST /v1/readings/sessions/:id/followup` (D1 `reading_sessions`, `reading_cards`) | **Server-authoritative deck & lock**: browser never receives unrevealed card identities. Supports idempotent retries, prevents double-spend/redraw on refresh, and enforces 4th expansion card (`ใบขยาย`) gating (`baseline` cannot see 4th card). |
| **AI-Native Reading Engine** | `reading.ts` (`createReading()` demo stub) | `backend-client` Reading Engine + Agentic Pipeline (D1 `knowledge_entries`, `reading_outputs`, `reading_traces`) | Replaces demo stub with: (1) scope/intent normalization (`answered` / `clarify` / `needs_owner` / `out_of_scope`); (2) deterministic rule/knowledge lookup (Engine A); (3) constrained AI rewriting with evidence verification (Engine B); (4) reader escalation queue fallback. |
| **User Accounts, Auth & History** | None (`ReadingExperience.tsx` says no history) | Auth endpoints + `GET /v1/me`, `GET /v1/me/readings` (D1 `users`/`profiles`, `sessions`) | Enables saved reading history, credit balance, and async escalation notifications (`needs_owner` resolved by reader). |
| **Orders, Credits & Payment Webhooks** | None (fake `UNLOCK` button) | `GET /v1/packages`, `POST /v1/orders`, `POST /v1/webhooks/payments/:provider`, `GET /v1/me/credits` (D1 `orders`, `payment_events`, `credit_ledger`, `credit_reservations`) | Integer `satang` money, idempotent webhooks with signature verification, atomic D1 batch/transaction credit reservation → commit/release. |

### 3.2 Move to `backend-admin` (Cloudflare Worker + D1 + R2)

| Feature / Data | Current / Legacy Location | Target in `backend-admin` | Notes & Rationale |
|---|---|---|---|
| **Card & Knowledge Curation / Releases** | `daily-cards.json` (`source`, `variants`), `reading-cards.json` (`source_*`), legacy Supabase Workshop | Admin CRUD + Review + Release Publish API (D1 `workshop_cases`, `knowledge_proposals`, `knowledge_entries`, `knowledge_releases`, `approval_events`) | Keeps original vs. edited vs. approved versions separate; enforces `development` vs. `holdout` split (holdout never leaks into retrieval/prompts); publishes immutable `knowledge_release` consumed by `backend-client`. |
| **Reader Escalation Queue (`needs_owner`)** | Not built (proposed in `05-reading-engine.md` & `06-admin-and-content-studio.md`) | `GET /admin/escalations`, `POST /admin/escalations/:id/respond` | Surfaces full trace (question, cards, positions, `knowledge_version`, failure reason) so the real reader can answer and optionally extract a new knowledge proposal. |
| **Agentic Evaluation & A/B Harness** | Proposed in `05-reading-engine.md` | Admin Eval APIs (D1 `eval_runs`, `eval_judgements`) | Blind A vs B comparison UI for the reader, cost/latency/token tracking per `knowledge_version` + `style_version`, holdout evaluation gates. |
| **Orders, Ledger & Reconciliation** | Not built | `GET /admin/orders`, `POST /admin/orders/:id/reconcile`, `POST /admin/credits/adjust` | Operational visibility into payments, stuck reservations, refunds, and ledger audits. |
| **Competitor Research & Social Center** | Legacy local Next.js + FastAPI + SQLite (`docs/reference/social-center.md`) | Admin Social/Competitor APIs (D1 `competitors`, `social_accounts`, `social_posts`, `social_metric_observations`, `social_comment_themes` + R2 raw snapshots) | Multi-user auth, encrypted Meta OAuth tokens, append-only raw snapshots in R2, scheduled backfill/sync via Cloudflare Cron Triggers / Queues. |
| **Content Studio & Mac/Cloud Render Worker API** | Local Blender/ffmpeg/Rhubarb/HyperFrames scripts (`06-admin-and-content-studio.md`) | `POST /admin/content-projects/:id/jobs`, `GET /admin/jobs/:id`, `POST /admin/jobs/:id/retry`, `POST /admin/jobs/:id/approve`, Worker pull/lease endpoints (D1 `content_projects`, `episodes`, `scripts`, `assets`, `generation_jobs`, `job_attempts`, `cost_ledger`) | Durable job state machine (`draft → awaiting_approval → queued → running → awaiting_review → approved → published`), budget reservation before paid AI providers, Mac worker pulls jobs via scoped token (never exposes Mac port). |
| **Audit Log & Governance** | Not built | `GET /admin/audit-events` (D1 `audit_events`) | Tracks actor, action, version, budget approvals, and publish approvals without leaking PII/raw user audio in plain logs. |

### 3.3 Move to Cloudflare R2 (Object Storage)

| Asset Group | Bucket / Visibility | Current Location | Target R2 Layout & Notes |
|---|---|---|---|
| **Published Card Art (`v3/*.webp` + future 4 missing + high-res)** | **Public R2 bucket** (via CDN custom domain, e.g. `assets.maemorkratai...`) | `frontend-client/public/cards/v3/*.webp` (`2.4 MB`) | Immutable content-hashed filenames (`<card-id>-<hash>.webp`). Note: update `deck.js` / `controller.js` or serve `deck` manifest dynamically from `backend-client` so the `/scene/index.html` iframe loads from R2 cleanly (with proper CORS). |
| **Scene Videos & Card Back** | **Public R2 bucket** (or keep small SVGs in frontend `public/`) | `public/scene/idle-loop.mp4`, `draw-shuffle.mp4` (`~1.4 MB`), `card-back.webp` | Keep tiny critical UI SVGs (`card-back.svg`, `gsap.min.js`) local for zero-latency boot if desired; move `.mp4` and large rasters to R2 CDN. |
| **Workshop Audio & Transcripts** | **Private R2 bucket** (short-lived signed URLs after owner/reader auth check) | Legacy Supabase `workshop-audio` / `workshop-review` | Reader voice recordings, ASR transcripts, immutable review snapshots. Never publicly accessible. |
| **Content Studio Assets & Render Artifacts** | **Private R2 bucket** (signed preview URLs for admin) | Owner's local disk | Character `.blend`/`.glb`, voice samples, music/SFX, shot renders, composed MP4s, QA manifests, Social Center append-only JSON snapshots. |

### 3.4 Keep Strictly in `frontend-client/` (Pure UI & Experience)

- **Visual Design & Layout**: Moonlit room aesthetic, palette, typography, responsive layout (`DESIGN.md`, `globals.css`, `reading.css`, `motion.css`).
- **Interactive Animations & 3D**: Three.js `DeckScene.tsx`, GSAP `CardReveal.tsx` flip/halo/stars, `Table.tsx` fan/drag-and-drop/grid toggle, `AnimatedRabbit` & `public/scene/index.html` SVG room, reduced-motion (`prefersStillScene`, `prefers-reduced-motion`) fallbacks.
- **Accessibility**: Focus traps, `inert` background management, keyboard navigation (`ArrowLeft`/`ArrowRight`/`Home`/`End`), `aria-live` status announcements.
- **Presentation-Only State**: Hover waves, card flight coordinates, drag ghost physics, animation phase timing (driven by server session state rather than owning truth).

---

## 4. Handover Bridge for Spec 01: Rust + Cloudflare (Workers, D1, R2) AI-Native & Agentic-Native Backend Scaffold

The owner has confirmed the target platform direction for **Spec 01 (Backend Scaffold for `backend-client` and `backend-admin`)**:
- **Language**: **Rust**
- **Platform**: **Cloudflare**
  - Compute: **Cloudflare Workers** (`backend-client` Worker and `backend-admin` Worker, communicating via shared D1/R2 or internal Service Bindings)
  - Database: **Cloudflare D1** (SQLite-based serverless relational DB — note: supersedes the earlier Postgres/Supabase proposal in `docs/context/03-architecture.md` / `ADR-04` / `ADR-10`; formal ADR update needed in Spec 01)
  - Object Storage: **Cloudflare R2** (public asset CDN bucket + private audio/drafts/studio bucket)
- **Design Philosophy**: **AI-Native & Agentic-Native**

### 4.1 Implications & Key Considerations for Spec 01 (To Probe in Spectre Session)

Because `gb-sdlc` prohibits assuming unconfirmed architecture details, **Spec 01** should run a **Researcher spike** (checking current 2026 Rust `worker` crate / `workers-rs` / D1 / R2 / Cloudflare Queues / AI Gateway / MCP / agentic tooling support on `wasm32-unknown-unknown`) and resolve the following with Spectre:

1. **Workspace & Crate Topology**:
   - Should `backend-client` and `backend-admin` share a Cargo workspace or stay strictly decoupled crates (respecting `gb-sdlc` folder isolation per `backend-client/GEMINI.md` and `backend-admin/GEMINI.md`) with a shared OpenAPI contract in `docs/api/` (and optionally a shared schema/domain package if permitted by monorepo rules)?
   - How will local dev (`wrangler dev`), staging, and production environments be structured?
2. **D1 (SQLite) Design vs. Earlier Postgres Assumptions**:
   - Earlier context docs (`03-architecture.md`, `04-data-and-payments.md`) assumed **Postgres + Supabase RLS**. D1 uses **SQLite** with Worker-enforced authorization (no Postgres RLS) and `batch()` / Durable Objects for strict transactionality / concurrency.
   - How should `credit_ledger` + `credit_reservations` and reading session locks guarantee zero double-spend under concurrent requests in D1 / Workers (e.g., D1 `batch` conditional updates, idempotency keys, or a Cloudflare Durable Object per user/session)?
3. **AI-Native & Agentic-Native Architecture in Rust Workers**:
   - What does **AI-native & agentic-native** mean concretely for `backend-client` and `backend-admin`? Candidates to decide in Spec 01:
     - **Structured Tool/Agent Interfaces**: Exposing capabilities as typed, schema-validated tools (e.g., MCP server endpoints or internal agent tool registries) with machine-readable OpenAPI + JSON Schema contracts.
     - **Reading Engine Agent Pipeline**: Multi-step agentic workflow in Rust (Intent/Scope Classifier → Deterministic Knowledge Retriever from D1 → Constrained Rewriter via LLM/Cloudflare AI Gateway → Evidence/Hallucination Verifier → Escalation Router to `needs_owner`), with full `reading_traces` observability.
     - **Admin Content & Knowledge Agents**: Agentic ingestion/extraction of reader audio/text into `knowledge_proposals` (never auto-published without human sign-off), automated competitor clip idea extraction, and agent-assisted Content Studio storyboard/script orchestration with budget guardrails.
     - **Agentic Guardrails**: Strict output schema verification, evidence-ID grounding, prompt-injection resilience on user questions, and hard spend/token caps before calling any external LLM.
4. **R2 Integration & Frontend Scene Iframe**:
   - Public R2 bucket URL + CORS configuration so `frontend-client` (`DeckScene.tsx` Three.js `TextureLoader`, `CardFace.tsx`, and `/scene/index.html` iframe) can load card textures without tainted canvas or CORS errors.
   - Presigned/Worker-proxied URLs for private R2 objects (workshop audio, content studio video previews).
5. **Auth & Security Boundary (OWASP #1)**:
   - Auth mechanism for customers (`frontend-client` static export -> `backend-client` Worker) vs. admin/reader (`frontend-admin` -> `backend-admin` Worker) vs. Mac local automation worker (scoped API token / mTLS / HMAC).
6. **API Contract First (`docs/api/`)**:
   - Before writing frontend integration or backend handlers, agree on `docs/api/client/openapi.yaml` and `docs/api/admin/openapi.yaml` and record ADRs in `docs/decisions/` (updating ADR-04, ADR-09, ADR-10 for Rust + Cloudflare Workers + D1 + R2).

---

## 5. Recommended Next Steps for Spec 01 Session

1. **Confirm Monorepo vs. Service-Level Spec Scope**: Decide whether Spec 01 scaffolds both `backend-client/` and `backend-admin/` together (or splits into `SPEC-001` in each service's `docs/specs/`).
2. **Run Researcher Spike (`docs/researches/`)**: Verify current 2026 Rust Cloudflare Workers ecosystem (`worker` crate, D1 bindings, R2 bindings, Queues/Workflows/Durable Objects, OpenAPI generation in Rust/Wasm, MCP/agentic crates compatible with `wasm32-unknown-unknown`).
3. **Run Spectre 5-Phase Spec Loop**:
   - Lock decisions (`D01.md`+ and `docs/decisions/NNNN-*.md`) for Rust Workers architecture, D1 schema/migrations strategy, R2 bucket topology, Auth, and AI-native/agentic patterns.
   - Draft & audit Spec 01 for the backend scaffold (`health`, `config`, `D1` migrations, `R2` bindings, OpenAPI contract skeleton, AI/agentic runtime abstraction, CI/test harness).
4. **Keep `frontend-client` Healthy**: Clean up unused PoC files (`src/lib/cards.json`, `public/card-scenes/`) in a separate frontend PR once R2/API contracts are ready.
