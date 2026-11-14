# docs

The shared space between frontend and backend. Read `context/` first, then agree contracts in `api/`.

| Path | Contents |
|---|---|
| [`context/`](context) | Project knowledge, translated from the old repo: overview, current state, architecture, data and payments, reading engine, admin/Content Studio, roadmap, open decisions, glossary |
| [`reference/`](reference) | Raw reference material: legacy workshop SQL migrations, the never-migrated schema proposal (DBML), Social Center summary |
| `api/client/` | API contract for the customer website. Use OpenAPI (`openapi.yaml`). |
| `api/admin/` | API contract for the admin dashboard and content automation. |
| `decisions/` | Decision records, named `NNNN-short-title.md` (for example `0001-auth.md`). |

Start with [`context/01-overview.md`](context/01-overview.md).

## Workflow

1. Need a new or changed endpoint? Open a branch and a pull request that edits `api/`.
2. The other side reviews and approves before anyone writes code.
3. Once merged, the contract is final: frontend mocks against it, backend implements it.

## Source of the context docs

Translated and condensed on 2026-11-14 from the old repo `riwjung/MaeMorKratai`: `docs/consult/*` (2026-11-05), `backend/READING_ENGINE_PLAN.md` (2026-10-26), `backend/db/maemorkratai.dbml` (2026-10-17), `services/social-center/README.md`. Dated status notes, local machine paths and private project IDs were removed. Real reader answers, audio and holdout data are not in this repo and must never be added.
