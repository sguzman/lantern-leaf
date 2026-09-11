# LanternLeaf Current Status

Updated: 2026-09-10 after Goal 0009 final real-desktop acceptance and activation of Goal 0012 pretty presentation/media work.

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

Final desktop evidence after A3 establishes:

- TTS playback is stable and audible;
- ordinary playback no longer repeats completed lines;
- pretty view highlights and auto-follows the audible canonical sentence;
- text-only also renders, highlights, and auto-follows the audible canonical sentence on both the previously failing `A General History and Collection of Voyages` source and the previously working `Buffalo Bill` source;
- switching between pretty and text-only preserves synchronization;
- new/unoverridden Windows books inherit the portable Zira preference;
- explicit per-book Windows voice selection persists across reopen;
- unavailable Piper is transactionally rejected;
- bounded diagnostics, persistent Close book, persistence-gated Safe Quit, and stale-source filtering remain accepted.

A3 implementation `8975cfcb286508e19ac1a983b3e49d35b83d38cf` / worker terminal `7f5f3f77538ecd5fd0acca938f86402003224b2e` / Windows CI `34558938955` are final accepted evidence. Goal 0009 is closed.

## Goal 0012 — pretty presentation controls and inline images

**READY / SINGLE AUTHORIZED GOAL**

The latest real desktop pass identified two presentation defects outside Goal 0009:

- the native reader no longer exposes the detailed visual/presentation controls remembered from earlier builds;
- embedded EPUB images do not appear in pretty view.

The current config model still contains font family/weight, font size, line spacing, margins, word/letter spacing, highlight colors, `PrettyUiConfig`, and per-book presentation overrides. The current pretty model also contains image blocks and image references, so Goal 0012 will recover the native presentation UI and repair the complete inline-image provenance/decode/render path.

Goal 0012 must preserve Goal 0008/0009 TTS/highlight/follow behavior and large-document responsiveness. Disk I/O/image decode/heavy preparation must stay off the egui render thread.

## Caliberate catalog covers / availability UX

**QUEUED AS GOAL 0010 — NOT ACTIVE**

A prior desktop attempt appeared to show Caliberate book `42866` failing during materialization, but the user later clarified that Caliberate was not running at the time. That incident is withdrawn as evidence of a LanternLeaf materialization/format defect.

The remaining real catalog issue is cover availability: main catalog entries can appear as black placeholders before open, while Recents can display real covers after local materialization. Goal 0010 remains queued for first-class lazy catalog covers and clear provider-unavailable/loading/no-cover states.

## Windows Natural/HD voices

**DEFERRED BY USER — DO NOT WORK ON OR TEST UNTIL RE-AUTHORIZED**

Additional Windows Natural/Narrator voices such as Aria, Guy, and Jenny are being handled in another context. Preserve the existing ordinary Windows voice backend and ignore Natural/Narrator/HD capability work until the user explicitly reopens that surface. Goal 0011 remains a dormant placeholder only.

## PDF

**CORE CONTRACTS REPAIRED; NATIVE VISUAL STABILITY FUTURE**

Gate 3 native PDF visual stability remains future work. It is not the active goal while Goal 0012 is authorized.

## Workflow status

**MACRO-GOAL / MULTI-ATTEMPT PROTOCOL ACTIVE**

Goal 0012 is the single authorized goal in `docs/work/ready/`. Goal 0010 remains queued. Goal 0011 is deferred by user. No PDF implementation is authorized during Goal 0012.
