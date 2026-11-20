# Battleground Debate Record

- **Plan**: `SPEC-001-infra-scaffold`
- **Task**: `axum-worker-state-bridging`
- **Date**: 2026-11-16
- **Status**: RESOLVED

## 1. The Conflict / Decision Point
How should `backend-client` bridge `worker::Env` (`D1Database`, `PUBLIC_ASSETS` R2 `Bucket`, `PUBLIC_RATE_LIMITER`) into `axum::Router` while strictly enforcing `D02` least-privilege isolation (never touching `PRIVATE_STORAGE`) and supporting host `cargo test`?

## 2. Competing Options

### Option A: Direct `SendWrapper<worker::Env>` Inside Axum Handlers
- **Advocate**: Contender Agent A
- **Security (#1)**: Passing `SendWrapper<worker::Env>` to every handler allows any future route to probe arbitrary bindings (`PRIVATE_STORAGE` or secrets) and scatters D1/R2 error mapping across handlers, risking internal error leaks (`OWASP A01`, `A05`).
- **Technical Debt (#2)**: Routes couple directly to `worker::Env`, preventing host `cargo test` execution of `GET /health` and rate-limiting middleware without `workerd`.
- **Speed (#3)**: Saves roughly 60 lines of trait definitions in `SPEC-001`, then slows every `SPEC-002` test cycle.

### Option B: Trait-Backed `ClientPorts` with `WorkerClientAdapter` and Host Mocks
- **Advocate**: Contender Agent B
- **Security (#1)**: `ClientPorts` exposes only `HealthPort` and `RateLimitPort` to Axum routes and middleware. `WorkerClientAdapter` encapsulates `SendWrapper<worker::Env>` in one adapter module, binding exclusively `DB`, `PUBLIC_ASSETS`, and `PUBLIC_RATE_LIMITER` (never `PRIVATE_STORAGE`). All D1 and R2 failures convert into typed `ClientError` variants that serialize solely as the `{ request_id, code, message }` envelope (`D04`).
- **Technical Debt (#2)**: Host `cargo test` verifies `GET /health` (`200` and degraded `503`), strict CORS, rate limiting (`429`), and body limits (`413`/`400`) deterministically via `tower::ServiceExt::oneshot`. Future specs (`SPEC-002`, `TODO-005`) add domain traits onto `ClientPorts` without modifying router wiring.
- **Speed (#3)**: Adds ~100 lines of port traits and mock adapters upfront and delivers instant local test feedback.

## 3. Outcome & Winning Rationale
- **Winner**: Option B
- **Justification**: Option B wins on Priority 1 Security and Priority 2 Technical Debt by enforcing `D02` binding isolation at the Rust type boundary and enabling full host test coverage for `SPEC-001` Scenarios 1, 2, and 4.
