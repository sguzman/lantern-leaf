# LanternLeaf Priorities

These priorities are ordered by current verified evidence. Historical attempt detail belongs in work reports/reviews rather than this file.

## P0 — Trustworthy Windows/native baseline

**COMPLETE**

Reproducible Windows build/check/test, native-egui launch, repo-native `deps.ps1` / `qa.ps1`, Windows CI, separate renderer probe, and Scoop dependency convention are established.

## P1 — Backend-neutral TTS + Windows speech

**WORKING WINDOWS PATH COMPLETE**

Canonical reader/session semantics are backend-neutral; ordinary WinRT Windows voice enumeration/synthesis/playback works; first-sample boundaries drive canonical playback identity; physical Windows speech and interactive ordinary voice changes are verified.

## P2 — Non-PDF reader/TTS

**CURRENT TXT / MARKDOWN / HTML / EPUB PATH COMPLETE**

The accepted non-PDF path includes ingestion/parity, responsive native pretty rendering, spoken-sentence highlight/follow, ordinary Windows TTS, layered voice configuration, presentation controls, inline imagery, and stable reflow anchoring.

## P2.5 — First-class Caliberate reader integration

**COMPLETE — GOAL 0008 CLOSED**

Caliberate catalog/materialization/native EPUB/Windows TTS and synchronized pretty rendering have automated and real-desktop acceptance.

## P2.6 — Goal 0009: TTS playback polish + layered voice configuration

**COMPLETE — AUTOMATED + REAL-DESKTOP ACCEPTED**

Stable sustained playback, synchronized pretty/text-only highlighting, Zira app-default inheritance, per-book voice overrides, transactional Piper rejection/recovery, Close Book, and Safe Quit are accepted.

## P2.7 — Goal 0012: pretty presentation controls + inline images

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Goal 0012 is closed with accepted presentation controls, inline imagery, geometry invalidation, font fallback safety, bounded worker architecture, readable tables/TOCs, restrained blockquotes, and durable canonical TTS highlighting/follow.

## P2.8 — Goal 0013: starter shell responsive containment

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Implementation `58ae9de` uses actual center width, a deterministic `1120px` two-column breakpoint, one-column fallback below it, bounded starter groups, wrapped controls, and bounded long-content presentation while preserving Calibre virtualization and off-render-thread work.

## P2.9 — Goal 0014: reader presentation-geometry anchor stability

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

A3 removed the prior violent multi-frame reflow excursions while preserving canonical highlight ownership, TTS-follow precedence, bounded virtualization, and explicit user-scroll authority.

A minor residual under severe text-metric edits is split to Goal 0015 rather than keeping Goal 0014 open indefinitely.

## P2.10 — Goal 0010: Caliberate catalog covers + provider availability UX

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

The provider contract and lazy-cover architecture are integrated in both repositories. Caliberate serves `/api/v1/books/{id}/cover`; LanternLeaf carries `has_cover`, bounds/coalesces visible-row work, keeps request/disk/decode work off the GUI thread, and handles per-book completion ownership safely.

Real-desktop closure verified covers before first open, continued lazy cover population while scrolling, extremely fast representative EPUB opens, and preservation of EPUB TTS / pretty-reader / visual-settings behavior. See `docs/work/reviews/0010-a7-real-desktop-acceptance.md`.

Do not resurrect the withdrawn fake materialization defect from the test where Caliberate itself was not running.

## P2.11 — Goal 0016: starter library live-state continuity

**A2 DIRECTOR-ACCEPTED + INTEGRATED — REAL-DESKTOP SIGNOFF ACTIVE**

The progressive catalog and same-session Recents repair is integrated on `main`. Caliberate pages publish incrementally while the full provider walk continues off-thread; the starter UI exposes partial/loading progress and loaded-row search/sort scope; successful source persistence triggers a background Recents refresh without requiring restart.

A2 closes the director-found ownership defects: live lazy covers survive later catalog reconciliation, provider failure with stale fallback remains visibly failed/degraded, and catalog refreshes are single-worker/coalesced so overlapping full walks and stale durable-cache races are prevented.

Implementation `602e8952d077796ba478bd28e0b0cfe2d1e6bb51` passed Windows baseline run `34832563563` with native-workspace and hosted-renderer-probe success. The only remaining gate is focused physical Windows validation. See `docs/work/reviews/0016-a2-director-acceptance.md`.

## P2.12 — Goal 0015: highlight viewport-band reflow polish

**QUEUED — MINOR POLISH**

During severe letter-spacing/font-scale reflow, an already-visible canonical highlight can drift farther than desired before ordinary auto-follow restores it. Preserve a temporary transaction-scoped viewport band when practical without permanent highlight pinning or fighting user scroll. This remains below Goal 0016 physical closure.

## P2.13 — Goal 0011: Windows Natural/HD voice capability

**DEFERRED BY USER — DORMANT UNTIL EXPLICITLY RE-AUTHORIZED**

Do not investigate, implement, or test Natural/Narrator/HD voices for now. Preserve the existing ordinary Windows voice backend and current Zira/per-book override behavior.

## P3 — Native PDF visual stability

**FUTURE CORE PRODUCT GATE**

Current physical PDF viewing is not accepted. The next PDF phase must establish page raster/render ownership, texture/cache lifecycle, viewport scheduling, zoom/scroll stability, bounded memory/performance on representative PDFs, and visual behavior independent of TTS.

## P4 — PDF text/TTS/highlight synchronization

After P3: canonical sentence/page mapping, geometry confidence/overlays, jump/follow behavior, OCR/degraded modes, first-sample playback integration, and regression corpus.

## P5 — Format expansion / ingestion hardening

DOCX/Word, further HTML edge cases, shared source/document boundaries, and broader format fixtures.

## P6 — Ergonomics, latency, packaging

Startup/TTS latency, broader UI cleanup, large-document ergonomics, optional Piper model/voice management, library/import polish, release packaging, and measured dependency cleanup.
