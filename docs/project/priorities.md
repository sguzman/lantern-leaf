# LanternLeaf Priorities

These priorities are ordered by current verified evidence. Historical attempt detail belongs in work reports/reviews rather than this file.

## P0 — Trustworthy Windows/native baseline

**COMPLETE**

Reproducible Windows build/check/test, native-egui launch, repo-native `deps.ps1` / `qa.ps1`, Windows CI, separate renderer probe, and Scoop dependency convention are established.

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

## P2.8 — Goal 0013: starter shell responsive containment

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Implementation `58ae9de` uses actual center width, a deterministic `1120px` two-column breakpoint, one-column fallback below it, bounded starter groups, wrapped controls, and bounded long-content presentation while preserving Calibre virtualization and off-render-thread work.

## P2.9 — Goal 0014: reader presentation-geometry anchor stability

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

A2 established generalized semantic anchoring. A3 removed the prior violent multi-frame reflow excursions by retaining one stable viewport witness across edit bursts, using measured geometry for final reconciliation, keeping estimates only as bounded neighborhood guidance, preserving TTS-follow precedence, and yielding to explicit user scrolling.

Real-desktop closure verifies calm horizontal-margin behavior, no more instant violent distant-area jerks, correct canonical highlight ownership, and physically working media max-width/max-height controls.

A minor residual under severe text-metric edits is split to Goal 0015 rather than keeping Goal 0014 open indefinitely.

## P2.10 — Goal 0010: Caliberate catalog covers + provider availability UX

**A6 REJECTED BEFORE HUMAN QA — A7 CORRECTION IS THE ACTIVE PRIORITY**

Preserve A6's explicit Caliberate cover contract, `has_cover` propagation, explicit book-identified terminal outcomes, bounded automatic visible-row scheduling, off-GUI-thread request/decode work, retry semantics, targeted sibling Caliberate tests, and passed Windows CI.

A7 must repair the remaining event-order race: concurrent cover effects run on independent worker threads, but the UI currently uses one global `last_calibre_cover_event_request_id` watermark. A higher-ID completion arriving first can cause a valid lower-ID completion for another book to be dropped, stranding that book in pending/loading state.

Use per-book request ownership/freshness or an equivalent concurrency-safe model. Different books must complete in arbitrary order; stale older completions for the same book must not overwrite newer retry state; only the currently owned request may clear/replace that book's pending state. The manual `Ensure thumbnail` action must use the same bounded/coalesced ownership path rather than bypassing the four-request bound.

Required tests must explicitly deliver two books' completions out of order and exercise a stale same-book completion after a newer retry. See `docs/work/reviews/0010-a6-director-rejection.md`.

Do not resurrect the withdrawn fake materialization defect from the test where Caliberate itself was not running.

## P2.11 — Goal 0015: highlight viewport-band reflow polish

**QUEUED — MINOR POLISH**

During severe letter-spacing/font-scale reflow, an already-visible canonical highlight can drift farther than desired before ordinary auto-follow restores it. This is solvable viewport-coordination polish, not a canonical highlight correctness failure. Preserve a temporary transaction-scoped viewport band when practical without permanent highlight pinning or fighting user scroll.

## P2.12 — Goal 0011: Windows Natural/HD voice capability

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
