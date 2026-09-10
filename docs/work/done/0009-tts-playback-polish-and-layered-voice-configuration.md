# 0009 — A2 correction: text-only follow, bounded diagnostics, and reliable reader/app exit

## Outcome

Close the remaining real-desktop defects after A1 without regressing the now-verified A1 wins.

A1 real-desktop evidence to preserve:

- the same large Caliberate EPUB pretty view remains fast and responsive;
- sustained Windows TTS had no unsolicited duplicate just-finished line reads;
- pretty highlight and viewport follow remain synchronized with audible speech;
- a new/unoverridden book resolves to Zira;
- an explicit Mark voice selection persisted after close/restart/reopen;
- unavailable Piper produces an actionable missing-model failure and the app remains alive.

Still not fully human-signed: same-session Windows Play after a failed Piper attempt.

A1 implementation: `52ae85dad02f2e5588c14d33817abf0c5db69916`.

A1 worker terminal: `fcbe092cdf543e8095ce73c8317c3a777b68d88e`.

A1 Windows CI: `34517286850`.

## A2 production failures

1. Pretty -> text-only during active playback produces neither visible sentence highlight nor auto-scroll. This is worse than the pre-A1 state, where text-only follow worked even though styling did not.
2. A long Piper failure message forces the left TTS/settings panel to expand aggressively and makes useful manual resizing effectively unavailable until the content changes.
3. Safe quit confirmation does not close the application. Director code audit confirms `RuntimeEffect::SafeQuit` reaches `handle_safe_quit`, which only logs and returns no event/action.
4. Leaving the current book is not discoverable or trustworthy. The reader currently contains `Back to starter` and `Close reader session`, but `Close reader session` dispatches the destructive close before setting its confirmation modal flag, and the controls are buried in document content rather than persistent reader chrome.

These failures reopen Goal 0009. PDF work remains unauthorized.

## Authorized pass A — production text-only follow and highlight

Reproduce the actual production transition: active pretty playback -> `ToggleTextOnly` -> continued `SentenceStarted` boundaries. Do not treat the A1 arithmetic/projection unit test as proof of UI behavior.

Diagnose the live canonical cursor, current playback page, `text_only_override`, stale/full `ReaderSnapshot` state, mode-toggle event ordering, and `AutoScrollState` pending/consume lifecycle. Text-only selection and follow must derive from the same current playback projection actually used by production. Do not use `snapshot.current_page` or page-local snapshot identity when lightweight playback has newer canonical/page identity.

Requirements:

- switching pretty -> text-only during active playback immediately preserves and visibly selects the currently audible canonical sentence;
- the switch itself arms or retains follow for that current sentence so the text-only viewport lands on it;
- subsequent boundaries continue to update both visible selection and scroll follow;
- Pause retains the highlight;
- text-only -> pretty preserves the same canonical identity;
- no heavyweight per-frame snapshot/layout work is introduced.

Prefer one small production-owned projection/helper (for example `TextOnlyRowProjection`) that determines the current canonical row and follow target, and have deterministic tests call that exact code. Regression coverage must simulate a mode switch plus dozens of subsequent boundaries and assert both row-selected styling state and scroll/follow consumption, not merely canonical index arithmetic.

## Authorized pass B — bounded/resizable diagnostics panel

The left settings/stats/TTS panel must have sane minimum/default/maximum widths and remain user-resizable. Content must never force it past the maximum.

Long TTS errors and long filesystem paths must wrap or otherwise remain inside the panel. Full detail may remain available through wrapped text, a tooltip, or another bounded/copyable diagnostic presentation. A 300+ character Piper path/error must not create an intrinsic minimum width wider than the panel maximum or trigger application/window resizing.

Do not hide the actionable error; fix its layout behavior.

## Authorized pass C — reliable Close book / Back to library

Add one clear persistent reader-mode action in top chrome, named `Close book` or `Back to library`. It must be visible regardless of document scroll position.

The authoritative close-book lifecycle is:

`stop/cancel current TTS -> persist current bookmark/book overrides -> clear reader session -> return to Starter/library`

It must not exit the application. Closing a book during active TTS must invalidate/cancel stale boundary/runtime work so events from the old book cannot mutate a subsequently opened book.

Fix the existing confirmation semantics. Never perform the destructive close before asking for confirmation. It is acceptable to remove/simplify duplicate in-content `Back to starter` / `Close reader session` controls once the persistent action is authoritative.

## Authorized pass D — Safe Quit actually exits

`AppCommand::SafeQuit` must result in an actual native egui application/window close. The current logging-only `handle_safe_quit` is not an implementation.

Preserve ordering:

`stop/cancel TTS -> persist/flush current state -> persistence completion/terminal handling -> request native egui viewport/app close`

Do not dispatch persistence flush and native close as an unordered race. Do not call `process::exit` from a worker thread. If needed, introduce a lightweight `QuitReady`/equivalent event consumed by the egui owner after persistence completion. The confirmation modal's Quit button must deterministically reach the real close path. Normal window-X behavior remains available.

## Authorized pass E — complete failed-Piper recovery proof

Preserve A1 transactional validation and add an end-to-end same-session regression:

`Windows working -> request unavailable Piper -> failure visible -> no Piper override persisted -> select/retain Windows -> Play succeeds immediately in the same open ReaderSession`

No source close/reopen is allowed in that regression. Error presentation must also obey the bounded panel rule.

## Regression protection

Preserve all verified A1/Goal 0008 behavior, especially:

- no duplicate ordinary playback across refill/window progression;
- source-born EPUB canonical identity and first-sample boundaries;
- accurate responsive pretty highlight/follow;
- large Caliberate EPUB responsiveness;
- one canonical shared ReaderSession;
- lightweight TTS/persistence hot paths;
- portable Zira default and explicit per-book voice persistence;
- Caliberate provider/materialization behavior.

## Non-goals

- PDF implementation;
- full Piper model downloader/catalog/provisioning UX;
- broad UI redesign;
- WebView/Tauri fallback;
- another ReaderSession authority;
- undoing A1 voice/configuration or Goal 0008 synchronization architecture.

## Acceptance gates

1. Existing 300+ ordinary-boundary/no-repeat regression remains green and real-desktop A1 duplicate fix is protected.
2. Pretty playback highlight/follow synchronization remains green.
3. Pretty -> text-only during active playback immediately visibly highlights the audible canonical sentence and lands/follows the text-only viewport on it.
4. Continued text-only playback updates both highlight and auto-scroll across many subsequent boundaries; Pause retains highlight; text-only -> pretty preserves identity.
5. Long Piper errors/paths cannot force the side panel or app window wider; the side panel remains usefully user-resizable.
6. A persistent `Close book` / `Back to library` action is visible in reader mode and returns to Starter/library without exiting the app.
7. Any close-book confirmation occurs before destructive close, never afterward.
8. Closing a book while TTS is active cancels/invalidates old playback and stale events cannot affect the next book.
9. Safe quit confirmation causes an actual native app/window close only after ordered TTS cancellation and persistence completion/terminal handling.
10. Failed/unready Piper remains transactional, persists no broken backend override, and Windows Play works immediately afterward in the same open session.
11. Zira default inheritance and explicit Mark-style per-book voice persistence remain green.
12. Workspace check/test, repo-native Windows QA preparation, Windows TTS probe, and hosted renderer probe pass.

## Validation

At minimum:

- focused production-path text-only mode-switch + follow/highlight tests;
- deterministic long-diagnostic/panel-bound layout test or equivalent bounded presentation test;
- close-book lifecycle test including active TTS and stale-event isolation;
- safe-quit ordering/actual-close handshake test;
- failed-Piper -> Windows same-session playback recovery test;
- existing 300+ boundary regression;
- `cargo check --workspace`;
- `cargo test --workspace` under repository policy;
- `git diff --check`;
- repo-native Windows QA preparation and normal Windows CI.

## Repository handoff

Branch remains `codex/0009-tts-playback-polish-and-layered-voice-configuration`.

Report remains `docs/work/reports/0009.md`; append Attempt A2 and preserve A1 history.

This is a fresh Codex Goal execution session but the same repository macro-goal. Synchronize this director correction from `main`, move this file `ready -> active`, re-arm the existing Goal 0009 watcher, implement/validate, then use the normal terminal `done`/`blocked` handoff and restore the shared checkout to `main` without merging.

## Human verification

**No human QA during A2 implementation.** After director review/acceptance only, request one focused real-desktop pass covering text-only highlight+follow, bounded Piper error layout and same-session Windows recovery, Close book, actual Safe Quit, and a sanity check of the already-passing pretty/no-repeat/Zira/per-book voice behavior.

## Stop / escalation conditions

Stop rather than redesign broadly if fixing text-only requires replacing source-born canonical identity, if native egui cannot provide an ordered close request without a broader app-shell lifecycle decision, or if reliable old-session event invalidation requires changing the single canonical ReaderSession ownership model. Otherwise these are bounded Goal 0009 corrections.