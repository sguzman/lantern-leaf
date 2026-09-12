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

**A5 DIRECTOR-ACCEPTED — REAL-DESKTOP SIGNOFF IS THE ACTIVE GATE**

The implementation now provides:

1. discoverable native Presentation controls separate from TTS;
2. live font, spacing, margin, highlight, heading/base/paragraph/block/media rendering controls;
3. app-default -> per-book persistence and presentation-only reset;
4. safe EPUB image provenance/reference resolution with production-chain fixtures;
5. lazy bounded off-render-thread pretty preparation and image decoding;
6. worker-completion repaint wakeups for pretty-build success and image decode success/failure while TTS is idle;
7. exact registered-font alias tracking and deterministic safe fallback for unavailable regular/bold/monospace/optional families;
8. controlled egui/epaint tests that force Body/Heading/Monospace and pretty `LayoutJob` layout under missing/partial font availability;
9. preserved Goal 0008/0009 TTS/canonical synchronization regressions.

Accepted automated evidence: A4 production fix `c92256fb429aea1d13707a811b0147e4337bbd05`; A5 layout-proof implementation `46d70b34d2a560373e831471f24b58d17b1fe8bc`; worker terminal `e2938b2cf543df237beb79f83e5159e4e598b4cd`; Windows CI `34718069167`.

One focused Windows desktop pass is required before Goal 0012 is finally closed. Startup on the previously crashing machine/config is the first gate; only then verify presentation persistence/reset, idle inline images, and brief TTS synchronization.

## P2.8 — Goal 0010: Caliberate catalog covers + provider availability UX

**QUEUED — NOT ACTIVE**

Goal 0010 remains about provider-unavailable classification, first-class lazy catalog covers, bounded off-GUI-thread visible-row loading, covers before first open/materialization, Recents/local fallback, and intentional loading/no-cover/provider-error states.

## P2.9 — Goal 0011: Windows Natural/HD voice capability

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
