# Guardrail 04: Testing, Verification & Closed-Loop Feedback

## 1. Verification Standards
- Code cannot be marked verified solely because it compiles.
- Every functional requirement and acceptance scenario defined in `docs/specs/<spec>.md` MUST have corresponding test coverage (unit, integration, or contract tests).
- All human decisions recorded in `docs/decisions/D01.md` ... `DNN.md` must be respected.

## 2. Verification Protocol
1. **Automated Suite**: Run project linters, type checks, and test suites.
2. **Spec Compliance Audit**: Compare actual behavior against acceptance criteria in `docs/specs/`.
3. **Security Audit**: Perform the OWASP Top 10 check outlined in `security-owasp.md`.

## 3. Handling Pass vs. Fail
- **PASS**:
  - Verifier writes verification report.
  - Moves the plan folder from `docs/plans/<plan-name>/` to `docs/plan-records/<plan-name>/`.
  - Notifies human developer that the feature is ready for final handover.
- **FAIL**:
  - Verifier generates a structured `failure-feedback.md` detailing the failure, root cause, and violated spec/guardrail item.
  - Routes feedback back to the **Planner Agent** to update the plan and task files.
