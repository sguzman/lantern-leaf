# 0022 — Native PDF embedded-text / TTS trustworthy path — A4

## Status

**DONE — A4 CORRECTION AFTER A3 DIRECTOR REJECTION**

Read first:

- `docs/work/reviews/0022-a1-director-rejection.md`
- `docs/work/reviews/0022-a2-director-rejection.md`
- `docs/work/reviews/0022-a3-director-rejection.md`
- `docs/architecture/pdf-text-recovery-boundary-2026-09.md`
- `docs/work/reviews/0020-a3-real-desktop-acceptance.md`

Goal 0019/0020 remain physically accepted and authoritative. Goal 0022 must enrich the working continuous native PDF reader without weakening visual responsiveness or the hard no-heavy/blocking-work-on-egui rule.

## Outcome

For PDFs with trustworthy embedded text, LanternLeaf asynchronously obtains or reuses page-aligned native text, promotes the canonical session into trusted Text-only/search/TTS capability while exact visual sentence geometry remains disabled, preserves native page identity, and keeps all cache/native/document-scale work off the render thread.

Visual PDF browsing remains independently usable before, during, after, or despite enrichment/cache failure.

Exact spoken overlays, hostile/mixed recovery, Quack-check, Python, Docling, and OCR remain out of scope.

## Non-negotiable architecture

- Native Rust + `eframe`/`egui` + bundled/native Pdfium only.
- Exactly one process-wide `PdfNativeService` / one Pdfium owner.
- No second Pdfium binding for text concurrency.
- No Python, Quack-check, Docling, OCR, or `scripts/quack-check/*` in the Goal-0022 production path.
- Visual source open never waits for text enrichment or text-cache IO.
- Current and already-visible raster responsiveness outrank background text extraction.
- Heavy/blocking/document-scale work never executes on the egui/render thread.
- UI-facing native/cache request submission must itself be nonblocking.
- Extraction/adoption/cache results are source/generation/revision stale-safe.
- PDF canonical text preserves native PDF page identity; do not line-count-repaginate it.
- Existing backend-neutral / first-sample Windows TTS ownership remains authoritative.

## Preserve accepted A1/A2/A3 direction

Preserve:

- typed native embedded-text and prepared-result event boundaries;
- one process-wide native Pdfium worker;
- cooperative/resumable native text extraction;
- source-generation cancellation;
- deterministic conservative trust gate;
- page-aligned prepared canonical text and sentence provenance;
- trusted-text runtime policy promotion to Text-only/search/TTS with exact visual sync disabled;
- off-thread document-scale sentence/count/anchor preparation;
- off-thread cache persistence;
- Current and Nearby raster preemption;
- bounded eight-page native text chunks or an equivalent proven bounded strategy;
- versioned native-text cache identity;
- Goal 0019/0020 visual/viewport/zoom behavior;
- representative EPUB visual/TTS behavior.

## A4 correction 1 — move native-text cache load/parse completely off egui

A3 still calls `load_pdf_render_precomputed_state()` synchronously from `update_pdf_render_state()` on PDF source change.

That is forbidden.

Required behavior:

1. On PDF source change, egui may enqueue a bounded cache lookup request and immediately continue rendering.
2. Cache filesystem read, parse, validation, source-identity checks, and document-scale allocation happen off-thread.
3. A valid cache hit enters the same stale-safe trusted-text preparation/adoption pipeline as native extraction.
4. A cache miss/corrupt/stale result launches native extraction without blocking egui.
5. Cache failure never affects visual PDF usability.
6. Cache load and cache persistence are both worker-owned.

Add a production-path test/diagnostic proving a warm cached PDF open performs no native-text cache filesystem/parse work on the egui thread.

## A4 correction 2 — replace pseudo-bounded snapshot publication with a truly bounded adoption patch

A3's `snapshot_with_prepared_canonical_sentences()` still calls `snapshot_internal()` and merely disables the snapshot counter. The underlying path still performs work that scales with document size, including page-count/stat scans and document-vector cloning.

Required behavior:

- Do not call `ReaderSession::snapshot()` or `snapshot_internal()` from trusted PDF enrichment publication.
- The worker owns all document-scale prepared state.
- The egui adoption commit validates source/generation/revision/page count/trust, moves/swaps already-prepared session state, publishes a bounded runtime patch/projection, and returns.
- No operation in this commit may scale with total PDF pages/sentences.
- Do not clone full `page_sentence_counts`, full search-result vectors, canonical sentence vectors, or other document-wide payloads on egui.
- If runtime state needs document-owned data, share immutable prepared state (`Arc` or equivalent) or publish handles/revisions plus current-page-local projection.
- Current page, native page count, Text-only capability, search capability, TTS capability/state, settings, and current-page-local text/sentence data must update correctly.
- Do not load config/normalizer/filesystem state from the adoption commit.

Replace the counter-only test with instrumentation that would fail if snapshot assembly, document-wide stats scans, or document-vector clones run during egui adoption.

## A4 correction 3 — make advertised FullText PDF search actually document-wide

Trusted native PDF policy advertises `PdfSearchPolicy::FullText`, but A3 still searches only `current_sentences()`.

Required semantics:

- FullText search operates over the accepted canonical PDF sentence domain across all native pages.
- Search results carry deterministic native-page + page-local/canonical sentence provenance sufficient for navigation.
- Search Next/Previous can move to the owning native page and sentence.
- Native page domain remains authoritative.
- No guessed visual geometry is required.
- No line-count repagination.

Add a regression where the query exists only on a later PDF page while page 1 is current; prove search finds it and navigation lands on the correct native page/canonical sentence.

## A4 correction 4 — UI-facing native request submission must be nonblocking

`metadata_async()` and `embedded_text_async()` currently use blocking `SyncSender::send()` calls.

Required behavior:

- Calls reachable from egui/source-change/update paths must not block waiting for bounded native request-channel capacity.
- Use `try_send`, a dedicated dispatcher worker, or an equivalent nonblocking enqueue contract.
- Saturation must be explicit and safe: coalesce, retry later, supersede stale work, or return a typed busy/deferred result.
- Source generation/cancellation semantics remain authoritative.
- Do not silently drop the current source's only enrichment request without a deterministic retry path.

Add a saturation regression showing a full native text/metadata request queue cannot stall the caller/UI thread.

## Trusted-text policy contract

After trusted adoption:

- Text-only is allowed;
- document-wide canonical search is allowed;
- ordinary Windows TTS is allowed through existing canonical machinery;
- native `pdf_page_count` and current page remain authoritative;
- bookmark/canonical sentence ownership remains native-page aligned;
- visual sentence highlighting remains disabled;
- `pretty_sync_enabled = false` and `exact_sentence_sync = false` until real sentence geometry exists;
- no guessed rectangles or fake visual sync.

Rejected/untrusted text must remain visual-only.

## Cache / extraction contract

- Cache identity includes source/content identity plus explicit extraction revision.
- Warm trusted cache avoids native re-extraction.
- Corrupt/stale cache is ignored/rebuilt safely.
- Raster textures remain ephemeral.
- Current and Nearby raster requests preempt background text chunks.
- Cooperative extraction must use substantially fewer native document opens than page count.
- Rapid visual use may temporarily starve text enrichment; visual responsiveness wins.

## Required deterministic coverage

At minimum prove:

- one process-wide Pdfium owner serves metadata/raster/text;
- Current raster preempts text;
- Nearby/visible raster preempts text;
- cooperative extraction avoids one-open-per-page amplification;
- native/document-scale preparation stays off egui;
- cache load/parse stays off egui;
- cache persistence stays off egui;
- UI-facing request submission cannot block on full native channels;
- trusted enrichment commit does not call snapshot/snapshot_internal and does not perform O(document) work;
- production render-only PDF policy promotes correctly;
- Text-only works after trusted adoption;
- FullText search finds a later-page-only query and navigates correctly;
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

Do not terminalize or signal Goal achieved until the hosted workflow for the substantive A4 implementation lineage is green.

Update `docs/work/reports/0022.md` with A4 evidence. Move ready -> active -> done normally, push before terminal signaling, restore shared checkout to `main`, and do not request human QA. The director reviews first.
