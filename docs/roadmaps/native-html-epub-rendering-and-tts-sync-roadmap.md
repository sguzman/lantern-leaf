# Native HTML EPUB Rendering and TTS Sync Roadmap

## Objective
- [x] Render EPUB and HTML sources as native HTML in Pretty Text mode (no markdown intermediary for those source types).
- [x] Keep TTS, normalization, sentence splitting, and playback control strictly bound to canonical plain text.
- [x] Maintain deterministic mapping between HTML render anchors and plain-text sentence indices.

## Phase 1: Data Model and Contracts
- [x] Introduce explicit pretty payload model fields:
- [x] `pretty_html: Option<String>` for EPUB/HTML sources.
- [x] `tts_text: String` as canonical audio pipeline input.
- [x] `pretty_kind: enum { html, markdown, none }` for renderer selection.
- [x] Keep backward compatibility for existing `reading_markdown` consumers during migration window.
- [x] Add tracing fields to snapshot/log events showing chosen pretty payload kind and source type.

## Phase 2: Ingestion Pipeline (EPUB/HTML)
- [x] EPUB ingest outputs two artifacts:
- [x] `pretty_html` from EPUB-native HTML extraction/render preparation.
- [x] `tts_text` from text extraction pipeline used by TTS.
- [x] HTML ingest outputs two artifacts:
- [x] sanitized/normalized `pretty_html` for display.
- [x] `tts_text` for TTS pipeline.
- [x] Ensure image/media extraction stores assets in stable cache paths scoped by source hash.
- [x] Rewrite HTML asset references to stable local URLs (or resolvable relative paths) for the webview.
- [x] Add tracing spans for extraction/asset rewrite/duration/fallback outcomes.

## Phase 3: Native HTML Rendering Path
- [x] Add Pretty Text renderer branch for `pretty_kind = html`.
- [x] Preserve link behavior:
- [x] intra-document anchors (`#id`) scroll within the current rendered document.
- [x] external links open safely in browser context.
- [x] Render inline images from extracted asset store with lazy loading and stable sizing.
- [x] Add safe HTML sanitization policy with explicit allowlist for tags/attributes used by EPUB/HTML content.
- [x] Remove markdown fallback for EPUB/HTML once HTML path is stable.

## Phase 4: Plain-Text/TTS Canonical Ownership
- [x] Confirm all normalization and sentence planning consume `tts_text` only.
- [x] Guarantee Pretty Text mode switches do not alter sentence indexing, playback index, or bookmark semantics.
- [x] Add explicit tracing markers proving each TTS plan originates from `tts_text` and never `pretty_html`.

## Phase 5: HTML-to-TTS Sync Mapping
- [x] Build a source map from plain-text sentence spans to HTML anchor offsets.
- [x] Persist per-page/per-sentence anchor map in cache alongside dual artifacts.
- [x] On sentence highlight/playback transitions:
- [x] scroll to mapped HTML anchor when available.
- [x] apply nearest-anchor fallback when exact anchor is missing.
- [x] Emit mapping telemetry (`hit`, `nearby_fallback`, `missing`) for debugging and regressions.

## Phase 6: Pagination and Navigation Strategy
- [x] Define pagination contract for native HTML sources:
- [x] Option A: sentence-window-driven virtual pagination over one HTML document.
- [x] Option B: chapter/section pagination with sentence-index continuity.
- [x] Keep page transitions synchronized with plain-text sentence boundaries.
- [x] Preserve keyboard shortcuts and TTS controls independent of visual pagination mode.

## Phase 7: Cache, Migration, and Recovery
- [x] Extend cache schema for `pretty_html`, asset manifest, and sentence-anchor map.
- [x] Add cache version bump and migration from markdown-centric artifacts.
- [x] On cache miss/corruption, rebuild artifacts non-destructively with clear tracing.
- [x] Ensure recent-delete removes HTML assets, plain text, mapping artifacts, and thumbnails idempotently.

## Phase 8: Calibre/Recents Consistency for Covers and Assets
- [x] Unify thumbnail extraction logic so recents and calibre list use the same EPUB-cover fallback behavior.
- [x] Ensure list views can hydrate missing thumbnails on demand without full catalogue reload.
- [x] Add tracing for thumbnail source (`cache`, `sidecar`, `epub_cover`, `server`, `materialized_epub`).

## Phase 9: Testing and Validation
- [x] Unit tests for HTML sanitizer and asset URL rewriting.
- [x] Unit tests for sentence-to-anchor mapping generation and fallback behavior.
- [x] Integration tests validating TTS continuity across Pretty/Text-only toggles.
- [x] Integration tests for EPUB with heavy images, internal links, footnotes, and tables.
- [x] Regression tests ensuring no raw markdown/link artifacts appear for EPUB/HTML pretty view.
- [x] Manual QA checklist for:
- [x] cover/thumbnail consistency across calibre and recents.
- [x] image rendering in pretty view.
- [x] internal link navigation.
- [x] sentence highlight and auto-scroll sync under playback.

## Phase 10: Rollout and Cleanup
- [x] Gate native HTML path behind a config flag for staged rollout.
- [x] Add observability dashboards/log summaries for mapping fallback rates and render errors.
- [x] Remove legacy EPUB/HTML markdown conversion path after stability window.
- [x] Update docs for architecture, cache layout, and debugging workflow.

## Acceptance Criteria
- [x] EPUB/HTML Pretty Text renders as native HTML with working images and links.
- [x] TTS behavior remains deterministic and fully sourced from plain text.
- [x] Highlight/scroll sync remains stable across sentence transitions and page changes.
- [x] Calibre and recents show consistent cover thumbnails for EPUB sources.
- [x] Full project build verification passes (excluding rpm/deb/AppImage packaging targets).
