# 0022 — Native PDF embedded-text / TTS trustworthy path

## Current state

Goal 0019 established the native Pdfium visual foundation. Goal 0020 established the physically accepted continuous native viewport, practical 25–400% zoom, Fit Width/Fit Page/Reset, responsive current-page ownership, jump controls, and aggressive long-document scrolling without EPUB/TTS regression.

Goal 0021 is queued minor zoom-transition polish and does not block Gate 4.

The next core problem is PDF text/TTS. The previous Quack-check-derived path is **not** allowed to become a source-open prerequisite again.

Read first:

- `docs/architecture/pdf-renderer-contract.md`
- `docs/architecture/pdf-text-recovery-boundary-2026-09.md`
- `docs/work/reviews/0020-a3-real-desktop-acceptance.md`
- `docs/roadmaps/restart-master-roadmap-2026-09.md`

## Outcome

For PDFs with trustworthy embedded text, LanternLeaf can asynchronously enrich the already-open native PDF session with page-aligned canonical text and use the existing reader/TTS pipeline without Python, Docling, OCR, or Quack-check.

A user must be able to:

1. open and browse the PDF immediately exactly as today;
2. allow native text enrichment to complete in the background;
3. use Text-only/search/TTS when the embedded text passes a conservative trust gate;
4. keep native PDF page identity aligned with canonical sentences;
5. continue browsing normally if text extraction fails or is rejected as untrustworthy.

This goal does **not** implement exact spoken overlay geometry yet. That follows after the native text/TTS ownership path is physically trustworthy.

## Hard architecture rules

### 1. Preserve the accepted visual reader unchanged in authority

- Native Rust + `eframe`/`egui` + bundled/native Pdfium only.
- One process-wide `PdfNativeService` / one Pdfium owner.
- Visual source open remains independent of text recovery.
- No Python, Quack-check, Docling, OCR, or external recovery process is invoked by Goal 0022.
- Text extraction failure must never transition an already-open PDF into source error.
- No heavy/native work on the egui/render thread.

### 2. Native text extraction must go through the existing Pdfium owner

Extend the shared native PDF service with a typed, bounded text-enrichment request/result path rather than constructing another Pdfium renderer/binding.

The native result must be page-aligned and carry enough identity to reject stale results. At minimum:

- source identity/path;
- request/source generation or revision;
- native page count;
- one extracted text payload per native PDF page;
- worker-thread evidence for tests/diagnostics;
- explicit terminal success/failure.

Use Pdfium's native text facilities through the existing `pdfium-render` dependency. Do not shell out to Python for the trustworthy embedded-text path.

### 3. Conservative trust gate

Do not equate “Pdfium returned some strings” with trustworthy canonical text.

Implement a small explicit native embedded-text quality assessment suitable for deciding whether this goal may enable canonical Text-only/search/TTS. It should be deterministic and conservative, based on signals available from the native page-aligned text result, such as:

- non-empty/text-bearing page coverage;
- text density / character counts;
- replacement/control/garbage character rate;
- pathological whitespace or duplicate-text indicators that are cheaply available;
- obvious empty/image-only behavior.

The exact thresholds must be named/tested rather than hidden magic scattered through UI code.

This goal only needs to distinguish:

- **trustworthy embedded text** -> adopt as canonical;
- **not trustworthy / absent / unknown** -> remain visual-only and record a degraded reason for future recovery.

Do not attempt to solve mixed/scanned/hostile PDF recovery here.

### 4. PDF canonical text must preserve native page identity

When native embedded text is accepted, do not repaginate the transcript into arbitrary line-count reader pages.

The canonical relationship must remain:

`native PDF page N -> canonical accepted page text -> canonical sentences belonging to page N`

Add/adjust `ReaderSession` support so a PDF session can adopt page-aligned canonical text after visual open while preserving:

- native `pdf_page_count`;
- current native page;
- native viewport position ownership;
- bookmark page identity;
- ordinary reader settings;
- source identity;
- existing continuous PDF visual state.

The PDF text page/sentence domain must not diverge from native page indexes.

### 5. Asynchronous session enrichment

Visual open currently creates a render-only PDF session with empty TTS text. Goal 0022 must add an explicit enrichment lifecycle rather than reopening the source synchronously.

Required behavior:

- visual session becomes usable first;
- native text work starts off-thread;
- completion is delivered through a typed app/runtime event;
- source/generation identity is checked before adoption;
- switching/closing the PDF makes old enrichment harmless/stale;
- accepted enrichment updates the session atomically enough that the UI never observes half-rebuilt sentence/page state;
- failed/rejected enrichment leaves the existing visual session intact.

Do not let a worker mutate the live session directly behind app/runtime ownership.

### 6. Use the existing canonical TTS pipeline

Once trustworthy text is adopted:

- Text-only mode shows the accepted canonical PDF text;
- search uses that canonical text;
- sentence splitting/normalization uses the normal reader pipeline;
- Windows TTS uses the existing backend-neutral/first-sample ownership path;
- Play/Pause/seek/repeat semantics are not reimplemented specially for PDFs;
- canonical highlighted sentence identity changes only on the existing first-sample boundary.

If a trustworthy PDF has canonical text but no exact geometry yet, visual PDF sentence highlighting must remain explicitly unavailable rather than faked.

### 7. Page-level playback relationship

Because canonical PDF sentences are page-aligned, the session/runtime must be able to resolve a canonical sentence to its native PDF page deterministically.

Goal 0022 should provide the stable sentence -> native page provenance needed by the next overlay goal.

If existing active-TTS-page scheduling/follow machinery can safely use this provenance without inventing exact geometry, it may keep the spoken page warm or perform explicitly page-level behavior. Do not implement fake sentence rectangles.

### 8. Durable cache reuse

Reuse or replace the existing versioned PDF precompute/cache artifact shape where semantically appropriate (`PdfRenderPrecomputedState`, page texts, sentence-page hints, sync metadata).

Requirements:

- cache is keyed by source identity/content plus an explicit native-text extraction revision;
- cache reuse avoids rerunning native extraction on every reopen;
- corrupt/version-stale artifacts are ignored/removed and rebuilt non-destructively;
- loading a bad text cache cannot break visual PDF open;
- do not persist ephemeral raster textures as part of this goal.

## Quack-check explicit non-use

Goal 0022 must not call:

- `load_pdf_with_quack_check`;
- `run_pdf_to_text*`;
- `PythonEngine`;
- Docling;
- OCR scripts;
- `scripts/quack-check/*` for production native-text enrichment.

The old vendored code may remain in the repository for later recovery work, but it is not authoritative for this goal.

## Required deterministic coverage

Add production-path tests proving at minimum:

1. the process-wide native PDF service can return page-aligned embedded text from a real valid multi-page PDF fixture after the service has already been initialized/idle;
2. native text extraction uses the same Pdfium owner rather than a second binding;
3. text work is off the egui thread;
4. a trustworthy embedded-text fixture passes the native trust gate;
5. an empty/image-only/garbage fixture is rejected/degraded rather than promoted;
6. visual source open completes independently of text-enrichment success/failure;
7. accepted enrichment preserves native PDF page count/current page and maps canonical sentences to native pages;
8. PDF text adoption does not repaginate into an unrelated line-count page domain;
9. stale enrichment from source A cannot mutate source B after a rapid source switch;
10. Text-only/search become meaningful only after accepted enrichment;
11. PDF TTS uses the normal canonical TTS pipeline and existing first-sample ownership semantics;
12. extraction failure leaves the PDF continuously browsable/renderable;
13. cached native text is reused on reopen and stale/corrupt cache is safe;
14. Goal 0019/0020 native rendering/continuous viewport/zoom regressions remain green;
15. representative EPUB visual controls + Windows TTS remain green.

## Hosted/Windows validation

Run focused tests plus workspace check/build/test and repo-native Windows QA preparation.

Extend hosted Windows coverage enough to prove the native embedded-text Pdfium request path on Windows. Do not terminalize the repository goal until required hosted native jobs for the implementation lineage are green.

No Python/Docling installation should be required for Goal 0022 hosted acceptance.

## Physical QA after director acceptance

The director will request physical QA only after source/CI review.

Expected focused pass:

- open a real text-bearing PDF and confirm visual page appears immediately;
- confirm PDF remains fully scrollable while/after text enrichment;
- confirm Text-only contains plausible text when enrichment is accepted;
- play Windows TTS and verify audible speech/Play/Pause/seek ownership;
- verify current native page identity remains sensible;
- switch PDF/source during/after enrichment and check stale safety;
- open an EPUB and verify existing visual/TTS behavior remains normal.

Exact PDF spoken sentence overlay is **not** an acceptance requirement for Goal 0022.

## Explicit non-goals

Do not implement:

- Quack-check recovery;
- Docling;
- OCR;
- mixed/scan hostile-PDF policy beyond conservative rejection;
- sentence rectangle overlays;
- PDF click-to-sentence reverse mapping;
- final PDF auto-follow polish;
- Goal 0021 zoom-transition polish;
- Goals 0015/0017/0018;
- Windows Natural/HD voices;
- unrelated UI redesign.

## Repository handoff

- Repository goal: `0022-native-pdf-embedded-text-tts`.
- This is the only ready substantive macro-goal after Goal 0020 closure.
- Preserve Goal 0021 as queued minor polish.
- Codex must sync director `main`, move this file `ready -> active`, re-arm the watcher, implement only this bounded native trustworthy-text path, validate, write `docs/work/reports/0022.md`, wait for hosted Windows success, move to `done/`, push before terminal signaling, and restore the shared checkout to `main`.
- Do not request human QA. The director reviews first.
