# 0019 — Native PDF visual stability

## Current state

**REOPENED FOR A5 DIRECTOR CORRECTION — DO NOT REQUEST HUMAN QA YET**

A1/A2/A3 established the native Pdfium/egui renderer, real presentation-scale zoom, current-priority scheduling, stale-safe ownership, and deterministic texture residency. A4 correctly removed Quack-check/transcript recovery as a fatal prerequisite for visual PDF open, but director review found that PDF page navigation still inherits ordinary text-pagination ownership. A visual-only PDF therefore becomes a one-page Reader session even when the actual PDF has many pages.

Read these reviews in order:

- `docs/work/reviews/0019-a1-director-rejection.md`
- `docs/work/reviews/0019-a2-director-rejection.md`
- `docs/work/reviews/0019-a3-director-acceptance.md`
- `docs/work/reviews/0019-a3-real-desktop-rejection.md`
- `docs/work/reviews/0019-a4-director-rejection.md`

Continue the existing implementation/report lineage on `codex/0019-native-pdf-visual-stability` / `docs/work/reports/0019.md`.

## Outcome remains unchanged

Make PDF a real native reader surface in the authoritative Rust + `eframe`/`egui` application.

A readable PDF must enter the Reader without waiting for transcript/OCR/TTS recovery, show the actual native Pdfium-rasterized page, expose truthful native page count/navigation, support real zoom and scrolling, remain responsive under resize/navigation, and keep heavy PDF work off the egui/render thread.

## Accepted architecture to preserve

- Native Rust + egui + bundled/native Pdfium only. No Tauri, React, WebView, pdf.js, browser DOM overlays, or browser-owned rendering.
- Pdfium initialization, native PDF open/metadata work, page rasterization, bitmap conversion, and renderer CPU caching remain off the egui/render thread.
- The egui thread only consumes bounded metadata/results, computes bounded requests, uploads bounded textures, and composes controls/presentation.
- Visual PDF open must not depend on Quack-check, Python, Docling, OCR, transcript cache generation, or Gate 4 text recovery.
- Preserve A4's render-only degraded state (`PdfGeometryMode::RenderOnlyNoSync`, `PdfSyncStrategy::RenderOnly`) until later text recovery enriches it.
- Preserve source/generation/page/render-dimension request identity, stale-result rejection, duplicate coalescing, newest-current priority, real presentation-scale zoom, bounded/quantized raster dimensions, and deterministic viewport-aware residency.
- Preserve QA staging of repo-owned `scripts/quack-check` resources for later Gate 4 use.

## A5 blocking correction — native PDF page-domain ownership

A4 returns a visual-only PDF `SourceContent` with empty `tts_text`. Ordinary `ReaderSession::repaginate()` then creates one empty text page. `ReaderSnapshot.total_pages` is still `self.pages.len()`, and `SessionCommand::NextPage` / `PrevPage` / `SetPage` clamp against that text-page vector. The PDF UI uses the same canonical cursor and page count.

This means a real multi-page PDF is structurally stuck at page 1 even if Pdfium rendering itself works.

A5 must introduce explicit PDF page-domain ownership that is independent of text pagination.

### 1. Native page count is authoritative for visual PDF navigation

Obtain a trustworthy PDF page count from the native PDF open/metadata path, off the UI thread. The count may briefly be pending/unknown while native open completes, but once available it must become authoritative for PDF visual navigation.

Do not infer page count from extracted text, transcript pages, or `tts_text` pagination.

### 2. Reader PDF cursor must use the PDF page domain

For `PrettyKind::Pdf` / visual PDF sessions, canonical Reader `current_page`, `total_pages`, Next, Previous, SetPage, page label, bookmark page ownership, and renderer current-page request must operate in native PDF page coordinates.

Text/TTS sentence pagination remains a separate future synchronization domain and may be empty in Gate 3.

Do not create fake empty text pages merely to simulate PDF pages if a cleaner explicit page-domain representation is available.

### 3. Preserve visual-first open

Do not reintroduce Quack-check or transcript work into the visual-open critical path. A valid materialized/local PDF must still reach the native Reader when Quack-check/scripts/Python/Docling/OCR are unavailable.

### 4. Parse validity must be truthful

A `%PDF-` prefix alone is not proof that Pdfium can open a document. The authoritative native metadata/open path must distinguish a genuine parse/open failure from a valid renderable PDF.

A parse-invalid file may fail visual ownership or enter a bounded native-render error state, but it must not be falsely treated as a healthy multi-page document merely because the header matches.

### 5. Keep all heavy/native work off the render thread

Native open, page-count probing, Pdfium calls, transcript recovery, OCR, filesystem-heavy work, and rasterization stay off egui. Do not solve page-count ownership by opening/parsing the PDF inside `render_pdf_surface()` or any frame-time/UI callback.

## Deterministic acceptance coverage

Add tests proving:

1. a visual-only PDF session can represent a native page count greater than one while `tts_text` remains empty;
2. Next/Prev/SetPage move correctly across the PDF page domain and clamp at native PDF bounds;
3. `ReaderSnapshot.total_pages` reports native PDF page count for PDF sessions;
4. the PDF UI/render request follows the canonical PDF current page after navigation;
5. local and successfully materialized provider PDFs converge on the same page-domain contract after a path exists;
6. Quack-check/scripts deliberately unavailable still do not block visual PDF session ownership;
7. parse-invalid/header-only PDF input is distinguished from a valid native PDF;
8. source switching cannot leak stale page count or page imagery from the prior document;
9. all accepted A1/A2/A3 renderer/zoom/scheduler/residency tests remain green;
10. representative EPUB/non-PDF/TTS behavior remains green.

Use deterministic unit/session tests where a full native fixture is unsuitable, but include a real valid multi-page PDF fixture or hosted/native probe evidence sufficient to prove actual page-count extraction rather than mocked text pagination.

## Required validation

Before terminalizing A5:

- focused PDF source/session/page-domain + renderer tests;
- `cargo test -p lanternleaf-egui` and relevant app/core tests;
- `cargo check --workspace`;
- workspace tests and native Windows build gate;
- repo-native QA preparation;
- hosted Windows baseline workflow success including `native-workspace` and `hosted-renderer-probe`;
- `git diff --check`;
- no human QA request during implementation.

## Real-desktop recheck after director integration

After director source/CI acceptance, the next human pass begins narrowly:

1. open the same representative Caliberate PDF;
2. verify actual page 1 appears promptly;
3. verify the displayed total page count is believable and greater than one for a known multi-page PDF;
4. verify Next reaches page 2 and Prev returns to page 1;
5. only then continue with rapid navigation, zoom/scroll, resize responsiveness, multi-page residency, source switching, and one representative EPUB regression check.

## Explicit non-goals

Do not expand A5 into PDF TTS playback correctness, sentence/text-layer geometry mapping, spoken-sentence highlighting, OCR quality work, click-to-sentence mapping, text selection/copy parity, broad PDF settings redesign, Goal 0015, Goal 0017, Goal 0018, Natural/HD voices, or unrelated UI redesign.

## Repository handoff

- Repository goal: `0019-native-pdf-visual-stability`
- Branch: `codex/0019-native-pdf-visual-stability`
- This is a correction continuation under the same repository goal ID.
- Start from/synchronize current director `main` before implementation.
- Move this file `ready -> active` and re-arm the normal watcher for the fresh Codex Goal attempt.
- Preserve accepted A1/A2/A3 work and accepted A4 visual-first decoupling; implement the A5 native page-domain correction.
- Update `docs/work/reports/0019.md` with A5 implementation/test/CI evidence and exact commits.
- Terminalize to `done/` only after all required gates pass.
- Push before terminal signaling, restore shared checkout to `main`, and do not request human QA. The director reviews first.