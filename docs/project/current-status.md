# LanternLeaf Current Status

Updated: 2026-09-14 after Goal 0010 real-desktop closure and Goal 0016 promotion.

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

Stable audible TTS, no unsolicited duplicate ordinary lines, correct pretty/text-only spoken-sentence highlight/follow, portable Zira inheritance, per-book voice persistence, transactional Piper rejection/recovery, Close Book, and Safe Quit are accepted.

## Goal 0012 — pretty presentation controls and inline images

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Accepted presentation/image behavior includes literal margins, presentation geometry invalidation, scrollable settings, working word/letter spacing, readable tables/TOCs, restrained blockquotes, explicit font fallback state, inline EPUB imagery, bounded off-render-thread pretty/image work, and durable canonical spoken highlighting/follow after geometry changes.

## Goal 0013 — starter shell responsive panel containment

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Implementation `58ae9de` uses actual center width, a deterministic `1120px` two-column breakpoint, one-column fallback below it, width-bounded groups, wrapped action/control rows, and bounded long-path/URL presentation while preserving Calibre virtualization and off-render-thread work.

## Goal 0014 — reader presentation-geometry anchor stability

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

A3 removed the high-severity multi-frame reflow instability by retaining one last-stable semantic viewport witness across an edit burst, avoiding recapture from unstable intermediate geometry, keeping bounded virtualization in the anchor neighborhood, reconciling after measured geometry is available, preserving pending TTS-follow precedence, and yielding to explicit user wheel/drag input.

Final physical Windows evidence: no more instant violent distant-area jerks; horizontal-margin changes behave beautifully; rapid presentation edits feel much calmer; canonical highlight ownership remains correct; and binding media max-width/max-height behavior is physically verified.

A lower-severity residual remains under severe cumulative text-metric edits such as aggressive letter spacing and font scaling. This is queued separately as Goal 0015 and does not keep Goal 0014 open.

## Goal 0010 — Caliberate catalog covers / provider availability UX

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Goal 0010 is closed.

Caliberate exposes the explicit `/api/v1/books/{id}/cover` provider contract; LanternLeaf propagates `has_cover`, lazily fetches covers for visible/near-visible rows, keeps network/disk/decode work off the UI thread, distinguishes intentional cover states, and never materializes an EPUB merely to obtain a thumbnail.

A6 introduced explicit book-identified `CalibreCoverCompleted` terminal outcomes. A7 replaced the remaining global completion watermark with per-book request ownership/freshness, so unrelated books may finish in arbitrary order, stale same-book completions cannot overwrite newer retries, and automatic plus manual thumbnail requests share the bounded/coalesced path.

Real Windows closure verified real covers before first open, lazy cover population during scroll, extremely fast representative EPUB opens, and preservation of EPUB TTS / pretty-reader / visual-settings behavior. The physical pass also exposed two separate starter-state continuity defects now owned by Goal 0016. See `docs/work/reviews/0010-a7-real-desktop-acceptance.md`.

The earlier `42866` open failure remains withdrawn as evidence of a LanternLeaf materialization defect because Caliberate was not running during that attempt.

## Goal 0016 — starter library live-state continuity

**READY NEXT**

Real-desktop Goal 0010 QA exposed two same-session starter-state defects:

- on a cold/incompatible catalog cache, LanternLeaf can look empty while it walks the real 105,570-book Caliberate catalog in the provider's supported 500-row pages;
- a successfully opened source is durably persisted as a recent, but the current in-memory Recents panel does not update until an explicit reload/restart.

Goal 0016 will make the large catalog useful progressively/cache-first while preserving truthful partial-loading state and stale-request ownership, and will refresh/update Recents after successful opens without inventing a second persistence model. All catalog/network/disk/decode work remains off the render thread. Contract: `docs/work/ready/0016-starter-library-live-state-continuity.md`.

## Goal 0015 — highlight viewport-band reflow polish

**QUEUED — MINOR POLISH**

Preserve a stronger temporary viewport band for an already-visible canonical highlight during severe text-metric reflow without permanent highlight pinning or fighting user scrolling. Goal 0016 takes precedence because it is ordinary library/startup usability exposed by current physical QA.

## Windows Natural/HD voices

**DEFERRED BY USER — DO NOT WORK ON OR TEST UNTIL RE-AUTHORIZED**

Preserve the existing ordinary Windows voice backend. Goal 0011 remains a dormant placeholder only.

## PDF

**CORE CONTRACTS EXIST; NATIVE VISUAL STABILITY AND PDF TTS/HIGHLIGHT REMAIN FUTURE GATES**

Current physical behavior is not accepted as a working PDF reader. Gate 3 is native PDF visual stability; Gate 4 is PDF text/TTS/highlight synchronization. They are not regressions from Goal 0010 and are not part of Goal 0016.

## Workflow status

**GOAL 0016 READY NEXT**

Goal 0010 is closed. Goal 0016 is the one ready implementation goal. Goal 0015 remains queued minor polish; Goal 0011 remains deferred; PDF work remains later unless the director/user reprioritizes after Goal 0016.
