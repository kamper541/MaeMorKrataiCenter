# 05 · Reading engine

Condensed from the old repo's `backend/READING_ENGINE_PLAN.md` (2026-10-26). Nothing here is built. No model has been trained or called for customers.

## Core proposal

Start with **reader-approved knowledge + reading rules + short style examples + an LLM that only rewrites**, and compare it against pure rules. Do not start with fine-tuning or a local LLM machine. Let evaluation decide what AI is needed.

Putting examples in a prompt or retrieving knowledge for the model is not fine-tuning; it does not change model weights.

## Source data (workshop, 2026-10-26)

| Item | Finding | Meaning for the engine |
|---|---|---|
| Current readings | 60/60 cases in `review` | Data entry is done |
| Topics | love 30; work, money, health 10 each | Evaluate per topic; love dominates the total |
| Text | 57 cases have text before and after the 4th card (114 parts) | Can be normalized right away |
| Audio | 3 cases are audio for both stages: 6 files, 479 s | Only these need transcription |
| History | 120 revisions | Not 120 independent cases; baseline and follow-up of a case stay together |
| Split | development 40 / holdout 20; related cases never cross splits | Keep the split. Holdout never feeds rules, style, or retrieval |
| Card coverage | 66/78 cards appear overall; 59/78 in development | Not every card, and appearing once ≠ a rule for every position/topic |
| Duplicates | `case-15` and `case-16` baselines are identical | Ask the reader; never auto-fix |
| Extra fields | evidence / uncertainty / question / phrases filled in only a few cases | Reasons may be in the main text or audio; extract, then ask only where unclear |

## Pipeline

```text
question + context + cards/positions
  → check scope and missing info
  → look up reader-approved rules/knowledge valid for this case
  → assemble the reading content from evidence
  → A: fill a template, or B: LLM rewrites per the style guide
  → verify output and evidence → answer / ask to clarify / send to the reader queue
```

### Proposed contract (design target, not implemented)

```ts
type ReadingDecision = {
  status: 'answered' | 'clarify' | 'needs_owner' | 'out_of_scope';
  answer: string | null;
  clarification: string | null;
  evidenceIds: string[];
  knowledgeVersion: string;
  styleVersion: string;
};

// generateReading({ question, category, intent, context, horizon,
//   stage, cards: [{ id, position, orientation }], baselineReading? })
//   -> ReadingDecision
```

### Rules

- **Data prep:** back up text, history, audio and manifest/hash before transforming. One current reading per reader per case (`workshop_readings`); history stays history. Link to the case by ID and version. Keep original audio, raw transcript and corrected text as separate versions.
- **Fields per knowledge item:** `source_case_id`, `source_revision`, `case_version`, `stage`, `split`, `source_type`, `audio_ref`, `review_status`, `approved_by`, `approved_at`, `version`.
- **Separate method from voice** (development set only):
  - *Interpretation:* meaning by card / position / topic / condition, how cards combine, how the 4th card expands the future, what must not be concluded.
  - *Style:* openings and closings, pronouns, length, how decisive, words used or avoided, how advice and follow-up questions are phrased.
  - *Reference examples:* question + context + cards + approved reading, with source.
- Everything extracted is a **proposal until the reader approves it**. Never invent a rule from a short answer and label it the reader's knowledge.
- **Lookup first** by topic / intent / card ID / position / condition. No vector DB, embedding service, LangChain or local LLM server until tests show lookup misses same-meaning questions.
- **The LLM only rewrites** allowed content. It must not produce fortune-telling conclusions from its general knowledge. Prompts alone do not guarantee this; verify and fall back.
- Never return an old answer just because the question is similar or 1–2 cards match. Same cards + same context should give the same substance (no random variation to look clever).
- Ambiguous input → `clarify`. In scope but not enough evidence → `needs_owner`. Outside the service → `out_of_scope`. Never rely on the model's self-reported confidence alone.
- The server validates card IDs, counts and positions. The **baseline must not contain any 4th-card info**. The follow-up expands the future and never rewrites the original reading.
- Verify schema, evidence IDs in the approved set, rule conditions, over-claiming, contradictions and banned phrases. Having an evidence ID does not prove the text matches it.
- LLM timeout / failure / failed checks → use A only if evidence is enough; otherwise send to the reader queue, with status visible to the user.
- **Frontend hook point:** `frontend-client/src/lib/reading.ts` `createReading()` (returns `status: 'demo'` today) and `src/app/reading/page.tsx`. The real function must call a server API. Never ship service or model keys to the browser.

## A/B evaluation

- **A:** rules + templates from approved knowledge, no LLM.
- **B:** same evidence as A, plus an LLM rewriting with the style guide and development examples. Check model ID, account access, price and Thai quality before trying; set request and spend caps.
- Record model / prompt / knowledge / style version, latency, tokens and real cost.
- Blind review: the reader sees the question, cards and two answers in random left/right order, picks A / B / tie / both fail, and edits sentences. Scores: matches method, sounds like the reader, answers the question, no over-claiming, 4th card linked correctly. Accuracy before language preference.

## Holdout test and launch scope

- The 20 holdout cases stay out of retrieval, prompt examples, style extraction and rule extraction, directly and indirectly.
- Evaluate holdout only after method, prompt and knowledge version are locked. If holdout errors are used to fix the system, say so and build a new test set before claiming "unseen" results.
- Add cases for out of scope, ambiguous questions, missing knowledge, prompt injection, conflicting info, and opening the 4th card early.
- Pilot target (to confirm, not a result): no serious errors (wrong card/position, 4th card seen early, invented personal events, out of scope), reader accepts ≥ 18/20 holdout, average style score ≥ 4/5.
- Report answered / clarify / sent-to-reader rates separately from answer quality, so the system cannot "pass" by sending everything to a human. Report per topic.
- Health questions are never interpreted as a diagnosis; define scope for sensitive questions with reader-approved wording first.
- Ship B only where it wins on language without losing accuracy, at acceptable cost and latency. Have feature flags, rate limits, timeouts, answer/version history and the reader queue ready before beta.

## Why not fine-tuning yet

What is missing is approved knowledge, explainable rules and evaluation. Fine-tuning may help style later, but it does not add knowledge the reader has not given and does not guarantee correct refusals. Consider training only when retrieval and knowledge are proven correct but style or behaviour errors persist after prompt and example tuning, and there is enough approved data and a fresh test set to measure it.
