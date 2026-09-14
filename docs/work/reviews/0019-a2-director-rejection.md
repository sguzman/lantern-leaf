# Goal 0019 A2 — director rejection

## Decision

**REJECTED BEFORE HUMAN QA — CONTINUE AS A3**

A2 fixes the three intended architectural defects from A1: real presentation-scale zoom, a coalescing/prioritized single-owner render scheduler, and a deterministic viewport-aware texture-eviction policy. The A2 Windows workflow also passed. However, the production Reader still contains the old arbitrary HashMap eviction loop *before* the new deterministic eviction policy runs, so the central current-page residency guarantee is not actually enforced at runtime.

Do not integrate the implementation branch or request physical Windows QA yet.

## Accepted A2 work to preserve

- Preserve the dedicated native Pdfium worker; PDF open/load/raster/bitmap conversion/renderer CPU cache remain off the egui/render thread.
- Preserve source + generation + page + render-dimension identity and stale-result rejection.
- Preserve the bounded coalescing scheduler and newest-current priority semantics.
- Preserve separation of presentation size from quantized/bounded raster resolution so zoom changes real logical page size.
- Preserve `PdfResidentEntry` and `choose_resident_texture_evictions()` as the deterministic residency policy.
- Preserve the new deterministic tests for zoom/presentation size, landscape aspect, navigation key identity, failure terminalization, current-page eviction pressure, duplicate coalescing, and stale-queue/current priority.
- Preserve passing hosted Windows workflow `34840618165` (`native-workspace` and `hosted-renderer-probe`).

## Blocking defect — legacy arbitrary texture eviction still runs first

In `render_pdf_surface()`, after draining worker results and inserting newly completed textures, the A1 loop is still present:

```rust
while self.pdf_textures.len() > 8 {
    if let Some(key) = self.pdf_textures.keys().next().cloned() {
        self.pdf_textures.remove(&key);
    }
}
```

Later in the same function A2 correctly builds `PdfResidentEntry` values, pins `current_key`, marks viewport keep pages, and calls `choose_resident_texture_evictions(..., 8)`.

But the legacy loop executes first. Because `HashMap::keys().next()` is arbitrary, it can already remove the current visible page or a useful near-page texture before the deterministic policy sees the resident set. The later viewport-aware eviction cannot restore a texture that has already been arbitrarily discarded.

Therefore A2's pure policy test is valid in isolation, but the production path still violates the Goal 0019 invariant:

> current/visible page is never evicted merely to preserve farther pages.

This can produce avoidable `Rendering PDF page…` flashes/churn during result bursts or navigation, exactly the failure class A2 was supposed to remove.

## A3 correction

A3 is intentionally narrow:

1. Remove the legacy arbitrary `HashMap::keys().next()` texture-eviction loop from the production PDF surface.
2. Make the deterministic viewport-aware residency policy the **only** production path that bounds `pdf_textures`.
3. Keep `pdf_render_errors` bounded, but if its current arbitrary trimming remains, make clear that it is error-diagnostic retention only and cannot affect visible texture ownership. A deterministic retention order is preferable if easy, but do not expand scope unnecessarily.
4. Add an integration-level/pure production-policy regression that exercises the same sequence as the Reader: insert more than capacity including the current key, apply the actual residency-bounding helper/path, and prove the current key remains resident while irrelevant entries are removed. Do not rely solely on testing `choose_resident_texture_evictions()` detached from the caller.
5. Preserve every accepted A1/A2 worker, zoom, scheduler, stale-safety, and off-render-thread guarantee.
6. Re-run focused/workspace/Windows gates and hosted renderer probe.

## A3 acceptance

A3 may return to director review when:

- there is no arbitrary pre-eviction of `pdf_textures` in `render_pdf_surface()` or an equivalent production path;
- one deterministic viewport-aware policy exclusively enforces the texture-capacity bound;
- the current visible texture is provably pinned under an over-capacity production-style sequence;
- all A2 tests and native/Windows gates remain green.

Do not request human QA during A3. The director reviews the repository first.