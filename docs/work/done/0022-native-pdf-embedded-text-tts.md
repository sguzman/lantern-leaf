# 0022 — Native PDF embedded-text / TTS trustworthy path — A11

## Status

**READY — A11 CORRECTION AFTER A10 DIRECTOR SOURCE REJECTION**

This continuation preserves the accepted A10 implementation on the existing `codex/0022-native-pdf-embedded-text-tts` branch and report lineage. The attached A11 correction brief is authoritative where current `main` has not yet received the named A10 rejection document.

## A11 correction blockers

1. Reproduce and close burst seek correctness through the actual asynchronous `TtsRuntime` worker and simulated first-sample path. Commands must use the normal queue/worker path, cross PDF page boundaries and empty pages, exercise rapid accepted Next/Previous, inject a stale first-sample event from a superseded generation, prove stale rejection, monotonic one-sentence canonical movement, boundary clamping, coherent page/local/global identity, and representative EPUB parity. If no defect reproduces, document the existing rejection mechanism explicitly and test it.
2. Reproduce and diagnose damaged EPUB through a production-shaped temporary cache/persistence close, full session release, same-source reopen lifecycle. Stress Pretty/Text-only through the desired-state path, preserve valid bookmark/canonical position, reject stale prior-session Pretty worker completion, exercise valid dual-view/structured artifacts, reject or safely rebuild corrupt/incomplete derived artifacts, and prove complete Pretty content is immediate without TTS/Next healing. Do not globally purge healthy caches. Record the evidence and actual causal finding.

Read first:

- `docs/work/reviews/0022-a9-real-desktop-rejection.md`
- `docs/work/reviews/0022-a9-director-acceptance.md`
- `docs/work/reviews/0022-a7-real-desktop-rejection.md`
- `docs/architecture/pdf-text-recovery-boundary-2026-09.md`
- `docs/project/qa-evidence-ledger.md`

A9 source architecture remains accepted in direction, but physical Windows QA rejected Goal 0022 on interaction/state correctness.

## Preserve

Preserve the accepted native baseline:

- Rust + `eframe`/`egui` authority;
- one process-wide Pdfium owner;
- visual-first PDF open independent of text/search/TTS/recovery;
- Goal 0020 continuous virtualized PDF scrolling/responsiveness;
- cooperative bounded native text extraction with Current/Nearby raster priority;
- trusted page-aligned `PreparedPdfEmbeddedText`;
- prefix-indexed bounded enriched-PDF projections;
- one bounded off-egui PDF retirement worker;
- document-scale preparation/cache/search off egui;
- source/generation/query-revision stale safety;
- A8/A9 `SetTextOnly { enabled }` latest-target semantics;
- A9 Search synchronous draft and actual-editor-focus ownership;
- A9 runtime-level natural PDF page continuation;
- global canonical identity + page-local plan-boundary correctness;
- ordinary Windows TTS first-sample authority;
- exact PDF sentence geometry/highlight/follow remains disabled;
- Quack-check/Python/Docling/OCR/hostile recovery remain forbidden in Goal 0022.

Physical A9 evidence to preserve:

- violent PDF scrolling remains excellent;
- `Rendering page N` placeholders are fast/transient;
- PDF Text-only is page-aligned;
- natural TTS crossed at least one native page boundary;
- burst Next mostly works;
- PDF Text-only stress works;
- new EPUB Pretty/Text-only stress no longer reproduces the original nuke;
- PDF reopen remains snappy.

## A10 correction 1 — synchronous TTS settings editor state

The TTS Speed/Volume sliders currently rebuild their visible values from asynchronous `ReaderSnapshot` settings every frame and emit `ApplySettings` on every changed frame.

Required:

- app-owned synchronous draft state for Speed and Volume while editing;
- no visible snap-back to stale acknowledged values;
- stale `ReaderUpdated` cannot overwrite a newer local draft;
- same-value acknowledgement settles the draft cleanly;
- source/session change intentionally resynchronizes;
- commit is bounded/coalesced: prefer commit-on-drag-stop or an equivalent bounded desired-state protocol rather than one unbounded settings effect per pixel/frame;
- persisted canonical settings eventually equal the final visible target;
- changing Speed/Volume must not move the central reading viewport merely because status/diagnostic strings change.

Add delayed-ack and rapid-drag deterministic regressions.

## A10 correction 2 — diagnostic/status presentation must not reflow the reader

Transient command/status diagnostics are debugging evidence, not document geometry.

Required:

- changing `Last command`, status messages, persistence notices, TTS control notices, or repeated command traffic must not change the central reader viewport rect/height;
- use a fixed-height bounded status region, truncation/ellipsis, overlay/toast, or another stable layout;
- cap visible message count/width;
- diagnostics may remain inspectable through a diagnostics panel/log;
- spamming Next/Previous or dragging a settings slider must not vertically/horizontally shove reader content.

Add an egui/layout regression or deterministic layout-policy projection proving central reader geometry is invariant to short/long diagnostic strings and command churn.

## A10 correction 3 — make Search usable, not merely computable

A9 successfully keeps Search open and preserves typed text, but the panel exposes no navigation.

Required Search UX:

- visible `Previous match` and `Next match` controls;
- Enter = next match;
- Shift+Enter = previous match;
- show selected match position as `X / Y`;
- show native page provenance when known, e.g. `Page 127`;
- show a bounded excerpt/snippet for the selected canonical sentence;
- PDF visual mode must jump native page ownership to the selected match even though exact in-page rectangle geometry does not yet exist;
- Text-only mode may select/follow the exact canonical sentence row;
- empty/no-match states are explicit;
- query changes reset/repair selected-match state deterministically;
- Search stays open while navigating.

Do not implement guessed PDF rectangles.

Add production-shaped later-page PDF tests for buttons + Enter/Shift+Enter + provenance, plus EPUB parity.

## A10 correction 4 — burst TTS seek monotonicity

Physical A9 behavior: repeated Next worked; repeated Previous eventually repeated the same sentence.

Required:

- through the real `TtsRuntime` control worker, a burst of accepted `SeekNext` commands moves monotonically forward one canonical sentence at a time until true end;
- burst `SeekPrev` moves monotonically backward one canonical sentence at a time until true beginning;
- page boundaries and empty PDF pages do not cause replay;
- stale first-sample events from canceled/replaced playback requests cannot reassert the sentence being left;
- command ordering remains bounded and deterministic;
- behavior is correct for enriched PDF and representative EPUB.

Use simulated runtime/first-sample driver coverage, not helper-only tests.

## A10 correction 5 — explicit coarse PDF TTS positioning without fake geometry

Exact click-on-rendered-text cannot exist truthfully until native sentence geometry exists. Do not fake it.

But visual PDF mode must expose an obvious page-level positioning action:

- `Play from current PDF page` / `Start TTS at visible page` in the quick controls;
- it must use authoritative viewport/current native page ownership;
- first sentence on the next non-empty text-bearing native page may be used if the visible page has no accepted text;
- action must be discoverable without opening a diagnostics panel;
- ordinary Text-only sentence rows remain click-to-play for precise canonical positioning.

Exact rendered-text click-to-sentence, visual spoken overlays, and visual PDF auto-follow remain subsequent geometry work.

## A10 correction 6 — stale/damaged EPUB presentation must not survive reopen

The EPUB damaged during A7 reopened later in the same truncated-looking state, while a newly opened EPUB no longer reproduced the stress-toggle race. TTS/Next activity appeared to recover the damaged presentation.

A10 must diagnose this path from evidence rather than assume which cache is guilty.

Required:

- reproduce a prior/stale presentation state through the actual persistence/cache/session reopen path;
- identify whether the surviving state comes from bookmark/session state, dual-view artifacts, Pretty cache identity, structured-document restoration, text-only state, or another derived artifact;
- transient/incomplete presentation state must never become authoritative durable document content;
- stale/corrupt derived presentation artifacts must be rejected/rebuilt safely;
- reopening the same EPUB after stress must render the complete Pretty document immediately, without requiring TTS or Next to repair it;
- preserve bookmark/canonical reading position where valid;
- do not globally discard healthy caches as a brute-force fix;
- new EPUB stress-toggle behavior must remain green.

Add a close/reopen regression that uses persisted QA/cache state, not only one in-memory session.

## Scope exclusions

Do not add:

- PDF sentence rectangles;
- visual spoken sentence overlay;
- visual PDF auto-follow;
- OCR;
- Quack-check;
- Python;
- Docling;
- hostile/mixed recovery.

The Caliberate Recents title/cover defect for materialized hash-named sources is real but belongs to queued Goal 0024, not A10.

## Required validation

At minimum:

- TTS Speed/Volume delayed-ack draft stability;
- bounded/coalesced settings commit behavior;
- central reader layout invariant under diagnostic/status churn;
- Search Previous/Next controls;
- Enter/Shift+Enter search navigation;
- selected match X/Y + native page provenance + excerpt;
- later-page PDF Search navigation;
- burst PDF TTS Next monotonicity;
- burst PDF TTS Previous monotonicity;
- stale first-sample rejection during seek bursts;
- coarse `Play from current PDF page` semantics including empty native pages;
- persisted EPUB close/reopen after Text-only stress;
- complete Pretty document after reopen without TTS repair;
- A8/A9 idempotent Text-only stress regressions;
- A9 Search focus/draft regressions;
- A9 runtime natural page continuation regression;
- Goal 0019/0020 renderer regressions;
- representative EPUB visual/TTS regressions.

Run:

- focused A10 tests;
- `cargo test --workspace -- --test-threads=1`;
- `cargo check --workspace`;
- `cargo build --workspace`;
- `git diff --check`;
- repo-native Windows QA preparation;
- fresh hosted `native-workspace`;
- fresh hosted `hosted-renderer-probe` for the substantive A10 lineage.

Do not terminalize Goal 0022 or signal Goal achieved until the fresh hosted Windows workflow is green.

Update `docs/work/reports/0022.md` with A10 evidence.

Move ready -> active -> done normally.

Push before terminal signaling.

Restore the shared checkout to `main`.

Do not request human QA. Director reviews A10 first.
