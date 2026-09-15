# LanternLeaf Current Status

Updated: 2026-09-15 after Goal 0022 A9 real-desktop rejection and A10 reopening.

This file contains current verified/bounded state. Detailed attempt history lives in `docs/work/reports/`, `docs/work/reviews/`, and the durable project-memory files under `docs/project/`.

## Workspace / architecture

**VERIFIED / AUTHORITATIVE**

- Native Rust + `eframe`/`egui` is the production desktop architecture.
- Rust owns canonical document/session/playback state.
- Tauri/React/WebView is historical reference only.
- Windows human workflow is repo-native: `git pull -> .\qa.ps1`.
- Scoop is the active Windows CLI dependency convention.
- Heavy/blocking work must never run on the egui/render thread.
- Chat is coordination; Git is durable project memory.

## Gate 0 — Windows baseline

**COMPLETE**

Hosted Windows CI covers MSVC/Pandoc setup, workspace check/build/test, repo-native QA preparation, Windows TTS synthesis/decode, and a separate hosted renderer probe.

## Gate 1 — backend-neutral TTS + Windows TTS

**COMPLETE FOR THE WORKING WINDOWS PATH**

Accepted runtime shape:

`canonical display sentence -> backend synthesis -> prepared audio -> Rodio first-sample boundary -> canonical ReaderSession cursor -> native UI projection`

Real Windows evidence proves audible ordinary Windows speech, Play/Pause, ordinary installed voice switching, and first-sample sentence identity.

Windows Natural/HD voices remain **DEFERRED BY USER** and must not be investigated/tested until re-authorized.

## Non-PDF reader / TTS baseline

**COMPLETE, WITH ONE CURRENT GOAL-0022 REGRESSION INVESTIGATION**

The accepted EPUB/TXT/Markdown/HTML path includes native Pretty rendering, Text-only, ordinary Windows TTS, spoken-sentence highlight/follow, layered voice configuration, presentation controls, inline imagery, and semantic reflow anchoring.

A7 introduced a severe EPUB Pretty/Text-only race. A8/A9 fixed the easy reproduction with idempotent `SetTextOnly { enabled }`, and a new EPUB now survives stress toggling. However, the EPUB previously damaged under A7 later reopened in a truncated-looking state until TTS/Next activity repaired it. A10 owns diagnosis of that stale/durable presentation-state path before Goal 0022 can close.

## Goal 0008 — Caliberate first-class reader integration

**COMPLETE — AUTOMATED + REAL-DESKTOP ACCEPTED**

Caliberate catalog/materialization/native EPUB/Windows TTS integration is accepted. Some provider-backed books require Caliberate running; provider unavailable is distinct from source corruption.

## Goal 0009 — TTS playback polish / layered voices

**COMPLETE — AUTOMATED + REAL-DESKTOP ACCEPTED**

Stable ordinary Windows TTS, Zira inheritance, per-book voice persistence, transactional Piper rejection/recovery, Close Book, Safe Quit, and non-PDF pretty/text-only spoken synchronization are accepted.

## Goal 0012 — pretty presentation controls / inline images

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

The severe hidden-width/gutter, settings clipping, table/TOC, font fallback, image sizing, spacing, and stale highlight issues are accepted as repaired.

## Goal 0013 — starter shell responsive containment

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Actual center width, deterministic two-column breakpoint, one-column fallback, bounded groups, wrapping, and long-string containment are accepted.

## Goal 0014 — reader presentation anchor stability

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

The violent multi-frame reflow jump is fixed. Minor severe-metric highlight-band polish remains Goal 0015.

## Goal 0010 — Caliberate covers / provider UX

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Catalog covers are lazy/visible-near-visible, bounded/coalesced, and do not require book materialization merely to obtain thumbnails.

## Goal 0016 — starter library live-state continuity

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

The real ~105,570-book library becomes usable progressively with loaded/total state, live cover reconciliation, same-session Recents refresh, and durable Recents.

## Queued non-core work

### Goal 0015 — highlight viewport-band reflow polish

**QUEUED — MINOR POLISH**

Preserve a stronger temporary viewport band for an already-visible canonical highlight during severe text-metric reflow.

### Goal 0017 — progressive cover backpressure / cached hydration

**QUEUED — CONFIRMED LIBRARY SCALING POLISH**

Warm runs exposed roughly-O(total-books) thumbnail hydration/cache rewrite behavior plus transient cover retry/logging issues.

### Goal 0018 — Windows QA bootstrap idempotence

**QUEUED — QA INFRASTRUCTURE**

Repeated `qa.ps1` in one PowerShell can eventually produce `The input line is too long`; use a fresh PowerShell for physical QA until fixed.

### Goal 0021 — fit-to-manual zoom transition polish

**QUEUED — MINOR PDF UX POLISH**

After Fit Width/Fit Page, `+/-` resumes the old manual zoom ladder instead of stepping from effective fit percentage.

### Goal 0024 — Caliberate materialized source identity in Recents

**QUEUED — REAL-DESKTOP UX DEFECT**

Materialized provider PDFs are correctly cached as source files under paths such as `calibre-downloads/caliberate/725-<hash>.pdf`, but Recents currently exposes the hash-like materialized file stem as the title and loses the known cover/provider identity. Preserve durable Caliberate provenance/title/cover without O(total-catalog) lookup or rematerialization.

## Goal 0019 / Gate 3 — native PDF visual stability

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

One process-wide native Pdfium service owns metadata/raster work; visual open is independent of transcript/OCR; native page ownership and bounded residency are accepted.

## Goal 0020 / Gate 3.1 — continuous PDF viewport / practical zoom

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Physical QA on a real 638-page Caliberate PDF verified continuous adjacent-page scrolling, extremely responsive violent scrolling/scrollbar dragging, responsive page ownership, Previous/Next page jumps, Fit Width/Fit Page/Reset, manual 25–400% zoom, high-zoom horizontal access, and approximate focal preservation.

A9 physical QA re-confirmed that this visual foundation remains excellent. Transient `Rendering page N` placeholders still appear during extreme movement but disappear quickly and are not a blocker.

## Gate 4 — PDF text/TTS/highlight synchronization

**ACTIVE CORE PRODUCT GATE — GOAL 0022 A10 READY**

The architecture remains native-first and recovery-isolated:

1. native visual Pdfium reader is immediately usable;
2. trustworthy embedded text is asynchronously extracted through the same Pdfium owner or reused from safe cache;
3. hostile/mixed/scanned recovery comes later behind a typed optional provider boundary.

Quack-check is not baseline infrastructure. Goal 0022 excludes Quack-check, Python, Docling, OCR, hostile recovery, exact PDF sentence rectangles, visual spoken overlays, and visual PDF auto-follow.

### Goal 0022 accepted architecture through A9

Preserve:

- one process-wide Pdfium owner;
- visual-first PDF open;
- cooperative native text extraction with Current/Nearby raster priority;
- trusted page-aligned `PreparedPdfEmbeddedText`;
- prefix-indexed bounded enriched projections;
- bounded off-egui PDF retirement;
- off-egui document preparation/cache/search;
- source/generation/query-revision stale safety;
- trusted PDF Text-only/search/ordinary Windows TTS policy;
- idempotent `SetTextOnly { enabled }` desired-state semantics;
- Search synchronous draft + actual editor focus ownership;
- runtime-level natural PDF page continuation;
- document-global canonical identity with page-local bounded-plan identity.

A9 substantive commit `8a991a3f7ce72c42e5e0138d09bcbc8f2228f66e` passed hosted workflow `34991458105` (`native-workspace` + `hosted-renderer-probe`) and was integrated through PR #30.

### A9 physical Windows result

**REJECTED.**

What worked:

- violent PDF scrolling/dragging remains excellent;
- PDF Text-only works and is page-aligned;
- natural TTS crossed a native page boundary;
- burst Next mostly works;
- PDF Text-only stress works;
- a newly opened EPUB survives Pretty/Text-only stress;
- reopen remains snappy.

What failed:

- TTS Speed/Volume UI is asynchronously owned: slider values can snap back (observed speed returning to `2.5`) and settings traffic causes severe layout churn;
- changing command/status diagnostics visibly moves reader layout;
- Search computes matches but offers no useful navigation: Enter does nothing and selected match/page/excerpt are not presented;
- burst Previous can repeat instead of moving monotonically backward;
- visual PDF offers no discoverable coarse way to reposition TTS, while exact click-on-rendered-text correctly remains unavailable without geometry;
- the EPUB damaged under A7 can reopen in a truncated-looking state, implying stale presentation/persistence/cache state still needs diagnosis.

A9 physical rejection: `docs/work/reviews/0022-a9-real-desktop-rejection.md`.

### Goal 0022 A10

**READY — CURRENT AUTHORIZED MACRO-GOAL**

A10 owns:

- synchronous/stale-safe/bounded TTS Speed/Volume editor state;
- fixed/bounded diagnostics that cannot reflow reader geometry;
- Search Previous/Next + Enter/Shift+Enter + selected match X/Y/page/excerpt;
- real-runtime burst seek monotonicity;
- explicit coarse `Play from current PDF page` without pretending exact geometry;
- persisted EPUB close/reopen recovery so damaged transient presentation state cannot survive as authoritative content.

Authoritative A10 contract: `docs/work/ready/0022-native-pdf-embedded-text-tts.md`.

## Next after Goal 0022 physical acceptance

### Native PDF sentence geometry / highlight / follow

Build native sentence -> page-relative geometry, visual spoken overlays, continuous-viewport follow/jump semantics, and eventually truthful click-to-sentence behavior where confidence permits.

### Hostile/mixed PDF recovery

Only after the trustworthy native baseline is physically accepted should Quack-check/Docling/OCR concepts be considered behind the typed optional recovery boundary.

## Workflow status

**GOAL 0022 A10 READY — ONE AUTHORIZED MACRO-GOAL.**

No human QA is authorized until A10 passes director source/CI review. Goals 0015, 0017, 0018, 0021, and 0024 remain queued. Goal 0011 remains deferred.
