# Task NN: Documentation & Architecture Update

- **Plan Overview**: [00_OVERVIEW.md](./00_OVERVIEW.md)
- **Target Knowledge File**: `docs/knowledges/features/<feature-name>.md`
- **Executor / Coder Rule**:
  1. ALWAYS read [00_OVERVIEW.md](./00_OVERVIEW.md) first before executing.
  2. Execute ONLY this documentation task as a fresh agent. When complete, hand off to Verifier.

## 1. Objective
Produce complete, permanent documentation for the feature just implemented and update project architecture records.

## 2. Feature Documentation Content
Write to `docs/knowledges/features/<feature-name>.md`:
- **Overview**: What the feature does and its architecture
- **How to Use / API Specs**: Request/response contracts, UI flows, CLI flags
- **Guidelines & Invariants**: Security constraints, usage rules, edge cases handled
- **Linked Decisions**: References to `docs/decisions/`

## 3. Architecture Update
If any new library, database entity, cloud service, or environmental variable was added:
- Update `docs/architectures/tech-stack.md` or relevant architecture doc.
