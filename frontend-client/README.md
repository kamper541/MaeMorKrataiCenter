# frontend-client

Customer-facing tarot website for MaeMorKratai. **Pure UI**: no API calls, no environment variables, no backend.
Card and reading data are static JSON files in `src/lib/`, taken from the live site at https://mae-mor-kratai.vercel.app/.
When the backend is ready, replace those JSON imports with calls defined in [`docs/api/client/`](../docs/api/client).

User-facing copy and card content are in Thai on purpose. Code, comments and docs are in English.

## Run

Requires Node.js 20.9 or newer (24 LTS recommended).

```sh
npm ci
npm run dev        # http://127.0.0.1:3010/
npm run typecheck
npm run build      # static export to out/
```

## Structure

| Path | Contents |
|---|---|
| `src/app/page.tsx` | `/`: daily single-card reading |
| `src/app/reading/` | `/reading`: three-card spread (demo) |
| `src/lib/` | Card data (`cards.json`, `daily-cards.json`, `reading-cards.json`) and motion helpers |
| `public/cards/` | Card art (`v3/` is the current 78-card deck) |
| `public/scene/` | Animated MaeMorKratai room, loaded in an iframe, plus shuffle videos |
| `public/card-scenes/` | Illustrated scenes for the original five pilot cards |
| `docs/` | Design notes and card image sizes |

## Origin

Split from `MaeMorKratai/web/tarot` (commit `a1d6fef`) on 2026-11-14. Python QA/ingest scripts, launch reports and local `.env` files were left out.
