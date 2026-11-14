# Design

Theme: MaeMorKratai's tarot room under the moonlight. The animated rabbit fortune teller is the central visual.

## Look

- Palette: `#1c102c` (violet-black), `#482451`, `#a77bbe` (lavender), `#ef9858` (peach/pumpkin accent), `#fff0cf` (cream text).
- Type: Georgia for the Latin wordmark, the system Thai sans-serif for UI, generous line height. Do not bundle or redistribute system fonts.
- Decoration: thin gold borders and stars. No dashboard-style widgets on the customer site.
- No strobe, neon or hard lens flare.

## Layout

- Desktop: two-column welcome, large framed animated room on the right, draw action on the left.
- Mobile: short intro, room, primary draw button, result.

## Daily reading (`/`)

1. The user picks one of six relationship contexts.
2. Draw: the rabbit shuffles in the room, holds the drawn card, and the card moves into a centered dialog with a soft ivory/peach halo (about 0.9 s).
3. Closing flips the card to the purple rabbit card back and returns it to the hand. Reopening keeps the same card and context.
4. Below the dialog, the reading shows overview, work, money, love for the chosen context, a message, lucky colour and an affirmation.

Repeat clicks are disabled while dealing.

## Three-card spread (`/reading`)

- Topic screen: conversation on the left, arched artwork with three card backs on the right; stacked on mobile.
- Table: mascot above the question, three fixed slots (past, present, future), then a scrollable curved deck with a grid alternative.
- The user confirms a raised card before it is dealt. The deck animation uses 16 visual layers; selection draws from all 78 unique cards.
- Three sequential flips, then the free demo result.

## Accessibility

Keyboard access, modal focus trap, Escape and backdrop to close, inert background while the dialog is open, safe mobile sizing, and a full reduced-motion path (fade only).
