# Battleground Debate Record

- **Plan**: `SPEC-001-infra-scaffold`
- **Task**: `turnstile-and-jwks-guard-design`
- **Date**: 2026-11-16
- **Status**: RESOLVED

## 1. The Conflict / Decision Point
How should `backend-client` structure the reusable `TurnstileGuard` and `JwksVerifier` scaffolding modules (`SPEC-001` Requirement #6 and Scenario 5) so they fail closed in production and verify cleanly in host tests?

## 2. Competing Options

### Option A: Concrete `worker::Fetch`-Coupled Helper Functions
- **Advocate**: Contender Agent A
- **Security (#1)**: Standalone functions called manually inside handlers risk omission on future `SPEC-002+` routes (`OWASP A01`, `A04`). Coupling directly to `worker::Fetch` prevents fast host unit testing of fail-closed upstream outage paths (`503`), missing/rejected Turnstile tokens (`403`), and JWT algorithm confusion (`alg: none`, `HS256`, expired `exp`, unknown `kid` -> `401`).
- **Technical Debt (#2)**: Upgrading manual function calls into route middleware layers later requires rewriting future endpoints.
- **Speed (#3)**: Slightly less boilerplate initially, but forces manual `wrangler dev` testing for every edge case.

### Option B: Trait-Based `TurnstileTransport` and `JwksSource` with `TurnstileGuard` and `JwksVerifier`
- **Advocate**: Contender Agent B
- **Security (#1)**: `TurnstileGuard` and `JwksVerifier` run as composable Axum extractors and middleware, enforcing checks before handler execution (`OWASP A01`, `A07`). `TurnstileGuard` fails closed with HTTP `403` on missing or invalid tokens and HTTP `503` on upstream transport errors, timeouts, or non-200 responses, never bypassing verification (`D04`). `JwksVerifier` restricts verification strictly to `ES256` (`p256`) and `RS256` (`rsa` verify-only), rejects `alg: none` and symmetric algorithms, validates `kid`, `iss`, `aud`, `exp`, and `nbf`, and rejects failures with HTTP `401` via the sanitized error envelope.
- **Technical Debt (#2)**: `WorkerFetchTransport` implements the traits for `wasm32-unknown-unknown`, while deterministic fixture transports exercise every valid, invalid, expired, and upstream-error path under host `cargo test` (`SPEC-001` Scenario 5). `SPEC-002` and `TODO-005` attach the guards directly to routes with zero core middleware refactoring.
- **Speed (#3)**: Moderate upfront trait and verifier code inside `src/security/`, paired with instant host test execution.

## 3. Outcome & Winning Rationale
- **Winner**: Option B
- **Justification**: Option B wins on Priority 1 Security and Priority 2 Technical Debt by enforcing fail-closed guard boundaries (`401`/`403`/`503`), supporting `ES256`/`RS256` key rotation, and proving every failure mode under host `cargo test`.
