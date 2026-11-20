# Guardrail 01: Core Rules & Agent Operational Discipline

## 1. The Core Philosophy
> *"The developer's output is not code — it's the factory that produces code."*  
> *"If the specification is clear, concise, and unambiguous, the entire downstream factory runs cleanly. If the specification is vague, assumed, or bloated, everything downstream degrades."*
- The human developer operates in the **Developer Zone** alongside **Spectre** (spec co-authoring, scope architect & formal spec auditor) and **Researcher** (technical spikes & live web investigation) to define high-level specs and make explicit architectural decisions.
- **Spectre Zero-Assumption Principle**: Spectre MUST NOT unilaterally choose architecture, database designs, auth flows, or edge-case behaviors, and MUST NOT assume missing requirements. All strategic choices belong to the human developer.
- **5-Phase Interactive Loop**: Spec authoring follows a cyclic loop: *1. Understand User -> 2. Confirm Understanding ('Is this right or correct?') -> 3. Socratic Interrogation -> 4. Suggest Solutions & Record Decisions -> 5. Draft Spec, Spectre Auditor Review & Joint Sign-Off*.
- **Patience & Time-Rich Collaboration**: Spec definition is thorough; sessions can take an hour or more of interactive chatting. Rushing a spec is strictly forbidden.
- The **Agent Factory Floor** executes autonomously once a spec is settled and approved.

## 2. Live Web Search & Technical Spikes (`docs/researches/`)
- Pre-trained model weights are static and prone to obsolete patterns. When evaluating third-party packages, framework APIs, or architecture standards:
  - Agents MUST NOT guess or assume based on outdated training data.
  - The **Researcher** agent MUST perform **live web searches** to verify current internet knowledge (e.g., modern best practices for 2026, latest stable releases, breaking changes, and active CVEs).
  - All research spikes MUST be recorded in `docs/researches/<YYYY-MM-DD>-<topic>.md` and hyperlinked in the relevant spec.

## 3. Spectre Auditor Review & Joint Approval Gate (`docs/specs/<spec-name>.md`)
- **The Spec Approval Gate**: The human developer reviews and approves only once in the entire SDLC cycle: at the Specification Gate (`docs/specs/<spec-name>.md`).
- **Spectre as Co-Auditor**: Spectre acts as the dedicated, independent auditor alongside the human developer for every specification. A specification CANNOT be approved or signed off unilaterally.
- **Mandatory User Prompt**: When a spec is drafted and the human developer indicates readiness to approve or sign off:
  - The AI assistant / Spectre MUST explicitly prompt the user that Spectre must review and interrogate the spec first before any sign-off or approval can take place.
  - *Prompt template*: "Before this specification can be approved or signed off, Spectre must conduct an Auditor Review and Interrogation first. As your co-auditor, Spectre must verify zero assumptions, stress-test edge cases, validate EARS compliance, check security guardrails, and confirm scope boundaries. The spec cannot be approved or signed off until this audit passes."
- **Mandatory Spectre Interrogation & Audit**:
  - Spectre audits the draft across 6 core criteria: (1) Zero Assumptions, (2) Deep Socratic Interrogation & Edge Cases, (3) EARS Syntax, (4) Concrete Acceptance Criteria, (5) Scope Watchdog / Split-Spec Check, and (6) Security & OWASP alignment.
  - If any ambiguities, missing failure modes, or hidden assumptions are discovered during audit interrogation, Spectre raises them for collaborative resolution with the human developer.
- **Joint Sign-Off Requirement**:
  - A spec reaches `Status: APPROVED` ONLY when both conditions are satisfied:
    1. **Spectre Audit**: `spectre_audit: PASSED` recorded in the spec frontmatter and Section 7 checklist.
    2. **Human Approval**: The developer provides explicit human sign-off (`Status: APPROVED`).
  - Neither Spectre nor the human developer can unilaterally bypass the audit gate.
- Once joint sign-off is achieved, the **Agent Factory Floor** (Planner -> Contender -> Executor/Coder -> Verifier) executes autonomously.

## 4. Human Decision Logging (`docs/decisions/D01.md`)
- Every strategic choice, trade-off, or constraint decided by the human MUST be logged in sequential order: `docs/decisions/D01.md`, `D02.md`, etc.
- Every decision file must be explicitly hyperlinked inside the corresponding `docs/specs/<spec-name>.md`.

## 5. Context Size Watchdog & The Split-Spec Mandate (`OUT_OF_SCOPE.md`)
- If a conversation with the human about a spec grows too long, or if the feature expands across multiple subsystems, user roles, or milestones:
  - **Spectre MUST proactively suggest splitting the specification** into smaller, modular sub-specs (e.g., `SPEC-001a-<feature>-core.md` and `SPEC-001b-<feature>-advanced.md`).
  - All deferred items, secondary capabilities, or unaddressed edge cases MUST be written to `docs/specs/OUT_OF_SCOPE.md` or logged to `TODO.md` as candidate Proto-Specs.
  - `OUT_OF_SCOPE.md` serves as the seed input for the next Spectre session.

## 6. Full-Code Plan Requirement & In-Code `// TODO` Ban
- The **Planner Agent** MUST provide complete, exact, syntactically valid code in every task file (`01_<TASK>.md` ... `NN_DOCUMENTATION.md`).
- Vague summaries, placeholders (`// TODO: implement later`), or hand-waving text instructions are strictly forbidden in task files and implementation code.
- If a developer or agent discovers an optimization, refactoring opportunity, or secondary feature that is **worth not doing right now, but can be added later**, they MUST NOT insert in-code `// TODO` comments. Instead, log it to `TODO.md` as a **Proto-Spec** (see Rule 10).

## 7. Executor / Coder "Zero Assumption" & Single-Task Lifecycle Law
- The **Executor / Coder Agent** is a pure code applier. It executes the exact code in the task file without inventing or improvising.
- **Mandatory Pre-Execution Ingestion**: The Executor / Coder agent MUST ALWAYS read `00_OVERVIEW.md` first before touching code or starting task execution.
- **One Agent, One Task Isolation**: 1 Executor / Coder agent executes strictly ONE task. Chaining multiple tasks within the same agent context is strictly prohibited to avoid context bleed and token degradation. Once the task is completed and verified, the agent concludes, and a fresh Executor / Coder agent is instantiated for the next task.
- **Deviation Escalation Protocol**:
  - If a task's code deviates from existing repository state, fails to compile, or contains a contradiction:
  - **The Executor / Coder MUST NOT assume or guess.**
  - The agent MUST immediately halt and spawn a resolution subagent reading the original Spec, `docs/decisions/`, and `00_OVERVIEW.md`.
  - If multiple resolution paths exist, a task-level **Battleground** MUST be run in `docs/battlegrounds/<plan-name>/<task-name>/` to determine the winning fix.

## 8. Mandatory Plan Bookends
- Every plan folder (`docs/plans/<plan-name>/`) MUST begin with `00_OVERVIEW.md`.
- Every plan folder MUST conclude with `NN_DOCUMENTATION.md`, which writes complete user/developer documentation into `docs/knowledges/features/<feature>.md` and updates `docs/architectures/`.

## 9. Session Continuity & Handoff (`HANDOFF.md`)
- A living handoff record MUST be maintained at root (`HANDOFF.md`) to preserve state continuity across agent turns, human sessions, and execution stages.
- `HANDOFF.md` tracks:
  1. **Executive Summary & Progress**: High-level completion snapshot and milestone checklist.
  2. **What We Have in Current**: Currently working components, active architecture, test/security status.
  3. **Active In-Flight Work**: Active spec, active plan, current task in-flight.
  4. **Leftovers, Blockers & Next Steps**: Immediate next action for the incoming agent, remaining tasks, technical blockers.
- **Agent Lifecycle Obligation**: Every agent (Spectre, Planner, Executor, Verifier) MUST read `HANDOFF.md` at session start and update its sections before concluding.

## 10. The Proto-Spec Incubator (`TODO.md`)
- During planning, deterministic execution, and verification, agents and developers frequently uncover valuable improvements, architectural refactorings, or edge cases that are **worth not doing right now, but should be added later**.
- Instead of losing these discoveries or leaving scattered in-code comments, they MUST be captured in `TODO.md` as structured **Proto-Specs** (Candidate Specs).
- Each Proto-Spec captures:
  - ID, Title, Priority, and Origin (which plan/task spawned it)
  - "Why Not Now?" (explicit rationale for deferral)
  - Problem statement and suggested approach sketch
  - Graduation criteria
- **Promotion to Spec**: When initiating a new development cycle, **Spectre** reviews `TODO.md` with the human developer and promotes high-value proto-specs into formal specifications (`docs/specs/<spec-name>.md`).

## 11. Project Skills Awareness & Ingestion (`.gb-sdlc/skills/`)
- Projects maintain reusable domain instructions and standards inside `.gb-sdlc/skills/<skill-name>/SKILL.md`.
- Active skills are recorded in `.gb-sdlc/config.yaml` under `skills:` (`built_in` and `linked`).
- **Agent Skill Ingestion Law**:
  - Before authoring specs (Spectre), breaking down plans (Planner), or executing task code (Executor / Coder), agents MUST inspect `.gb-sdlc/config.yaml` to identify all active built-in and linked skills.
  - If a skill's triggers or domain match the active task, the agent MUST load `.gb-sdlc/skills/<skill-name>/SKILL.md` and strictly follow its guidelines.
  - Agents MUST NOT invent conflicting conventions when an active project skill provides authoritative instructions.

## 12. Explicit Model & Effort Routing (No Parent Inheritance)
- Every spawned agent MUST be given an explicit `model` and `effort`. An agent never runs on the model of the session that spawned it, so a human chatting with an expensive model does not make the whole factory expensive.
- Defaults live in `.gb-sdlc/config.yaml` under `models:`. Use the cheapest tier that can do the job reliably:
  - Repo lookups and file finding: Haiku, low effort.
  - Research, Battleground contenders, plan estimation, standard planning, plan audit, verification: Sonnet at medium to high effort.
  - Planning a spec marked `complexity: complex`: Opus, medium effort. `extreme`: Opus, high effort, only after the human confirms.
  - **Executors NEVER run on Opus.** Plans contain the complete code, so execution is mechanical: Sonnet medium by default, Haiku low for mechanical edits. If a task is too hard for Sonnet, the task is under-specified: the Planner or Plan Auditor splits it smaller or writes it out more explicitly. A bigger executor model is not an option.
- **Escalation is suggested, never silent**, and applies to planners only. When a spec looks unusually hard (reverse engineering an existing app, very large branching logic, subtle concurrency, security-critical code), the agent that notices MUST tell the human, say why, and recommend a stronger planner.
- The orchestrator (`/gb-execute`) MUST pass the task's `model` and `effort` when spawning its Executor. If a task file lacks them, use `models.executor`.

## 13. Token Budget (Per Agent Session)
- Target: a session sits around **200k tokens**. Above **300k** is classified as **over** (`token_budget` in `config.yaml`).
- Work is sized to fit, not squeezed in:
  - The **Plan Estimator** splits a spec into plans so one Planner session fits the target (rule 14).
  - A task is sized so one Executor session fits well under the target (typically one concern, a handful of files).
  - Agents read narrowly: use a Scout (Haiku) for lookups instead of reading large files or directories, read line ranges, never re-read what is already in context.
- An agent that approaches 300k MUST stop expanding scope: write what is done and what is left into the plan folder and `HANDOFF.md`, report, and let a fresh agent continue. It never silently degrades.

## 14. Plan Estimation, Parallel Planners & Plan Audit
- After a spec is approved and audited, the **Plan Estimator** runs first. It sizes the spec and writes `docs/plans/<spec-name>-ESTIMATE.md`: how many plans, each plan's scope, rough token cost, dependencies between plans, and a **Shared Contract** (file paths, exported names, signatures, data shapes that more than one plan touches).
- One **Planner** is then spawned per plan, in parallel where plans are independent. Each plans only its slice and writes code against the Shared Contract.
- Planners run in parallel and coordinate only through the Shared Contract and `## Cross-Plan Assumptions` in each `00_OVERVIEW.md`; nothing blocks on another planner. Parallel planners do not write `HANDOFF.md` or `TODO.md`: the orchestrator and the Plan Auditor do (proto-spec candidates live in each overview until the auditor merges them).
- Later plans often depend on code that does not exist yet. The **Plan Auditor** therefore runs last, reads every plan's interface ledger, and aligns all plans and tasks (names, signatures, ordering, file ownership, test harness, lint config), editing task files in place and writing `docs/plans/<spec-name>-PLAN-AUDIT.md`.
- Execution MUST NOT start until the audit verdict is `ALIGNED` for the plan being run.

## 15. Code Standards & Linters
- All code follows one machine-enforced standard (`guardrails/code-standards.md`). Follow the project's existing linter/formatter; if none exists, the first plan creates the default for the stack from `config.yaml` `standards:` (JavaScript/TypeScript: Biome).
- Standards live in `docs/knowledges/code-standards.md` and grow with every plan, preferably as lint rules.

## 16. Test-First With a Harness
- Tests and the test harness come before implementation in every task: harness, failing tests (red), implementation, passing tests (green). See `guardrails/testing-verification.md`.
