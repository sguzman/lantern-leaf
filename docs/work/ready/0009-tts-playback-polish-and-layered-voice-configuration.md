# 0009 — A3 correction: canonical text-only document projection

## Outcome

Close the source-dependent text-only failure exposed by A2.1 real-desktop QA without regressing the now-verified pretty EPUB/TTS path or A2.1 lifecycle/configuration fixes.

## Starting evidence

A2.1 automated gates and Windows CI were green, but real desktop behavior is source-dependent:

- `A General History and Collection of Voyages` opens in pretty view but text-only has no text and no highlight;
- `Buffalo Bill` text-only works correctly;
- pretty view remains spotlessly synchronized with audible Windows TTS;
- code audit identifies the source-dependent ownership bug: `ReaderSession::current_sentences()` uses `ensure_current_plan(normalizer).audio_sentences` for default text-only presentation, and `current_highlight_idx()` substitutes `highlighted_audio_idx` while text-only is active.

That makes visible document text depend on a transient bounded TTS normalization/audio plan. A source whose audio projection is empty, stale, chunked differently, or otherwise unlike its canonical display stream can therefore render no usable text even though the document itself is healthy.

## Authorized A3 correction

### A — separate document presentation from audio preparation

Text-only is a document presentation mode, not an audio-plan presentation mode.

Requirements:

- every canonical/display sentence has exactly one text-only row;
- visible text-only rows come from stable document/canonical display data;
- `current_plan.audio_sentences`, prepared audio batches, or any transient TTS normalization window must never own whether text exists on screen;
- `highlighted_audio_idx` must never be the semantic identity of a text-only row;
- TTS may map one canonical display sentence to zero/one/many audio items internally without changing the one-row-per-canonical-sentence UI contract.

### B — preserve the original-text option without exposing audio chunks

`text_only_show_original_text = true` should display the original/raw canonical display sentence text for the current logical page/stream.

When false, LanternLeaf may display a stable display-normalized version, but it must still remain one-to-one with canonical display sentence identity. It must not expose the TTS audio chunk list or merge/split rows according to synthesis normalization.

Prefer a stable `TextOnlySentenceProjection`/equivalent derived from source/document state and canonical IDs.

### C — text-only must be independent of TTS lifecycle

Text-only content must remain available and nonempty whenever the document has canonical/display text, including when:

- TTS is stopped;
- TTS has never been played;
- playback is paused;
- the current audio plan is absent;
- the current audio plan has been evicted/rebuilt;
- backend validation/synthesis failed;
- a canonical sentence normalizes to zero or multiple audio chunks.

Rendering/snapshotting text-only must not call `ensure_current_plan()` merely to obtain visible text.

### D — canonical highlight/follow remains authoritative

Preserve the A8/A8.3/A2.1 canonical cursor architecture:

- high-frequency `highlighted_canonical_idx` is the semantic spoken-sentence identity;
- text-only projects that canonical ID into the visible row;
- selection styling and auto-follow consume the same canonical projection;
- no audio-index substitution;
- pretty -> text-only during active playback immediately shows/follows the current canonical sentence;
- Pause retains it; Resume continues; text-only -> pretty preserves identity.

### E — regress the actual source-dependent failure class

Add production-path regressions that would fail under the current `audio_sentences` ownership model.

At minimum:

1. Build a ReaderSession/document with canonical display sentences but no current TTS plan; enable text-only and assert visible rows remain nonempty and preserve canonical count/order.
2. Use canonical sentences whose TTS normalization deliberately produces zero/multiple audio items or a different audio count; text-only must still expose exactly one row per canonical sentence.
3. Construct/rebuild/evict or otherwise replace the TTS plan after text-only projection and prove visible text identity/count do not change.
4. Assert requesting a text-only snapshot/projection does not itself construct a TTS normalization plan; use an existing/new diagnostic counter if useful.
5. Cover two source shapes: one straightforward and one awkward/archaic/metadata-heavy EPUB-like stream representative of the observed source-dependent failure.
6. Drive at least 48 canonical playback boundaries through the production text-only projection, including a page/stream transition where applicable, and assert selection + follow remain canonical.
7. Test stopped -> text-only -> Play and active pretty -> text-only -> Pause -> Resume -> pretty.

Do not satisfy this with index arithmetic alone.

## Preserve

A3 must retain all accepted behavior from Goal 0008 and Goal 0009 A1/A2.1:

- fast/snappy large EPUB opening;
- pretty spoken-sentence highlight and viewport follow;
- first-sample audio boundaries and canonical sentence identity;
- no unsolicited duplicate ordinary line reads;
- Windows voice playback and portable Zira default;
- per-book Windows voice override persistence;
- bounded/resizable diagnostics panel;
- transactional failed-Piper recovery;
- persistent confirmation-first Close book lifecycle;
- persistence-gated Safe Quit;
- stale-source playback rejection;
- one canonical ReaderSession and lightweight hot paths.

## Explicit non-goals

Do not expand A3 into:

- Windows Narrator Natural/HD voice support; queued separately;
- Caliberate materialization hardening or catalog covers; queued separately;
- PDF visual work;
- full Piper model catalog/downloader UX;
- broad UI redesign;
- WebView/Tauri fallback.

## Acceptance gates

1. Text-only visible sentence data is derived from canonical/display document state, never from `TtsNormalizationPlan.audio_sentences` or prepared audio batches.
2. Text-only row identity uses canonical/display ID, never `highlighted_audio_idx`.
3. A document with canonical text and no TTS plan still renders complete text-only content.
4. Zero/one/many audio-item normalization cannot change text-only row count or erase text.
5. Text-only rendering/snapshotting does not create a TTS plan as a presentation side effect.
6. Production pretty -> text-only transition, active playback, Pause/Resume, stopped playback, 48+ subsequent canonical boundaries, and return-to-pretty all preserve selection/follow identity.
7. Source-dependent regression fixtures cover both a straightforward and awkward EPUB-like sentence stream.
8. Existing pretty sync, 300+ no-repeat playback, voice inheritance/override, Piper recovery, Close book, Safe Quit, stale-source, Caliberate, and Windows TTS regressions remain green.
9. `cargo check --workspace`, repository-policy workspace tests, `git diff --check`, repo-native Windows QA preparation, Windows TTS probe, and hosted renderer probe pass.
10. No human QA until director review accepts A3.

## Repository handoff

Continue branch `codex/0009-tts-playback-polish-and-layered-voice-configuration` and report `docs/work/reports/0009.md`. Preserve all A1/A2/A2.1 history and append Attempt A3. This is a fresh Codex Goal execution attempt, not a new macro-goal. Synchronize current director state from `main`, move this goal `ready -> active`, re-arm the Goal 0009 watcher, implement/validate, terminalize `done` or `blocked`, push, signal terminal state, and restore the shared checkout to `main` without merging.

## Human verification

No human QA during A3. After director acceptance, repeat text-only on both `A General History and Collection of Voyages` and `Buffalo Bill`, plus a short pretty-path regression sanity check.

## Stop / escalation

Stop rather than redesign the source-born canonical identity architecture. If canonical document state itself is genuinely absent for the failing source, report that evidence and exact ingestion seam instead of masking it with an audio-plan fallback.
