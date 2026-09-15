# 0022 — Native PDF embedded-text / TTS trustworthy path — A7

## Status

**READY — A7 CORRECTION AFTER A6 DIRECTOR REJECTION**

Read first:

- `docs/work/reviews/0022-a1-director-rejection.md`
- `docs/work/reviews/0022-a2-director-rejection.md`
- `docs/work/reviews/0022-a3-director-rejection.md`
- `docs/work/reviews/0022-a4-director-rejection.md`
- `docs/work/reviews/0022-a5-director-rejection.md`
- `docs/work/reviews/0022-a6-director-rejection.md`
- `docs/architecture/pdf-text-recovery-boundary-2026-09.md`
- `docs/work/reviews/0020-a3-real-desktop-acceptance.md`

Goal 0019/0020 remain physically accepted and authoritative. Goal 0022 must enrich the working continuous native PDF reader without weakening its visual responsiveness or putting document-scale work on egui.

## Outcome

For PDFs with trustworthy embedded text, LanternLeaf asynchronously obtains or reuses page-aligned native text, promotes the canonical session into trusted Text-only/document-wide-search/TTS capability while exact visual sentence geometry remains disabled, preserves native page identity and current mutable reader state, and keeps native/cache/document-scale work away from egui.

A7 is a focused final lifecycle/boundedness correction after A6 fixed canonical identity and live search reconciliation.

Visual PDF browsing remains independently usable before, during, after, or despite enrichment/cache/search failure.

Exact spoken overlays, hostile/mixed recovery, Quack-check, Python, Docling, and OCR remain out of scope.

## Non-negotiable architecture

- Native Rust + `eframe`/`egui` + bundled/native Pdfium only.
- Exactly one process-wide `PdfNativeService` / one Pdfium owner.
- No Python, Quack-check, Docling, OCR, or `scripts/quack-check/*` in the Goal-0022 production path.
- Visual source open never waits for text enrichment, text-cache IO, search reconciliation, or payload retirement.
- Current and already-visible raster responsiveness outrank background text extraction.
- Heavy/blocking/document-scale allocation, cloning, flattening, scanning, destruction, cache IO, and native work never execute on egui.
- Ordinary reader interaction must not create an unbounded number of OS threads.
- UI-facing native/cache request submission remains nonblocking.
- Extraction/adoption/cache/search results are source/generation/revision stale-safe.
- PDF canonical text preserves native PDF page identity; do not line-count-repaginate it.
- Existing backend-neutral / first-sample Windows TTS ownership remains authoritative.

## Preserve accepted A1–A6 direction

Preserve:

- one process-wide native Pdfium worker;
- visual-first PDF source open;
- cooperative/resumable native embedded-text extraction;
- deterministic conservative trust gate;
- shared immutable `Arc<PreparedPdfEmbeddedText>` trusted document state;
- current live session authority at adoption time;
- trusted policy promotion to Text-only/document-wide search/TTS with exact visual sync disabled;
- off-thread document-scale preparation and native-text cache lookup/parse/persistence;
- Current and Nearby raster preemption;
- bounded eight-page text chunks or equivalent proven bounded strategy;
- nonblocking UI-facing metadata/text request enqueue;
- correct later-page/global canonical sentence identity;
- correct first-sample identity across native page boundaries;
- true final-document TTS exhaustion without replay;
- query-revision/source/generation-safe off-thread search reconciliation;
- document-wide native-PDF search with native-page/page-local provenance/navigation;
- explicit bounded enriched-PDF publication API;
- versioned cache identity/reuse;
- Goal 0019/0020 visual/viewport/zoom behavior;
- representative EPUB visual/TTS behavior.

## A7 correction 1 — replace per-snapshot OS-thread creation with one bounded retirement path

A6 puts document-scale payload release off egui by implementing custom `Drop` for enriched `ReaderSnapshot` / `ReaderSession`, but `ReaderSnapshot::drop()` calls `std::thread::spawn()` directly.

That is not acceptable for ordinary reader churn. Continuous PDF viewport ownership emits `SetPage` as native page ownership changes; reader commands produce replacement full snapshots. Rapid scrolling, navigation, search, and settings therefore can retire many enriched snapshots. A6 can create a new native OS thread for each retirement even when the shared document is not at its final reference.

Required A7 behavior:

1. Use one shared/bounded retirement mechanism rather than one thread per snapshot/session drop.
2. A snapshot/session drop on egui may only perform a bounded nonblocking handoff of document-scale handles.
3. One process-wide or app-owned retirement worker/queue must perform final Arc release/destruction off egui.
4. Replacing snapshots during rapid native-page changes must not increase retirement-worker thread count.
5. Source switch, reader close, re-adoption, runtime projection replacement, and ordinary snapshot churn must all preserve off-egui final destruction.
6. Do not clone document-scale data to achieve retirement safety.
7. If the retirement queue is bounded, define safe backpressure/fallback semantics that never perform document-scale destruction on egui and never block the render thread.

### Required retirement/churn regression

Simulate production-shaped enriched PDF snapshot churn corresponding to many rapid `SetPage`/reader updates.

Prove:

- final trusted-document payload destruction occurs off the commit/test thread;
- many snapshot replacements use one stable retirement worker (or another explicitly bounded worker count), not one OS thread per snapshot;
- ordinary snapshot retirement does not block waiting for the retirement worker;
- source close and trusted-document replacement still retire the final payload off the caller thread;
- Goal 0020-style rapid page ownership remains free of new per-page thread creation.

## A7 correction 2 — make the enriched-PDF projection truly bounded in total document size

A6 correctly changed canonical identity to the active trusted document, but several helpers used by `snapshot_enriched_pdf_bounded()` still scan page-count vectors.

The prepared document already owns `page_sentence_prefix_sums` and `page_word_prefix_sums`. Use them.

Required A7 behavior for enriched PDFs:

- `global_display_idx()` obtains page base from the prepared sentence-prefix index in O(1);
- `has_canonical_sentence_after_current()` uses the prepared total/prefix index rather than summing all page counts;
- `has_sentence_before_current_page()` and `has_sentence_after_current_page()` use prefix/total counts rather than scanning before/after pages;
- `current_tts_audio_display_ids()` uses the same O(1) trusted page base;
- `page_idx_for_global_sentence()` uses the prepared prefix index with binary search or equivalent O(log pages), not a linear page walk;
- audit bookmark/TTS/search/current-page helpers for other direct full-page-count prefix scans in the enriched-PDF path;
- current-page-local sentence cloning is allowed; total-document scanning/flattening is not.

The explicit `snapshot_enriched_pdf_bounded()` publication path must remain current-page-local plus Arc clones and must not perform work proportional to native page count or canonical sentence count.

### Required boundedness regression

Build equivalent enriched-PDF fixtures whose page counts differ by at least an order of magnitude and instrument the bounded projection/helper path.

Prove that projection work does not increase linearly with total PDF pages/sentences. Prefer structural counters/assertions over fragile wall-clock thresholds.

## Trusted-text policy contract

After trusted adoption:

- Text-only is allowed;
- document-wide canonical search is allowed;
- Search Next/Previous navigate native-page provenance correctly;
- ordinary Windows TTS is allowed through existing canonical machinery;
- native `pdf_page_count` and current page remain authoritative;
- bookmark/canonical sentence ownership remains native-page aligned;
- visual sentence highlighting remains disabled;
- `pretty_sync_enabled = false` and `exact_sentence_sync = false` until real sentence geometry exists;
- no guessed rectangles or fake visual sync.

Rejected/untrusted text remains visual-only.

## Cache / extraction / request contract

Preserve A6 behavior:

- cache lookup/read/parse/validation and persistence stay off egui;
- warm trusted cache avoids native re-extraction;
- corrupt/stale cache falls through safely;
- UI-facing metadata/text enqueue cannot block egui;
- raster textures remain ephemeral;
- Current and Nearby raster requests preempt background text chunks;
- cooperative extraction uses substantially fewer native document opens than page count;
- rapid visual use may temporarily starve text enrichment; visual responsiveness wins.

## Required deterministic coverage

At minimum prove:

- one Pdfium owner serves metadata/raster/text;
- Current and Nearby raster preempt text;
- reduced native-document-open amplification;
- document preparation/cache/search stay off egui;
- trusted enrichment preserves newer mutable live state;
- bounded enriched-PDF publication does not construct ordinary document-scale snapshots;
- retirement uses a stable bounded worker count under snapshot churn;
- final document-scale payload destruction occurs off egui;
- no total-page scan in the enriched bounded projection/global identity helpers;
- production render-only -> trusted policy promotion;
- later-page/global canonical identity and first-sample identity remain correct;
- final-document TTS exhaustion remains correct;
- pre-enrichment/changed/cleared search reconciliation remains stale-safe;
- later-page FullText search navigation remains correct;
- Text-only and ordinary TTS work;
- exact visual sentence sync remains disabled;
- trust-gate rejection cases remain correct;
- source-switch stale safety and cache reuse/corruption safety remain correct;
- Goal 0019/0020 regressions remain green;
- representative EPUB visual/TTS regressions remain green.

## Validation / handoff

Run focused tests plus serialized workspace tests as appropriate, `cargo check --workspace`, `cargo build --workspace`, repo-native Windows QA preparation, `git diff --check`, hosted `native-workspace`, and hosted renderer/native-PDF capability coverage.

Do not terminalize or signal Goal achieved until the hosted workflow for the substantive A7 implementation lineage is green.

Update `docs/work/reports/0022.md` with A7 evidence. Move ready -> active -> done normally, push before terminal signaling, restore shared checkout to `main`, and do not request human QA. The director reviews first.
