# Goal 0019 A5 — director acceptance

## Decision

**ACCEPTED FOR REAL-DESKTOP QA — INTEGRATED TO `main`**

A5 resolves the director-blocking page-domain defect from A4 while preserving the accepted native PDF visual architecture and visual-first source-open correction.

## Accepted implementation

- PDF visual open remains independent of Quack-check, Python, Docling, OCR, transcript cache generation, and Gate 4 text recovery.
- Native Pdfium metadata probing now validates/open-parses the PDF and obtains authoritative page count before the Reader session is published.
- Native PDF metadata probing runs through the off-UI effect dispatcher, not the egui/render thread.
- `ReaderSession` owns an explicit PDF page count separate from transcript/text pagination.
- PDF `current_page`, `total_pages`, Next/Prev/SetPage bounds, bookmark page ownership, page label, and render requests now operate in native PDF page coordinates.
- Empty transcript/TTS state no longer manufactures a one-page visual PDF domain.
- Header-shaped but parse-invalid PDFs are rejected by the authoritative native Pdfium metadata path.
- Accepted A1/A2/A3 source/generation/page/size identity, current-priority scheduling, real presentation-scale zoom, bounded worker behavior, stale-result rejection, and deterministic current-page-pinned residency remain intact.
- A4 QA staging of repo-owned Quack-check scripts remains available for later Gate 4 work without becoming a visual-open prerequisite.

## Review evidence

A5 implementation commit `bad7fe37d5bbcbcdeb1592a7495c9eefa1c25cdc` adds explicit `pdf_page_count` session ownership, PDF-domain navigation bounds, native page-count probing through bundled Pdfium, and deterministic valid-two-page/header-only probe coverage.

The full terminal branch is fast-forward integrated to `main` at `8227778f9d642071c8ee9e0e738c51e42db303e0`.

Hosted Windows workflow `34850668093` passed both `native-workspace` and `hosted-renderer-probe`, including workspace check/build/test, QA preparation, Windows TTS probe, and native renderer capability probing.

## Remaining gate

Goal 0019 is **not closed yet**. It now requires the focused real-desktop pass:

1. open the same representative Caliberate PDF;
2. verify actual page 1 appears promptly;
3. verify the displayed total page count is believable and greater than one for a known multi-page PDF;
4. verify Next reaches page 2 and Prev returns to page 1;
5. then continue with rapid navigation, zoom/scroll, resize responsiveness, multi-page residency, source switching, and one representative EPUB regression check.

PDF TTS/highlight/OCR synchronization remains Gate 4 and is not part of Goal 0019 closure.
