# 0022 — Native PDF embedded-text / TTS trustworthy path — A8

## Status

**READY — A8 CORRECTION AFTER A7 REAL-DESKTOP REJECTION**

Read first:

- `docs/work/reviews/0022-a7-real-desktop-rejection.md`
- `docs/work/reviews/0022-a7-director-acceptance.md`
- `docs/architecture/pdf-text-recovery-boundary-2026-09.md`
- `docs/work/reviews/0020-a3-real-desktop-acceptance.md`

A7 source architecture remains accepted, but physical Windows QA rejected Goal 0022 because natural TTS continuation can stop at native page boundaries, Search UI cannot stay open long enough to type, and rapid Text-only toggling can corrupt EPUB presentation state.

## Preserve

Preserve the accepted native baseline:

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
- exact PDF visual sentence geometry/highlight/follow remains disabled;
- Quack-check, Python, Docling, OCR, hostile recovery remain forbidden.

## A8 correction 1 — explicit natural cross-page TTS continuation

The runtime must distinguish `end of current prepared page/window` from `end of document`.

When the last prepared audio item on native page N starts/completes and canonical text exists later:

- advance ReaderSession to the next non-empty native page through an explicit canonical continuation transition;
- rebuild the bounded audio plan from that page's first unspoken canonical sentence;
- retain first-sample ownership and canonical global IDs;
- do not replay the final sentence from page N;
- do not create an empty same-page batch as the continuation mechanism;
- skip empty native pages deterministically;
- final document exhaustion must terminalize exactly once.

The correction must cover title/front-matter pages containing only one sentence, because that is the physical failure case.

Manual Next/Previous sentence at page boundaries must share the same canonical transition semantics and must never repeat solely because a bounded plan ended.

## A8 correction 2 — persistent, real Search panel state

`pending_search_focus` is a one-shot focus request, not panel visibility.

Introduce persistent Search-panel visibility distinct from pending focus.

Required behavior:

- Focus Search / shortcut opens Search persistently;
- the actual query editor receives egui focus once;
- an empty query does not close the panel on the next frame;
- clearing a query does not close the panel;
- Search remains open until explicit close or reader/source close;
- opening Search must not depend on existing matches;
- later-page PDF search continues to use the accepted off-thread canonical search path;
- Search Next/Previous must navigate correct native page provenance.

## A8 correction 3 — idempotent Text-only desired-state semantics

Remove the race where optimistic `text_only_override` and stale snapshots can emit multiple relative `ToggleTextOnly` commands.

Prefer an explicit canonical command such as:

`SessionCommand::SetTextOnly { enabled: bool }`

or an equivalent desired-state/generation protocol.

Required behavior:

- the latest requested target is authoritative;
- duplicate requests for the same target are no-ops;
- stale ReaderUpdated acknowledgements cannot cause another relative inversion;
- user may rapidly alternate Pretty/Text-only and the session converges to the last click;
- command coalescing/serialization remains bounded;
- EPUB pretty document/cache state is preserved intact through stress toggling;
- returning to Pretty renders the complete same document, not a fragment;
- current canonical sentence/location and TTS state remain coherent;
- PDF Text-only toggling remains correct.

Do not paper over this with longer debounce delays. Fix the state semantics.

## Visual highlight scope

Do not implement native PDF sentence rectangles or pretty-surface visual follow in A8. Physical absence of those is expected until the subsequent geometry/highlight/follow goal.

Text-only canonical spoken-row highlighting/follow should remain governed by existing first-sample identity and may be tested as a regression.

## Required tests

Add production-shaped deterministic coverage for:

- one-sentence page -> next native page natural TTS continuation;
- multiple consecutive native page crossings;
- empty-page skipping;
- no replay at bounded-plan/page boundary;
- final-document exhaustion once;
- manual seek next/prev across boundaries;
- Search opened with empty query remains visible across multiple frames/state updates;
- actual Search text edit focus request is consumed without closing the panel;
- clearing query leaves Search open;
- later-page PDF search navigation;
- rapid alternating EPUB Pretty/Text-only requests converge to latest target;
- no duplicate relative-toggle race;
- complete EPUB pretty content survives stress toggling;
- EPUB canonical cursor/TTS remains intact;
- PDF Text-only toggling remains intact;
- Goal 0019/0020 continuous PDF regressions;
- ordinary Windows TTS and representative EPUB parity.

## Validation / handoff

Run focused tests, serialized workspace tests where appropriate, `cargo check --workspace`, `cargo build --workspace`, `git diff --check`, repo-native Windows QA preparation, and fresh hosted `native-workspace` + `hosted-renderer-probe` validation for the substantive A8 lineage.

Do not terminalize or signal Goal achieved until hosted Windows validation is green.

Update `docs/work/reports/0022.md` with A8 evidence, move ready -> active -> done normally, push before signaling, restore shared checkout to `main`, and do not request human QA. Director reviews first.
