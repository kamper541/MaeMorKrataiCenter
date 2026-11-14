# 06 · Admin dashboard and Content Studio

## Owner goal

Run the tarot website and the content channels from one dashboard. Later: pick a tarot series / episode / style / character, start video generation, watch the preview, fix single shots, and approve publishing, all from the dashboard. Today the video pipeline runs from local tools; the dashboard that drives it is new work.

## Proposed dashboard menu (`frontend-admin`)

| Menu | Data / actions |
|---|---|
| Overview | Questions, pending work, sales, credits, real cost by period |
| Readings and escalations | Trace of cards / positions / knowledge version; reader takes over cases |
| Knowledge library | Original vs edited, approvals, releases, evaluation results |
| Orders and credits | Provider events, ledger, refunds, reconciliation |
| Competitor research | Collect competitor clips, extract ideas (hook, topic, format), track trends |
| Content Studio | Series, episodes, scripts, storyboards, templates, start jobs, preview, approve |
| Asset library | Cards, characters, voices, music, fonts; rights, versions, hashes |
| Jobs and costs | Queued / running / failed / retry, provider IDs, reserved budget, actual spend |
| Channels and analytics | Connected social accounts, post performance, reports (Social Center was the local start) |
| Settings / audit | Members, roles, provider credentials, limits, budgets, change log |

## Existing video pipeline

Example: `animations/TheFools_Type/funmaxxing-expression-cut-v1` in the old repo has Blender scripts, timing JSON, mouth cues, caption overlay and ffmpeg compose. It makes a 61 s vertical clip locally, but references the owner's asset paths and is not a service. GLB/.blend files, real voice audio, video and licensed fonts are not in Git.

**Pilot with one existing template:** input = approved script + chosen voice + asset manifest; output = MP4 + preview + QA manifest. Add automatic image / 3D / video generation later; those vary in quality, time and price.

```mermaid
flowchart LR
    Brief[Pick series / episode + brief] --> Script[Script + knowledge references]
    Script --> Approve[Owner approves script and budget]
    Approve --> Job[Create job + reserve budget]
    Job --> Assets[Pick assets / generate when approved]
    Assets --> Audio[Voice / timing / alignment]
    Audio --> Shots[Scenes / camera / expressions / mouth cues]
    Shots --> Render[Render per shot]
    Render --> Compose[Captions / music / SFX / end card / compose]
    Compose --> QA[Check duration / frames / decode / source hashes]
    QA --> Preview[Owner previews and fixes shots]
    Preview --> Publish[Publish approval, separate from generation]
```

## Job API (proposal; contract goes in `docs/api/admin/`)

- `POST /admin/content-projects/:id/jobs`: create a job from a config version and asset IDs the server has authorized. Idempotency key required. Never accept credentials or shell commands from the browser.
- `GET /admin/jobs/:id`: phase, progress, current shot, preview URLs, estimated and actual cost.
- `POST /admin/jobs/:id/retry`: retry only failed steps, reusing artifacts and provider job IDs.
- `POST /admin/jobs/:id/approve`: store approver, asset versions and approved budget. Publishing is a separate permission and command.

The worker accepts typed config only and never executes user-entered code. Allowlist templates and providers; validate input and output.

### Queue, state, recovery

- States: `draft → awaiting_approval → queued → running → awaiting_review → approved → published`, plus `failed`, `cancelled`, `reconciling`.
- Store attempt, heartbeat, lease, cancellation, checkpoint and per-shot artifacts. Messages can arrive twice: claim with a transaction/lease and check step output before redoing work.
- If a paid provider accepted a job but the response was lost, **do not resubmit**. Store the provider request ID / idempotency key and reconcile first.
- Track reserved budget and actual cost separately. Quote and get approval before spending provider credits. Spend caps per job, per day, and on concurrency.

### Assets and reproducibility

Store content hash, source and rights, model/provider/version, prompt/config, seed, voice ID, shot timing, parent artifact and review. Never reuse old mouth cues with changed audio without re-timing. Editing one shot must show which outputs need re-rendering and which can be reused. Keep raw / edited / approved as separate versions.

### Worker

Start with the Mac as a worker that pulls jobs using a limited-scope credential. Never open a control port on the Mac to the internet. After measuring one template, choose VM / container / GPU and storage. A dependency and asset manifest that reruns on a fresh machine is required before calling cloud rendering ready.

## Social Center → admin backend

Moving Social Center's SQLite and Meta OAuth to a server is its own project: auth, token encryption, OAuth callback, Meta permission review. See [`reference/social-center.md`](../reference/social-center.md) for what the local version does today.
