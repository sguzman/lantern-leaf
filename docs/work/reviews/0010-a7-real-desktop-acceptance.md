# Goal 0010 A7 — real-desktop acceptance

## Decision

**ACCEPTED — GOAL 0010 CLOSED**

The real Windows pass confirms the primary user-visible outcome of Goal 0010: LanternLeaf can consume the explicit Caliberate cover contract, show real catalog covers before first open, continue lazy visible-row cover loading during scroll, open representative Caliberate EPUBs extremely quickly, and preserve the accepted EPUB reader/TTS/visual-settings path.

## Physical evidence

- Caliberate was running on the expected `127.0.0.1:8181` provider and LanternLeaf successfully reached the real 105,570-book catalog.
- Real cover thumbnails appeared in the starter catalog before opening the corresponding books.
- Additional covers showed `Loading cover…` while scrolling and subsequently populated, demonstrating the intended lazy visible-row path.
- Representative EPUBs opened extremely quickly.
- EPUB TTS and presentation/settings behavior remained good during the pass.
- The earlier mysterious black-cover symptom is no longer the observed normal path.

## Follow-up evidence discovered during this pass

Two defects were found, but neither reopens Goal 0010 because neither is a failure of the accepted provider/cover ownership contract:

1. A cold/incompatible catalog cache can leave the starter catalog visually empty while LanternLeaf walks all 105,570 books in Caliberate's real 500-row pages. The provider remains healthy and pages arrive continuously; the problem is lack of progressive catalog publication / useful partial-state UX.
2. A successfully opened EPUB is durably persisted as a recent source but the in-memory Recents panel is stale until the next explicit reload/restart. Restarting LanternLeaf makes the tested EPUBs appear, proving recent persistence was not lost.

These are now combined as Goal 0016, `starter-library-live-state-continuity`, because both are same-session starter-state continuity problems exposed by the library QA.

## Scope clarification

The `qa.ps1` harness intentionally points `LANTERNLEAF_CACHE_DIR` at the isolated `.qa/windows/cache` tree. The empty Recents panel at QA startup therefore did not demonstrate deletion of the user's ordinary LanternLeaf cache.

PDF view/TTS/highlight behavior was also observed to remain incomplete. That is expected: native PDF visual stability and PDF text/TTS/highlight synchronization remain later product gates and are not Goal 0010 regressions.

## Automated-only residual checks

The user did not separately repeat every synthetic A7 concurrency scenario on the physical desktop. In particular, provider-down/restart recovery and repeated manual `Ensure thumbnail` coalescing remain primarily backed by the accepted deterministic A6/A7 tests, director review, and passing Windows CI. The real desktop pass exercised the live provider, real cover fetch/decode/cache path, lazy scroll loading, and representative reader/TTS regression. That combined evidence is sufficient for director closure rather than requiring redundant manual stress reproduction.

## Closure

Goal 0010 is closed. Do not reopen it for the cold-catalog blank-wait or same-session Recents issue; those are Goal 0016. Do not reinterpret the withdrawn book `42866` incident as a materialization defect.