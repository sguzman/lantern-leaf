# Goal 0010 A5 — director rejection

## Decision

**REJECTED FOR CORRECTION BEFORE HUMAN QA**

A5 has the right broad architecture, but it is not safe to integrate or send to real-desktop QA yet.

## What is good and should be preserved

- The worker inspected the actual Caliberate contract instead of probing invented legacy Calibre thumbnail routes.
- `has_cover` is carried through the Caliberate/catalog/app contracts.
- LanternLeaf uses an explicit Caliberate cover route and keeps legacy thumbnail routes limited to the legacy Calibre provider.
- Caliberate catalog loading no longer prefetches remote covers wholesale or materializes EPUBs just to obtain thumbnails.
- Visible-row scheduling is bounded to four in-flight requests and request/disk/decode work remains off the egui render thread.
- The UI introduces intentional textual placeholder states instead of unexplained black rectangles.
- The sibling Caliberate change is minimal: one explicit `/api/v1/books/{id}/cover` route serving the existing attached `cover.jpg`/`jpeg`/`png`/`webp` sidecar.

These are the implementation foundation for the correction attempt.

## Blocking defects

### 1. Successful per-cover work leaks the global Calibre busy scope

`AppCommand::EnsureCalibreThumbnail` currently starts `OperationScope::CalibreLoad`. On success, `handle_calibre_thumbnail` returns `handle_calibre_cached_books`, which emits `CalibreBooksLoaded { from_cache: true }`. The reducer intentionally clears `OperationScope::CalibreLoad` only for `from_cache == false` full catalog loads.

Therefore the first successful lazy thumbnail request can leave `operations.calibre_load` permanently true, causing the starter to remain in a loading/busy state.

A bounded multi-request cover scheduler should not use the full-catalog boolean operation scope as per-row completion ownership. Give cover requests their own completion/outcome path or another concurrency-safe ownership model.

### 2. Provider failure can leave a row stuck in `Loading cover…`

The UI's local `calibre_cover_pending` set is cleared on a loaded thumbnail or by observing a failed `calibre_load_event`. But `emit_failure_progress` only emits `CalibreLoadProgress(failed)` for `LoadCalibreBooks`; `EnsureCalibreThumbnail` failure emits `CommandFailed` without a matching per-book cover event.

The pending book ID therefore remains in the local set after connection refusal/provider failure, so the row can remain `Loading cover…` indefinitely instead of becoming `Provider unavailable` and retryable.

### 3. 404 / no returned cover / decode failure can also remain pending forever

`ensure_thumbnail_for_book` returns `Ok(false)` when the explicit cover endpoint yields no cover. Image decode/write errors are currently swallowed by the `write_thumbnail_file(...).is_ok()` chain and also collapse to no thumbnail. `handle_calibre_thumbnail` then reports ordinary cached books with no explicit cover outcome.

Because the UI removes pending IDs only when a thumbnail appears (or through the unrelated full-load failure event), these terminal cover outcomes can also remain stuck as loading. This violates the Goal 0010 state contract.

A correction must produce an explicit per-book outcome sufficient to distinguish at least:

- loaded;
- no cover advertised (no request);
- advertised cover unavailable / endpoint 404;
- provider unavailable / timeout / connection failure;
- cover fetch or decode failure.

Transient provider failures must remain retryable with bounded backoff/manual refresh; ordinary unavailable/negative results need sane invalidation and must not spin forever.

### 4. A5 used a stale LanternLeaf base/lifecycle contract

The implementation branch merge base is `c4e0f3148152f232198ce0b89ede14d9591dcc0c`; current director `main` was already seven commits ahead at Goal 0010 launch. Those seven commits are documentation/lifecycle-only, so the source-code design can be salvaged, but A5 incorrectly activated/terminalized the old queued copy instead of the current ready contract.

The correction must synchronize current `main` first and preserve the accepted Goal 0014 closure / Goal 0015 queue state.

### 5. Sibling Caliberate validation was not post-change, and required CI was not terminal

The report says the Caliberate server test suite passed **before** the cover-contract push. The new endpoint therefore lacks post-change validation evidence in the handoff. Add targeted coverage for the cover contract and rerun the relevant Caliberate suite after the endpoint commit.

LanternLeaf Windows CI run `34777773736` was still `in_progress` after A5 had already terminalized and signaled `Goal achieved`. Goal 0010's contract requires Windows CI to pass before successful terminalization. The correction must wait for required CI to complete successfully.

## Correction contract (A6)

Continue Goal 0010 on `codex/0010-caliberate-catalog-reliability`, but first synchronize the current director `main`. Preserve the useful A5 implementation and repair completion ownership rather than redesigning the library.

Use an explicit per-cover completion/outcome seam that carries book identity and terminal result back to UI/state. It must be safe with multiple concurrent visible-row requests; do not use a single boolean full-catalog busy flag as ownership for four independent cover fetches. Every started request must reach a terminal state and leave the pending set.

Sync `sguzman/caliberate` from its current `main`, replay the minimal cover endpoint, add focused endpoint/content tests, and rerun its server tests after the change. Do not merge either repository branch until director review.

Re-run LanternLeaf focused/workspace/Windows gates and wait for Windows CI success before terminal signaling. Do not request human QA; the director will review A6 first.
