# Goal 0019 A4 — director rejection

## Decision

**REJECTED BEFORE HUMAN QA — DO NOT INTEGRATE A4 YET**

A4 correctly fixes the real-desktop blocker that made Quack-check/transcript initialization a fatal prerequisite for visual PDF open. The visual-first direction is accepted: PDF source ingestion now establishes render-only degraded state without invoking Quack-check, and the QA harness stages the repo-owned Quack-check scripts for later Gate 4 use. Hosted Windows run `34847481675` passed both `native-workspace` and `hosted-renderer-probe`.

However, source review found a blocking navigation/state defect that makes the multi-page native PDF reader contract impossible even if page 1 renders successfully.

## Blocking defect — PDF visual navigation still inherits text-pagination ownership

A4 creates a render-only PDF `SourceContent` with an empty `tts_text`. `ReaderSession::repaginate()` therefore creates exactly one empty text page. `ReaderSnapshot.total_pages` remains `self.pages.len()`, and the PDF UI displays that value and dispatches ordinary `SessionCommand::NextPage` / `PrevPage`.

The ordinary session navigation logic clamps against `self.pages.len()`. With A4's empty visual-only source this means:

- `snapshot.total_pages == 1` by construction;
- `NextPage` returns immediately because `current_page + 1 >= self.pages.len()`;
- the native PDF render surface can request only page 0 through the canonical Reader cursor;
- a real multi-page PDF cannot navigate to page 2 even though Pdfium can render it.

The new A4 session test accidentally codifies this defect by asserting `snapshot.total_pages == 1` for a visual PDF instead of proving native PDF page-count/navigation ownership.

This violates Goal 0019's accepted outcome: stable real PDF page navigation and truthful page count are part of Gate 3, not Gate 4.

## Accepted A4 work to preserve

- Keep visual PDF open independent of Quack-check/Python/Docling/OCR/transcript availability.
- Keep `PdfGeometryMode::RenderOnlyNoSync` / `PdfSyncStrategy::RenderOnly` as the visual-only degraded contract until Gate 4 enriches it.
- Keep missing provider/materialization failure fatal when no local PDF path exists.
- Keep all Pdfium, page metadata probing, transcript, OCR, disk-heavy work, and rasterization off the egui/render thread.
- Keep QA staging of repo-owned `scripts/quack-check` resources.
- Preserve all accepted A1/A2/A3 renderer, zoom, scheduler, stale-result and residency work.

## Required A5 correction

Introduce explicit PDF page-domain ownership independent of text pagination.

A5 must provide the canonical Reader with a trustworthy native PDF page count (or a bounded pending/unknown state until obtained) from an off-UI native PDF metadata/open path. Once known, PDF `current_page`, `total_pages`, Set/Next/Prev behavior, render planning, bookmarks, and the displayed `Page X / Y` must use the PDF page domain rather than `tts_text` pagination.

Do not fake page count from text, do not run Pdfium on the egui thread, and do not restore Quack-check as a prerequisite. Prefer one authoritative native document/page metadata path rather than an unrelated second interpretation of the PDF.

A5 should also make the parse-validity boundary truthful. A file that merely starts with `%PDF-` is not necessarily a readable PDF. The native metadata/render path should distinguish a genuine parse/open failure from a valid renderable document without blocking visual open on transcript recovery.

## Deterministic acceptance coverage

Add tests proving at minimum:

1. a visual-only multi-page PDF session can represent a native page count greater than one without transcript text;
2. Next/Prev/SetPage operate across the PDF page domain while text/TTS remains unavailable;
3. `ReaderSnapshot.total_pages` and the PDF page label use the native PDF page count;
4. render planning follows the PDF current page after navigation;
5. local and successfully materialized Caliberate PDFs converge on the same page-domain ownership after a path exists;
6. Quack-check unavailable still does not block visual PDF ownership;
7. a parse-invalid PDF does not masquerade as a valid one merely because it has a `%PDF-` prefix;
8. all accepted renderer/scheduler/zoom/residency/stale-source tests remain green.

## Integration status

A4 branch head is not accepted into `main`. Continue the existing repository Goal 0019 lineage with a fresh Codex Goal attempt. The director will review A5 before any further human PDF QA.