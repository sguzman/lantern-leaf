# Goal 0019 A3 — real-desktop rejection

## Decision

**REJECTED ON REAL-DESKTOP QA — CONTINUE AS A4**

The accepted A3 native Pdfium/egui renderer architecture remains valuable and should be preserved, but Goal 0019 cannot close because a real Caliberate PDF never reaches the Reader/Pdfium surface.

## Physical evidence

On the focused Windows pass, selecting Caliberate PDF book `725` correctly entered `SourceLoading` and materialized the PDF to the LanternLeaf QA cache in about 2.4 seconds. Immediately afterward, source opening failed before the Reader surface became active:

```text
Failed to transcribe PDF with in-process quack-check module ...
canonicalize scripts_dir: A:\Code\mycode\lantern-leaf\.qa\windows\scripts/quack-check:
The system cannot find the path specified. (os error 3)
```

The shell then transitioned from `SourceLoading` to `SourceError`.

This is not a Pdfium raster failure. The native visual worker never receives ownership because the generic PDF source-ingestion path currently makes successful source open contingent on Quack-check transcript initialization.

## Root architectural defect

`load_source_content()` routes every PDF through `load_pdf_with_quack_check()`. On a transcript cache miss, that path executes the in-process Quack-check pipeline and propagates any Quack-check configuration/script/runtime failure as a fatal source-open error.

The staged QA config uses a relative `scripts_dir = "scripts/quack-check"` with script-directory pinning enabled. In the staged `.qa/windows` configuration context this resolves to a nonexistent QA-relative path, exposing the immediate failure.

Merely copying scripts into `.qa/windows` or fixing the relative path is **not sufficient** for Goal 0019. Gate 3 explicitly requires visual PDF rendering to stand independently from Gate 4 text/OCR/TTS recovery. Even a correctly configured Quack-check run may be slow or may fail on a hostile document. The user must still get the native PDF page surface promptly.

## Accepted A1/A2/A3 work to preserve

- Native Rust + egui + bundled/native Pdfium only; no WebView/pdf.js/Tauri ownership.
- Pdfium initialization, PDF open/load, rasterization, bitmap conversion, and CPU cache remain on the dedicated bounded worker.
- Source/generation/page/render-size identity and stale-result rejection.
- Coalescing/current-priority scheduler semantics.
- Real presentation-scale zoom with bounded/quantized native raster dimensions.
- Deterministic viewport-aware texture residency with current-page pinning.
- Existing focused scheduler/zoom/residency tests and Windows renderer probe.
- No heavy PDF work on the egui/render thread.

## A4 blocking correction — visual-first PDF session

Goal 0019 A4 must decouple the native visual PDF open path from Quack-check/transcript/OCR availability.

### Required behavior

1. **A readable PDF file is enough to enter the native PDF Reader surface.**
   - After local selection or Caliberate materialization succeeds, the app must establish PDF visual/session ownership and allow the Pdfium worker to render the current page without waiting for Quack-check.
   - Gate 3 visual rendering must not be gated by transcript cache presence, Python, Docling, OCR, Quack-check scripts, or Quack-check configuration.

2. **Quack-check/transcript failure is non-fatal to visual open.**
   - Missing scripts, missing Python/Docling, OCR failure, malformed transcript artifacts, or unavailable text recovery may degrade text/TTS/search capability, but must not turn a visually renderable PDF into `SourceError`.
   - Surface a bounded degraded/text-unavailable diagnostic if useful; do not dump giant backend errors into the main Reader surface.

3. **Do not solve A4 by making visual open synchronously wait for Quack-check.**
   - A4 may skip/defer transcript work entirely for the visual gate, or formalize an independent background text-artifact path, but opening the PDF page must remain prompt and visually first.
   - Any transcript/recovery work remains off the UI thread.

4. **Session/page metadata must have an independent native path.**
   - If the current `ReaderSession` construction assumes canonical text exists before a document can open, introduce the smallest explicit PDF visual-only/degraded session state needed to carry source identity, current page, total pages/page count when known, and `PrettyKind::Pdf`.
   - Obtain expensive PDF metadata/page count off the render thread. Do not fake text or block visual rendering on OCR.

5. **Quack-check path resolution must be made robust enough for future Gate 4, but remain subordinate.**
   - Fix the staged/config-relative script path defect or resolve repo-owned Quack-check resources explicitly so the subsystem can be used later.
   - This fix is secondary: a deliberately unavailable Quack-check installation/configuration must still leave visual PDF opening functional.

6. **Local and Caliberate PDF paths must converge.**
   - A local PDF and a successfully materialized Caliberate PDF must reach the same native visual Reader path.
   - A materialization/provider failure may remain fatal because no readable PDF exists; transcript failure may not.

## Required deterministic coverage

Add tests proving at least:

- a PDF with Quack-check/scripts deliberately unavailable still establishes the visual PDF reader/session path rather than `SourceError`;
- a transcript/recovery error is recorded as degraded/non-fatal for visual open;
- local PDF and materialized-provider PDF use the same visual-open contract after a path exists;
- a true missing/unreadable PDF remains a real open failure;
- existing source/generation/page/zoom stale-safety and renderer tests remain green;
- representative EPUB/non-PDF behavior remains green.

Where a full physical Pdfium fixture is unsuitable for a pure unit test, keep policy/session state transitions deterministic and retain the hosted native renderer probe for Pdfium capability evidence.

## Required validation

Before terminalizing A4:

- focused PDF open/session + renderer tests;
- `cargo test -p lanternleaf-egui` and relevant app/core tests;
- `cargo check --workspace`;
- workspace tests/build;
- repo-native QA preparation;
- hosted Windows baseline workflow, including `native-workspace` and `hosted-renderer-probe`;
- `git diff --check`;
- no human QA request during implementation.

## Real-desktop recheck after director integration

The next physical pass begins with one very small gate: open the same representative Caliberate PDF and confirm the native first page actually appears. Only after that succeeds should navigation/zoom/resize/stale-source behavior be exercised.

PDF TTS/highlight/OCR quality remains Gate 4 and must not be pulled into this correction.
