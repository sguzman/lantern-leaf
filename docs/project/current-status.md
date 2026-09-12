# LanternLeaf Current Status

Updated: 2026-09-12 after Goal 0012 A4 director review.

This file contains current verified/bounded state. Detailed attempt history lives in `docs/work/reports/` and `docs/work/reviews/`.

## Workspace / architecture

**VERIFIED / AUTHORITATIVE**

- Native Rust + `eframe`/`egui` is the production desktop architecture.
- Rust owns canonical document/session/playback state.
- Tauri/React/WebView is historical reference only.
- Windows human workflow is repo-native: `git pull -> .\qa.ps1`.
- Scoop is the active Windows CLI dependency convention.
- Heavy/blocking work must never run on the egui/render thread.

## Gate 0 — Windows baseline

**COMPLETE**

Hosted Windows CI covers MSVC/Pandoc setup, workspace check/build/test, repo-native QA preparation, Windows TTS synthesis/decode, and a separate hosted renderer probe.

## Gate 1 — backend-neutral TTS + Windows TTS

**COMPLETE FOR THE WORKING WINDOWS PATH**

Accepted runtime shape:

`canonical display sentence -> backend synthesis -> prepared audio -> Rodio first-sample boundary -> canonical ReaderSession cursor -> native UI projection`

Real Windows evidence proves audible Windows speech, Play/Pause, ordinary installed voice switching, and sentence-boundary-driven pretty/text-only synchronization.

## Goal 0008 / Gate 2.5 — Caliberate first-class reader integration

**COMPLETE — AUTOMATED + REAL-DESKTOP ACCEPTED**

The large real Caliberate EPUB opens quickly, remains responsive, speaks through Windows TTS, highlights the actually audible sentence in native pretty view, and follows playback correctly.

## Goal 0009 / Gate 2.6 — TTS playback polish and layered voice configuration

**COMPLETE — AUTOMATED + REAL-DESKTOP ACCEPTED**

Stable audible TTS, no unsolicited duplicate ordinary lines, correct pretty/text-only spoken-sentence highlight/follow, portable Zira inheritance, per-book voice persistence, transactional Piper rejection/recovery, and accepted Close book / Safe Quit lifecycle are real-desktop accepted.

## Goal 0012 — pretty presentation controls and inline images

**A4 REJECTED BEFORE HUMAN QA; A5 LAYOUT-PROOF CORRECTION READY / SINGLE AUTHORIZED GOAL**

A1–A3 remain substantively accepted and preserved: native Presentation controls, layered persistence/reset, EPUB inline-image provenance/rendering, bounded off-render-thread pretty/image workers, and explicit worker-completion repaint wakeups.

The first A3 desktop signoff exposed a startup panic when the configured Lexend family was absent: egui attempted to use unbound `FontFamily::Name("LanternLeafProportionalRegular")`.

A4 implementation `c92256fb429aea1d13707a811b0147e4337bbd05` is directionally correct and should be preserved. It replaces the coarse `fonts_configured` boolean with an exact alias `FontRegistry`, uses registered aliases only for global styles, and falls back to built-in proportional/monospace families when requested aliases are absent. Windows workflow rerun `34714389917` passed native workspace and hosted renderer jobs.

A4 is not director-accepted because:

- the worker branch did not synchronize the actual latest director `main` and is diverged from the director A4 documentation/status lineage;
- the new font regression only tests resolver/helper return values and does not install controlled font definitions into an egui `Context` and force the real epaint text-layout path that previously panicked;
- the full controlled missing/partial font matrix from the A4 contract is not covered.

A5 is bounded to synchronizing current director `main`, preserving `c92256f`, and adding deterministic real-egui layout regressions for missing Lexend, partial aliases, missing bold/monospace, unavailable per-book family, available alias selection, bound-name invariants, and config preservation.

No human QA is authorized until A5 is terminal, CI-green, and director-accepted.

## Caliberate catalog covers / availability UX

**QUEUED AS GOAL 0010 — NOT ACTIVE**

The earlier `42866` open failure occurred while Caliberate was not running and remains withdrawn as evidence of a LanternLeaf materialization defect.

The remaining catalog issue is cover availability: main catalog entries can show black placeholders before open while Recents can show real covers after local materialization. Goal 0010 remains queued for first-class lazy catalog covers and clear provider-unavailable/loading/no-cover states.

## Windows Natural/HD voices

**DEFERRED BY USER — DO NOT WORK ON OR TEST UNTIL RE-AUTHORIZED**

Preserve the existing ordinary Windows voice backend. Goal 0011 remains a dormant placeholder only.

## PDF

**CORE CONTRACTS REPAIRED; NATIVE VISUAL STABILITY FUTURE**

Gate 3 native PDF visual stability remains future work and is not authorized during Goal 0012 A5.

## Workflow status

**MACRO-GOAL / MULTI-ATTEMPT PROTOCOL ACTIVE**

Goal 0012 A5 is the single authorized goal in `docs/work/ready/`. Continue the existing `codex/0012-pretty-presentation-controls-and-inline-images` branch/report lineage, synchronize the actual latest `main` before implementation, and preserve A4 implementation `c92256f`. Goal 0010 remains queued; Goal 0011 remains deferred; PDF work remains future.
