# Browser Tabs Import via Browsr Roadmap

## Objective
- [x] Add a new source category in the app for importing live browser tabs through the local `browsr` server.
- [x] Let the user select a browser window, then a tab, and import that tab as readable source content.
- [x] Ingest tab HTML like other HTML sources for Pretty Text rendering.
- [x] Ingest tab plain text as the canonical Text-only and TTS payload.
- [x] Preserve deterministic sync between imported tab HTML and the Text-only/TTS cursor.

## Phase 1: External Service Contract
- [x] Add a local integration contract for `browsr`:
- [x] `GET /health`
- [x] `GET /v1/windows`
- [x] `GET /v1/tabs`
- [x] `POST /v1/tabs/{tab_id}/snapshot`
- [x] Add app config for `browsr` base URL, request timeout, and feature enable/disable flag.
- [x] Validate startup behavior when `browsr` is unavailable or the extension is disconnected.
- [x] Add tracing for server reachability, extension connectivity, and request failures.

## Phase 2: Browser Tabs Source UX
- [x] Add a new source section in the starter/import UI called `Browser Tabs`.
- [x] Show connection status for `browsr` and extension health.
- [x] Load and display available browser windows from `browsr`.
- [x] Allow selecting a window when multiple windows are present.
- [x] Load and display tabs for the selected window.
- [x] Provide search/filter for tabs by title and URL.
- [x] Show tab metadata useful for selection:
- [x] title
- [x] URL
- [x] active/audible/pinned state when available
- [x] last updated timestamp if exposed
- [x] Add clear empty/error states for:
- [x] server offline
- [x] extension disconnected
- [x] no windows
- [x] no tabs
- [x] restricted/unavailable tab snapshot

## Phase 3: Snapshot and Import Pipeline
- [x] Add a browser-tab import action that requests `snapshot_tab` from `browsr`.
- [x] Request both HTML and text payloads when available.
- [x] Store snapshot metadata:
- [x] window id
- [x] tab id
- [x] tab title
- [x] URL
- [x] language
- [x] ready state
- [x] capture timestamp
- [x] Handle truncation metadata from `browsr` explicitly and surface degraded-mode messaging when HTML/text was capped.
- [x] Add tracing for snapshot latency, payload sizes, truncation, and import outcome.

## Phase 4: Reader Data Model and Ownership
- [x] Represent imported browser tabs as a first-class source type, not a fake file path.
- [x] Define dual-payload contract for imported tabs:
- [x] `pretty_html` from tab snapshot HTML
- [x] `tts_text` from tab snapshot plain text
- [x] Keep TTS, normalization, sentence splitting, bookmarks, and search ownership strictly bound to `tts_text`.
- [x] Preserve source metadata so reopened sessions can display original tab title and URL.

## Phase 5: Pretty Text Rendering for Imported Tabs
- [x] Reuse the native HTML rendering path for imported tab HTML.
- [x] Sanitize imported HTML while preserving readable structure, links, images, headings, tables, and figures where safe.
- [x] Rewrite asset URLs and relative links as needed for stable rendering.
- [x] Preserve internal anchor navigation when possible.
- [x] Add protections against app-shell style bleed from imported page CSS.
- [x] Add tracing for sanitizer outcomes and rewritten resource/link behavior.

## Phase 6: Text-only and TTS Pipeline
- [x] Text-only mode renders only imported tab `tts_text`.
- [x] Sentence planning and TTS consume only `tts_text`.
- [x] Pretty Text/Text-only switching does not alter sentence indices or playback position.
- [x] Add explicit tracing proving playback uses `tts_text` rather than rendered HTML.

## Phase 7: HTML-to-Text Sync Mapping
- [x] Build a sync map from imported tab `tts_text` sentences back to rendered HTML anchors.
- [x] Prefer text-evidence-based mapping over coarse proportional mapping.
- [x] Keep mapping deterministic for long pages, repeated headings, tables of contents, and lazy-loaded sections.
- [x] Add conservative fallback when exact anchors are missing.
- [x] Persist per-import sync artifacts in cache.
- [x] Add tracing for mapping hits, low-confidence matches, drift, and missing anchors.

## Phase 8: Browser Tab Import Lifecycle
- [x] Define whether imported tabs are:
- [x] one-time snapshots
- [x] refreshable snapshots
- [x] live-refresh sources
- [x] Add a manual refresh action to re-snapshot the currently imported tab.
- [x] Define conflict behavior if the browser tab content changed since the last import.
- [x] Preserve existing bookmarks and reading position when refreshing if sync confidence is sufficient.
- [x] Add tracing for refresh/reimport behavior and position-preservation decisions.

## Phase 9: Cache, Persistence, and Recents
- [x] Extend cache layout for browser-tab sources:
- [x] snapshot HTML
- [x] snapshot text
- [x] sync map
- [x] metadata manifest
- [x] optional favicon/thumbnail if available later
- [x] Add recents support for imported browser tabs with display title and source URL.
- [x] Ensure recent-delete removes all browser-tab artifacts cleanly.
- [x] Define reopen semantics when the original tab no longer exists:
- [x] open cached snapshot only
- [x] optionally offer reimport if a matching live tab exists
- [x] Add tracing around cache reads, writes, invalidation, and delete flows.

## Phase 10: Security and Privacy Boundaries
- [x] Keep integration limited to the user-configured local `browsr` endpoint.
- [x] Never auto-import tabs without explicit user action.
- [x] Surface clearly when a tab cannot be captured because it is browser-restricted or extension-restricted.
- [x] Review logging so imported page content is not dumped into traces unintentionally.
- [x] Add safeguards around maximum payload size and oversized HTML/text snapshots.

## Phase 11: Validation and Regression Coverage
- [x] Unit tests for `browsr` client request/response handling and error mapping.
- [x] Unit tests for tab snapshot ingestion into `pretty_html` and `tts_text`.
- [x] Unit tests for imported-tab sync mapping and fallback behavior.
- [x] Integration tests for starter UI window/tab selection flow.
- [x] Integration tests for imported-tab playback continuity across Pretty Text and Text-only toggles.
- [x] Regression tests for:
- [x] long article pages
- [x] tab snapshots with truncated HTML/text
- [x] pages with heavy CSS
- [x] pages with relative links and images
- [x] extension disconnected/server offline conditions
- [x] Manual QA checklist covering multi-window selection, tab import, playback sync, refresh, recents, and delete/reopen.

## Acceptance Criteria
- [x] The app exposes `Browser Tabs` as a first-class import source.
- [x] The user can choose a window, choose a tab, and import it successfully through `browsr`.
- [x] Imported tab HTML renders in Pretty Text mode through the native HTML path.
- [x] Imported tab plain text is the sole Text-only and TTS source.
- [x] Highlight and scroll sync remain aligned between rendered tab HTML and the Text-only/TTS cursor.
- [x] Full project build verification passes after implementation, excluding `deb`, `rpm`, and AppImage packaging targets.
