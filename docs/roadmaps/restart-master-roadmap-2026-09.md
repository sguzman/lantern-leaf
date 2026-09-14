# LanternLeaf Restart Master Roadmap — September 2026

This is the active restart roadmap for the Windows/native-egui line. Completion is evidence-driven: accepted implementation plus the human-only runtime evidence a gate actually requires.

## Gate 0 — Trustworthy Windows baseline

**STATUS: COMPLETE**

Goals 0001–0005 established reproducible Windows CI, deterministic cache/test behavior, native-egui launch, repository-owned Windows QA, macro-goal notifications, and repaired bounded core contracts.

## Gate 1 — Backend-neutral TTS + Windows TTS

**STATUS: COMPLETE FOR THE WORKING WINDOWS PATH**

Accepted flow:

`canonical sentence -> backend synthesis -> prepared audio -> Rodio first-sample boundary -> canonical ReaderSession -> native UI`

Windows speaker playback, Play/Pause, ordinary installed voice switching, and first-sample-driven canonical cursor ownership are verified on the real Windows machine.

## Workflow UX — macro-goal notifications

**STATUS: IMPLEMENTED / MULTI-ATTEMPT HARDENED**

Repository goal identity is durable; Codex Goal sessions are disposable attempts. Correction attempts reuse the repository goal ID, re-arm the watcher, push before signaling terminal state, and return the shared checkout to `main`.

## Gate 2 — Non-PDF reader/TTS

**STATUS: COMPLETE FOR CURRENT TXT / MARKDOWN / HTML / EPUB READER PATH**

Goal 0006 established automated parity for TXT, Markdown, HTML, and EPUB. Goal 0008 strengthened native EPUB identity and proved fast responsive pretty rendering, audible Windows speech, accurate spoken-sentence highlight, and viewport follow on the real machine.

Goal 0009 closed sustained playback correctness, pretty/text-only synchronization, layered Windows voice configuration, Piper failure recovery, Close book, and Safe Quit behavior with automated plus real-desktop acceptance.

Human workflow remains `git pull -> .\qa.ps1`; no ordinary manual QA uses downloaded CI artifacts.

## Gate 2.5 — First-class Caliberate library service

**STATUS: COMPLETE — GOAL 0008 CLOSED**

Accepted relationship:

`Caliberate -> HTTP/JSON v1 at 127.0.0.1:8181 -> existing library browser -> materialized source -> normal reader/TTS pipeline`

Large-catalog behavior, materialization, native EPUB ingestion, responsive rendering, Windows speech, canonical first-sample boundaries, pretty highlighting, and viewport follow have automated plus real-desktop acceptance.

## Gate 2.6 — TTS playback polish + layered voice configuration

**STATUS: COMPLETE — GOAL 0009 CLOSED**

Accepted behavior includes sustained ordinary Windows playback without prior duplicate-line refill, synchronized pretty/text-only highlighting and follow, Zira app-default inheritance, per-book voice overrides, transactional Piper rejection/recovery, bounded diagnostics, Close book, and ordered Safe Quit.

## Gate 2.7 — Pretty presentation controls + inline images

**STATUS: COMPLETE — GOAL 0012 CLOSED**

Goal 0012 has automated, director, Windows CI, and real-desktop acceptance for the native pretty-reader presentation surface.

Accepted behavior includes literal margins, geometry invalidation, scrollable settings, functional word/letter spacing, readable tables/TOCs, restrained blockquotes, explicit font fallback state, inline EPUB imagery, bounded off-render-thread pretty/image work, and correct canonical spoken highlighting/follow.

## Near-term shell / reader cleanup

### Goal 0013 — starter shell responsive panel containment

**STATUS: COMPLETE**

The responsive starter-shell correction is accepted with worker/CI/director evidence plus a clean focused real-desktop pass. Two-column layout is width-gated, narrow windows fall back to one column, and long starter content stays contained.

### Goal 0014 — reader presentation-geometry anchor stability

**STATUS: COMPLETE**

A3 made semantic anchoring stable across multiple frames and rapid edit bursts by retaining the last settled semantic witness, avoiding recapture from unstable intermediate geometry, keeping bounded virtualization near the target neighborhood, reconciling with measured geometry, preserving TTS-follow precedence, and yielding to explicit user scrolling.

A lower-severity residual under severe letter-spacing/font-scaling/media edits is queued separately as Goal 0015 rather than keeping Goal 0014 open indefinitely.

### Goal 0010 — Caliberate catalog covers + provider availability UX

**STATUS: COMPLETE — GOAL CLOSED**

The explicit Caliberate cover contract and lazy catalog-cover path are integrated in both repositories. LanternLeaf carries `has_cover`, requests only visible/near-visible covers, bounds/coalesces in-flight work, keeps network/disk/decode work off the render thread, and presents intentional cover/provider states. Caliberate serves `/api/v1/books/{id}/cover` without requiring full-book materialization.

A7 removed the last global completion-order assumption and made ownership/freshness per book/request. Real-desktop closure verifies real covers before first open, continued lazy cover population while scrolling, fast warm representative EPUB opens, and preservation of the accepted EPUB TTS / pretty-reader / visual-settings path.

### Goal 0016 — starter library live-state continuity

**STATUS: COMPLETE — GOAL CLOSED**

The real 105,570-book Caliberate catalog now publishes progressively while the full provider walk continues off-thread. The starter surface exposes useful loaded/total progress, keeps partial rows usable, preserves live lazy-cover state through final reconciliation, and refreshes Recents during the same process after successful source persistence.

Real-desktop closure verified progressive rows, durable Recents across restart, immediate warm cached EPUB reopen, and preserved EPUB TTS/visual settings. See `docs/work/reviews/0016-a2-real-desktop-acceptance.md`.

### Goal 0015 — highlight viewport-band reflow polish

**STATUS: QUEUED — MINOR POLISH**

Under severe text-metric/media changes, an already-visible canonical highlight can drift farther than desired before ordinary auto-follow restores it. Future polish should preserve a temporary transaction-scoped viewport band without permanent highlight pinning or fighting user scroll.

### Goal 0017 — progressive cover backpressure/error-state polish

**STATUS: QUEUED — MINOR POLISH**

Cold full-catalog provider pressure can cause short cover requests to timeout, display `Cover fetch/decode failed`, and retry more aggressively than desirable before succeeding after pressure subsides. Add sane transient backoff/coalescing and readable theme-aware catalog/provider error presentation.

### Goal 0018 — Windows QA bootstrap idempotence

**STATUS: QUEUED — INFRASTRUCTURE**

Repeated `qa.ps1` execution in one PowerShell process must not accumulate Visual Studio environment state until `VsDevCmd.bat` fails with `The input line is too long`.

### Goal 0011 — Windows Natural/HD voices

**STATUS: DEFERRED BY USER**

Do not investigate or test until explicitly re-authorized. Preserve the ordinary working Windows voice backend.

## Gate 3 — Native PDF visual stability

**STATUS: READY — GOAL 0019**

Goal 0019 is the current substantive product gate.

The repository already has native Pdfium raster scaffolding, PDF viewport/eviction policy helpers, zoom policy types, and PDF diagnostics, but physical PDF reading is not accepted because the user-facing Reader surface does not yet provide a stable production native page canvas.

Goal 0019 must establish:

- page raster/render ownership on a bounded worker, never the egui/render thread;
- stale-safe source/page/zoom request identity and coalescing;
- zoom-aware native render/cache keys rather than stretching one low-resolution raster;
- bounded in-flight work, CPU image/texture residency, overscan, and eviction;
- correct current-page native visual presentation;
- previous/next navigation, zoom, resize, and scroll stability;
- recoverable native rendering errors;
- no WebView/pdf.js/Tauri production fallback.

PDF TTS/highlight/OCR synchronization is explicitly deferred to Gate 4. Authoritative contract: `docs/work/ready/0019-native-pdf-visual-stability.md`.

## Gate 4 — PDF text, TTS, and highlight synchronization

**STATUS: FUTURE CORE PRODUCT GATE**

After Gate 3: canonical sentence/page mapping, geometry confidence, overlays, first-sample audio-boundary identity, jump/follow behavior, OCR/degraded modes, and representative regression corpus.

## Gate 5 — Format expansion and ingestion cleanup

DOCX/Word, further HTML edge cases, common source/document boundaries, and broader format fixtures.

## Gate 6 — Ergonomics, performance, packaging

Startup/TTS latency, measured cold-open performance, broader UI cleanup, large-document ergonomics, optional Piper model/voice management, library/import polish, release packaging, and dependency cleanup justified by measured problems.

## Director rule

ChatGPT may combine related repair passes into one macro-goal when doing so removes needless human/Codex round trips without opening architectural ambiguity. Current verified state and active goal contracts outrank historical roadmap text.
