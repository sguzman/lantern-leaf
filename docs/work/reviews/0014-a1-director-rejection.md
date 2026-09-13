# Goal 0014 A1 — director rejection

Date: 2026-09-12

## Decision

**REJECTED FOR CORRECTION BEFORE HUMAN QA.**

The implementation direction is promising and CI is green, but the terminal worker branch did not execute against the authoritative Goal 0014 contract or current director `main`.

## Blocking workflow defect

The authoritative generalized Goal 0014 contract was already on director `main` at `bca97d7b8b6e74f7110d0539eeb9b00e834c74db` before this attempt began. It lives at `docs/work/ready/0014-reader-presentation-anchor-stability.md` and explicitly requires branch `codex/0014-reader-presentation-anchor-stability`.

Attempt A1 instead reports that it started from `d6b2b35`, used branch `codex/0014-reader-media-sizing-anchor-stability`, and terminalized the older media-only Goal 0014 document. The branch therefore diverges from current `main` at `d6b2b35` and is six commits behind the authoritative director state. This violates the repository handoff requirement to synchronize current director `main` before implementation and means its lifecycle/report artifacts do not represent the authoritative goal.

## Implementation review

The production change correctly identifies the central defect class: old egui raw Y scroll state survives a presentation geometry transition after measured pretty-block heights are invalidated. A1 captures a block-index plus within-block offset before invalidation and restores an estimated Y for the same block after reflow. It also gives pending canonical TTS follow precedence over idle restoration. This is aligned with the intended architecture and should be preserved where valid.

However, the generalized authoritative contract asks for semantic block/sentence continuity and bounded displacement through non-uniform reflow. A1's anchor is only a block index plus an absolute pixel offset within the old block. That can still drift substantially inside a large paragraph/block when its internal line wrapping changes. The correction attempt must explicitly evaluate whether block identity plus absolute within-block pixels is sufficient. Prefer a fractional within-block position or canonical sentence/segment anchor where production mapping permits, and add production-adjacent coverage that proves semantic continuity rather than only helper arithmetic.

## Required correction

Reuse Goal 0014. Do not create a new goal ID.

1. Start from current director `main` and the authoritative ready file `docs/work/ready/0014-reader-presentation-anchor-stability.md`.
2. Use the authorized branch `codex/0014-reader-presentation-anchor-stability`.
3. Carry forward A1's valid implementation ideas rather than rewriting blindly.
4. Reconcile the anchor representation with the generalized contract: prove the visible semantic location remains stable through materially rewrapping paragraphs, text metrics, spacing, media changes, and compound changes; pending canonical TTS follow retains precedence.
5. Retain media max-width/max-height binding-limit verification.
6. Run the full repository/Windows gates and CI from the corrected branch.
7. Terminalize the authoritative generalized goal only after all gates pass.

No human QA should be requested until this correction receives director acceptance.
