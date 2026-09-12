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

**A7 DIRECTOR-ACCEPTED — REAL-DESKTOP SIGNOFF IS THE ACTIVE GATE**

Goal 0012 now has automated/director acceptance for:

1. discoverable Presentation controls with app-default -> per-book persistence/reset;
2. inline EPUB imagery with safe provenance/path normalization;
3. bounded off-render-thread pretty/image workers and completion repaint wakeups;
4. exact font alias safety and real egui missing-font layout fallback;
5. literal horizontal margins with no hidden 720-px cap;
6. vertical viewport insets instead of scroll-document padding;
7. presentation-geometry invalidation of measured virtualization heights;
8. scrollable settings/presentation panel body;
9. real-egui word/letter spacing;
10. readable TOC/table minimum widths with horizontal overflow;
11. restrained measured blockquote rule geometry;
12. visible requested/effective font fallback state;
13. media width/height sizing evidence;
14. stateful geometry-change -> follow -> consume -> ordinary render-window coverage across 64 canonical boundaries without changing canonical playback ownership.

A7 implementation `6fd324869ce6cca0c858c3ee32fabe627efba47b`, terminal `762452ada592b7b906b0537319fd4b3060ad1c34`, and Windows baseline `34723376577` are accepted automated evidence.

One focused Windows desktop pass is required before Goal 0012 finally closes. No new Codex macro-goal is authorized until that pass is reviewed.

## P2.8 — Goal 0010: Caliberate catalog covers + provider availability UX

**QUEUED — NOT ACTIVE**

Goal 0010 remains about provider-unavailable classification, first-class lazy catalog covers, bounded off-GUI-thread visible-row loading, covers before first open/materialization, Recents/local fallback, and intentional loading/no-cover/provider-error states.

## P2.9 — Goal 0013: starter shell responsive containment

**QUEUED — NOT ACTIVE**

Physical QA showed Recents and Browser Tabs/adjacent starter groups bleeding across their allocated columns. Goal 0013 will make starter columns responsive/contained, wrap long rows, and stack to one column when two readable columns do not fit. It remains separate from Goal 0012 reader presentation semantics.

## P2.10 — Goal 0011: Windows Natural/HD voice capability

**DEFERRED BY USER — DORMANT UNTIL EXPLICITLY RE-AUTHORIZED**

Do not investigate, implement, or test Natural/Narrator/HD voices for now. Preserve the existing ordinary Windows voice backend and current Zira/per-book override behavior.

## P3 — Native PDF visual stability

**FUTURE CORE PRODUCT GATE; NOT AUTHORIZED WHILE GOAL 0012 AWAITS SIGNOFF**

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
