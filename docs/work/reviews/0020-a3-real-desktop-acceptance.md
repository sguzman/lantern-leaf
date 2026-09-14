# Goal 0020 A3 — real-desktop acceptance

## Decision

**ACCEPTED — GOAL 0020 CLOSED**

Real Windows QA verifies the continuous native PDF viewport and practical zoom gate.

Accepted physical behavior:

- continuous wheel scrolling crosses PDF page boundaries without Next/Previous;
- adjacent page portions are simultaneously visible at seams;
- aggressive fast scrolling through the 638-page representative PDF remains extremely responsive;
- the displayed native page count/current page keeps up with scrolling;
- Previous/Next operate correctly as jumps within the continuous document;
- Fit width, Fit page, Reset/100%, and the full tested 25%–400% manual zoom range work;
- high zoom remains navigable;
- zoom approximately preserves the user's place in the document;
- representative EPUB rendering, Windows TTS, and visual controls remain green.

This closes the core Goal 0020 outcome. The remaining fit/manual transition behavior is minor polish rather than a reason to keep the continuous viewport gate open.

## Residual split

A small real-desktop UX defect remains: after entering Fit width or Fit page, pressing `+` or `-` resumes from the previously remembered manual zoom level rather than treating the current effective fit scale as the baseline for the new manual zoom step. Reset/100% followed by `+`/`-` behaves correctly.

This residual is split to Goal 0021 and must not block Gate 4.

## Cache clarification

Caliberate materialized source files remain disk-cached under LanternLeaf's Calibre download cache. In repo-native QA the configured cache root is `.qa/windows/cache`, LanternLeaf adds its `lantern-leaf` application subdirectory, and Calibre materialization uses the `calibre-downloads` subdirectory. Rasterized PDF page textures remain bounded in-memory render/cache state rather than durable page-image files.

## Next substantive gate

Gate 4 remains next: native PDF text/TTS/highlight synchronization, auto-follow/jump behavior, confidence-aware text geometry, OCR/degraded modes, and Quack-check hostile-PDF recovery as a subordinate background capability.
