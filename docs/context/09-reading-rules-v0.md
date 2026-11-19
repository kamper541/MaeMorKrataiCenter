# 09 · Reading rules v0 (3+1 card spread)

Status: **draft for review** (2026-11-19, v0.2 adds answer modes). Derived from the reader's development cases 01–03 (audio answers, re-transcribed locally with Whisper large-v3). Real answers stay out of Git; this document holds only the derived rules and scores.

Applies to Engine A in [05 · Reading engine](05-reading-engine.md). Data model: [10 · Reading master data](10-reading-master-data.md).

## Why scores

The reader weighs each card by the feeling of its picture. We turn that into a number per card so the engine can combine cards the way she does, and keep every number editable by her.

## Rule 1 · Card score per category, with intent overrides

- Every card has a **score from 1 to 10** per category (`love`, `work`, `money`, `health`).
  - Meaning: how good this card is **for the person asking**. 5 = neutral, 1 = very bad, 10 = very good.
  - Always rated in this direction, even for negative questions. Rule 2 flips polarity when needed.
- Base data: 78 cards × 4 categories = **312 rows**.
- Each row also holds short texts the answer is built from:
  - `keyword` (2–4 words)
  - `meaning` (one sentence for this category)
  - `condition` (optional "if …" that unlocks the good outcome, e.g. Knight of Wands: if goals and passion match)
  - `person_trait` (optional, how the card describes the other person, e.g. The Fool: free-spirited)
- **Intent override:** some cards read differently for a specific question type. Example: Justice is generally fair/balanced, but for `love / commitment` the reader reads it as "equals, more like friends than lovers" → 5. Store an override row only where the reader disagrees with the category score.
- A missing score is never guessed. If any drawn card has no score for the category, the session goes to the reader (`needs_owner`).

## Rule 2 · Weighted total sets tone and level

```
total = Σ score(position) × weight(position)
```

| Position | Weight (v0) |
|---|---|
| past | 0.20 |
| present | 0.20 |
| future | 0.20 |
| expansion (4th card) | 0.40 |

- Weights are a starting point; tune them against all 40 development cases.
- **Polarity:** each intent declares whether a high score means "yes" (`commitment`, `reconcile`, `feelings`) or "no" (`breakup`: high score = less likely to end).
- The total maps to an **answer ladder** defined per intent. Example for `love / commitment`:

  | Total | Level | Meaning |
  |---|---|---|
  | < 4.0 | 1 | Not yet |
  | 4.0–5.5 | 2 | Moves to "serious talking", defines the relationship, not a couple yet |
  | 5.5–7.0 | 3 | Good chance to become a couple, with conditions |
  | > 7.0 | 4 | Serious, ready to go public |

Ladder thresholds and texts are master data, written and approved by the reader.

## Rule 3 · The expansion card has a role

The 4th card does not only add weight. It says what the future leads to. Compare it with the future card:

| Expansion vs future | Role | What the answer does |
|---|---|---|
| Same side of 5, at least 2 points higher | `amplify` | Raise the outcome one ladder level |
| Within ±1 | `confirm` | Keep the outcome |
| Same side of 5, at least 2 points lower | `cap` | Keep the level from the total (the low 4th card is already weighted in); phrase the future as "only up to this level" |
| Opposite side of 5 | `turn_after` | Level from the first three cards only (equal weights); answer that outcome first, then the turn from the 4th card with its condition (e.g. break up, then a chance to reconcile if the recurring fights are resolved) |
| Reader reads the 4th card as "what to do" | `advise` (v0.2) | Keep the level; add one concrete action (seen in reconcile and "will they come back" cases: e.g. Ace of Swords = say it directly) |

Order of checks: `turn_after` (crosses 5) first, then `amplify` / `cap` / `confirm` by delta. `advise` is set by the card data (the 4th card has an `advice` text for this category and intent) and can combine with `confirm`.

Ladder shift: `amplify` +1 level (max top level). All other roles: no shift. `turn_after` replaces the 4-card total with the 3-card mean.

`turn_after` is the case a pure average gets wrong, so it is decided before the total is used.

## Rule 4 · Answer structure (the reader's pattern)

Every reading the reader gave follows this order:

1. **Past** – what the relationship/situation was like (card meaning + person trait)
2. **Present** – what is happening now, with reasons as a short list
3. **Future** – the likely direction, with the card's `condition` ("if …")
4. **Expansion** – "with [card] added …" and its role from Rule 3
5. **Verdict line** – one short status per position (e.g. "past: not yet · present: not yet · future: a chance"), then the ladder answer stated plainly

Engine A fills a template with these parts. Engine B (later) may rewrite the same parts in the reader's voice, never adding new claims.

## Rule 5 · Guards

- The baseline (3 cards) never sees the 4th card. The follow-up never rewrites the past or present reading already shown.
- Same cards + same question + same context → same substance.
- No timing claims (e.g. "about 6 months") unless a reader-approved timing rule exists.
- Health questions are never a diagnosis.
- Every answer records `knowledge_version`, the scores used, the role, the total and the ladder level (trace).

## Rule 6 · Two answer modes (v0.2)

Each intent has an `answer_mode`:

| Mode | Intents | How the answer is built |
|---|---|---|
| `verdict` | commitment, breakup, reconcile, feelings, "will they come back" | Rules 2–3: weighted total → ladder level, expansion role, verdict line |
| `advice` | direction, clarify ("how should I …") | Each card's `advice` text in the category; the 4th card adds one concrete action to the position it expands (usually the future card); the answer ends with that action instead of a verdict line |

The mode follows the **form of the question** (yes/no vs how/what), not the old workshop intent label: e.g. "Will he come back?" is labelled `clarify` in the workshop but is a `verdict` question.

Scores still set the tone in `advice` mode (gentle vs firm), but no ladder level is shown. `choice` stays open (see below).

## Worked examples

Question for both: *Is he ready to get serious?* · `love / commitment` · situation `talking` (คุยกัน).

| Position | Card | Score | Status |
|---|---|---|---|
| Past | The Fool | 3 | **confirmed by the reader** |
| Present | Seven of Cups | 3 | **confirmed by the reader** |
| Future | Knight of Wands | 7 | **confirmed by the reader** |
| Expansion (case 01) | Justice | 5 (intent override) | **confirmed by the reader** |
| Expansion (case 02) | The Emperor | 9 | draft, to confirm |

- **Case 01:** total = 0.6 + 0.6 + 1.4 + 2.0 = **4.6** → level 2. Justice (5) vs Knight of Wands (7): `cap`. Answer: becomes serious talking and they discuss what they are, not a couple yet. ✅ Matches the reader.
- **Case 02:** total = 0.6 + 0.6 + 1.4 + 3.6 = **6.2** → level 3. The Emperor (9) vs 7: `amplify` → level 4. Answer: very likely a serious, public relationship. ✅ Matches the reader.
- **Counter-example:** if the cards were scored by general feeling (5, 7, 8, 9), the total is 7.6 → "yes, serious", which contradicts the reader on case 01. Scores must come from the reader, per intent.

Question: *Is this relationship likely to end?* · `love / breakup` (high score = less likely to end).

| Position | Card | Score (draft) |
|---|---|---|
| Past | The Tower | 2 |
| Present | The Moon | 3 |
| Future | Eight of Cups | 2 |
| Expansion | Temperance | 7 |

- **Case 03:** Temperance (7) vs Eight of Cups (2) are on opposite sides of 5 → `turn_after`. Answer: likely breaks up, then a chance to get back together if the recurring fights are resolved. ✅ Matches the reader. Level from the first three cards: (2 + 3 + 2) / 3 = 2.3 → likely ends; then the turn. (The 4-card average alone, 4.0, would only say "likely to end" with no turn.)

## Open questions

1. `choice` intent (A or B) cannot use a single total. Proposal: score the spread once per option, or ask the reader for a separate pattern.
2. Card pairs the reader reads together (e.g. Knight of Wands + The Emperor) – add a `card_pair_rules` table after v0 works.
3. Timing (the reader sometimes gives a time range) – no data yet.
4. Reversed cards – all cards are upright only for now.
5. Category naming: the customer site uses `life`, the workshop uses `health`. Pick one set before seeding data.

## Next steps

1. Draft the 312 category scores from the 40 development answers and the daily meanings, labelled `draft`; the reader edits them (spreadsheet first, admin page later).
2. Build Engine A and run it on the 40 development cases; compare ladder levels with the reader's answers; tune weights and thresholds.
3. Lock the version, then evaluate once on the 20 holdout cases.
