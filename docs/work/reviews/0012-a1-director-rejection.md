# Goal 0012 — A1 director rejection

Status: **A1 IMPLEMENTATION SUBSTANTIVELY GOOD, BUT NOT YET ACCEPTED — ONE ASYNC WAKEUP DEFECT MUST BE CORRECTED BEFORE HUMAN QA.**

Implementation: `03a315b61d9cf4449b2de94ed59c1287fd2efba0` plus fixup `b60f83b26e6579c99382c85049250d060748c1c4`.

Worker terminal: `762ecb54bb135b1568baf9cfd5ebab8dc718cc37`.

Authoritative Windows CI: `34628740493` — green.

## What A1 got right

The implementation is otherwise aligned with the Goal 0012 architecture:

- a separate native-egui Presentation surface exists and routes presentation changes through the existing `ReaderSettingsPatch` / `BookReaderOverrides` ownership model;
- presentation reset is presentation-only and preserves TTS/backend/voice state;
- font family/weight, font size, line spacing, margins, word/letter spacing, highlight colors, heading/base/paragraph/block/media controls have native render consumers rather than being persistence-only no-ops;
- pretty block preparation/source-target alignment was moved behind a bounded worker request queue;
- EPUB image records now carry source reference, normalized path, aliases, chapter/source order, local extracted path, and alt text;
- the generated EPUB regression exercises multi-spine nested/relative encoded PNG/JPEG references and verifies extraction-root containment/provenance;
- image file reading and decoding occur in a dedicated worker behind `sync_channel(32)` and `try_send`, so the egui frame does not block on disk/decode;
- image failures have visible placeholder/alt behavior;
- texture state is bounded/evicted and failed decodes have a temporary negative cache;
- existing bounded pretty rendering and canonical spoken-sentence follow/highlight architecture remain intact;
- Windows CI, workspace tests, QA preparation, Windows TTS probe, and hosted renderer probe are green.

## Blocking defect: worker completion does not wake an idle egui event loop

Goal 0012 explicitly requires **request repaint when a decode completes**. A1 does not yet satisfy that lifecycle guarantee.

`PrettyImageCache::new()` spawns `pretty_image_worker(worker_rx, worker_tx)` with no `egui::Context`/repaint notifier. The worker sends `ImageReady`, but cannot wake egui. `PrettyImageCache::poll_ready()` calls `ctx.request_repaint()` only *after another frame has already occurred and polled the receiver*.

That distinction matters. If TTS is idle and the user does nothing after an image decode finishes, there is no guaranteed frame that will call `poll_ready()`. The placeholder can therefore remain visible until unrelated input or some other repaint source occurs. Continuous TTS repaint can mask the defect, but image rendering must work while the reader is idle too.

The newly asynchronous pretty-block builder has the same structural problem. `start_pretty_builder()` sends `PrettyBuildResult` from its worker without a repaint/wakeup handle. `render_pretty_page()` polls results on a later frame, but the worker itself does not guarantee that later frame. A newly opened/changed pretty page can therefore remain in `Preparing pretty view…` until unrelated UI activity if the native event loop goes idle.

The global update loop schedules 24 ms repaints only while `tts_runtime.needs_repaint()` is true, so TTS activity is not an acceptable wakeup mechanism for these independent presentation workers.

## Required A2 correction

Preserve A1. Do not redesign its presentation, provenance, cache, or TTS architecture.

1. Give both asynchronous presentation workers an explicit egui wakeup path. The simplest acceptable form is a cloned `egui::Context` (or a narrow repaint notifier) owned by the worker and `request_repaint()` immediately after successfully publishing a completion/result.
2. Pretty image decode completion — success **and failure** — must wake egui so texture upload or placeholder/error state is consumed promptly while TTS is stopped and the user is hands-off.
3. Pretty block build completion must likewise wake egui so `Preparing pretty view…` transitions to ready content without mouse/keyboard/TTS activity.
4. Keep request submission nonblocking (`try_send`/bounded queue). Do not turn repaint repair into render-thread waits.
5. Keep disk read, decode, source parsing, and target alignment off the render thread.
6. Add deterministic regression seams/tests proving a worker completion emits a repaint notification independently of a subsequent frame poll. Do not merely assert that `poll_ready()` itself calls `request_repaint()`.
7. Preserve every existing A1 test and Goal 0008/0009 regression.

## Acceptance after A2

After the correction branch is terminal and Windows CI is green, director review will re-check the wakeup path and then, if accepted, integrate Goal 0012 and request one focused real-desktop QA pass for presentation controls + inline images + a brief TTS synchronization sanity check.

No human QA is authorized before that acceptance.
