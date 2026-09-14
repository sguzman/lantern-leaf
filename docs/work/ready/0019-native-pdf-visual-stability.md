# 0019 — Native PDF visual stability

## Current state

**REOPENED FOR A4 AFTER REAL-DESKTOP FAILURE — DO NOT REQUEST HUMAN QA YET**

Goal 0019 remains the one authorized repository macro-goal. A1/A2/A3 established and repaired the native Pdfium/egui visual renderer, but the first real-desktop PDF open exposed a higher-level source-ingestion blocker: PDF source opening is still fatally coupled to Quack-check transcript initialization, so a transcript/configuration failure prevents the native visual Reader from appearing at all.

Read these director reviews in order:

- `docs/work/reviews/0019-a1-director-rejection.md`
- `docs/work/reviews/0019-a2-director-rejection.md`
- `docs/work/reviews/0019-a3-director-acceptance.md`
- `docs/work/reviews/0019-a3-real-desktop-rejection.md`

Continue the existing implementation/report lineage on `codex/0019-native-pdf-visual-stability` / `docs/work/reports/0019.md`.

## Outcome

Make PDF a real native reader surface in the authoritative Rust + `eframe`/`egui` application.

A visually readable PDF must be able to enter the Reader and show its actual native Pdfium-rasterized page promptly, with stable navigation, real zoom, responsive scroll/resize behavior, bounded scheduling/residency, and no heavy PDF work on the egui/render thread.

**Gate 3 visual rendering must not depend on Gate 4 transcript/OCR/TTS availability.**

## Accepted visual architecture to preserve

- Native Rust + egui + bundled/native Pdfium only. No Tauri, React, WebView, pdf.js, browser DOM overlays, or browser-owned rendering.
- Pdfium initialization, PDF open/load, page rasterization, bitmap conversion, and renderer CPU caching stay on the dedicated bounded worker.
- The egui thread only computes bounded requests, receives completed images, uploads bounded textures, and composes lightweight controls/presentation.
- Render identity includes source/document identity, generation, page, and quantized/bounded render dimensions.
- Stale source/generation results are rejected and duplicate work coalesces.
- Newest current/visible work outranks obsolete queued nearby work.
- Presentation size is distinct from raster resolution: zoom changes actual logical page size and may exceed the viewport; native raster resolution follows through bounded/quantized keys.
- Deterministic viewport-aware texture residency is the sole visible texture-capacity path; current page is pinned, nearby/keep pages preferred, irrelevant pages evicted first.
- Preserve all A1/A2/A3 zoom/scheduler/residency/native-render tests.

## A4 blocking correction — visual-first PDF open

The physical failure is upstream of Pdfium rendering. A Caliberate PDF successfully materialized, then `load_source_content()` routed it through `load_pdf_with_quack_check()`. On a transcript cache miss, Quack-check initialization failed because staged QA resolved `scripts_dir` to a nonexistent `.qa/windows/scripts/quack-check` path; the error propagated as `source_open_failed`, and the shell moved to `SourceError` before the native Reader could render.

A4 must establish a visual-first PDF open contract.

### 1. A readable PDF path is sufficient for visual Reader ownership

After local selection or provider materialization yields a readable PDF path, establish PDF visual/session state and permit the native render worker to render page 1/current page **without waiting for Quack-check, Python, Docling, OCR, or transcript cache generation**.

If current session construction requires canonical text before opening, introduce the smallest explicit PDF visual-only/degraded session representation needed to carry source identity, `PrettyKind::Pdf`, current page, total/page-count metadata when known, and existing Reader controls.

### 2. Transcript recovery is subordinate and non-fatal

Quack-check/transcript failures may degrade PDF text/TTS/search/highlight capability, but must not turn a visually renderable PDF into `SourceError`.

Missing scripts, Python/Docling absence, OCR failure, bad transcript cache, or other recovery failure should be represented as bounded degraded/text-unavailable state or deferred work. Do not show giant backend error dumps in the main Reader surface.

A4 may defer transcript work entirely from the visual open path. Do not broaden this correction into Gate 4 implementation.

### 3. Keep expensive metadata/recovery off the UI thread

Any PDF page-count probing, transcript extraction, Quack-check work, OCR, disk-heavy artifact work, or Pdfium operations remain off the egui/render thread.

The UI thread may consume already-produced page-count/session/render results only.

### 4. Local and Caliberate PDFs converge after a path exists

Once a source path exists, local and Caliberate PDFs must enter the same visual Reader contract. Provider/materialization failure may remain fatal because there is no readable PDF. Transcript/recovery failure may not.

### 5. Repair Quack-check resource resolution for later use, without restoring the dependency

The staged QA configuration currently copies `conf/quack-check.toml`, whose relative `scripts_dir = "scripts/quack-check"` is resolved into the QA staging tree and fails script pinning. Repair resource/path resolution or staging so Quack-check remains usable for future Gate 4 and hostile-PDF recovery.

This repair is secondary. Deliberately making Quack-check unavailable must still leave Gate 3 visual PDF opening functional.

## Deterministic acceptance coverage

Add tests proving:

1. a PDF with Quack-check/scripts deliberately unavailable still establishes visual PDF Reader/session ownership rather than `SourceError`;
2. transcript/recovery error becomes degraded/non-fatal state for visual open;
3. local PDF and a successfully materialized provider PDF converge on the same visual-open contract after obtaining a path;
4. truly missing/unreadable PDF remains a real open failure;
5. existing source/generation/page/zoom stale-result, scheduler, presentation, and resident-texture tests remain green;
6. representative EPUB/non-PDF reader/TTS/catalog behavior remains green.

Use deterministic policy/session tests when invoking a full Pdfium/Quack-check stack is unsuitable; retain the hosted renderer probe as native Pdfium capability evidence.

## Required validation

Before terminalizing A4:

- focused PDF source-open/session + renderer tests;
- `cargo test -p lanternleaf-egui` and relevant app/core tests;
- `cargo check --workspace`;
- workspace tests and native build;
- repo-native QA preparation;
- hosted Windows baseline workflow success including `native-workspace` and `hosted-renderer-probe`;
- `git diff --check`;
- no human QA request during implementation.

## Real-desktop recheck after director integration

The next physical verification starts with a deliberately tiny gate:

1. open the same representative Caliberate PDF;
2. verify the actual native first page appears;
3. only then proceed to Next/Previous, real zoom/scroll, resize responsiveness, multi-page cache behavior, source switching, and one representative EPUB regression check.

## Explicit non-goals

Do not expand A4 into PDF TTS playback correctness, canonical sentence/text-layer geometry mapping, spoken-sentence highlighting, OCR quality work, click-to-sentence mapping, text selection/copy parity, broad PDF settings redesign, Goal 0015, Goal 0017, Goal 0018, Natural/HD voices, or unrelated UI redesign.

## Repository handoff

- Repository goal: `0019-native-pdf-visual-stability`
- Branch: `codex/0019-native-pdf-visual-stability`
- This is a correction continuation under the same repository goal ID.
- Synchronize current director `main` before implementation.
- Move this file `ready -> active` and re-arm the normal watcher for the fresh Codex Goal attempt.
- Preserve accepted A1/A2/A3 renderer work; implement the A4 visual-first source/session correction.
- Update `docs/work/reports/0019.md` with A4 implementation/test/CI evidence and exact commits.
- Terminalize to `done/` only after all required gates pass.
- Push before terminal signaling and restore the shared checkout to `main`.
- Do not request human QA. The director reviews first.
