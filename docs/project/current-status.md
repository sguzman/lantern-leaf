# LanternLeaf Current Status

Updated: 2026-09-10 after Goal 0009 A2.1 real-desktop QA exposed a source-dependent text-only presentation defect.

This file contains current verified/bounded state. Detailed attempt history lives in `docs/work/reports/` and `docs/work/reviews/`.

## Workspace / architecture

**VERIFIED / AUTHORITATIVE**

- Native Rust + `eframe`/`egui` is the production desktop architecture.
- Rust owns canonical document/session/playback state.
- Tauri/React/WebView is historical reference only.
- Windows human workflow is repo-native: `git pull -> .\qa.ps1`.
- Scoop is the active Windows CLI dependency convention.

## Gate 0 — Windows baseline

**COMPLETE**

Hosted Windows CI covers MSVC/Pandoc setup, workspace check/build/test, repo-native QA preparation, Windows TTS synthesis/decode, and a separate hosted renderer probe.

## Gate 1 — backend-neutral TTS + Windows TTS

**COMPLETE FOR THE WORKING WINDOWS PATH**

Accepted runtime shape:

`canonical display sentence -> backend synthesis -> prepared audio -> Rodio first-sample boundary -> canonical ReaderSession cursor -> native UI projection`

Real Windows evidence proves audible Windows speech, Play/Pause, ordinary installed voice switching, and sentence-boundary-driven pretty synchronization.

## Goal 0008 / Gate 2.5 — Caliberate first-class reader integration

**COMPLETE — AUTOMATED + REAL-DESKTOP ACCEPTED**

The large real Caliberate EPUB opens quickly, remains responsive, speaks through Windows TTS, highlights the actually audible sentence in native pretty view, and follows playback correctly.

## Goal 0009 / Gate 2.6 — TTS playback polish and layered voice configuration

**A3 REOPENED — TEXT-ONLY PRESENTATION MUST BE DECOUPLED FROM TTS AUDIO PLAN**

Accepted/verified wins that remain authoritative:

- pretty EPUB rendering and spoken-sentence synchronization remain spotlessly correct in the latest desktop pass;
- sustained ordinary Windows playback no longer shows the prior unsolicited duplicate-line behavior;
- a new/unoverridden Windows book resolves to Zira;
- explicit per-book voice selection persists across reopen;
- unavailable Piper is transactionally rejected;
- A2.1 adds ordered Close book / Safe Quit lifecycle handshakes, stale-source rejection, and bounded diagnostics with green Windows CI `34532877674`.

A2.1 real desktop QA exposed a new source-dependent text-only failure:

- `A General History and Collection of Voyages`: text-only shows no text and no highlight;
- `Buffalo Bill`: text-only works correctly.

Director code audit found the ownership defect: while text-only is active, `ReaderSession::current_sentences()` can populate visible rows from `ensure_current_plan(normalizer).audio_sentences`, and `current_highlight_idx()` can use `highlighted_audio_idx`. Visible document text therefore depends on a transient bounded synthesis-normalization plan.

Goal 0009 A3 is reopened in `docs/work/ready/`. It must make text-only a canonical document presentation with exactly one row per canonical/display sentence, independent of TTS plan existence/chunking/backend state, while preserving canonical highlight/follow identity and all verified pretty/TTS/config/lifecycle behavior.

No human QA is requested until A3 passes director review.

## Caliberate catalog reliability

**QUEUED AS GOAL 0010**

Latest desktop QA also produced a separate provider failure: Caliberate book `42866` failed during materialization before reader open. Current diagnostics expose only the top-level `calibre_open_failed` / materialization message, so format/stage/transport detail remains insufficient.

Catalog covers are also a separate provider problem: Caliberate catalog entries can show black placeholders before open, while Recents can display covers after the EPUB has been materialized and its embedded cover becomes locally available. Goal 0010 will address first-class lazy Caliberate covers plus actionable materialization diagnostics/recovery without bulk-fetching the ~100k catalog.

## Windows Natural/HD voices

**QUEUED AS GOAL 0011**

The user has installed additional Windows Natural voices such as Aria, Guy, and Jenny, but LanternLeaf's current `SpeechSynthesizer::AllVoices()` catalog does not surface them on this machine. Goal 0011 will first establish the supported Windows application API/capability boundary, then integrate supported Natural/HD voices if available without regressing the current WinRT voice backend. Do not change the user's default voice until requested.

## PDF

**CORE CONTRACTS REPAIRED; NATIVE VISUAL STABILITY WAITS FOR GOAL 0009**

Gate 3 native PDF visual stability remains future work and is not authorized while Goal 0009 A3 is active.

## Workflow status

**MACRO-GOAL / MULTI-ATTEMPT PROTOCOL ACTIVE**

Goal 0009 is the single authorized goal in `docs/work/ready/`. Continue the existing branch/report lineage as A3. Goals 0010 and 0011 are queued only. No PDF implementation is authorized yet.
