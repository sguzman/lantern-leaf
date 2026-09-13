# 0014 — Reader media-sizing anchor stability and visible control effect

## Outcome

Make live changes to Presentation `media max width` / `media max height` controls behave predictably on real EPUBs: changing media constraints must visibly resize eligible inline media when the constraint is active and must not throw the reader viewport to an unrelated location.

## Starting evidence

Goal 0012 A6 added automated aspect-ratio/max-size evidence and Goal 0012 ultimately passed its focused real-desktop reader-presentation signoff. During that final physical pass, however, the user could not verify the media max-width/max-height controls because changing them caused a violent/unexpected scroll jump and the visible image appeared unchanged.

Treat this as a separate interactive geometry/anchoring follow-up. Do not reopen Goal 0012 or discard its accepted presentation, image, font, and TTS work.

## Scope

When promoted:

- reproduce live max-width/max-height changes around a currently visible inline image;
- determine whether the apparent no-op is caused by the image already being below the selected bound, stale media geometry, control wiring, or another production defect;
- preserve a stable reading/media anchor across media-geometry invalidation instead of jumping to an unrelated document position;
- ensure an actually binding max-width or max-height change produces an observable image-size change while preserving aspect ratio;
- preserve bounded virtualization and off-render-thread image loading/decoding;
- preserve canonical sentence identity, TTS highlight/follow, and all Goal 0012 accepted presentation behavior;
- add production-adjacent regressions for geometry invalidation plus viewport-anchor continuity around inline media.

## Acceptance gates

1. On a representative real EPUB image, reducing a binding media max-width visibly reduces rendered width while preserving aspect ratio.
2. Reducing a binding media max-height visibly reduces rendered height while preserving aspect ratio.
3. Changing either control while the image is visible does not jump the viewport to an unrelated location.
4. Geometry invalidation cannot leave stale image/block measurements driving ordinary render-window selection.
5. Non-binding limits may legitimately produce no visual size change, but the implementation/tests make that distinction explicit rather than silently masking a wiring defect.
6. Inline images, Presentation controls, Goal 0008/0009 TTS semantics, and Goal 0012 highlight/follow behavior remain green.
7. Heavy image or document work remains off the egui/render thread.

## Non-goals

- starter-shell panel containment (Goal 0013);
- Caliberate catalog cover loading/provider UX (Goal 0010);
- broad reader presentation redesign;
- Windows Natural/HD voices;
- PDF work.

## Priority

Queued after Goal 0013. Re-evaluate against Goal 0010 once starter-shell containment is accepted; this is intentionally a narrow residual follow-up rather than a reason to keep Goal 0012 open.
