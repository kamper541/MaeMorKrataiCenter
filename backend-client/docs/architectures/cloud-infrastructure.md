# Cloud Infrastructure & Deployment Topology

## 1. Platform & Compute
- **Provider**: Cloudflare Workers (`wasm32-unknown-unknown`, built with `worker-build --release`, deployed via Wrangler `^4.0.0`).
- **Entrypoint**: `#[event(fetch)]` in `src/lib.rs`, bridging `worker::HttpRequest` into the Axum `Router` (`build_router`).

## 2. Environment Isolation (`dev`, `staging`, `prod`)
Configured in `backend-client/wrangler.toml` with strict per-environment isolation (`D02`):

| Environment | Worker Name | D1 Binding (`DB`) | R2 Binding (`PUBLIC_ASSETS`) | Rate Limiter (`PUBLIC_RATE_LIMITER`) |
| :--- | :--- | :--- | :--- | :--- |
| `dev` (default) | `maemorkratai-backend-client-dev` | `maemorkratai-db-dev` | `maemorkratai-public-dev` | `namespace_id = "2001"` (`60/60s`) |
| `staging` | `maemorkratai-backend-client-staging` | `maemorkratai-db-staging` | `maemorkratai-public-staging` | `namespace_id = "2002"` (`60/60s`) |
| `prod` | `maemorkratai-backend-client-prod` | `maemorkratai-db-prod` | `maemorkratai-public-prod` | `namespace_id = "2003"` (`60/60s`) |

## 3. Least-Privilege Bindings & Secrets
- **R2 Storage**: Binds exclusively to `PUBLIC_ASSETS`. `PRIVATE_STORAGE` is never bound to `backend-client` (`D02`).
- **Rate Limiting**: Global `[[ratelimits]]` binding `PUBLIC_RATE_LIMITER` (`60` requests per `60` seconds per key) enforced in `security_pipeline_middleware`.
- **CORS Allowlists (`ALLOWED_ORIGINS`)**:
  - `dev`: `http://localhost:3000,http://127.0.0.1:3000`
  - `staging`: `https://staging.maemorkratai.com`
  - `prod`: `https://maemorkratai.com,https://www.maemorkratai.com`
- **Secrets**: `TURNSTILE_SECRET_KEY` is provided via `.dev.vars` locally (`.dev.vars.example` for reference) and Wrangler Secrets in `staging` and `prod`.
