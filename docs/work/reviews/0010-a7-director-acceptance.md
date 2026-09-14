# Goal 0010 A7 — director acceptance

## Decision

**ACCEPTED FOR FOCUSED REAL-DESKTOP QA**

A7 repairs the last director-found concurrency defect in Goal 0010 without reopening the accepted provider/cover architecture.

## Accepted correction

- Cover completion ownership is per book/request rather than one global request-ID watermark.
- Independent books may complete in arbitrary order without dropping a valid lower-ID completion.
- Stale older completions for the same book cannot replace newer retry state.
- Automatic visible-row cover fetches and the manual `Ensure thumbnail` action use the same bounded/coalesced dispatch path.
- The four-request in-flight bound remains enforced.
- Cover network, disk, and image decode work remains off the egui/render thread.
- The A6 explicit `CalibreCoverCompleted` loaded / cover-unavailable / provider-unavailable / cover-error seam remains intact.
- The sibling Caliberate endpoint remains the explicit `/api/v1/books/{id}/cover` provider contract.

## Evidence

LanternLeaf implementation commit: `54e22c172745171b084221a6f80465e3f6ffe77d`.

Goal terminal commit: `fb02dde145e881a04c6052fba0baa2aff6e579db`.

Windows baseline run `34793890003` completed successfully. Both `native-workspace` and `hosted-renderer-probe` jobs passed, including workspace check/build/test, repo-native QA preparation, notification workflow test, Windows TTS diagnostics, and renderer capability probing.

The A7 worker also reports successful serialized workspace tests and deterministic coverage for reverse-order completion across two books, stale same-book completion after a newer retry, per-book reducer freshness, and repeated/manual bounded ownership.

Sibling `sguzman/caliberate` A6 endpoint branch `codex/0010-cover-endpoint-a6` was based directly on current sibling `main`, passed the post-change server suite, and has been fast-forward integrated to Caliberate `main` at `3799ccac03ce05404700efbbf33e489aa965f757`.

The LanternLeaf Goal 0010 branch was based directly on current director `main` and has been fast-forward integrated to LanternLeaf `main` at `fb02dde145e881a04c6052fba0baa2aff6e579db`.

## Remaining gate

Goal 0010 is not closed until one focused Windows pass verifies the behavior that cannot be established from worker/CI evidence alone:

1. With current Caliberate running, visible catalog rows acquire real covers before those books are opened/materialized.
2. Scrolling through the catalog does not leave visible rows permanently stuck on `Loading cover…` after requests complete.
3. Books without an advertised/available cover show an intentional non-black placeholder state.
4. Stop Caliberate, refresh/scroll enough to trigger cover work, and confirm provider-unavailable state is understandable and the library remains usable.
5. Restart Caliberate and confirm retry/refresh recovers covers without restarting LanternLeaf.
6. Repeated clicking of `Ensure thumbnail` on one row does not create visible runaway work, global busy lock, or permanent pending state.
7. Opening a representative Caliberate EPUB still works and the previously accepted reader/TTS path remains intact.

No further Codex correction is authorized unless this physical pass finds a concrete defect.
