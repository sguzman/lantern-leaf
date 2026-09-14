# Goal 0020 A1 — director rejection

## Decision

**REJECTED BEFORE HUMAN QA — CONTINUE AS A2**

A1 moves the production Reader toward a continuous vertical PDF stack and preserves the accepted native Pdfium worker architecture, but it does not yet satisfy the Goal 0020 viewport/zoom contract. Do not request physical QA for A1.

## Accepted A1 direction

Preserve these parts in A2:

- native Rust + egui + the existing single `PdfNativeService` / Pdfium owner;
- continuous vertical page-slot presentation rather than mandatory one-page swaps;
- broader manual zoom levels;
- explicit Prev/Next jump intent;
- source/generation/page/render-size stale identity;
- Pdfium/raster work remaining off the egui thread;
- the new deterministic pure viewport-geometry helpers where they remain valid.

## Blocking defect 1 — production viewport planning is still single-page

The application-level PDF viewport lifecycle still constructs `visible_page_indexes = vec![snapshot.current_page]` and builds the canonical `PdfViewportRenderPlan` from that single-page set.

The new Reader surface separately computes a continuous visible set inside `ScrollArea::show_viewport()`, but it does not publish that set back into the canonical viewport planner/residency lifecycle. Instead, pages missing from the old plan are requested ad hoc from inside the render loop.

Consequences:

- the existing viewport planner is not actually driving the continuous surface;
- visible/overscan ownership is split between two unrelated mechanisms;
- residency pinning still uses the old plan, so genuinely visible pages outside the single-page plan can be treated as disposable under texture pressure;
- the Goal 0020 requirement that visible pages be highest priority and pinned is therefore not established.

A2 must make the actual continuous visible/overscan set the authoritative input to planning, scheduling, and residency. Do not leave a second ad-hoc render-request policy in the UI loop.

## Blocking defect 2 — Fit page is not implemented in production

A1 adds a correct-looking `PdfZoomState::effective_level()` helper and tests it, but the production Reader does not use it.

Production currently maps:

- Fit width -> hard-coded `1.0`;
- Fit page -> hard-coded `0.72`.

`0.72` is not a fit-page computation and cannot account for viewport height, page aspect ratio, mixed page sizes, side panels, or resize. The toolbar percentage also continues to display the stored manual `zoom_level`, not the effective fit mode/percentage.

A2 must connect the real fit computation to production. Fit width/page must derive from current viewport dimensions and the target page geometry, update on resize, and display truthful mode/effective scale.

## Blocking defect 3 — manual zoom above 100% cannot be navigated horizontally

The continuous surface uses `ScrollArea::vertical()` while setting document/page width to `viewport_width * zoom`.

At 200–400% the logical page can be several times wider than the viewport, but there is no horizontal scrolling/panning surface. Large portions of the zoomed page therefore become inaccessible.

A2 must provide practical horizontal navigation when content exceeds the viewport while preserving natural vertical continuous scrolling. This must remain immediate-mode and must not introduce a retained/WebView surface.

## Blocking defect 4 — zoom/resize focal anchoring is not wired to production

Goal 0020 requires zooming in the middle of a page to preserve the user's logical focal location.

A1 contains a pure `jump_offset()` helper/test, but the actual zoom buttons simply change generation/zoom and let egui retain the old absolute scroll offset against newly-scaled document geometry. There is no capture of `(page identity, intra-page fraction, viewport fraction)` before zoom/resize and no restoration after geometry changes.

That means zooming a long document can move the user to a different semantic location/page even though tests for the standalone helper pass.

A2 must implement production semantic viewport anchoring for manual zoom, Fit width/page transitions, and resize/side-panel geometry changes.

## Blocking defect 5 — page geometry is texture-derived and can move after raster completion

A1 initializes every unknown page with a default aspect ratio of `0.707`, then replaces an individual page's aspect ratio only after its raster image completes. The full stack geometry is rebuilt from those values each frame.

For landscape/mixed-size PDFs this means a placeholder can have one height, then change height when its image arrives, shifting every downstream page. This directly conflicts with the stable-stack requirement.

A2 must establish stable page geometry independently of raster completion. Use native PDF page metadata through the existing off-thread owner/service, or another bounded native metadata strategy. Texture arrival must not be the event that discovers layout-critical page aspect ratio.

## Blocking defect 6 — full-document geometry is rebuilt on every egui frame

The production Reader currently creates a `Vec<PdfPageGeometry>` for `0..snapshot.total_pages` and rebuilds every page slot/prefix position every frame.

That is O(total-pages) render-thread work and conflicts with the stated immediate/bounded long-document architecture. The ready contract explicitly permits lightweight geometry for the bounded neighborhood needed to determine visibility; it does not authorize rebuilding an arbitrarily large document's full geometry every frame.

A2 must retain/index geometry so ordinary scrolling and repaint cost is bounded or logarithmic with document size. Recompute only when source/page-metadata/zoom/layout geometry actually changes, not every frame.

## Validation / workflow defect

A1's report itself says hosted `windows-baseline` remains required after push. No hosted workflow run is attached to the A1 commit, yet the goal was moved to `done` and signaled as achieved.

Goal 0020 requires hosted `native-workspace` and `hosted-renderer-probe` success before terminal signaling. A2 must not terminalize/signpost completion until those required hosted gates have actually passed.

## A2 required correction

1. Preserve the accepted A1 continuous-stack/native-worker direction.
2. Make the actual continuous visible/overscan set the single authoritative viewport-plan input.
3. Drive render priority and texture pin/eviction from that plan; remove split ad-hoc ownership.
4. Implement real production Fit width / Fit page using viewport + page geometry and truthful displayed mode/scale.
5. Make >100% zoom practically navigable horizontally.
6. Add production semantic focal anchoring across zoom and resize.
7. Obtain layout-critical page aspect ratios from stable off-thread PDF metadata rather than texture completion.
8. Avoid O(total-pages) geometry reconstruction every frame; cache/index geometry and invalidate only on relevant state changes.
9. Add deterministic production-path tests for these behaviors, not helper-only tests.
10. Run and wait for required hosted Windows CI before terminalizing.

Goal 0020 remains open. Human PDF QA is deferred until A2 passes director source/CI review.