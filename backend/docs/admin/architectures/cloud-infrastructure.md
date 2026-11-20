# Cloud Infrastructure & Deployment Topology

## 1. Platform & Constraints
- **Compute**: Cloudflare Workers (`wasm32-unknown-unknown`), targeting Cloudflare Workers Free tier (`10 ms` CPU cap per invocation).
- **Compatibility Date**: `2026-11-01` (`wrangler.toml`)
- **Build Command**: `worker-build --release` (with `opt-level = "s"`, `lto = true`, `strip = true`, `codegen-units = 1`).

## 2. Bindings & Storage Topology (`D02`)
`backend-admin` binds the following Cloudflare resources per environment:
- **D1 (`DB`)**: Shared relational database (read/write; `backend-admin` owns migrations).
- **R2 (`PUBLIC_ASSETS`)**: Shared public asset bucket (e.g., published card art/scene media).
- **R2 (`PRIVATE_STORAGE`)**: Admin-only private R2 bucket (audio, drafts, Content Studio artifacts). Never bound to `backend-client`.
- **Rate Limiting (`[[ratelimits]]` -> `AUTH_RATE_LIMITER`)**: Native Cloudflare rate limiter configured at `limit = 10`, `period = 60` seconds.

## 3. Environment Isolation (`wrangler.toml`)
Three strictly isolated environments are configured:

| Environment | Worker Name | D1 Database (`DB`) | R2 `PUBLIC_ASSETS` | R2 `PRIVATE_STORAGE` | Rate Limiter Namespace |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `dev` (default) | `maemorkratai-backend-admin-dev` | `maemorkratai-db-dev` | `maemorkratai-public-dev` | `maemorkratai-private-dev` | `1001` |
| `staging` | `maemorkratai-backend-admin-staging` | `maemorkratai-db-staging` | `maemorkratai-public-staging` | `maemorkratai-private-staging` | `1002` |
| `prod` | `maemorkratai-backend-admin-prod` | `maemorkratai-db-prod` | `maemorkratai-public-prod` | `maemorkratai-private-prod` | `1003` |

### Environment Variables & Secrets
- **Vars (`[vars]`)**: `ENVIRONMENT`, `ALLOWED_ORIGINS`, `JWT_ISSUER`, `JWT_AUDIENCE`, `ACCESS_TOKEN_TTL_SECS` (`900`), `REFRESH_TOKEN_TTL_SECS` (`604800`), `COOKIE_NAME` (`mmk_admin_rt`), `MAX_BODY_BYTES` (`65536`).
- **Secrets (`.dev.vars` locally; Wrangler Secrets in `staging`/`prod`)**:
  - `PASSWORD_PEPPER`
  - `KEY_ENCRYPTION_KEY` (base64-encoded 256-bit key)
