# Goal 0009 — A1 Director Acceptance

Status: **ACCEPTED FOR REAL-DESKTOP QA; Goal 0009 remains open pending human signoff.**

Accepted worker terminal head: `fcbe092cdf543e8095ce73c8317c3a777b68d88e`

Accepted implementation: `52ae85dad02f2e5588c14d33817abf0c5db69916`

Authoritative Windows CI: `34517286850`

## Director findings

The A1 implementation is coherent with the Goal 0009 contract and preserves the accepted Goal 0008 architecture.

- App configuration now owns a portable `windows_voice_preference`, checked in as `Zira`. An explicit installed per-book voice ID wins; otherwise Windows resolves the preferred display name deterministically, preferring en-US, then falls back non-fatally to the OS default.
- Per-book persistence is now explicit and versioned through `BookReaderOverrides`. Omitted values inherit current app configuration rather than freezing a whole `AppConfig`. Legacy whole-config book caches retain reader-local appearance/behavior while copied backend/voice values are deliberately not promoted to override intent.
- Reader settings patches record explicit book-local TTS backend/voice/speed/volume intent while process/global model, resource, thread, logging, integration, and keybinding configuration remains app-owned.
- The TTS runtime now carries a continuation cursor across the eight-item preparation/refill batches and bounded normalization-plan windows rather than rebuilding from the last first-sample boundary. The deterministic runtime regression drives 300 ordinary boundaries and observes exactly `0..299`, crossing multiple 64-sentence windows without duplicate ordinary starts. Pause/Resume, Repeat, Next, and Previous retain explicit control semantics.
- Text-only rendering now chooses row selection from the same high-frequency canonical playback ID used for follow/scroll, rather than a stale page-local snapshot projection.
- Real backend/voice setting changes are validated before mutating the canonical session. An unavailable Piper model/config/eSpeak setup produces a failed event and leaves the last-known-good Windows session configuration intact, so a failed switch is transactional rather than session-poisoning.
- No WebView fallback, second ReaderSession authority, whole-document layout regression, or GUI-thread heavy-work path was introduced.

The worker report records passing workspace tests/check, repo-native Windows QA preparation, focused regressions, and `git diff --check`. Windows workflow `34517286850` passed both `native-workspace` and `hosted-renderer-probe`, including staged Windows QA synthesis, workspace check/build/test, Windows TTS probe, and renderer capability probe.

Repository-wide `cargo fmt --all -- --check` still sees pre-existing formatting drift outside Goal 0009; changed files were formatted and this is not treated as a Goal 0009 rejection.

## Remaining human evidence

One normal repo-native Windows QA pass is required before Goal 0009 closes. Verify the same large Caliberate EPUB through sustained Windows speech, then exercise the exact residuals this goal owns:

1. sustained playback does not spontaneously replay just-finished sentences/items;
2. pretty highlight and viewport follow remain synchronized with audible speech;
3. switching to text-only during playback visibly highlights the audible sentence while scroll follow remains correct;
4. a new/unoverridden book uses Zira when installed;
5. changing one book to another Windows voice survives close/reopen while another unoverridden book still inherits the app preference;
6. attempting unavailable/unready Piper fails visibly, after which Windows Play still works in the same open reader session.

If this passes, Goal 0009 / Gate 2.6 closes and Gate 3 native PDF visual stability can be authorized. If any item fails, capture the normal QA behavior/log and reopen Goal 0009 as a bounded correction; do not advance to PDF work first.
