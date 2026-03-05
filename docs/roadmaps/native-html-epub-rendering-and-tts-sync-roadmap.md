# Native HTML EPUB Rendering and TTS Sync Roadmap

## Objective
- [ ] Render EPUB and HTML sources as native HTML in Pretty Text mode (no markdown intermediary for those source types).
- [ ] Keep TTS, normalization, sentence splitting, and playback control strictly bound to canonical plain text.
- [ ] Maintain deterministic mapping between HTML render anchors and plain-text sentence indices.

## Phase 1: Data Model and Contracts
- [ ] Introduce explicit pretty payload model fields:
- [ ] `pretty_html: Option<String>` for EPUB/HTML sources.
- [ ] `tts_text: String` as canonical audio pipeline input.
- [ ] `pretty_kind: enum { html, markdown, none }` for renderer selection.
- [ ] Keep backward compatibility for existing `reading_markdown` consumers during migration window.
- [ ] Add tracing fields to snapshot/log events showing chosen pretty payload kind and source type.

## Phase 2: Ingestion Pipeline (EPUB/HTML)
- [ ] EPUB ingest outputs two artifacts:
- [ ] `pretty_html` from EPUB-native HTML extraction/render preparation.
- [ ] `tts_text` from text extraction pipeline used by TTS.
- [ ] HTML ingest outputs two artifacts:
- [ ] sanitized/normalized `pretty_html` for display.
- [ ] `tts_text` for TTS pipeline.
- [ ] Ensure image/media extraction stores assets in stable cache paths scoped by source hash.
- [ ] Rewrite HTML asset references to stable local URLs (or resolvable relative paths) for the webview.
- [ ] Add tracing spans for extraction/asset rewrite/duration/fallback outcomes.

## Phase 3: Native HTML Rendering Path
- [ ] Add Pretty Text renderer branch for `pretty_kind = html`.
- [ ] Preserve link behavior:
- [ ] intra-document anchors (`#id`) scroll within the current rendered document.
- [ ] external links open safely in browser context.
- [ ] Render inline images from extracted asset store with lazy loading and stable sizing.
- [ ] Add safe HTML sanitization policy with explicit allowlist for tags/attributes used by EPUB/HTML content.
- [ ] Remove markdown fallback for EPUB/HTML once HTML path is stable.

## Phase 4: Plain-Text/TTS Canonical Ownership
- [ ] Confirm all normalization and sentence planning consume `tts_text` only.
- [ ] Guarantee Pretty Text mode switches do not alter sentence indexing, playback index, or bookmark semantics.
- [ ] Add explicit tracing markers proving each TTS plan originates from `tts_text` and never `pretty_html`.

## Phase 5: HTML-to-TTS Sync Mapping
- [ ] Build a source map from plain-text sentence spans to HTML anchor offsets.
- [ ] Persist per-page/per-sentence anchor map in cache alongside dual artifacts.
- [ ] On sentence highlight/playback transitions:
- [ ] scroll to mapped HTML anchor when available.
- [ ] apply nearest-anchor fallback when exact anchor is missing.
- [ ] Emit mapping telemetry (`hit`, `nearby_fallback`, `missing`) for debugging and regressions.

## Phase 6: Pagination and Navigation Strategy
- [ ] Define pagination contract for native HTML sources:
- [ ] Option A: sentence-window-driven virtual pagination over one HTML document.
- [ ] Option B: chapter/section pagination with sentence-index continuity.
- [ ] Keep page transitions synchronized with plain-text sentence boundaries.
- [ ] Preserve keyboard shortcuts and TTS controls independent of visual pagination mode.

## Phase 7: Cache, Migration, and Recovery
- [ ] Extend cache schema for `pretty_html`, asset manifest, and sentence-anchor map.
- [ ] Add cache version bump and migration from markdown-centric artifacts.
- [ ] On cache miss/corruption, rebuild artifacts non-destructively with clear tracing.
- [ ] Ensure recent-delete removes HTML assets, plain text, mapping artifacts, and thumbnails idempotently.

## Phase 8: Calibre/Recents Consistency for Covers and Assets
- [ ] Unify thumbnail extraction logic so recents and calibre list use the same EPUB-cover fallback behavior.
- [ ] Ensure list views can hydrate missing thumbnails on demand without full catalogue reload.
- [ ] Add tracing for thumbnail source (`cache`, `sidecar`, `epub_cover`, `server`, `materialized_epub`).

## Phase 9: Testing and Validation
- [ ] Unit tests for HTML sanitizer and asset URL rewriting.
- [ ] Unit tests for sentence-to-anchor mapping generation and fallback behavior.
- [ ] Integration tests validating TTS continuity across Pretty/Text-only toggles.
- [ ] Integration tests for EPUB with heavy images, internal links, footnotes, and tables.
- [ ] Regression tests ensuring no raw markdown/link artifacts appear for EPUB/HTML pretty view.
- [ ] Manual QA checklist for:
- [ ] cover/thumbnail consistency across calibre and recents.
- [ ] image rendering in pretty view.
- [ ] internal link navigation.
- [ ] sentence highlight and auto-scroll sync under playback.

## Phase 10: Rollout and Cleanup
- [ ] Gate native HTML path behind a config flag for staged rollout.
- [ ] Add observability dashboards/log summaries for mapping fallback rates and render errors.
- [ ] Remove legacy EPUB/HTML markdown conversion path after stability window.
- [ ] Update docs for architecture, cache layout, and debugging workflow.

## Acceptance Criteria
- [ ] EPUB/HTML Pretty Text renders as native HTML with working images and links.
- [ ] TTS behavior remains deterministic and fully sourced from plain text.
- [ ] Highlight/scroll sync remains stable across sentence transitions and page changes.
- [ ] Calibre and recents show consistent cover thumbnails for EPUB sources.
- [ ] Full project build verification passes (excluding rpm/deb/AppImage packaging targets).
