# 0012 — A3 correction: synchronize director state and implement async wakeups

## Outcome

Preserve the successful Goal 0012 presentation-controls and inline-image implementation while completing the one remaining director-blocking lifecycle correction: asynchronous pretty-build and image-decode completions must wake an otherwise idle native egui event loop so ready content appears without mouse/keyboard input or active TTS.

## Why A3 exists

A1 (`03a315b61d9cf4449b2de94ed59c1287fd2efba0` + `b60f83b26e6579c99382c85049250d060748c1c4`) implemented the substantive presentation/image work and passed Windows CI `34628740493`.

Director review then reopened Goal 0012 as A2 with one narrow wakeup contract. The next worker attempt (`f952ad680040a625c8009f5a206709b9cf351562`, terminal `9d6c332b23430ebdde0710894163c9af1ea9ce2e`, Windows CI `34652876171`) did **not** synchronize the latest director state from `main`. It therefore terminalized against the stale original Goal 0012 contract and added useful evidence/normalization fixes without implementing the requested async wakeup correction.

See:

- `docs/work/reviews/0012-a1-director-rejection.md`
- `docs/work/reviews/0012-a2-director-rejection.md`

A3 is not a new product goal. It is the same Goal 0012 branch/report lineage with the workflow synchronization failure made explicit.

## Mandatory first step — synchronize director state

Before implementing anything:

1. fetch current `main`;
2. synchronize current director `main` into `codex/0012-pretty-presentation-controls-and-inline-images` by merge or rebase while preserving the A1 implementation and useful prior A2 additions;
3. resolve the goal lifecycle files so **this A3 contract from current `main` is authoritative**;
4. re-arm the Goal 0012 watcher;
5. only then implement the correction below.

Do not terminalize against the stale original `done/0012...` copy. Do not discard A1 merely because the branch diverged from `main`.

## Accepted implementation to preserve

Preserve the working Goal 0012 implementation unless a direct conflict requires a minimal adjustment:

- native Presentation section separate from TTS;
- live font family/weight/size, line spacing, margins, word/letter spacing, highlight colors, heading/base/paragraph/block/media controls;
- app-default -> explicit per-book presentation override persistence and presentation-only reset;
- source-born canonical highlight/follow identity unchanged;
- bounded off-frame pretty block construction;
- EPUB image provenance and safe relative/nested/encoded path resolution;
- generated multi-spine PNG/JPEG EPUB regression;
- bounded nonblocking image request queue;
- disk read/image decode off the render thread;
- bounded texture cache, transient negative cache, aspect-ratio/media sizing, visible failure placeholders;
- useful A2 additions for normalized provenance lookup, UTF-8 percent decoding, query/fragment handling, production-chain EPUB coverage, and layered presentation-reset coverage;
- all Goal 0008/0009 TTS and large-document regressions.

## Blocking lifecycle defect

### Image decode

`PrettyImageCache::poll_ready()` can call `ctx.request_repaint()` only after a frame already exists and polls the result. Worker completion itself must create the future frame.

A successful decode and a failed/corrupt decode must both wake egui after publishing their result.

### Pretty build

The pretty-build worker must wake egui after successfully publishing `PrettyBuildResult` so an idle reader cannot remain stuck on `Preparing pretty view…`.

The TTS repaint cadence is not an acceptable substitute because Goal 0012 presentation work must complete correctly with TTS inactive.

## Authorized A3 correction

### A — explicit completion wakeup

Give both worker paths an explicit repaint notifier.

Preferred shape:

- clone `egui::Context` into the worker, or inject a narrow `RepaintNotifier`/callback abstraction;
- after a result is successfully sent to the result channel, invoke repaint notification immediately;
- image success and image failure both notify;
- pretty-build completion notifies;
- notification must happen from worker completion semantics, not from later receiver polling.

### B — preserve render-thread discipline

Do **not** fix this with continuous polling or a permanent high-frequency repaint loop.

Preserve:

- bounded request queues;
- nonblocking render-thread submission (`try_send` or equivalent);
- no file reads on the render thread;
- no image decoding on the render thread;
- no structured pretty parsing/alignment on the render thread;
- UI thread limited to lightweight result polling and egui texture upload.

### C — deterministic idle/TTS-off regressions

Add production-adjacent tests or seams proving:

1. successful image worker completion publishes a result and emits repaint notification;
2. failed/corrupt image worker completion publishes failure and emits repaint notification;
3. pretty-build worker completion publishes blocks/targets and emits repaint notification;
4. request-queue-full behavior remains nonblocking;
5. none of these tests relies on calling `poll_ready()` to create the repaint;
6. TTS is inactive/unneeded for all completion-wakeup tests.

A small injected repaint-counter/notifier seam is acceptable if direct `egui::Context` repaint observation is awkward.

## Acceptance gates

1. Worker branch is synchronized with current director `main` before implementation and uses this A3 contract.
2. Image success completion wakes egui from worker completion semantics.
3. Image failure completion wakes egui from worker completion semantics.
4. Pretty-build completion wakes egui from worker completion semantics.
5. No continuous repaint loop is introduced as a workaround.
6. Request submission remains bounded and nonblocking.
7. Disk/decode/pretty-build heavy work remains off the render thread.
8. Deterministic idle/TTS-off tests cover all three completion wakeup cases.
9. A1 presentation/image behavior and useful prior A2 evidence remain green.
10. Goal 0008/0009 TTS/canonical sync regressions remain green.
11. `cargo check --workspace`, repository-policy workspace tests, `git diff --check`, repo-native Windows QA preparation, Windows TTS probe, hosted renderer probe, and Windows CI pass.
12. No human QA until director accepts A3.

## Explicit non-goals

Do not broaden this attempt into:

- presentation redesign;
- additional media-format work unrelated to the wakeup correction;
- Caliberate catalog-cover work from Goal 0010;
- Windows Natural/Narrator/HD voice work;
- PDF work;
- Piper model/catalog management;
- a new UI architecture.

## Repository handoff

Continue branch `codex/0012-pretty-presentation-controls-and-inline-images` and existing report `docs/work/reports/0012.md`.

Append **Attempt A3** to the report. Move this goal `ready -> active`, implement only this bounded correction, validate, terminalize `done` or `blocked`, push, signal terminal state, and restore the shared checkout to `main` without merging.

## Human verification

None during A3. After director acceptance, one focused Windows desktop QA pass will verify live presentation controls, persistence/reset, inline real-EPUB images including idle appearance, and a short Goal 0009 TTS synchronization regression.
