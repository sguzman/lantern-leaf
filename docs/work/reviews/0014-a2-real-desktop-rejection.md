# Goal 0014 A2 — real-desktop rejection for final closure

Date: 2026-09-13

## Decision

**FUNCTIONAL IMPROVEMENT ACCEPTED; GOAL CLOSURE REJECTED. A3 CORRECTION REQUIRED.**

The A2 semantic-anchor work is materially successful and should be preserved. The real Windows pass verified the major behavior across presentation settings, including finally proving that binding media max-width and max-height controls visibly resize inline images. However, the remaining viewport behavior still violates Goal 0014's stability contract.

## Real-desktop evidence

Accepted from A2:

- large presentation changes generally preserve the intended semantic reading area far better than before;
- horizontal margin, font/spacing changes, and media sizing are functionally responsive;
- binding image max-width and max-height changes are now physically verified to resize media;
- canonical highlighting and Jump to highlight remain functional;
- the catastrophic persistent teleport behavior is substantially reduced.

Residual defects:

1. **Transient violent reflow excursion.** During some live presentation edits the visible document jerks to a distant area and then returns. Even when final recovery is correct, exposing an unrelated semantic area for an intermediate frame is jarring and fails the bounded-visual-displacement requirement.
2. **Occasional wrong final anchor.** Some geometry edits settle on a different semantic area rather than the area that was visible before the change. This has been observed while the highlighted sentence was visible; Jump to highlight can manually recover, which proves canonical identity remains available while viewport restoration can still choose the wrong location.

Changing visual settings should preserve where the user is reading. Reflow work is allowed; semantic teleportation and visible oscillation are not.

## Director diagnosis / likely seam

The production A2 path currently handles a geometry-key transition in one shot:

- capture an anchor from persisted `pretty_page` scroll state plus the current height table;
- clear measured block heights;
- derive a replacement offset immediately from fresh **estimates**;
- feed that estimate into `ScrollArea::vertical_scroll_offset` while the newly reflowed layout is still being measured.

That architecture can explain both residuals. Estimated cumulative heights may temporarily identify the wrong virtualized window, and rapid slider changes can cause repeated capture -> invalidate -> estimated restore cycles before the prior geometry has stabilized. A later frame may recover once measurements improve, producing the visible out-and-back jerk. Re-anchoring from an already transient/intermediate frame can also compound error and occasionally settle on the wrong location.

A2 also infers canonical sentence proximity from normalized within-block position mapped into block text. That is a useful fallback, but it is weaker than retaining the last **actually stable visible semantic witness** when one is available. In particular, if the highlighted canonical sentence was visibly on screen before reflow, a one-shot geometry transition should be able to preserve that known visible sentence without turning it into permanent auto-follow.

These are hypotheses to verify in A3, not instructions to blindly patch one line.

## A3 architectural requirement

Treat a burst of presentation edits as a **reflow transaction**, not as independent one-frame scroll resets.

- Preserve the last stable semantic viewport witness across the entire unstable geometry burst. Do not recapture the transaction anchor from an intermediate estimated/reflowing frame.
- Coalesce successive geometry-key changes while the user drags/edits controls; carry the same stable anchor forward until the new geometry settles or the user explicitly scrolls.
- Estimates may be used for bounded virtualization / locating the target region, but they must not become the final authoritative semantic restore if the relevant new geometry has not actually been measured.
- Reconcile the anchor against actual rendered/measured new geometry and preserve the anchor's viewport-local position with bounded displacement.
- Prefer an actually visible canonical sentence/segment witness when available. If the current highlighted canonical sentence is visibly present at transaction start, it may serve as the one-shot transition anchor even when no TTS follow request is pending. This must not become permanent highlighted-target forcing.
- A legitimate pending TTS follow remains higher priority than idle/edit preservation.
- User wheel/drag input during a pending transaction takes ownership and must cancel or supersede stale automatic correction rather than fight the user.
- Do not hide the problem by moving document/image work onto the egui/render thread.

## Required regressions

A3 must exercise production-adjacent state across **multiple frames**, not only pure helper arithmetic:

- geometry A -> B where estimates materially disagree with final measured heights;
- rapid A -> B -> C -> D slider/edit burst while retaining the original stable semantic anchor;
- a visible highlighted canonical sentence used as a one-shot reflow anchor without a pending follow request;
- active TTS follow precedence;
- explicit user scroll during an unsettled reflow canceling stale restoration;
- large mid-book cumulative reflow with non-uniform blocks/media;
- proof that no intermediate/final virtualized render window selects an unrelated semantic region because of stale/estimated geometry.

## Acceptance for A3

Goal 0014 closes only when a focused real-desktop pass shows that presentation edits may rewrap/recompute but do not violently flash to unrelated areas, and the final viewport remains at the same semantic reading location. Minor pixel-level movement inside the same semantic neighborhood is acceptable. Binding media controls and accepted Goal 0012 highlight/TTS behavior must remain intact.
