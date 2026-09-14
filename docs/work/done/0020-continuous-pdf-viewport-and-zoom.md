# 0020 — Continuous native PDF viewport and practical zoom

## Current state

**REOPENED FOR A2 DIRECTOR CORRECTION — DO NOT REQUEST HUMAN QA YET**

Goal 0019 / Gate 3 is physically accepted. Goal 0020 A1 introduced a continuous native page stack and broader manual zoom, but director review found that production viewport ownership, fit modes, zoom navigation/anchoring, geometry stability, long-document render-thread cost, and hosted validation do not yet satisfy the contract.

Read `docs/work/reviews/0020-a1-director-rejection.md` before implementation.

Continue the existing report lineage in `docs/work/reports/0020.md`.

## Outcome remains unchanged

Replace the temporary single-page PDF presentation with a smooth continuous-scrolling native PDF viewport.

The user must be able to scroll naturally across page boundaries, stop with portions of adjacent pages visible, zoom without losing semantic viewport position, and retain explicit page identity/navigation. Rendering must remain native, bounded, immediate-mode, and responsive.

This goal remains visual only. Do not expand into PDF TTS/highlight/OCR synchronization; Gate 4 follows after this viewport is trustworthy.

## Architecture to preserve

- Native Rust + `eframe`/`egui` + native/bundled Pdfium only. No WebView/pdf.js/browser DOM/Tauri production fallback.
- Preserve the single authoritative `PdfNativeService` / single native Pdfium owner established in Goal 0019.
- Keep Pdfium initialization, parse/open metadata, rasterization, bitmap conversion, and other heavy work off the egui/render thread.
- Preserve source/generation/page/render-size stale identity and rejection.
- Preserve bounded/current-priority worker scheduling and deterministic residency.
- Preserve explicit native PDF page-domain ownership in `ReaderSession` and truthful page count/bookmark semantics.
- Preserve visual-first PDF open independent of Quack-check/Python/Docling/OCR/transcript recovery.
- Preserve A1's continuous-stack direction, broader manual zoom ladder, and pure deterministic viewport-geometry helpers where they remain valid.

## A2 blocking correction

### 1. One authoritative continuous viewport lifecycle

A1 still feeds `visible_page_indexes = vec![snapshot.current_page]` into the canonical PDF viewport planner, while the Reader independently computes a multi-page visible set inside `show_viewport()` and submits ad-hoc render requests.

A2 must remove this split ownership.

The actual continuous viewport must publish the authoritative visible-page set and bounded overscan set into the existing PDF viewport planning lifecycle. That same plan must drive:

- current/visible/near-visible render priority;
- superseding stale queued raster work;
- texture keep/pin/eviction decisions;
- diagnostics/viewport ownership state.

All actually visible pages must be protected from residency eviction. Do not maintain a second independent UI-loop scheduling policy.

### 2. Real production fit modes

A1 production currently hard-codes Fit width to `1.0` and Fit page to `0.72` even though a viewport/page-dimension-based fit helper exists.

A2 must make production Fit width and Fit page derive from the actual viewport dimensions and stable target-page geometry. They must respond correctly to resize/side-panel changes and mixed portrait/landscape pages.

The toolbar must display truthful zoom state: manual percentage or clearly named fit mode plus effective percentage where useful.

Reset/100% must remain distinct from Fit width/page.

### 3. Practical horizontal navigation above 100%

At manual zoom above the viewport width, the user must be able to reach the entire page horizontally while continuing to scroll vertically across pages.

Do not leave oversized content clipped inside a vertical-only scroll surface. Use immediate-mode egui scrolling/panning semantics; do not introduce a retained/browser surface.

### 4. Semantic focal anchoring for zoom and resize

Before a zoom-mode/manual-zoom/resize geometry transition, capture a semantic PDF viewport witness such as:

- page identity;
- intra-page vertical fraction (and horizontal fraction when relevant);
- viewport anchor fraction.

After geometry changes, restore that witness within bounded tolerance. Zooming in the middle of page N must not throw the user to another distant page or page top.

Explicit page jumps retain priority over passive focal restoration.

### 5. Stable page geometry independent of raster completion

Do not derive layout-critical aspect ratio from a texture only after raster completion.

A2 must obtain stable page dimensions/aspect ratios from the existing off-thread native PDF owner/service or another bounded native metadata path. Texture arrival/resolution replacement must not change the page's logical slot dimensions.

Mixed portrait/landscape/page-size documents must therefore have stable downstream offsets before/independent of raster completion.

### 6. Bounded long-document geometry work

Do not rebuild `0..total_pages` page-geometry vectors and all slot positions every egui frame.

Cache/index document geometry and invalidate it only when source/native page metadata/zoom/layout geometry changes. Ordinary scrolling/repaint should be bounded or logarithmic with document length, not O(total-pages) per frame.

A full prefix/index table may exist if built/rebuilt off the hot frame path; visibility lookup should avoid scanning every page on every repaint.

### 7. Required hosted validation before terminal signaling

A1 terminalized without the required hosted Windows workflow actually attached/passed.

A2 must not move to terminal/done or signal Goal achieved until required hosted `native-workspace` and `hosted-renderer-probe` jobs have completed successfully for the implementation lineage.

## Required presentation behavior

### Continuous virtualized page stack

Present pages vertically in document order inside one continuous scroll surface. More than one page may be partially visible. Crossing page boundaries must require only ordinary scrolling.

Do not instantiate/render all pages of a large document. Visible pages are highest priority; bounded near-visible overscan may be prefetched; far pages must consume neither raster work nor texture residency.

### Canonical current-page ownership

`ReaderSession.current_page` remains meaningful and is derived deterministically from the viewport with hysteresis/stability near boundaries.

Prev/Next/SetPage remain explicit jumps within the continuous document rather than page swaps.

Bookmarks preserve native page identity and, where practical, a bounded intra-page viewport anchor.

### Smooth geometry and resize

Window resize, side-panel changes, placeholder-to-texture replacement, and resolution upgrades must not cause distant jumps.

Portrait, landscape, and mixed-size pages must preserve aspect ratio and stable stack geometry.

### Practical zoom

Support at least Fit width, Fit page, Reset/100%, and a substantially wider bounded manual range than the old 75–175% scaffold.

Logical zoom must respond immediately. Correct-resolution raster can arrive asynchronously without changing document identity or semantic viewport position.

### Immediate-mode interaction quality

Scrolling, horizontal panning when needed, zoom input, resize, and jumps must feel immediate while workers catch up. The UI thread must never wait synchronously for Pdfium.

## Deterministic acceptance coverage

Add production-path tests proving at minimum:

1. actual continuous viewport visible pages feed the authoritative PDF plan/residency policy;
2. a boundary position exposes portions of adjacent pages;
3. visible textures remain pinned under over-capacity pressure;
4. current-page derivation is stable under tiny boundary movement;
5. Next/Previous/SetPage resolve to continuous-scroll jump targets;
6. rapid viewport movement prioritizes newest visible pages and supersedes obsolete queued work;
7. stable native page metadata supplies portrait/landscape/mixed page geometry before raster completion;
8. long-document geometry/visibility lookup avoids O(total-pages) reconstruction/scanning on ordinary repaint;
9. Fit width and Fit page use real viewport/page dimensions in the production path;
10. manual zoom up to the chosen upper bound remains horizontally navigable;
11. zoom/fit/resize preserve an intra-page semantic focal anchor within bounded tolerance;
12. displayed zoom/mode state is truthful;
13. source switching cannot display stale prior-document imagery;
14. Goal 0019 native service/page-domain regressions remain green;
15. representative EPUB/TTS behavior remains green.

## Physical QA after director acceptance

Use at least one long real PDF and, if available, a mixed/landscape document.

Verify continuous page crossing, partial adjacent pages, rapid long-distance scrolling, page jumps, Fit width/Fit page/100%/broad manual zoom, horizontal access at high zoom, zoom/resize anchor stability, fast return to recently viewed pages, stale-safe source switching, and one representative EPUB/TTS regression.

## Explicit non-goals

Do not implement PDF TTS playback correctness, sentence/text-layer geometry, spoken highlighting, OCR alignment, text selection/copy parity, Quack-check integration, Goal 0015, Goal 0017, Goal 0018, Natural/HD voices, or unrelated UI redesign.

## Repository handoff

- Repository goal: `0020-continuous-pdf-viewport-and-zoom`.
- This is an A2 correction under the same repository goal ID.
- Start/recreate the Codex branch from current director `main` following normal workflow.
- Move this file `ready -> active`, re-arm the watcher, preserve accepted A1 work, implement A2, update `docs/work/reports/0020.md`, validate, wait for hosted Windows success, terminalize to `done/`, push before signaling, and restore the shared checkout to `main`.
- Do not request human QA. The director reviews first.
