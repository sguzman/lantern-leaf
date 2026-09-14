# Goal 0022 A3 — director rejection before real-desktop QA

## Decision

**REJECTED BEFORE HUMAN QA.** Do not integrate the Codex branch and do not ask the human to test it.

A3 fixes several important A2 defects: trusted native text now promotes the canonical PDF runtime policy to Text-only/search/TTS while keeping exact visual sentence sync disabled; Current and Nearby raster requests both outrank background text chunks; extraction uses bounded eight-page chunks rather than reopening once per page; the duplicate cache artifact was removed from the prepared UI event; and hosted Windows workflow `34907494360` is green on substantive commit `e7d6e537861bafeb0d479b2b82d843563abf6188`.

Three production-path defects still violate the Goal-0022 contract and the project's hard no-heavy/blocking-work-on-egui rule.

## Blocker 1 — native-text cache loading still performs synchronous filesystem/parse work on egui

`LanternLeafApp::update_pdf_render_state()` runs from the egui frame. On PDF source change it directly calls:

`self.cache_service.load_pdf_render_precomputed_state(&source_path)`

before deciding whether to use cached page text or launch native extraction.

That cache contains the document-scale native page text payload. Loading/parsing it can therefore perform filesystem IO and large TOML/document allocation synchronously on the render thread, especially on the real 638-page PDF or larger books.

The Goal-0022 cache contract explicitly requires cache **load and write** to stay off egui. A2/A3 moved writes off-thread but left the read on the frame path.

### Required A4 correction

Move native-text cache lookup/read/parse/validation to an off-thread worker/effect path. The egui source-change path may enqueue/request a lookup and continue rendering immediately, but it must not read or parse the cache synchronously.

A cache hit should produce the same typed prepared/enrichment pipeline as native extraction, with source/generation/revision stale checks before adoption. A cache miss should then launch native extraction without blocking egui.

Add production-path instrumentation proving a warm cached PDF source transition does not call native-text cache load/parse on the egui thread.

## Blocker 2 — the “bounded snapshot” path still does document-scale work on egui, and its test only disables the counter

A3 introduces `snapshot_with_prepared_canonical_sentences()` and asserts that the ordinary snapshot-construction counter does not increment. However, that method simply calls the same `snapshot_internal()` with `count_construction = false`.

The actual work still includes document-scale operations on the egui adoption path:

- `stats()` scans/sums page-count vectors up to the current page and sums all page word counts;
- `ReaderSnapshot` clones the full `page_sentence_counts` vector;
- it clones the full `search_matches` vector;
- other snapshot fields continue through the ordinary snapshot assembly path.

The canonical sentence flatten itself is correctly prepared off-thread, but turning off the instrumentation counter does not prove the commit is bounded.

### Required A4 correction

Do not route PDF enrichment publication through `snapshot_internal()` at all.

Publish a genuinely bounded runtime patch/projection containing only what changes at trusted-text adoption, for example capability/policy state plus current-page-local text/TTS/search projection and stable references/handles to document-owned prepared state.

No operation in the egui adoption commit may scale with total PDF page/sentence count. If document-owned data must be present in runtime state, prepare/share it off-thread (for example behind `Arc`/immutable prepared state) rather than cloning document vectors into a new snapshot on egui.

Replace the counter-only regression with instrumentation that measures/guards the actual bounded publication path and would fail if `snapshot_internal()`, document-wide stats scans, or document-vector clones are invoked.

## Blocker 3 — “FullText” PDF search is not actually full-document search

A3 truthfully sets `PdfSearchPolicy::FullText`, but `ReaderSession::update_search_matches()` still searches only `current_sentences()`, i.e. the current native PDF page. Its result indexes are therefore page-local.

The A3 regression searches for a term on page 1, so it does not catch this mismatch. A term that exists only on page 2+ will not be found while page 1 is current despite the session advertising FullText search.

### Required A4 correction

Make trusted native PDF `FullText` search operate over the canonical document sentence domain with deterministic native-page provenance. Search results must identify enough information to navigate to the owning native page and sentence. Do not line-count-repaginate or invent geometry.

Add a regression where the query exists only on a later PDF page, start from page 1, and prove search finds it and navigation lands on the correct native page/canonical sentence.

## Additional blocking-risk cleanup required in A4

`PdfNativeService::metadata_async()` and `embedded_text_async()` currently use blocking `SyncSender::send()` calls. They are invoked from PDF source-change/update paths. A full request channel can therefore block the caller even though the API is named async.

Convert UI-facing request submission to a nonblocking/bounded enqueue contract (`try_send`, a dispatcher worker, or equivalent). Queue saturation must degrade/retry safely rather than stall egui.

## Preserved A3 work

A4 should preserve:

- Quack-check/Python/Docling/OCR remain quarantined;
- one process-wide Pdfium owner;
- visual source open independent of text/cache enrichment;
- trusted native text promotes Text-only/search/TTS but not exact visual sentence geometry;
- source/generation stale safety;
- conservative trust gate;
- page-aligned canonical text;
- off-thread document preparation and cache persistence;
- Current and Nearby raster preemption;
- bounded eight-page native text chunks;
- versioned cache identity;
- existing backend-neutral/first-sample Windows TTS ownership;
- Goal 0019/0020 continuous PDF behavior and EPUB/TTS regressions;
- green hosted Windows validation discipline.

## Acceptance consequence

Goal 0022 remains open. A3 is not integrated. A4 must remove synchronous cache/request work from egui, replace the pseudo-bounded snapshot publication with a truly bounded commit, and make advertised FullText search actually document-wide before real-desktop QA is authorized.
