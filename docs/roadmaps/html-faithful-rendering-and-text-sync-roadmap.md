# HTML Faithful Rendering and Text Sync Roadmap

## Objective
- [x] Render HTML sources faithfully in Pretty Text mode with layout, links, images, and document structure preserved.
- [x] Keep Text-only mode strictly bound to extracted plain text with no HTML styling noise.
- [x] Ensure TTS, normalization, sentence splitting, and playback control are driven only by Text-only plain text.
- [x] Keep Pretty Text HTML and Text-only/TTS positions synchronized deterministically.

## Phase 1: Source Contracts and Ownership
- [x] Define HTML source contract with two canonical payloads:
- [x] `pretty_html: String` for Pretty Text rendering.
- [x] `tts_text: String` for Text-only rendering and TTS ownership.
- [x] Document that `tts_text` is the only input to normalization, sentence planning, and audio playback.
- [x] Add tracing fields showing chosen payload source, source type, and sync mode.

## Phase 2: HTML Ingestion Pipeline
- [x] Build a dedicated HTML ingest path that outputs `pretty_html` and `tts_text` side by side.
- [x] Preserve document structure in `pretty_html` including headings, paragraphs, lists, tables, blockquotes, figures, and inline emphasis.
- [x] Extract `tts_text` from the same HTML source with scripts, styles, boilerplate, and non-readable artifacts removed.
- [x] Normalize whitespace and block boundaries in `tts_text` so sentence segmentation is stable.
- [x] Add tracing spans for ingest duration, text extraction outcome, and fallback decisions.

## Phase 3: Pretty Text HTML Rendering
- [x] Render `pretty_html` natively in Pretty Text mode without markdown conversion.
- [x] Preserve internal anchors and section navigation behavior.
- [x] Preserve external links with safe browser handling.
- [x] Render inline images, figures, captions, and tables from sanitized HTML.
- [x] Respect source HTML/CSS where safe, while preventing global style bleed into the app shell.
- [x] Add tracing for sanitizer decisions, stripped nodes, and asset rewrite outcomes.

## Phase 4: Text-only Rendering and TTS Ownership
- [x] Text-only mode renders only `tts_text`.
- [x] Sentence splitting runs only against `tts_text`.
- [x] TTS playback plans are generated only from `tts_text`.
- [x] Pretty Text/Text-only toggles do not alter sentence indices, playback position, bookmarks, or search ownership.
- [x] Add explicit tracing proving each playback step originated from `tts_text`.

## Phase 5: HTML-to-Text Sync Mapping
- [x] Build a persistent mapping between `tts_text` sentence indices and Pretty Text HTML anchors.
- [x] Prefer text-evidence-based mapping over proportional heuristics.
- [x] Keep mapping monotonic so playback cannot jump backward or several sections ahead unexpectedly.
- [x] Allow paragraph-level or sentence-level HTML anchors, but require deterministic fallback behavior.
- [x] Persist sync artifacts in cache per source and page/chunk.
- [x] Add tracing for mapping hits, low-confidence matches, fallback drifts, and missing anchors.

## Phase 6: Playback Highlight and Scroll Behavior
- [x] Highlight the currently spoken unit in Pretty Text mode based on the HTML sync map.
- [x] Keep scroll ownership stable within the same mapped paragraph/anchor.
- [x] Only auto-scroll when playback advances to a new mapped HTML anchor, page, or explicit jump target.
- [x] Keep Text-only and Pretty Text highlight positions aligned to the same `tts_text` cursor.
- [x] Add tracing for highlight target resolution and scroll trigger reasons.

## Phase 7: Cache, Recovery, and Migration
- [x] Extend cache layout to store `pretty_html`, `tts_text`, and HTML sync mapping artifacts.
- [x] Add cache versioning for HTML dual-payload entries.
- [x] Recover cleanly from missing or corrupted HTML/text artifacts by rebuilding them non-destructively.
- [x] Ensure delete/reopen flows remove and rebuild all HTML-related artifacts consistently.
- [x] Add tracing around cache reads, writes, invalidation, and rebuilds.

## Phase 8: Validation and Regression Coverage
- [x] Unit tests for HTML sanitization and safe rendering behavior.
- [x] Unit tests for plain-text extraction from representative HTML documents.
- [x] Unit tests for sentence-to-HTML anchor mapping and fallback behavior.
- [x] Integration tests for playback continuity across Pretty Text and Text-only mode switches.
- [x] Regression tests for documents with tables of contents, internal anchors, repeated headings, images, and footnotes.
- [x] Manual QA checklist covering faithful rendering, text-only cleanliness, and playback sync stability.

## Acceptance Criteria
- [x] Pretty Text mode renders HTML documents faithfully enough that source structure and visuals are preserved.
- [x] Text-only mode shows only clean extracted text.
- [x] TTS, normalization, and playback indexing are fully owned by `tts_text`.
- [x] Pretty Text highlight/scroll stays aligned with the Text-only/TTS cursor during playback.
- [x] Full project build verification passes after implementation, excluding `deb`, `rpm`, and AppImage packaging targets.
