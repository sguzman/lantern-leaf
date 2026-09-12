# Goal 0012 A3 director acceptance

Status: **ACCEPTED FOR FOCUSED REAL-DESKTOP QA**

A3 implementation: `dccc5b999aa7732d1f71a248d8595cf5bde4a40d`
Worker terminal: `ae411ffaadebebb23f137b40728dfb4dbc944048`
Windows CI: `34655821185` — green

## Director review

A3 satisfies the async-wakeup correction that blocked A1/A2 desktop QA.

- The worker synchronized the authoritative director A3 contract from `main` before implementation.
- Pretty-build completion publishes `PrettyBuildResult` and then invokes a repaint notifier from worker completion semantics.
- Image decode success and image decode failure both publish their result and then invoke the repaint notifier from the worker.
- The production notifier is backed by cloned native egui `Context::request_repaint()`.
- Deterministic tests wait for repaint notifications before touching result receivers, so receiver polling is not the wakeup source.
- Queue submission remains bounded/nonblocking; no permanent repaint loop was introduced.
- File I/O, image decode, structured pretty preparation, and source-target alignment remain off the egui/render thread.
- A1 presentation controls, persistence/reset, image provenance/resolution, placeholders/sizing/cache behavior, and Goal 0008/0009 regressions remain green.

The terminal Windows baseline run `34655821185` passed native workspace and hosted renderer jobs, including repo-native QA preparation, workspace check/build/test, watcher-policy tests, Windows TTS probe, and hosted renderer capability probe.

## Human signoff

One focused Windows desktop QA pass is authorized. Verify:

1. Presentation controls are visible and change the pretty view live.
2. Change a few presentation values, close/reopen the same book, and verify persistence; use `Use app presentation defaults` and verify reset.
3. Open a real EPUB containing images with TTS stopped. Inline images should appear in the correct document position without requiring mouse movement/key presses to wake the UI.
4. Scroll to another image and leave the UI idle briefly; image completion should appear on its own.
5. Briefly start Windows TTS and verify the already-accepted pretty highlight/follow synchronization remains correct.

Goal 0012 is not finally closed until this focused real-desktop QA passes.
