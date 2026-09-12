# LanternLeaf Current Status

Updated: 2026-09-12 after Goal 0012 A3 director acceptance for focused real-desktop QA.

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

**A3 DIRECTOR-ACCEPTED — FOCUSED REAL-DESKTOP QA PENDING**

Goal 0012 now includes:

- a separate native-egui Presentation settings surface;
- live font family/weight/size, line spacing, margins, word/letter spacing, highlight colors, heading/base/paragraph/block/media controls;
- app-default -> per-book presentation persistence and presentation-only reset;
- safe EPUB image provenance/reference resolution including normalized, relative, nested, encoded, query/fragment-bearing references;
- generated multi-spine PNG/JPEG EPUB production-chain coverage;
- lazy bounded image decode and pretty-build workers with heavy work off the render thread;
- bounded texture state, transient negative cache, visible placeholders, and aspect-ratio/media sizing;
- explicit worker-completion repaint wakeups for pretty-build success and image-decode success/failure, so idle/TTS-off readers consume completed async work without needing unrelated input;
- deterministic tests proving those repaint notifications occur before receiver polling and queue-full submission remains nonblocking.

A3 implementation `dccc5b999aa7732d1f71a248d8595cf5bde4a40d`, worker terminal `ae411ffaadebebb23f137b40728dfb4dbc944048`, and terminal Windows CI `34655821185` are director-accepted automated evidence.

One focused Windows desktop pass is now authorized for live presentation controls, persistence/reset, real EPUB inline images while idle, and a short Goal 0009 TTS synchronization sanity check. Goal 0012 is not finally closed until that human pass succeeds.

## Caliberate catalog covers / availability UX

**QUEUED AS GOAL 0010 — NOT ACTIVE**

The earlier `42866` open failure occurred while Caliberate was not running and remains withdrawn as evidence of a LanternLeaf materialization defect.

The remaining catalog issue is cover availability: main catalog entries can show black placeholders before open while Recents can show real covers after local materialization. Goal 0010 remains queued for first-class lazy catalog covers and clear provider-unavailable/loading/no-cover states.

## Windows Natural/HD voices

**DEFERRED BY USER — DO NOT WORK ON OR TEST UNTIL RE-AUTHORIZED**

Preserve the existing ordinary Windows voice backend. Goal 0011 remains a dormant placeholder only.

## PDF

**CORE CONTRACTS REPAIRED; NATIVE VISUAL STABILITY FUTURE**

Gate 3 native PDF visual stability remains future work and is not authorized while Goal 0012 awaits desktop signoff.

## Workflow status

**HUMAN QA GATE ACTIVE**

Goal 0012 is in `docs/work/done/` at the worker level and is director-accepted for one focused desktop pass. No new Codex goal is authorized until that pass is reviewed. Goal 0010 remains queued; Goal 0011 remains deferred; PDF work remains future.