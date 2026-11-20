# Database Architecture & Migration Policy

## 1. Database Engine
- **Engine**: Cloudflare D1 (SQLite at the edge), accessed via `worker::D1Database` (`d1` feature in `worker 0.8.5`).
- **Binding Name**: `DB` across `dev`, `staging`, and `prod` in `wrangler.toml`.

## 2. Shared Schema Ownership Policy (`D01`)
- `backend-client` shares the D1 database with `backend-admin` per environment (`maemorkratai-db-dev`, `maemorkratai-db-staging`, `maemorkratai-db-prod`).
- **Canonical Migrations**: All SQL migrations are authored, stored, and applied exclusively from `../backend-admin/migrations/`.
- `backend-client/wrangler.toml` MUST NOT declare `migrations_dir` and `backend-client/` MUST NOT author or run D1 schema migrations.

## 3. Phase 1 Access Posture & Test Isolation
- In `SPEC-001`, `backend-client` performs only a readiness probe (`HealthPort::check_d1`) via `WorkerClientAdapter` (`src/adapters/worker_env.rs`).
- Auth tables (`users`, `sessions`, `jwks_keys` per `D03`) are owned by `backend-admin` migrations; `backend-client` does not issue tokens or access `PASSWORD_PEPPER` / `KEY_ENCRYPTION_KEY`.
- Host unit and integration tests (`cargo test`) use `MemoryClientStore` (`src/adapters/memory.rs`) implementing `HealthPort` and `RateLimitPort` without requiring a live D1 instance.
