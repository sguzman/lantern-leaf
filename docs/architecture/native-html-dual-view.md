# Native HTML + TTS Plain-Text Architecture

## Contract
- Canonical TTS input is always `tts_text` (plain text).
- Pretty view payload is selected by `pretty_kind`:
- `html`: render `reading_html_page`.
- `markdown`: render `reading_markdown_page`.
- `none`: show sentence list fallback.
- `sentence_anchor_map` is generated from canonical page sentences and mapped to pretty anchors.

## Native HTML Pagination Modes
- `sentence_window`:
- Active mode. Page transitions are driven by `tts_text` pagination and sentence continuity.
- Pretty HTML stays a single rendered document and playback highlights scroll to mapped anchors.
- `chapter_section`:
- Contract is defined and config-supported.
- Sentence continuity remains canonical in `tts_text`; section navigation is non-authoritative for TTS.
- Current implementation preserves sentence-window behavior for playback determinism.

## Cache Layout
- Cache root: `.cache/lantern-leaf/<source_hash>/`
- Dual-view artifacts:
- `content/layout-version.txt`
- `content/tts-text.txt`
- `content/reading-markdown.md` (if present)
- `content/reading-html.html` (if present)
- `content/sentence-anchor-map/page-*.toml`
- Extracted images:
- `images/img-*.{png|jpg|...}`

## Recovery Semantics
- Cache metadata/text mismatches are treated as cache misses.
- Corrupt cache metadata is treated as recoverable and triggers non-destructive rebuild.
- Rebuild path logs explicit miss/corruption cause through `tracing`.

## Debugging Workflow
- Inspect source conversion + cache decisions:
  - `rg "pandoc cache|PDF transcript cache|rebuilding artifacts|Source load complete" logs -n`
- Inspect mapping telemetry summary:
  - `rg "Sentence mapping telemetry summary|Display->audio mapping fallback|Audio->display mapping fallback" logs -n`
- Inspect pretty payload selection per snapshot:
  - `rg "Prepared reader snapshot payload" logs -n`
