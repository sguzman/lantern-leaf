# LanternLeaf Current Status

Updated: 2026-09-12 after Goal 0014 A1 director rejection for stale-base/contract drift.

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

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Goal 0012 is closed.

Accepted presentation/image behavior includes literal margins, presentation geometry invalidation, scrollable settings, working word/letter spacing, readable tables/TOCs, restrained blockquotes, explicit font fallback state, inline EPUB imagery, bounded off-render-thread pretty/image work, and durable canonical spoken highlighting/follow after geometry changes.

A6 implementation `6dcf9815ef87303caa3a3421bb8cc9e832f6b8ea` and A7 implementation `6fd324869ce6cca0c858c3ee32fabe627efba47b` are accepted production corrections. Windows baseline run `34723376577` passed native workspace and hosted renderer jobs. See `docs/work/reviews/0012-real-desktop-acceptance.md`.

## Goal 0013 — starter shell responsive panel containment

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Implementation `58ae9de` uses actual center width, a deterministic `1120px` two-column breakpoint, one-column fallback below it, width-bounded groups, wrapped action/control rows, and bounded long-path/URL presentation while preserving Calibre virtualization and off-render-thread work.

GitHub Windows baseline run `34725734155` passed. The focused physical Windows pass reported the revised starter shell looked good enough and found no remaining Goal 0013 blocker. See `docs/work/reviews/0013-real-desktop-acceptance.md`.

## Goal 0014 — reader presentation-geometry anchor stability

**A1 REJECTED — CORRECTION REQUIRED; GENERALIZED READY CONTRACT REMAINS AUTHORITATIVE**

The physical defect remains correctly framed as generic semantic viewport-anchor loss across presentation geometry changes: canonical highlighting is currently correct, while scroll position can jump to a different semantic location after reflow.

The authoritative generalized contract is `docs/work/ready/0014-reader-presentation-anchor-stability.md` on director `main`. It requires semantic block/sentence continuity across horizontal-margin/content-width reflow, text metrics, block/heading spacing, media sizing, compound geometry changes, and active-TTS follow precedence.

Attempt A1 produced a promising implementation direction: it captures the old `pretty_page` scroll position as block index plus within-block offset before invalidation, restores against new estimates, and gives pending canonical TTS follow precedence. Its focused tests and Windows CI passed.

A1 is not accepted because it started from stale director `main` `d6b2b35` instead of the already-authoritative generalized main `bca97d7b...`, used the obsolete media-only branch/goal document, and terminalized lifecycle artifacts for the superseded contract. The correction must start from current director `main`, preserve valid A1 work, and prove semantic continuity under materially rewrapping blocks rather than relying only on absolute within-block pixel arithmetic. See `docs/work/reviews/0014-a1-director-rejection.md`.

## Caliberate catalog covers / availability UX

**QUEUED AS GOAL 0010 — NOT ACTIVE**

The earlier `42866` open failure occurred while Caliberate was not running and remains withdrawn as evidence of a LanternLeaf materialization defect.

The remaining catalog issue is cover availability: main catalog entries can show black placeholders before open while Recents can show real covers after local materialization. Goal 0010 remains queued for first-class lazy catalog covers and clear provider-unavailable/loading/no-cover states.

## Windows Natural/HD voices

**DEFERRED BY USER — DO NOT WORK ON OR TEST UNTIL RE-AUTHORIZED**

Preserve the existing ordinary Windows voice backend. Goal 0011 remains a dormant placeholder only.

## PDF

**CORE CONTRACTS REPAIRED; NATIVE VISUAL STABILITY FUTURE**

Gate 3 native PDF visual stability remains future work and is not currently authorized while the near-term non-PDF correction sequence proceeds.

## Workflow status

**GOAL 0014 CORRECTION REQUIRED**

Goal 0013 is closed. Goal 0014 remains the single authorized macro-goal under its generalized ready contract; A1 is rejected and must be corrected before human QA. Goal 0010 remains queued; Goal 0011 remains deferred; PDF work remains future.
