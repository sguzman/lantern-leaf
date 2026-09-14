# 0016 — Starter library live-state continuity

## Outcome

Make the starter library surface stay truthful and useful during the same running session: a very large Caliberate catalog must not present as an empty library while a cold fetch walks the entire provider, and a successfully opened source must appear in Recents without requiring an application restart.

## Starting evidence

Goal 0010 A7 is functionally successful on the real Windows desktop. With the current Caliberate server running, LanternLeaf reaches the real 105,570-book catalog, real catalog covers appear before first open, covers continue to populate lazily as the user scrolls, representative EPUBs materialize/open extremely quickly, and the accepted EPUB TTS / pretty-reader / visual-settings path remains good.

Two starter-shell continuity defects were exposed by that QA:

1. **Cold catalog blank-wait.** The QA cache had no compatible full catalog cache, so LanternLeaf fetched Caliberate in 500-book pages. Caliberate itself caps `/api/v1/books` browse pages at 500. LanternLeaf accumulated the entire result internally and did not publish useful catalog rows until the full ~105k walk completed. During that time the starter shell showed an empty catalog plus `Loading…`, even though Caliberate was healthy and pages were arriving continuously. Restarting LanternLeaf restarted the walk from offset zero.
2. **Recents stale in-session.** Opening an EPUB persisted the source/cache state correctly, but the in-memory starter Recents model was not refreshed after successful source open. Returning to the starter in the same process could still show `No recent books yet.` Restarting the app caused the persisted entries to appear immediately, proving persistence worked and the defect is live-state refresh, not cache loss.

The isolated `qa.ps1` cache behavior is intentional and must remain isolated from ordinary user cache state.

## Architectural direction

### A. Progressive / cache-first catalog presentation

The catalog fetch remains background work. Never move HTTP, JSON parsing, disk I/O, thumbnail decode, or whole-catalog processing onto egui's render thread.

For a cold or incompatible cache, publish useful catalog state incrementally instead of waiting for all pages. Prefer a page/batch event seam owned by the app state, with enough metadata to distinguish an in-progress partial catalog from a complete catalog. The UI should be able to show rows after the first successful page while the worker continues fetching subsequent pages.

Requirements:

- preserve the existing stable final catalog ordering and complete result once loading finishes;
- preserve existing cached-catalog fast path when a compatible cache exists;
- do not issue one request per book and do not increase Caliberate's provider limit by inventing an unsupported contract;
- preserve cancellation/stale-request ownership so an old fetch cannot append into a newer refresh;
- do not duplicate rows when batches replay or arrive after cancellation;
- keep client-side search/sort semantics truthful while the catalog is partial. If search/sort only covers loaded rows during refresh, expose that state rather than implying completeness;
- show useful progress such as loaded count / provider total when available, without noisy diagnostics;
- a provider failure after some pages have loaded should leave already-valid rows usable and clearly mark refresh/provider failure rather than replacing them with an empty library;
- once the complete catalog is available, write the normal durable catalog cache and clear partial-loading state.

The implementation may introduce a dedicated catalog-batch event/contract or an equivalent reducer seam. Avoid giant full-vector clones on every 500-row batch if a bounded append/update model is practical.

### B. Same-session Recents refresh

After a source has successfully opened and its recent-source persistence is complete, refresh/update the in-memory Recents model so returning to Starter immediately shows that source.

Requirements:

- do not rely on application restart;
- preserve existing durable recent-book derivation from the cache rather than introducing a second divergent persistence format;
- refresh only after successful open/persistence, not on failed or cancelled opens;
- preserve current ordering/dedup semantics;
- Caliberate-opened materialized sources, normal local files, and browser-tab sources must not regress;
- do not perform expensive recent discovery synchronously on the render thread.

## Acceptance gates

1. With an empty/clean QA catalog cache and the real ~105k Caliberate provider, the first catalog rows become visible well before the complete catalog finishes loading.
2. The UI communicates that the catalog is still loading/partial and, when available, shows meaningful progress rather than appearing empty or frozen.
3. The complete catalog eventually reaches the same stable final set/order and durable cache behavior as before.
4. Cancelling/restarting/refreshing a catalog load cannot mix stale batches from an older request into the current catalog.
5. Provider failure mid-refresh preserves already-valid rows and reports provider failure without misclassifying books as corrupt.
6. Opening a representative EPUB and returning to Starter in the same LanternLeaf process shows it in Recents immediately; no restart is required.
7. Restarting LanternLeaf still reconstructs the same recent entry from durable cache state.
8. Existing Goal 0010 lazy-cover behavior remains green: visible-row covers load off-thread, no permanent `Loading cover…` state, no EPUB materialization for thumbnails, bounded/coalesced cover ownership.
9. Existing EPUB open/TTS/highlight/visual-settings behavior remains green.
10. Workspace, Windows CI, repo-native QA preparation, and renderer probe pass.

## Non-goals

- PDF visual/TTS/highlight work.
- Goal 0015 highlight viewport-band polish.
- Windows Natural/HD voices.
- Caliberate database/library redesign.
- Removing `qa.ps1` cache isolation.
- Broad starter-shell redesign.

## Repository handoff

Implementation branch: `codex/0016-starter-library-live-state-continuity`.

Before implementation, synchronize current director `main`, move this file from `ready/` to `active/`, and re-arm the repository watcher for this Codex Goal.

Inspect the current LanternLeaf catalog event/reducer/effect ownership before changing it. Inspect the current Caliberate `/api/v1/books` contract only to respect its real pagination behavior; no sibling Caliberate code change is expected or authorized unless a concrete blocker is demonstrated first.

Add deterministic reducer/worker tests for progressive batches, stale-request rejection, partial-provider failure, final completion/cache semantics, and same-session Recents refresh after successful open.

Keep all heavy/blocking work off the egui/render thread. Run focused tests, `cargo check --workspace`, serialized workspace tests, QA build/preparation, and the required Windows CI. Write `docs/work/reports/0016.md`, terminalize only on success or a real escalation, push before terminal signaling, and restore the shared checkout to `main`.

Do not request human QA during implementation. The director reviews the repository first.