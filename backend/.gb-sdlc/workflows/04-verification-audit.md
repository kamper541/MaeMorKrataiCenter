# Workflow 04: Verification, Security Audit & Plan Archival

## Objective
Perform complete functional verification and an OWASP Top 10 security audit before archiving the plan.

## Step-by-Step Flow
1. **Verification**:
   - Run tests and verify all acceptance criteria from the spec.
   - Check compliance with all human decisions in `docs/decisions/`.
2. **Security Audit**:
   - Run OWASP Top 10 audit checklist from `security-owasp.md`.
3. **Branch on Outcome & Handoff**:
   - **FAIL**:
     - Create `failure-feedback.md` in the plan directory.
     - Update `HANDOFF.md` (Project Status: `BLOCKED`, Next Step: Route to Planner for re-planning).
     - Route back to **Planner** (Workflow 02).
   - **PASS**:
     - Write `security-audit.md` in the plan directory.
     - Move `docs/plans/<plan-name>/` to `docs/plan-records/<plan-name>/`.
     - Log any non-blocking security or performance suggestions into `TODO.md` as Proto-Specs.
     - Update `HANDOFF.md` (Project Status: `IDLE`, update What We Have in Current, Next Step: Ready for next feature spec).
     - Notify human developer that feature is verified and ready for deployment.
