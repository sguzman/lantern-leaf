# Goal 0020 A3 — director acceptance

## Decision

**ACCEPTED FOR REAL-DESKTOP QA — GOAL 0020 REMAINS OPEN PENDING PHYSICAL EVIDENCE**

A3 repairs the render-spec lifecycle defects found in A2 without regressing the accepted continuous-native direction.

## Accepted implementation evidence

- Production now uses one canonical `PdfRenderSpec` for source, generation, and final quantized raster dimensions.
- Authoritative scheduler requests and Reader presentation lookup both derive exact `PdfRenderKey`s through the same `spec.key_for(page)` path.
- Render-result acceptance is exact against the current render specification, so old-width/old-generation results cannot become authoritative after zoom, fit, resize, or source changes.
- Render-spec identity participates in viewport commit identity. A render-spec change forces a commit even when visible and overscan page ranges remain unchanged.
- Manual zoom, Reset/100%, Fit width, Fit page, and fit-scale changes from geometry/resize therefore schedule bounded replacement raster work without requiring a scroll event.
- A2's accepted continuous viewport remains intact: actual visible-page publication, native metadata-derived page geometry, cached/indexed visibility lookup, horizontal and vertical scrolling, semantic viewport witnesses, wider manual zoom, fit modes, and visible-page residency protection.
- Heavy/native PDF work remains owned by the native Pdfium service off the egui/render thread.

## Validation

A3 implementation commit `39fff7ab0d7ec6b488396b277494d5ae8681cee1` passed hosted Windows workflow run `34879006018`:

- `native-workspace` — success;
- `hosted-renderer-probe` — success, including shared native PDF metadata/raster lifecycle and renderer capability probe.

The terminal branch commit is `665f2063c99ee80ece066cab0cb2e65d8a722770`; its post-CI changes are goal terminalization/report bookkeeping rather than a new implementation change.

## Director source-review notes

The previous deterministic mismatch is gone: scheduler and presentation no longer independently calculate raster width. Stationary geometry/render-spec changes now force authoritative replacement work and exact-current-spec completions are the only results admitted to presentation.

The remaining risk is interaction quality that is inherently best judged on the real desktop: continuous boundary scrolling, focal-anchor feel during zoom/fit/resize, high-zoom horizontal navigation, and behavior under rapid scrolling through the long real PDF.

## Physical QA gate

Use the already-proven long Caliberate PDF first. Verify continuous page crossing, adjacent-page visibility, rapid scrolling, explicit Next/Previous jumps, Fit width, Fit page, Reset/100%, broad manual zoom, horizontal navigation at high zoom, and resize/zoom focal stability. Then briefly verify representative EPUB rendering/TTS remains green.

If those pass, Goal 0020 can close and Gate 4 PDF text/TTS/highlight work can be promoted.
