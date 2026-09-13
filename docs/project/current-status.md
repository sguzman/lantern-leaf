# LanternLeaf Current Status

Updated: 2026-09-13 after Goal 0014 A2 real-desktop rejection for final closure and A3 correction promotion.

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

**A2 FUNCTIONALLY IMPROVED BUT REAL-DESKTOP CLOSURE REJECTED — A3 READY**

A2 is a substantial success and remains the production foundation. The reader now uses semantic viewport anchors rather than blindly retaining stale raw scroll Y across reflow. Canonical sentence identity plus normalized within-sentence position is preferred when available, same-block normalized position is the fallback, and a legitimate pending canonical TTS follow retains precedence.

The real-desktop pass verified that representative visual settings work, semantic continuity is far better than before, and—importantly—binding media max-width and max-height changes visibly resize inline images on the real Windows machine.

Goal 0014 is not closed because two residual defects remain:

- some presentation edits visibly jerk to a distant/unrelated area and then return as geometry settles;
- some edits occasionally settle on the wrong semantic area, including cases where the highlighted sentence had been visible. `Jump to highlight` still recovers, showing canonical identity is intact while viewport restoration can be wrong.

Current production behavior restores immediately from fresh estimated heights after invalidating measured heights. Rapid slider changes can therefore repeat capture/invalidate/estimated-restore before prior geometry has stabilized. A3 must verify this mechanism and replace one-frame restoration with a bounded multi-frame reflow transaction that retains the last stable semantic viewport witness across an edit burst, coalesces successive geometry changes, reconciles against actual measured new geometry, preserves active TTS follow priority, and yields to explicit user scrolling.

The A3 contract is `docs/work/ready/0014-reader-presentation-anchor-stability.md`. See `docs/work/reviews/0014-a2-real-desktop-rejection.md`.

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

**GOAL 0014 A3 READY**

Goal 0014 remains the single authorized macro-goal until reflow stabilization passes director and real-desktop review. Goal 0010 remains queued; Goal 0011 remains deferred; PDF work remains future.
