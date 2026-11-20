# Project Roadmap & Proto-Spec Incubator (`TODO.md`)

> **The Proto-Spec Concept**:
> In `gb-sdlc`, task files strictly require full code and forbid in-code `// TODO: implement later` placeholders.
> However, during planning, execution, and code review, developers and agents frequently uncover insights, edge cases, refactorings, or architectural enhancements that are **worth not doing right now, but valuable to add later**.
>
> Rather than letting these ideas get lost or polluting active tasks, capture them here as **Proto-Specs** (Candidate Specs).
> When the project is ready for its next cycle, **Spectre** reviews this file and graduates high-value proto-specs into formal specifications (`docs/specs/<spec-name>.md`).

---

## 1. Proto-Spec Register

| ID | Title | Priority | Status | Discovered During | Graduated Spec |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `TODO-001` | <Title of Item> | Med | PROPOSED | Plan <name> / Task <nn> | - |

---

## 2. Active Proto-Specs

### [TODO-001] <Title of Idea / Enhancement>
- **Status**: `PROPOSED` <!-- PROPOSED | UNDER_REVIEW | GRADUATED_TO_SPEC | DISMISSED -->
- **Discovered On**: <YYYY-MM-DD>
- **Discovered By / During**: <Developer | Executor on Plan <plan> / Task <task> | Verifier>
- **Priority**: `P1 (High)` | `P2 (Medium)` | `P3 (Low)`
- **Why Not Now?**: <Rationale for deferring (e.g., MVP scope protection, avoids premature optimization, depends on external refactor)>
- **Context & Problem Statement**:
  <What problem does this solve? What behavior was observed that triggered this idea?>
- **Proto-Spec Sketch (Suggested Approach)**:
  - <Potential architecture, libraries, or patterns to consider>
  - <Key design considerations or tradeoffs>
- **Graduation / Promotion Criteria**:
  - <What condition triggers promoting this into a formal `docs/specs/<spec>.md`?>

---

## 3. Graduated & Archived Proto-Specs

### [TODO-000] <Title of Promoted Feature>
- **Status**: `GRADUATED_TO_SPEC`
- **Graduated On**: <YYYY-MM-DD>
- **Graduated Spec**: [`docs/specs/<spec-name>.md`](docs/specs/<spec-name>.md)
- **Outcome / Notes**: <Promoted by Spectre into active spec pipeline>
