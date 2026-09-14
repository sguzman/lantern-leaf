# LanternLeaf Current Status

Updated: 2026-09-14 after Goal 0022 A2 director rejection and A3 reopening.

This file contains current verified/bounded state. Detailed attempt history lives in `docs/work/reports/` and `docs/work/reviews/`.

## Workspace / architecture

**VERIFIED / AUTHORITATIVE**

- Native Rust + `eframe`/`egui` is the production desktop architecture.
- Rust owns canonical document/session/playback state.
- Tauri/React/WebView is historical reference only.
- Windows human workflow is repo-native: `git pull -> .\qa.ps1`.
- Scoop is the active Windows CLI dependency convention.
- Heavy/blocking work must never run on the egui/render thread.

## Gate 0 — Windows baseline

**COMPLETE**

Hosted Windows CI covers MSVC/Pandoc setup, workspace check/build/test, repo-native QA preparation, Windows TTS synthesis/decode, and a separate hosted renderer probe.

## Gate 1 — backend-neutral TTS + Windows TTS

**COMPLETE FOR THE WORKING WINDOWS PATH**

Accepted runtime shape:

`canonical display sentence -> backend synthesis -> prepared audio -> Rodio first-sample boundary -> canonical ReaderSession cursor -> native UI projection`

Real Windows evidence proves audible Windows speech, Play/Pause, ordinary installed voice switching, and sentence-boundary-driven pretty/text-only synchronization.

## Goal 0008 / Gate 2.5 — Caliberate first-class reader integration

**COMPLETE — AUTOMATED + REAL-DESKTOP ACCEPTED**

The large real Caliberate EPUB opens quickly, remains responsive, speaks through Windows TTS, highlights the actually audible sentence in native pretty view, and follows playback correctly.

## Goal 0009 / Gate 2.6 — TTS playback polish and layered voice configuration

**COMPLETE — AUTOMATED + REAL-DESKTOP ACCEPTED**

Stable audible TTS, no unsolicited duplicate ordinary lines, correct pretty/text-only spoken-sentence highlight/follow, portable Zira inheritance, per-book voice persistence, transactional Piper rejection/recovery, Close Book, and Safe Quit are accepted.

## Goal 0012 — pretty presentation controls and inline images

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Accepted presentation/image behavior includes literal margins, presentation geometry invalidation, scrollable settings, working word/letter spacing, readable tables/TOCs, restrained blockquotes, explicit font fallback state, inline EPUB imagery, bounded off-render-thread pretty/image work, and durable canonical spoken highlighting/follow after geometry changes.

## Goal 0013 — starter shell responsive panel containment

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Implementation `58ae9de` uses actual center width, a deterministic `1120px` two-column breakpoint, one-column fallback below it, width-bounded groups, wrapped action/control rows, and bounded long-path/URL presentation while preserving Calibre virtualization and off-render-thread work.

## Goal 0014 — reader presentation-geometry anchor stability

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

A3 removed the high-severity multi-frame reflow instability by retaining one last-stable semantic viewport witness across an edit burst, avoiding recapture from unstable intermediate geometry, keeping bounded virtualization in the anchor neighborhood, reconciling after measured geometry is available, preserving pending TTS-follow precedence, and yielding to explicit user wheel/drag input.

A lower-severity residual remains under severe cumulative text-metric edits such as aggressive letter spacing and font scaling. This is queued separately as Goal 0015.

## Goal 0010 — Caliberate catalog covers / provider availability UX

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

Caliberate exposes the explicit `/api/v1/books/{id}/cover` provider contract; LanternLeaf propagates `has_cover`, lazily fetches covers for visible/near-visible rows, keeps network/disk/decode work off the UI thread, distinguishes intentional cover states, and never materializes an EPUB merely to obtain a thumbnail.

## Goal 0016 — starter library live-state continuity

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

The real 105,570-book Caliberate library becomes usable while the catalog is still loading: provider pages publish progressively, the starter shell shows truthful loaded/total progress, final reconciliation preserves live cover state, and successful source persistence refreshes Recents in the same process.

## Goal 0015 — highlight viewport-band reflow polish

**QUEUED — MINOR POLISH**

Preserve a stronger temporary viewport band for an already-visible canonical highlight during severe text-metric reflow without permanent highlight pinning or fighting user scrolling.

## Goal 0017 — progressive cover backpressure / cached-hydration scaling

**QUEUED — CONFIRMED LIBRARY SCALING POLISH**

Warm cached runs proved repeated roughly-O(total-books) thumbnail-hydration scans and giant catalog-cache rewrites. Goal 0017 owns bounded transient backoff/retry plus lazy/indexed cached-thumbnail association and bounded logging/cache rewrites.

## Goal 0018 — Windows QA bootstrap idempotence

**QUEUED — QA INFRASTRUCTURE**

Repeated `qa.ps1` invocations in one PowerShell process can eventually make `VsDevCmd.bat` fail with `The input line is too long`. Until fixed, use a fresh PowerShell for physical QA.

## Windows Natural/HD voices

**DEFERRED BY USER — DO NOT WORK ON OR TEST UNTIL RE-AUTHORIZED**

Preserve the existing ordinary Windows voice backend. Goal 0011 remains dormant.

## Goal 0019 / Gate 3 — native PDF visual stability

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

The native Pdfium/egui visual foundation is physically proven on Windows. Accepted behavior includes visual-first PDF open independent of transcript/OCR prerequisites, one process-wide `PdfNativeService`, truthful native page-domain ownership, native raster presentation, stale-safe render identity, bounded residency, and all heavy Pdfium work off the egui thread.

## Goal 0020 / Gate 3.1 — continuous native PDF viewport and practical zoom

**COMPLETE — AUTOMATED + DIRECTOR + REAL-DESKTOP ACCEPTED**

A3 is physically accepted on a real 638-page Caliberate PDF.

Verified behavior:

- continuous wheel scrolling across page boundaries;
- portions of adjacent pages visible simultaneously;
- extremely responsive rapid long-document scrolling;
- responsive viewport-derived `Page N / 638` ownership;
- Previous/Next as continuous-stack jumps;
- Fit Width, Fit Page, Reset/100%, and manual 25–400% zoom;
- horizontal navigation at high zoom;
- approximate semantic focal preservation during zoom;
- no representative EPUB visual/TTS regression.

Real-desktop acceptance: `docs/work/reviews/0020-a3-real-desktop-acceptance.md`.

## Goal 0021 — fit-to-manual zoom transition polish

**QUEUED — MINOR PDF UX POLISH**

`+/-` entered after Fit Width/Fit Page currently resume the prior manual zoom ladder rather than stepping from the current effective fit percentage. Reset/100% plus manual `+/-` works. This does not keep Goal 0020 open.

## Gate 4 — PDF text/TTS/highlight synchronization

**ACTIVE NEXT CORE PRODUCT GATE — GOAL 0022 A3 READY**

The Gate-4 architecture remains native-first and recovery-isolated. Quack-check is **not** trusted as baseline infrastructure and cannot block visual open. The accepted recovery boundary is documented in `docs/architecture/pdf-text-recovery-boundary-2026-09.md`.

The trust order remains:

1. native visual Pdfium reader is immediately usable;
2. trustworthy embedded text is extracted asynchronously through the same process-wide Pdfium owner;
3. only later, for degraded/mixed/scanned PDFs, a typed background recovery provider may reuse/rewrite Quack-check/Docling/OCR components.

The standalone `sguzman/quack-check` repository is not a runtime dependency. Goal 0022 explicitly excludes Quack-check, Python, Docling, OCR, and exact PDF sentence overlays.

### Goal 0022 — native PDF embedded-text/TTS trustworthy path

**READY A3 — A1 AND A2 REJECTED BEFORE HUMAN QA**

A1 established useful native text/trust/cache machinery but was rejected because whole-document extraction monopolized the sole Pdfium owner and document-scale adoption/cache work ran on egui.

A2 corrected those defects with cooperative extraction, Current-raster preemption, off-thread canonical preparation/cache persistence, and green hosted Windows validation. Director review still found three production blockers:

- trusted text was adopted without promoting the production render-only PDF policy, so Text-only/search/TTS would remain disabled;
- the egui completion still called the ordinary full `ReaderSession::snapshot()` path, cloning document-scale canonical sentence state on the render thread;
- only `Current` raster work preempted text extraction, so queued `Nearby`/adjacent-visible pages could still wait behind the background text job.

A2 also reopened/reparsed the native PDF once per extracted text page; A3 must preserve cooperative yielding while reducing that amplification.

A3 therefore requires truthful trusted-text/no-geometry policy promotion, a genuinely bounded UI commit, all pending visual raster work ahead of text enrichment, and bounded-chunk/resumable native extraction without one document open per page.

Authoritative contract: `docs/work/ready/0022-native-pdf-embedded-text-tts.md`.
A2 rejection: `docs/work/reviews/0022-a2-director-rejection.md`.

## Workflow status

**GOAL 0022 A3 READY NEXT — ONE AUTHORIZED MACRO-GOAL**

No human QA is authorized for A2. Goals 0015, 0017, 0018, and 0021 remain queued. Goal 0011 remains deferred. Exact native PDF spoken overlays/follow come after Goal 0022. Hostile-PDF Quack-check/Docling/OCR recovery comes only after the native trustworthy-text path is physically accepted.
