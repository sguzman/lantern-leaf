# 0022 — Native PDF embedded-text / TTS trustworthy path — A5

## Status

**DONE — A5 CORRECTION ACCEPTANCE GATES PASSED**

Read first:

- `docs/work/reviews/0022-a1-director-rejection.md`
- `docs/work/reviews/0022-a2-director-rejection.md`
- `docs/work/reviews/0022-a3-director-rejection.md`
- `docs/work/reviews/0022-a4-director-rejection.md`
- `docs/architecture/pdf-text-recovery-boundary-2026-09.md`
- `docs/work/reviews/0020-a3-real-desktop-acceptance.md`

Goal 0019/0020 remain physically accepted and authoritative. Goal 0022 must enrich that working continuous native PDF reader without weakening visual responsiveness or the hard no-heavy/blocking/document-scale-work-on-egui rule.

## Outcome

For PDFs with trustworthy embedded text, LanternLeaf asynchronously obtains or reuses page-aligned native text, promotes the canonical session into trusted Text-only/document-wide-search/TTS capability while exact visual sentence geometry remains disabled, preserves native page identity and current mutable reader state, and keeps all native/cache/document-scale work away from egui.

Visual PDF browsing remains independently usable before, during, after, or despite enrichment/cache failure.

Exact spoken overlays, hostile/mixed recovery, Quack-check, Python, Docling, and OCR remain out of scope.

## Non-negotiable architecture

- Native Rust + `eframe`/`egui` + bundled/native Pdfium only.
- Exactly one process-wide `PdfNativeService` / one Pdfium owner.
- No second Pdfium binding for text concurrency.
- No Python, Quack-check, Docling, OCR, or `scripts/quack-check/*` in the Goal-0022 production path.
- Visual source open never waits for text enrichment or text-cache IO.
- Current and already-visible raster responsiveness outrank background text extraction.
- Heavy/blocking/document-scale allocation, cloning, flattening, destruction, cache IO, and native work never execute on egui.
- UI-facing native/cache request submission remains nonblocking.
- Extraction/adoption/cache results are source/generation/revision stale-safe.
- PDF canonical text preserves native PDF page identity; do not line-count-repaginate it.
- Existing backend-neutral / first-sample Windows TTS ownership remains authoritative.

## Preserve accepted A1/A2/A3/A4 direction

Preserve:

- typed native embedded-text and prepared-result event boundaries;
- one process-wide native Pdfium worker;
- cooperative/resumable native text extraction;
- source-generation cancellation;
- deterministic conservative trust gate;
- page-aligned prepared canonical text and sentence provenance;
- trusted-text runtime policy promotion to Text-only/document-wide search/TTS with exact visual sync disabled;
- off-thread document-scale sentence/count/anchor preparation;
- off-thread native-text cache lookup/read/parse/validation;
- off-thread cache persistence;
- Current and Nearby raster preemption;
- bounded eight-page native text chunks or an equivalent proven bounded strategy;
- nonblocking UI-facing metadata/text request enqueue;
- document-wide native-PDF search with native-page/page-local provenance and navigation;
- versioned native-text cache identity;
- Goal 0019/0020 visual/viewport/zoom behavior;
- representative EPUB visual/TTS behavior.

## A5 correction 1 — do not publish a worker snapshot built from stale mutable session state

A4 captures a clone of the live `ReaderSession` before spawning document preparation, applies prepared text to that clone on the worker, builds a final `ReaderSnapshot`, and later publishes that snapshot after the real live session has continued evolving.

That is not safe.

During async preparation/cache work the user is allowed to keep using the continuous visual reader. Current page, panels, reader settings, playback/highlight state, and other mutable fields may therefore change after preparation begins.

Required A5 behavior:

1. Background workers prepare **immutable document-owned PDF text state only**.
2. Do not build the final runtime `ReaderSnapshot` from an earlier clone of mutable live session state.
3. At commit time, validate source/generation/revision/page count/trust against the current source.
4. Atomically attach/swap the prepared immutable PDF text document into the **current live session**.
5. Preserve mutable state as of commit time, including at minimum:
   - current native page;
   - panel state;
   - reader settings/book overrides;
   - TTS playback state and current canonical cursor where semantically valid;
   - search query;
   - current visual viewport ownership remains independent.
6. Publish a bounded runtime patch/projection that reflects the current live mutable state plus the newly attached trusted-text capability/document handle.
7. Do not regress current page or settings merely because preparation started earlier on another page/state.

### Required race regression

Use a production-shaped visual-first PDF session.

- Begin trusted-text preparation while the session is on page A.
- Before commit, mutate the real live session to page B and change at least one other mutable field (for example TTS paused/playing state, search query, or a reader setting).
- Commit the prepared trusted-text document.
- Prove the live session and published runtime projection both remain on B and retain the newer mutable field.
- Prove Text-only/search/TTS capabilities become enabled.
- Prove exact visual sentence sync remains disabled.

## A5 correction 2 — make document ownership shared/immutable and make egui adoption truly O(1)/bounded, including destructor behavior

A4 still passes document-scale `PreparedPdfEmbeddedText` through the egui commit and `ReaderSession::apply_prepared_pdf_embedded_text()` returns the full canonical `Vec<String>`. The A4 call site ignores that result with `Ok(_)`, causing the document-wide vector of owned sentence strings to be destroyed on egui.

That is forbidden.

Required A5 behavior:

- The egui trusted-text commit must not allocate, clone, flatten, scan, or destroy work proportional to total PDF pages/sentences.
- Do not return/drop a document-wide canonical sentence `Vec<String>` from live adoption.
- Prefer one shared immutable prepared PDF text document handle (`Arc` or equivalent) containing document-owned data such as:
  - native-page-aligned canonical page text;
  - per-page canonical sentences;
  - global canonical sentence domain;
  - page sentence/word counts;
  - sentence -> native page and page-local provenance;
  - immutable search/index support where appropriate;
  - any other trusted document-scale text metadata required by runtime/TTS.
- `ReaderSession` mutable state should reference that prepared document rather than own a second independently cloned copy where practical.
- Runtime/UI projections should share immutable document-owned state by handle rather than forcing duplicate document vectors.
- Swapping/adopting the trusted document on egui must be O(1) or bounded to current-page-local state.
- If an old document-scale payload must be destroyed, arrange for its final ownership/drop to occur off egui.

Do not simply hide work by disabling an instrumentation counter.

### Required bounded-adoption regression

Add production-path instrumentation/diagnostics that can detect the actual commit thread and document-scale lifecycle operations.

Prove:

- no `ReaderSession::snapshot()` / `snapshot_internal()` document assembly executes on egui during enrichment;
- no full live `ReaderSession` clone is required on egui for worker publication;
- no document-wide canonical sentence clone/flatten happens on egui;
- no document-wide canonical sentence vector is destroyed on egui;
- no cache filesystem/serialization work occurs on egui;
- commit cost remains bounded when fixture page/sentence count is multiplied substantially.

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

Preserve A4 behavior:

- cache identity includes source/content identity plus explicit extraction revision;
- cache lookup/read/parse/validation occurs off egui;
- warm trusted cache avoids native re-extraction;
- corrupt/stale cache falls through safely to native extraction;
- cache persistence stays off egui;
- UI-facing metadata/text enqueue cannot block egui on channel capacity;
- raster textures remain ephemeral;
- Current and Nearby raster requests preempt background text chunks;
- cooperative extraction uses substantially fewer native document opens than page count;
- rapid visual use may temporarily starve text enrichment; visual responsiveness wins.

## Required deterministic coverage

At minimum prove:

- one process-wide Pdfium owner serves metadata/raster/text;
- Current raster preempts text;
- Nearby/visible raster preempts text;
- cooperative extraction avoids one-open-per-page amplification;
- native/document-scale preparation stays off egui;
- cache load/parse and persistence stay off egui;
- UI-facing request submission cannot block on full native channels;
- trusted enrichment commit preserves newer mutable live state that changed after preparation began;
- no stale worker snapshot can overwrite current page/settings/playback state;
- trusted enrichment commit does not construct a document-scale snapshot on egui;
- trusted enrichment commit does not clone/drop/flatten document-scale canonical text on egui;
- production render-only PDF policy promotes correctly;
- Text-only works after trusted adoption;
- FullText search finds later-page-only queries and navigates correctly;
- ordinary TTS uses existing first-sample semantics;
- visual sentence sync remains disabled without geometry;
- trustworthy fixture passes trust gate;
- empty/image-only/garbage/noise fixtures degrade;
- visual open succeeds independently of enrichment/cache outcome;
- trusted adoption preserves native page count/current page;
- sentence provenance remains deterministic;
- source-switch stale results are ignored;
- trusted cache is reused on reopen;
- corrupt/stale cache is safe;
- Goal 0019/0020 regressions remain green;
- representative EPUB visual/TTS regressions remain green.

## Validation / handoff

Run focused tests plus serialized workspace tests as appropriate, `cargo check --workspace`, `cargo build --workspace`, repo-native Windows QA preparation, `git diff --check`, hosted `native-workspace`, and hosted renderer/native-PDF capability coverage.

Do not terminalize or signal Goal achieved until the hosted workflow for the substantive A5 implementation lineage is green.

Update `docs/work/reports/0022.md` with A5 evidence. Move ready -> active -> done normally, push before terminal signaling, restore shared checkout to `main`, and do not request human QA. The director reviews first.
