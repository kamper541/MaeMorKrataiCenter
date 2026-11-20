# CLAUDE.md

This project follows the `gb-sdlc` Spec-Driven Development framework. The master instructions are shared across all AI coding agents (Claude Code, Antigravity, Cursor) and live in `GEMINI.md`; they are imported below so there is a single source of truth.

@GEMINI.md

## Claude Code Notes

- Read `HANDOFF.md` first at the start of every session and update it before finishing.
- Agent personas (Spectre, Researcher, Planner, Executor, Verifier) are prompt presets in `.gb-sdlc/agents/`; adopt them by reading the file, or run them as subagents via the Agent tool, giving each the preset file path.
- When the user asks to plan an approved spec ("plan out the spec", "plan docs/specs/x.md"), do not plan it in this chat. Run the `/gb-plan` procedure from `.claude/skills/gb-plan/SKILL.md` (use the complex/deep variant only if the spec's `complexity` says so): estimator, parallel planners, then plan auditor. The same applies to "execute the plan" (`/gb-execute`) and "verify the plan" (`/gb-verify`).
- Read active skills listed in `.gb-sdlc/config.yaml` before planning or writing code.
- Add project-specific Claude Code guidance (build/test commands, conventions) below this line.
