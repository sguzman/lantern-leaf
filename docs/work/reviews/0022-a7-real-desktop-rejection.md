# Goal 0022 A7 — real-desktop rejection

## Decision

**REJECTED IN REAL-DESKTOP QA. Goal 0022 remains open.**

A7's native visual/text architecture survives physical testing well enough to preserve: continuous PDF scrolling remains extremely responsive under violent wheel/scrollbar movement, transient `Rendering page N` placeholders converge quickly, Text-only becomes available and appears native-page aligned, PDF visual/Text-only toggling works, close/reopen is snappy, and PDF Previous/Next controls work.

Three blocking physical defects remain before Goal 0022 can close.

## Blocker 1 — natural PDF TTS continuation does not reliably cross native page boundaries

Physical evidence:

- starting TTS near the front read the title/current page and then stopped;
- natural page-boundary continuation was inconsistent;
- later in the PDF one boundary did cross, so the underlying canonical text/page mapping is present, but runtime continuation is not reliable;
- manual Previous/Next mostly works, with occasional Next repeating instead of advancing.

Source review exposes a production-shaped boundary flaw in `lanternleaf-app/src/tts_runtime.rs`.

The runtime prepares only the current page/window. When the prepared batch reaches the end of `plan.sentences` but `ReaderSession::has_canonical_sentence_after_current()` reports more canonical text, it sets `resume_start_override = Some(next_start)` and loops. It does **not** authoritatively advance the session to the next non-empty native page before recollecting the plan. On a one-sentence title page, `next_start == plan.sentences.len()`; the next loop recollects the same page, clamps the override to the same page's end, prepares an empty batch, and pauses with `Prepared TTS batch was empty`.

A8 must make natural playback continuation explicitly global/native-page aware. Reaching the end of the current native page while later canonical sentences exist must advance to the next non-empty native page and continue from its first canonical/audio sentence without requiring a manual Next command and without replaying the previous boundary.

## Blocker 2 — Search UI is effectively a one-frame panel

Physical evidence: attempting to open/focus Search makes the panel immediately disappear, so real PDF search cannot be QA'd.

Source review confirms this is a UI-state bug rather than a text-index failure. `render_panels()` shows Search only while `pending_search_focus` is true, or while a non-empty query/result already exists. `render_center()` then clears `pending_search_focus = false` in the same frame after rendering a placeholder label (`Search field would be focused`). There is no persistent Search-panel-open state and no actual focus request on the input widget. Therefore an empty newly opened Search panel disappears on the next frame before the user can type.

A8 must separate persistent Search-panel visibility from a one-shot focus request. Ctrl/Focusing Search should open it persistently, request actual egui keyboard focus on the query field once, and keep it open until the user explicitly closes it (or the reader closes). Empty query state must not auto-close the panel.

## Blocker 3 — rapid Text-only toggling can corrupt the EPUB reader presentation

Physical evidence: a representative EPUB initially rendered and spoke normally, but repeatedly toggling Text-only/Pretty eventually left the EPUB with only a small fragment of the book rendering and the rest apparently missing/broken.

Source review shows a race-prone toggle protocol:

- the UI maintains `text_only_override` as desired local presentation state;
- the button always sends the non-idempotent `SessionCommand::ToggleTextOnly`;
- `maybe_reapply_text_only()` can send another `ToggleTextOnly` whenever the optimistic override says true while the latest authoritative snapshot still says false;
- rapid user toggles and asynchronous ReaderUpdated snapshots can therefore queue multiple relative toggles against stale state rather than converge on one requested target.

A8 must replace this relative-toggle race with idempotent desired-state semantics. Prefer a canonical `SetTextOnly { enabled }` reader command (or an equivalent generation/desired-state protocol) so repeated clicks coalesce/converge to the latest requested state and stale acknowledgements cannot invert it. Stress toggling must not truncate/disappear EPUB pretty content, poison pretty-builder/cache state, move to an unrelated canonical location, or leave optimistic UI state divergent from ReaderSession.

## Not a Goal-0022 blocker — PDF visual spoken highlight/follow

The user also observed no spoken highlight or visual scroll sync in the native PDF surface. That is **expected at this stage**: Goal 0022 deliberately leaves exact PDF sentence geometry, pretty-surface spoken rectangles, and native PDF visual auto-follow disabled. Those belong to the next Gate-4 geometry/highlight/follow goal after the trustworthy text/TTS baseline is physically accepted.

Text-only spoken-row identity should continue to use the existing canonical first-sample path; A8 should retain/cover that behavior, but it must not invent PDF visual rectangles.

## Preserved A7 evidence

Preserve all accepted A7 architecture unless a correction directly requires touching it:

- one process-wide Pdfium owner;
- visual-first native PDF open independent of text/search/TTS/recovery;
- cooperative bounded embedded-text extraction with Current/Nearby raster priority;
- shared immutable trusted PDF text document;
- prefix-indexed bounded projections;
- one bounded off-egui PDF retirement worker;
- off-egui cache/search/document-scale work;
- source/generation/query-revision stale safety;
- trustworthy Text-only policy promotion;
- exact visual sentence sync remains disabled;
- no Quack-check, Python, Docling, OCR, or hostile recovery in this path;
- Goal 0020 continuous PDF responsiveness.

## A8 acceptance requirements

Automated production-shaped coverage must prove at minimum:

1. Natural TTS starting on a one-sentence PDF title page continues onto the next non-empty native page.
2. Natural TTS crosses multiple native page boundaries monotonically with first-sample canonical IDs and no replay.
3. Empty native pages are skipped correctly; final-document exhaustion still pauses/terminalizes once.
4. Next/Previous sentence at page boundaries do not repeat or flicker the canonical cursor.
5. Search panel opened with empty query persists for multiple frames and the actual query editor receives requested focus.
6. Search close is explicit; typing/clearing a query does not accidentally collapse the panel.
7. Existing off-thread full-text PDF search then remains stale-safe and navigates later native pages correctly.
8. Rapid EPUB Text-only/Pretty stress toggling converges to the latest desired state, produces no duplicate relative-toggle race, preserves the complete pretty document, and leaves TTS/canonical identity intact.
9. PDF Text-only/Pretty toggling remains correct.
10. Goal 0019/0020 PDF visual regressions and representative EPUB visual/TTS regressions remain green.

No further human QA is authorized until A8 passes director source/CI review.
