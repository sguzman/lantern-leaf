# Goal 0019 A3 — director acceptance

## Decision

**ACCEPTED FOR REAL-DESKTOP QA — INTEGRATED TO `main`**

A3 resolves the remaining director-blocking production defect from A2. The legacy arbitrary `HashMap::keys().next()` texture pre-eviction path is removed, and the viewport-aware deterministic residency path is now the sole production mechanism enforcing the visible PDF texture-capacity bound.

## Accepted implementation

- Preserve the native-only Rust + egui + bundled Pdfium production path.
- Pdfium initialization, PDF open/load, rasterization, bitmap conversion, and renderer CPU caching remain on the dedicated bounded worker rather than the egui/render thread.
- Presentation zoom changes actual logical page size, while bounded/quantized raster dimensions track that presentation scale.
- Render identity remains source/generation/page/render-dimensions aware, with stale source/generation results rejected.
- The coalescing scheduler prioritizes newest current-page work over obsolete queued nearby work.
- Visible/current texture ownership is deterministic: the current key is pinned, planned nearby pages are preferred, and irrelevant pages are evicted first.
- The old arbitrary pre-eviction loop has been removed.
- `pdf_render_errors` remains separately bounded and does not participate in visible texture ownership.

## Validation evidence

- Focused PDF renderer/scheduler/residency tests passed, including the new production-style over-capacity current-page preservation regression.
- `cargo test -p lanternleaf-egui` passed.
- `cargo check --workspace` passed.
- Windows build and QA preparation passed.
- Hosted Windows workflow `34842799704` passed both `native-workspace` and `hosted-renderer-probe`, including workspace tests/build and the native renderer capability probe.
- The local workspace run reported two unrelated Calibre assertions, but the hosted Windows workspace test gate passed on the terminal branch and A3 did not touch those Calibre source areas.

## Integration

Terminal branch head `380a4b654516f33feb158e4c73722b3624d10138` was fast-forward integrated to `main` after director source/CI review.

## Remaining gate

Goal 0019 now requires focused real-desktop Windows verification only. The physical pass should verify representative local and Caliberate PDFs, page navigation, real zoom and scrolling, resize responsiveness, stale-source safety, sustained multi-page navigation, and preservation of representative EPUB behavior.

PDF text/TTS/highlight/OCR synchronization remains the next separate product gate and is not part of Goal 0019 closure.
