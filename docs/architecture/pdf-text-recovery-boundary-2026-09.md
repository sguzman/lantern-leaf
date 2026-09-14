# PDF Text Recovery Boundary — September 2026

## Decision

Gate 4 must not make Quack-check, Python, Docling, OCR, or any transcript recovery process a prerequisite for opening or browsing a PDF.

The accepted Goal 0019/0020 visual path remains authoritative and independent:

`source PDF -> one process-wide PdfNativeService/Pdfium owner -> metadata/raster work off-thread -> continuous egui viewport`

Text/TTS capability is an **asynchronous enrichment** of that already-open visual document.

## Audit conclusion: Quack-check is useful material, not trusted infrastructure

The standalone `sguzman/quack-check` repository has several good ideas worth preserving:

- explicit probe -> policy -> chunk plan -> engine -> postprocess stages;
- deterministic job identity from config + input hash;
- structured reports and stable artifacts;
- explicit quality tiers and fallback decisions;
- bounded chunking for hostile/large PDFs;
- backend separation between Rust orchestration and extraction subprocesses.

But it is not suitable as LanternLeaf's baseline PDF text path:

- extraction depends on Python scripts and optional Docling/OCR dependencies;
- the pipeline is sequential and can legitimately take minutes on a difficult document;
- physical splitting and several probe/extraction paths depend on Python PDF libraries;
- Docling behavior is version-sensitive and many options are best-effort;
- the current LanternLeaf Quack-check config still contains an old Unix-style Docling virtualenv path;
- subprocess/tool/model availability is environmental rather than guaranteed by the native reader;
- a recovery failure must never poison the already-working visual document.

LanternLeaf already contains a vendored/evolved Quack-check-derived module with cancellation, richer probe diagnostics, cache integration, and PDF quality artifacts. The standalone repository therefore must **not** become a runtime repository dependency. If Quack-check logic is used later, LanternLeaf owns the integration contract and may replace or rewrite any part of it.

## Trust hierarchy

### Tier 0 — visual truth

The native Pdfium render is always the first usable PDF experience. It opens independently of text recovery and remains usable if every text/OCR subsystem fails.

### Tier 1 — native embedded text

For PDFs with trustworthy embedded text, use the existing process-wide `PdfNativeService` to extract page text natively through Pdfium.

This is the preferred Gate-4 path because it:

- reuses the already-owned native PDF process boundary;
- requires no Python, Docling, OCR models, or Quack-check scripts;
- can preserve native PDF page identity directly;
- can later expose native text geometry from the same document owner.

Only conservative, explicit trust rules may promote this text to canonical TTS/search ownership. Low-confidence text must degrade rather than pretend to be exact.

### Tier 2 — bounded recovery provider

Only when native embedded text is absent or fails the trust contract may LanternLeaf request a recovery job.

Quack-check is one candidate implementation of this provider, not the definition of the provider.

A recovery provider must return a typed result containing at minimum:

- source identity/digest;
- extractor/provider identity and version/revision;
- page-aligned recovered text when available;
- canonical transcript candidate;
- page/sentence provenance where available;
- geometry/alignment evidence where available;
- explicit confidence/quality/degraded reasons;
- audit report and terminal success/failure state.

The core reader decides whether and how to adopt that result.

## Hard isolation rules

1. **Never block visual open.** Recovery begins only after the native PDF is already usable.
2. **Never run recovery on the egui/render thread.** Python/Docling/OCR/subprocess IO is worker-only.
3. **Never let recovery own Pdfium.** Native rendering/text metadata stay behind the single `PdfNativeService` owner.
4. **Never mutate the active session from an unvalidated stale result.** Source identity + generation/revision must match before adoption.
5. **Never downgrade a working visual session to an error screen because text recovery failed.** Failure produces a degraded text/TTS state, not source failure.
6. **Never claim exact sentence geometry without evidence.** Fallback order remains exact sentence -> fuzzy/local block -> page-only -> no sync.
7. **Never require Quack-check's standalone repository at runtime.** LanternLeaf owns its local recovery adapter/code.
8. **Never hard-code one developer-machine Python environment.** Any optional recovery environment must be discovered/validated explicitly and be Windows-native for the supported Windows path.
9. **Never re-run expensive recovery unnecessarily.** Versioned cache artifacts are reused by source identity and invalidated deliberately.
10. **Cancellation is mandatory.** Closing/switching a source must terminate or orphan safely any recovery work without affecting the new source.

## Canonical PDF text/session model

For PDF text that is accepted as canonical, page identity should remain the native PDF page domain rather than repaginating the transcript into arbitrary reader pages.

The intended shape is:

`native PDF page N -> accepted page text -> canonical sentences on page N`

This gives LanternLeaf a stable bridge for:

- Text-only mode;
- search;
- TTS planning;
- bookmarks/resume;
- page-level TTS follow;
- later sentence geometry overlays.

The existing cache types (`PdfRenderPrecomputedState`, sentence-page hints, sentence maps, OCR alignment artifacts, sync metadata) may be reused where they remain semantically correct, but old artifacts are not authoritative merely because they exist.

## Gate-4 implementation sequence

### Goal 0022 — trustworthy native embedded-text/TTS path

No Quack-check/Docling/OCR. Extend the shared Pdfium service with page-aligned native text extraction, conservatively adopt trustworthy text into the PDF session, enable Text-only/search/TTS through the normal canonical pipeline, preserve native page identity, cache the result, and keep visual browsing independent of enrichment failure.

### Follow-up — native geometry/highlight/follow

Use native Pdfium text geometry for accepted embedded-text PDFs. Build sentence -> page-relative rect mappings, spoken highlight overlays, continuous-viewport auto-follow, jump semantics, and explicit confidence downgrade behavior.

### Follow-up — hostile/mixed PDF recovery

Only after the native trustworthy path is physically proven, harden a typed recovery-provider boundary. Then evaluate/reuse/rewrite Quack-check components behind that boundary, fix Windows environment assumptions, add doctor/capability checks, and test actual degraded/scan PDFs. Docling/OCR remains subordinate background work.

## Safety property

The Gate-4 invariant is simple:

**A broken text extractor may cost LanternLeaf TTS for that PDF. It may never cost LanternLeaf the PDF.**
