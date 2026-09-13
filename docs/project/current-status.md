# LanternLeaf Current Status

Updated: 2026-09-12 after Goal 0013 A1 director acceptance for focused real-desktop signoff.

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

Goal 0012 is finally closed.

Accepted presentation/image behavior includes:

- literal horizontal margins with no hidden 720-px text-column cap;
- vertical margin as viewport/frame inset rather than scroll-document padding;
- presentation geometry-key invalidation of measured pretty-block heights;
- vertically scrollable settings/presentation body;
- real-egui word/letter-spacing behavior;
- readable table/TOC widths with horizontal overflow;
- restrained measured blockquote styling;
- visible optional-font availability/effective fallback state;
- inline EPUB/cover imagery with bounded off-render-thread preparation/decode;
- stateful geometry-change -> follow -> consume -> ordinary render-window behavior that preserves active spoken highlighting without permanent target forcing;
- canonical Goal 0008/0009 TTS ownership and synchronization.

A6 implementation `6dcf9815ef87303caa3a3421bb8cc9e832f6b8ea` and A7 implementation `6fd324869ce6cca0c858c3ee32fabe627efba47b` are the accepted production corrections. Windows baseline run `34723376577` passed native workspace and hosted renderer jobs.

Final physical Windows QA confirmed the previously failing visual controls now work, inline images remain present, and spoken-sentence highlighting remains continuously visible after follow/scroll. See `docs/work/reviews/0012-real-desktop-acceptance.md`.

One non-blocking residual observation is intentionally separate: live media max-width/max-height changes caused an unexpected scroll jump and the visible image-size effect could not be physically verified in that pass. Automated sizing evidence remains accepted; the interactive anchoring/observability issue is queued as Goal 0014 rather than reopening Goal 0012.

## Goal 0013 — starter shell responsive panel containment

**A1 DIRECTOR-ACCEPTED — FOCUSED REAL-DESKTOP SIGNOFF PENDING**

Implementation `58ae9de` replaces the overlap-prone unconditional starter layout with a responsive policy derived from actual center width: two columns at/above `1120px`, one-column fallback below it, width-bounded groups, wrapped action/control rows, and bounded long-path/URL presentation. Existing Calibre virtualization and off-render-thread work remain preserved.

The Goal 0013 worker branch terminalized with a duplicate stale `active/` lifecycle copy alongside `done/`; the director removed that bookkeeping artifact before integration. Production implementation and validation were unaffected.

GitHub Windows baseline run `34725734155` passed. See `docs/work/reports/0013.md` and `docs/work/reviews/0013-a1-director-acceptance.md`.

Because the original defect is visual/responsive and came from the real desktop, one focused physical pass remains: verify no Recents / Calibre / Browser Tabs overlap at the ordinary failing width, then resize narrower/wider to confirm stable one/two-column behavior, contained long rows, reachable diagnostics, and no horizontal application overflow.

## Goal 0014 — reader media-sizing anchor stability

**QUEUED — NOT ACTIVE**

During final Goal 0012 physical signoff, changing media max-width/max-height controls could throw the reader viewport to an unrelated location while the visible image appeared unchanged. Goal 0014 will distinguish genuinely non-binding media limits from stale/wired incorrectly media geometry and preserve a stable viewport/media anchor across live media-size changes.

Do not reopen Goal 0012 for this residual.

## Caliberate catalog covers / availability UX

**QUEUED AS GOAL 0010 — NOT ACTIVE**

The earlier `42866` open failure occurred while Caliberate was not running and remains withdrawn as evidence of a LanternLeaf materialization defect.

The remaining catalog issue is cover availability: main catalog entries can show black placeholders before open while Recents can show real covers after local materialization. Goal 0010 remains queued for first-class lazy catalog covers and clear provider-unavailable/loading/no-cover states.

## Windows Natural/HD voices

**DEFERRED BY USER — DO NOT WORK ON OR TEST UNTIL RE-AUTHORIZED**

Preserve the existing ordinary Windows voice backend. Goal 0011 remains a dormant placeholder only.

## PDF

**CORE CONTRACTS REPAIRED; NATIVE VISUAL STABILITY FUTURE**

Gate 3 native PDF visual stability remains future work and is not currently authorized while the starter shell / near-term non-PDF cleanup sequence proceeds.

## Workflow status

**HUMAN QA GATE ACTIVE**

Goal 0013 A1 is worker-terminal, CI-green, director-accepted, and ready for one focused desktop pass. Goal 0010 and Goal 0014 remain queued; Goal 0011 remains deferred; PDF work remains future.
