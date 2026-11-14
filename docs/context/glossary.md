# Glossary

## Project terms

| Term | Meaning |
|---|---|
| **MaeMorKratai** (แม่หมอกระต่าย) | The brand: "the rabbit fortune teller". Also the mascot character. |
| **The reader** / แม่หมอ | The real tarot reader (the owner's partner). Source and approver of all fortune-telling knowledge. |
| **Owner** | Runs the project and the frontend. Also the `owner` role in the workshop. |
| **Workshop** | Legacy app where the reader answered 60 practice cases. Source of training/evaluation data. |
| **Case** | One practice question with context and cards, in the workshop. |
| **Baseline** | The first reading of a case, with 3 cards (past / present / future). Must not see the 4th card. |
| **Follow-up** | The second stage of a case: the 4th card is revealed and expands the future. |
| **4th card / expansion card** (ใบขยาย) | Extra card that expands the "future" position. Planned for the paid reading. |
| **Development set** | 40 cases allowed for building rules, style and examples. |
| **Holdout set** | 20 cases kept private and used only for final evaluation. |
| **Knowledge release** | A versioned, reader-approved set of rules and texts the engine may use. |
| **Engine A / B** | A = rules + templates. B = same evidence, rewritten by an LLM. |
| **Daily reading** (ดวงรายวัน) | Free single-card reading on `/`. |
| **Three-card spread** (ไพ่ 3 ใบ) | Past / present / future on `/reading`. Demo today. |
| **Siamsi** (เซียมซี) | Thai fortune sticks. Proposed future service in the old schema. |
| **Social Center** | Legacy local dashboard for channel analytics (Meta APIs). |
| **Content Studio** | Planned dashboard area to create tarot video series. |
| **Satang** | 1/100 THB. All money is stored as integer satang. |

## Card IDs

78 cards, kebab-case English names, the same everywhere (frontend data, art filenames, proposed `cards.id`):

- Major Arcana: `the-fool`, `the-magician`, `the-high-priestess`, … `the-world`
- Minor Arcana: `ace-of-cups` … `king-of-pentacles` (suits: `wands`, `cups`, `swords`, `pentacles`)

Art file names add a content hash: `public/cards/v3/the-fool-e11e1ee168a4.webp` (see `public/cards/v3/manifest.json`). All cards are upright only for now (`orientation: "upright_only"`).

## Daily reading fields (`frontend-client/src/lib/daily-cards.json`)

| Field | Thai label | Meaning |
|---|---|---|
| `theme` | ธีม | One-line theme of the day |
| `overview` | ภาพรวม | General reading |
| `work` | การงาน | Work |
| `money` | การเงิน | Money |
| `love_by_status` | ความรัก | Love text per relationship status (below) |
| `rabbit_message` | ข้อความจากแม่หมอ | Message from the rabbit |
| `colors` | สีมงคล | Lucky colours |
| `affirmation` | คำยืนยัน | Affirmation |

## Relationship statuses

| Key | Thai (as shown in UI) | Meaning |
|---|---|---|
| `single` | คนโสด | Single |
| `crush` | แอบชอบ | Has a secret crush |
| `talking` | คุยกัน | Talking / getting to know someone |
| `partnered` | มีแฟน | In a relationship |
| `ex_partner` | คนรักเก่า (daily page) / เลิกกัน (3-card page) | About an ex ("old lover"). The source text is written for คนรักเก่า; do not silently treat it as "broken up". |
| `complicated` | ซับซ้อน | Complicated. Missing for the 22 Major Arcana. |

`null` means the source has no text. It is not permission to generate one.

## Three-card topics (`src/lib/reading.ts`)

| ID | Thai | Situations |
|---|---|---|
| `love` | ความรัก | the six relationship statuses |
| `work` | การงาน | current job, looking for a new job, own business |
| `money` | การเงิน | income and spending, saving plan, long-term goals |
| `life` | ภาพรวมชีวิต | going through change, a decision to make, self-reflection |

Positions: `อดีต` past, `ปัจจุบัน` present, `อนาคต` future. The workshop also uses a `health` topic.
