# Plan 00: Overview & Task Manifest

- **Spec**: [SPEC-NAME](../../specs/<spec-file>.md)
- **Created**: <YYYY-MM-DD>
- **Status**: IN_PROGRESS # IN_PROGRESS | VERIFYING | COMPLETED

## 1. Objective & Architecture Impact
<Summary of this plan and how it affects the project's architecture>

## 2. Battleground Summary
- **Debate Record**: [docs/battlegrounds/<plan-name>/<task-name>/debate.md](../../battlegrounds/<plan-name>/<task-name>/debate.md)
- **Winning Approach**: <Summary of winning design and rationale>

## 3. Interface Ledger
> Exact. The Plan Auditor aligns plans from this table.

| Task | Creates / modifies | Exports (name, signature) | Consumes (and where it comes from) |
| :--- | :--- | :--- | :--- |
| `01_<TASK>` | `<path>` | `<symbol(signature)>` | `<symbol>` (repo / Shared Contract / task NN) |

## 3b. Cross-Plan Assumptions
- <what this plan relies on from other plans, by Shared Contract name>

## 3c. Proto-Spec Candidates
- <ID, title, "Why Not Now?"; the Plan Auditor merges these into TODO.md>

## 4. Task Manifest
> **Execution Policy**: Each task below MUST be executed by a dedicated, fresh **Executor / Coder** agent. Before execution, each agent must read this `00_OVERVIEW.md` file first. Strictly 1 agent per 1 task.
> Each line ends with the executor tier: `[model/effort]`. Tasks run in order; each is read together with this file.
- [ ] `01_<TASK_NAME>.md` — <Brief description> `[sonnet/medium]`
- [ ] `02_<TASK_NAME>.md` — <Brief description> `[sonnet/medium]`
- [ ] `NN_DOCUMENTATION.md` — Write feature docs to `docs/knowledges/features/<feature>.md` and update `docs/architectures/` `[sonnet/medium]`
