# 0012 — A2 correction: async pretty/image worker wakeups

## Outcome

Preserve the successful A1 presentation-controls and inline-image implementation while fixing the one director-blocking lifecycle defect: asynchronous pretty-build and image-decode completions must wake an otherwise idle native egui event loop so ready content appears without mouse/keyboard input or active TTS.

## Starting evidence

A1 implementation `03a315b61d9cf4449b2de94ed59c1287fd2efba0` plus fixup `b60f83b26e6579c99382c85049250d060748c1c4` is substantively aligned with Goal 0012 and Windows CI run `34628740493` is green.

Accepted A1 work to preserve:

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
- all Goal 0008/0009 TTS and large-document regressions.

Director rejection detail is recorded in `docs/work/reviews/0012-a1-director-rejection.md`.

## Blocking defect

A1's workers publish completion into channels but do not themselves wake egui.

### Image decode

`PrettyImageCache::poll_ready()` calls `ctx.request_repaint()` only after a later frame has already begun and polled the result. `pretty_image_worker` has no `egui::Context` or equivalent repaint notifier.

When TTS is stopped and there is no user input, a completed decode is therefore not guaranteed to produce the frame needed to consume/upload it. A placeholder may remain until unrelated UI activity.

### Pretty build

`start_pretty_builder()` has the same lifecycle gap: the worker sends `PrettyBuildResult`, but does not request a repaint. An idle reader can remain on `Preparing pretty view…` until another event happens to create a frame.

The global 24 ms repaint cadence is conditional on `tts_runtime.needs_repaint()` and cannot be used as the wakeup mechanism for independent presentation work.

## Authorized A2 correction

### A — explicit completion wakeup

Give both worker paths an explicit native-egui wakeup mechanism.

Preferred shape:

- clone `egui::Context` (or pass a narrow repaint callback/notifier) into the pretty-build worker and pretty-image worker;
- after a result is successfully published to the result channel, call `request_repaint()` immediately;
- image failures must wake the UI just like successful decodes so failure/placeholder state becomes current;
- a pretty-build completion must wake the UI so the new blocks/targets are consumed promptly.

The exact abstraction is flexible, but the semantic contract is not: **worker completion itself causes the future frame**.

### B — preserve render-thread discipline

Do not fix this by polling continuously or blocking the render thread.

Preserve:

- bounded `sync_channel` request queues;
- `try_send`/nonblocking submission from egui;
- no file reads on the render thread;
- no image decode on the render thread;
- no structured pretty parsing/alignment on the render thread;
- lightweight result polling + egui texture upload only on the UI thread.

Do not create a permanent 24/60 Hz repaint loop merely to discover worker completions.

### C — regression the idle-event-loop failure class

Add deterministic production-adjacent tests/seams proving wakeup occurs independently of a later frame poll.

At minimum prove:

1. successful image worker completion publishes a result and emits one repaint notification;
2. failed/corrupt image worker completion publishes failure and emits repaint notification;
3. pretty-build worker completion publishes blocks/targets and emits repaint notification;
4. request queue full behavior remains nonblocking;
5. no test satisfies the contract merely by calling `poll_ready()` and observing that *polling* requests another repaint;
6. TTS does not need to be active for any of the above.

A small injected `RepaintNotifier`/counter test seam is acceptable if direct `egui::Context` repaint observation is awkward.

## Preserve A1 and prior goals

A2 must not broaden into presentation redesign. Preserve all A1 behavior and all accepted Goal 0008/0009 behavior, including:

- pretty/text-only audible sentence sync and follow;
- no duplicate ordinary TTS lines;
- Windows Zira/default/per-book voice behavior;
- transactional Piper failure recovery;
- close/safe-quit lifecycle;
- bounded pretty virtualization;
- presentation persistence/reset;
- real EPUB image provenance/order/path containment;
- lazy bounded image decode/cache;
- inline image placeholders and sizing.

Windows Natural/Narrator/HD voice work remains deferred. Goal 0010 Caliberate catalog covers and PDF work remain out of scope.

## Acceptance gates

1. Image success completion wakes egui from worker context/notifier, not from a later poll.
2. Image failure completion also wakes egui.
3. Pretty-build completion wakes egui.
4. No continuous repaint loop is introduced as a workaround.
5. Request submission remains bounded and nonblocking.
6. Disk/decode/pretty-build heavy work remains off the render thread.
7. Deterministic tests cover all three completion wakeup cases while TTS is inactive.
8. All existing Goal 0012 A1 presentation/image tests remain green.
9. All Goal 0008/0009 TTS/canonical sync regressions remain green.
10. `cargo check --workspace`, repository-policy workspace tests, `git diff --check`, repo-native Windows QA preparation, Windows TTS probe, hosted renderer probe, and Windows CI pass.
11. No human QA until director accepts A2.

## Repository handoff

Continue branch `codex/0012-pretty-presentation-controls-and-inline-images` and report `docs/work/reports/0012.md`.

Preserve the A1 implementation and append Attempt A2 to the existing report. Synchronize the latest director state from `main`, move this goal `ready -> active`, re-arm the Goal 0012 watcher, implement only this bounded correction, validate, terminalize `done` or `blocked`, push, signal terminal state, and restore the shared checkout to `main` without merging.

## Human verification

None during A2. After director acceptance, one focused Windows desktop QA pass will verify presentation controls, persistence/reset, inline real-EPUB images while idle, and a short Goal 0009 TTS synchronization regression.
