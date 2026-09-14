# Goal 0019 A1 — director rejection

## Decision

**REJECTED BEFORE HUMAN QA — CONTINUE AS A2**

A1 establishes the right native architecture but does not yet satisfy the visual-stability contract. Do not integrate the implementation branch or request physical Windows QA yet.

## Accepted A1 work to preserve

- Keep the native-only Pdfium + Rust + egui production path. Do not introduce WebView/pdf.js/Tauri ownership.
- Keep Pdfium initialization, PDF open/load, rasterization, bitmap conversion, and renderer CPU cache on the dedicated bounded worker thread rather than the egui/render thread.
- Keep explicit render identity including source, generation, page, and requested render dimensions.
- Keep stale source/generation result rejection and duplicate same-key coalescing.
- Keep bounded worker request/result channels and the existing current-plus-nearby scheduling direction.
- Keep the real PDF reader surface, page controls, text-only fallback, bounded readable loading/error state, and accepted non-PDF behavior.
- Preserve the passing Windows workflow evidence from run `34838434420`; both `native-workspace` and `hosted-renderer-probe` passed.

## Blocking defect 1 — zoom changes raster resolution but not presentation scale

The A1 UI requests a raster width derived from `available_width * zoom_level`, but when presenting the resulting texture it immediately scales any texture wider than the viewport back down to `ui.available_width()`:

```rust
let width = (ui.available_width().max(320.0)
    * self.pdf_render_state.zoom_level.clamp(0.75, 1.75))
.round() as u32;
...
let size = texture.size_vec2();
let scale = (ui.available_width() / size.x.max(1.0)).min(1.0);
ui.add(Image::new(texture).fit_to_exact_size(size * scale));
```

For the common portrait case, this makes 125%, 150%, and 175% produce a higher-resolution raster that is then shrunk back to approximately the same on-screen width as 100%. The zoom label changes and render ownership changes, but the page does not meaningfully become larger in the scrollable viewport.

This violates Goal 0019's explicit requirement that zoom be a real user-facing zoom and that the viewport become scrollable when the rendered page exceeds the available view. It also defeats the snappy native zoom interaction the product requires.

### A2 requirement

Separate **native raster resolution** from **egui presentation size**. A zoom step must change the page's presented logical size. Fit/reset may fit the page/width, but explicit zoom-in must be allowed to exceed the viewport so `ScrollArea::both()` has real work to do. The native raster should track that presentation scale with bounded/quantized resolution so zoomed text remains crisp without unbounded texture sizes.

Add deterministic pure tests around the presentation-size/zoom policy so this cannot regress into resolution-only zoom again.

## Blocking defect 2 — resident texture eviction can evict the current page arbitrarily

A1 bounds `pdf_textures` by repeatedly removing `HashMap::keys().next()` while the map has more than eight entries. HashMap iteration order is arbitrary and is unrelated to viewport ownership, recency, or the current page.

Therefore, after navigation/prefetch causes the resident set to exceed the limit, the eviction loop may remove the current page texture or a nearer page while preserving irrelevant older/farther pages. The current page can then fall back to `Rendering PDF page…` and be requested again, producing avoidable flashes/churn.

This violates the contract that pages outside the keep set be evicted predictably and that the current/visible page never be evicted merely to preserve farther pages.

### A2 requirement

Use explicit texture-residency ownership/eviction. The current page must be pinned. Nearby planned pages may be retained according to deterministic keep/LRU or distance policy. Pages outside the keep set should be removed first. Reuse the existing viewport-budget direction where useful rather than arbitrary HashMap iteration.

Add a deterministic test proving current-page preservation under resident-texture pressure, not merely preservation in the request-page list.

## Blocking defect 3 — stale FIFO render work can outrank a newly visible page

The worker uses a single FIFO bounded channel of eight requests. Rapid page changes or zoom/source generation changes can leave old generation/page/overscan jobs ahead of the newly visible page. Stale results are correctly rejected on return, but the expensive Pdfium work still runs first. If the queue is full, the new current-page request is temporarily refused and must wait for stale work to drain.

That is stale-safe for correctness but not sufficient for the contract's priority requirement or the intended snappy navigation experience. The current/visible page must be highest priority, not merely first among requests attempted on the latest UI frame.

### A2 requirement

Give the newest current/visible render ownership a way to supersede stale queued work. A single Pdfium-owning worker remains preferred, but queue semantics should favor current source/generation/page over obsolete overscan. Acceptable approaches include a small coalescing scheduler/mailbox where current ownership is replaceable and overscan is separately bounded, or another deterministic latest-generation priority model. Do not create unbounded cancellation machinery and do not move Pdfium work onto the UI thread.

Add a deterministic test in which stale queued work exists, then a new current-page/generation request arrives, and verify the new current render is not forced behind an entire obsolete queue.

## Missing acceptance coverage

The goal contract explicitly required automated coverage for landscape/aspect-ratio dimension calculation, page navigation changing the owned visible render target, render failure terminalizing ownership, and visible/current page preservation under eviction pressure. The A1 report claims focused coverage, but the added `pdf_renderer.rs` tests only cover render-key identity, stale result rejection, bounded request page ordering, and raw channel capacity. A2 must add the missing deterministic ownership/policy tests rather than relying on human QA to discover these classes.

## A2 acceptance

A2 may return to director review when all of the following are true:

1. Zoom changes actual on-screen PDF page scale; zoom-in can exceed the viewport and scroll/pan naturally while reset/fit remains understandable.
2. Raster resolution follows zoom/presentation size through bounded quantized render keys rather than stretching one low-resolution page.
3. Resident texture eviction is deterministic and cannot evict the current page to preserve irrelevant pages.
4. Newly visible/current work supersedes or bypasses obsolete queued source/page/zoom work sufficiently to keep navigation responsive.
5. Source/page/zoom stale-result rejection and all A1 off-render-thread guarantees remain intact.
6. Landscape/aspect-ratio, navigation ownership, failure terminalization, current-page eviction pressure, and stale-queue priority have deterministic tests.
7. Focused/workspace/Windows gates and hosted renderer probe pass again.

Do not request human QA during A2. The director reviews the repository first.