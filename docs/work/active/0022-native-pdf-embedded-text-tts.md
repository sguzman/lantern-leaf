# 0022 — Native PDF embedded-text / TTS trustworthy path — A3

## Status

**READY — A3 CORRECTION AFTER A2 DIRECTOR REJECTION**

Read first:

- `docs/work/reviews/0022-a1-director-rejection.md`
- `docs/work/reviews/0022-a2-director-rejection.md`
- `docs/architecture/pdf-text-recovery-boundary-2026-09.md`
- `docs/work/reviews/0020-a3-real-desktop-acceptance.md`

Goal 0019/0020 remain physically accepted and authoritative. Goal 0022 must enrich that working continuous native PDF reader without weakening visual responsiveness or the hard no-heavy-work-on-egui rule.

## Outcome

For PDFs with trustworthy embedded text, LanternLeaf asynchronously obtains page-aligned native text through the existing process-wide Pdfium owner, promotes the session into an explicit trusted-text/no-geometry policy, enables Text-only/search/ordinary Windows TTS, preserves native page identity, and reuses a durable versioned text cache.

Visual PDF browsing remains independently usable before, during, after, or despite text enrichment failure.

Exact visual spoken overlays, hostile/mixed recovery, Quack-check, Python, Docling, and OCR remain out of scope.

## Non-negotiable architecture

- Native Rust + `eframe`/`egui` + bundled/native Pdfium only.
- Exactly one process-wide `PdfNativeService` / one Pdfium owner.
- No second Pdfium binding for text concurrency.
- No Python, Quack-check, Docling, OCR, or `scripts/quack-check/*` in the Goal-0022 production path.
- Visual source open never waits for text enrichment.
- Current and already-visible raster responsiveness outrank background text extraction.
- Heavy/blocking/document-scale work never executes on the egui/render thread.
- Extraction/adoption/cache results are source/generation/revision stale-safe.
- PDF canonical text preserves native PDF page identity; do not line-count-repaginate it.
- Existing backend-neutral / first-sample Windows TTS ownership remains authoritative.

## Preserve accepted A1/A2 direction

Preserve the good work already on the Goal-0022 branch:

- typed `PdfEmbeddedTextCompleted` / prepared-result boundaries;
- native Pdfium embedded-text extraction rather than Python extraction;
- one process-wide native Pdfium worker;
- cooperative/resumable background text job;
- source-generation cancellation;
- deterministic conservative trust gate;
- page-aligned prepared canonical text and sentence provenance;
- off-thread document-scale sentence/count/anchor preparation;
- off-thread durable text-cache persistence;
- versioned native-text cache reuse;
- Goal 0019/0020 visual/viewport/zoom behavior;
- representative EPUB visual/TTS behavior.

## A3 correction 1 — trusted text must actually promote production PDF capabilities

The production visual-first PDF session begins with:

- `PdfGeometryMode::RenderOnlyNoSync`;
- `PdfSyncStrategy::RenderOnly`;
- Text-only disabled;
- search disabled;
- `tts_allowed = false`.

A2 adopts page text but leaves that policy unchanged, so the normal reader still rejects Text-only and TTS.

When a native embedded-text result passes the trust gate and page-count/source identity validation, atomically promote the live session into an explicit **trusted canonical text, no exact visual geometry yet** state.

Required semantics after trusted adoption:

- Text-only is allowed and shows the accepted canonical PDF text;
- search is allowed over the trusted canonical PDF text;
- ordinary Windows TTS is allowed through the existing reader/TTS pipeline;
- native page count/current page/bookmark ownership remain unchanged;
- sentence -> native page provenance remains deterministic;
- visual sentence highlighting stays disabled because exact geometry has not yet been implemented;
- `pretty_sync_enabled` / exact sentence sync remain false unless real geometry evidence exists;
- no guessed rectangles or fake sentence overlays are introduced.

Do not merely set a flag in UI code. The canonical `ReaderSession` PDF runtime policy/capability state must become truthful so existing `toggle_text_only()`, search, `tts_play()`, play-from-page/highlight, seek/repeat, persistence, and snapshots observe the same ownership decision.

### Required policy regression

Start from the real production-shaped visual-only PDF policy, then apply a trusted prepared native-text payload and prove:

- Text-only becomes allowed;
- full canonical search becomes allowed;
- TTS becomes allowed;
- ordinary TTS commands enter the existing canonical playback state;
- native page domain remains unchanged;
- sentence overlay/highlight sync remains disabled;
- rejected/untrusted text does not promote policy.

Do not use a synthetic PDF session with `pdf_runtime_policy = None` as the only acceptance evidence.

## A3 correction 2 — make the egui adoption commit genuinely bounded

A2 moved sentence preparation off-thread, but its egui completion still calls ordinary `ReaderSession::snapshot()`. That function flattens/clones canonical sentences across the entire document, so large-PDF enrichment still performs O(document) allocation/copy work on the render thread.

A2 also sends a duplicate full cache artifact through `PdfEmbeddedTextPreparedEvent` after the cache has already been persisted. The UI does not need that payload and should not perform document-scale destruction of it.

Required A3 behavior:

1. The worker/effect side owns all document-scale preparation.
2. The egui commit validates source/generation/revision/page count/trust and swaps already-prepared session state with bounded work.
3. The PDF enrichment commit must **not** call the ordinary document-scale `ReaderSession::snapshot()` path.
4. Runtime/UI publication must use a bounded patch/projection or an already-prepared projection whose expensive document fields were built off-thread.
5. Do not clone/flatten every canonical PDF sentence on egui merely to publish enrichment.
6. Do not send the already-persisted duplicate cache artifact through the UI event unless the UI genuinely needs it. Prefer leaving/dropping it on the worker side.
7. Cache serialization/write remains worker-only.
8. Reuse the app-owned normalizer/config state or otherwise avoid fresh filesystem/config loading from the enrichment commit path.

### Required bounded-commit regression

Use the existing snapshot-construction instrumentation or equivalent production-path diagnostics to prove:

- trusted PDF enrichment commits without invoking the full `ReaderSession::snapshot()` construction path on egui;
- no document-scale cache serialization/write occurs on egui;
- no duplicate full cache payload is destroyed on egui as part of normal trusted adoption;
- the runtime/UI sees the needed current-page/Text-only/search/TTS capability update after the bounded commit.

## A3 correction 3 — all pending visual raster work outranks text enrichment

A2 only checks `PdfRequestPriority::Current` before the next text unit. `Nearby` requests can remain queued while the text job runs to completion.

For the continuous reader, Nearby includes non-anchor page work needed for visible/adjacent/overscan presentation. A page already visible at a seam must not wait behind hundreds of background text pages.

Before each background text unit/chunk:

- inspect the normal raster scheduler;
- if any raster request is queued, service raster work first;
- preserve the scheduler's existing Current-over-Nearby ordering;
- then allow bounded background text progress when no raster is waiting.

Rapid visual navigation may starve background text temporarily. That is acceptable: visual responsiveness is authoritative.

### Required arbitration regressions

Keep the existing Current-preemption probe and add a deterministic Nearby/visible case:

- native text job is in progress and incomplete;
- no Current raster is pending;
- submit a Nearby raster request;
- Nearby raster completes before text terminalization;
- text then resumes/completes;
- metadata/raster/text still share the same native Pdfium owner/thread.

## A3 correction 4 — cooperative extraction must not reopen the document once per page

A2 yields between pages but `extract_embedded_text_page()` calls `load_pdf_from_file()` for every page. On a 638-page book this can mean hundreds of full native document opens/reparses.

Preserve cooperative arbitration while reducing reopen amplification.

Acceptable implementation shapes include:

- a small bounded page chunk per document open;
- an owner-local resumable text document handle if the Pdfium lifetime model permits it cleanly;
- another bounded strategy that does not require one full document open per page.

The worker must still yield frequently enough that pending raster work is serviced promptly.

Add a deterministic diagnostic/test showing the long text probe uses substantially fewer document opens than page count while retaining Current/Nearby preemption.

## Trust gate

Keep a deterministic conservative embedded-text trust assessment with named/tested thresholds or constants. At minimum reject:

- absent/image-only text;
- clearly insufficient text-bearing page coverage;
- obvious replacement/control garbage;
- obvious duplicate/noise pathologies.

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

Cache identity must include source/content identity and explicit native-text extraction revision. Reopen should reuse trusted page text without rerunning native extraction. Corrupt/version-stale artifacts must be ignored/removed/rebuilt non-destructively. Raster textures remain ephemeral.

Cache load/write must not block egui. Cache persistence failure must not break the active visual or trusted-text session.

## Required deterministic coverage

At minimum prove:

- one process-wide Pdfium owner serves metadata, raster, and native text;
- Current raster preempts in-progress text extraction;
- Nearby/visible raster preempts in-progress text extraction;
- cooperative extraction does not reopen the PDF once per page;
- native/document-scale heavy work stays off egui;
- document-scale canonical preparation stays off egui;
- the enrichment commit does not build an ordinary full ReaderSnapshot on egui;
- cache persistence/cache-payload destruction stays off egui;
- production render-only PDF policy promotes correctly after trusted text;
- trusted Text-only/search/TTS work through normal canonical machinery;
- sentence visual highlighting remains disabled without geometry;
- trustworthy fixture passes trust gate;
- empty/image-only/garbage/noise fixtures degrade;
- visual open succeeds independently of enrichment outcome;
- trusted adoption preserves native page count/current page;
- canonical sentences map deterministically to native pages;
- PDF text is not arbitrary line-count repaginated;
- source-switch stale results are ignored;
- extraction/cache failure leaves PDF visually browsable;
- trusted cache is reused on reopen;
- corrupt/stale cache is safe;
- Goal 0019/0020 regressions remain green;
- representative EPUB visual/TTS regressions remain green.

## Validation / handoff

Run focused tests plus serialized workspace tests as appropriate, `cargo check --workspace`, `cargo build --workspace`, repo-native Windows QA preparation, `git diff --check`, hosted `native-workspace`, and hosted renderer/native-PDF capability coverage.

Do not terminalize or signal Goal achieved until the hosted workflow for the substantive A3 implementation lineage is green.

Update `docs/work/reports/0022.md` with A3 evidence. Move ready -> active -> done normally, push before terminal signaling, restore shared checkout to `main`, and do not request human QA. The director reviews first.
