# LanternLeaf Current Status

Updated: 2026-09-13 after Goal 0010 A7 director acceptance and integration for focused real-desktop QA.

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

**A7 DIRECTOR-ACCEPTED + INTEGRATED — FOCUSED REAL-DESKTOP QA PENDING**

Goal 0010 now has the intended provider/cover architecture plus the A7 concurrency correction. Caliberate exposes the explicit `/api/v1/books/{id}/cover` provider contract; LanternLeaf propagates `has_cover`, loads covers lazily only for visible/near-visible rows, keeps network/disk/decode work off the UI thread, distinguishes loading/no-cover/provider-unavailable/fetch-error states, and never materializes an EPUB merely to obtain a thumbnail.

A6 introduced explicit book-identified `CalibreCoverCompleted` terminal outcomes. A7 replaced the remaining global completion watermark with per-book request ownership/freshness. Different books may now finish in arbitrary order, stale older same-book completions cannot overwrite newer retry state, and automatic plus manual `Ensure thumbnail` paths share the same four-request bounded/coalesced ownership path.

LanternLeaf implementation `54e22c172745171b084221a6f80465e3f6ffe77d` passed Windows baseline run `34793890003`, including both native-workspace and hosted-renderer-probe jobs. The Goal 0010 terminal branch was fast-forward integrated to LanternLeaf `main` at `fb02dde145e881a04c6052fba0baa2aff6e579db` before the director acceptance record. The sibling Caliberate endpoint/test lineage was fast-forward integrated to Caliberate `main` at `3799ccac03ce05404700efbbf33e489aa965f757`.

One focused physical Windows pass remains: covers before first open, no permanent `Loading cover…` rows, intentional no-cover state, provider-down classification and recovery after restart, bounded manual ensure behavior, and representative existing Caliberate open/TTS regression. See `docs/work/reviews/0010-a7-director-acceptance.md`.

The earlier `42866` open failure remains withdrawn as evidence of a LanternLeaf materialization defect because Caliberate was not running during that attempt.

## Goal 0015 — highlight viewport-band reflow polish

**QUEUED — MINOR POLISH**

Preserve a stronger temporary viewport band for an already-visible canonical highlight during severe text-metric reflow without inventing permanent highlight pinning or fighting user scrolling. Do not prioritize ahead of Goal 0010 closure unless new evidence makes it materially disruptive.

## Windows Natural/HD voices

**DEFERRED BY USER — DO NOT WORK ON OR TEST UNTIL RE-AUTHORIZED**

Preserve the existing ordinary Windows voice backend. Goal 0011 remains a dormant placeholder only.

## PDF

**CORE CONTRACTS REPAIRED; NATIVE VISUAL STABILITY FUTURE**

Gate 3 native PDF visual stability remains future work and is not currently authorized while the near-term non-PDF sequence proceeds.

## Workflow status

**HUMAN QA GATE ACTIVE — GOAL 0010 A7**

Goal 0010 A7 is director-accepted and integrated in both LanternLeaf and Caliberate. No new Codex Goal is authorized right now. Goal 0015 is queued minor polish; Goal 0011 remains deferred; PDF work remains future.
