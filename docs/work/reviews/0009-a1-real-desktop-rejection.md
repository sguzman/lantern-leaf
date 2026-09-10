# Goal 0009 — A1 real-desktop review

Status: **PARTIAL PASS; FINAL ACCEPTANCE REJECTED; A2 CORRECTION AUTHORIZED.**

A1 implementation: `52ae85dad02f2e5588c14d33817abf0c5db69916`.

A1 worker terminal: `fcbe092cdf543e8095ce73c8317c3a777b68d88e`.

A1 Windows CI: `34517286850`.

## Real-desktop evidence

The A1 run produced meaningful verified progress and those wins must be preserved:

- the same large Caliberate EPUB pretty view is working well, responsive, and without the prior hiccups;
- sustained Windows playback did not reproduce the unsolicited duplicate just-finished line reads;
- pretty spoken-sentence highlight and viewport follow remain synchronized;
- a new/unoverridden book selected Zira as the default Windows voice;
- changing a book to Mark persisted across restart/reopen;
- unavailable Piper emitted an actionable missing-model error and did not crash the app.

Final acceptance fails because the same run exposed/confirmed these production defects:

1. switching pretty -> text-only during active playback produces neither visible sentence highlight nor auto-scroll. This is a regression from the pre-A1 state where text-only follow worked even though highlighting did not;
2. the long Piper failure text aggressively expands the left TTS/settings panel and makes resizing practically unusable until the text changes;
3. Safe quit + its confirmation does not close the application;
4. the user could not discover a reliable way to leave the current book and had to terminate the whole app to open a clean session.

Same-session Windows playback after a failed Piper attempt was not conclusively human-verified and remains part of A2.

## Director code audit

The desktop failures are consistent with concrete production code, not a reason to repeat the same QA.

- `RuntimeEffect::SafeQuit` reaches `handle_safe_quit`, but that function only logs `Safe quit requested (egui dispatcher)` and returns `Ok(Vec::new())`; it has no native-close action.
- `AppCommand::SafeQuit` currently plans a persistence flush and `RuntimeEffect::SafeQuit` as separate effects, so an eventual real close must also establish ordering rather than racing persistence.
- reader content contains `Back to starter` and `Close reader session`, but the latter dispatches `CloseReaderSession` before setting `show_reader_confirm_modal = true`, so its confirmation occurs after the destructive command.
- the left egui `SidePanel` has no explicit width bounds while long TTS failure text is presented directly in panel content, allowing intrinsic content width to dominate the shell.
- A1 text-only rendering does compare a selectable row against a canonical playback index, but real desktop proves that this projection/follow lifecycle does not survive the actual pretty -> text-only mode transition. A2 must test the production transition and subsequent boundaries, not merely index arithmetic.

## Decision

Reopen repository Goal 0009 for Attempt A2 on the same branch/report lineage. Preserve A1's verified audio/configuration/pretty wins while repairing production text-only follow/highlight, bounded diagnostic layout, a persistent Close book/Back to library lifecycle, actual ordered Safe Quit, and explicit failed-Piper -> Windows same-session playback recovery.

Gate 3 / PDF remains unauthorized until A2 receives director review plus the required focused real-desktop signoff.
