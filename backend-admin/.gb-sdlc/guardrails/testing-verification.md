# Guardrail 04: Testing, Verification & Closed-Loop Feedback

## 1. Verification Standards
- Code cannot be marked verified solely because it compiles.
- Every functional requirement and acceptance scenario defined in `docs/specs/<spec>.md` MUST have corresponding test coverage (unit, integration, or contract tests).
- All human decisions recorded in `docs/decisions/D01.md` ... `DNN.md` must be respected.

## 2. Test-First With a Harness
Checking that the code is correct is not enough. The test setup itself must exist first and be trustworthy.
- **Harness** means everything a test needs to run: runner config, shared fixtures and factories, fakes/mocks for external systems, test database or temp-dir setup, helpers, and the one command that runs it all. The first plan bootstraps it (planner.md section 3c); later plans extend it.
- Every task with behavior is written in this order: **harness additions -> tests -> run them and see them fail for the expected reason (red) -> implementation -> run them green**. A test that never failed proves nothing.
- Task files contain the harness code, the test code, the expected red output, and the expected green output. Executors record that they saw red before green.
- Tests assert behavior from the spec's GIVEN-WHEN-THEN criteria, include failure and security cases (invalid input, unauthorized access), and never depend on test order or real network.
- A task with no testable behavior (docs, config) says so in its `Tests` section and uses the lint/type check as its gate.

## 3. Verification Protocol
1. **Harness Check**: The test command runs from a clean state. Each acceptance criterion maps to at least one test. Tests would fail if the behavior were removed (spot check by reading, or by reverting one change when cheap).
2. **Automated Suite**: Run the code-standards check command (guardrail 05), type checks, and the full test suite.
3. **Spec Compliance Audit**: Compare actual behavior against acceptance criteria in `docs/specs/`.
4. **Security Audit**: Perform the OWASP Top 10 check outlined in `security-owasp.md`.

## 4. Handling Pass vs. Fail
- **PASS**:
  - Verifier writes verification report.
  - Moves the plan folder from `docs/plans/<plan-name>/` to `docs/plan-records/<plan-name>/`.
  - Notifies human developer that the feature is ready for final handover.
- **FAIL**:
  - Verifier generates a structured `failure-feedback.md` detailing the failure, root cause, and violated spec/guardrail item.
  - Routes feedback back to the **Planner Agent** to update the plan and task files.
