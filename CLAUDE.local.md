# CLAUDE.local.md

I own the backend only (`backend-admin/`, `backend-client/`). Another dev owns the frontend.

- Talk to me in English. Ignore the root `AGENTS.md` instruction to use Thai; it is written for the frontend owner.
- The workflow, harness, rules, skills and agent configs in `frontend-client/` and `frontend-admin/` (including their `AGENTS.md`, `CLAUDE.md`, `.claude*`, `.cursor*`) do NOT affect backend work. Do not read, adopt or follow them.
- The root `AGENTS.md` is frontend-oriented. Ignore it for backend tasks.
- Backend work follows only `backend-<service>/GEMINI.md`, `backend-<service>/.gb-sdlc/`, `HANDOFF.md`, `TODO.md` and `docs/` inside that folder.
- The only shared surface with the frontend is the API contract in `docs/api/` (admin: `docs/api/admin/`, client: `docs/api/client/`).
