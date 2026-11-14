# Social Center (legacy, local only)

Summary of the old repo's `services/social-center/README.md`. It is a reference for `frontend-admin` and `backend-admin`, not code in this repo.

## What it is

- UI: Next.js / React / TypeScript on the owner's Mac (port 3000).
- API: FastAPI + SQLite (port 8787). Postgres was never set up.
- Reads Meta Graph API through Facebook Login, or CSV exports from Meta Business Suite.
- Token stored in macOS Keychain (service `maemorkratai.meta`), never in files, URLs or logs.
- Read-only: no posting, no comment replies, no private messages.

## Connecting an account

- A project-only Meta app with Facebook Login. The account needs page tasks ANALYZE/MANAGE; Instagram must be Business/Creator and linked to the page.
- Starting permissions: `pages_show_list`, `pages_read_engagement`, `read_insights`, `instagram_basic`, `instagram_manage_insights`. Comments may need `instagram_manage_comments` / `pages_read_user_content`.
- The user enters the API version and token in the UI. The server checks `/me/permissions` and `/me/accounts` before storing, then stores a page token after the user picks the page.
- API version has no default; metric names are configurable "candidate probes". Unsupported metrics are reported as `unsupported`, never as zero.

## Data collection

- Initial backfill pages through posts/media, Facebook video reels, comments and replies the API allows.
- Cursors saved in SQLite; max 500 requests per platform per run; up to 3 retries on rate limits.
- After backfill: refresh the last 90 days of posts, comments and account insights.
- Metrics stored with native name, unit, period, start/end, source ID, observed time; lifetime / day / total_over_range / snapshot kept apart.
- 30-day comparisons only when daily buckets are complete or totals match the range exactly. Day boundaries in Asia/Bangkok.
- Followers are snapshots; no invented follower history; reach is not summed as unique people.
- Raw snapshots are append-only (new UUID file each time); tokens stripped from stored responses.
- Comments stay local; reports contain counts and themes only, never individuals.

## CSV import

- UTF-8, UTF-16 or Thai cp874; comma, semicolon or tab.
- User maps columns; missing columns are never filled with zero.
- Types: posts/reels, daily overview, comments. ISO dates (`YYYY-MM-DD`); ambiguous slash dates rejected.
- `watch_time` in CSV is seconds; API `ig_reels_avg_watch_time` is milliseconds.
- Re-importing the same file + mapping + scope adds nothing.

## Jobs

- A macOS LaunchAgent runs a daily sync at 08:00 Asia/Bangkok, with catch-up if the Mac starts later.
- Tests use a temporary database and a fake transport.

## Takeaways for the new admin backend

- Keep: native metric metadata, append-only raw snapshots, no zero-filling, comment privacy, idempotent imports.
- Change: multi-user auth, server-side encrypted tokens, OAuth callback instead of pasted tokens, a real job scheduler, Postgres.
- New: competitor tracking (proposed tables `competitors`, `social_accounts`, `social_posts`, `social_metric_observations`, `social_comment_themes` in [`schema-proposal-v0.1.dbml`](schema-proposal-v0.1.dbml)).
