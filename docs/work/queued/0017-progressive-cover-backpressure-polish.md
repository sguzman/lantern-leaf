# 0017 — Progressive catalog cover backpressure and cache-hydration polish

## Outcome

Make catalog-cover behavior remain calm, lazy, and scalable for the real ~105k Caliberate library, both during progressive provider loading and on warm cached startup.

## Starting evidence

Goal 0016 real-desktop QA proved progressive catalog publication works. During that same cold ~105k provider walk, visible rows often entered `Cover fetch/decode failed`, diagnostics showed repeated `calibre_ensure_thumbnail` planning, and covers then populated successfully once provider pressure dropped.

A later warm physical pass exposed a second scaling defect. The catalog cache was already present, but startup launched multiple local thumbnail-hydration passes over nearly the entire ~104,732-row cached catalog. The attached run showed three interleaved passes: two hit the four-second budget after processing roughly 92k and 95k rows, while another processed all 104,732 rows; each changed catalog thumbnail state and rewrote the full cache.

The underlying cover/provider contract remains sound: real covers fetch, decode, cache, and survive restart. The defects are scheduling/backpressure and O(N) rediscovery of cached thumbnail state, not missing cover bytes.

## Current source evidence

- remote cover fetches use the existing short `THUMB_FETCH_TIMEOUT`;
- visible-row scheduling is bounded to four in-flight requests;
- a generic cover fetch/decode failure becomes immediately eligible for another automatic visible-row request because only provider-unavailable state receives a retry delay;
- `load_cached_books()` currently calls `hydrate_book_thumbnails(..., usize::MAX, THUMB_CACHED_PREFETCH_BUDGET, ..., false)`, causing per-book filesystem/path work across as much of the full cache as the budget permits;
- other cached/catalog load paths can run additional hydration passes, so startup can repeat/overlap large scans;
- changed hydration state causes a full ~105k catalog-cache rewrite;
- thumbnail prefetch currently logs progress every 25 books, generating thousands of lines on a large library;
- catalog refresh errors are rendered with literal `Color32::YELLOW`, unreadable on the accepted light theme.

## Architectural direction

Keep all cover HTTP, disk, and image decode work off the egui/render thread and preserve Goal 0010 per-book ownership/coalescing.

### Provider pressure

Treat provider pressure/timeouts as transient without lying that an image is corrupt. Add bounded retry/backoff or equivalent scheduling discipline so a visible row cannot hammer the provider every frame after a transient timeout. Permanent decode/content failures may remain terminal until explicit/manual retry.

Do not globally disable covers for the entire catalog walk unless measurement proves that is the smallest reliable solution; prefer allowing bounded concurrent cover progress when the provider can service it.

### Cached startup must remain lazy

Do **not** walk ~105k catalog entries on every warm startup merely to rediscover which thumbnail files already exist.

Prefer one of the following coherent ownership models (or an equivalent measured design):

- persist trustworthy thumbnail identity/path state in the catalog cache and validate lazily when a row becomes visible/near-visible;
- build/read one compact thumbnail index/directory snapshot off-thread rather than issuing per-book filesystem probes across the entire catalog;
- resolve existing cached thumbnail paths through the same visible/near-visible demand path used by remote cover scheduling.

The desired property is architectural, not implementation-specific: warm startup work should scale with visible/near-visible demand or one bounded/indexed operation, **not O(total books) repeated scans**. Multiple concurrent hydration passes over the same catalog are not acceptable.

Avoid rewriting the entire catalog cache merely because a handful of thumbnail paths were rediscovered. If durable cover association changes require persistence, batch/coalesce it intentionally.

### Presentation / diagnostics

Error/warning presentation must be theme-aware and readable. Do not hard-code bright yellow text on a light background.

Large-library thumbnail diagnostics must be useful without emitting thousands of progress lines; use coarse milestones/summary telemetry.

## Acceptance gates

1. During a cold progressive ~105k catalog load, visible cover requests remain bounded and do not produce repeated immediate retry churn.
2. Transient provider pressure/timeouts are represented as delayed/retryable state rather than generic scary decode corruption.
3. Covers can still populate before catalog completion when the provider responds within the allowed budget; otherwise they recover after pressure subsides without manual restart.
4. Permanent malformed-image/decode failure remains distinguishable from provider unavailability/timeout.
5. Manual `Ensure thumbnail` continues to use the same bounded/coalesced ownership path.
6. A warm cached ~105k startup does not scan most/all catalog rows to rediscover thumbnail files and does not launch overlapping whole-catalog hydration passes.
7. Existing cached covers for visible rows become available promptly without requiring the user to scroll through the entire library or wait for a full-catalog pass.
8. Small thumbnail-state changes do not trigger repeated giant cache rewrites.
9. Catalog/provider error text is readable in both light and dark themes using semantic/theme-aware styling.
10. Large-library cover logging is bounded/coarse rather than thousands of per-25-row progress messages.
11. Goal 0010 and Goal 0016 catalog/cover behavior remains green; all heavy work stays off the render thread.

## Priority

Queued library polish. It does not preempt Goal 0019 A4's blocking PDF-open correction, but the warm-start O(N) scan is now a confirmed scaling defect rather than merely cosmetic cover churn.
