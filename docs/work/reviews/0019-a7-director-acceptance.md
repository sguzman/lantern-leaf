# Goal 0019 A7 — director source/CI acceptance

## Decision

**ACCEPTED FOR NARROW REAL-DESKTOP RECHECK — GOAL 0019 REMAINS OPEN PENDING PHYSICAL QA**

A7 corrects the A6 idle-service liveness defect without regressing the accepted native PDF architecture.

## Source review

The native PDF runtime still has one authoritative `PdfNativeService` / one native Pdfium owner. Source-open metadata and page raster work remain serialized through that service; no effect-thread Pdfium binding is reintroduced.

The A7 worker correction exits the empty raster wait after one bounded wait and returns to top-level arbitration, where metadata is checked first. This closes the production lifetime failure `service starts -> becomes idle -> user opens PDF -> metadata request remains stranded` that invalidated A6.

Metadata therefore retains priority over raster work while current-page raster requests preserve the accepted A1-A6 scheduling, stale-result, zoom, and residency behavior.

The PDF container precheck is also bounded now: a small header read plus at most a 64 KiB tail read replaces the earlier full-file duplicate read. Pdfium remains authoritative for actual parse/page-count/raster validity.

A6's detached-effect panic boundary remains intact: unexpected effect panics terminalize as failure instead of silently leaving `SourceLoading` forever.

## Deterministic regression coverage

The shared native lifecycle probe now deliberately lets the service settle idle before the first metadata request, then verifies native two-page metadata, malformed/header-only rejection, page-1 rasterization through the same native worker, a second deliberate idle period, and a second successful metadata request. Metadata and raster results are asserted to come from the same native worker thread.

This directly covers the startup-race hole that allowed A6 CI to pass while the real desktop could still hang.

## CI evidence

Hosted Windows workflow `34867657265` passed both `native-workspace` and `hosted-renderer-probe`. The renderer probe includes the deliberate idle shared-service metadata+raster lifecycle check before the native startup probe.

## Remaining gate

Source/CI review is accepted, but Goal 0019 is not closed until the same representative real Caliberate PDF passes the narrow physical sequence:

1. page 1 actually appears promptly;
2. displayed native page count is believable and greater than one;
3. Next reaches page 2;
4. Previous returns to page 1.

Only after those four pass should broader zoom/scroll/resize/source-switch testing continue.
