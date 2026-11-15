# Guardrail 02: Security Priority #1 & OWASP Top 10 Compliance

## 1. Security as the Absolute Priority
In every architectural decision, Battleground debate, and code implementation, **Security is Priority #1**. Speed of delivery and convenience never justify compromising security.

## 2. OWASP Top 10 Mandatory Audit Checklist
Before any plan is handed over, the **Verifier Agent** must audit the changes against these 10 categories:

1. **A01: Broken Access Control**
   - Enforce least privilege. Verify RBAC/authorization on every endpoint, handler, and state transition.
   - Deny by default.
2. **A02: Cryptographic Failures**
   - Never hardcode secrets, API keys, or private tokens in source code or specs.
   - Use strong algorithms and secure key management (environment variables, vault).
3. **A03: Injection (SQL, NoSQL, Command, Template, XSS)**
   - Parameterize queries. Validate, sanitize, and escape all user input.
   - Never concatenate raw strings into shells or database queries.
4. **A04: Insecure Design**
   - Threat-model business logic. Prevent race conditions and state bypasses.
5. **A05: Security Misconfiguration**
   - Disable debug modes, directory listings, and default credentials in production configurations.
   - Set restrictive CORS and CSP headers.
6. **A06: Vulnerable and Outdated Components**
   - Verify that new dependencies have no known high/critical CVEs.
7. **A07: Identification and Authentication Failures**
   - Enforce strong session management, token expiry, and rate limiting against brute force.
8. **A08: Software and Data Integrity Failures**
   - Validate checksums, signatures, and untrusted deserialization.
9. **A09: Security Logging and Monitoring Failures**
   - Log authentication events, authorization failures, and boundary errors without logging sensitive PII or credentials.
10. **A10: Server-Side Request Forgery (SSRF)**
    - Validate and whitelist URLs before making backend HTTP requests.

## 3. Verifier Gate Requirement
- If any OWASP Top 10 issue or high-severity vulnerability is detected, the Verifier MUST emit a `FAIL` status with detailed remediation feedback back to the Planner.
