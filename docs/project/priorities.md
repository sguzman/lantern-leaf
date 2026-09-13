# LanternLeaf Priorities

These priorities are ordered by current verified evidence. Historical attempt detail belongs in work reports/reviews rather than this file.

## P0 — Trustworthy Windows/native baseline

**COMPLETE**

Reproducible Windows build/check/test, native egui launch, repo-native `deps.ps1` / `qa.ps1`, Windows CI, separate renderer probe, and Scoop dependency convention are established.

## P1 — Backend-neutral TTS + Windows speech

**WORKING WINDOWS PATH COMPLETE**

Canonical reader/session semantics are backend-neutral; ordinary WinRT Windows voice enumeration/synthesis/playback works; first-sample boundaries drive canonical playback identity; physical Windows speech and interactive ordinary voice changes are verified.

## P2 — Non-PDF reader/TTS

**GOALS 0008 + 0009 COMPLETE**

- TXT/Markdown/HTML/EPUB ingestion/parity remains covered;
- native pretty rendering is bounded and responsive;
- pretty and text-only spoken-sentence highlight/follow are accepted on real EPUBs through Goal 0009;
- ordinary Windows TTS no longer shows the prior duplicate-line refill bug;
- app-level Zira preference and per-book voice overrides work;
- TTS/audio normalization no longer owns text-only document presentation.

## P2.5 — First-class Caliberate reader integration

**COMPLETE — GOAL 0008 CLOSED**

Caliberate catalog/materialization/native EPUB/Windows TTS and synchronized pretty rendering have automated and real-desktop acceptance.

## P2.6 — Goal 0009: TTS playback polish + layered voice configuration

**COMPLETE — AUTOMATED + REAL-DESKTOP ACCEPTED**

A3 implementation `8975cfcb286508e19ac1a983b3e49d35b83d38cf` / Windows CI `34558938955` is final accepted evidence.

## P2.7 — Goal 0012: pretty presentation controls + inline images

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Goal 0012 is closed with accepted presentation controls, inline imagery, geometry invalidation, font fallback safety, bounded worker architecture, readable tables/TOCs, restrained blockquotes, and durable canonical TTS highlighting/follow.

The final physical passes distinguish canonical highlight correctness from viewport-anchor continuity. Goal 0012's highlight architecture remains accepted; Goal 0014 owns the separately isolated semantic-anchor/reflow-stability problem.

## P2.8 — Goal 0013: starter shell responsive containment

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Implementation `58ae9de` uses actual center width, a deterministic `1120px` two-column breakpoint, one-column fallback below it, bounded starter groups, wrapped controls, and bounded long-content presentation while preserving Calibre virtualization and off-render-thread work.

Worker validation and GitHub Windows baseline run `34725734155` passed. The focused physical pass reported the starter shell looked good enough with no remaining containment blocker.

## P2.9 — Goal 0014: reader presentation-geometry anchor stability

**A3 READY — NEXT AUTHORIZED CODEX CORRECTION**

A2 materially improved semantic viewport preservation and is the accepted implementation foundation. Canonical sentence identity plus normalized within-sentence position is preferred when available; normalized same-block position is the fallback; a genuine pending TTS follow request has precedence. Binding media max-width/max-height behavior is now physically verified on the real Windows machine.

A2 did not pass final real-desktop closure because live geometry edits can still expose an unrelated document area transiently before returning, and can occasionally settle on the wrong semantic area. This directly violates Goal 0014's bounded visual displacement / semantic-location contract.

A3 must treat geometry editing as a multi-frame reflow transaction rather than repeated independent estimated restores:

- capture and retain the last stable semantic viewport witness across a burst of slider/control changes;
- do not recapture from unstable intermediate frames;
- use estimates only for bounded virtualization / locating the target neighborhood, then reconcile against actual measured new geometry;
- preserve an actually visible canonical sentence/segment witness when available, including the visible highlight as a one-shot edit anchor without inventing permanent auto-follow;
- keep pending TTS follow precedence;
- yield to explicit user wheel/drag input;
- prevent both intermediate unrelated-area flashes and wrong final semantic anchoring.

The A3 contract is `docs/work/ready/0014-reader-presentation-anchor-stability.md`.

## P2.10 — Goal 0010: Caliberate catalog covers + provider availability UX

**QUEUED — NOT ACTIVE**

Goal 0010 remains about provider-unavailable classification, first-class lazy catalog covers, bounded off-GUI-thread visible-row loading, covers before first open/materialization, Recents/local fallback, and intentional loading/no-cover/provider-error states.

## P2.11 — Goal 0011: Windows Natural/HD voice capability

**DEFERRED BY USER — DORMANT UNTIL EXPLICITLY RE-AUTHORIZED**

Do not investigate, implement, or test Natural/Narrator/HD voices for now. Preserve the existing ordinary Windows voice backend and current Zira/per-book override behavior.

## P3 — Native PDF visual stability

**FUTURE CORE PRODUCT GATE; NOT CURRENTLY AUTHORIZED**

- page raster/render ownership;
- texture/cache lifecycle;
- viewport scheduling;
- zoom/scroll stability;
- bounded memory/performance on representative PDFs;
- visual behavior independent of TTS.

## P4 — PDF text/TTS/highlight synchronization

After P3: canonical sentence/page mapping, geometry confidence/overlays, jump/follow behavior, OCR/degraded modes, first-sample playback integration, and regression corpus.

## P5 — Format expansion / ingestion hardening

DOCX/Word, further HTML edge cases, shared source/document boundaries, and broader format fixtures.

## P6 — Ergonomics, latency, packaging

Startup/TTS latency, broader UI cleanup, large-document ergonomics, optional Piper model/voice management, library/import polish, release packaging, and measured dependency cleanup.
