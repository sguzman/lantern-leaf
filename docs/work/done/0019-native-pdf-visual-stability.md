# 0019 — Native PDF visual stability

## Current state

**REOPENED FOR A3 DIRECTOR CORRECTION — DO NOT REQUEST HUMAN QA YET**

Goal 0019 remains the one authorized repository macro-goal. A1 established the native Pdfium/egui visual path but was rejected before desktop QA. A2 correctly repaired real presentation-scale zoom, current-priority render scheduling, deterministic residency policy, and missing policy tests, but director review found one remaining production-path defect: the old arbitrary HashMap texture-eviction loop still runs before the new deterministic residency policy.

Read these reviews in order:

- `docs/work/reviews/0019-a1-director-rejection.md`
- `docs/work/reviews/0019-a2-director-rejection.md`

The implementation branch/report lineage remains `codex/0019-native-pdf-visual-stability` / `docs/work/reports/0019.md`.

## Outcome remains unchanged

Make PDF a real native reader surface in the authoritative Rust + `eframe`/`egui` application.

Opening a PDF must show the actual native Pdfium-rasterized page with stable navigation, real zoom, responsive resize/scroll behavior, bounded caching/residency, and no heavy PDF work on the egui/render thread. PDF text/TTS/highlight/OCR synchronization remains the next separate product gate.

## Accepted architecture to preserve

- Native Rust + egui + bundled/native Pdfium only. Do not introduce Tauri, React, WebView, pdf.js, browser DOM overlays, or browser-owned rendering.
- Pdfium initialization, PDF open/load, page rasterization, bitmap conversion/decode, and renderer CPU caching stay on the dedicated bounded worker.
- The egui thread may only compute bounded requests, receive already-rendered images, upload bounded textures, and compose lightweight controls/presentation.
- Render identity includes source/document identity, generation, page, and requested quantized/bounded render dimensions.
- Stale source/generation results are rejected.
- Duplicate work coalesces.
- The single Pdfium-owning scheduler gives newest current/visible work priority over obsolete queued nearby work.
- Presentation size is distinct from raster resolution. Zoom changes actual logical page size and may exceed the viewport, while raster dimensions follow through bounded/quantized render keys.
- Current/visible page is highest priority; nearby overscan is bounded; CPU/texture residency is bounded; source switches cannot show stale imagery.
- `PdfResidentEntry` + `choose_resident_texture_evictions()` are the accepted deterministic residency direction: current page pinned, keep/nearby pages preferred, irrelevant pages evicted first.
- Preserve the new A2 deterministic tests for zoom/presentation, landscape aspect, navigation identity, failure terminalization, resident current-page preservation, duplicate coalescing, and stale-queue/current priority.

## A3 blocking correction

A2 still contains this legacy A1 production loop immediately after completed textures are inserted:

```rust
while self.pdf_textures.len() > 8 {
    if let Some(key) = self.pdf_textures.keys().next().cloned() {
        self.pdf_textures.remove(&key);
    }
}
```

Later in the same `render_pdf_surface()` call, A2 correctly computes viewport-aware `PdfResidentEntry` values and calls `choose_resident_texture_evictions(..., 8)`. The legacy loop executes first, so arbitrary HashMap iteration can already discard the current visible texture before the deterministic policy runs.

A3 must:

1. remove the legacy arbitrary pre-eviction of `pdf_textures`;
2. make the deterministic viewport-aware residency path the **only** production mechanism enforcing the texture capacity bound;
3. ensure the current visible `current_key` is pinned and cannot be evicted merely to preserve irrelevant/farther pages;
4. keep `pdf_render_errors` bounded; diagnostic error retention must not affect visible texture ownership. Deterministic error retention is preferred if trivial but is not a reason to broaden scope;
5. add a production-style regression that inserts more than the resident capacity including the current key, runs the same residency-bounding helper/path used by the Reader, and proves the current key remains while irrelevant entries are removed. Do not rely only on the isolated pure eviction chooser test;
6. preserve every accepted A1/A2 zoom, scheduler, stale-safety, native-only, bounded-worker, and off-render-thread guarantee.

## Required validation

Before terminalizing A3:

- focused PDF renderer/scheduler/residency tests;
- `cargo test -p lanternleaf-egui`;
- `cargo check --workspace`;
- workspace tests and normal Windows/native build gate;
- repo-native QA preparation;
- hosted Windows baseline workflow success including `native-workspace` and `hosted-renderer-probe`;
- `git diff --check`;
- no human QA request during implementation.

## Real-desktop acceptance after director integration

Once director review accepts and integrates A3, the focused Windows pass should verify:

- representative local PDF renders its actual first page;
- representative Caliberate PDF reaches the same native page surface after materialization;
- next/previous page shows the correct page without stale-page flashes;
- zoom in/out/reset changes real page scale and remains crisp/stable;
- zoomed pages can scroll naturally;
- resize/scroll/navigation remain responsive;
- repeated multi-page navigation does not show obvious unbounded memory growth or worsening lag;
- switching PDFs cannot show stale imagery from the prior source;
- representative EPUB/TTS/pretty behavior remains green.

## Explicit non-goals

Do not expand A3 or Goal 0019 into PDF TTS playback correctness, sentence/text-layer geometry mapping, spoken-sentence overlays/highlighting, OCR implementation/quality repair, click-to-sentence reverse mapping, text selection/copy parity, broad PDF settings redesign, Goal 0015, Goal 0017, Goal 0018, Natural/HD voices, or unrelated UI redesign.

## Repository handoff

- Repository goal: `0019-native-pdf-visual-stability`
- Branch: `codex/0019-native-pdf-visual-stability`
- This is a correction continuation under the same repository goal ID.
- Start from/synchronize current director `main` and continue the existing implementation/report lineage.
- Move this file `ready -> active`.
- Re-arm the normal watcher for the fresh Codex Goal attempt.
- Read both director rejection reviews before editing.
- Preserve accepted A1/A2 work; implement only the narrow A3 correction plus its regression coverage.
- Update `docs/work/reports/0019.md` with A3 implementation/test/CI evidence and exact commits.
- Terminalize to `done/` only after all required gates pass.
- Push before terminal signaling, then restore the shared checkout to `main`.
- Do not request human QA. The director reviews first.