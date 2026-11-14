# 02 · Current state

Facts as of 2026-11-05 (old repo), with folder paths updated to this repo on 2026-11-14.

## 1. Customer website: live

- URL: https://mae-mor-kratai.vercel.app/
- Source now: `frontend-client/` (was `web/tarot` in the old repo)
- Next.js 16.4.2, React 19, TypeScript, static export on Vercel. GSAP and Three.js for the scene and card picking.
- **`/` daily reading:** draws 1 of 78 cards with WebCrypto in the browser. Shows overview, work, money, love by relationship status, a message from the rabbit, lucky colours and an affirmation. The text was reviewed by the reader on 2026-11-05.
- Data: `src/lib/daily-cards.json` (published text), `src/lib/reading-cards.json` and `public/cards/v3/manifest.json` (art). No LLM is called.
- Art ready for 74 of 78 cards. Missing: Six of Wands, Nine of Wands, Page of Pentacles, King of Pentacles.
- The 22 Major Arcana have no "complicated relationship" text in the source; the site says so instead of inventing one.
- **`/reading` three-card spread is a demo.** `src/lib/reading.ts` `createReading()` returns `status: 'demo'`, reuses old love meanings, and other topics show placeholder text. There is no free-question engine and no 4th card on the customer site.
- No accounts, history, orders, credits or payments on this site.

## 2. Workshop (reader data collection): live, legacy

- URL: https://maemorkratai-workshop.vercel.app/
- Source: old repo only, `web/workshop-standalone`. **Not moved into this repo.**
- Next.js full stack + Supabase Auth, Postgres and private Storage. Email/password login, active membership, roles `owner` and `reader`.
- Catalog of 60 cases in 4 topics. Flow per case: baseline 3 cards → seal → follow-up with the 4th card → submit for review, with text or audio answers and revision history.
- `/review` lets the owner check, edit and approve text, with history in a separate private bucket.
- Data status (2026-10-26): all 60/60 cases submitted; split into development 40 / holdout 20; 6 audio files transcribed locally. **The latest approved count was not re-checked**; confirm with the owner before building the engine.
- Approving sample cases in `/review` does not create general card rules and does not publish anything to customers.
- Schema: [`reference/workshop-migrations/`](../reference/workshop-migrations). Real answers and audio are not in any repo.

## 3. Dashboard and video: local only

- **Social Center** (old repo `services/social-center`): Next.js UI on port 3000 + FastAPI on port 8787 + SQLite. Reads Meta (Facebook/Instagram) APIs or CSV exports to show channel stats and build reports. Token in macOS Keychain. Single-user, localhost only. Summary: [`reference/social-center.md`](../reference/social-center.md).
- **Video pipeline:** Blender, ffmpeg, Rhubarb (lip sync), HyperFrames/GSAP captions, and the reader's chosen voice. Latest example: a 61 s vertical clip, 1080×1920 at 30 fps. Runs from local scripts with local asset paths.
- Not built yet: video job API, job queue, cloud render worker, cost tracking, generation from the web, posting to channels.

## Where we are

Website and data collection are done. **Next: approve the reader's knowledge and build an engine that can be evaluated.** Close the knowledge release and the data/permissions/transaction design before a paid beta. A video pilot can run in parallel with one template, but needs job, cost and review workflow before anyone else uses it.
