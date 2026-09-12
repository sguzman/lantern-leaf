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
- pretty and text-only spoken-sentence highlight/follow are accepted on real EPUBs;
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

**A4 FONT REGISTRY / MISSING-FONT FALLBACK IS THE SINGLE AUTHORIZED GOAL**

A1–A3 already provide the desired Presentation UI, layered persistence/reset, EPUB image provenance and production fixtures, lazy bounded off-render-thread image decode, bounded texture/cache behavior, and explicit idle worker-completion repaint wakeups.

A3 automated evidence (`dccc5b9`, terminal `ae411ff`, Windows CI `34655821185`) was accepted for desktop QA, but the first physical Windows run crashed immediately after startup font discovery:

`FontFamily::Name("LanternLeafProportionalRegular") is not bound to any fonts`

The correction is narrow. A4 must:

1. replace the coarse `inserted_any` / `fonts_configured` assumption with exact production-owned alias availability/registry semantics;
2. ensure global egui TextStyles never reference an unbound LanternLeaf named family;
3. ensure pretty/per-book font selection never manufactures an unbound family alias;
4. provide deterministic fallback to a bound family or egui built-in family when requested regular/bold/monospace/optional fonts are absent;
5. preserve configured app/book family intent instead of silently rewriting config because a font is unavailable on one machine;
6. add controlled-font-availability tests, independent of CI font inventory, that force egui layout and catch the exact panic class;
7. keep runtime render frames free of font discovery/file I/O;
8. preserve all existing Goal 0012 presentation/image/async-wakeup behavior and Goal 0008/0009 TTS/canonical sync regressions.

No human QA until A4 is director-accepted.

## P2.8 — Goal 0010: Caliberate catalog covers + provider availability UX

**QUEUED — NOT ACTIVE**

Goal 0010 remains about provider-unavailable classification, first-class lazy catalog covers, bounded off-GUI-thread visible-row loading, covers before first open/materialization, Recents/local fallback, and intentional loading/no-cover/provider-error states.

## P2.9 — Goal 0011: Windows Natural/HD voice capability

**DEFERRED BY USER — DORMANT UNTIL EXPLICITLY RE-AUTHORIZED**

Do not investigate, implement, or test Natural/Narrator/HD voices for now. Preserve the existing ordinary Windows voice backend and current Zira/per-book override behavior.

## P3 — Native PDF visual stability

**FUTURE CORE PRODUCT GATE; NOT AUTHORIZED DURING GOAL 0012 A4**

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
