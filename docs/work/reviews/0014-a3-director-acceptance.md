# Goal 0014 A3 — director acceptance

Date: 2026-09-13

## Decision

**ACCEPTED FOR FOCUSED REAL-DESKTOP SIGNOFF.**

A3 addresses the exact residual defect from the A2 physical pass: reflow is no longer modeled as a chain of independent estimated restores. The implementation introduces a stateful reflow transaction that retains one stable semantic witness across rapid geometry changes, keeps the anchor neighborhood rendered while new geometry is measured, reconciles restoration from measured geometry, preserves pending TTS follow precedence, and cancels stale automatic correction on explicit user scroll/drag input.

## Accepted evidence

- Correction started from authoritative director `main` `d844333` and preserved the accepted A2 semantic-anchor architecture.
- Production implementation `430e9c1` adds `PrettyViewportWitness` and `PrettyReflowTransaction` state to the native egui app.
- A geometry-change burst captures one stable witness instead of recapturing from unstable intermediate frames.
- A visible canonical highlighted sentence/segment can seed the stable witness as a one-shot viewport anchor without becoming permanent auto-follow.
- Estimates remain available for bounded virtualization / target-neighborhood rendering; restoration offset is recomputed after the target block has real measured geometry.
- Pending canonical TTS follow clears idle reflow ownership and remains authoritative.
- Explicit wheel/drag input clears pending reflow correction so user intent wins.
- Stateful tests cover rapid A→B→C→D bursts, non-uniform reflow, visible-highlight witnessing, active-follow precedence, user-scroll cancellation, and existing media behavior.
- Focused reader suites, full workspace test/check/build, repo-native Windows QA preparation, renderer smoke, and `git diff --check` passed.
- GitHub Actions Windows baseline run `34768445726` completed successfully for native workspace/TTS and hosted renderer-capability jobs.

## Remaining gate

Because the rejected A2 symptom was specifically perceptual and multi-frame, one focused real-desktop pass remains necessary.

At a mid-book location, edit geometry controls both singly and rapidly. Verify:

- no violent unrelated-area flash followed by snap-back;
- no final settle on an unrelated semantic area;
- if the highlighted sentence is visibly on screen before the edit, the view remains semantically around it after reflow;
- repeated rapid slider changes retain one stable neighborhood rather than walking the anchor through intermediate states;
- active TTS still follows/highlights correctly;
- manual wheel/drag input during or immediately after a reflow wins and is not undone by a stale correction.

A few pixels/lines of local displacement inside the same semantic neighborhood are acceptable. A distant semantic excursion, even transiently, is not.
