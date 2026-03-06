# Browser Tabs Import via Browsr Roadmap

## Objective
- [ ] Add a new source category in the app for importing live browser tabs through the local `browsr` server.
- [ ] Let the user select a browser window, then a tab, and import that tab as readable source content.
- [ ] Ingest tab HTML like other HTML sources for Pretty Text rendering.
- [ ] Ingest tab plain text as the canonical Text-only and TTS payload.
- [ ] Preserve deterministic sync between imported tab HTML and the Text-only/TTS cursor.

## Phase 1: External Service Contract
- [ ] Add a local integration contract for `browsr`:
- [ ] `GET /health`
- [ ] `GET /v1/windows`
- [ ] `GET /v1/tabs`
- [ ] `POST /v1/tabs/{tab_id}/snapshot`
- [ ] Add app config for `browsr` base URL, request timeout, and feature enable/disable flag.
- [ ] Validate startup behavior when `browsr` is unavailable or the extension is disconnected.
- [ ] Add tracing for server reachability, extension connectivity, and request failures.

## Phase 2: Browser Tabs Source UX
- [ ] Add a new source section in the starter/import UI called `Browser Tabs`.
- [ ] Show connection status for `browsr` and extension health.
- [ ] Load and display available browser windows from `browsr`.
- [ ] Allow selecting a window when multiple windows are present.
- [ ] Load and display tabs for the selected window.
- [ ] Provide search/filter for tabs by title and URL.
- [ ] Show tab metadata useful for selection:
- [ ] title
- [ ] URL
- [ ] active/audible/pinned state when available
- [ ] last updated timestamp if exposed
- [ ] Add clear empty/error states for:
- [ ] server offline
- [ ] extension disconnected
- [ ] no windows
- [ ] no tabs
- [ ] restricted/unavailable tab snapshot

## Phase 3: Snapshot and Import Pipeline
- [ ] Add a browser-tab import action that requests `snapshot_tab` from `browsr`.
- [ ] Request both HTML and text payloads when available.
- [ ] Store snapshot metadata:
- [ ] window id
- [ ] tab id
- [ ] tab title
- [ ] URL
- [ ] language
- [ ] ready state
- [ ] capture timestamp
- [ ] Handle truncation metadata from `browsr` explicitly and surface degraded-mode messaging when HTML/text was capped.
- [ ] Add tracing for snapshot latency, payload sizes, truncation, and import outcome.

## Phase 4: Reader Data Model and Ownership
- [ ] Represent imported browser tabs as a first-class source type, not a fake file path.
- [ ] Define dual-payload contract for imported tabs:
- [ ] `pretty_html` from tab snapshot HTML
- [ ] `tts_text` from tab snapshot plain text
- [ ] Keep TTS, normalization, sentence splitting, bookmarks, and search ownership strictly bound to `tts_text`.
- [ ] Preserve source metadata so reopened sessions can display original tab title and URL.

## Phase 5: Pretty Text Rendering for Imported Tabs
- [ ] Reuse the native HTML rendering path for imported tab HTML.
- [ ] Sanitize imported HTML while preserving readable structure, links, images, headings, tables, and figures where safe.
- [ ] Rewrite asset URLs and relative links as needed for stable rendering.
- [ ] Preserve internal anchor navigation when possible.
- [ ] Add protections against app-shell style bleed from imported page CSS.
- [ ] Add tracing for sanitizer outcomes and rewritten resource/link behavior.

## Phase 6: Text-only and TTS Pipeline
- [ ] Text-only mode renders only imported tab `tts_text`.
- [ ] Sentence planning and TTS consume only `tts_text`.
- [ ] Pretty Text/Text-only switching does not alter sentence indices or playback position.
- [ ] Add explicit tracing proving playback uses `tts_text` rather than rendered HTML.

## Phase 7: HTML-to-Text Sync Mapping
- [ ] Build a sync map from imported tab `tts_text` sentences back to rendered HTML anchors.
- [ ] Prefer text-evidence-based mapping over coarse proportional mapping.
- [ ] Keep mapping deterministic for long pages, repeated headings, tables of contents, and lazy-loaded sections.
- [ ] Add conservative fallback when exact anchors are missing.
- [ ] Persist per-import sync artifacts in cache.
- [ ] Add tracing for mapping hits, low-confidence matches, drift, and missing anchors.

## Phase 8: Browser Tab Import Lifecycle
- [ ] Define whether imported tabs are:
- [ ] one-time snapshots
- [ ] refreshable snapshots
- [ ] live-refresh sources
- [ ] Add a manual refresh action to re-snapshot the currently imported tab.
- [ ] Define conflict behavior if the browser tab content changed since the last import.
- [ ] Preserve existing bookmarks and reading position when refreshing if sync confidence is sufficient.
- [ ] Add tracing for refresh/reimport behavior and position-preservation decisions.

## Phase 9: Cache, Persistence, and Recents
- [ ] Extend cache layout for browser-tab sources:
- [ ] snapshot HTML
- [ ] snapshot text
- [ ] sync map
- [ ] metadata manifest
- [ ] optional favicon/thumbnail if available later
- [ ] Add recents support for imported browser tabs with display title and source URL.
- [ ] Ensure recent-delete removes all browser-tab artifacts cleanly.
- [ ] Define reopen semantics when the original tab no longer exists:
- [ ] open cached snapshot only
- [ ] optionally offer reimport if a matching live tab exists
- [ ] Add tracing around cache reads, writes, invalidation, and delete flows.

## Phase 10: Security and Privacy Boundaries
- [ ] Keep integration limited to the user-configured local `browsr` endpoint.
- [ ] Never auto-import tabs without explicit user action.
- [ ] Surface clearly when a tab cannot be captured because it is browser-restricted or extension-restricted.
- [ ] Review logging so imported page content is not dumped into traces unintentionally.
- [ ] Add safeguards around maximum payload size and oversized HTML/text snapshots.

## Phase 11: Validation and Regression Coverage
- [ ] Unit tests for `browsr` client request/response handling and error mapping.
- [ ] Unit tests for tab snapshot ingestion into `pretty_html` and `tts_text`.
- [ ] Unit tests for imported-tab sync mapping and fallback behavior.
- [ ] Integration tests for starter UI window/tab selection flow.
- [ ] Integration tests for imported-tab playback continuity across Pretty Text and Text-only toggles.
- [ ] Regression tests for:
- [ ] long article pages
- [ ] tab snapshots with truncated HTML/text
- [ ] pages with heavy CSS
- [ ] pages with relative links and images
- [ ] extension disconnected/server offline conditions
- [ ] Manual QA checklist covering multi-window selection, tab import, playback sync, refresh, recents, and delete/reopen.

## Acceptance Criteria
- [ ] The app exposes `Browser Tabs` as a first-class import source.
- [ ] The user can choose a window, choose a tab, and import it successfully through `browsr`.
- [ ] Imported tab HTML renders in Pretty Text mode through the native HTML path.
- [ ] Imported tab plain text is the sole Text-only and TTS source.
- [ ] Highlight and scroll sync remain aligned between rendered tab HTML and the Text-only/TTS cursor.
- [ ] Full project build verification passes after implementation, excluding `deb`, `rpm`, and AppImage packaging targets.
