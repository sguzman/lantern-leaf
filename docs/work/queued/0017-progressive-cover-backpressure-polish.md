# 0017 — Progressive catalog cover backpressure and error-state polish

## Outcome

Make lazy catalog-cover behavior remain calm and truthful while the large Caliberate catalog is itself loading progressively.

## Starting evidence

Goal 0016 real-desktop QA proved progressive catalog publication works. During that same cold ~105k provider walk, visible rows often entered `Cover fetch/decode failed`, the diagnostics showed repeated `calibre_ensure_thumbnail` planning, and the covers then populated successfully once the catalog walk completed.

The underlying cover/provider contract remains sound: after provider pressure dropped, real covers fetched, decoded, cached, and survived restart. The defect is transient-pressure handling and presentation, not missing covers.

Current source evidence relevant to this follow-up:

- remote cover fetches use the existing short `THUMB_FETCH_TIMEOUT`;
- visible-row scheduling is bounded to four in-flight requests;
- a generic cover fetch/decode failure becomes immediately eligible for another automatic visible-row request because only provider-unavailable state receives a retry delay;
- catalog refresh errors are rendered with literal `Color32::YELLOW`, which is unreadable on the accepted light theme.

## Architectural direction

Keep all cover HTTP, disk, and image decode work off the egui/render thread and preserve Goal 0010 per-book ownership/coalescing.

Treat provider pressure/timeouts as transient without lying that an image is corrupt. Add bounded retry/backoff or equivalent scheduling discipline so a visible row cannot hammer the provider every frame after a transient timeout. Permanent decode/content failures may remain terminal until explicit/manual retry.

Do not globally disable covers for the entire catalog walk unless measurement proves that is the smallest reliable solution; prefer allowing bounded concurrent cover progress when the provider can service it.

Error/warning presentation must be theme-aware and readable. Do not hard-code bright yellow text on a light background.

## Acceptance gates

1. During a cold progressive ~105k catalog load, visible cover requests remain bounded and do not produce repeated immediate retry churn.
2. Transient provider pressure/timeouts are represented as delayed/retryable state rather than generic scary decode corruption.
3. Covers can still populate before catalog completion when the provider responds within the allowed budget; otherwise they recover after pressure subsides without manual restart.
4. Permanent malformed-image/decode failure remains distinguishable from provider unavailability/timeout.
5. Manual `Ensure thumbnail` continues to use the same bounded/coalesced ownership path.
6. Catalog/provider error text is readable in both light and dark themes using semantic/theme-aware styling.
7. Goal 0010 and Goal 0016 catalog/cover behavior remains green; all heavy work stays off the render thread.

## Priority

Queued polish. Do not preempt the next substantive PDF gate unless cover churn becomes materially disruptive in ordinary library use.
