# Goal 0022 A6 — director rejection before real-desktop QA

## Decision

**REJECTED BEFORE HUMAN QA.** Do not integrate the Codex branch and do not ask the human to test it.

A6 fixes the explicit A5 correctness gaps: trusted native-PDF sentence identity now uses the active prepared document on later pages, the production tests cover first-sample identity and final-document exhaustion, search reconciliation is source/generation/query-revision stale-safe and runs off egui, the enriched-PDF projection is explicitly named, and fresh hosted Windows workflow `34968802323` is green on substantive commit `71f07aad7d540303fd73f204e4164a6d5592514e` with both `native-workspace` and `hosted-renderer-probe` passing.

Two production-path lifecycle/performance defects remain. They are especially important because Goal 0020 physically established extremely responsive continuous PDF scrolling as authoritative behavior.

## Blocker 1 — enriched `ReaderSnapshot::drop()` creates a new OS thread for every retired full snapshot

A6 adds a custom `Drop` implementation to `ReaderSnapshot` so document-scale `Arc` ownership will be released away from egui. The intent is correct, but the mechanism is not:

```rust
impl Drop for ReaderSnapshot {
    fn drop(&mut self) {
        let Some(document) = self.pdf_document_handle.take() else {
            return;
        };
        // ... move document-scale Arc fields out ...
        std::thread::spawn(move || {
            drop((document, canonical_sentences, page_sentence_counts, search_matches));
        });
    }
}
```

This is not a one-time source-close path. `ReaderSnapshot` is the full reader-document projection. Continuous PDF viewport ownership calls `SetPage` as the viewport-derived current native page changes; `handle_reader_command()` produces a new `ReaderUpdated` full snapshot for each such command; `AppState::set_reader_document()` replaces the previous `Arc<ReaderSnapshot>` on the egui event path.

Therefore an enriched PDF that is rapidly scrolled across many native pages can retire many snapshots and synchronously invoke `std::thread::spawn()` from snapshot destruction. Settings/search/navigation commands can do the same. The custom drop also spawns even when the document-wide Arcs are not the final references, so it can create a short-lived OS thread merely to decrement shared reference counts.

That is an unbounded thread-creation path coupled to ordinary reader interaction. It risks regressing the exact fast-scroll behavior physically accepted in Goal 0020 and is not an acceptable interpretation of “defer document-scale destruction off egui.”

### Required A7 correction

Use one bounded/shared retirement mechanism rather than one OS thread per snapshot/session drop. For example:

- one process-wide or app-owned PDF document-retirement worker with a channel;
- snapshot/session drop performs only a bounded nonblocking handoff of document-scale handles;
- the worker owns final Arc release/destruction;
- ordinary current-page-local snapshot replacement does not create a new native thread.

The solution must preserve the invariant that final document-scale destruction cannot occur on egui, including source switch/close/re-adoption/runtime projection replacement.

Add a deterministic churn regression that replaces/drops many enriched PDF snapshots as rapid page ownership would, proves document payload retirement occurs off the commit thread, and proves the implementation does **not** create one retirement thread per snapshot.

## Blocker 2 — the named “bounded” enriched-PDF projection still performs O(total native pages) scans

A6 correctly renamed the trusted projection to `snapshot_enriched_pdf_bounded()`, but the call graph is not actually bounded in document length.

`global_display_idx()` now uses the correct active document, but computes the page base by scanning/summing every preceding page:

```rust
let page_base: usize = self
    .active_page_sentence_counts()
    .iter()
    .take(self.current_page)
    .sum();
```

The bounded projection calls `highlighted_canonical_idx()`, which calls `global_display_idx()`. Its `tts_view()` also calls `has_sentence_before_current_page()` / `has_sentence_after_current_page()`, which scan the page-count domain before/after the current page.

The prepared trusted document already contains `page_sentence_prefix_sums`. The A6 contract explicitly says the enriched projection must not scan the whole PDF. A 638-page real document is already our physical baseline, and this should not quietly become O(page-count) UI work on larger documents.

### Required A7 correction

Make trusted-PDF global/page-existence calculations use the prepared immutable indexes:

- `global_display_idx()` should use `page_sentence_prefix_sums[current_page]` for enriched PDFs;
- `has_canonical_sentence_after_current()` should use the prepared total sentence count/prefix sums;
- `has_sentence_before_current_page()` and `has_sentence_after_current_page()` should use prefix/total counts rather than page scans;
- `current_tts_audio_display_ids()` should use the same O(1) page base;
- `page_idx_for_global_sentence()` should use the prepared prefix index (binary search is sufficient) rather than linearly walking every page when enriched;
- other trusted-PDF canonical-page prefix calculations should be audited for the same issue.

The bounded enriched snapshot/projection must remain current-page-local plus Arc clones, with no work proportional to total native page count or total canonical sentence count.

## Preserved A6 work

A7 should preserve:

- no Quack-check/Python/Docling/OCR in the native trustworthy-text path;
- one process-wide Pdfium owner;
- visual source open independent of text/cache/search enrichment;
- cooperative bounded native-text extraction with Current/Nearby raster preemption;
- off-egui cache lookup/parse/persistence;
- nonblocking UI-facing native request enqueue;
- shared immutable `Arc<PreparedPdfEmbeddedText>` document state;
- current live mutable session authority;
- trusted policy promotion to Text-only/document-wide search/ordinary Windows TTS while exact visual sync remains disabled;
- correct later-page canonical identity, first-sample identity, bookmark identity, and true final-document exhaustion;
- query-revision/source/generation-safe off-thread search reconciliation;
- explicit `snapshot_enriched_pdf_bounded()` publication;
- versioned cache reuse and stale/corrupt safety;
- Goal 0019/0020 visual behavior and representative EPUB/TTS regressions;
- green hosted Windows validation discipline.

## Acceptance consequence

Goal 0022 remains open. A6 is not integrated. A7 is a focused bounded-lifecycle correction: replace per-snapshot thread creation with a shared retirement path and make the enriched-PDF projection truly independent of total PDF page count before real-desktop QA is authorized.
