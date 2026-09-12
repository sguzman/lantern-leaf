# LanternLeaf Current Status

Updated: 2026-09-12 after Goal 0012 A6 director review and A7 reopening.

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

**A6 REJECTED BEFORE HUMAN QA; A7 BOUNDED CORRECTION READY / SINGLE AUTHORIZED GOAL**

Physical Windows QA after A5 already verified:

- LanternLeaf starts on the machine/config that previously crashed;
- inline/cover EPUB images render in pretty view;
- the Presentation section exists;
- multiple presentation controls visibly work;
- canonical TTS identity/follow remains synchronized.

A6 implementation `6dcf9815ef87303caa3a3421bb8cc9e832f6b8ea` is cleanly based on director `main`, and Windows workflow `34721716717` passed native workspace and hosted renderer jobs.

A6 successfully adds/preserves:

- literal horizontal margin semantics with no hidden 720-px column cap;
- vertical margin as viewport/frame inset rather than scroll-document padding;
- presentation geometry-key invalidation of measured pretty-block heights;
- vertically scrollable settings/presentation panel body;
- real-egui word/letter-spacing behavior;
- readable table/TOC minimum widths with horizontal overflow;
- visible optional-font availability/effective fallback state;
- media width/height sizing evidence;
- inline images, async repaint wakeups, exact font fallback safety, and Goal 0008/0009 TTS behavior.

A6 is not director-accepted because the blockquote left rule still derives its height from pre-layout `ui.max_rect()` instead of the final measured quote block rectangle, leaving the original long-rule geometry failure class present. Its 64-boundary highlight test also does not exercise the required stateful geometry-A -> geometry-B -> follow -> consume -> subsequent normal-window lifecycle.

A7 is deliberately narrow: preserve A6, bind quote decoration to measured quote geometry, and add the missing post-follow stateful highlight proof. Production highlight/window logic should only change if that stronger proof exposes a real render-only defect.

See `docs/work/reviews/0012-a6-director-rejection.md` and the single ready Goal 0012 A7 contract.

No human QA is authorized until A7 is terminal, CI-green, and director-accepted.

## Starter shell containment

**QUEUED AS GOAL 0013 — NOT ACTIVE**

Physical QA showed Recents and Browser Tabs/adjacent starter groups visually overlapping. This is tracked separately as responsive starter-shell containment rather than mixed into Goal 0012.

## Caliberate catalog covers / availability UX

**QUEUED AS GOAL 0010 — NOT ACTIVE**

The earlier `42866` open failure occurred while Caliberate was not running and remains withdrawn as evidence of a LanternLeaf materialization defect.

The remaining catalog issue is cover availability: main catalog entries can show black placeholders before open while Recents can show real covers after local materialization. Goal 0010 remains queued for first-class lazy catalog covers and clear provider-unavailable/loading/no-cover states.

## Windows Natural/HD voices

**DEFERRED BY USER — DO NOT WORK ON OR TEST UNTIL RE-AUTHORIZED**

Preserve the existing ordinary Windows voice backend. Goal 0011 remains a dormant placeholder only.

## PDF

**CORE CONTRACTS REPAIRED; NATIVE VISUAL STABILITY FUTURE**

Gate 3 native PDF visual stability remains future work and is not authorized during Goal 0012 A7.

## Workflow status

**MACRO-GOAL / MULTI-ATTEMPT PROTOCOL ACTIVE**

Goal 0012 A7 is the single authorized goal in `docs/work/ready/`. Continue `codex/0012-pretty-presentation-controls-and-inline-images`, synchronize current `main`, preserve A6 `6dcf981`, re-arm the watcher, and append Attempt A7 to `docs/work/reports/0012.md`.

Goal 0010 and Goal 0013 remain queued; Goal 0011 remains deferred; PDF work remains future.
