# docs

The shared space between frontend and backend.

| Path | Contents |
|---|---|
| `api/client/` | API contract for the customer website. Use OpenAPI (`openapi.yaml`). |
| `api/admin/` | API contract for the admin dashboard and content automation. |
| `decisions/` | Decision records, named `NNNN-short-title.md` (for example `0001-auth.md`). |

## Workflow

1. Need a new or changed endpoint? Open a branch and a pull request that edits `api/`.
2. The other side reviews and approves before anyone writes code.
3. Once merged, the contract is final: frontend mocks against it, backend implements it.
