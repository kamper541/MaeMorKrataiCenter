# Workflow 03: Deterministic Execution & Deviation Escalation

## Objective
Execute tasks with zero improvisation, strict single-task agent isolation (1 task = 1 fresh Executor / Coder agent), pre-execution context loading via `00_OVERVIEW.md`, and immediate deviation escalation.

## Step-by-Step Flow

1. **Spawn Fresh Executor / Coder Agent (1 Agent per Task)**:
   - For the next uncompleted task in `docs/plans/<plan-name>/`, instantiate a fresh, isolated **Executor / Coder** agent.
   - **Strict Rule**: 1 Executor / Coder agent executes strictly 1 task. Never reuse an agent session across multiple tasks.

2. **Mandatory Pre-Execution Ingestion**:
   - The newly spawned agent MUST ALWAYS read `00_OVERVIEW.md` FIRST before reading or executing the assigned task file.
   - Ingest plan context, architectural scope, and task manifest position.

3. **Deterministic Task Execution & Proto-Spec Logging**:
   - The agent opens the single assigned task file (e.g., `01_<TASK>.md`).
   - Applies the exact file creations, modifications, and shell commands without improvisation.
   - **No in-code `// TODO`**: If non-blocking ideas or secondary improvements are uncovered, log them to `TODO.md` as Proto-Specs.
   - Runs the task's verification command.

4. **Deviation Escalation Protocol**:
   - If commands or code fail, or existing code conflicts with task instructions:
   - **DO NOT GUESS OR IMPROVISE.**
   - Spawn a resolution subagent with the Spec, `docs/decisions/`, and `00_OVERVIEW.md`.
   - If multiple fixes exist, run a task-level Battleground in `docs/battlegrounds/<plan-name>/<task-name>/`.
   - Apply the resolved code and re-verify.

5. **Task Completion & Handoff Update**:
   - Upon successful verification, mark the task checkbox complete in `00_OVERVIEW.md`.
   - Update `HANDOFF.md` (What We Have in Current, next task in Immediate Next Steps).
   - The current Executor / Coder agent terminates / concludes its turn.
   - If more tasks remain, return to Step 1: spawn a brand-new Executor / Coder agent for the next task.

6. **Documentation Task (`NN_DOCUMENTATION.md`)**:
   - A dedicated fresh Executor / Coder agent executes `NN_DOCUMENTATION.md` to produce feature docs in `docs/knowledges/features/<feature>.md` and update `docs/architectures/`.

7. **Trigger Verification**:
   - Once all tasks in `00_OVERVIEW.md` are marked complete, hand off to Workflow 04 (Verifier).
