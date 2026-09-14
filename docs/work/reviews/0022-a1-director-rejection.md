# Goal 0022 A1 — director rejection before real-desktop QA

## Decision

**REJECTED BEFORE HUMAN QA.** Do not integrate the Codex branch and do not ask the human to test it.

The A1 branch establishes several useful pieces of the native embedded-text path: typed completion events, cache reuse, page-aligned session adoption, a conservative trust gate, and no production dependency on Quack-check/Python/Docling/OCR. Hosted Windows workflow `34893256515` is green on substantive correction commit `acf26e967ffde48a489999742e3194f1764aa3eb`.

Two architecture defects still violate the Gate-4 contract and the project's hard render-thread rule.

## Blocker 1 — whole-document text extraction monopolizes the only Pdfium owner

`PdfNativeService` correctly remains the one process-wide Pdfium owner, but the new text request is executed as one monolithic `extract_embedded_text()` operation. That method opens the document and iterates `0..pages.len()`, extracting every page before returning.

The service worker checks metadata, then embedded-text work, then raster scheduling. Once the text request begins, the sole Pdfium worker cannot service current/visible raster requests until extraction of the entire document finishes.

That topology is unacceptable for the physically accepted continuous reader. On a large text-bearing PDF, cold enrichment can delay first/next visible page rasters or leave newly scrolled pages waiting behind hundreds of text pages. The UI thread may remain responsive, but the visual PDF is no longer independently responsive while enrichment runs.

This directly violates the Gate-4 invariant that visual browsing remains authoritative and usable before/during/after text enrichment.

The current hosted lifecycle test uses a two-page PDF and therefore does not exercise this starvation class.

### Required A2 correction

Keep one Pdfium owner, but make embedded-text extraction cooperative/bounded under that owner. Acceptable shapes include a resumable page/chunk text job or equivalent arbitration where current/visible raster work can preempt or interleave with text extraction. Do not create a second Pdfium binding merely to gain concurrency.

Raster/current-page work must have explicit priority over background text enrichment. A large cold text extraction must not block visible-page rendering for the duration of the whole document.

Add deterministic service-level coverage proving that a long/incremental text-enrichment job is in progress, a new current raster request arrives, and the raster completes before the entire text job finishes.

## Blocker 2 — document-scale adoption and cache IO run on the egui thread

`handle_effect_events()` receives `PdfEmbeddedTextCompleted` on the egui app path and calls `apply_pdf_embedded_text_event()` directly. The accepted event then performs document-scale work synchronously:

- clones the full `page_texts` payload;
- calls `ReaderSession::adopt_pdf_embedded_text()`, which joins all page text, splits sentences across every PDF page, builds page sentence/word counts, and rebuilds anchor maps;
- constructs a full `ReaderSnapshot`;
- splits every page into sentences a second time to build cache hints;
- calls `FilesystemCacheService::persist_pdf_render_precomputed_state()`, which performs synchronous filesystem persistence.

This is exactly the class of heavy/blocking work that must never execute on the render/UI thread.

### Required A2 correction

Move canonical PDF text preparation and durable cache serialization/persistence off the egui thread. The worker/effect side should prepare a typed, immutable adoption payload containing the already-normalized page text, sentence ownership/provenance/counts/hints, and any cache artifact needed for persistence.

The egui-thread commit step must be bounded: validate source identity/revision, swap/apply precomputed state, publish a bounded reader update, and return. It must not resplit the entire PDF, rebuild document-scale indexes, or write files.

Cache persistence must be dispatched through an off-thread effect/worker path. Avoid a second full-document sentence-splitting pass merely to produce cache hints.

Add a production-path test/diagnostic proving document-scale preparation and cache persistence are not executed on the egui thread.

## Preserved A1 work

A2 should preserve the parts that are directionally correct:

- no Quack-check/Python/Docling/OCR in the native trustworthy-text path;
- one process-wide Pdfium owner;
- visual source open independent of enrichment failure;
- typed `PdfEmbeddedTextCompleted` event boundary;
- source/generation stale rejection;
- page-aligned native PDF text domain;
- conservative trust gate;
- existing canonical Windows TTS/first-sample semantics;
- versioned native-text cache reuse;
- Goal 0019/0020 continuous PDF behavior and EPUB/TTS regressions.

## Acceptance consequence

Goal 0022 remains open. A1 is not integrated. A2 must correct worker arbitration and UI-thread ownership before real-desktop QA is authorized.
