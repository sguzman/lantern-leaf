# 0009 — TTS playback polish and layered voice configuration

## Outcome

Finish the bounded reader/TTS defects discovered during the successful Goal 0008 real-desktop signoff without disturbing the now-working Caliberate/native-EPUB synchronization architecture.

Desired end state:

- normal continuous TTS does not spontaneously replay a just-finished audio item;
- text-only mode visibly highlights the same canonical sentence that its scroll-follow already tracks;
- a new Windows book inherits an app-level preferred Windows voice, **Zira**, from app TOML;
- explicit per-book voice/backend changes persist as book-scoped overrides and win on reopen;
- a failed Piper switch cannot poison the current TTS session, and switching/reverting to Windows restores playback without reopening the book;
- Goal 0008's fast EPUB opening, snappy native UI, pretty sentence highlight, audio-boundary synchronization, and follow behavior remain unchanged.

## Why now

Goal 0008 is finally accepted on real-desktop evidence. The same large Caliberate EPUB now opens quickly, Windows speech works, Windows voice switching works, pretty highlighting follows the actually audible sentence accurately, and viewport scrolling follows playback correctly.

That successful run exposed four narrower defects that should be repaired before moving into native PDF work:

1. audio occasionally repeats the sentence/line it just finished, approximately once every several lines;
2. text-only scrolling tracks playback but the visible sentence highlight is absent/broken;
3. selecting Piper on the Windows QA machine can leave TTS unable to play for the rest of that reader session even though the app remains alive;
4. the desired app-default/per-book voice inheritance model is not represented correctly by the current config merge.

These are one coherent TTS/readability polish goal. They are not a reason to reopen Goal 0008.

## Starting evidence

### Accepted baseline — do not regress

Goal 0008 A8.3 established:

- one canonical shared `ReaderSession` across reader effects, TTS, and persistence;
- source-born `StructuredDocument` sentence identity for native EPUB/HTML;
- explicit canonical IDs on prepared audio and first-sample sentence-boundary messages;
- semantic `SentenceStarted` events instead of predicted-duration cursor advancement;
- active egui repaint scheduling during TTS;
- bounded pretty rendering/virtualization;
- explicit structured source-block/range identity in pretty rendering;
- accurate real-desktop pretty highlight + audio synchronization;
- accurate viewport follow;
- working Windows speech and Windows voice selection;
- fast/snappy large-EPUB reader behavior.

Authoritative A8.3 implementation: `771c31ae90e0bf331195979fffebac8eb2f4ffd6`.

Authoritative Windows CI: `34431615684`.

### Current configuration problem

`AppConfig` already contains `tts_backend` and `windows_voice_id`, and `[tts]` in app TOML already owns those fields.

Per-book cache currently persists an entire `AppConfig` to `<book-cache>/config.toml`. On load, however, `load_session_for_source_with_cancel` explicitly overwrites cached `tts_backend` and `windows_voice_id` with the app/base values. Thus live voice changes can be written but are intentionally prevented from becoming authoritative book-level choices on reopen.

This is the wrong ownership model.

### Current Windows voice resolution

The Windows backend resolves an explicit WinRT voice ID when supplied; otherwise it uses the OS default voice. Enumerated voices expose stable ID, display name, language, and gender.

A portable app default must therefore prefer **Zira by a human-readable preference**, then resolve it to the installed machine-specific WinRT ID. Do not hardcode one machine's opaque voice ID into checked-in config.

### Piper status

Piper already exists as a backend. The current Windows problem is not “implement Piper from scratch.” It is incomplete local provisioning/readiness and unsafe live backend-switch failure recovery. Full Piper model discovery/download/catalog UX is explicitly deferred.

## Architecture decision — layered configuration ownership

Use this precedence:

```text
compiled/platform defaults
        ↓
app conf/config.toml
        ↓
per-book reader overrides
        ↓
live reader session
```

The app config is the inherited baseline. The per-book file stores only book-scoped override intent, not a frozen copy of all global application state.

Introduce an explicit book override schema, e.g. `BookReaderOverrides` / `BookConfigOverrides`, with optional fields. Exact type names are flexible.

Global infrastructure fields remain app-global, including at least:

- logging;
- integration URLs/provider configuration;
- cache location;
- thread counts;
- backend resource/model paths unless there is an explicit future reason to make a model selection book-local;
- keybindings;
- other process/runtime infrastructure.

Book-scoped override fields include at minimum:

- `tts_backend`;
- explicit selected `windows_voice_id`;
- TTS speed/volume if those existing controls are intended to remain book-specific;
- existing reader appearance/behavior fields that the current product intentionally persists per book.

Do not let an old per-book full `AppConfig` silently freeze unrelated app-global values forever.

## Authorized pass A — app-level Zira preference + book override inheritance

### A1. Portable app voice preference

Add a portable app-level Windows voice preference under `[tts]`, conceptually:

```toml
[tts]
windows_voice_preference = "Zira"
```

The exact field name may be `windows_voice_preference`, `windows_voice_name`, or a comparably clear name. Prefer `windows_voice_preference` because it communicates fallback semantics.

The checked-in app configuration should prefer **Windows Zira** on Windows for a new/unoverridden book.

Do not hardcode an opaque WinRT ID in the repository.

Resolution order on Windows:

1. explicit valid per-book `windows_voice_id`;
2. app-level preferred voice match, defaulting to Zira;
3. Windows OS default voice if the preference is unavailable.

Preferred-name matching must be deterministic and practical across Windows display-name variants. A case-insensitive exact/normalized match should win; a unique whole-name/token match for `Zira` is acceptable. If more than one candidate matches, prefer en-US and then a deterministic ID order. Log/diagnose the choice.

If Zira is unavailable, fallback to the Windows default is nonfatal and should produce a concise visible/logged warning rather than making a first-open book unable to speak.

### A2. Per-book override semantics

For a book with no explicit voice/backend override:

- inherit the current app default each time the book is opened;
- do **not** create a book voice override merely by opening it.

When the user explicitly changes the Windows voice for that book:

- persist the selected stable `windows_voice_id` as a book override;
- reopen that book with the selected voice, even if the app-level Zira preference remains unchanged.

When the user explicitly changes backend for a book and that switch succeeds:

- persist that backend as a book override.

Changing app defaults later must affect books with no corresponding override, while books with explicit overrides retain their choices.

Required deterministic scenario:

```text
app preference = Zira
open new Book A -> Zira
change Book A -> Voice B
close/reopen Book A -> Voice B
open new Book B -> Zira
change app preference -> Voice C
reopen Book A -> Voice B
open/reopen unoverridden Book B -> Voice C
```

### A3. Safe migration of legacy per-book config

Existing per-book `config.toml` files are whole serialized `AppConfig` values. The previous loader explicitly discarded cached `tts_backend` and `windows_voice_id` in favor of base config, so those legacy fields cannot automatically be treated as proven user override intent.

Implement an explicit migration/read-compatibility policy:

- preserve existing intentionally book-local reader settings where possible;
- do not interpret a legacy copied backend/voice value as an intentional new override unless there is reliable evidence it was explicitly selected;
- app-global fields continue to come from app config;
- once rewritten, persist the new versioned/explicit override schema rather than another whole `AppConfig` snapshot.

Add schema/version tagging if useful. Migration must be deterministic and covered by tests.

## Authorized pass B — diagnose and eliminate unsolicited audio replay

Do not treat this as another visual synchronization problem. Real desktop evidence shows the pretty highlight remains synchronized when the repeated audio happens.

Instrument the TTS preparation/queue/boundary path enough to distinguish:

- legitimate multiple audio chunks derived from one canonical display sentence;
- the same prepared audio item being queued twice;
- a completed item being replayed at batch/refill/window boundaries;
- stale generation/request work surviving Pause/Resume, voice change, or plan refill.

Each prepared/enqueued/started item should have sufficient diagnostic identity, conceptually:

```text
request/generation id
canonical_display_id
audio_idx
chunk ordinal within display sentence
content hash or stable text hash
prepare/refill batch identity
```

Fix the actual cause. Do **not** add a broad text-string dedupe filter that would suppress legitimate repeated prose or explicit Repeat behavior.

Required deterministic regression:

- drive at least 300 ordinary sequential audio boundaries;
- cross many 8-item prepare batches;
- cross multiple 64-display normalization windows;
- include canonical sentences that normalize into multiple distinct audio chunks;
- assert every normal prepared audio item is enqueued/started exactly once;
- assert a display sentence with multiple distinct chunks can legitimately produce multiple starts while retaining the same canonical visual ID;
- Pause/Resume and normal refill do not replay the last completed item;
- a voice/backend generation change cannot let stale queued audio replay;
- explicit Repeat is the only tested operation allowed to intentionally replay the same selected semantic item;
- Next/Prev retain their explicit navigation semantics.

If the current architecture cannot deterministically reproduce the observed replay, retain the diagnostics and prove the strongest queue/refill invariants available; do not claim root cause without evidence.

## Authorized pass C — text-only visible highlight

The real machine already proves text-only **scroll follow is correct** while the visible highlight is absent/broken. Therefore preserve the canonical cursor/follow system and fix the rendering/style/index projection only.

Requirements:

- text-only row selection and text-only row highlight styling consume the same high-frequency `highlighted_canonical_idx` used by scroll follow;
- do not derive visual selection from a stale full document snapshot when newer lightweight playback state exists;
- the exact currently audible canonical sentence row is visibly highlighted;
- switching pretty -> text-only during active playback shows the same canonical sentence without changing playback cursor;
- switching text-only -> pretty preserves the same identity;
- Pause retains the highlight;
- no per-frame heavyweight snapshot work is introduced.

Add a deterministic rendering/projection regression that distinguishes “row was scrolled to” from “row actually received highlight styling.”

## Authorized pass D — Piper failure recovery on Windows

This goal does **not** implement a Piper model store/catalog/downloader.

It must make existing Piper selection safe.

Before committing a live backend switch to Piper, perform an appropriate lightweight readiness check for the configured Piper resources, including the configured model and its required companion configuration/eSpeak assets as applicable.

If Piper is not ready:

- show an actionable visible error identifying the missing/invalid resource;
- do not persist the failed backend selection as a book override;
- do not poison/cancel the canonical reader session into an unrecoverable TTS state;
- preserve or revert to the last-known-good backend/voice configuration.

After a failed Piper attempt, selecting/returning to Windows must allow Play to work immediately **without closing/reopening the book**.

If Piper is correctly provisioned in a deterministic test fixture, a valid Piper switch should still work through the shared backend-neutral runtime.

## Authorized pass E — regression protection for Goal 0008

Preserve and re-run the accepted Goal 0008 contract:

- Caliberate catalog/provider behavior;
- materialization/native EPUB ingestion/cache recovery;
- fast bounded source open;
- Arc-backed large catalog/document state;
- one canonical shared ReaderSession;
- snapshot-free TTS/persistence hot paths;
- bounded pretty virtualization;
- source-born structured sentence provenance;
- first-sample `SentenceStarted` boundaries;
- active-TTS egui repaint scheduling;
- exact pretty sentence highlight and follow identity;
- Windows voice enumeration/synthesis/playback behavior.

No WebView/Tauri fallback. No full-document layout regression. No heavy GUI-thread work.

## Non-goals

- full Piper model downloading or voice catalog UX;
- packaging a Piper voice collection;
- redesigning source-born EPUB sentence identity;
- PDF rendering or PDF synchronization;
- Caliberate API redesign;
- broad settings-screen redesign;
- unrelated UI cleanup;
- Nix/mise introduction;
- replacing Scoop.

## Constraints

- Native Rust + `eframe`/`egui` remains authoritative.
- Preserve `qa.ps1` as the human Windows entrypoint.
- The QA-scoped Windows backend baseline must not prevent runtime/per-book settings from being tested and changed.
- Book config persistence must be explicit about inheritance/override ownership; absence of an override has semantic meaning.
- Failed backend changes should be transactional: validate/apply/persist only after the requested backend is usable, or restore the last-known-good state on failure.
- Keep high-frequency runtime events lightweight.

## Acceptance gates

1. App TOML contains a portable Windows voice preference whose checked-in default is Zira.
2. On Windows, a new/unoverridden book resolves that preference to installed Zira when available, without hardcoding an opaque machine-specific ID.
3. Missing Zira falls back nonfatally to the OS default with actionable diagnostics.
4. Explicit book voice selection persists as a book-scoped override and wins on reopen.
5. Successful explicit book backend selection persists as a book-scoped override.
6. Changing app voice preference affects books without a voice override but not books with one.
7. Legacy full-AppConfig book caches migrate/read safely without freezing app-global settings or falsely manufacturing voice/backend override intent.
8. 300+ deterministic ordinary playback boundaries cross prepare/refill/normalization windows without an unsolicited duplicate prepared audio item start.
9. Multi-chunk normalization for one canonical display sentence remains valid and does not confuse duplicate detection.
10. Pause/Resume/refill/voice-generation changes do not replay the last completed normal item; explicit Repeat remains intentional replay.
11. Text-only view visibly highlights the exact high-frequency canonical sentence that its scroll-follow tracks.
12. Pretty/text-only mode switches preserve one canonical playback identity.
13. Failed Piper readiness/synthesis does not persist a broken backend or poison the reader session.
14. Windows playback works immediately after reverting from a failed Piper attempt without reopening the source.
15. Goal 0008 pretty highlight/audio/scroll synchronization regressions remain green.
16. Windows workspace check/build/test, repo-native QA preparation, staged Windows TTS synthesis/decode, Windows TTS probe, and hosted renderer probe pass.

## Validation

At minimum:

- `cargo check --workspace`;
- `cargo test --workspace` with normal parallelism and any serial run still required by repository policy;
- focused config inheritance/migration tests;
- focused 300+ item TTS queue/refill replay regression;
- focused text-only visual highlight regression;
- focused failed-Piper -> Windows-recovery regression;
- `git diff --check`;
- repo-native Windows QA preparation;
- normal Windows CI workflow and Windows TTS probe.

Do not weaken existing A8.3 synchronization tests to make this goal pass.

## Repository handoff

Branch: `codex/0009-tts-playback-polish-and-layered-voice-configuration`

Report: `docs/work/reports/0009.md`

Use the normal macro-goal lifecycle:

1. start from current `main`;
2. move this file `ready -> active`;
3. launch the detached Windows goal watcher for Goal 0009;
4. implement/diagnose/test all authorized passes;
5. move to `done` only if all acceptance gates pass, otherwise `blocked` on a true escalation condition;
6. write the report;
7. commit and push the terminal branch;
8. signal terminal state after push;
9. restore the shared checkout to `main` without merging.

## Human verification

**No human QA during implementation.**

After director acceptance only, one real-desktop QA pass should verify:

1. open the same large Caliberate EPUB and let it speak through a sustained stretch; no unsolicited sentence replay should occur;
2. pretty highlight/audio/scroll remain synchronized;
3. switch to text-only while playing; the audible sentence is visibly highlighted and scroll follows it;
4. on a new/unoverridden book, Windows Zira is selected when installed;
5. change Book A to another Windows voice, close/reopen it, and confirm that voice persists;
6. open another unoverridden book and confirm it still inherits the app preference;
7. attempt Piper with an intentionally unavailable/unready configuration, confirm an actionable error, switch back to Windows, and confirm Play works without reopening the book.

Full Piper provisioning/model browsing is future work and is not required for this QA.

## Stop / escalation conditions

Stop and report instead of improvising if:

- existing per-book settings semantics cannot be migrated without a director decision about which non-TTS fields are intended to remain book-local;
- Windows voice enumeration cannot identify Zira in a portable way and requires a product decision beyond deterministic preference matching/fallback;
- the duplicate replay proves to originate below LanternLeaf in an audio/device layer that cannot be deterministically controlled at the current abstraction boundary;
- safe Piper recovery requires a broader backend lifecycle redesign outside the bounded transactional switch contract;
- fixing any item would require reopening Goal 0008's accepted source-born synchronization architecture rather than preserving it.