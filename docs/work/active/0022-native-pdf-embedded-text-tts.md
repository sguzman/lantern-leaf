# 0022 — Native PDF embedded-text / TTS trustworthy path — A2

## Status

**READY — A2 CORRECTION AFTER A1 DIRECTOR REJECTION**

A1 is rejected before human QA. Read `docs/work/reviews/0022-a1-director-rejection.md` first.

Goal 0019/0020 remain physically accepted and authoritative. Goal 0022 must enrich that working continuous native PDF reader without ever making text work capable of degrading visual responsiveness or blocking the egui thread.

## Outcome

For PDFs with trustworthy embedded text, LanternLeaf asynchronously obtains page-aligned native text through the existing process-wide Pdfium owner, conservatively adopts it as canonical PDF text, enables Text-only/search/ordinary Windows TTS, and reuses a versioned durable cache.

Visual PDF browsing remains independently usable before, during, after, or despite text enrichment failure.

Exact sentence overlay geometry, hostile-PDF recovery, Quack-check, Docling, and OCR remain out of scope.

## Non-negotiable architecture

- Native Rust + `eframe`/`egui` + bundled/native Pdfium only.
- Exactly one process-wide `PdfNativeService` / one Pdfium owner.
- No second Pdfium binding for text concurrency.
- No Python, Quack-check, Docling, OCR, or `scripts/quack-check/*` in the Goal-0022 production path.
- Visual source open never waits for text enrichment.
- Current/visible raster responsiveness outranks background text extraction.
- Heavy/blocking/document-scale work never executes on the egui/render thread.
- Extraction/adoption/cache results are source/generation/revision stale-safe.
- PDF canonical text preserves native PDF page identity; do not line-count-repaginate the transcript.
- Existing backend-neutral/first-sample Windows TTS ownership remains authoritative.

## A2 correction 1 — cooperative text extraction under the single Pdfium owner

A1 executes whole-document `extract_embedded_text()` as one uninterrupted operation on the only Pdfium worker. This can strand current/visible raster requests behind hundreds of text pages.

Replace that with a bounded/cooperative text job owned by the same native service.

Required properties:

1. Text extraction progresses page-by-page or in small bounded chunks.
2. Between text units/chunks, the service returns to arbitration.
3. Current/visible raster requests have higher priority than background text enrichment.
4. Metadata/source-critical requests remain bounded and live.
5. Text work resumes deterministically after higher-priority raster work.
6. Cancellation/stale source changes safely abandon the old text job.
7. A large cold PDF cannot monopolize the native service until all text pages finish.
8. The final typed text result still contains source identity, source generation/revision, native page count, one text payload per native page, worker evidence, trust decision/degraded reason, and terminal success/failure.

Do not solve this by creating another Pdfium instance or another owner thread that binds Pdfium independently.

### Required arbitration regression

Add a deterministic service-level test with a deliberately multi-step/long text job:

- service already initialized/idle;
- start text enrichment;
- prove text job is not yet complete;
- submit a new `Current` raster request while enrichment is in progress;
- current raster completes before the text job terminal result;
- text job then resumes and completes;
- same owner/thread identity remains true.

The existing two-page lifecycle probe is not sufficient by itself.

## A2 correction 2 — no document-scale adoption or cache IO on egui

A1 receives `PdfEmbeddedTextCompleted` on the app/UI path and then synchronously performs full-document sentence splitting, word/sentence counts, anchor-map building, full snapshot construction, a second full-document sentence split for cache hints, and synchronous cache persistence.

That is forbidden.

Move document-scale preparation off-thread.

The worker/effect side must prepare an immutable typed adoption payload containing the information required to commit trusted PDF text without recomputing the whole document on the UI thread. At minimum this should cover, as appropriate to the existing session model:

- normalized page-aligned canonical page text;
- canonical per-page sentences;
- page sentence counts;
- page word counts;
- deterministic sentence -> native page provenance / cache hints;
- any anchor/index structures needed by the session;
- cache artifact payload and version/source identity metadata;
- trust/degraded decision.

The egui-thread completion step must be bounded:

1. validate current source + generation/revision;
2. reject stale/untrusted/mismatched page-count payloads;
3. atomically swap/apply already-prepared session text state;
4. publish only the needed reader/runtime update;
5. return promptly.

It must not:

- split every page into sentences;
- join/rebuild the entire document more than unavoidable bounded assignment/copy cost;
- rebuild document-wide indexes from raw text;
- synchronously write cache files;
- perform another full-document split solely for cache hints.

Durable cache persistence must run through an off-thread effect/worker path. Failure to persist text cache must not affect the active PDF session.

Add a production-path test or explicit thread diagnostic proving document-scale preparation and cache persistence do not execute on the egui thread.

## Preserve accepted A1 direction

A2 should retain the parts of A1 that are correct:

- typed native embedded-text result and `AppEvent::PdfEmbeddedTextCompleted` boundary;
- native Pdfium text extraction, not Python extraction;
- conservative deterministic trust gate;
- page-aligned canonical PDF session ownership;
- Text-only/search/TTS using the normal reader pipeline;
- existing first-sample speech ownership;
- no fake sentence rectangles;
- versioned cache reuse when source identity/revision/page count match;
- corrupt/stale cache cannot break visual open;
- stale source-A enrichment cannot mutate source B;
- Goal 0019/0020 continuous rendering, zoom, viewport, scheduling, and residency behavior;
- representative EPUB rendering/TTS behavior.

## Trust gate

Keep a deterministic conservative embedded-text trust assessment with named/tested thresholds or constants. At minimum reject absent/image-only text, clearly insufficient coverage, obvious replacement/control garbage, and obvious pathological duplicate/noise cases.

This goal remains binary for adoption:

- trustworthy embedded text -> canonical PDF text/TTS eligible;
- otherwise -> visual-only with explicit degraded reason.

Do not attempt mixed/scanned/hostile recovery here.

## PDF page-domain / TTS contract

Accepted text must preserve:

`native PDF page N -> canonical page text -> canonical sentences belonging to page N`

Preserve native `pdf_page_count`, current native page, bookmarks, continuous viewport position, reader settings, and source identity.

Text-only/search/Play/Pause/seek/repeat must use existing canonical ReaderSession/TTS machinery. No separate PDF TTS engine.

Exact visual spoken sentence highlighting is still a later goal.

## Cache contract

Cache identity must include source/content identity and explicit native-text extraction revision. Reopen should reuse trusted page text without rerunning native extraction. Corrupt/version-stale artifacts must be ignored/removed/rebuilt non-destructively. Raster textures remain ephemeral and are not persisted here.

Cache load/write must not block the egui thread.

## Required deterministic coverage

At minimum prove:

- same process-wide Pdfium owner serves metadata, raster, and native embedded text;
- cooperative text extraction cannot starve a new current raster request;
- text/native heavy work stays off egui;
- document-scale adoption preparation stays off egui;
- cache persistence stays off egui;
- trustworthy fixture passes trust gate;
- empty/image-only/garbage/noise fixtures degrade;
- visual open succeeds independently of enrichment outcome;
- trusted adoption preserves native page count/current page;
- canonical sentences map deterministically to native pages;
- PDF text is not arbitrary line-count repaginated;
- source-switch stale result is ignored;
- Text-only/search become meaningful after accepted enrichment;
- ordinary Windows TTS uses existing first-sample semantics;
- extraction/cache failure leaves PDF visually browsable;
- trusted cache is reused on reopen;
- corrupt/stale cache is safe;
- Goal 0019/0020 regressions remain green;
- representative EPUB visual/TTS regressions remain green.

## Validation / handoff

Run focused tests plus serialized workspace tests as appropriate, `cargo check --workspace`, `cargo build --workspace`, repo-native Windows QA preparation, `git diff --check`, hosted `native-workspace`, and hosted renderer/native-PDF capability coverage.

Do not terminalize or signal Goal achieved until the hosted workflow for the substantive A2 implementation lineage is green.

Update `docs/work/reports/0022.md` with A2 evidence. Move ready -> active -> done normally, push before terminal signaling, restore shared checkout to `main`, and do not request human QA. The director reviews first.
