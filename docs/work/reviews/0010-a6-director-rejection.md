# Goal 0010 A6 — director rejection

## Decision

**REJECTED FOR CORRECTION BEFORE HUMAN QA**

A6 fixes the A5 global busy-scope leak and adds explicit book-identified terminal cover outcomes, but its UI event-consumption logic is still not concurrency-safe. Do not integrate or send A6 to real-desktop QA.

## What is good and should be preserved

- LanternLeaf is synchronized from the current director `main` and preserves the accepted Goal 0014 closure / Goal 0015 queue state.
- The explicit Caliberate `/api/v1/books/{id}/cover` contract and `has_cover` propagation remain the correct provider design.
- Visible-row scheduling is bounded in the automatic path and request/disk/decode work remains off the egui/render thread.
- A6 replaces the inappropriate global `OperationScope::CalibreLoad` ownership with explicit `CalibreCoverCompleted` outcomes carrying `request_id` and `book_id`.
- Loaded, advertised-cover-unavailable, provider-unavailable, and fetch/decode-error outcomes now exist explicitly.
- Transient provider failures are not written as permanent negative cache entries.
- The replayed sibling Caliberate branch is based on current `main`, includes targeted cover-route/content tests, and its post-change server suite passed.
- LanternLeaf focused/workspace validation and Windows CI run `34778760376` passed before terminalization.

These are all worth preserving.

## Blocking defect — global request-id watermark loses valid concurrent completions

The egui starter processes `model.calibre_cover_events` using a single global `last_calibre_cover_event_request_id` watermark:

```rust
if event.request_id <= self.last_calibre_cover_event_request_id {
    continue;
}
self.last_calibre_cover_event_request_id = self
    .last_calibre_cover_event_request_id
    .max(event.request_id);
```

That ordering assumption is invalid for this dispatcher. `EffectDispatcher` launches each planned effect on its own worker thread, so independent cover requests can complete in any order.

Example:

1. book A starts as request 100;
2. book B starts as request 101;
3. B finishes first, so the UI advances the global watermark to 101;
4. A then finishes normally with request 100;
5. A's valid terminal event is discarded because `100 <= 101`.

The result is exactly the class of defect A6 was meant to eliminate: book A can remain in the local pending set and render `Loading cover…` indefinitely despite having produced a valid terminal completion.

This is not hypothetical ordering trivia; the runtime explicitly allows out-of-order completion by spawning independent effect threads.

## Secondary bounded-concurrency issue

The automatic visible-row scheduler inserts book IDs into `calibre_cover_pending` and enforces `pending.len() < 4`, but the visible `Ensure thumbnail` button dispatches `EnsureCalibreThumbnail` directly without entering that ownership set first. Repeated manual clicks can therefore bypass the advertised four-request bound and also do not project the same loading ownership as automatic requests.

Goal 0010's concurrency bound should apply to all cover-fetch entry points, or the manual action should be disabled/coalesced through the same scheduler.

## Correction contract (A7)

Continue the same Goal 0010 and the same Codex session/branch. Do not redesign the Caliberate endpoint; the sibling A6 endpoint branch can be preserved unless a new defect is found there.

Replace the global completion watermark with concurrency-safe **per-book request ownership/freshness**. Requirements:

- completions for different books must be processed regardless of request-id order;
- an older completion for the same book must not overwrite a newer retry/request;
- only the completion corresponding to the currently owned request may clear/replace that book's pending state;
- every started current request must reach a terminal visible state: loaded, cover unavailable, provider unavailable/retryable, or fetch/decode error;
- user-triggered `Ensure thumbnail` must use the same ownership/concurrency path or be disabled/coalesced while pending so total cover work remains bounded;
- no cover network/disk/decode work may move onto the UI thread.

Add deterministic tests that complete at least two book requests out of order (`higher request_id` first), prove both books leave pending correctly, then test same-book stale completion after a newer retry and prove the stale result cannot clobber the newer request/status. Also cover the manual ensure path against duplicate/unbounded dispatch.

Re-run focused/workspace/Windows gates and wait for required CI success before terminal signaling. Do not request human QA; the director reviews A7 first.
