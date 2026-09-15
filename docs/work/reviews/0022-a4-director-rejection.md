# Goal 0022 A4 — director rejection before real-desktop QA

## Decision

**REJECTED BEFORE HUMAN QA.** Do not integrate the Codex branch and do not ask the human to test it.

A4 fixes the explicit A3 blockers: native-text cache lookup/read/parse moved off the egui frame, `PdfSearchPolicy::FullText` now searches the whole accepted native-page-aligned sentence domain, metadata/text request submission no longer blocks the caller on a full bounded channel, Current/Nearby raster arbitration remains preserved, bounded eight-page text extraction remains preserved, and hosted Windows workflow `34912292604` is green on substantive commit `0d2d8e7180858c14d52457cf61ac912666ac3020`.

Two production-path defects still violate the core Goal-0022 contract and make physical QA premature.

## Blocker 1 — the worker-built ReaderSnapshot is based on a stale clone of mutable live session state

`prepare_pdf_embedded_text_event()` captures a full clone of the current `ReaderSession` on the egui path:

```rust
let snapshot_session = self
    .effect_session
    .lock()
    .ok()
    .and_then(|session| session.clone());
```

The detached preparation worker then applies the prepared PDF text to that clone and builds the `ReaderSnapshot` that will later be published into runtime.

This creates a race against the very visual independence Goal 0022 is supposed to preserve. While document-scale preparation/cache work runs, the user can keep scrolling the continuous PDF, change page, alter settings, change playback state, or otherwise mutate the real live session. At commit time A4 correctly applies the prepared text to the **current live session**, but then publishes the **old worker snapshot** captured from the earlier clone.

The live `effect_session` and runtime `ReaderUpdated` projection can therefore disagree about current page, settings, playback/highlight state, and other mutable session fields. On a long PDF this can manifest as state reverting or jumping when text enrichment finishes even though the visual reader remained interactive during extraction.

The current tests do not simulate a mutable-session change between preparation start and enrichment commit.

### Required A5 correction

The background worker must prepare **immutable document-owned text state only**. It must not manufacture a final `ReaderSnapshot` from an earlier clone of mutable live session state.

At egui commit time:

- validate source/generation/revision/page count/trust;
- atomically attach/swap the prepared immutable PDF text document into the current live session;
- preserve the live session's current page, panels, reader settings, playback/highlight state, and other mutable state as of commit time;
- publish a truly bounded runtime patch/projection derived from that current state without any O(document) reconstruction.

Add a deterministic race regression: capture/prep begins while the PDF is on page A, mutate the live session to page B (and at least one other mutable field such as TTS state or a reader setting) before commit, then prove trusted-text adoption preserves B/current state while enabling Text-only/search/TTS.

## Blocker 2 — document-scale canonical sentence destruction still happens on egui

`ReaderSession::apply_prepared_pdf_embedded_text()` still returns the full document-wide `Vec<String>` of canonical sentences:

```rust
pub fn apply_prepared_pdf_embedded_text(
    &mut self,
    prepared: PreparedPdfEmbeddedText,
) -> Result<Vec<String>, String>
```

A4's live egui commit calls it as:

```rust
match session.apply_prepared_pdf_embedded_text(prepared) {
    Ok(_) => { ... }
```

The returned document-wide vector is no longer needed because the worker-built snapshot already owns its own prepared canonical sentence payload. `Ok(_)` therefore destroys that entire `Vec<String>` on the egui thread. Dropping thousands of owned sentence strings is O(document) allocator/destructor work and directly violates the rule that no operation in trusted-text adoption may scale with total PDF pages/sentences.

This also reveals that the current prepared-state model duplicates document-owned text between the live session and the worker snapshot instead of sharing a single immutable prepared document representation.

### Required A5 correction

Do not return/drop a document-scale canonical sentence vector from live adoption.

Prefer a shared immutable prepared PDF text document (for example an `Arc`-owned canonical document state) that contains the page text, per-page sentences, global canonical sentence domain, page/local provenance, counts/search index data, and other immutable document-scale structures. The live `ReaderSession` and runtime projection may share this state by handle while keeping mutable cursor/settings/playback state separate.

The egui commit must be O(1) or bounded in document size: move/swap handles and current-page-local state only. Document-scale allocation, cloning, flattening, and destruction must happen on worker threads.

Add instrumentation/test coverage that would fail if the adoption thread drops/clones/flattens a document-scale canonical sentence payload.

## Preserved A4 work

A5 should preserve:

- no Quack-check/Python/Docling/OCR in the native trustworthy-text path;
- one process-wide Pdfium owner;
- visual source open independent of text/cache enrichment;
- cache lookup/read/parse off egui;
- cache persistence off egui;
- trusted text policy promotion to Text-only/full-document search/TTS while exact visual sync remains disabled;
- document-wide native-PDF search with deterministic native-page provenance/navigation;
- nonblocking UI-facing metadata/text request submission;
- source/generation stale safety;
- Current and Nearby raster preemption;
- bounded eight-page native text chunks;
- conservative trust gate;
- versioned cache identity/reuse;
- existing backend-neutral/first-sample Windows TTS ownership;
- Goal 0019/0020 continuous PDF behavior and representative EPUB/TTS regressions;
- green hosted Windows validation discipline.

## Acceptance consequence

Goal 0022 remains open. A4 is not integrated. A5 must remove stale mutable-state publication and make the final trusted-text adoption boundary genuinely bounded, including destruction behavior, before real-desktop QA is authorized.
