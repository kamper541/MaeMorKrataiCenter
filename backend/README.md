# backend

Single backend workspace for MaeMorKratai (Rust + Axum on Cloudflare Workers, D1, R2).

| Folder | Purpose |
|---|---|
| `client/` | Public API Worker for `frontend-client`. Contract: `../docs/api/client/` |
| `admin/` | Admin API Worker (auth, JWKS issuer) for `frontend-admin`. Contract: `../docs/api/admin/` |
| `migrations/` | Shared D1 migrations (canonical, owned by admin) |
| `tools/create-user/` | CLI to create admin users |
| `docs/client/`, `docs/admin/` | gb-sdlc specs, plans, decisions per service |
| `.gb-sdlc/` | Spec-driven development framework (shared) |

Each of `client/` and `admin/` is its own Worker with its own `wrangler.toml`. See `Makefile` for commands.
