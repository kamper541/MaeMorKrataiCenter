# Project Handoff & Session State (`HANDOFF.md`)

> **Role & Purpose**: This document preserves cross-session continuity between human developers and autonomous AI agents.
> **Rule for Incoming Agents**: Always read this file at the start of a session to understand current status, what has been completed, in-flight work, and immediate next steps. Update this file before concluding your session.

---

## 1. Executive Summary & Progress

- **Project Status**: `INITIALIZED` <!-- INITIALIZED | SPEC_DRAFTING | PLANNING | EXECUTING | VERIFYING | IDLE -->
- **Last Updated**: <YYYY-MM-DD HH:MM>
- **Active Session Agent**: <Spectre | Planner | Executor (Task NN) | Verifier | Developer>
- **Progress Snapshot**: <Brief 1-2 sentence summary of overall project progress>

### Milestone Checklist
- [ ] Milestone 1: <Name/Description>
- [ ] Milestone 2: <Name/Description>

---

## 2. What We Have in Current (Current State)

<Summary of currently working features, verified capabilities, and system components>

- **Working Components**:
  - `<component/feature>`: <what it does / status>
- **Architecture & Tech Stack**:
  - Core Stack: <e.g., Node.js / Python / Rust / Bash>
  - Database / Storage: <if applicable>
- **Verification Status**:
  - Tests: `Passing (N tests)` | `Pending`
  - Security Audit: `Passed` | `Pending`

---

## 3. Active In-Flight Work

- **Active Spec**: [`docs/specs/<spec-name>.md`](docs/specs/<spec-name>.md) <!-- or None -->
- **Active Plan**: [`docs/plans/<plan-name>/00_OVERVIEW.md`](docs/plans/<plan-name>/00_OVERVIEW.md) <!-- or None -->
- **Current Task In-Flight**: `docs/plans/<plan-name>/<task-file>.md` <!-- or None -->
- **Active Battleground**: <link or None>

---

## 4. Leftovers, Blockers & Next Steps

### Immediate Next Steps (For Next Agent / Developer)
1. [ ] <Exact next action / command to run>
2. [ ] <Follow-up action>

### Leftovers & Technical Nuances
- **Leftover Tasks**: <Items remaining in current plan or milestone>
- **Known Blockers / Warnings**: <Any unresolved issues, fragile dependencies, or environment quirks>

---

## 5. Proto-Specs & Deferred Ideas

- For improvements or architectural insights discovered during execution that are worth noting for future specs, see [TODO.md](./TODO.md).

---

## 6. Context & Reference Pointers

- **Master Instructions**: [GEMINI.md](./GEMINI.md) | [AGENTS.md](./AGENTS.md)
- **Architectures**: [docs/architectures/](docs/architectures/)
- **Decisions Log**: [docs/decisions/](docs/decisions/)
- **Researches & Spikes**: [docs/researches/](docs/researches/)
- **Completed Plan Records**: [docs/plan-records/](docs/plan-records/)
