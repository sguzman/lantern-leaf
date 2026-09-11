# LanternLeaf Current Status

Updated: 2026-09-10 after Goal 0009 A3 canonical text-only correction director acceptance and correction of the Caliberate-offline QA interpretation.

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

**A3 ACCEPTED — FINAL REAL-DESKTOP TEXT-ONLY SIGNOFF PENDING**

Verified wins remain authoritative:

- pretty EPUB rendering and spoken-sentence synchronization are correct on the real large EPUB;
- sustained ordinary Windows playback no longer shows the prior duplicate-line refill bug;
- new/unoverridden Windows books resolve to Zira;
- explicit per-book Windows voice selection persists across reopen;
- unavailable Piper is transactionally rejected;
- bounded diagnostics, persistent Close book, persistence-gated Safe Quit, and stale-source filtering are implemented and covered by green Windows CI.

A2.1 desktop QA exposed source-dependent text-only behavior: `A General History and Collection of Voyages` had no text/highlight while `Buffalo Bill` worked. A3 fixes the ownership defect: visible text-only rows now come from stable canonical/display document sentences, not `TtsNormalizationPlan.audio_sentences`; text-only row identity remains canonical/display-owned rather than `highlighted_audio_idx`; and snapshot/stats presentation no longer constructs a TTS plan merely to show document text.

A3 implementation `8975cfcb286508e19ac1a983b3e49d35b83d38cf` and Windows CI `34558938955` are accepted. Remaining evidence is one focused desktop pass on both real EPUBs plus a brief pretty-view sanity check.

## Caliberate catalog covers / availability UX

**QUEUED AS GOAL 0010**

A prior desktop attempt appeared to show Caliberate book `42866` failing during materialization, but the user later clarified that Caliberate was not running at the time. That incident is withdrawn as evidence of a LanternLeaf materialization/format defect.

The remaining real catalog issue is cover availability: main catalog entries can appear as black placeholders before open, while Recents can display real covers after local materialization. Goal 0010 is narrowed to first-class lazy catalog covers plus clear provider-unavailable/cover-loading/no-cover states. It must not invent materialization hardening based on the withdrawn offline-provider incident.

## Windows Natural/HD voices

**DEFERRED BY USER — DO NOT WORK ON OR TEST UNTIL RE-AUTHORIZED**

Additional Windows Natural/Narrator voices such as Aria, Guy, and Jenny are being handled in another context. LanternLeaf should preserve the existing working Windows voice backend and ignore Natural/Narrator/HD capability work until the user explicitly reopens that surface. Goal 0011 remains queued/dormant only as a placeholder.

## PDF

**CORE CONTRACTS REPAIRED; NATIVE VISUAL STABILITY WAITS FOR GOAL 0009 CLOSE**

Gate 3 native PDF visual stability remains future work and is not authorized until Goal 0009 receives final real-desktop signoff.

## Workflow status

**MACRO-GOAL / MULTI-ATTEMPT PROTOCOL ACTIVE**

Goal 0009 A3 is integrated and awaiting one human text-only signoff. Goal 0010 is queued in narrowed cover/provider-availability form. Goal 0011 is deferred by user. No PDF implementation is authorized yet.
