---
model: sonnet  # haiku | sonnet. Never opus: a task too hard for sonnet must be split or written out more explicitly
effort: medium # low | medium
reason: "<one line: why this tier, e.g. exact code supplied, mechanical edit>"
---

# Task <NUMBER>: <Task Name>

- **Executor / Coder Rule**:
  1. ALWAYS read [00_OVERVIEW.md](./00_OVERVIEW.md) first before starting execution.
  2. Execute ONLY this single task. When done and verified, terminate so a fresh Executor / Coder agent handles the next task.
  3. Apply exact code below without assumption or improvisation. Order: harness, tests, red, implementation, green, standards check.

## 1. Action Items
- [ ] Create / Update `<path/to/file>`
- [ ] Run tests (red), implement, run tests (green), run the standards check

## 2. Harness (fixtures, fakes, helpers this task adds; write "none" if none)

### File: `<path/to/harness/file>`
```<lang>
// Exact full code
```

## 3. Tests (written first; from the spec's GIVEN-WHEN-THEN, include failure and security cases)

### File: `<path/to/test/file>`
```<lang>
// Exact full test code
```

## 4. Red (run before implementing)
```bash
<test command>
```
Expected: FAIL, because `<exact expected failure message>`.

## 5. Implementation (exact code, written to pass the project's linter/formatter)

### File: `<path/to/file>`
```<lang>
// Exact full code goes here
```

## 6. Green + Standards Check
```bash
<test command>
<code-standards check command, e.g. biome check .>
```
Expected: all tests PASS, check reports no errors.
