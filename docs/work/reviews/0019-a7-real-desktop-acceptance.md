# Goal 0019 A7 — real-desktop acceptance

## Decision

**ACCEPTED — GOAL 0019 / GATE 3 CLOSED**

The A7 build passed the real Windows desktop recheck after the earlier A3/A5 failures and A6 pre-QA rejection.

## Physical evidence

The representative Caliberate PDF (`725-13e8b7a0.pdf`) now:

- enters the native Reader successfully instead of remaining in `SourceLoading`;
- reports a truthful multi-page native page domain (`Page 13 / 638` observed during QA);
- renders the actual PDF page through the native Pdfium/egui surface;
- advances with Next page and returns with Previous page;
- remains usable under aggressive ordinary PDF browsing;
- preserves representative EPUB behavior with no observed EPUB regression.

This closes the core Gate-3 requirement: LanternLeaf now has a functioning native visual PDF reader independent of Quack-check/transcript/OCR prerequisites, with one shared native Pdfium owner, off-render-thread native work, native page-domain navigation, real raster presentation, stale-safe request identity, and bounded texture residency.

## Accepted residuals / newly clarified product direction

The current production surface is deliberately single-page/paginated rather than a continuous page stack. Physical QA clarified that the desired long-term reader experience is **continuous scrolling across PDF pages** while retaining explicit page identity/navigation.

The current zoom policy is also intentionally conservative/scaffold-like: fixed steps from 75% through 175%. This is adequate for Gate 3 but not the desired final PDF reader ergonomics.

These are not reasons to keep Goal 0019 open because the native visual foundation is now physically proven. They are promoted into the next bounded visual goal before Gate 4 text/TTS synchronization:

- continuous virtualized page-stack presentation;
- visible-page/overscan driven render ownership using the existing viewport planning primitives;
- broader practical zoom and fit controls;
- preservation of snappy immediate-mode scrolling, bounded residency, current-page identity, and source/zoom stale safety.

Gate 4 PDF text/TTS/highlight/OCR integration should build on that continuous viewport rather than forcing auto-follow/highlight behavior onto the temporary single-page presentation.

## Follow-up

See `docs/work/ready/0020-continuous-pdf-viewport-and-zoom.md`.

Goal 0017 remains queued for cover hydration/backpressure scaling. Goal 0018 remains queued for Windows QA bootstrap idempotence. Goal 0015 remains queued minor EPUB reflow polish.
