# Plan Estimate: <spec-name>

- **Spec**: [<spec-file>](../specs/<spec-file>.md)
- **Created**: <YYYY-MM-DD>
- **Recommended planner tier**: sonnet/high | opus/medium | opus/high (human confirms)

## 1. Sizing
- Deliverables found: <modules / endpoints / screens / migrations / harness>
- Token budget: target ~200k per session, over at 300k.

## 2. Plans
| # | Plan name | Scope (what it owns) | Est. planner tokens | Est. tasks | Depends on (execution) |
| :--- | :--- | :--- | :--- | :--- | :--- |
| 1 | `<spec>-p1-<slug>` | <scope, includes standards + harness bootstrap if the project has none> | ~<N>k | <n> | none |
| 2 | `<spec>-p2-<slug>` | <scope> | ~<N>k | <n> | p1 |

## 3. Shared Contract
> Single agreed definition for anything more than one plan touches. Planners treat this as law.

| Item | Path | Name / signature / shape | Owner plan | Used by |
| :--- | :--- | :--- | :--- | :--- |
| <symbol, table, route, config key> | `<path>` | `<signature>` | p1 | p2, p3 |

## 4. Notes
- <repo findings from scouts, risks, anything the human should adjust before planners run>
