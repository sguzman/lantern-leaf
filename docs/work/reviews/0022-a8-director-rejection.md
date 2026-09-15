# Goal 0022 A8 — director rejection before real-desktop QA

## Decision

**REJECTED BEFORE HUMAN QA. Goal 0022 remains open.**

A8 correctly addresses much of the A7 physical failure report: it introduces idempotent `SetTextOnly { enabled }`, adds persistent Search-panel visibility, and adds an explicit PDF page-advance path for TTS. Fresh hosted Windows workflow `34986927340` passed on substantive commit `7c1e04883a67f33d68e355b8280b4abac29706b9`.

However, source review found two remaining blocking correctness defects plus one required regression gap. Do not physically QA A8.

## Blocker 1 — Search input focus is still not authoritative and typed keys can execute reader shortcuts

A8 separates `search_panel_open` from the one-shot `pending_search_focus`, which fixes the immediate one-frame disappearance. But `LanternLeafApp::update_shell_state()` still passes only `pending_search_focus` into `ShellState::update_from_app_state()`. `ShellState` therefore reports `FocusOwner::PanelInput` only while the one-shot focus request is pending.

`render_search_panel()` consumes that request as soon as it calls `response.request_focus()`. On the following frame the Search `TextEdit` can still hold real egui keyboard focus while `ShellState::focus_owner` has already fallen back to `Reader`.

`handle_shortcuts()` suppresses reader/global shortcuts only for `FocusOwner::Modal | FocusOwner::PanelInput`. Therefore ordinary characters typed into Search can simultaneously trigger reader shortcuts after the one-shot flag is consumed. With current bindings this is especially dangerous for letters/keys such as `f`, `s`, `r`, and space.

A9 must make shortcut suppression reflect **actual text-input focus**, not the transient request used to acquire it. A persistent Search panel being open is not by itself sufficient; the authoritative rule should be tied to the actual focused editor / egui keyboard-input ownership so shortcuts resume normally when the editor loses focus.

## Blocker 2 — Search query text is controlled from asynchronous reader state and can lose/flicker keystrokes

`render_search_panel()` currently does this every frame:

1. clones `state.reader_ui.search_query` into a temporary local `String`;
2. passes that temporary string to `TextEdit`;
3. on `response.changed()`, dispatches `SessionCommand::SearchSetQuery` through the reader effect pipeline.

The reader effect is asynchronous. `AppCommand::Reader` publishes only an operation-start local event and sends `ApplyReaderCommand` to the effect dispatcher; authoritative `ReaderUpdated` arrives later.

That means the next egui frame may reconstruct the text editor from the **old** `state.reader_ui.search_query` before the effect result lands. Fast typing can therefore revert/flicker/drop characters and can dispatch queries from stale text.

A9 must give the Search editor an app-owned persistent draft buffer (or equivalent synchronous UI input state) and reconcile it deliberately with authoritative reader search state. The editor must not be rebuilt each frame from an asynchronously acknowledged query string.

Required semantics:

- keystrokes update the visible draft immediately;
- dispatch may remain asynchronous/off-thread;
- stale ReaderUpdated/search reconciliation cannot overwrite newer draft text;
- external/source changes can deliberately resynchronize the draft;
- clearing remains stable;
- closing/reopening Search preserves or intentionally restores the authoritative current query without flicker.

## Blocker 3 — the new PDF natural-continuation branch is not exercised by a runtime-level regression, and a canonical/local plan-boundary mismatch remains

The physical A7 failure occurred in `run_tts_runtime_loop()`, but A8's new test `enriched_pdf_tts_continues_from_title_across_empty_pages_and_seeks_without_replay` directly calls `ReaderSession::advance_tts_to_next_non_empty_page()`. It does not execute the actual runtime loop branch that decides between same-page bounded refill and native-page transition, does not consume simulated first-sample boundaries through the runtime, and does not prove the old empty-same-page failure cannot recur in production.

A9 must add a real `TtsRuntimeMode::Simulated` regression that runs the control/runtime loop against an enriched PDF fixture, emits first-sample boundaries through `SimulatedBoundaryDriver`, starts on a one-sentence title page, skips empty pages, crosses multiple native page boundaries, and reaches final exhaustion without replay or empty-batch pause.

Source review also found a coordinate mismatch in `ReaderSession::apply_tts_sentence_boundary()`: `canonical_display_id` is document-global, while `current_plan_display_end` is page-local. The invalidation check currently compares the global canonical ID directly against the local plan end. On later pages with a large canonical base this can invalidate/rebuild the bounded normalization plan on effectively every spoken boundary.

A9 must compare the page-local sentence index (`local_idx`) against the page-local plan range, while retaining the global canonical ID as the authoritative identity. Add a later-page fixture whose global canonical base is much larger than the local plan range so the test cannot pass accidentally.

## Accepted A8 direction to preserve

Preserve unless directly required by the corrections above:

- native Rust + eframe/egui baseline;
- one process-wide Pdfium owner;
- visual-first PDF open independent of text/search/TTS/recovery;
- Goal 0020 continuous PDF responsiveness;
- trusted page-aligned native embedded text;
- prefix-indexed bounded PDF projections;
- bounded off-egui PDF retirement;
- raster priority over background text extraction;
- `SetTextOnly { enabled }` desired-state semantics and the EPUB stress regression;
- persistent `search_panel_open` separate from a one-shot focus request;
- actual Search `TextEdit` widget;
- explicit PDF next-non-empty-page TTS transition;
- empty-page skipping and page-aware manual seek logic;
- off-thread PDF full-text search/reconciliation;
- exact PDF visual sentence geometry/highlight/follow remains out of scope;
- no Quack-check, Python, Docling, OCR, or hostile recovery.

## A9 acceptance requirements

Automated production-shaped coverage must prove at minimum:

1. Search remains visibly open with an empty query.
2. The real Search editor owns keyboard focus after the one-shot focus request is consumed.
3. While Search `TextEdit` is focused, reader/global shortcuts do not execute for typed query characters.
4. When the editor loses focus, ordinary reader shortcuts work again.
5. Rapid multi-character typing remains visible and ordered even when reader search acknowledgements are delayed/reordered.
6. Clearing the query remains stable and does not close Search.
7. Later-page PDF search still navigates correct native-page provenance.
8. A simulated runtime-level PDF TTS test naturally continues from a one-sentence title page to the next non-empty native page without manual helper calls.
9. The runtime test crosses multiple native page boundaries, skips empty pages, retains first-sample global IDs, and terminalizes once at true document exhaustion.
10. Later-page global canonical IDs do not spuriously invalidate a page-local normalization plan; plan-boundary comparison uses local identity.
11. Manual Seek Next/Previous at page boundaries remain monotonic/non-repeating.
12. A8's idempotent Text-only stress behavior remains green for EPUB and PDF.
13. Goal 0019/0020 visual regressions and representative EPUB visual/TTS regressions remain green.

No human QA is authorized until A9 passes director source/CI review.
