# 0015 — Highlight viewport-band continuity during severe reflow

## Outcome

Polish live presentation editing so an already-visible canonical highlighted sentence remains in a stable viewport band during severe text-metric reflow, rather than drifting substantially and relying on ordinary auto-follow to snap it back afterward.

## Starting evidence

Goal 0014 is closed. A3 eliminated the prior high-severity behavior: no more instant violent distant-area flashes, horizontal-margin reflow behaves beautifully on the real desktop, semantic location is generally preserved, and canonical highlight/TTS ownership remains correct.

A lower-severity residual remains under cumulative text-metric changes such as aggressive letter spacing and font scaling. The document reflows substantially enough that the visible highlighted sentence can drift away from its preferred viewport relationship before ordinary highlight auto-follow catches it and restores visibility.

This is a perceptual/viewport-coordination polish issue, not evidence that canonical highlight identity is wrong.

## Architectural direction

Do not reintroduce permanent highlighted-target forcing.

When a canonical highlighted sentence is already visible at the start of a presentation-reflow transaction, retain its viewport-local band/relative position as a temporary transaction constraint until the new geometry settles. The transaction may use measured target geometry to make bounded corrections while the edit burst remains active, then release ownership immediately after stabilization.

Requirements:

- keep the already-visible highlighted sentence within a reasonable viewport band during severe font/letter/line metric changes where practical;
- preserve the user's semantic viewport witness when no highlight is visible/relevant;
- preserve pending TTS follow precedence;
- yield immediately to explicit user wheel/drag input;
- do not create oscillation or repeated fighting between edit anchoring and normal auto-follow;
- preserve bounded virtualization and all heavy-work-off-render-thread rules.

## Acceptance gates

1. With a visible highlighted sentence, aggressive letter-spacing and font-scale changes do not allow that sentence to wander far outside the viewport before correction.
2. The correction is transaction-scoped and releases after reflow stabilization; no permanent highlight pinning exists.
3. Idle reading without a visible highlight still preserves the ordinary semantic viewport witness from Goal 0014.
4. Active TTS follow remains authoritative.
5. User scroll/drag cancels temporary automatic ownership.
6. Horizontal-margin behavior and all Goal 0014 accepted reflow stability remain green.
7. Workspace and Windows validation pass.

## Priority

Queued as minor polish. Do not prioritize ahead of Goal 0010 Caliberate catalog covers/provider UX unless new physical evidence makes this materially disruptive in ordinary reading.
