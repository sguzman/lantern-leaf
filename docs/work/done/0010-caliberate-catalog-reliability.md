# 0010 — Caliberate catalog covers and provider-availability UX

## Outcome

Make the Caliberate-backed library trustworthy before a book is opened: catalog covers should be available through a first-class provider path, while ordinary provider-unavailable states should be reported clearly without being misdiagnosed as book materialization corruption.

## Corrected evidence

A previous desktop pass appeared to show Caliberate book `42866` failing during materialization. The user later clarified that **Caliberate was not running during that attempt**. That incident is therefore withdrawn as evidence of a LanternLeaf materialization/format defect.

Do not preserve or reintroduce a fake requirement to harden materialization based on that incident.

The remaining real catalog defect is independent and reproducible:

- Caliberate catalog entries can render black cover placeholders before a book has ever been opened;
- the same book can later display a real cover in Recents after the EPUB has been materialized locally and its embedded cover becomes available;
- this proves main-catalog cover presentation and Recents/local-cover presentation are using different availability paths.

The goal is to give the Caliberate catalog a first-class cover contract so books do not need to be opened/materialized before their covers can appear.

## Authorized passes

### A — inspect the actual Caliberate API contract

Inspect the current `sguzman/caliberate` server/API rather than guessing legacy Calibre routes.

Determine whether Caliberate already exposes cover bytes, a cover endpoint, a stable cover URL/path field, or cover-presence metadata.

If no first-class cover contract exists, coordinate the smallest explicit Caliberate-side API addition if authorized, or terminalize blocked with the exact cross-repository dependency. Do not probe invented legacy thumbnail URLs.

### B — provider availability state

A Caliberate service that is not running/unreachable is not a per-book materialization defect.

Requirements:

- detect/report provider-unavailable/connection-refused/timeout distinctly from a book-format/content failure;
- keep the library shell recoverable;
- retrying after Caliberate becomes available must work without restarting LanternLeaf where practical;
- diagnostics should identify the provider and availability class without dumping secrets or giant traces into the UI.

### C — first-class lazy catalog covers

For Caliberate books with cover availability:

- fetch covers lazily for visible/near-visible catalog rows only;
- perform network/disk/decode work off the egui render thread;
- cache successful cover bytes/textures and negative results with sane invalidation;
- bound in-flight requests/concurrency;
- never materialize/download the full EPUB merely to obtain a thumbnail;
- never issue cover requests for all ~100k catalog books at startup;
- preserve Recents/local embedded-cover fallback where appropriate.

### D — black-placeholder semantics

A missing/not-yet-loaded cover should have an intentional placeholder state rather than an unexplained black rectangle.

Differentiate at least:

- cover loading;
- no cover advertised/available;
- provider unavailable;
- cover fetch/decode failure.

Do not allow a transient provider failure to poison the negative-cover cache permanently.

## Acceptance gates

1. The withdrawn `42866` incident is not treated as evidence of a materialization-format bug.
2. Provider unavailable/connection failure is classified separately from book/content/materialization failure.
3. A provider-unavailable state is recoverable once Caliberate returns, without corrupting reader/library state.
4. Catalog cover loading uses an explicit Caliberate provider contract, not legacy route guessing or full-book download.
5. Visible-row/lazy cover scheduling is bounded and off the UI thread; a ~100k catalog does not create unbounded cover work.
6. A catalog book can display a cover before it has ever been opened/materialized.
7. Recents/local cover behavior remains working.
8. Placeholder/loading/no-cover/provider-error states are visually intentional and distinguishable enough for debugging/UX.
9. Existing Goal 0008/0009 large EPUB/TTS behavior and Goal 0012–0014 reader/presentation behavior remain green.
10. Windows CI and normal workspace validation pass.

## Director correction A6

A5 is rejected before human QA; see `docs/work/reviews/0010-a5-director-rejection.md`.

Preserve A5's explicit Caliberate cover route, `has_cover` propagation, bounded visible-row scheduling, off-render-thread request/decode work, and intentional placeholder UI, but correct its per-cover completion ownership:

- do not use the single full-catalog `OperationScope::CalibreLoad` boolean as ownership for independent concurrent thumbnail requests;
- every started cover request must produce an explicit book-identified terminal outcome and leave the pending set;
- provider unavailable, endpoint/no-cover, and fetch/decode failure must not remain stuck as `Loading cover…`;
- transient provider failure must remain retryable without permanent negative poisoning;
- synchronize current LanternLeaf `main` before replaying A5 so the accepted Goal 0014 closure and Goal 0015 queue state are retained;
- synchronize current `sguzman/caliberate` `main`, replay the minimal cover endpoint, add targeted post-change cover-route/content tests, and run the relevant Caliberate suite after the change;
- wait for required LanternLeaf Windows CI success before terminal signaling.

A6 implemented these pieces, but director review found one remaining concurrency flaw; see A7 below.

## Director correction A7

A6 is rejected before human QA; see `docs/work/reviews/0010-a6-director-rejection.md`.

Preserve A6's explicit `CalibreCoverCompleted` event seam, book identity, terminal outcome taxonomy, retry behavior, current-main synchronization, Caliberate endpoint/tests, and successful Windows CI. Correct only the remaining completion-ownership race and manual bypass:

- do **not** use one global `last_calibre_cover_event_request_id` watermark for multiple independent cover requests;
- independent worker effects can complete out of request-id order, so valid completions for different books must be processed regardless of global request-id ordering;
- track request freshness/ownership per book (or an equivalent concurrency-safe model) so an older completion for the same book cannot overwrite a newer retry;
- only the currently owned request may clear/replace that book's pending state;
- every started current request must leave pending with a terminal visible state;
- the `Ensure thumbnail` user action must use the same pending/concurrency ownership path or be disabled/coalesced while a request is pending, so manual clicks cannot bypass the bounded in-flight limit;
- keep all cover network/disk/decode work off the egui/render thread.

Required deterministic tests:

1. start cover requests for two different books with request IDs A < B;
2. deliver B's completion first and A's completion second;
3. prove **both** completions are consumed and both books leave pending correctly;
4. for one book, start a newer retry before an older request completes, deliver the newer completion, then deliver the stale older completion and prove the stale event cannot clobber the newer state;
5. prove repeated/manual ensure cannot create duplicate or unbounded concurrent work for the same visible catalog path.

Do not redesign or rework the sibling Caliberate endpoint unless a new concrete defect is found there. Re-run LanternLeaf focused/workspace/Windows gates and wait for required CI success before terminal signaling. Do not request human QA during A7; the director reviews first.

## Non-goals

- speculative format fallback/materialization hardening based on the withdrawn offline-provider incident;
- reflow/highlight polish queued as Goal 0015;
- Windows Natural/HD voice support;
- PDF implementation;
- broad library UI redesign.

## Repository handoff

Continue branch `codex/0010-caliberate-catalog-reliability` and the existing Goal 0010 Codex session. This is a correction/follow-up, **not a new macro-goal and not a reason to start a new Codex session**.

Synchronize current director `main`, move this goal `ready -> active`, re-arm the watcher, implement A7 narrowly on top of the accepted A6 foundation, run repository and Windows gates, update `docs/work/reports/0010.md`, terminalize only on full success, push before signaling terminal state, and restore the shared checkout to `main`.

Do not request human QA during implementation. The director will review the pushed A7 branch first.
