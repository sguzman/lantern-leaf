# Goal 0022 A5 — director rejection before real-desktop QA

## Decision

**REJECTED BEFORE HUMAN QA.** Do not integrate the Codex branch and do not ask the human to test it.

A5 materially fixes the A4 ownership boundary: the worker now prepares shared immutable `Arc<PreparedPdfEmbeddedText>` state instead of a stale worker `ReaderSnapshot`; the live session remains the authority for current page/settings/playback/search state; document-scale canonical data is shared by handle; cache IO remains off egui; Current and Nearby raster work still preempt text extraction; extraction remains bounded in eight-page chunks; and fresh hosted Windows workflow `34918108092` is green on substantive commit `46506aad4afbc61bb4f9e212c26686e31dbcbd1d`.

Director source review still found two user-visible correctness defects in the accepted-text path, plus one lifecycle hardening item. Physical QA would therefore be premature.

## Blocker 1 — canonical global sentence identity still uses the stale pre-enrichment page-count domain

A5 correctly routes most PDF text operations through `active_page_sentence_counts()` / the shared prepared document. However `ReaderSession::global_display_idx()` still computes its page base from the legacy `self.page_sentence_counts` field:

```rust
fn global_display_idx(&self) -> Option<usize> {
    let page_base: usize = self
        .page_sentence_counts
        .iter()
        .take(self.current_page)
        .sum();
    self.highlighted_display_idx.map(|idx| page_base + idx)
}
```

For the production visual-first PDF path, the session begins with an empty transcript. The legacy page-count vector therefore does not become the trusted native PDF sentence domain when A5 attaches `pdf_text_document`.

After trusted adoption on native page N > 0, `highlighted_canonical_idx()` can consequently report a page-local index instead of the true document-global canonical sentence index.

This is not cosmetic. That identity is consumed by playback/bookmark logic. In particular, `has_canonical_sentence_after_current()` uses `highlighted_canonical_idx()` to decide whether TTS has more canonical content. On the final native page, a wrongly small page-local index can make the runtime believe more text remains after the actual final sentence, risking incorrect continuation/replanning/replay behavior at document end.

The A5 race regression asserts current page/search/playback/settings preservation, but does not exercise global canonical identity on a later page or end-of-document first-sample behavior.

### Required A6 correction

Every canonical-global identity calculation for an enriched PDF must use the shared trusted document domain. At minimum:

- `global_display_idx()` must use `active_page_sentence_counts()` or the prepared document's prefix sums;
- bookmark canonical identity must use the same domain;
- first-sample playback identity, cross-page TTS continuation, and end-of-document detection must remain globally correct;
- no trusted-PDF path may silently fall back to stale empty/pre-enrichment page-count ownership.

Add a production-shaped multi-page regression beginning from a real render-only PDF session, then trusted adoption, then:

1. move to a later native page;
2. prove page-local sentence K maps to the correct document-global canonical ID;
3. prove a first-sample boundary on that page preserves the same global ID;
4. advance to the final sentence of the final page and prove `has_canonical_sentence_after_current()` is false;
5. prove the TTS runtime stops/terminalizes rather than rebuilding or replaying an earlier window.

## Blocker 2 — a search query entered before/during enrichment is preserved but not resolved after trust promotion

A5 captures `(search_query, search_allowed)` before trusted text exists. The production visual-first policy is render-only at that point, so `search_allowed` is false and worker preparation intentionally computes no matches.

At live adoption, if the current query still equals the prepared query, A5 installs the prepared empty match list. If the query changed during preparation, A5 also installs an empty match list. The query string itself survives, but trusted-text adoption does not automatically make that already-entered query meaningful.

A user can therefore type a search while text enrichment is pending, have trusted text become available, and still see no results until they edit/resubmit the search query. That violates the intended transition from visual-only to full canonical search ownership.

The A5 race test currently proves only that the newer query string is preserved, not that it resolves against the newly trusted document.

### Required A6 correction

Trusted adoption must reconcile the **current live search query** against the newly attached shared document without performing an O(document) scan on egui.

Acceptable shapes include:

- an immutable worker-built search index in `PreparedPdfEmbeddedText` with bounded query lookup at commit time; or
- a stale-safe off-thread search reconciliation job triggered after adoption when the live query differs from the worker-prepared query.

Requirements:

- an unchanged pre-enrichment query gains matches automatically after trusted adoption;
- a query changed while preparation was running gains matches for the newer query, not the stale one;
- match -> native page/page-local provenance remains deterministic;
- Search Next/Previous navigation works immediately once reconciliation lands;
- no document-wide query scan executes on egui.

Add deterministic races for both unchanged and changed-during-preparation queries.

## Lifecycle hardening — final document-scale Arc destruction must stay off egui

A5 correctly retires the replaced top-level `Arc<PreparedPdfEmbeddedText>` on a detached thread. However runtime projections also hold cloned inner document-scale Arcs such as canonical sentences/page counts/search matches. If those projection Arcs become the final owners after the top-level document is retired, replacing the old runtime projection on egui can still perform the final destruction of large vectors there.

A6 should make document retirement ownership explicit enough that the final destruction of document-scale trusted-PDF payloads cannot accidentally occur on egui during re-adoption/source replacement. This may use a shared top-level document handle in projections, a deferred-retirement queue, or an equivalent design with testable thread ownership.

This is a lifecycle guarantee, not a request to reintroduce document copying.

## Bounded projection note

A5 still calls `session.snapshot(...)` from the enrichment commit, but enriched sessions immediately route to `snapshot_pdf_enrichment()`, which shares document-wide Arc state and only materializes current-page-local fields. The architectural intent of the A5 bounded projection is therefore substantially met today.

A6 should make that guarantee explicit in the API (for example a named bounded enriched-PDF projection) so future changes cannot accidentally route enrichment back through ordinary document assembly. Do not reintroduce document-scale snapshot construction.

## Preserved A5 work

A6 must preserve:

- no Quack-check/Python/Docling/OCR in the trustworthy native path;
- one process-wide Pdfium owner;
- visual source open independent of enrichment/cache work;
- shared immutable prepared PDF text document state;
- current live session authority at adoption time;
- trusted policy promotion to Text-only/full search/TTS with exact visual sync disabled;
- cache lookup/parse/persistence off egui;
- nonblocking UI-facing native request enqueue;
- Current and Nearby raster preemption;
- bounded eight-page text chunks / reduced native document opens;
- source/generation stale safety;
- existing backend-neutral/first-sample Windows TTS ownership;
- Goal 0019/0020 continuous PDF behavior;
- representative EPUB/TTS behavior;
- fresh hosted Windows validation discipline.

## Acceptance consequence

Goal 0022 remains open. A5 is not integrated. A6 must make the shared trusted document authoritative for canonical global identity, reconcile live search state when trust arrives, and close the document-retirement thread-ownership hole before real-desktop QA is authorized.
