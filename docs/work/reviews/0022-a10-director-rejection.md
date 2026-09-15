# Goal 0022 A10 — director rejection before physical QA

## Decision

**REJECTED BEFORE HUMAN QA. Do not integrate the A10 Codex branch and do not ask for physical Windows testing.**

A10 contains useful UI work and its fresh hosted Windows workflow is green, but two required correctness paths are not actually implemented/tested at the authority level requested by the A10 contract.

Fresh hosted workflow reported by Codex: `35019581500`.

- `native-workspace` job `104551641881`: passed.
- `hosted-renderer-probe` job `104555190742`: passed.

Green CI does not override the missing production-shaped regressions below.

## Accepted A10 direction to preserve

The following changes are directionally accepted and should be preserved in A11:

- app-owned synchronous TTS speed/volume drafts;
- stale-ack-resistant visible slider state;
- coalesced/drag-stop settings submission instead of one settings effect per drag frame;
- removal of `Last command` from central reader geometry;
- fixed/bounded bottom status presentation;
- Search Previous/Next controls;
- Enter/Shift+Enter Search navigation;
- selected match X/Y, native-page provenance, and bounded excerpt presentation;
- `Start TTS at visible page` / page-level PDF TTS positioning;
- clearing in-memory Pretty presentation/reflow cache on reader session/source transitions;
- all prior Goal 0022 native/Pdfium/text/cache/recovery boundaries.

## Blocker 1 — burst seek physical failure is still not covered by the real runtime/first-sample path

The A10 contract explicitly required burst `SeekNext` / `SeekPrev` through the real `TtsRuntime` control worker with simulated first-sample events, including stale first-sample rejection after a seek cancels/replaces playback.

A10 instead adds `simulated_runtime_burst_seeks_are_monotonic_across_pdf_pages`, but the test calls `runtime.apply_command(TtsCommand::SeekNext/SeekPrev)` synchronously and inspects the immediate returned view. It does not submit commands through the asynchronous runtime loop, does not use `SimulatedBoundaryDriver`, and does not inject a stale first-sample event from the superseded playback request.

This matters because the physical A9 defect was specifically that repeated Previous could repeat after interactive/runtime churn. The synchronous ReaderSession transition was already capable of moving backward; the suspected failure boundary is runtime request/boundary ownership. A helper/synchronous transition test cannot prove that defect is fixed.

The substantive A10 production diff in `tts_runtime.rs` is test-only. No runtime generation/boundary correction accompanies the new burst-seek test.

A11 must reproduce the interaction through the real worker path:

1. start simulated playback;
2. obtain/emit first-sample boundaries through the runtime driver;
3. issue rapid accepted seek commands through the normal runtime command submission path;
4. deliberately deliver a stale boundary from the superseded request/generation;
5. prove it is rejected and cannot reassert the sentence being left;
6. prove monotonic forward/backward canonical ownership across native pages/empty pages;
7. prove true beginning/end clamp exactly once.

If no production code change is needed after that real reproduction, the test must demonstrate why. If it reproduces, fix the runtime ownership bug rather than ReaderSession helper semantics.

## Blocker 2 — the stale EPUB reopen requirement was replaced by an in-memory cache reset, not diagnosed through persistence/cache/session reopen

The A10 contract required reproduction through the actual close/reopen persistence/cache/session path and identification of the owner of the previously truncated-looking EPUB state.

A10 only clears egui in-memory Pretty cache/reflow fields when `last_reader_source` changes:

- `pretty_page_cache_key`;
- `pretty_page_cache_blocks`;
- `pretty_sentence_targets`;
- `pretty_block_heights`;
- pending Pretty build/geometry/reflow/witness state.

That is a reasonable defensive reset, but it does not establish whether the A7-damaged EPUB state came from:

- persisted bookmark/cursor state;
- dual-view content artifacts;
- structured-document restoration;
- book overrides/text-only state;
- stale Pretty worker completion;
- or only in-memory presentation cache.

No cache/persistence/document-loading source changed in A10, and the A10 evidence report does not cite the required persisted close/reopen regression.

A11 must add a production-shaped lifecycle regression that actually closes and reopens the same EPUB using the repository persistence/cache services (temporary QA cache root is fine), after stressing Text-only/Pretty state. It must verify:

- complete Pretty document is available immediately after reopen;
- no TTS/Next command is required to heal it;
- valid bookmark/canonical position survives;
- stale in-flight Pretty completion from the prior session cannot attach to the reopened session;
- healthy durable dual-view artifacts are reused when valid;
- corrupt/incomplete derived artifacts are rejected/rebuilt rather than blindly trusted.

If investigation proves the bug was only egui in-memory Pretty identity, document and deterministically prove that with the real close/reopen lifecycle. Do not claim a durable-cache repair without evidence.

## Scope remains unchanged

A11 must not add:

- PDF sentence rectangles;
- visual PDF spoken highlight;
- visual PDF auto-follow;
- exact rendered-text click-to-sentence;
- Quack-check;
- Python;
- Docling;
- OCR;
- hostile/mixed recovery.

Goal 0024 still owns Caliberate materialized-source title/cover identity in Recents.

## Director conclusion

A10 is close and its UI corrections should be preserved, but the two hardest real-desktop failures were converted into weaker evidence rather than closed at their actual lifecycle/runtime boundaries. Goal 0022 remains open as A11.
