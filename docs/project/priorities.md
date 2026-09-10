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

**PRETTY EPUB PATH ACCEPTED; TEXT-ONLY A2 CORRECTION OPEN**

- TXT/Markdown/HTML/EPUB automated parity is established;
- native pretty rendering is bounded and responsive on the real large EPUB;
- sustained A1 real-desktop playback had no duplicate ordinary line reads;
- pretty spoken-sentence highlight and viewport follow remain accurate;
- text-only production mode transition currently loses both visible highlight and auto-scroll and must be corrected before this gate closes.

## P2.5 — First-class Caliberate service

**COMPLETE — GOAL 0008 CLOSED**

Caliberate catalog/materialization/native EPUB/Windows TTS and synchronized pretty rendering have automated and real-desktop acceptance.

## P2.6 — Goal 0009: TTS playback polish + layered voice configuration

**A2 CORRECTION READY — TEXT-ONLY + EXIT/PANEL UX**

A1 implementation: `52ae85dad02f2e5588c14d33817abf0c5db69916`.

A1 Windows CI: `34517286850`.

A1 real-desktop wins to preserve:

- unsolicited duplicate line replay was not reproduced over sustained playback;
- pretty rendering/highlight/follow remain strong;
- a new book inherited Zira;
- an explicit Mark voice persisted across restart/reopen;
- unavailable Piper produced a useful failure instead of crashing.

A2 blockers:

1. repair actual pretty -> text-only production transition so visible canonical highlight and auto-scroll both work immediately and continue through subsequent boundaries;
2. bound the left TTS/settings panel so long errors/paths wrap rather than resizing the shell and user resizing remains usable;
3. provide a persistent, reliable Close book/Back to library lifecycle that stops TTS, persists, clears the reader, and returns to Starter without exiting;
4. make Safe Quit actually close the native application after ordered TTS cancellation and persistence completion; current egui SafeQuit handler is a no-op;
5. prove unavailable Piper -> Windows -> Play recovery in the same open session, while preserving transactional configuration behavior.

The same Goal 0009 branch/report lineage is reopened for A2. Do not authorize PDF implementation until A2 passes director review and focused real-desktop signoff.

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
