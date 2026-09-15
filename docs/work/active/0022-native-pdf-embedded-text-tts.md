# 0022 — Native PDF embedded-text / TTS trustworthy path — A6

## Status

**READY — A6 CORRECTION AFTER A5 DIRECTOR REJECTION**

Read first:

- `docs/work/reviews/0022-a1-director-rejection.md`
- `docs/work/reviews/0022-a2-director-rejection.md`
- `docs/work/reviews/0022-a3-director-rejection.md`
- `docs/work/reviews/0022-a4-director-rejection.md`
- `docs/work/reviews/0022-a5-director-rejection.md`
- `docs/architecture/pdf-text-recovery-boundary-2026-09.md`
- `docs/work/reviews/0020-a3-real-desktop-acceptance.md`

Goal 0019/0020 remain physically accepted and authoritative. Goal 0022 must enrich that working continuous native PDF reader without weakening visual responsiveness, canonical sentence identity, or the hard no-heavy/blocking/document-scale-work-on-egui rule.

## Outcome

For PDFs with trustworthy embedded text, LanternLeaf asynchronously obtains or reuses page-aligned native text, promotes the canonical session into trusted Text-only/document-wide-search/TTS capability while exact visual sentence geometry remains disabled, preserves native page identity and current mutable reader state, and keeps all native/cache/document-scale work away from egui.

A6 closes the remaining correctness/lifecycle gaps in the shared trusted-text model:

- the prepared document is authoritative for **all** PDF global canonical sentence identity;
- search queries that exist before or change during enrichment are automatically reconciled when trusted text becomes available;
- document-scale shared state is retired so final destruction cannot accidentally occur on egui;
- the enriched-PDF runtime projection is explicitly bounded/current-page-local.

Visual PDF browsing remains independently usable before, during, after, or despite enrichment/cache/search failure.

Exact spoken overlays, hostile/mixed recovery, Quack-check, Python, Docling, and OCR remain out of scope.

## Non-negotiable architecture

- Native Rust + `eframe`/`egui` + bundled/native Pdfium only.
- Exactly one process-wide `PdfNativeService` / one Pdfium owner.
- No second Pdfium binding for text concurrency.
- No Python, Quack-check, Docling, OCR, or `scripts/quack-check/*` in the Goal-0022 production path.
- Visual source open never waits for text enrichment, text-cache IO, or search reconciliation.
- Current and already-visible raster responsiveness outrank background text extraction.
- Heavy/blocking/document-scale allocation, cloning, flattening, scanning, destruction, cache IO, and native work never execute on egui.
- UI-facing native/cache request submission remains nonblocking.
- Extraction/adoption/cache/search results are source/generation/revision stale-safe.
- PDF canonical text preserves native PDF page identity; do not line-count-repaginate it.
- Existing backend-neutral / first-sample Windows TTS ownership remains authoritative.

## Preserve accepted A1–A5 direction

Preserve:

- typed native embedded-text and prepared-result event boundaries;
- one process-wide native Pdfium worker;
- cooperative/resumable native text extraction;
- source-generation cancellation;
- deterministic conservative trust gate;
- page-aligned prepared canonical text and sentence provenance;
- shared immutable `Arc`-owned trusted PDF text document state;
- current live session authority at adoption time;
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

## A6 correction 1 — shared trusted document owns all canonical global identity

A5 routes most native-PDF sentence operations through the shared prepared document, but `ReaderSession::global_display_idx()` still uses the legacy pre-enrichment `self.page_sentence_counts` vector.

For a production visual-first PDF, that legacy vector was created from an empty transcript and is not the trusted native sentence domain.

Required A6 behavior:

1. For an enriched PDF, every local-page-sentence -> global canonical sentence calculation must use the shared trusted document.
2. Use `active_page_sentence_counts()` or the prepared document's prefix sums instead of stale legacy page counts.
3. `highlighted_canonical_idx()` must be truthful on every native page.
4. Bookmark canonical identity must use the same global domain.
5. `current_tts_audio_display_ids()` / first-sample boundaries / `apply_tts_sentence_boundary()` must agree with that domain.
6. `has_canonical_sentence_after_current()` must be correct on page boundaries and at the final sentence of the final page.
7. End-of-document TTS must stop/terminalize rather than rebuild/replay because of a falsely small canonical ID.
8. Preserve native `pdf_page_count`, page-local display identity, and exact-visual-sync-disabled policy.

### Required canonical-identity regression

Start from the actual production-shaped visual-first render-only PDF session, then attach a trusted prepared multi-page document.

Prove:

- a sentence on page 2+ maps to the expected global canonical ID;
- the same identity survives `current_tts_audio_display_ids()` and a simulated first-sample boundary;
- crossing from the final sentence on page N to page N+1 uses the correct global ID;
- the final sentence of the final native page makes `has_canonical_sentence_after_current()` false;
- the TTS runtime does not schedule/replay another canonical window after true document exhaustion;
- bookmark sentence identity/restoration uses the same trusted page/global mapping.

Audit the enriched-PDF path for any other direct use of legacy `self.page_sentence_counts`, `self.raw_page_sentences`, or pre-enrichment pagination state where the shared document should be authoritative.

## A6 correction 2 — live search query must reconcile automatically when trust arrives

A5 worker preparation captures the search query while the PDF is still render-only. At that time search capability is disabled, so the prepared document may contain no matches even if a query string already exists.

The user is allowed to type or change a query while asynchronous text extraction/preparation is running. Trusted adoption must not preserve the string while leaving results stale/empty until the user edits it again.

Required A6 behavior:

1. The **current live query at/after adoption** is authoritative.
2. If the query was already present before enrichment and remains unchanged, matches must become available automatically once trusted text is adopted.
3. If the query changed while preparation was in flight, the newer query must be evaluated, not the stale captured query.
4. Query reconciliation must not scan the whole document on egui.
5. Search Next/Previous must navigate native page/page-local provenance correctly once the result lands.
6. Source/generation/query-revision stale safety must prevent an old query result from replacing a newer one.
7. Empty query clears matches deterministically.

Acceptable implementation shapes include:

- a worker-built immutable search index embedded in the trusted document with bounded query lookup; or
- a typed off-thread search reconciliation request/result after adoption, keyed by source/generation/query revision.

Do not duplicate the entire canonical document merely to search it.

### Required search races

Production-shaped tests must cover:

- query entered before enrichment -> trusted adoption -> matching results appear automatically;
- query changed from A to B during preparation -> only B results become authoritative;
- query cleared during preparation -> stale A/B result cannot reappear;
- query matches only a later native page -> Search Next jumps to the correct native page/local sentence;
- no document-wide search scan occurs on egui.

## A6 correction 3 — explicit off-egui retirement of document-scale shared payloads

A5 retires the replaced top-level prepared document Arc on a detached thread, but runtime projections hold cloned inner Arcs. If those projection Arcs become the last owners, replacing a runtime snapshot on egui can still become the final destructor of document-scale vectors.

Required A6 behavior:

- document-scale trusted-PDF payload ownership must have an explicit retirement path;
- final destruction of canonical sentence/page/index/search payloads must not occur on egui during re-adoption, source switch, or runtime projection replacement;
- do not solve this by cloning document-scale data;
- prefer projections to share a top-level immutable document handle or use a deferred-retirement queue/guard whose last-drop thread is testable;
- current-page-local strings/vectors may be created/dropped on egui; document-wide payloads may not.

Add instrumentation or a deterministic destructor sentinel proving the final document-scale payload drop occurs off the egui/test commit thread during replacement/close.

## A6 correction 4 — make the bounded enriched-PDF projection explicit

A5's `ReaderSession::snapshot()` currently short-circuits enriched PDFs to `snapshot_pdf_enrichment()`. That implementation is bounded/current-page-local and shares document-wide Arcs, but the enrichment commit still calls the generic snapshot API.

Make the contract explicit enough that a future refactor cannot accidentally send enrichment through ordinary document assembly.

Required:

- expose/use a named bounded enriched-PDF projection or equivalent bounded publication API in the trusted-text commit path;
- it may clone current-page text/current-page sentence rows/settings/playback state;
- it must share document-wide canonical/page-count/search state by handle;
- it must not increment ordinary document-snapshot construction instrumentation;
- it must not scan/flatten the total PDF;
- runtime projection must reflect the **current live mutable session** at commit time.

Do not remove the ordinary snapshot API needed by non-PDF/other flows.

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

Preserve A5 behavior:

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
- trusted enrichment publication uses the explicit bounded PDF projection and does not construct an ordinary document-scale snapshot;
- document-scale shared payload final destruction occurs off egui;
- production render-only PDF policy promotes correctly;
- local -> global canonical sentence identity is correct on later pages;
- first-sample canonical identity remains correct across native page boundaries;
- final-page final-sentence exhaustion is truthful and does not replay/replan;
- pre-enrichment live search query reconciles automatically;
- changed/cleared-in-flight queries are stale-safe;
- FullText search finds later-page-only queries and navigates correctly;
- Text-only works after trusted adoption;
- ordinary TTS uses existing first-sample semantics;
- visual sentence sync remains disabled without geometry;
- trustworthy fixture passes trust gate;
- empty/image-only/garbage/noise fixtures degrade;
- visual open succeeds independently of enrichment/cache/search outcome;
- trusted adoption preserves native page count/current page;
- sentence provenance remains deterministic;
- source-switch stale results are ignored;
- trusted cache is reused on reopen;
- corrupt/stale cache is safe;
- Goal 0019/0020 regressions remain green;
- representative EPUB visual/TTS regressions remain green.

## Validation / handoff

Run focused tests plus serialized workspace tests as appropriate, `cargo check --workspace`, `cargo build --workspace`, repo-native Windows QA preparation, `git diff --check`, hosted `native-workspace`, and hosted renderer/native-PDF capability coverage.

Do not terminalize or signal Goal achieved until the hosted workflow for the substantive A6 implementation lineage is green.

Update `docs/work/reports/0022.md` with A6 evidence. Move ready -> active -> done normally, push before terminal signaling, restore shared checkout to `main`, and do not request human QA. The director reviews first.
