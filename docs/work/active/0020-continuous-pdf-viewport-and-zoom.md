# 0020 — Continuous native PDF viewport and practical zoom

## Current state

**REOPENED FOR A3 DIRECTOR CORRECTION — DO NOT REQUEST HUMAN QA YET**

Goal 0019 / Gate 3 remains physically accepted. Goal 0020 A1 established the continuous native page-stack direction. A2 substantially improved viewport ownership, stable native page metadata, cached geometry, horizontal navigation, fit calculations, focal anchoring, residency, and hosted validation.

Director review nevertheless found a deterministic production render-identity/lifecycle defect: the canonical scheduler and Reader presentation derive different raster widths/`PdfRenderKey`s, and zoom/fit generation changes are not guaranteed to force a new scheduler commit when visible page ranges stay unchanged.

Read:

- `docs/work/reviews/0020-a1-director-rejection.md`
- `docs/work/reviews/0020-a2-director-rejection.md`
- `docs/work/reports/0020.md`

before implementation.

## Outcome remains unchanged

Deliver a smooth continuous-scrolling native PDF viewport with practical zoom, stable current-page identity, bounded/native rendering, explicit page jumps, horizontal access at high zoom, and stable semantic viewport position across zoom/resize.

This goal remains visual only. Do not implement PDF TTS/highlight/OCR/text-selection/Quack-check integration here.

## Architecture to preserve

Preserve all accepted Goal 0019 and A1/A2 architecture:

- native Rust + `eframe`/`egui` + bundled/native Pdfium only;
- one authoritative `PdfNativeService` / one native Pdfium owner;
- all Pdfium/open/metadata/raster/decode/heavy PDF work off the egui/render thread;
- visual-first PDF open independent of transcript/OCR recovery;
- explicit native PDF page-domain ownership;
- continuous immediate-mode page stack;
- actual continuous visible pages published to the canonical viewport planner;
- native page dimensions/aspect ratios obtained off-thread before layout depends on them;
- cached/indexed long-document geometry with bounded/logarithmic ordinary visibility lookup;
- horizontal + vertical egui scrolling;
- semantic page/intra-page viewport witness capture/restore;
- broad 25–400% manual zoom ladder plus Fit width, Fit page, and Reset/100%;
- source/generation/page/render-spec stale safety;
- deterministic visible-page residency protection;
- hosted Windows validation before terminal signaling.

## A3 blocking correction

### 1. One canonical render specification

Planning, scheduling, native raster requests, texture/cache identity, and Reader presentation must use the **same** production render specification.

Do not independently calculate scheduler raster width from `pdf_viewport_width * scale` while the Reader independently calculates lookup width from `PDF_BASE_PAGE_WIDTH * effective_zoom`.

Introduce or reuse one canonical value/object for the effective raster/presentation specification, including at least what is necessary to derive the exact `PdfRenderKey`:

- source;
- generation/render-spec revision;
- page index;
- effective logical zoom/mode where needed;
- final quantized raster width/height.

The authoritative scheduler and presentation lookup must agree byte-for-byte on key identity.

Manual 100%, manual high zoom, Fit width, and Fit page must all request and consume matching raster keys.

### 2. Render-spec changes must force authoritative replanning

A stationary viewport must still rerender when its render specification changes.

Viewport commit identity must therefore account for at least one explicit geometry/render-spec revision or equivalent identity covering:

- `pdf_generation`;
- manual zoom changes;
- Reset/100%;
- Fit width / Fit page mode changes;
- effective fit scale changes after resize/side-panel changes;
- quantized raster-size changes;
- stable native page-metadata geometry revision when it affects layout/render spec.

Do not allow `should_commit_viewport_update()` to suppress necessary work merely because visible and overscan page ranges are unchanged.

A render-spec change must supersede obsolete queued work and issue bounded new authoritative requests for the currently visible/near-visible pages.

### 3. Presentation during replacement

The UI must remain immediate while replacement rasters are generated.

It is acceptable to temporarily scale a compatible old texture or show a bounded placeholder, but:

- the UI must not freeze;
- stale generation/spec completions must not become authoritative;
- stationary zoom/fit changes must converge automatically without requiring the user to scroll to wake the scheduler;
- successfully completed authoritative renders must be discoverable by the exact presentation lookup key.

### 4. Preserve A2 continuous viewport behavior

Do not regress:

- continuous crossing of page boundaries;
- partial adjacent pages;
- visible-page/overscan ownership;
- current-page derivation;
- explicit Next/Previous/SetPage jump semantics;
- stable native metadata geometry;
- mixed portrait/landscape aspect ratios;
- long-document cached/indexed geometry;
- horizontal high-zoom access;
- focal witness restoration;
- visible-page pinning/residency;
- source switching stale safety;
- Goal 0019 native service/page-domain behavior;
- EPUB/TTS behavior.

## Required deterministic acceptance coverage

Add production-path tests proving at minimum:

1. scheduler request keys exactly match Reader lookup/presentation keys at manual 100%;
2. the same key agreement holds at high manual zoom;
3. the same key agreement holds for Fit width;
4. the same key agreement holds for Fit page;
5. changing manual zoom while visible page ranges stay unchanged still commits/schedules the new render specification;
6. Reset/100% while stationary schedules replacement rasters;
7. Fit width / Fit page while stationary schedule replacement rasters;
8. resize that changes effective fit scale schedules the new raster spec;
9. a completed authoritative raster is found by the presentation lookup key;
10. stationary zoom cannot leave the viewport permanently blank;
11. rapid movement still prioritizes newest visible pages and supersedes stale queued work;
12. visible pages remain pinned under residency pressure;
13. source/generation/spec stale results remain rejected;
14. continuous geometry/visibility/current-page/witness/mixed-page/long-document tests remain green;
15. Goal 0019 native lifecycle/page-domain tests and representative EPUB/TTS regressions remain green.

## Hosted validation

Do not terminalize or signal Goal achieved until the required hosted Windows workflow has completed successfully for the A3 implementation lineage, including both:

- `native-workspace`;
- `hosted-renderer-probe`.

## Physical QA after director acceptance

After A3 source/CI acceptance, physical QA should verify continuous scrolling, adjacent-page visibility, rapid navigation, Fit width/Fit page/100%, broad manual zoom, horizontal high-zoom access, focal stability, resize stability, cache responsiveness, stale-safe source switching, and representative EPUB/TTS behavior.

## Explicit non-goals

Do not implement PDF TTS playback correctness, sentence/text-layer geometry, spoken highlighting, OCR alignment, text selection/copy parity, Quack-check integration, Goal 0015, Goal 0017, Goal 0018, Natural/HD voices, or unrelated UI redesign.

## Repository handoff

- Repository goal: `0020-continuous-pdf-viewport-and-zoom`.
- This is A3 under the same repository goal ID.
- Start/recreate the Codex branch from current director `main` following normal workflow.
- Move this file `ready -> active`, re-arm the watcher, preserve accepted A1/A2 work, implement A3, update `docs/work/reports/0020.md`, validate, wait for hosted Windows success, terminalize to `done/`, push before signaling, and restore the shared checkout to `main`.
- Do not request human QA. The director reviews first.
