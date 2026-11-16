# Workflow 02: Planning, Battleground & Full-Code Tasks (Agent Factory Floor)

## Objective
Convert an approved spec into atomic, full-code tasks while resolving architectural dilemmas via the Battleground.

## Step-by-Step Flow
1. **Review Approved Spec**:
   - Read `docs/specs/<spec>.md`, linked `docs/decisions/D*.md`, and `docs/architectures/`.
2. **Execute Battleground (If Multiple Approaches Exist)**:
   - Identify candidate approaches.
   - Spawn **Battleground Contender** agents to debate:
     1. Security (#1)
     2. Low Technical Debt (#2)
     3. Fast Implementation (#3)
   - Save debate in `docs/battlegrounds/<plan-name>/<task-name>/debate.md`.
3. **Scaffold Plan Folder**:
   - Create `docs/plans/<plan-name>/`.
   - Write `00_OVERVIEW.md` linking the spec, battleground debate, and task manifest.
4. **Draft Full-Code Tasks**:
   - Author `01_<TASK>.md` through `NN-1_<TASK>.md` containing complete, exact code and test commands.
   - Author `NN_DOCUMENTATION.md` to update `docs/knowledges/features/<feature>.md` and `docs/architectures/`.
5. **Trigger Executor / Coder Execution**:
   - Hand off to Workflow 03 (spawns 1 fresh Executor / Coder agent per task, reading `00_OVERVIEW.md` first).
