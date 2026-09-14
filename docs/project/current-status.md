# LanternLeaf Current Status

Updated: 2026-09-14 after Goal 0016 real-desktop closure and Goal 0019 promotion.

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

Caliberate exposes the explicit `/api/v1/books/{id}/cover` provider contract; LanternLeaf propagates `has_cover`, lazily fetches covers for visible/near-visible rows, keeps network/disk/decode work off the UI thread, distinguishes intentional cover states, and never materializes an EPUB merely to obtain a thumbnail.

A7 uses per-book request ownership/freshness so unrelated covers may complete out of order, stale same-book completions cannot overwrite newer retries, and automatic/manual requests share the bounded/coalesced path.

The earlier `42866` incident remains withdrawn as evidence of a LanternLeaf materialization defect because Caliberate was not running during that attempt.

## Goal 0016 — starter library live-state continuity

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Goal 0016 is closed. The real 105,570-book Caliberate library now becomes usable while the catalog is still loading: provider pages publish progressively, the starter shell shows truthful loaded/total progress, final reconciliation preserves live cover state, and successful source persistence refreshes Recents in the same process.

Physical Windows closure also verified durable Recents after restart, immediate warm cached EPUB reopen, and preservation of EPUB TTS / visual settings. The first open after `-ResetQaState` was slower because the isolated QA materialization/document cache was deliberately cold; this is retained as performance evidence rather than a demonstrated normal-path regression.

See `docs/work/reviews/0016-a2-real-desktop-acceptance.md`.

## Goal 0015 — highlight viewport-band reflow polish

**QUEUED — MINOR POLISH**

Preserve a stronger temporary viewport band for an already-visible canonical highlight during severe text-metric reflow without permanent highlight pinning or fighting user scrolling. Media-related severe reflow remains part of this bounded polish track.

## Goal 0017 — progressive cover backpressure/error-state polish

**QUEUED — MINOR LIBRARY POLISH**

During the full cold catalog walk, short cover timeouts can temporarily surface `Cover fetch/decode failed` and cause visible-row retry churn before covers later succeed. This goal owns provider-pressure-aware retry/backoff and readable theme-aware catalog error presentation, including removal of the hard-coded yellow-on-light-theme message.

## Goal 0018 — Windows QA bootstrap idempotence

**QUEUED — QA INFRASTRUCTURE**

Repeated `qa.ps1` invocations in one PowerShell process can eventually make `VsDevCmd.bat` fail with `The input line is too long`. This goal owns making the repository Windows environment bootstrap idempotent instead of requiring a fresh shell as recovery.

## Windows Natural/HD voices

**DEFERRED BY USER — DO NOT WORK ON OR TEST UNTIL RE-AUTHORIZED**

Preserve the existing ordinary Windows voice backend. Goal 0011 remains a dormant placeholder only.

## Goal 0019 / Gate 3 — native PDF visual stability

**READY NEXT — ONE AUTHORIZED MACRO-GOAL**

Current physical PDF behavior is not accepted as a working reader. The repository has useful native Pdfium, viewport, cache, zoom, and diagnostic scaffolding, but the user-facing Reader surface still does not provide a real accepted native page view.

Goal 0019 is now the one ready implementation goal. It must connect PDF page rastering to the native reader through a bounded off-render-thread worker, explicit stale-safe request ownership, real zoom-aware render keys, bounded texture/cache lifecycle, stable page navigation/resize/scroll behavior, and a user-facing native PDF canvas. It explicitly does not include PDF TTS/highlight synchronization.

Contract: `docs/work/ready/0019-native-pdf-visual-stability.md`.

## Gate 4 — PDF text/TTS/highlight synchronization

**FUTURE CORE PRODUCT GATE**

After Gate 3: canonical sentence/page mapping, geometry confidence/overlays, jump/follow behavior, OCR/degraded modes, first-sample playback integration, and representative regression corpus.

## Workflow status

**GOAL 0019 READY NEXT**

Goal 0016 is closed. Goal 0019 is the only authorized ready macro-goal. Goals 0015, 0017, and 0018 remain queued; Goal 0011 remains deferred. No human PDF QA is authorized until Goal 0019 implementation passes director source/CI review and is integrated to `main`.
