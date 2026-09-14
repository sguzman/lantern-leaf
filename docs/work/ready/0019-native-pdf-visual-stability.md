# 0019 — Native PDF visual stability

## Outcome

Make PDF a real native reader surface in the authoritative Rust + `eframe`/`egui` application.

Opening a PDF must show the actual PDF page raster in the Reader shell, with stable page navigation, zoom, resize/scroll behavior, bounded caching, and a responsive UI. This goal establishes the visual/rendering gate only. PDF text/TTS/highlight synchronization remains the next separate product gate.

## Starting evidence

The repository already contains useful PDF scaffolding, but the physical product is not yet an accepted PDF reader:

- `ReaderSnapshot` carries `PrettyKind::Pdf`, total/current page, PDF quality/runtime metadata, extracted text, and canonical reader state.
- `crates/lanternleaf-egui/src/pdf.rs` contains viewport planning and eviction-policy helpers.
- `crates/lanternleaf-egui/src/pdf_subsystem.rs` contains zoom/viewport policy types.
- `crates/lanternleaf-egui/src/pdf_renderer.rs` wraps bundled Pdfium and can raster a page into an egui `ColorImage`, but its current cache/render key is not production zoom-aware and the renderer is not wired into the actual user-facing PDF page surface.
- `LanternLeafApp` initializes `NativePdfRenderer` and maintains diagnostic/planning `PdfRenderState`, but the Reader UI currently routes `PrettyKind::Pdf` away from pretty rendering and primarily exposes text/canonical/diagnostic surfaces rather than a working native page view.
- The current physical observation is therefore expected: PDF view/TTS/highlight is not accepted as working.

The old native-PDF roadmap is historical design context only. This goal and current director-owned project documents are authoritative.

## Hard architectural rules

### 1. Never raster/decode/open PDF pages on the egui/render thread

This is a hard requirement.

The UI/render thread may:

- determine the current viewport/page and desired render size;
- enqueue/coalesce bounded work;
- receive already-rendered image results;
- upload a bounded number of completed images into egui textures;
- compose those textures and lightweight controls.

It must not execute Pdfium page rasterization, repeated PDF file open/load, image decode/resize, or unbounded cache work.

Create or formalize a dedicated bounded PDF render worker/service that owns the native renderer/Pdfium work. Prefer a single worker owning renderer state unless evidence justifies otherwise. Do not move the problem into another GUI callback.

### 2. Request/result ownership must be explicit and stale-safe

A render result must be identified by at least:

- source/document identity or generation;
- page index;
- requested render scale/size bucket;
- request/generation freshness sufficient to reject stale source/zoom results.

Switching PDFs, closing a book, paging rapidly, or changing zoom must never allow an older completion to replace the currently owned page texture.

Duplicate work for the same current render key must coalesce rather than queue repeatedly every frame.

### 3. Bounded viewport scheduling

Use the existing viewport/render-plan direction rather than rendering an entire PDF.

At minimum:

- current/visible page is highest priority;
- a small nearby overscan set may be prefetched;
- in-flight render work is bounded;
- resident CPU images / egui textures are bounded;
- pages outside the keep set are evicted predictably;
- current/visible page is never evicted merely to preserve farther pages.

The existing `PdfViewportRenderPlan`, budget helpers, and `PdfRenderState` may be repaired/reused, simplified, or replaced where they are only diagnostic scaffolding. Do not preserve dead architecture merely because it exists.

### 4. Zoom must correspond to real render ownership

Do not fake PDF zoom by permanently stretching one tiny 320x450 raster.

Use discrete/quantized render sizes so small resize noise does not trigger endless rerenders, but meaningful zoom changes produce an appropriately sized native raster. The render/cache key must include that scale/size identity.

Use a sane upper render-size/memory bound. Avoid rendering pathological giant textures simply because the window or zoom request is large.

### 5. Stable user-facing PDF surface

When `PrettyKind::Pdf` is active and the user is not deliberately in text-only fallback, the main Reader content must show the native PDF page rather than only a sentence list and diagnostics.

Provide lightweight PDF controls sufficient for this gate:

- page identity (`Page N / M`);
- previous/next page behavior through existing canonical session commands or equivalent existing navigation ownership;
- zoom out / reset or fit / zoom in using the existing zoom policy or a clearly bounded replacement;
- a scrollable viewport when the rendered page exceeds the available view.

Window resize and zoom should preserve understandable page position and must not produce continuous oscillation, flashing between stale page textures, or repeated recenters.

Text-only mode may remain an explicit fallback to the existing extracted-text/sentence surface.

### 6. Native-only production path

Do not resurrect Tauri, React, WebView, pdf.js, browser DOM overlays, or browser-owned PDF rendering.

Bundled/native Pdfium remains the accepted rendering basis unless a concrete blocker discovered during implementation requires director escalation.

## Visual-quality expectations

- Preserve the page's aspect ratio.
- Render the whole current page without cropping by default.
- At ordinary 100%/fit-width use, body text and line art should not be obviously destroyed by unnecessary low-resolution stretching.
- Rotated/landscape pages must retain their native orientation/aspect ratio rather than being forced into portrait assumptions.
- Placeholder/loading presentation should have stable dimensions where practical so a completed page does not violently rearrange the Reader shell.
- Render failure must produce a bounded readable state and remain recoverable by page/zoom/source changes; no giant diagnostic dump in the normal reader surface.

## Validation corpus

Use repository fixtures when available and add small deterministic PDF fixtures as necessary. Automated coverage must include at least:

1. render-request coalescing for repeated identical frame requests;
2. stale result rejection after source change;
3. stale result rejection after zoom/size-generation change;
4. bounded in-flight scheduling / bounded resident page ownership;
5. visible/current page preservation under eviction pressure;
6. render key includes page plus render size/zoom identity;
7. landscape/aspect-ratio dimension calculation;
8. page navigation changes the owned visible render target rather than leaving the previous page texture in place;
9. render failure clears/terminalizes the owned request instead of leaving permanent `loading` state;
10. existing non-PDF reader, TTS, Goal 0010/0016 catalog paths remain green.

If a fully realistic Pdfium render cannot run deterministically in one unit-test environment, keep pure ownership/scheduler/dimension tests deterministic and use the existing Windows renderer probe / integration surface for native Pdfium evidence. Do not weaken the architecture merely to make a synthetic test easy.

## Required Windows/CI gates

Before terminalizing:

- focused PDF unit/integration tests;
- `cargo check --workspace`;
- workspace tests;
- workspace/native build required by the repository's normal gate;
- repo-native QA preparation;
- Windows baseline workflow success, including the hosted renderer probe;
- no heavy-work-on-render-thread regression.

Human physical QA is requested only after director source/CI review and integration to `main`.

## Real-desktop acceptance target

The eventual focused Windows pass should verify:

- opening a representative local PDF visibly renders its actual first page;
- opening a representative Caliberate PDF also reaches the same native page surface after materialization;
- next/previous page shows the correct page without stale-page flashes;
- zoom in/out/reset/fit produces stable, readable rerenders;
- resizing and scrolling remain responsive;
- repeated navigation through a multi-page PDF does not cause obvious unbounded memory growth or worsening lag;
- closing one PDF and opening another cannot show stale imagery from the previous source;
- existing EPUB open/TTS/pretty behavior remains green.

## Explicit non-goals

Do **not** expand this goal into:

- PDF TTS playback correctness;
- sentence-to-page/text-layer geometry mapping;
- spoken-sentence overlays/highlighting;
- OCR implementation or OCR quality repair;
- click-to-sentence reverse mapping;
- text selection/copy parity;
- broad PDF settings redesign;
- Goal 0015 presentation-reflow polish;
- Goal 0017 cover-pressure polish;
- Goal 0018 Windows QA bootstrap cleanup;
- Windows Natural/HD voices;
- unrelated UI redesign.

Existing PDF extraction/classification/OCR artifacts may remain present and diagnostic, but this goal must not make visual rendering contingent on solving their synchronization semantics.

## Repository handoff

- Repository goal: `0019-native-pdf-visual-stability`
- Branch: `codex/0019-native-pdf-visual-stability`
- Start from current director `main` and fast-forward it.
- Move this file `ready -> active`.
- Launch/re-arm the normal goal watcher for this execution attempt.
- Inspect the actual current PDF scaffolding before rewriting it; preserve useful pure planning/test contracts, but delete or refactor dead diagnostic-only ownership when necessary for one coherent production path.
- Implement the complete bounded visual gate, not merely a screenshot/demo path.
- Write `docs/work/reports/0019.md` with implementation decisions, test/CI evidence, remaining bounded risks, and exact implementation SHA.
- Terminalize to `done/` only when automated acceptance is satisfied; use `blocked/` only for a true director-level architecture/dependency blocker.
- Push the terminal branch before signaling completion.
- Restore the shared checkout to `main` after terminal signaling.
- Do not request human QA during implementation. The director reviews first.
