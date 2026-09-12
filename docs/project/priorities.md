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

**A6 REAL-DESKTOP PRESENTATION GEOMETRY / USABILITY CORRECTION IS THE SINGLE AUTHORIZED GOAL**

A1–A5 already establish:

- discoverable Presentation controls;
- app-default -> per-book persistence/reset;
- EPUB inline images and safe provenance/path handling;
- bounded off-render-thread pretty/image workers;
- worker-completion repaint wakeups;
- exact font alias registry and safe missing-font fallback;
- real egui/epaint controlled-font layout safety.

Physical Windows QA after A5 confirms startup and inline imagery now work, but final presentation signoff is blocked by concrete layout defects. A6 must:

1. remove the hidden 720-px centered text-column behavior from horizontal-margin semantics;
2. make horizontal margin a literal monotonic inset with a useful UI range;
3. make vertical margin a viewport inset rather than scroll-document padding;
4. invalidate/version pretty block-height/prefix measurements when presentation geometry changes;
5. keep the currently spoken canonical sentence visibly highlighted after one-shot follow is consumed, including 48+ boundary regression coverage under changed geometry;
6. make the expanded settings/presentation side panel vertically scrollable and bounded;
7. prove word/letter spacing through real egui layout, correcting the production path if needed;
8. give pretty tables/TOCs readable minimum widths and horizontal overflow rather than character-level collapse;
9. preserve blockquote semantics with a restrained bounded visual treatment;
10. communicate unavailable/effective font fallback in the UI without changing persisted intent;
11. prove media max width/height controls;
12. preserve inline images, A3 wakeups, A4/A5 font safety, and Goal 0008/0009 TTS behavior.

No human QA until A6 is director-accepted.

## P2.8 — Goal 0010: Caliberate catalog covers + provider availability UX

**QUEUED — NOT ACTIVE**

Goal 0010 remains about provider-unavailable classification, first-class lazy catalog covers, bounded off-GUI-thread visible-row loading, covers before first open/materialization, Recents/local fallback, and intentional loading/no-cover/provider-error states.

## P2.9 — Goal 0013: starter shell responsive containment

**QUEUED — NOT ACTIVE**

Physical QA showed Recents and Browser Tabs/adjacent starter groups bleeding across their allocated columns. Goal 0013 will make starter columns responsive/contained, wrap long rows, and stack to one column when two readable columns do not fit. It is intentionally separate from Goal 0012 reader/TTS presentation semantics.

## P2.10 — Goal 0011: Windows Natural/HD voice capability

**DEFERRED BY USER — DORMANT UNTIL EXPLICITLY RE-AUTHORIZED**

Do not investigate, implement, or test Natural/Narrator/HD voices for now. Preserve the existing ordinary Windows voice backend and current Zira/per-book override behavior.

## P3 — Native PDF visual stability

**FUTURE CORE PRODUCT GATE; NOT AUTHORIZED DURING GOAL 0012 A6**

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
