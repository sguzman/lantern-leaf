# 0020 — Continuous native PDF viewport and practical zoom

## Current state

**READY NEXT — ONE AUTHORIZED MACRO-GOAL**

Goal 0019 / Gate 3 is physically accepted. The native Pdfium/egui visual reader now opens real PDFs, owns truthful native page counts, supports Next/Previous navigation, and preserves EPUB behavior.

Physical QA clarified that the intended PDF reading UX is not a paginated one-page-at-a-time surface. The desired production surface is a **continuous, immediate-mode, virtualized page stack** with broader practical zoom while preserving explicit page identity and navigation.

## Outcome

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

## Required presentation behavior

### 1. Continuous virtualized page stack

The PDF reader should present pages vertically in document order inside one continuous scroll surface.

At ordinary positions, more than one page may be partially visible. Crossing a page boundary must require only normal scrolling; there must be no mandatory Next/Previous page transition.

Do not instantiate/render all pages of a large document. Use the existing PDF viewport planning primitives (`visible_page_indexes`, overscan windows, priority pages, eviction budgets) as the semantic basis for virtualization.

The egui thread may lay out lightweight page placeholders/geometry for the bounded neighborhood needed to determine visible pages, but must not perform Pdfium work.

### 2. Canonical current-page ownership

`ReaderSession.current_page` remains meaningful in a continuous viewport.

Define the current page deterministically from the viewport (for example the page containing the viewport center or another documented stable anchor). Scrolling across boundaries should update canonical current-page state without jitter.

Explicit Next/Previous/SetPage controls remain available as **jumps** within the continuous document. They should scroll to the target page, not swap a separate single-page canvas.

Bookmarks must preserve the native page domain and, where practical, a bounded intra-page viewport fraction/anchor so reopening does not discard scroll position.

### 3. Smooth viewport geometry and resize

Window resizing, side-panel changes, and ordinary scrolling must not cause distant jumps.

Page geometry must remain stable enough that replacing a placeholder/old-resolution texture with a newly rasterized texture does not move surrounding pages unpredictably.

Preserve aspect ratio for portrait and landscape pages. Mixed-size documents must be supported.

### 4. Practical zoom model

The current fixed `PDF_ZOOM_LEVELS = [0.75, 0.9, 1.0, 1.1, 1.25, 1.5, 1.75]` is only a conservative Gate-3 scaffold.

Replace/extend it with practical reader controls that include at least:

- Fit width;
- Fit page;
- Reset/100%;
- substantially wider manual zoom than 75–175% (choose a bounded range appropriate for the native raster caps and interaction quality);
- predictable stepping and displayed percentage/state.

Zoom must change logical presentation size immediately. Correct-resolution native rasters may arrive asynchronously and replace temporary/scaled textures without changing document identity or viewport anchor.

Zooming while positioned in the middle of a page should preserve the user's logical focal location as closely as practical rather than snapping to page top.

### 5. Bounded render ownership

Visible pages are highest priority. Near-visible overscan pages may be prefetched. Far pages must not consume native render work or texture residency merely because the document is long.

A rapid scroll should allow newest visible-page requests to supersede obsolete queued work. The UI must never wait synchronously for Pdfium before moving.

Retain deterministic viewport-aware texture eviction; currently visible pages must be pinned from eviction.

### 6. Immediate-mode interaction quality

Scrolling, zoom-control input, resize, and page jumps should feel immediate even while native workers are catching up.

Temporary lower-resolution scaling/placeholders are acceptable during motion if they avoid blanking/freezing and converge promptly to the appropriate raster.

Do not add retained/browser-owned presentation state to achieve smoothness.

## Acceptance coverage

Add deterministic tests for at least:

1. viewport geometry maps a scroll offset to a stable bounded set of visible PDF pages;
2. a boundary position can expose portions of two adjacent pages;
3. current-page derivation is deterministic and does not oscillate under tiny scroll changes;
4. Next/Previous/SetPage produce continuous-viewport jump targets while preserving the native page domain;
5. visible + overscan planning remains bounded for very large page counts;
6. rapid viewport movement prioritizes newest visible pages over stale queued raster work;
7. current visible textures survive residency pressure;
8. portrait, landscape, and mixed page sizes preserve aspect and stable stack geometry;
9. Fit width, Fit page, 100%, and manual zoom states produce correct logical page sizes;
10. zoom preserves an intra-page focal anchor within bounded tolerance;
11. source switching cannot display stale pages from the prior PDF;
12. Goal 0019 native service/page-domain tests remain green;
13. representative EPUB/TTS behavior remains green.

## Physical QA after director acceptance

Use at least one long real PDF (hundreds of pages) and one mixed/landscape document if available.

Verify:

- mouse-wheel/trackpad scrolling crosses page boundaries continuously;
- partial adjacent pages can be visible simultaneously;
- rapid long-distance scrolling remains responsive;
- explicit page jumps still work;
- Fit width / Fit page / 100% / broader zoom work visibly;
- zoom does not throw the viewport to a distant page;
- resize remains stable;
- returning to recently viewed pages is fast and cache behavior feels bounded;
- switching PDFs never shows stale prior-document imagery;
- representative EPUB still renders/speaks normally.

## Explicit non-goals

Do not implement PDF TTS playback correctness, sentence/text-layer geometry, spoken highlighting, OCR alignment, text selection/copy parity, Quack-check integration, Goal 0015, Goal 0017, Goal 0018, Natural/HD voices, or unrelated UI redesign.

## Repository handoff

- Repository goal: `0020-continuous-pdf-viewport-and-zoom`
- This is the only ready macro-goal.
- Start from current director `main`.
- Move this file `ready -> active`, re-arm the normal watcher, implement, validate, write `docs/work/reports/0020.md`, terminalize to `done/`, push before signaling, and restore the shared checkout to `main`.
- Do not request human QA. The director reviews first.
