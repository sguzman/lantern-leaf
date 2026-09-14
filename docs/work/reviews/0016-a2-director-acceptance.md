# Goal 0016 A2 — director acceptance

## Decision

**ACCEPTED FOR FOCUSED REAL-DESKTOP QA**

A2 repairs the three director-blocking defects found in A1 without reopening the accepted progressive-catalog and same-session-Recents architecture.

## Accepted corrections

- Live lazy-cover projection survives later metadata batches and final catalog reconciliation. Incoming explicit thumbnail state may replace the current value; an absent incoming thumbnail no longer erases a cover already acquired by the running UI.
- A progressive provider failure remains a failed/degraded refresh even when a compatible stale cache exists. Fresh partial rows are retained; stale fallback data is only surfaced when no fresh provider page has arrived, and fallback availability does not convert the failed refresh into ordinary success.
- Catalog refresh ownership is single-worker/coalesced while a catalog load is active. The redundant cached-loader effect was removed from the ordinary load plan, preventing overlapping full provider walks and stale durable-cache races.
- Same-session Recents refresh remains triggered only after successful SourceOpen persistence and still uses the existing background Recents listing path.
- All HTTP, parsing, disk/cache, catalog traversal, cover fetching/decoding, and recent discovery remain off the egui/render thread.

## Evidence

A2 implementation commit: `602e8952d077796ba478bd28e0b0cfe2d1e6bb51` (`fix: harden progressive catalog ownership`).

Goal terminal commit: `ca94477c6280ae1e5c47a5b32d02947019a428c4`.

Windows baseline run `34832563563` for the implementation commit completed successfully. Both `native-workspace` and `hosted-renderer-probe` jobs passed, including QA preparation, workspace check/build/test, Windows TTS diagnostics, notification workflow checks, and renderer probing.

Director source review verified deterministic coverage for:

- live cover survival across duplicate batch plus final completion;
- explicit incoming cover replacement;
- duplicate catalog-refresh coalescing while the authoritative worker is active;
- one successful fresh provider page followed by provider failure with a stale fallback cache present;
- effect-layer failed-progress propagation without a false `CalibreBooksLoaded` terminal success;
- the A1 progressive-batch, stale-reducer, partial-state, and same-session Recents tests.

The Goal 0016 branch was based directly on director `main` and has been fast-forward integrated into `main` before this acceptance record.

## Remaining real-desktop gate

Goal 0016 is not closed until focused Windows QA verifies the behavior that repository/CI evidence cannot establish perceptually:

1. From a clean QA catalog cache with the real large Caliberate library, useful catalog rows appear after the first provider pages rather than after the entire ~105k walk.
2. While the full catalog is still loading, the starter UI visibly communicates partial/loading state and useful progress.
3. Covers continue to load normally on visible rows during the progressive catalog walk and do not disappear when the full catalog reaches completion.
4. Open a representative EPUB, return to Starter without restarting LanternLeaf, and confirm it appears in Recents during the same process.
5. Restart LanternLeaf and confirm the same recent entry is reconstructed from durable QA cache state.
6. Existing representative EPUB open/TTS/highlight/visual-settings behavior remains green.

No further Codex correction is authorized unless this physical pass finds a concrete defect.