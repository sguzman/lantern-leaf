# LanternLeaf Current Status

Updated: 2026-09-12 after Goal 0012 A5 physical presentation QA and A6 reopening.

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

**A5 STARTUP/FONT SAFETY ACCEPTED; PHYSICAL PRESENTATION SIGNOFF REJECTED; A6 READY / SINGLE AUTHORIZED GOAL**

Verified on the physical Windows machine after A5:

- LanternLeaf now starts on the same machine/config that previously crashed on an unbound font alias;
- inline/cover EPUB imagery visibly renders in pretty view;
- the expanded Presentation settings surface is present;
- font size, base font scale, paragraph spacing, block spacing, H1 scale, and H2 scale visibly work;
- TTS spoken identity remains synchronized and pretty follow/scroll still advances with speech.

The usable presentation pass exposed concrete layout defects:

- pretty view has a large implicit centered gutter because horizontal margin is conflated with a hidden 720-px max text width;
- horizontal margin can appear inert at wide viewport sizes;
- vertical margin is implemented as scroll-document padding rather than viewport inset;
- word/letter spacing did not visibly change text in physical QA despite value plumbing;
- the expanded left settings/presentation panel is not vertically scrollable and lower controls become unreachable;
- optional fonts safely fall back but the UI gives no availability/effective-fallback indication;
- long blockquotes use a visually dominant left rule;
- pretty tables/TOCs can collapse to near-character-width columns;
- spoken highlight now appears briefly then disappears while canonical TTS identity and follow/scroll remain correct;
- presentation geometry changes can leave measured pretty-block heights/prefix sums stale, which is a likely cause of the one-frame highlight symptom.

A6 is authorized to correct only these reader-presentation geometry/usability defects while preserving A3 async wakeups, A4/A5 font safety, images, persistence, and Goal 0008/0009 TTS semantics.

See `docs/work/reviews/0012-a5-real-desktop-rejection.md` and the single ready Goal 0012 A6 contract.

No human QA is authorized until A6 is terminal, CI-green, and director-accepted.

## Starter shell containment

**QUEUED AS GOAL 0013 — NOT ACTIVE**

The same physical session showed Recents and Browser Tabs/adjacent starter groups visually overlapping. This is tracked separately as responsive starter-shell containment rather than mixed into Goal 0012's reader/TTS correction.

## Caliberate catalog covers / availability UX

**QUEUED AS GOAL 0010 — NOT ACTIVE**

The earlier `42866` open failure occurred while Caliberate was not running and remains withdrawn as evidence of a LanternLeaf materialization defect.

The remaining catalog issue is cover availability: main catalog entries can show black placeholders before open while Recents can show real covers after local materialization. Goal 0010 remains queued for first-class lazy catalog covers and clear provider-unavailable/loading/no-cover states.

## Windows Natural/HD voices

**DEFERRED BY USER — DO NOT WORK ON OR TEST UNTIL RE-AUTHORIZED**

Preserve the existing ordinary Windows voice backend. Goal 0011 remains a dormant placeholder only.

## PDF

**CORE CONTRACTS REPAIRED; NATIVE VISUAL STABILITY FUTURE**

Gate 3 native PDF visual stability remains future work and is not authorized during Goal 0012 A6.

## Workflow status

**MACRO-GOAL / MULTI-ATTEMPT PROTOCOL ACTIVE**

Goal 0012 A6 is the single authorized goal in `docs/work/ready/`. Continue `codex/0012-pretty-presentation-controls-and-inline-images`, synchronize current `main` before implementation, re-arm the watcher, preserve A1–A5, and append Attempt A6 to `docs/work/reports/0012.md`.

Goal 0010 and Goal 0013 remain queued; Goal 0011 remains deferred; PDF work remains future.
