# Code Standards (Living)

> One way of writing code in this project, enforced by tools. Agents follow this file; the first plan creates the toolchain if it is missing (guardrail 05). Prefer adding a lint rule over adding prose.

## 1. Stack & Toolchain
| Stack | Linter / formatter | Type check | Check command |
| :--- | :--- | :--- | :--- |
| <e.g. TypeScript> | <e.g. Biome> | <e.g. tsc --strict> | `<e.g. biome check . && tsc --noEmit>` |

## 2. Test Harness
- Runner: `<tool>`; command: `<test command>`
- Shared fixtures / factories / fakes: `<paths>`
- Layout: `<where unit / integration / contract tests live>`

## 3. Conventions
- Naming: <files, symbols, tests>
- Structure: <folder layout, module boundaries>
- Errors: <error types and how they propagate>
- API / data shapes: <response envelope, validation approach>

## 4. Learned Rules
> Added by each plan's documentation task. Note whether a lint rule now enforces it.
- <YYYY-MM-DD> <plan>: <rule> (enforced by: <lint rule | review only>)
