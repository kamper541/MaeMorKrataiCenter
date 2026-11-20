# Guardrail 05: Code Standards, Linters & One Way of Writing Code

## 1. Principle
Free-style coding is a defect. Every project has one machine-enforced standard, so any agent or human produces code that looks the same. Prefer a linter or formatter rule over a prose rule: a tool fails the build, prose gets ignored.

## 2. Source of Truth
- `docs/knowledges/code-standards.md` is the living standards file: stack, toolchain, commands, naming, structure, error handling, test layout, and the rules learned from built code.
- `.gb-sdlc/config.yaml` `standards:` holds the default toolchain per stack (TypeScript/JavaScript use **Biome**, Python uses ruff, and so on). It applies only when the project has nothing yet.

## 3. Rules
1. **Follow, else create.** If the project already has a linter/formatter, follow it and never replace it silently. If not, the first plan MUST bootstrap the default for each stack in use, as its Task 01 (see `planner.md` section 3c).
2. **One command.** The project exposes a single check command (for example `biome check .`) and records it in `docs/knowledges/code-standards.md` and `docs/architectures/tech-stack.md`. Every task verification and the Verifier run it.
3. **Planners write code that passes the linter.** Task code is written to the configured style: imports, naming, formatting, strictness. Executors run the check command on every task.
4. **No disabling.** Suppression comments (`biome-ignore`, `eslint-disable`, `# noqa`, `@ts-ignore`) are forbidden unless the task states the exact reason and line. Strictness is never loosened to make a task pass.
5. **Learn from built code.** When a plan establishes a pattern (folder layout, error type, API response shape, naming), its `NN_DOCUMENTATION.md` task adds it to `docs/knowledges/code-standards.md`, and where the linter supports it, adds a lint rule so the pattern is enforced, not just described.
6. **Conflicts.** A task that would violate the standard is a plan defect: the Planner or Plan Auditor fixes the task, the Executor never works around it.
