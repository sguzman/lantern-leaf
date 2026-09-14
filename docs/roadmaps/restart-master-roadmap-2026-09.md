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

Final real-desktop QA reports no more instant violent distant-area jerks; horizontal-margin changes behave beautifully; general presentation changes are substantially calmer; canonical highlight identity remains correct; and binding media max-width/max-height behavior is physically verified.

A lower-severity residual under severe letter-spacing/font-scaling edits is queued separately as Goal 0015 rather than keeping Goal 0014 open indefinitely.

### Goal 0010 — Caliberate catalog covers + provider availability UX

**STATUS: COMPLETE — GOAL CLOSED**

The explicit Caliberate cover contract and lazy catalog-cover path are integrated in both repositories. LanternLeaf carries `has_cover`, requests only visible/near-visible covers, bounds/coalesces in-flight work, keeps network/disk/decode work off the render thread, and presents intentional cover/provider states. Caliberate serves `/api/v1/books/{id}/cover` without requiring full-book materialization.

A6 added explicit book-identified terminal cover outcomes. A7 removed the last global completion-order assumption and made ownership/freshness per book/request.

Real-desktop closure verifies real covers before first open, continued lazy cover population while scrolling, extremely fast representative EPUB opens, and preservation of the accepted EPUB TTS / pretty-reader / visual-settings path. See `docs/work/reviews/0010-a7-real-desktop-acceptance.md`.

### Goal 0016 — starter library live-state continuity

**STATUS: A2 DIRECTOR-ACCEPTED + INTEGRATED — FOCUSED REAL-DESKTOP QA PENDING**

Progressive Caliberate catalog publication and same-session Recents refresh are now integrated on `main`.

The catalog worker publishes bounded provider pages while the full walk continues off-thread. The starter surface exposes partial/loading progress and loaded-row search/sort scope, then reconciles to the stable full catalog and durable cache only on successful completion. Successful SourceOpen persistence now triggers the existing background Recents listing path so a newly opened source can appear in the current process.

A2 hardens the implementation by preserving live lazy-cover state through later metadata/final reconciliation, retaining a visibly failed/degraded state when a progressive provider refresh fails despite stale fallback data, and coalescing catalog-load ownership so only one authoritative full walk can run and commit durable cache state at a time.

Implementation `602e8952d077796ba478bd28e0b0cfe2d1e6bb51` passed Windows baseline run `34832563563` with both native-workspace and hosted-renderer-probe success. Goal terminal commit `ca94477c6280ae1e5c47a5b32d02947019a428c4` is integrated to `main`.

One focused human pass remains: verify early rows/progress from a clean QA catalog cache, cover continuity through full completion, immediate same-session Recents after opening a representative EPUB, Recents durability after restart, and preservation of representative EPUB/TTS behavior. See `docs/work/reviews/0016-a2-director-acceptance.md`.

### Goal 0015 — highlight viewport-band reflow polish

**STATUS: QUEUED — MINOR POLISH**

Under severe text-metric changes, an already-visible canonical highlight can drift farther than desired before ordinary auto-follow restores it. Future polish should preserve a temporary transaction-scoped viewport band without permanent highlight pinning or fighting user scroll. It remains below Goal 0016 physical closure.

### Goal 0011 — Windows Natural/HD voices

**STATUS: DEFERRED BY USER**

Do not investigate or test until explicitly re-authorized. Preserve the ordinary working Windows voice backend.

## Gate 3 — Native PDF visual stability

**STATUS: FUTURE CORE PRODUCT GATE**

Current physical PDF behavior is not accepted as a working reader. After Goal 0016 physical closure, Gate 3 remains the substantive PDF product gate unless priorities are explicitly changed:

- page raster/rendering;
- texture/cache lifecycle;
- viewport scheduling;
- zoom/scroll stability;
- bounded memory/performance;
- reliable visual behavior independent of TTS.

## Gate 4 — PDF text, TTS, and highlight synchronization

After Gate 3: canonical sentence/page mapping, geometry confidence, overlays, first-sample audio-boundary identity, jump/follow behavior, OCR/degraded modes, and representative regression corpus.

## Gate 5 — Format expansion and ingestion cleanup

DOCX/Word, further HTML edge cases, common source/document boundaries, and broader format fixtures.

## Gate 6 — Ergonomics, performance, packaging

Startup/TTS latency, broader UI cleanup, large-document ergonomics, optional Piper model/voice management, library/import polish, release packaging, and dependency cleanup justified by measured problems.

## Director rule

ChatGPT may combine related repair passes into one macro-goal when doing so removes needless human/Codex round trips without opening architectural ambiguity. Current verified state and active goal contracts outrank historical roadmap text.
