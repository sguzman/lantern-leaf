# 0019 — Native PDF visual stability

## Current state

**REOPENED FOR A7 DIRECTOR CORRECTION — DO NOT REQUEST HUMAN QA YET**

A1-A3 established the native Pdfium/egui renderer, real presentation-scale zoom, current-priority scheduling, stale-safe ownership, and deterministic texture residency. A4 removed Quack-check/transcript recovery as a prerequisite for visual open. A5 added truthful native PDF page-domain ownership. A6 correctly consolidated Gate-3 Pdfium work behind one shared native service and added effect panic containment, but director review found an idle-worker wakeup bug that can still strand source opening forever.

Read `docs/work/reviews/0019-a6-director-rejection.md` before implementation.

Continue the existing report lineage in `docs/work/reports/0019.md`.

## Outcome remains unchanged

Make PDF a real native reader surface in the authoritative Rust + `eframe`/`egui` application.

A valid local/materialized PDF must enter the Reader without transcript/OCR prerequisites, show the actual Pdfium-rasterized page, expose truthful native page count/navigation, support real zoom/scrolling, remain responsive, and never run heavy/native PDF work on the egui/render thread.

## Accepted architecture to preserve

- Native Rust + egui + native/bundled Pdfium only. No WebView, pdf.js, browser DOM, Tauri, or React production fallback.
- Visual PDF open remains independent of Quack-check, Python, Docling, OCR, and transcript recovery.
- Preserve explicit native PDF page-domain ownership from A5.
- Preserve A6's **single authoritative `PdfNativeService` / single native Pdfium owner** for metadata and rasterization. Do not regress to a second binding or renderer on an effect thread.
- Preserve source/generation/page/render-size identity, stale-result rejection, duplicate coalescing, newest-current render priority, real presentation-scale zoom, bounded/quantized raster dimensions, deterministic viewport-aware texture residency, current-page pinning, and source-switch safety.
- Preserve A6 detached-effect panic terminalization.

## A7 blocking correction — idle metadata liveness

### 1. Fix the native-service wait topology

The A6 worker checks `metadata_rx.try_recv()` only before entering an inner raster-scheduler wait loop. That inner loop never returns to metadata polling when no raster key exists; a metadata notification or 10 ms timeout simply wakes and waits again.

This breaks the normal production lifetime `service starts -> becomes idle -> user opens PDF -> metadata request`.

Refactor request arbitration so an already-idle native service always services a newly-arrived metadata request promptly without requiring a raster request to kick the worker.

A single typed native request queue is acceptable and may be preferable, provided current source-open metadata cannot sit indefinitely behind stale/nearby raster work. If retaining separate metadata/raster structures, the wait loop must explicitly re-check metadata before sleeping again and must not lose notifications.

### 2. Preserve one native owner

All Pdfium initialization, parse/open validation, native page-count lookup, and rasterization remain serialized through the one shared native owner/service.

Do not solve A7 by creating another `NativePdfRenderer`, another Pdfium binding, or doing native work on egui.

### 3. Bounded current-open priority

Metadata for the source currently being opened must have bounded priority over obsolete nearby raster backlog. Repeated source opens and source switching must not leak stale metadata or imagery.

### 4. Keep failures terminal

Malformed/native-open failure returns a normal terminal source-open error. The detached effect panic boundary remains intact. A dead/stopped native service must produce a bounded failure rather than eternal `SourceLoading`.

### 5. Make container precheck bounded

`validate_pdf_source()` currently reads the full PDF merely to test `%PDF-` and `%%EOF`. Replace that with a bounded header/tail check or rely on the authoritative native parse path. Do not impose an O(file-size) duplicate read before Pdfium parses a large PDF.

This work remains off the UI thread either way.

## Deterministic acceptance coverage

Add tests proving at minimum:

1. **production idle lifecycle:** start the actual shared native service, deliberately wait long enough for its worker to settle idle, then request metadata for a real valid multi-page PDF and require bounded completion;
2. after that metadata result, raster page 1 through the same owner and verify success/thread identity;
3. **repeated idle cycle:** metadata succeeds, service returns idle, then a second metadata request succeeds without any raster request being needed to wake it;
4. repeated PDF opens do not create a second Pdfium owner/binding and do not hang;
5. source switching cannot leak stale metadata/page imagery;
6. malformed/header-only input returns a terminal failure;
7. visual-only PDF still opens with Quack-check/scripts/Python/Docling/OCR unavailable;
8. PDF Next/Prev/SetPage and `ReaderSnapshot.total_pages` remain native-page-domain correct;
9. accepted A1-A6 zoom/scheduler/stale/residency/panic-boundary tests remain green;
10. representative EPUB/non-PDF/TTS behavior remains green.

The idle-lifecycle regression must remove the startup race that allowed A6 CI to pass: insert a deterministic delay or explicit idle witness before the first metadata request.

## Required validation

Before terminalizing A7:

- focused shared-PDF-service idle-liveness / source-open / page-domain / renderer tests;
- `cargo test -p lanternleaf-egui` plus relevant app/core tests;
- `cargo test --workspace -- --test-threads=1`;
- `cargo check --workspace`;
- native Windows build and repo-native QA preparation;
- hosted Windows baseline success including `native-workspace` and `hosted-renderer-probe`;
- hosted/native probe must include the deliberate `start -> idle -> metadata -> raster -> idle -> metadata` production lifecycle;
- `git diff --check`;
- no human QA request during implementation.

## Real-desktop recheck after director integration

The next human pass remains deliberately narrow:

1. open the same representative Caliberate PDF;
2. actual page 1 must appear promptly;
3. displayed total page count must be believable and >1 for that known multi-page PDF;
4. Next must visibly reach page 2 and Prev return to page 1;
5. only then test rapid navigation, zoom/scroll, resize, source switching, and one representative EPUB regression.

## Explicit non-goals

Do not expand A7 into Gate 4 PDF TTS/highlighting/text geometry/OCR quality, text selection/copy parity, Goal 0015, Goal 0017, Goal 0018, Natural/HD voices, or unrelated UI work.

## Repository handoff

- Repository goal: `0019-native-pdf-visual-stability`
- Branch: continue/recreate `codex/0019-native-pdf-visual-stability` from current director `main` as the normal Codex workflow requires.
- This is a correction continuation under the same repository goal ID.
- Move this file `ready -> active`, re-arm the normal watcher, implement A7, update `docs/work/reports/0019.md`, validate, terminalize to `done/`, push before signaling, and restore the shared checkout to `main`.
- Do not request human QA. The director reviews first.