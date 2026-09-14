# PDF Renderer Contract

## Ownership

- The **source PDF** is the visual document of record.
- Native page rendering/metadata are owned by the process-wide `PdfNativeService` / single Pdfium owner.
- `ReaderSession.current_page` uses the native PDF page domain.
- Once trustworthy PDF text is accepted, canonical `tts_text` / canonical sentences own TTS, search, text-only presentation, and sentence-level playback identity.
- Canonical text is an enrichment of the visual PDF; failure to obtain text must not make the visual source fail.

## Renderer

The production desktop PDF renderer is native Rust + `eframe`/`egui` + bundled/native Pdfium through `pdfium-render` / `pdfium-auto`.

No production PDF rendering path is owned by pdf.js, a browser DOM, WebView, or Tauri.

The native service owns:

- Pdfium initialization;
- open/parse validation;
- native page count and page dimensions;
- page rasterization and bitmap conversion;
- future native text/geometry requests that require Pdfium.

All heavy/native PDF work runs off the egui/render thread.

## Continuous viewport

The accepted native viewport is continuous and virtualized.

- Wheel/trackpad scrolling crosses page boundaries normally.
- Adjacent pages may be partially visible at the same time.
- Actual visible pages feed the authoritative render planner.
- Near-visible overscan is bounded.
- Far pages do not consume unbounded raster/texture work.
- Current-page ownership is derived deterministically from viewport position.
- Previous/Next/SetPage are jumps into the continuous stack, not page swaps.
- High zoom supports horizontal as well as vertical navigation.
- Fit Width, Fit Page, Reset/100%, and bounded manual zoom are native viewport operations.

## Raster identity and residency

Planning, scheduling, native requests, cache identity, and presentation share the same `PdfRenderSpec` / `PdfRenderKey` identity.

Render identity includes source, generation/render-spec revision, page, and final quantized raster dimensions. Stale source/generation/spec results must never become authoritative.

Visible/current pages are protected by deterministic bounded residency rules. New viewport anchors supersede obsolete queued nearby work while in-flight stale completions are rejected by identity.

## Text and geometry confidence

PDF text geometry is a quality-classified signal, not an unconditional source of truth.

Fallback order remains:

1. exact sentence geometry;
2. fuzzy/local sentence geometry;
3. paragraph/block geometry;
4. page-level location;
5. render-only / no sync.

Low-confidence evidence must degrade downward rather than pretending to provide exact sync.

The Gate-4 recovery boundary is defined in `docs/architecture/pdf-text-recovery-boundary-2026-09.md`.

## Cache contract

Durable cache artifacts may store accepted canonical PDF text, page/sentence provenance, sync metadata, sentence maps, OCR/recovery alignment artifacts, and versioned precomputed state under the hashed source cache directory.

Native viewport raster textures are ephemeral/bounded unless a later measured requirement justifies durable raster caching.

Corrupt or version-incompatible PDF text/sync artifacts are removed or ignored and rebuilt non-destructively. Their failure must not invalidate the native visual reader.
