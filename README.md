# MaeMorKratai Center

Monorepo for MaeMorKratai (แม่หมอกระต่าย), a Thai tarot reading website and its admin tools.

| Folder | Owner | Purpose |
|---|---|---|
| [`frontend-client/`](frontend-client) | Frontend | Customer tarot website. Pure UI, no backend yet. |
| [`frontend-admin/`](frontend-admin) | Frontend | Admin dashboard: site overview, competitor clip research, content automation control. |
| [`docs/`](docs) | Both | API contracts (OpenAPI) and decision records. The shared meeting point. |
| [`backend-client/`](backend-client) | Backend | API for the customer website. |
| [`backend-admin/`](backend-admin) | Backend | API for the admin dashboard. |

## Rules

- Frontend and backend talk only through the API contracts in [`docs/api/`](docs/api). Never import code across folders.
- Work on your own branch and open a pull request into `main`.
- To change an API contract, edit `docs/api/` in its own pull request and get approval from the other side before writing code.
- Never commit secrets. `.env*` files are ignored; commit a `.env.example` with placeholder values instead.

The content automation program runs only on the frontend owner's machine and talks to the backend through the admin API.
