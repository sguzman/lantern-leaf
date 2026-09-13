# LanternLeaf Current Status

Updated: 2026-09-13 after Goal 0010 A5 director rejection and A6 correction promotion.

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

## Goal 0013 — starter shell responsive panel containment

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Implementation `58ae9de` uses actual center width, a deterministic `1120px` two-column breakpoint, one-column fallback below it, width-bounded groups, wrapped action/control rows, and bounded long-path/URL presentation while preserving Calibre virtualization and off-render-thread work.

## Goal 0014 — reader presentation-geometry anchor stability

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Goal 0014 is closed.

A2 established generalized semantic anchoring across presentation reflow. A3 fixed the remaining high-severity multi-frame instability by retaining one last-stable semantic viewport witness across an edit burst, avoiding recapture from unstable intermediate geometry, keeping bounded virtualization in the anchor neighborhood, reconciling after measured geometry is available, preserving pending TTS-follow precedence, and yielding to explicit user wheel/drag input.

Final physical Windows evidence: no more instant violent distant-area jerks; horizontal-margin changes behave beautifully; rapid presentation edits feel much calmer; canonical highlight ownership remains correct; and binding media max-width/max-height behavior is physically verified.

A lower-severity residual remains under severe cumulative text-metric edits such as aggressive letter spacing and font scaling: the visible highlight can drift farther than desired before normal auto-follow restores it. This is queued separately as Goal 0015 and does not keep Goal 0014 open. See `docs/work/reviews/0014-a3-real-desktop-acceptance.md`.

Implementation `430e9c1` and Windows baseline run `34768445726` are the accepted A3 implementation/CI evidence.

## Goal 0010 — Caliberate catalog covers / provider availability UX

**A5 REJECTED BEFORE HUMAN QA — A6 CORRECTION READY**

A5 established a promising foundation: explicit Caliberate cover endpoint support, `has_cover` propagation, bounded visible-row lazy scheduling, off-render-thread network/disk/decode work, and intentional placeholder text. The sibling Caliberate branch added a minimal `/api/v1/books/{id}/cover` sidecar route.

Director review found blocking completion-state defects before integration. Per-cover requests incorrectly use the global `CalibreLoad` boolean scope; a successful cached-books response does not clear that scope, and provider failure / endpoint 404 / decode failure can leave local cover IDs permanently pending as `Loading cover…`. The UI is also watching full-catalog `calibre_load_event` for thumbnail failure even though `EnsureCalibreThumbnail` does not emit that event.

A5 also started from stale LanternLeaf lifecycle docs and terminalized while required Windows CI run `34777773736` was still in progress. The sibling Caliberate tests cited in the report were run before the cover endpoint push rather than after it.

A6 must preserve the useful A5 architecture but add an explicit book-identified terminal cover outcome, concurrency-safe pending ownership, correct retry/unavailable/error states, current-main synchronization in both repositories, post-change Caliberate endpoint tests, and completed Windows CI before success signaling. See `docs/work/reviews/0010-a5-director-rejection.md` and the correction section in `docs/work/ready/0010-caliberate-catalog-reliability.md`.

The earlier `42866` open failure remains withdrawn as evidence of a LanternLeaf materialization defect because Caliberate was not running during that attempt.

## Goal 0015 — highlight viewport-band reflow polish

**QUEUED — MINOR POLISH**

Preserve a stronger temporary viewport band for an already-visible canonical highlight during severe text-metric reflow without inventing permanent highlight pinning or fighting user scrolling. Do not prioritize ahead of Goal 0010 unless new evidence makes it materially disruptive.

## Windows Natural/HD voices

**DEFERRED BY USER — DO NOT WORK ON OR TEST UNTIL RE-AUTHORIZED**

Preserve the existing ordinary Windows voice backend. Goal 0011 remains a dormant placeholder only.

## PDF

**CORE CONTRACTS REPAIRED; NATIVE VISUAL STABILITY FUTURE**

Gate 3 native PDF visual stability remains future work and is not currently authorized while the near-term non-PDF sequence proceeds.

## Workflow status

**GOAL 0010 A6 READY**

Goal 0014 is closed. Goal 0010 remains the single authorized macro-goal, now as an A6 correction. Goal 0015 is queued minor polish; Goal 0011 remains deferred; PDF work remains future.
