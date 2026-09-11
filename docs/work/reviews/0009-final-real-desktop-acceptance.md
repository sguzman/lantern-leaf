# Goal 0009 — final real-desktop acceptance

Status: **COMPLETE — AUTOMATED + REAL-DESKTOP ACCEPTED.**

Final accepted implementation: `8975cfcb286508e19ac1a983b3e49d35b83d38cf`.

Worker terminal: `7f5f3f77538ecd5fd0acca938f86402003224b2e`.

Authoritative Windows CI: `34558938955` — green.

## Final human evidence

The real desktop pass after A3 closes the source-dependent text-only failure and completes Goal 0009.

The human reports that:

- Windows TTS playback is working correctly;
- ordinary playback no longer repeats completed lines;
- native pretty view spoken-sentence highlighting is synchronized with audible speech;
- pretty-view viewport follow is synchronized;
- text-only now renders text correctly on the previously failing source;
- text-only spoken-sentence highlighting and auto-follow are synchronized;
- both `A General History and Collection of Voyages` and `Buffalo Bill` behave correctly;
- switching between pretty and text-only preserves working synchronization.

Previously accepted real-desktop behavior also remains authoritative:

- new/unoverridden Windows books inherit the app-level Zira preference;
- explicit per-book Windows voice changes persist across reopen;
- unavailable Piper selection is rejected transactionally rather than poisoning the active session;
- Close book / return-to-library and persistence-gated Safe Quit lifecycle fixes remain part of the accepted implementation.

## Goal closure

Goal 0009 is closed. TTS/playback/sentence identity/pretty synchronization/text-only synchronization and layered ordinary-Windows voice ownership are no longer active implementation work.

Two newly observed presentation defects are explicitly **not** grounds to reopen Goal 0009:

1. the native reader no longer exposes the detailed pretty-view visual controls remembered from earlier builds, even though the current config model still contains font family/weight, font size, line spacing, margins, word spacing, letter spacing, highlight colors, and `PrettyUiConfig` fields;
2. real EPUB embedded images are not appearing in pretty view even though the pretty model contains an Image block path.

Those presentation/media issues move to Goal 0012 so the accepted TTS architecture is not destabilized.

Caliberate catalog covers/provider-availability UX remains queued separately as Goal 0010. Windows Natural/Narrator/HD voices remain deferred by the user and must not be investigated or tested until explicitly re-authorized.
