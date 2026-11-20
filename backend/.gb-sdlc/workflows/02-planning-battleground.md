# Workflow 02: Estimate, Plan, Battleground, Audit (Agent Factory Floor)

## Objective
Convert an approved spec into atomic, full-code tasks without any agent session passing ~200k tokens (over = 300k), resolving architectural dilemmas via the Battleground, then align every plan before execution.

## Step-by-Step Flow
1. **Gate**: the spec has `spectre_audit: PASSED` and `Status: APPROVED`.
2. **Estimate** (Plan Estimator, `agents/plan-estimator.md`):
   - Sizes the spec, decides how many plans, writes `docs/plans/<spec-name>-ESTIMATE.md` with each plan's scope, dependencies and the **Shared Contract**.
   - The human may adjust the cut before planners start.
3. **Plan in parallel** (one Planner per plan, `agents/planner.md`):
   - Each Planner owns one slice, codes against the Shared Contract, and never blocks on another plan. Planners do not message each other or write `HANDOFF.md`/`TODO.md`; assumptions go in `## Cross-Plan Assumptions`, proto-spec candidates in `## Proto-Spec Candidates`.
   - Each Planner runs Battlegrounds where several approaches exist (Security #1, Low Technical Debt #2, Fast Implementation #3), saved to `docs/battlegrounds/<plan-name>/<task-name>/debate.md`.
   - Each writes `docs/plans/<plan-name>/00_OVERVIEW.md` (with Interface Ledger), `01_<TASK>.md` ... `NN-1_<TASK>.md`, and `NN_DOCUMENTATION.md`. Tasks are test-first with the harness, written to the project's linter (guardrails 04 and 05). The bootstrap plan's Task 01 creates the standards toolchain and harness when missing.
4. **Plan Audit** (Plan Auditor, `agents/plan-auditor.md`):
   - Aligns all plans: references to code that does not exist yet, names/signatures, ordering, file ownership, harness and lint prerequisites, acceptance coverage, executor bar.
   - Fixes task files in place, writes `docs/plans/<spec-name>-PLAN-AUDIT.md` with the global execution order and a verdict.
5. **Trigger Execution**: on `ALIGNED`, hand off to Workflow 03, running plans and tasks in the audited order (1 fresh Executor / Coder agent per task, reading `00_OVERVIEW.md` first). On `NEEDS HUMAN`, the human resolves the listed questions first.
