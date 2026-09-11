# LanternLeaf Current Status

Updated: 2026-09-11 after Goal 0012 A1 director review.

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

Final desktop evidence establishes stable audible TTS, no unsolicited duplicate ordinary lines, correct pretty/text-only spoken-sentence highlight/follow, portable Zira inheritance, per-book voice persistence, transactional Piper rejection/recovery, and accepted Close book / Safe Quit lifecycle.

A3 implementation `8975cfcb286508e19ac1a983b3e49d35b83d38cf` / worker terminal `7f5f3f77538ecd5fd0acca938f86402003224b2e` / Windows CI `34558938955` are final accepted evidence. Goal 0009 is closed.

## Goal 0012 — pretty presentation controls and inline images

**A1 REJECTED BY DIRECTOR; A2 BOUNDED CORRECTION READY / SINGLE AUTHORIZED GOAL**

A1 implementation `03a315b61d9cf4449b2de94ed59c1287fd2efba0` plus `b60f83b26e6579c99382c85049250d060748c1c4` is substantively strong and Windows CI `34628740493` is green. It adds the native Presentation surface, live/persistent presentation settings, EPUB image provenance, safe relative/nested/encoded asset resolution, generated EPUB fixtures, bounded off-render-thread image decoding, placeholders, sizing, and bounded texture state.

Director review found one blocker before human QA: the asynchronous pretty-build and image-decode workers publish channel results but do not themselves wake an idle egui event loop. Their receivers call `request_repaint()` only after some later frame already occurs. With TTS stopped and no user input, ready pretty content or decoded images are therefore not guaranteed to appear promptly.

A2 is strictly bounded to explicit worker-completion repaint notification. It must preserve A1, keep request submission bounded/nonblocking, keep all heavy work off the render thread, and add deterministic tests proving successful image decode, failed image decode, and pretty-build completion each cause a repaint wakeup independently of TTS or subsequent polling.

No human QA is authorized until A2 is director-accepted.

## Caliberate catalog covers / availability UX

**QUEUED AS GOAL 0010 — NOT ACTIVE**

The earlier `42866` open failure occurred while Caliberate was not running and remains withdrawn as evidence of a LanternLeaf materialization defect.

The remaining catalog issue is cover availability: main catalog entries can show black placeholders before open while Recents can show real covers after local materialization. Goal 0010 remains queued for first-class lazy catalog covers and clear provider-unavailable/loading/no-cover states.

## Windows Natural/HD voices

**DEFERRED BY USER — DO NOT WORK ON OR TEST UNTIL RE-AUTHORIZED**

Preserve the existing ordinary Windows voice backend. Goal 0011 remains a dormant placeholder only.

## PDF

**CORE CONTRACTS REPAIRED; NATIVE VISUAL STABILITY FUTURE**

Gate 3 native PDF visual stability remains future work and is not authorized during Goal 0012.

## Workflow status

**MACRO-GOAL / MULTI-ATTEMPT PROTOCOL ACTIVE**

Goal 0012 A2 is the single authorized goal in `docs/work/ready/`. Continue the existing `codex/0012-pretty-presentation-controls-and-inline-images` branch/report lineage. Goal 0010 remains queued; Goal 0011 remains deferred; PDF work is not authorized.
