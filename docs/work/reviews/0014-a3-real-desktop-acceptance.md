# Goal 0014 A3 — real-desktop acceptance

## Decision

**ACCEPTED / GOAL 0014 CLOSED**

The final focused Windows pass establishes that the high-severity reflow defect is fixed well enough to close Goal 0014.

Physical evidence:

- presentation edits no longer produce the prior instant violent distant-area jerk/snap-back behavior;
- horizontal-margin changes now preserve the visible semantic neighborhood very well;
- the spoken/highlighted sentence remains recoverable and ordinary TTS auto-follow continues to work;
- binding media max-width and max-height behavior was physically verified in the preceding A2/A3 sequence;
- rapid live editing feels substantially calmer and more accommodating rather than repeatedly teleporting through unrelated document regions.

A residual lower-severity polish defect remains under aggressive text-metric edits, especially letter spacing and font scaling: cumulative reflow can move the highlighted sentence farther than desired and the viewport may temporarily lose its ideal relation to the highlight before ordinary auto-follow brings it back. This is not treated as a canonical-highlight identity failure; `Jump to highlight` and normal TTS follow still recover the correct canonical sentence.

This residual is real and solvable, but it is no longer severe enough to keep Goal 0014 open. It is separated as queued Goal 0015 so the project does not endlessly extend a completed semantic-anchor correction with progressively smaller perceptual polish.

## Accepted architecture

Goal 0014's accepted production shape is:

- geometry-key transitions invalidate stale measured pretty geometry;
- a last-stable semantic viewport witness survives a burst of presentation edits;
- canonical sentence/segment identity is preferred when available, with normalized same-block fallback;
- estimates are used only to keep bounded virtualization near the target neighborhood;
- measured geometry reconciles the final viewport correction;
- pending canonical TTS follow has precedence;
- explicit user scroll/drag cancels stale automatic restoration;
- heavy document/image work remains off the egui/render thread.

Implementation `430e9c1` and Windows baseline run `34768445726` remain the accepted A3 implementation/CI evidence.

## Residual disposition

Queue a separate minor reflow-polish goal for stronger viewport-band continuity of an already-visible canonical highlight during severe text-metric changes. Do not reopen Goal 0014 unless a future regression reintroduces semantic teleporting, violent unrelated-area flashes, or wrong final canonical ownership.
