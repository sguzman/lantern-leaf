# LanternLeaf Restart Master Roadmap — September 2026

This is the active restart roadmap for the Windows/native-egui line. Completion is evidence-driven: accepted implementation plus the human-only runtime evidence a gate actually requires.

## Gate 0 — Trustworthy Windows baseline

**STATUS: COMPLETE**

Goals 0001–0005 established reproducible Windows CI, deterministic cache/test behavior, native-egui launch, repository-owned Windows QA, macro-goal notifications, and repaired bounded core contracts.

## Gate 1 — Backend-neutral TTS + Windows TTS

**STATUS: COMPLETE FOR THE WORKING WINDOWS PATH**

Accepted flow:

`canonical sentence -> backend synthesis -> prepared audio -> Rodio first-sample boundary -> canonical ReaderSession -> native UI`

Windows speaker playback, Play/Pause, ordinary installed voice switching, and first-sample-driven canonical cursor ownership are physically verified.

## Gate 2 — Non-PDF reader/TTS

**STATUS: COMPLETE FOR CURRENT TXT / MARKDOWN / HTML / EPUB PATH**

The accepted path includes native pretty rendering, canonical text ownership, synchronized spoken highlight/follow, Windows TTS, layered voice configuration, presentation controls, inline imagery, and stable reflow anchoring.

Related completed goals include 0008, 0009, 0012, 0013, 0014, 0010, and 0016.

### Queued non-core polish / infrastructure

- Goal 0015 — highlight viewport-band severe-reflow polish.
- Goal 0017 — Caliberate cover backpressure and cached-thumbnail hydration scaling.
- Goal 0018 — idempotent Windows QA environment bootstrap.
- Goal 0011 — Windows Natural/HD voices remains deferred by user.

## Gate 3 — Native PDF visual stability

**STATUS: COMPLETE — GOAL 0019 CLOSED**

The accepted native foundation includes:

- native Rust/egui/Pdfium presentation with no WebView/pdf.js/Tauri production fallback;
- one authoritative process-wide `PdfNativeService` / one native Pdfium owner;
- all Pdfium open/metadata/raster/bitmap work off the egui thread;
- visual-first PDF open independent of Quack-check/Python/Docling/OCR/transcript recovery;
- truthful native page-domain ownership and page count;
- stale-safe source/page/render-spec identity;
- bounded current-priority raster scheduling and residency;
- recoverable/terminal failure behavior that cannot silently strand source-open state.

Real-desktop QA verified a representative 638-page Caliberate PDF opening and basic native browsing with no representative EPUB regression.

## Gate 3.1 — Continuous native PDF viewport + practical zoom

**STATUS: COMPLETE — GOAL 0020 CLOSED**

Goal 0020 A3 is accepted with automated, director, hosted-Windows, and real-desktop evidence.

Physical Windows QA verified:

- continuous wheel scrolling across page boundaries;
- partial adjacent pages visible simultaneously;
- extremely responsive rapid long-document scrolling;
- responsive viewport-derived `Page N / 638` ownership;
- Previous/Next as jumps within the continuous stack;
- Fit Width, Fit Page, Reset/100%, and manual 25–400% zoom;
- horizontal access at high zoom;
- approximate semantic focal preservation during zoom;
- no representative EPUB visual/TTS regression.

### Goal 0021 — fit-to-manual zoom transition polish

**STATUS: QUEUED — MINOR PDF UX POLISH**

After Fit Width/Fit Page, `+/-` currently resumes the remembered prior manual zoom rather than stepping from the current effective fit percentage. Reset/100% + manual stepping works. This does not keep Goal 0020 open.

## Gate 4 — PDF text, TTS, and highlight synchronization

**STATUS: ACTIVE NEXT CORE GATE — GOAL 0022 READY**

Gate 4 is deliberately staged so old PDF recovery technology cannot destabilize the now-trustworthy visual reader.

Architecture boundary: `docs/architecture/pdf-text-recovery-boundary-2026-09.md`.

### Gate 4A — Goal 0022: trustworthy native embedded text + TTS

**STATUS: READY — ONLY AUTHORIZED SUBSTANTIVE MACRO-GOAL**

Extend the existing process-wide `PdfNativeService` with typed, bounded native Pdfium text extraction. For PDFs whose embedded text passes a conservative deterministic trust gate:

- adopt page-aligned canonical text asynchronously after visual open;
- preserve native PDF page identity rather than repaginating into arbitrary reader pages;
- enable Text-only/search/TTS through the existing canonical reader and first-sample Windows TTS pipeline;
- cache accepted page-aligned text with explicit extraction revisioning;
- reject stale enrichment after source switches;
- leave the visual PDF completely usable if extraction fails or is rejected.

Goal 0022 explicitly forbids Quack-check, Python, Docling, OCR, and exact sentence overlays.

Authoritative contract: `docs/work/ready/0022-native-pdf-embedded-text-tts.md`.

### Gate 4B — native sentence geometry, spoken highlight, and follow

**STATUS: NEXT AFTER GOAL 0022 PHYSICAL ACCEPTANCE**

For accepted embedded-text PDFs, use native Pdfium text geometry to build canonical sentence -> page-relative rectangle mappings, spoken highlight overlays, continuous-viewport auto-follow/jump behavior, and explicit confidence downgrade semantics.

Canonical text remains the owner of TTS/search identity; geometry is a projection/evidence layer.

### Gate 4C — hostile/mixed/scanned PDF recovery

**STATUS: AFTER THE NATIVE TRUSTWORTHY PATH IS PHYSICALLY ACCEPTED**

Only then introduce a typed recovery-provider boundary for PDFs whose native embedded text is absent or untrustworthy.

Quack-check is treated as historical/useful source material, not trusted infrastructure. LanternLeaf may reuse, rewrite, or discard its components behind the provider boundary. Docling/OCR/Python remain optional cancellable background work and may never block or invalidate the native visual reader.

Core invariant:

**A broken text extractor may cost LanternLeaf TTS for that PDF. It may never cost LanternLeaf the PDF.**

## Gate 5 — Format expansion and ingestion cleanup

DOCX/Word, further HTML edge cases, common source/document boundaries, and broader format fixtures.

## Gate 6 — Ergonomics, performance, packaging

Startup/TTS latency, measured cold-open performance, broader UI cleanup, large-document ergonomics, optional Piper model/voice management, library/import polish, release packaging, and dependency cleanup justified by measured problems.

## Workflow UX — Codex macro-goals

Repository goal identity is durable; Codex Goal sessions are disposable attempts. Correction attempts reuse the repository goal ID, re-arm the watcher, push before signaling terminal state, and return the shared checkout to `main`.

Human physical QA is requested only after director source/CI acceptance.

## Director rule

ChatGPT may combine related repair passes into one macro-goal when doing so removes needless human/Codex round trips without opening architectural ambiguity. Current verified state and active goal contracts outrank historical roadmap text.
