# LanternLeaf Current Status

Updated: 2026-09-14 after Goal 0016 A2 director acceptance and integration for focused real-desktop QA.

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

**A2 DIRECTOR-ACCEPTED + INTEGRATED — FOCUSED REAL-DESKTOP QA ACTIVE**

Goal 0016 now has the intended progressive catalog and same-session Recents architecture plus the A2 ownership correction.

Caliberate pages are published progressively into starter state while the full provider walk continues in the background. Partial/loading counts are explicit, search/sort truthfully identifies loaded-row scope while incomplete, stale reducer events are rejected, and full catalog/cache completion remains a background terminal step.

After successful `SourceOpen` persistence, LanternLeaf refreshes Recents through the existing background listing path so the current process can display the newly opened source without restart.

A2 additionally preserves live lazy-cover state through metadata/final catalog reconciliation, keeps mid-refresh provider failure visibly failed/degraded instead of converting it to ordinary success through stale-cache fallback, and coalesces catalog refresh ownership so only one authoritative full catalog worker can run at a time.

Implementation `602e8952d077796ba478bd28e0b0cfe2d1e6bb51` passed Windows baseline run `34832563563`, including both native-workspace and hosted-renderer-probe jobs. Goal terminal commit `ca94477c6280ae1e5c47a5b32d02947019a428c4` was fast-forward integrated to `main` before the director acceptance record.

One focused physical Windows pass remains: early progressive rows/progress on a clean QA cache, cover continuity through full completion, immediate same-session Recents after open, durable Recents after restart, and representative EPUB/TTS regression. See `docs/work/reviews/0016-a2-director-acceptance.md`.

## Goal 0015 — highlight viewport-band reflow polish

**QUEUED — MINOR POLISH**

Preserve a stronger temporary viewport band for an already-visible canonical highlight during severe text-metric reflow without permanent highlight pinning or fighting user scrolling. Goal 0016 physical closure takes precedence.

## Windows Natural/HD voices

**DEFERRED BY USER — DO NOT WORK ON OR TEST UNTIL RE-AUTHORIZED**

Preserve the existing ordinary Windows voice backend. Goal 0011 remains a dormant placeholder only.

## PDF

**CORE CONTRACTS EXIST; NATIVE VISUAL STABILITY AND PDF TTS/HIGHLIGHT REMAIN FUTURE GATES**

Current physical behavior is not accepted as a working PDF reader. Gate 3 is native PDF visual stability; Gate 4 is PDF text/TTS/highlight synchronization. They are not regressions from Goal 0010 and are not part of Goal 0016.

## Workflow status

**HUMAN QA GATE ACTIVE — GOAL 0016 A2**

Goal 0010 is closed. Goal 0016 A2 is director-accepted and integrated; no new Codex Goal is authorized right now. Goal 0015 remains queued minor polish; Goal 0011 remains deferred; PDF work remains later unless the director/user reprioritizes after Goal 0016 closure.
