# HTML Faithful Rendering and Text Sync Roadmap

## Objective
- [ ] Render HTML sources faithfully in Pretty Text mode with layout, links, images, and document structure preserved.
- [ ] Keep Text-only mode strictly bound to extracted plain text with no HTML styling noise.
- [ ] Ensure TTS, normalization, sentence splitting, and playback control are driven only by Text-only plain text.
- [ ] Keep Pretty Text HTML and Text-only/TTS positions synchronized deterministically.

## Phase 1: Source Contracts and Ownership
- [ ] Define HTML source contract with two canonical payloads:
- [ ] `pretty_html: String` for Pretty Text rendering.
- [ ] `tts_text: String` for Text-only rendering and TTS ownership.
- [ ] Document that `tts_text` is the only input to normalization, sentence planning, and audio playback.
- [ ] Add tracing fields showing chosen payload source, source type, and sync mode.

## Phase 2: HTML Ingestion Pipeline
- [ ] Build a dedicated HTML ingest path that outputs `pretty_html` and `tts_text` side by side.
- [ ] Preserve document structure in `pretty_html` including headings, paragraphs, lists, tables, blockquotes, figures, and inline emphasis.
- [ ] Extract `tts_text` from the same HTML source with scripts, styles, boilerplate, and non-readable artifacts removed.
- [ ] Normalize whitespace and block boundaries in `tts_text` so sentence segmentation is stable.
- [ ] Add tracing spans for ingest duration, text extraction outcome, and fallback decisions.

## Phase 3: Pretty Text HTML Rendering
- [ ] Render `pretty_html` natively in Pretty Text mode without markdown conversion.
- [ ] Preserve internal anchors and section navigation behavior.
- [ ] Preserve external links with safe browser handling.
- [ ] Render inline images, figures, captions, and tables from sanitized HTML.
- [ ] Respect source HTML/CSS where safe, while preventing global style bleed into the app shell.
- [ ] Add tracing for sanitizer decisions, stripped nodes, and asset rewrite outcomes.

## Phase 4: Text-only Rendering and TTS Ownership
- [ ] Text-only mode renders only `tts_text`.
- [ ] Sentence splitting runs only against `tts_text`.
- [ ] TTS playback plans are generated only from `tts_text`.
- [ ] Pretty Text/Text-only toggles do not alter sentence indices, playback position, bookmarks, or search ownership.
- [ ] Add explicit tracing proving each playback step originated from `tts_text`.

## Phase 5: HTML-to-Text Sync Mapping
- [ ] Build a persistent mapping between `tts_text` sentence indices and Pretty Text HTML anchors.
- [ ] Prefer text-evidence-based mapping over proportional heuristics.
- [ ] Keep mapping monotonic so playback cannot jump backward or several sections ahead unexpectedly.
- [ ] Allow paragraph-level or sentence-level HTML anchors, but require deterministic fallback behavior.
- [ ] Persist sync artifacts in cache per source and page/chunk.
- [ ] Add tracing for mapping hits, low-confidence matches, fallback drifts, and missing anchors.

## Phase 6: Playback Highlight and Scroll Behavior
- [ ] Highlight the currently spoken unit in Pretty Text mode based on the HTML sync map.
- [ ] Keep scroll ownership stable within the same mapped paragraph/anchor.
- [ ] Only auto-scroll when playback advances to a new mapped HTML anchor, page, or explicit jump target.
- [ ] Keep Text-only and Pretty Text highlight positions aligned to the same `tts_text` cursor.
- [ ] Add tracing for highlight target resolution and scroll trigger reasons.

## Phase 7: Cache, Recovery, and Migration
- [ ] Extend cache layout to store `pretty_html`, `tts_text`, and HTML sync mapping artifacts.
- [ ] Add cache versioning for HTML dual-payload entries.
- [ ] Recover cleanly from missing or corrupted HTML/text artifacts by rebuilding them non-destructively.
- [ ] Ensure delete/reopen flows remove and rebuild all HTML-related artifacts consistently.
- [ ] Add tracing around cache reads, writes, invalidation, and rebuilds.

## Phase 8: Validation and Regression Coverage
- [ ] Unit tests for HTML sanitization and safe rendering behavior.
- [ ] Unit tests for plain-text extraction from representative HTML documents.
- [ ] Unit tests for sentence-to-HTML anchor mapping and fallback behavior.
- [ ] Integration tests for playback continuity across Pretty Text and Text-only mode switches.
- [ ] Regression tests for documents with tables of contents, internal anchors, repeated headings, images, and footnotes.
- [ ] Manual QA checklist covering faithful rendering, text-only cleanliness, and playback sync stability.

## Acceptance Criteria
- [ ] Pretty Text mode renders HTML documents faithfully enough that source structure and visuals are preserved.
- [ ] Text-only mode shows only clean extracted text.
- [ ] TTS, normalization, and playback indexing are fully owned by `tts_text`.
- [ ] Pretty Text highlight/scroll stays aligned with the Text-only/TTS cursor during playback.
- [ ] Full project build verification passes after implementation, excluding `deb`, `rpm`, and AppImage packaging targets.
