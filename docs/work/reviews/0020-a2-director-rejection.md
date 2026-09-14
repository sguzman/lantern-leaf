# Goal 0020 A2 — director rejection

## Decision

**REJECTED BEFORE HUMAN QA — REOPEN AS A3**

A2 resolves most of the A1 structural concerns: real visible-page publication, native page metadata, cached/indexed continuous geometry, horizontal scrolling, semantic viewport witnesses, real fit calculations, and hosted Windows validation are all substantial improvements.

However, production source review found a deterministic render-identity/lifecycle defect that makes the continuous viewport unsafe to send to physical QA.

## Blocking defect 1 — planner render keys do not match presentation lookup keys

The authoritative render scheduler and the Reader compute raster width from different coordinate systems.

`request_authoritative_pdf_renders()` currently derives raster width from:

`quantized_render_width(pdf_viewport_width, scale)`

where manual mode uses `scale = zoom_level` and fit modes use `scale = 1.0`.

The Reader presentation instead looks up textures using:

`quantized_render_width(PDF_BASE_PAGE_WIDTH, effective_zoom)`

These are not generally the same key.

Examples:

- Manual 100% in a 1200 px viewport: scheduler requests approximately 1200 px, while the Reader looks for approximately 816 px.
- Manual 200% has the same mismatch at a larger scale.
- Fit page usually requests viewport-width raster identity while the Reader looks for the smaller fit-page effective width.
- Fit width happens to align algebraically in ordinary cases, but that accidental special case does not repair the general contract.

Because `PdfRenderKey` includes width, a successfully rendered image under the scheduler key is not found under the Reader key. The UI can therefore remain on `Rendering page ...` even though the native worker completed a raster.

A2's initial default render can partially mask the defect because `pdf_viewport_width` begins near the base-page width and initial overscan may pre-render adjacent pages at a matching quantized width. Scrolling farther or changing zoom can expose the mismatch later, which makes this particularly unsuitable for human QA.

## Blocking defect 2 — zoom/fit generation changes are not part of viewport commit identity

The Reader increments `pdf_generation` and clears/invalidates presentation textures when manual zoom, Reset, Fit width, or Fit page changes.

But the canonical viewport scheduler's `should_commit_viewport_update()` decides repeated work only from visible/overscan page ranges plus a narrow trigger. If the user changes zoom while remaining on the same visible pages, the target ranges remain equal and the scheduler may reject the update as `repeat_target`.

For `+` / `-`, `apply_zoom_level()` clears `last_viewport_update`, but `should_commit_viewport_update()` still rejects an unchanged visible/overscan target because `Init` is not treated as a forced geometry/render-spec change.

For Reset/Fit mode changes, the generation changes without even resetting the viewport commit identity.

The result can be:

- old-generation textures become unusable;
- the Reader asks for new-generation/new-width keys;
- no new raster requests are issued until a later page-range change happens to force another scheduler commit.

So a zoom control can blank the document while the user remains stationary.

## Accepted A2 work to preserve

A3 should preserve, not redesign:

- the continuous immediate-mode page stack;
- one authoritative native Pdfium owner/service;
- actual continuous visible-page publication to the canonical planner;
- off-thread native page metadata and stable page aspect ratios;
- cached/indexed geometry and binary-search visibility lookup;
- horizontal + vertical egui navigation;
- semantic viewport witness capture/restore;
- broad 25–400% manual zoom ladder;
- real viewport/page-dimension fit calculations;
- visible-page residency protection;
- hosted Windows validation discipline.

## A3 correction direction

Use one canonical production render specification shared by planning, scheduling, cache identity, and presentation. At minimum it must contain the effective presentation/raster scale or final quantized raster width/height together with source/generation/page identity.

The Reader must never independently derive a different `PdfRenderKey` than the authoritative scheduler.

Viewport commit/replan identity must include geometry/render-spec changes such as generation, zoom mode/effective zoom, raster width, viewport-size changes, or an explicit render-spec revision. A change in render specification must force new authoritative raster work even when visible/overscan page ranges are unchanged.

Temporary scaling of an old texture while a replacement arrives is acceptable, but stale generation/spec results must never become authoritative.

## Required regression evidence

A3 needs production-path tests proving at least:

1. scheduler request keys exactly match Reader lookup/presentation keys for manual 100%, manual high zoom, Fit width, and Fit page;
2. zooming in place with unchanged visible pages still schedules the new generation/render specification;
3. Reset/100% in place schedules replacement rasters;
4. Fit width and Fit page in place schedule replacement rasters;
5. a completed authoritative render becomes discoverable by the presentation key;
6. stationary zoom cannot leave the document permanently blank;
7. continuous visible/overscan planning, focal anchoring, metadata geometry, residency, stale safety, Goal 0019 regressions, EPUB/TTS regressions, and hosted Windows jobs remain green.

## Human QA

Do not request another real-desktop pass until A3 passes director source/CI review.
