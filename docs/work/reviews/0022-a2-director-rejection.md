# Goal 0022 A2 — director rejection before real-desktop QA

## Decision

**REJECTED BEFORE HUMAN QA.** Do not integrate the Codex branch and do not ask the human to test it.

A2 materially improves the design: Quack-check/Python/Docling/OCR remain quarantined; text extraction is resumable; a new Current raster can preempt the background text job; document-scale sentence preparation and cache persistence moved off the egui thread; and hosted Windows `native-workspace` plus `hosted-renderer-probe` are green on substantive commit `c2b0e64f3f15f9f9fd746d490666beca30e19e41`.

Three production-path defects still violate the Goal-0022 contract.

## Blocker 1 — trusted text is adopted, but the production PDF policy remains render-only

The visual-first PDF session is created with an empty transcript and therefore a render-only runtime policy: Text-only disabled, search disabled, and `tts_allowed = false`.

A2's `apply_prepared_pdf_embedded_text()` swaps trusted page text/sentences/counts into the live `ReaderSession`, but it does **not** promote the PDF runtime policy after trust succeeds.

That means the production session can contain trusted native text while the ordinary reader machinery still refuses to use it:

- `toggle_text_only()` returns early because `pdf_text_only_allowed()` is false;
- `tts_play()` / play-from-page / play-from-highlight return early because `pdf_tts_allowed()` is false;
- search preparation starts from the old render-only search policy.

The existing adoption test does not catch this because its synthetic PDF session has no production render-only policy, so the `unwrap_or(true)` paths make the test permissive.

### Required A3 correction

Trusted native embedded-text adoption must atomically promote the PDF session into an explicit **trusted text / no visual sentence geometry yet** policy state.

That state must enable canonical Text-only, full-text search, bookmarks/canonical sentence ownership as appropriate, and ordinary Windows TTS while keeping visual sentence highlighting/sync explicitly disabled until the later geometry goal.

Do not fake sentence rectangles and do not imply exact visual sync merely because text is trusted.

Add a production-shaped regression that begins from the real visual-first render-only PDF policy, adopts trusted prepared native text, and proves Text-only, search, and TTS become allowed while sentence-overlay sync remains disabled.

## Blocker 2 — the egui commit still constructs and drops document-scale state

A2 correctly precomputes page text/sentences/counts off-thread, but `apply_prepared_pdf_embedded_text_event()` still calls `session.snapshot(...)` on the egui thread.

`ReaderSession::snapshot()` flattens and clones `raw_page_sentences` across **the entire document** into `canonical_sentences`, so a large PDF still performs O(document) allocation/copy work during the UI-thread adoption commit. This is the exact full-snapshot class that the A2 contract forbids.

The prepared event also carries a duplicate full `cache_artifact` after cache persistence has already completed. The UI does not use that artifact; dropping the event can therefore destroy another document-scale `Vec<String>` / hint payload on the egui thread.

### Required A3 correction

Make the trusted-text UI commit genuinely bounded.

- Do not call the ordinary document-scale `ReaderSession::snapshot()` from the PDF enrichment commit path.
- Publish a bounded runtime/session patch or an already-prepared projection whose expensive document-owned fields were constructed off-thread.
- Move, share, or retain precomputed document-owned vectors without re-cloning all canonical sentences on egui.
- Do not carry a duplicate cache artifact through the UI event after off-thread persistence; keep/drop document-scale cache payloads on the worker side.
- Avoid fresh filesystem/config loading on the adoption commit path when the app already owns the needed normalizer/config state.

Add instrumentation proving the PDF enrichment commit does not increment the full snapshot-construction counter and does not perform document-scale cache payload destruction/persistence on egui.

## Blocker 3 — only `Current` rasters preempt text; other visible raster work can still starve

A2's native worker calls `take_current_render()` before taking the next text page. That helper only considers `PdfRequestPriority::Current`.

`Nearby` requests remain queued while an active text job advances page after page. In the continuous reader, non-anchor pages that are already visible (for example the adjacent page at a seam) can therefore remain unrendered behind the background text job.

The contract says **current/visible raster responsiveness** outranks background text enrichment. The new 40-page probe proves Current preemption only; it does not exercise queued Nearby/adjacent-visible work.

### Required A3 correction

Before each background text unit/chunk, service any pending raster work through the normal scheduler. Preserve the scheduler's Current-over-Nearby ordering, but do not let queued visible/Nearby raster work sit behind the remainder of a text job.

Add deterministic coverage where a text job is in progress, a Nearby raster is queued with no Current raster pending, and the raster completes before text terminalization.

## Efficiency note promoted into A3

The cooperative job currently reopens/reparses the PDF for every single text page. On a 638-page book that can mean hundreds of native document opens.

Keep the cooperative yield property, but avoid an O(page-count) sequence of full document opens. Prefer a small bounded text chunk per native document open (or an equivalent resumable owner-local strategy) so the worker can yield frequently without reparsing the source once per page.

## Preserved A2 work

A3 should preserve:

- no Quack-check/Python/Docling/OCR in the native trustworthy-text path;
- one process-wide Pdfium owner;
- visual source open independent of enrichment;
- source/generation stale safety;
- conservative trust gate;
- native-page-aligned canonical text;
- off-thread document preparation and cache persistence;
- versioned cache reuse;
- existing first-sample Windows TTS ownership;
- Goal 0019/0020 continuous PDF behavior;
- representative EPUB/TTS behavior;
- green hosted Windows validation discipline.

## Acceptance consequence

Goal 0022 remains open. A2 is not integrated. A3 must make trusted text functionally usable under production policy, make the egui adoption commit truly bounded, and give all visible raster work priority over background enrichment before real-desktop QA is authorized.
