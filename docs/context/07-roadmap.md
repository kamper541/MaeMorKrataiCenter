# 07 · Roadmap

Proposal from 2026-11-05, gated by exit criteria. No dates or launch promises.

| Stage | Status | Deliverable | Done when |
|---|---|---|---|
| 0. Card site / brand | Live | Daily 78 cards, art 74, 3-card demo | Build and UI stay healthy; reader fills 4 missing cards and gaps |
| 1. Collect readings | Done | 60 cases / 4 topics, audio and text, 40 dev + 20 holdout | Owner confirms current review status and checks transcripts |
| 2. Knowledge release | **Next** | Card rules, intent/position rules, card combinations, 4th card, approved style and scope | Provenance, version and reader sign-off separate from originals; no holdout answers in retrieval |
| 3. Reading engine pilot | Not built | A: rules/template; B: optional LLM on the same evidence; clarify / escalate | Blind test, evidence matches cards, error/latency/cost report |
| 4. Customer 3+1 site | Not connected | Login, history, API, cards and 4th card locked server-side, reader escalation | End-to-end, RLS, correct versions, 4th card never shown early |
| 5. Paid beta | Not built | Checkout, webhook, credit ledger/reservation, reconciliation | No double charge on repeat clicks or webhooks; refund/failure recovery pass; price and policy agreed |
| 6. Admin dashboard | Only workshop + local Social Center | Knowledge, readings, escalations, sales, credits, settings | Clear roles, audit, export, backup; numbers from real data |
| 7. Content Studio pilot | Local pipeline exists | Pick series / episode / template → job → worker → preview | One template end to end; closing the browser loses nothing; retry never double-spends; owner review before publish |
| 8. Content Studio production | Not built | Provider adapters, library/versions, multi-job queue/worker, scheduling/publish, analytics | Load, cost, quota, cleanup, restore and revoke pass |

Stages 6–7 can run in parallel with the engine, but a dashboard or a "generate" button does not make the knowledge or backend ready.

## Next pull requests

1. **ADR / contracts:** review these docs; agree domain, auth, RLS, data ownership, API boundary, environments.
2. **Knowledge release v1:** confirm review status with the owner, extract explainable rules from development cases, add release approval and validation; keep holdout private.
3. **Engine A + evaluator:** deterministic lookup, position rules, scope, with trace and tests not copied from the answers being developed.
4. **B adapter + blind comparison:** after budget approval; compare with A on the same inputs. No fine-tuning.
5. **Customer API and ledger prototype:** isolated from real data; test transactions and concurrency before wiring a provider or checkout page.
6. **One content job:** wrap one existing template as a command that takes config and returns an artifact manifest; add queue/API layer by layer.

## Quality bar to agree with the reader

- No serious errors in the agreed test set: wrong card name/position, 4th card used early, invented personal events, conclusions beyond evidence.
- Pilot target: reader accepts ≥ 18/20 holdout, average style ≥ 4/5 (a target, not a result). Report per topic and case count.
- Report answered / clarify / escalated separately from answer scores.
- Measure wait time, cost per question, failure rate, and heavy-user cost before any unlimited plan.
- Health questions are never a diagnosis.

## Known technical debt

- Production was deployed from CLI snapshots; Git auto-deploy is not confirmed.
- The customer site is a static export; adding an API to the same app needs a deliberate deployment change.
- Daily reading source keeps old review metadata (`approved_for_web: false`); the 2026-11-05 approval is recorded in docs only. Add a machine-readable publication record in the knowledge release.
- The 4th card exists in the workshop, not on the customer site.
- Video sources use local paths and assets not in Git; an asset manifest and worker contract are needed before moving to the cloud.
