# LanternLeaf Current Status

Updated: 2026-09-12 after Goal 0012 A3 real-desktop startup rejection and A4 reopening.

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

**A3 REAL-DESKTOP STARTUP REJECTED; A4 FONT-SAFETY CORRECTION READY / SINGLE AUTHORIZED GOAL**

A1–A3 substantially implemented and automated-validated:

- a separate native-egui Presentation settings surface;
- live font family/weight/size, line spacing, margins, word/letter spacing, highlight colors, heading/base/paragraph/block/media controls;
- app-default -> per-book presentation persistence and presentation-only reset;
- safe EPUB image provenance/reference resolution including normalized, relative, nested, encoded, query/fragment-bearing references;
- generated multi-spine PNG/JPEG EPUB production-chain coverage;
- lazy bounded image decode and pretty-build workers with heavy work off the render thread;
- bounded texture state, transient negative cache, visible placeholders, aspect-ratio/media sizing;
- explicit worker-completion repaint wakeups for pretty-build success and image-decode success/failure;
- deterministic tests proving async wakeup before receiver polling and nonblocking queue-full behavior.

A3 implementation `dccc5b999aa7732d1f71a248d8595cf5bde4a40d`, terminal `ae411ffaadebebb23f137b40728dfb4dbc944048`, and Windows CI `34655821185` were accepted for desktop QA.

The first Windows desktop run then exposed a startup regression before functional QA: after font discovery logged `requested_family=Lexend`, egui panicked because `FontFamily::Name("LanternLeafProportionalRegular")` was not bound to any fonts, and LanternLeaf exited with code 101.

Director diagnosis: Goal 0012's font setup uses a coarse `inserted_any` / `fonts_configured` success signal. Unrelated aliases can be registered while the exact configured/global or per-book family alias remains absent; production code then references a named family that was never bound. Missing optional fonts must fall back safely rather than crash.

A4 is bounded to a production-owned font registry/availability contract, safe global TextStyle fallback, safe per-book presentation family/weight fallback, and deterministic controlled-font-availability regressions that force egui text layout. Preserve all A1–A3 presentation/image/async-wakeup work and all Goal 0008/0009 behavior.

No human QA is authorized until A4 is terminal, CI-green, and director-accepted.

## Caliberate catalog covers / availability UX

**QUEUED AS GOAL 0010 — NOT ACTIVE**

The earlier `42866` open failure occurred while Caliberate was not running and remains withdrawn as evidence of a LanternLeaf materialization defect.

The remaining catalog issue is cover availability: main catalog entries can show black placeholders before open while Recents can show real covers after local materialization. Goal 0010 remains queued for first-class lazy catalog covers and clear provider-unavailable/loading/no-cover states.

## Windows Natural/HD voices

**DEFERRED BY USER — DO NOT WORK ON OR TEST UNTIL RE-AUTHORIZED**

Preserve the existing ordinary Windows voice backend. Goal 0011 remains a dormant placeholder only.

## PDF

**CORE CONTRACTS REPAIRED; NATIVE VISUAL STABILITY FUTURE**

Gate 3 native PDF visual stability remains future work and is not authorized during Goal 0012 A4.

## Workflow status

**MACRO-GOAL / MULTI-ATTEMPT PROTOCOL ACTIVE**

Goal 0012 A4 is the single authorized goal in `docs/work/ready/`. Continue the existing `codex/0012-pretty-presentation-controls-and-inline-images` branch/report lineage and synchronize current `main` before implementation. Goal 0010 remains queued; Goal 0011 remains deferred; PDF work remains future.
