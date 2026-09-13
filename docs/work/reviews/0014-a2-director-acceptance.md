# Goal 0014 A2 — director acceptance

Date: 2026-09-12

## Decision

**ACCEPTED FOR FOCUSED REAL-DESKTOP SIGNOFF.**

The A2 correction is based on the authoritative director mainline, implements the generalized Goal 0014 contract rather than the superseded media-only framing, passes repository/Windows validation, and is suitable for physical validation.

## Accepted evidence

- A2 starts from authoritative director `main` `c06114e` and uses the required branch `codex/0014-reader-presentation-anchor-stability`.
- The valid A1 lifecycle is retained: capture viewport semantic state before presentation-geometry invalidation, clear stale measured heights, restore against reflowed geometry, and let a legitimate pending canonical TTS follow suppress idle restoration.
- A2 corrects A1's weak absolute within-block pixel anchor. The production anchor now prefers canonical sentence identity plus a normalized within-sentence position when a mapped pretty segment is available. When canonical mapping is unavailable, it falls back to normalized position within the same pretty block.
- This is centralized at the geometry-key transition path rather than patched per slider, so horizontal/content-width reflow, text metrics, spacing, media sizing, and compound changes share one policy.
- Deterministic production-adjacent tests cover materially rewrapping/non-uniform blocks, multiple positions, canonical sentence continuity, normalized fallback behavior, representative geometry classes, idle reading, and pending-TTS-follow precedence.
- The existing Goal 0012 media binding/non-binding/aspect-ratio tests and presentation/highlight behavior remain covered.
- Focused reader tests passed in both egui library and binary targets; workspace check/test/build, repo-native Windows QA preparation, renderer smoke, and `git diff --check` all passed.
- GitHub Actions Windows baseline run `34733109955` completed successfully for the corrected branch, including the native Windows workspace/TTS job and hosted renderer probe.
- No heavy/blocking document or image work was moved onto the egui/render thread.

## Director cleanup

The terminal branch again retained an identical `docs/work/active/0014-reader-presentation-anchor-stability.md` alongside the `done/` copy. This was lifecycle bookkeeping only. The stale active copy was removed by the director before integration.

The corrected branch was then fast-forward integrated into `main`.

## Remaining gate

The defect is interactive viewport behavior under live presentation editing and originally came from the real desktop. One focused physical pass remains required before final closure.

Use a real mid-book EPUB location and deliberately change geometry while watching the semantic reading position:

- make a large horizontal-margin change that visibly rewraps paragraphs; the reader should stay at the same paragraph/sentence area rather than jump elsewhere;
- change font size or line spacing significantly; semantic position should remain stable;
- change one block/paragraph/heading spacing control; semantic position should remain stable;
- if an inline image is available, make media max width/height genuinely binding and verify the image visibly resizes without throwing the viewport elsewhere;
- during active TTS, change a geometry-affecting setting and verify the currently spoken sentence remains highlighted/followed rather than being overridden by idle-anchor restoration.

Minor pixel displacement inside the same semantic paragraph/sentence is acceptable; jumping to a different unrelated document region is not.

Goal 0014 should close immediately if this pass is clean.
