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

**A7 BOUNDED BLOCKQUOTE / POST-FOLLOW PROOF IS THE SINGLE AUTHORIZED GOAL**

A1–A5 establish Presentation controls, layered persistence/reset, inline EPUB images, bounded off-render-thread workers, async completion wakeups, exact font alias safety, and real egui missing-font layout proof.

A6 implementation `6dcf9815ef87303caa3a3421bb8cc9e832f6b8ea` adds substantial presentation fixes and is to be preserved:

1. literal horizontal margins with no hidden 720-px column cap;
2. vertical viewport insets rather than scroll-document padding;
3. presentation geometry invalidation of measured pretty-block heights;
4. scrollable settings/presentation side panel;
5. real-egui word/letter-spacing behavior;
6. readable table/TOC minimum widths plus horizontal overflow;
7. visible optional-font availability/effective fallback state;
8. media sizing evidence while preserving inline image behavior.

A6 Windows workflow `34721716717` is green, but director acceptance is blocked by two narrow issues:

- the quote rule still uses pre-layout `ui.max_rect()` height rather than final measured quote-block geometry, so the original long-rule failure class remains;
- the added 64-boundary highlight test does not exercise geometry A -> B invalidation plus follow request -> consume -> subsequent normal render-window selection.

A7 must preserve A6, bind quote decoration to final measured quote geometry, and add the missing stateful post-follow proof. Change production highlight/window behavior only if that stronger regression exposes a real defect. No human QA until A7 is director-accepted.

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

**FUTURE CORE PRODUCT GATE; NOT AUTHORIZED DURING GOAL 0012 A7**

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
