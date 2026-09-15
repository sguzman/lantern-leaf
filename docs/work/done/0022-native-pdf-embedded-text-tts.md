# 0022 — Native PDF embedded-text / TTS trustworthy path — A9

## Status

**READY — A9 CORRECTION AFTER A8 DIRECTOR REJECTION**

Read first:

- `docs/work/reviews/0022-a8-director-rejection.md`
- `docs/work/reviews/0022-a7-real-desktop-rejection.md`
- `docs/work/reviews/0022-a7-director-acceptance.md`
- `docs/architecture/pdf-text-recovery-boundary-2026-09.md`
- `docs/work/reviews/0020-a3-real-desktop-acceptance.md`

A8 is not authorized for human QA. Preserve its accepted direction while correcting Search input ownership and adding true runtime-level proof for PDF page-boundary TTS continuation.

## Preserve

Preserve the accepted native baseline and accepted A8 corrections:

- Rust + eframe/egui + one process-wide Pdfium owner;
- visual-first PDF open independent of text/TTS/search/recovery;
- continuous virtualized PDF scrolling and Goal 0020 responsiveness;
- cooperative bounded native-text extraction with raster priority;
- trusted page-aligned embedded text through shared immutable `PreparedPdfEmbeddedText`;
- prefix-indexed bounded PDF projections;
- one bounded off-egui PDF retirement worker;
- off-egui document-scale preparation/cache/search;
- source/generation/query-revision stale safety;
- trusted policy enabling Text-only/search/ordinary Windows TTS;
- A8 `SetTextOnly { enabled }` desired-state semantics;
- A8 persistent Search-panel visibility separate from one-shot focus acquisition;
- A8 explicit next-non-empty-page PDF TTS transition and page-aware manual seek logic;
- exact PDF visual sentence geometry/highlight/follow remains disabled;
- Quack-check, Python, Docling, OCR, hostile recovery remain forbidden.

## A9 correction 1 — actual Search editor focus must own shortcut suppression

A8 correctly introduced `search_panel_open`, but `ShellState::focus_owner` still depends on `pending_search_focus`, which is only the one-shot request used to acquire focus.

Once `render_search_panel()` consumes `pending_search_focus`, the real egui `TextEdit` can remain focused while the shell reports `FocusOwner::Reader`. `handle_shortcuts()` can then execute reader shortcuts for keys the user is trying to type into Search.

Required behavior:

- the one-shot focus request remains only an acquisition signal;
- shortcut suppression must follow actual text-input/editor focus, not `pending_search_focus`;
- while the Search `TextEdit` owns keyboard focus, ordinary reader/global shortcuts must not execute;
- when the editor genuinely loses focus, ordinary shortcuts resume;
- persistent panel visibility and actual input focus remain distinct concepts;
- modal focus continues to outrank everything else;
- do not globally suppress all shortcuts merely because Search is open if the editor is not focused.

Use an egui-authoritative focus signal (`Response::has_focus`, `Context::wants_keyboard_input`, or an equally explicit app-owned projection of actual editor focus) and keep shell focus ownership truthful.

## A9 correction 2 — persistent Search draft buffer, not asynchronous state echo

A8 constructs the query editor each frame from `state.reader_ui.search_query.clone()`, then asynchronously dispatches `SearchSetQuery` when the temporary string changes.

That is not a safe controlled-input model because `ReaderUpdated` acknowledgement can arrive after the next frame. The UI can reconstruct from stale authoritative state and overwrite newer typed characters.

Required behavior:

- add an app-owned persistent Search draft string or equivalent immediate UI state;
- keystrokes update the visible draft synchronously in the frame they occur;
- dispatch `SearchSetQuery` from that draft through the existing asynchronous reader path;
- delayed/stale ReaderUpdated/search reconciliation must not overwrite a newer local draft;
- external/source changes may intentionally resynchronize the draft using explicit source/query revision tracking;
- opening Search initializes the draft from the current authoritative query when appropriate;
- clearing the query remains stable and leaves Search open;
- closing/reopening Search preserves or deliberately restores the current authoritative query without flicker;
- do not move document-wide search work onto egui.

Add deterministic delayed-ack tests: type multiple characters across successive UI/state steps while authoritative query acknowledgement intentionally lags, then prove visible draft remains the full newest string and only matching/newer acknowledgement can reconcile it.

## A9 correction 3 — runtime-level PDF TTS continuation proof

The A7 failure occurred in `run_tts_runtime_loop()`. A8's session-level helper test does not execute that production branch.

Add a true simulated-runtime regression using `TtsRuntimeMode::Simulated` and `SimulatedBoundaryDriver` (or equivalent existing runtime harness).

The test must:

- create a trusted enriched PDF session;
- start on a one-sentence title/front-matter native page;
- include one or more empty native pages;
- include at least two later non-empty native pages;
- start TTS through the normal TtsRuntime command path;
- emit/consume actual simulated first-sample boundaries;
- prove the runtime itself advances to the next non-empty native page when the current page is exhausted;
- prove multiple page crossings occur without helper calls from the test;
- prove canonical global IDs are monotonic and no prior sentence is replayed;
- prove the old `Prepared TTS batch was empty` same-page failure does not occur;
- prove final document exhaustion terminalizes exactly once.

Manual Seek Next/Previous across native page boundaries must remain covered through the same authoritative session semantics.

## A9 correction 4 — fix global/local normalization-plan boundary mismatch

`ReaderSession::apply_tts_sentence_boundary()` receives a document-global `canonical_display_id` and resolves it to `(page, local_idx)`. `current_plan_display_end` is page-local.

Do not compare the global canonical ID directly to the page-local plan end.

Required behavior:

- retain `canonical_display_id` as the authoritative cross-document identity;
- use `local_idx` for comparisons against `current_plan_display_start/current_plan_display_end`;
- invalidate/rebuild a bounded plan only when the page-local spoken sentence actually reaches/leaves that local plan window;
- later PDF pages with large canonical bases must not trigger a plan rebuild merely because their global IDs are numerically large.

Add a later-page fixture whose canonical page base is far larger than `TTS_PLAN_WINDOW`, and instrument/verify that ordinary consecutive boundaries inside one local plan do not spuriously invalidate/rebuild it.

## Preserve A8 Text-only correction

Keep idempotent desired-state behavior:

- latest requested Text-only target is authoritative;
- duplicate same-target requests are no-ops;
- stale acknowledgements cannot invert the target;
- rapid alternating EPUB Pretty/Text-only requests converge to the final click;
- complete EPUB pretty content, canonical cursor, and TTS state survive stress toggling;
- PDF Text-only/visual toggling remains correct.

Do not reintroduce relative-toggle recovery loops or debounce-based correctness.

## Visual highlight scope

Do not implement native PDF sentence rectangles, visual spoken overlays, or native PDF visual auto-follow in A9. Those remain the next Gate-4 geometry/highlight/follow goal after Goal 0022 is physically accepted.

## Required tests

At minimum prove:

- Search opened with empty query remains visible;
- actual Search `TextEdit` acquires focus;
- after the one-shot focus request is consumed, actual editor focus still suppresses reader/global shortcuts;
- typing `f`, `s`, `r`, space, and ordinary multi-character text into Search does not execute reader controls;
- shortcuts resume after editor focus is lost;
- delayed asynchronous query acknowledgement cannot erase/reorder a newer local draft;
- clearing query leaves Search open and stable;
- later-page PDF FullText search navigation remains correct;
- simulated runtime-level one-sentence PDF title -> next non-empty page natural TTS continuation;
- multiple runtime-driven native page crossings;
- empty-page skipping;
- no prior-sentence replay;
- no empty same-page continuation batch;
- final-document exhaustion exactly once;
- later-page global canonical IDs do not spuriously invalidate local bounded plans;
- manual seek next/prev across boundaries;
- A8 idempotent Text-only EPUB stress regression;
- PDF Text-only toggle regression;
- Goal 0019/0020 continuous PDF regressions;
- ordinary Windows TTS and representative EPUB parity.

## Validation / handoff

Run focused tests, serialized workspace tests where appropriate, `cargo test --workspace`, `cargo check --workspace`, `cargo build --workspace`, `git diff --check`, repo-native Windows QA preparation, and fresh hosted `native-workspace` + `hosted-renderer-probe` validation for the substantive A9 lineage.

Do not terminalize or signal Goal achieved until hosted Windows validation is green.

Update `docs/work/reports/0022.md` with A9 evidence, move ready -> active -> done normally, push before signaling, restore shared checkout to `main`, and do not request human QA. Director reviews first.
