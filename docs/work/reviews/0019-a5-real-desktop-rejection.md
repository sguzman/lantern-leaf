# Goal 0019 A5 — real-desktop rejection

## Decision

**REJECTED ON REAL DESKTOP — REOPEN AS A6**

A5 passed source/CI review but failed the first focused physical PDF recheck. The representative Caliberate PDF materialized successfully from the local provider cache, the shell transitioned from `Starter` to `SourceLoading`, and then no Reader appeared. The attached runtime log terminates immediately after the `SourceLoading` transition: there is no `SourceOpened`, no `CommandFailed`, no `source_open_failed`, and no Pdfium diagnostic after that point.

This is not a provider/materialization failure. The Caliberate PDF path exists and materialization completed with a cache hit before the source-open task disappeared.

## Primary diagnosis — duplicate Pdfium process ownership

A5 introduced `probe_pdf_page_count()` in `crates/lanternleaf-egui/src/pdf_renderer.rs`. That helper constructs a fresh `NativePdfRenderer::new()`, which calls `pdfium_auto::bind_bundled()` and creates a Pdfium instance.

The production application already starts a long-lived `PdfRenderWorker`; its worker thread also constructs a `NativePdfRenderer::new()` and retains it for raster work. The vendored `pdfium-auto` layer ultimately binds the same process-global PDFium library through `Pdfium::new`.

Therefore the A5 production topology can initialize/bind Pdfium once for the render worker and then attempt to initialize/bind it again from a separate source-open effect thread merely to obtain page count. That is the wrong ownership model for a native library with process-global bindings and non-thread-safe native state.

The physical failure shape is consistent with that defect: the source-open effect reaches the A5 metadata probe and then disappears before it can emit either success or failure.

## Secondary resilience defect — detached effect panic can strand UI state

`EffectDispatcher` launches planned effects in detached `thread::spawn` tasks. The handler is not wrapped in a panic boundary. If the source-open task panics, the thread simply terminates and the shell is left permanently in `SourceLoading` because no terminal `AppEvent` is emitted.

A native/library panic must never be able to strand product state indefinitely. This resilience defect must be corrected alongside the Pdfium ownership error; panic containment is not a substitute for fixing the ownership model.

## CI gap

A5's real multi-page metadata test probes Pdfium in isolation. It does not reproduce the production lifetime in which the persistent PDF render worker is already initialized before source opening performs the page-count probe.

A6 must add a production-lifecycle test/harness that starts the same native PDF service/worker used by the app, obtains metadata for a real valid multi-page PDF, and renders through that already-initialized service. The test must prove there is one native Pdfium owner rather than two independent bindings.

## A6 required correction

1. Establish a **single process-wide/native PDF service owner** for Pdfium initialization and all Pdfium calls used by LanternLeaf Gate 3: open/parse validation, page-count metadata, and page rasterization.
2. Serialize native Pdfium work through that service/worker. Do not create a second `NativePdfRenderer` or second Pdfium binding on the source-open effect thread.
3. Keep all native PDF work off the egui/render thread.
4. Give source-open metadata requests bounded/current priority so opening a PDF cannot sit indefinitely behind obsolete raster backlog.
5. Return typed metadata success/failure to source opening. Valid PDFs publish native page count; malformed/native-open failures terminate truthfully.
6. Preserve A4 visual-first behavior: Quack-check/Python/Docling/OCR/transcript recovery remain outside the visual-open critical path.
7. Preserve A1-A5 page/zoom/generation identity, real presentation-scale zoom, stale rejection, current-priority rendering, deterministic current-page-pinned residency, and explicit PDF page-domain ownership.
8. Add panic containment around detached runtime-effect execution so an unexpected panic emits a bounded terminal failure rather than leaving `SourceLoading` forever.
9. Add deterministic tests for: production service already initialized -> metadata -> page-1 render; repeated PDF opens; source switching; malformed PDF; no second Pdfium bind/owner; and synthetic effect panic -> terminal failure event.

## Physical evidence

The real-desktop log for book 725 shows:

- Caliberate materialized cache hit for `725-13e8b7a0.pdf`;
- materialization stage finished in 0 ms;
- shell transitioned `Starter -> SourceLoading`;
- log ended there with no source-open terminal event.

Goal 0019 remains open. Do not request another human PDF recheck until A6 passes director source/CI review and is integrated to `main`.
