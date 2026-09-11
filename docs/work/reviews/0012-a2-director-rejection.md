# Goal 0012 A2 director rejection

Status: **REJECTED BEFORE HUMAN QA**

Worker terminal head: `9d6c332b23430ebdde0710894163c9af1ea9ce2e`
A2 implementation commit: `f952ad680040a625c8009f5a206709b9cf351562`
Windows CI: `34652876171` — green

## Why this attempt is rejected

The worker did not synchronize the latest director state from `main` before executing Goal 0012 A2.

The authoritative director contract on `main` was `docs/work/ready/0012-pretty-presentation-controls-and-inline-images.md` titled **A2 correction: async pretty/image worker wakeups**. Its only blocking requirement was that asynchronous pretty-build and image-decode completion must itself wake an otherwise idle egui event loop.

The worker branch instead terminalized against the stale original Goal 0012 contract. Its `docs/work/done/0012-pretty-presentation-controls-and-inline-images.md` is still the original presentation/image goal, not the A2 wakeup contract.

The A2 implementation commit consequently does not implement the director-requested lifecycle correction. Its substantive changes are additional image-reference normalization/provenance tests and layered presentation-persistence tests. Those changes are reasonable and may be preserved, but they do not satisfy the blocking A2 acceptance gates.

In particular, the attempt does **not** establish:

- worker-completion repaint notification for pretty-build success;
- worker-completion repaint notification for successful image decode;
- worker-completion repaint notification for failed/corrupt image decode;
- deterministic idle/TTS-off tests proving those wakeups occur independently of later receiver polling.

Therefore the underlying A1 lifecycle defect remains open: a worker may publish ready content while the native event loop is idle, leaving `Preparing pretty view…` or an image placeholder visible until unrelated input/TTS activity produces a later frame.

## Branch-state problem

The worker branch is diverged from current `main`. This is not an implementation reason to discard the A1 work; it is a workflow synchronization failure.

The next attempt must first synchronize current director `main` into the existing Goal 0012 branch while preserving the A1 implementation and useful A2 evidence additions. It must then execute the current ready contract, not the stale done copy.

## Accepted evidence from this attempt

Windows workflow `34652876171` passed on `f952ad680040a625c8009f5a206709b9cf351562`, and the added UTF-8-safe percent decoding, query/fragment stripping, normalized image-provenance lookup, production-chain EPUB coverage, and layered presentation-reset tests may be retained if they remain green after synchronization.

## Required next action

Reopen the same repository Goal 0012 as A3. Do not create a new product macro-goal and do not request human QA.

A3 is a bounded correction:

1. synchronize latest `main` into `codex/0012-pretty-presentation-controls-and-inline-images` before implementation;
2. preserve A1 and useful prior additions;
3. implement explicit worker-completion wakeups for pretty-build and image-decode success/failure;
4. add deterministic idle/TTS-off regressions for those wakeups;
5. keep bounded/nonblocking queues and all heavy work off the GUI/render thread;
6. rerun the existing workspace/Windows gates and terminalize on the same branch/report lineage.
