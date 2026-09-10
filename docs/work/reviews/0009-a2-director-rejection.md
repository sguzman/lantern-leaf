# Goal 0009 — A2 Director Review

Status: **REJECTED BEFORE HUMAN QA; A2.1 CORRECTION REQUIRED.**

A2 implementation: `e6bbc065462434950802818d0b4236464c244d5a`.

A2 worker terminal: `87ec9257948bbc8dff277c8a7d8c8b3d44ef3d31`.

A2 authoritative Windows CI: `34528119986` — `native-workspace` and `hosted-renderer-probe` both passed.

## What A2 got right

The production changes are directionally sound and must be preserved rather than rewritten:

- Safe Quit no longer plans an unordered persistence + close race. The app-shell path now waits for a SafeQuit persistence terminal event and asks egui to close through `ViewportCommand::Close` on the UI owner.
- Close book is now persistent top chrome, asks for confirmation before destruction, stops TTS, waits for SessionClose persistence, then clears the reader and returns to Starter; persistence failure leaves the reader open.
- stale TTS playback events from a closed/different source are filtered before they can update the active reader projection.
- the left panel is explicitly resizable with a 240 px minimum, 320 px default, and 460 px maximum, and long TTS/voice diagnostic labels are wrapped.
- text-only rendering now has a production-owned `TextOnlyRowProjection`; the mode-switch path re-arms follow from the live canonical playback identity, and a page transition refreshes the document projection only when the lightweight playback cursor changes pages.
- the existing A1/Goal 0008 no-repeat, pretty-sync, layered voice, Caliberate, and Windows build/test paths remain green in CI.

These are useful implementation advances. The rejection is not a request to undo them.

## Why A2 is not accepted

The A2 correction contract was deliberately stricter than A1 because A1 already demonstrated that arithmetic/unit-level cursor tests can pass while the actual desktop transition is broken. A2 terminalized `done` without supplying several explicitly required deterministic gates.

### 1. Text-only regression still tests arithmetic, not the production transition

The new test `text_only_projection_tracks_mode_switch_and_many_follow_boundaries` only loops `canonical_idx in 0..48`, calls `text_only_row_projection_for_page`, and checks `local_idx == canonical_idx`. It does **not** exercise the production pretty -> text-only action, `text_only_override`, asynchronous `ToggleTextOnly` ordering, `AutoScrollState::request_cursor/pending_for/decide_scroll/record`, visible selected-row state, subsequent `SentenceStarted` updates, Pause retention, a page transition, or text-only -> pretty identity preservation.

That is exactly the class of insufficient test the A2 contract prohibited. No new human QA should be used to discover whether the production lifecycle still fails.

### 2. Close-book lifecycle/stale-event gate lacks the required regression

The implementation has a plausible ordered close path and a stale-source playback filter, but the A2 validation does not demonstrate the requested lifecycle:

`active TTS -> confirmation before close -> Stop/cancel -> SessionClose persistence terminal success -> CloseReaderSession -> Starter`

It also does not prove the failure branch leaves the book open or that an old-source playback event cannot mutate a subsequently opened source through the app-level path.

### 3. Safe Quit has only a planning test, not the terminal-to-native-close handshake test

`safe_quit_plan_exposes_only_persistence_terminal_effect` correctly prevents the old race, but it does not prove the egui-owned handshake:

`SafeQuit -> Stop -> persistence pending -> no native close yet -> persistence success -> pending native close -> ViewportCommand::Close consumed`

Nor does it prove persistence failure leaves the app open. The production code is promising; the required lifecycle evidence is missing.

### 4. Long-diagnostic containment lacks deterministic protection

The 240/320/460 side-panel policy and wrapped labels are appropriate, but A2 added no deterministic regression/equivalent proving that a 300+ character path/error cannot create a width demand beyond the 460 px policy. This was an explicit acceptance item because the failure was a real-desktop layout regression.

### 5. Failed-Piper -> Windows -> Play same-session gate is still not tested

The existing `failed_piper_switch_is_transactional_and_leaves_session_recoverable` test is unchanged from A1. It verifies the failed Piper request emits an error and that `config.tts_backend` remains `Windows`, then stops. It never calls Play afterward, never observes playback/boundary progress, and therefore does not establish the A2 requirement:

`Windows working -> unavailable Piper -> failure -> Windows retained/restored -> Play succeeds immediately in the same ReaderSession`

No source close/reopen is permitted in this regression.

## A2.1 correction contract

Continue the same Goal 0009 branch/report lineage. Preserve A2 production code unless the required tests expose an actual defect. This is a bounded evidence-and-correction pass, not another architecture rewrite.

Required before terminalizing again:

1. **Production text-only lifecycle regression.** Exercise the same production-owned mode-switch helper/action used by the UI. Start with active pretty playback and a known canonical cursor, switch to text-only, assert the exact current row is selected and follow is pending then consumed, drive at least 48 subsequent playback/SentenceStarted cursor transitions including at least one page transition, and assert selection + follow move together. Pause must retain the row. Switching back to pretty must preserve canonical identity. If direct egui response inspection is impractical, extract a small production-owned transition/follow projection used by both UI and test; do not fall back to index arithmetic alone.
2. **Bounded diagnostic presentation regression.** A 300+ character Piper error/path must exercise production-owned panel/presentation policy and prove the maximum requested/allowed width is <= 460 px while the message remains available/wrapped. Extract a small layout policy helper if needed, but production must use it.
3. **Close-book lifecycle regression.** Prove confirmation precedes destruction; active TTS is stopped/cancelled; SessionClose persistence must terminalize successfully before CloseReaderSession; success returns to Starter; persistence failure leaves the reader open; and stale playback from the old source is ignored after another source becomes active.
4. **Safe-Quit handshake regression.** Prove no native-close intent exists before SafeQuit persistence completion; success arms exactly one native-close intent consumed by the egui owner; failure does not close and exposes failure state. A small production-owned shutdown state machine/helper is acceptable if direct viewport-command assertion is awkward.
5. **Failed Piper same-session recovery regression.** In one ReaderSession, establish a working Windows state, request unavailable Piper and observe transactional rejection, assert no broken Piper state/override becomes authoritative, immediately issue Windows Play without reopening the source, and prove playback begins (TTS state plus a first boundary/progress signal through a deterministic seam). If host tests cannot instantiate real WinRT speech, introduce the narrowest testable backend-readiness/playback seam rather than weakening the requirement.
6. Re-run the existing 300+ ordered no-repeat regression, Goal 0008 synchronization regressions, workspace check/test, repo-native Windows QA preparation, Windows TTS probe, hosted renderer probe, and `git diff --check`.

## Director decision

Do **not** merge A2 into `main` and do **not** ask for human QA yet. The implementation branch is ahead of the director base and should be continued in place. Start a fresh Codex Goal session as A2.1, synchronize this review from `main`, re-arm the Goal 0009 watcher, add the missing deterministic gates, fix anything those gates reveal, append A2.1 to `docs/work/reports/0009.md`, and terminalize only after the contract is actually satisfied.

Gate 3 / PDF remains unauthorized until Goal 0009 passes director review and the eventual focused real-desktop signoff.
