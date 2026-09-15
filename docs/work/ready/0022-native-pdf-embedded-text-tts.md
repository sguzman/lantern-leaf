# 0022 — Native PDF embedded-text / TTS trustworthy path — A11

## Status

**READY — A11 CORRECTION AFTER A10 DIRECTOR REJECTION**

Read first:

- `docs/work/reviews/0022-a10-director-rejection.md`
- `docs/work/reviews/0022-a9-real-desktop-rejection.md`
- `docs/architecture/pdf-text-recovery-boundary-2026-09.md`
- `docs/project/qa-evidence-ledger.md`

A10 is not authorized for physical QA. Preserve its accepted UI/state work, but close the two remaining evidence/correctness gaps at their actual runtime/lifecycle boundaries.

## Preserve

Preserve all accepted A10 direction:

- synchronous app-owned TTS Speed/Volume drafts;
- stale-ack resistance;
- bounded/coalesced drag-stop settings submission;
- fixed/bounded status presentation outside central reader geometry;
- Search Previous/Next;
- Enter/Shift+Enter navigation;
- selected X/Y + page provenance + bounded excerpt;
- `Start TTS at visible page` coarse PDF positioning;
- reader/source transition invalidation of in-memory Pretty presentation state;
- one process-wide Pdfium owner;
- visual-first PDF open;
- Goal 0020 continuous PDF responsiveness;
- trusted native embedded text and page-aligned canonical identity;
- off-egui document/cache/search work;
- Current/Nearby raster priority;
- idempotent `SetTextOnly { enabled }`;
- exact PDF visual sentence geometry/highlight/follow remains out of scope;
- Quack-check/Python/Docling/OCR/hostile recovery remain forbidden.

## A11 correction 1 — prove/fix burst seek through the actual runtime worker and first-sample ownership

A10's new burst seek test calls `TtsRuntime::apply_command()` synchronously. That does not reproduce the physical A9 failure boundary.

Required production-shaped harness:

- use `TtsRuntimeMode::Simulated` plus `SimulatedBoundaryDriver` or equivalent;
- submit playback/seek commands through the normal runtime command queue/worker path;
- start real simulated playback and emit accepted first-sample boundaries;
- issue rapid `SeekNext` and `SeekPrev` bursts while playback requests are being replaced;
- deliberately deliver a stale first-sample boundary from a superseded request/generation;
- prove stale boundary rejection;
- prove monotonic canonical movement one sentence at a time;
- cross native PDF page boundaries;
- skip empty native pages;
- clamp exactly at true beginning/end;
- prove no prior/current sentence replay caused by stale boundary ownership;
- add representative EPUB parity.

If the real harness reproduces the bug, fix runtime request/generation/boundary ownership. Do not paper over it only in ReaderSession helper methods.

## A11 correction 2 — diagnose same-EPUB close/reopen through real persistence/cache/session lifecycle

A10 clears in-memory Pretty caches on reader/source transitions, but did not satisfy the required persisted close/reopen diagnosis.

Build a real temporary-cache lifecycle regression using LanternLeaf persistence/cache/session services:

1. open an EPUB fixture with enough Pretty content to detect truncation;
2. stress Pretty/Text-only state changes;
3. persist/close through the normal lifecycle;
4. fully release the reader session;
5. reopen the same source from the configured temporary cache root;
6. render/build the Pretty projection without issuing TTS or Next;
7. verify complete document content immediately;
8. verify valid bookmark/canonical position survives;
9. verify stale in-flight Pretty results from the prior session/source generation cannot attach to the reopened session;
10. verify valid dual-view/structured artifacts can be reused;
11. verify corrupt/incomplete derived artifacts are rejected/rebuilt safely rather than trusted.

Investigate and identify the real owner of the A7-surviving truncated state. It may be in-memory, bookmark/session, dual-view artifacts, structured restoration, worker completion, or another derived artifact. Document the evidence.

Do not globally delete healthy caches as a workaround.

If the issue is proven to be only egui in-memory Pretty cache identity, keep the A10 reset but add the production lifecycle proof and document why durable artifacts were not culpable.

## Validation

Run at minimum:

- real asynchronous simulated-runtime SeekNext burst;
- real asynchronous simulated-runtime SeekPrev burst;
- stale first-sample after superseding seek;
- PDF empty-page/page-boundary seek cases;
- EPUB runtime seek parity;
- persisted same-source EPUB close/reopen after Text-only stress;
- complete Pretty document immediately on reopen;
- bookmark/canonical-position preservation;
- stale prior-session Pretty result rejection;
- corrupt/incomplete derived artifact recovery;
- A10 TTS draft regressions;
- A10 Search navigation regressions;
- A10 stable status/layout regressions;
- A10 `Start TTS at visible page` regression;
- A8/A9 Text-only/Search/natural page-continuation regressions;
- Goal 0019/0020 renderer regressions;
- representative EPUB visual/TTS regressions.

Then run:

- `cargo test --workspace -- --test-threads=1`;
- `cargo check --workspace`;
- `cargo build --workspace`;
- `git diff --check`;
- repo-native Windows QA preparation;
- fresh hosted `native-workspace`;
- fresh hosted `hosted-renderer-probe`.

Do not terminalize or signal Goal achieved until fresh hosted Windows validation for the substantive A11 lineage is green.

Update `docs/work/reports/0022.md` with A11 evidence.

Move ready -> active -> done normally.

Push before terminal signaling.

Restore shared checkout to `main`.

Do not request human QA. Director reviews A11 first.
