# LanternLeaf Priorities

These priorities are ordered by current verified evidence. Historical attempt detail belongs in work reports/reviews rather than this file.

## P0 — Trustworthy Windows/native baseline

**COMPLETE**

Reproducible Windows build/check/test, native egui launch, repo-native `deps.ps1` / `qa.ps1`, Windows CI, separate renderer probe, and Scoop dependency convention are established.

## P1 — Backend-neutral TTS + Windows speech

**WORKING WINDOWS PATH COMPLETE**

Canonical reader/session semantics are backend-neutral; Windows voice enumeration/synthesis/playback works; first-sample boundaries drive canonical playback identity; physical Windows speech and interactive voice changes are verified.

Full Piper model/catalog/downloader UX remains future work.

## P2 — Non-PDF reader/TTS

**PRETTY EPUB PATH ACCEPTED; GOAL 0009 A2.1 REGRESSION PROOF OPEN**

- TXT/Markdown/HTML/EPUB automated parity is established;
- native pretty rendering is bounded and responsive on the real large EPUB;
- sustained A1 real-desktop playback had no duplicate ordinary line reads;
- pretty spoken-sentence highlight and viewport follow remain accurate;
- A2 contains a plausible text-only transition/follow repair, but its new regression still tests projection arithmetic rather than the real production mode-switch/follow lifecycle.

## P2.5 — First-class Caliberate service

**COMPLETE — GOAL 0008 CLOSED**

Caliberate catalog/materialization/native EPUB/Windows TTS and synchronized pretty rendering have automated and real-desktop acceptance.

## P2.6 — Goal 0009: TTS playback polish + layered voice configuration

**A2 REJECTED BEFORE HUMAN QA — A2.1 REQUIRED**

A2 implementation: `e6bbc065462434950802818d0b4236464c244d5a`.

A2 terminal: `87ec9257948bbc8dff277c8a7d8c8b3d44ef3d31`.

A2 Windows CI: `34528119986` — green.

Preserve A2 production work:

- ordered Safe Quit persistence-terminal/native-close path;
- persistent confirmed Close book lifecycle;
- stale old-source playback filtering;
- bounded/resizable 240–460 px side panel with wrapped diagnostics;
- production-owned text-only canonical row projection and follow re-arm/page refresh.

A2.1 is narrowly about making the required evidence real and fixing anything those regressions expose:

1. production pretty->text-only transition regression with selected-row styling + AutoScrollState pending/consume across 48+ boundaries, Pause, and a page transition;
2. deterministic 300+ character diagnostic containment test against production panel/presentation policy;
3. close-book lifecycle test covering active TTS, confirmation-before-destruction, persistence success/failure, Starter return, and stale-event isolation after a new source opens;
4. Safe Quit persistence-terminal -> native-close handshake test, including the failure branch;
5. failed Piper -> immediate Windows Play proof in the **same ReaderSession**, with actual playback/boundary evidence and no source reopen;
6. preserve/re-run the existing no-repeat, pretty-sync, voice inheritance/persistence, Caliberate, workspace, Windows QA, Windows TTS probe, and renderer gates.

No human QA until A2.1 receives director acceptance. Do not authorize PDF implementation first.

## P3 — Native PDF visual stability

**NEXT CORE PRODUCT GATE AFTER GOAL 0009; NOT AUTHORIZED YET**

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
