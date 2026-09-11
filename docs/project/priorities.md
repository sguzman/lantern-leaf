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

A3 implementation `8975cfcb286508e19ac1a983b3e49d35b83d38cf` / Windows CI `34558938955` is final accepted evidence. Both `A General History and Collection of Voyages` and `Buffalo Bill` now have correct text-only rendering, highlight, and follow, while pretty synchronization remains correct.

## P2.7 — Goal 0012: pretty presentation controls + inline images

**READY / ACTIVE PRIORITY — SINGLE AUTHORIZED GOAL**

Goal 0012 must:

1. restore a discoverable native-egui Presentation settings surface separate from TTS controls;
2. wire existing font family/weight/size, line spacing, margins, word spacing, letter spacing, highlight/pretty settings into actual native rendering rather than exposing no-op controls;
3. persist presentation changes through existing app-default -> per-book override layering;
4. repair embedded EPUB image provenance/reference resolution and render images inline at the correct source position;
5. keep image file I/O and decode work lazy, bounded, cached, and **off the GUI/render thread**;
6. add a real EPUB fixture with nested/relative image paths, PNG/JPEG assets, text around images, and canonical identity assertions;
7. preserve Goal 0008/0009 TTS, pretty/text-only synchronization, and large-document responsiveness.

No human QA until director review accepts Goal 0012 implementation.

## P2.8 — Goal 0010: Caliberate catalog covers + provider availability UX

**QUEUED — NOT ACTIVE**

Corrected evidence: the earlier `42866` open failure occurred while Caliberate itself was not running. Do not treat that incident as evidence of a LanternLeaf materialization/format bug.

Goal 0010 remains about:

- distinguishing provider unavailable from book/content failure;
- first-class lazy Caliberate catalog covers;
- bounded off-GUI-thread cover loading for visible rows;
- covers before first book open/materialization;
- preserving Recents/local cover fallback;
- replacing unexplained black rectangles with intentional loading/no-cover/provider-error states.

## P2.9 — Goal 0011: Windows Natural/HD voice capability

**DEFERRED BY USER — DORMANT UNTIL EXPLICITLY RE-AUTHORIZED**

Do not investigate, implement, or test Natural/Narrator/HD voices for now. Preserve the existing ordinary Windows voice backend and current Zira/per-book override behavior.

## P3 — Native PDF visual stability

**FUTURE CORE PRODUCT GATE; NOT AUTHORIZED DURING GOAL 0012**

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
