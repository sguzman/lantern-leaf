# Goal 0016 A1 — director rejection

## Decision

**REJECTED BEFORE HUMAN QA**

A1 establishes the right general shape—progressive page events, partial/loading UI, stale-request reducer checks, and same-session Recents refresh after successful source persistence—but three ownership/continuity defects remain. Human QA would be premature because all three are visible from repository inspection.

## Blocking defects

### 1. Final catalog completion can erase already-loaded cover state

Goal 0010 made lazy cover state first-class and per-book. During a long progressive catalog load, visible rows can legitimately acquire `cover_thumbnail` values before the full catalog finishes.

A1's batch merge replaces an existing `CalibreBookDto` wholesale for duplicate IDs, and final `CalibreBooksLoaded` replaces the entire starter catalog wholesale. The progressive/catalog worker's final book vector does not own the asynchronously acquired lazy-cover state. Therefore a later metadata batch or final completion can replace a row that already has a live thumbnail with an otherwise-identical row whose `cover_thumbnail` is `None`, causing cover state to disappear/refetch at the end of the catalog walk.

A2 must preserve live per-book cover projection across metadata batch replacement and final catalog reconciliation. Incoming explicit non-empty cover state may replace old state; absent incoming cover state must not erase a thumbnail already acquired by the current UI/runtime.

Add deterministic coverage where a partial batch is applied, a cover completion populates a thumbnail, then a later duplicate batch and final catalog completion arrive; the thumbnail must survive.

### 2. Mid-refresh provider failure can be converted into apparent success by stale-cache fallback

The progressive Caliberate worker correctly emits fresh batches while paging. However `load_books_with_progress` retains the older one-shot fallback behavior: when provider paging fails, it may load a stale compatible-provider cache, emit that cache as another batch, and return `Ok(cached)`.

At the effect layer that becomes normal final `CalibreBooksLoaded` plus a `finished` progress event. The user therefore sees an apparently successful completed refresh instead of the required provider/refresh failure, and fresh partial rows can be replaced by stale cache state. This violates Goal 0016's truthful partial-provider-failure requirement.

A2 must make progressive refresh failure explicit. A stale cache may remain useful as previously-known/fallback data, but it must not convert a failed progressive provider refresh into a successful terminal refresh. If fresh partial rows have already arrived, preserve them and report the failure. If fallback cached rows are presented, preserve a clear failed/degraded refresh state rather than emitting ordinary success.

Add an end-to-end worker/effect test with at least one successful provider page followed by failure while fallback cache exists. Assert that usable rows remain and the terminal state reports failure/degraded provider status rather than ordinary completion.

### 3. Stale-request protection currently stops at the reducer; concurrent refresh workers remain unowned

The UI Refresh control can dispatch another catalog load while one is already active. Effect execution spawns independent worker threads and A1 passes no cancellation/ownership token into the catalog walk. The reducer rejects stale batches after a newer request ID becomes authoritative, but the older worker continues issuing provider requests and can still reach durable cache write. Two overlapping refreshes can therefore duplicate a full 105k walk and race to write the catalog cache; a logically stale request can finish after the newer request and overwrite durable state.

A2 must establish one authoritative catalog-load owner beyond the reducer. The smallest acceptable solution is to coalesce/disable a new catalog refresh while one is active, or introduce real request cancellation/generation ownership that also prevents stale workers from committing durable cache state. Do not solve this by doing coordination work on the render thread.

Add deterministic coverage proving an active catalog load cannot spawn an uncontrolled second full walk / stale durable completion.

## Preserve from A1

- Progressive Caliberate page/batch publication.
- Partial/loading counts and truthful UI wording.
- Book-ID deduplication and stale reducer rejection.
- Same-session Recents refresh only after successful `SourceOpen` persistence.
- Existing Goal 0010 four-request lazy-cover scheduler and off-render-thread cover work.
- No Caliberate sibling-repository changes.
- All heavy/blocking catalog, network, disk, parse, and image work off the egui/render thread.

## A2 validation

Run focused regression tests for the three defects above, then the existing Goal 0016 focused tests, `cargo check --workspace`, serialized workspace tests, QA build/preparation, and required Windows CI. Do not request human QA. The director reviews A2 first.
