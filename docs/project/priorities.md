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

**PRETTY EPUB ACCEPTED; A2.1 TEXT-ONLY/EXIT FIXES ACCEPTED FOR DESKTOP SIGNOFF**

- TXT/Markdown/HTML/EPUB automated parity is established;
- native pretty rendering is bounded and responsive on the real large EPUB;
- sustained A1 real-desktop playback had no duplicate ordinary line reads;
- pretty spoken-sentence highlight and viewport follow remain accurate;
- A2.1 now deterministically covers pretty->text-only mode transition, canonical selected-row/follow behavior across 48+ transitions/page changes, Pause retention, and return-to-pretty identity.

## P2.5 — First-class Caliberate service

**COMPLETE — GOAL 0008 CLOSED**

Caliberate catalog/materialization/native EPUB/Windows TTS and synchronized pretty rendering have automated and real-desktop acceptance.

## P2.6 — Goal 0009: TTS playback polish + layered voice configuration

**A2.1 ACCEPTED — FINAL FOCUSED REAL-DESKTOP SIGNOFF**

Accepted A2.1 implementation: `b5e348f06a1ff730d2363dc61bc4ac864d871f07`.

Accepted worker terminal: `66091555e41f03fe2fbce049c8d773024af55425`.

Authoritative Windows CI: `34532877674` — green.

Accepted automated behavior:

1. text-only selection and auto-follow share one canonical production projection and survive mode switch, 48+ cursor transitions, page transition, Pause, and return-to-pretty;
2. 300+ character diagnostics retain full text under the production 460 px panel maximum;
3. Close book is confirmation-first and persistence-gated before session destruction/Starter return;
4. Safe Quit is persistence-gated and arms one native viewport close only on successful terminal persistence;
5. stale old-source playback cannot mutate the newly active reader source;
6. unavailable Piper is transactionally rejected and the same ReaderSession can immediately use Windows Play through a first boundary/progress signal;
7. the A1 300-boundary no-repeat path, layered Zira/book voice behavior, and Goal 0008 synchronization remain green.

One real-desktop signoff remains. Do not authorize PDF implementation until it passes.

## P3 — Native PDF visual stability

**NEXT CORE PRODUCT GATE AFTER GOAL 0009 FINAL SIGNOFF; NOT AUTHORIZED YET**

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
