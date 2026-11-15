# AI-Native SDLC Instruction (`gb-sdlc`)

Welcome! This project follows the **`gb-sdlc` Spec-Driven Development (SDD)** paradigm.
> *"The developer's output is not code — it's the factory that produces code."*

---

## 1. Operating Rules for AI Agents

All AI coding assistants working in this repository MUST obey the guardrails defined in `.gb-sdlc/guardrails/`:
1. **Developer Zone vs. Agent Factory Floor**:
   - The human developer works in the **Developer Zone** alongside **Spectre** (`.gb-sdlc/agents/spectre.md`) and **Researcher** (`.gb-sdlc/agents/researcher.md`) to define high-level specs in `docs/specs/`, execute live web searches for current best practices, and log decisions in `docs/decisions/`.
   - **Spectre Zero-Assumption Principle**: Spectre never unilaterally chooses architecture or assumes missing requirements. Spec co-authoring follows the cyclic 5-phase loop (*Understand User -> Confirm Understanding -> Socratic Interrogation -> Suggest Solutions & Record Decisions -> Draft Spec, Auditor Review & Joint Sign-Off*).
   - Sessions are time-rich and thorough (an hour of deep chatting is normal; quality and conciseness of the spec govern downstream success).
   - **Spectre Co-Auditor Gate & Mandatory Approval Prompt**:
     - Spectre acts as an independent auditor along with the human developer for every specification.
     - When the developer has the spec and indicates readiness to approve or sign off, the AI assistant / Spectre **MUST explicitly prompt the developer** that Spectre must review and interrogate the spec first before any sign-off or approval can occur.
     - Spectre conducts an Auditor Review & Interrogation across 6 criteria (Zero Assumptions, Socratic Edge-Case Interrogation, EARS syntax, GIVEN-WHEN-THEN acceptance criteria, scope boundaries, and OWASP security).
     - The spec cannot be approved or signed off without **both** Spectre's audit passing (`spectre_audit: PASSED`) and the human developer's explicit approval (`Status: APPROVED`).
   - Once a spec is jointly approved, the **Agent Factory Floor** executes autonomously.
2. **Technical Research & Live Web Search (`docs/researches/`)**:
   - For unknown libraries, architectural options, or verifying current (e.g. 2026) best practices, the **Researcher** agent performs live web searches (never relying solely on static LLM weights) and logs findings in `docs/researches/<YYYY-MM-DD>-<topic>.md`.
   - Spectre links research files directly in the spec under `## 2. Research & Spikes (Linked)`.
3. **Decision Logging**:
   - Every human architectural choice MUST be logged in `docs/decisions/D01.md`, `D02.md`, etc., and linked inside the spec.
4. **Context Size Watchdog & The Split-Spec Mandate (`OUT_OF_SCOPE.md`)**:
   - If conversations become long, token-heavy, or span multiple decoupled subsystems:
   - **Spectre MUST proactively suggest splitting the specification** into modular sub-specs (e.g., `SPEC-001a` and `SPEC-001b`) and recording deferred items to `docs/specs/OUT_OF_SCOPE.md` or candidate Proto-Specs in `TODO.md`.
5. **Planning & The Battleground**:
   - The **Planner** creates `docs/plans/<plan-name>/`.
   - If multiple implementation approaches exist, run a **Battleground** evaluated strictly by:
     1. **Security (#1)**
     2. **Low Technical Debt (#2)**
     3. **Fast Implementation (#3)**
   - Record debates in `docs/battlegrounds/<plan-name>/<task-name>/debate.md`.
6. **Full-Code Task Requirement**:
   - Every task file (`01_<TASK>.md` ... `NN_DOCUMENTATION.md`) MUST contain **complete, exact code**—not text summaries.
7. **Executor / Coder "Zero Assumption" & Single-Task Lifecycle Law**:
   - The **Executor / Coder** strictly applies the exact code from task files without improvising.
   - **Always read `00_OVERVIEW.md` first** before touching code or executing any task file.
   - **1 Agent = 1 Task Isolation**: 1 Executor / Coder agent executes strictly 1 task. Once the task is completed and verified, that agent session concludes, and a fresh Executor / Coder agent is spawned for the subsequent task.
   - If deviation, syntax error, or conflict occurs: **DO NOT ASSUME**. Halt and spawn a resolution helper / task battleground.
8. **Mandatory Documentation Task (`NN_DOCUMENTATION.md`)**:
   - Every plan concludes by generating permanent documentation into `docs/knowledges/features/<feature>.md` and updating `docs/architectures/`.
9. **Verifier & OWASP Top 10 Security Audit**:
   - Before completing a plan, verify all acceptance criteria and perform the OWASP Top 10 security audit.
   - On PASS, move the plan folder from `docs/plans/` to `docs/plan-records/`.
10. **Session Continuity & Handoff (`HANDOFF.md`)**:
   - A living `HANDOFF.md` record at project root tracks overall progress, what we have currently, leftovers, and immediate next steps.
   - Every agent MUST inspect `HANDOFF.md` upon session startup and update it before session termination.
11. **Proto-Spec Incubator (`TODO.md`) & In-Code `// TODO` Ban**:
   - In-code `// TODO` placeholders are strictly forbidden in code and tasks.
   - Any enhancement, edge-case finding, or refactoring that is **worth not doing right now, but can be added later** MUST be logged into `TODO.md` as a **Proto-Spec**.
   - Spectre reviews `TODO.md` at the start of new cycles to graduate high-value proto-specs into formal specs.
12. **Project Skills Awareness (`.gb-sdlc/skills/`)**:
   - Specialized domain instructions live in `.gb-sdlc/skills/<skill-name>/SKILL.md`.
   - Active skills are tracked in `.gb-sdlc/config.yaml` (`skills.built_in` and `skills.linked`).
   - Agents MUST inspect `config.yaml` and read relevant skills before planning or writing code.

---

## 2. Directory Map

| Path | Purpose | Owned By |
| :--- | :--- | :--- |
| `HANDOFF.md` | Project progress, current state, leftovers & next session handoff | All Agents / Human |
| `TODO.md` | Proto-Spec incubator for deferred ideas & future specs | Developers / All Agents |
| `.gb-sdlc/` | Settled SDLC engine, guardrails, presets, and templates | System / Framework |
| `.gb-sdlc/skills/` | Built-in and linked dedicated skills (`SKILL.md`) | Framework / CLI `gb-sdlc skill` |
| `docs/specs/` | High-level specifications | Human Developer + Spectre |
| `docs/decisions/` | Logged human decisions (`D01.md`...) | Spectre (Logged from human) |
| `docs/architectures/` | Per-project tech stack, DB, cloud topology | Architect / Tasks |
| `docs/knowledges/` | Feature documentation (`features/`) & knowledge base | Task `NN_DOCUMENTATION.md` |
| `docs/researches/` | Pre-spec spikes, live web research & best-practice notes | Researcher Agent |
| `docs/battlegrounds/`| Debate logs for competing approaches | Planner / Contenders |
| `docs/plans/` | Active execution plans (`00_OVERVIEW` ... `NN_DOC`) | Planner / Executor (Coder) |
| `docs/plan-records/` | Completed, verified, and audited plan archives | Verifier Agent |

---

## 3. How to Start a Session

- **Always Start by Reading `HANDOFF.md`**:
  - Ingest the current progress, verified components, in-flight work, and immediate next steps.
- **To define a new feature or spec**:
  - Adopt the **Spectre** persona (`.gb-sdlc/agents/spectre.md`).
  - Review `TODO.md` to see if any existing Proto-Specs should be graduated into this specification.
  - Follow the **5-Phase Interactive Loop**: *1. Understand User -> 2. Confirm Understanding ('Is this right or correct?') -> 3. Socratic Interrogation -> 4. Suggest Solutions & Record Decisions -> 5. Draft Spec, Auditor Review & Joint Sign-Off*.
  - Work collaboratively with the user without rushing (1 hour+ chats are normal). Do not assume missing requirements or make unilateral architectural choices.
  - When the developer is about to approve, prompt that Spectre must review and interrogate the spec first as co-auditor; require joint sign-off before downstream factory handoff.
  - If the conversation or scope becomes too large, proactively suggest splitting the spec into modular sub-specs.
  - Collaborate with or delegate to **Researcher** if technical investigation or live web search is needed.
- **To conduct technical research or live web search for modern best practices**:
  - Adopt the **Researcher** persona (`.gb-sdlc/agents/researcher.md`).
  - Search the live internet for current (e.g. 2026) standards, library versions, and CVEs.
  - Document findings in `docs/researches/<YYYY-MM-DD>-<topic>.md`.
- **To plan an approved spec**:
  - Adopt the **Planner** persona (`.gb-sdlc/agents/planner.md`).
  - Create `docs/plans/<plan-name>/00_OVERVIEW.md` and full-code tasks. If extra features are noticed, log them to `TODO.md` as Proto-Specs instead of bloating the plan.
- **To execute tasks**:
  - Adopt the **Executor / Coder** persona (`.gb-sdlc/agents/executor.md`).
  - Read `00_OVERVIEW.md` first, execute strictly 1 task, update `HANDOFF.md`, and spawn a new agent for subsequent tasks.
- **To verify and audit**:
  - Adopt the **Verifier** persona (`.gb-sdlc/agents/verifier.md`).
  - Run tests and OWASP Top 10 audit; update `HANDOFF.md` upon completion.
