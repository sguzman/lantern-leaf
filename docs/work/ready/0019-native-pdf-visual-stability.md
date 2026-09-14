# 0019 — Native PDF visual stability

## Current state

**REOPENED FOR A6 DIRECTOR CORRECTION — DO NOT REQUEST HUMAN QA YET**

A1-A3 established the native Pdfium/egui renderer, real presentation-scale zoom, current-priority scheduling, stale-safe ownership, and deterministic texture residency. A4 removed Quack-check/transcript recovery as a prerequisite for visual open. A5 added truthful native PDF page-domain ownership, but the first real-desktop A5 recheck exposed a deeper production-lifetime defect: the app can create two independent Pdfium owners/bindings, and an unexpected effect-thread panic can strand the shell forever in `SourceLoading`.

Read `docs/work/reviews/0019-a5-real-desktop-rejection.md` before implementation.

Continue the existing report lineage on `docs/work/reports/0019.md`.

## Outcome remains unchanged

Make PDF a real native reader surface in the authoritative Rust + `eframe`/`egui` application.

A valid local/materialized PDF must enter the Reader without transcript/OCR prerequisites, show the actual Pdfium-rasterized page, expose truthful native page count/navigation, support real zoom/scrolling, remain responsive, and never run heavy/native PDF work on the egui/render thread.

## Accepted architecture to preserve

- Native Rust + egui + native/bundled Pdfium only. No WebView, pdf.js, browser DOM, Tauri, or React production fallback.
- Visual PDF open is independent of Quack-check, Python, Docling, OCR, and transcript recovery.
- Preserve explicit native PDF page-domain ownership from A5.
- Preserve source/generation/page/render-size identity, stale-result rejection, duplicate coalescing, newest-current render priority, real presentation-scale zoom, bounded/quantized raster dimensions, and deterministic viewport-aware texture residency.
- Preserve current-page pinning and bounded residency.
- Preserve source switching safety.

## A6 blocking correction — one native Pdfium owner

### 1. One Pdfium owner/service for the process

Refactor the native PDF runtime so LanternLeaf has one authoritative process-wide native PDF service/worker that owns Pdfium initialization and all Pdfium calls required by Gate 3.

That single owner must handle at least:

- native open/parse validation;
- native page-count metadata;
- page rasterization/bitmap conversion;
- renderer-side native cache state where appropriate.

Do **not** construct a fresh `NativePdfRenderer` or fresh `Pdfium` binding on the source-open effect thread merely to obtain metadata.

Pdfium/native calls must be serialized through the owner/service. A typed cloneable handle may be shared by the effect layer and egui PDF presentation, but ownership of the native object itself remains singular.

### 2. Metadata request/result contract

Source opening must request PDF metadata through the shared native service and receive a bounded typed result off the UI thread.

A successful metadata result must include truthful native page count and parse/open validity sufficient for A5 page-domain ownership.

A malformed/native-open failure must return a normal source-open error event rather than panic or hang.

Metadata/current-open work must have suitable priority so opening the current PDF cannot wait indefinitely behind stale/nearby raster requests.

### 3. Keep visual-first semantics

Do not reintroduce Quack-check/transcript/OCR work into visual source open. Once a provider/local path exists, native visual ownership depends only on the native PDF service for Gate 3.

### 4. No heavy work on egui

Pdfium initialization, library binding, PDF open/validation, metadata lookup, rasterization, bitmap conversion, filesystem-heavy work, transcript recovery, and OCR remain off the egui/render thread.

The UI thread may submit bounded requests, consume results, upload bounded textures, and compose immediate-mode presentation only.

### 5. Effect panic containment

`EffectDispatcher` currently executes effects in detached threads. Wrap effect execution in a bounded panic boundary so an unexpected panic cannot silently terminate a worker task and leave product state stuck forever.

A panic must be converted into a terminal `CommandFailed`/appropriate operation failure event with bounded diagnostics. Preserve ordinary result/error handling.

This resilience boundary does **not** replace the single-Pdfium-owner correction.

## Deterministic acceptance coverage

Add tests proving at minimum:

1. the production native PDF service is initialized once and can serve metadata then raster requests through the same owner;
2. with that service already alive, a real valid multi-page PDF returns native page count and then renders page 1 successfully;
3. repeated source opens do not attempt a second Pdfium binding/owner and do not hang;
4. switching between two PDFs cannot leak stale metadata/page imagery;
5. malformed/header-only input returns a terminal source-open failure;
6. visual-only PDF still opens with Quack-check/scripts/Python/Docling/OCR unavailable;
7. PDF Next/Prev/SetPage and `ReaderSnapshot.total_pages` remain native-page-domain correct;
8. accepted A1-A5 zoom/scheduler/stale/residency tests remain green;
9. a synthetic panic inside detached effect execution yields a terminal failure event rather than an eternal in-progress state;
10. representative EPUB/non-PDF/TTS behavior remains green.

The native lifecycle test must exercise the production ownership topology. An isolated helper that creates its own renderer is not sufficient evidence.

## Required validation

Before terminalizing A6:

- focused shared-PDF-service / source-open / page-domain / renderer tests;
- `cargo test -p lanternleaf-egui` plus relevant app/core tests;
- `cargo test --workspace`;
- `cargo check --workspace`;
- native Windows build and repo-native QA preparation;
- hosted Windows baseline success including `native-workspace` and `hosted-renderer-probe`;
- hosted/native probe must cover shared-service metadata + raster lifecycle, not only isolated renderer construction;
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

Do not expand A6 into Gate 4 PDF TTS/highlighting/text geometry/OCR quality, text selection/copy parity, Goal 0015, Goal 0017, Goal 0018, Natural/HD voices, or unrelated UI work.

## Repository handoff

- Repository goal: `0019-native-pdf-visual-stability`
- Branch: continue/recreate `codex/0019-native-pdf-visual-stability` from current director `main` as the normal Codex workflow requires.
- This is a correction continuation under the same repository goal ID.
- Move this file `ready -> active`, re-arm the normal watcher, implement A6, update `docs/work/reports/0019.md`, validate, terminalize to `done/`, push before signaling, and restore the shared checkout to `main`.
- Do not request human QA. The director reviews first.
