# Guardrail 03: Architectural Principles & Technical Debt Control

## 1. Architecture Alignment
- All implementation must respect the project's architecture documented in `docs/architectures/`.
- If a plan requires introducing a new database, cloud service, third-party library, or changing directory structure, `docs/architectures/` MUST be updated as part of the plan's final documentation task (`NN_DOCUMENTATION.md`).

## 2. Low Technical Debt Mandate
- Keep modules loosely coupled and highly cohesive.
- Follow Single Responsibility Principle (SRP) per file/function.
- Avoid premature over-engineering: build the simplest clean solution that satisfies the spec and security guardrails.
- Ensure strict typing, lint compliance, and idiomatic conventions for the project's language stack.

## 3. The Battleground Priority Hierarchy
When comparing technical approaches during planning, contenders are evaluated strictly in this order:
1. **Security**: Does this approach protect data and minimize attack surface?
2. **Low Technical Debt**: Is this approach maintainable, clean, and easy to modify later?
3. **Fast Implementation**: Can this be built pragmatically without unnecessary layers?
