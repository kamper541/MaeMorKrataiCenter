# Workflow 05: Session Handoff & Proto-Spec Management

## Objective
Preserve continuous operational context across agent turns, human sessions, and execution stages using `HANDOFF.md`, and capture deferred insights into `TODO.md` as Proto-Specs.

## Step-by-Step Flow

1. **Session Start (Mandatory Ingestion)**:
   - Any agent (Spectre, Planner, Executor, Verifier) or developer starting a session MUST read `HANDOFF.md` first.
   - Read Section 1 (Executive Summary), Section 2 (What We Have in Current), and Section 4 (Immediate Next Steps).
   - If mid-plan execution, read `docs/plans/<plan-name>/00_OVERVIEW.md` as required by Guardrail 01.

2. **Capturing Discoveries ("Worth Not Doing Right Now") $\rightarrow$ `TODO.md`**:
   - During planning, code execution, or review, when a developer or agent notices:
     - An architectural enhancement or optimization opportunity
     - An edge-case handling idea outside the current spec scope
     - A potential refactoring or cleanup
   - **Strict Rule**: DO NOT add in-code `// TODO` comments or silently bloat current task code.
   - Append the discovery to `TODO.md` under `## 2. Active Proto-Specs` using the proto-spec structure:
     - `Status: PROPOSED`
     - `Why Not Now?` (explicit deferral rationale)
     - Problem statement and suggested approach sketch.

3. **Promoting Proto-Specs into Formal Specs (Spectre)**:
   - When the user starts a new spec session with **Spectre**:
   - Spectre reviews `TODO.md` with the user.
   - If an active Proto-Spec is selected for execution:
     - Spectre drafts `docs/specs/<spec-name>.md` using the proto-spec's context.
     - Mark the item in `TODO.md` as `GRADUATED_TO_SPEC` pointing to the new spec file.

4. **Session End & Milestone Handover (`HANDOFF.md` Update)**:
   - Before completing an agent run or concluding work:
     - Update `HANDOFF.md` Section 1: `Last Updated`, `Active Session Agent`, and `Project Status`.
     - Update Section 2 (What We Have in Current): newly completed features, test status.
     - Update Section 3 (Active In-Flight Work): point to current active spec or plan.
     - Update Section 4 (Immediate Next Steps): write exact, actionable next steps for the incoming agent.
