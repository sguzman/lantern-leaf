# UI Lag Reduction Roadmap

## Goal

Reduce inconsistent UI lag and playback-time jank across the reader and starter screens by removing avoidable rerender churn, eliminating repeated DOM rescans, minimizing forced layout work, and isolating high-frequency TTS updates from the rest of the interface.

## Problem Statement

The current lag profile is caused by a stacked hot path rather than one isolated defect:

- broad app-level Zustand subscriptions cause large rerender surfaces
- playback updates emit both `reader` and `ttsStateEvent` churn
- `ReaderShell` owns too much UI under a single frequently changing prop object
- pretty-view sync logic rescans and mutates the DOM during playback
- scroll alignment performs layout reads and writes on active updates
- calibre thumbnail hydration can cause bursty starter-screen jank

The result is inconsistent latency:

- small/simple content can feel acceptable
- large HTML/EPUB pages with images and active playback can become very laggy
- controls that should feel cheap, such as the speed dial or panel toggles, inherit lag from unrelated reader updates

## Success Criteria

- [ ] Reader playback does not visibly lag unrelated controls such as the speed dial, settings toggles, and panel tabs.
- [ ] Pretty-text playback remains responsive on large HTML/EPUB chapters with images.
- [ ] Sentence/paragraph highlight and auto-scroll stay smooth without visible stutter.
- [ ] Switching between settings/stats and reader interactions does not feel delayed during active playback.
- [ ] Starter/calibre scrolling does not produce bursty frame drops when thumbnails hydrate.
- [ ] UI code paths are measurable with repeatable before/after profiling.

## Phase 1: Instrumentation and Baseline Profiling

- [ ] Add explicit profiling hooks around the reader render path using `performance.mark` and `performance.measure`.
- [ ] Add development-only tracing for:
  - [ ] App rerender frequency
  - [ ] `ReaderShell` rerender frequency
  - [ ] pretty-view highlight application count
  - [ ] HTML anchor-map rebuild count
  - [ ] auto-scroll invocation count
  - [ ] starter thumbnail override update count
- [ ] Add a lightweight dev diagnostic panel or console summary that reports hot-path counts per minute during playback.
- [ ] Capture baseline measurements for these scenarios:
  - [ ] text-only playback on a short document
  - [ ] pretty HTML playback on a large chapter with images
  - [ ] pretty EPUB playback with image-heavy content
  - [ ] toggling settings/stats while playback is active
  - [ ] starter/calibre scrolling with cold thumbnail cache
  - [ ] starter/calibre scrolling with warm thumbnail cache
- [ ] Define target thresholds for acceptable render/update frequency before optimization work begins.

## Phase 2: Store Subscription Isolation

- [ ] Refactor [App.tsx](/win/linux/Code/projects/lantern-leaf/ui/src/App.tsx) so it no longer subscribes to a broad app-store slice that changes on most reader/TTS updates.
- [ ] Split store selectors into narrowly scoped subscriptions for:
  - [ ] bootstrap/session shell state
  - [ ] reader snapshot
  - [ ] TTS event metadata
  - [ ] starter/calibre state
  - [ ] toast/error/telemetry state
- [ ] Ensure components subscribe only to the state they actually render.
- [ ] Prevent `ttsStateEvent` churn from forcing top-level app rerenders when the visible UI does not depend on it.
- [ ] Audit all Zustand selectors for object identity churn and convert them to stable primitive/tuple selectors where appropriate.
- [ ] Introduce selector helpers with explicit equality semantics for high-frequency state.

## Phase 3: ReaderShell Decomposition

- [ ] Break [ReaderShell.tsx](/win/linux/Code/projects/lantern-leaf/ui/src/components/ReaderShell.tsx) into smaller components with isolated props and rerender boundaries.
- [ ] Extract and memoize these reader subtrees separately:
  - [ ] top toolbar
  - [ ] search row
  - [ ] text-only sentence list
  - [ ] pretty markdown/html renderer container
  - [ ] bottom TTS player widget
  - [ ] right-side settings panel
  - [ ] right-side stats panel
  - [ ] right-side TTS tuning panel
  - [ ] quick actions / speed dial
- [ ] Ensure subcomponents receive only the smallest possible prop surface rather than the full `reader` snapshot.
- [ ] Remove unnecessary inline closures and object literals from hot render paths when they prevent memoization.
- [ ] Audit derived values in `ReaderShell` and move expensive derivations into `useMemo` or selector-level precomputation only where it materially reduces churn.

## Phase 4: TTS Event Flow and Render Churn Reduction

- [ ] Audit [jobsSlice.ts](/win/linux/Code/projects/lantern-leaf/ui/src/store/slices/jobsSlice.ts) for duplicate state writes during playback progression.
- [ ] Reduce paired `reader` plus `ttsStateEvent` update cascades where one update is sufficient.
- [ ] Separate structural reader updates from transient playback tick metadata.
- [ ] Ensure the UI only processes playback state transitions that change visible state.
- [ ] Coalesce bursty TTS events on the frontend when multiple updates arrive within the same frame.
- [ ] Avoid replacing the entire `reader` object when only a small TTS field changed, if backend and store architecture allow it safely.

## Phase 5: Pretty HTML/EPUB Sync Optimization

- [ ] Stop rebuilding the HTML sentence-anchor map on every reader playback update.
- [ ] Cache anchor-text extraction results by stable document/page identity.
- [ ] Precompute normalized anchor text once per rendered pretty document.
- [ ] Store HTML sync metadata separately from ephemeral playback state.
- [ ] Rebuild anchor maps only when one of these actually changes:
  - [ ] source path
  - [ ] page identity/content hash
  - [ ] pretty HTML content
  - [ ] image/source substitutions that affect text-bearing nodes
- [ ] Move expensive DOM scanning out of high-frequency playback effects.
- [ ] Replace full-container `querySelectorAll` rescans with stable indexed anchor references where possible.
- [ ] Keep native HTML sync mapping deterministic and monotonic while reducing runtime search work.

## Phase 6: Highlight and Mutation Work Reduction

- [ ] Audit highlight application logic so it updates only when the effective target paragraph/sentence changes.
- [ ] Restrict `MutationObserver` scope to the smallest subtree necessary, or remove it entirely if a deterministic render lifecycle can replace it.
- [ ] Prevent highlight reapplication on irrelevant DOM mutations such as late image loads that do not affect the active anchor.
- [ ] Ensure highlight logic uses cached element references or indexed lookup maps rather than repeated `querySelector` calls.
- [ ] Batch highlight class removal/addition into a single DOM mutation step where possible.
- [ ] Gate `requestAnimationFrame` retries so they do not accumulate under rapid TTS updates.
- [ ] Add diagnostics for highlight misses, retries, and DOM-remap causes.

## Phase 7: Scroll and Layout Reflow Optimization

- [ ] Audit `scrollSentenceIntoView` for forced synchronous layout reads and writes.
- [ ] Separate measurement reads from scroll writes to avoid layout thrashing.
- [ ] Short-circuit scroll logic when the active item is already acceptably visible.
- [ ] Avoid repeated auto-scroll within the same paragraph anchor when only the sentence index changes.
- [ ] Debounce or frame-limit scroll actions during rapid playback state changes.
- [ ] Prefer anchor-based block alignment strategies that reduce repeated manual geometry calculations where possible.
- [ ] Verify that visible scrollbars and sticky bottom player layout do not cause extra repaint or compositing churn.

## Phase 8: Quick Actions and Peripheral Control Responsiveness

- [ ] Decouple `ReaderQuickActions` from the main `ReaderShell` playback rerender path.
- [ ] Ensure the speed dial subscribes only to state required for its visible toggles.
- [ ] Keep panel toggle buttons and player controls responsive during active playback.
- [ ] Audit MUI components with expensive ripple/transition behavior and disable or simplify those effects where they materially contribute to lag.
- [ ] Ensure settings and stats panel layout does not reflow unnecessarily on each playback tick.

## Phase 9: Starter Screen and Thumbnail Hydration Optimization

- [ ] Audit [StarterShell.tsx](/win/linux/Code/projects/lantern-leaf/ui/src/components/StarterShell.tsx) visible-item thumbnail hydration flow.
- [ ] Batch thumbnail override updates so multiple arrivals commit in one store update.
- [ ] Avoid re-rendering the full calibre list when a single thumbnail arrives.
- [ ] Cache warm thumbnail lookups aggressively to minimize repeat work while scrolling.
- [ ] Introduce request throttling/backpressure so fast scrolling does not produce excessive async churn.
- [ ] Consider virtualization or row memoization for large calibre lists if current rendering scales poorly.

## Phase 10: Data Shape and Backend Coordination

- [ ] Review whether the backend can emit more granular reader-playback updates without replacing the full snapshot each tick.
- [ ] If feasible, split stable document/page data from transient playback cursor data in the bridge contract.
- [ ] Evaluate whether HTML/EPUB sync metadata can be computed once in the backend or cache layer instead of the hot frontend path.
- [ ] Ensure any new bridge events remain stable and traceable with `tracing` instrumentation on the Rust side.
- [ ] Add explicit logging around reader snapshot emission frequency and payload shape to confirm backend contribution to frontend lag.

## Phase 11: Testing and Regression Protection

- [ ] Add focused tests for store selector stability and prevention of unnecessary rerenders.
- [ ] Add component tests for memoized reader subcomponents.
- [ ] Add regression tests that assert HTML sync maps are not rebuilt on pure TTS cursor changes.
- [ ] Add regression tests that assert quick actions remain mounted and stable during playback updates.
- [ ] Add regression tests for auto-scroll suppression within the same paragraph anchor.
- [ ] Add starter-screen tests for batched thumbnail hydration behavior.
- [ ] Add benchmark-style development tests or scripts for representative large-document playback scenarios.

## Phase 12: Manual Profiling and Acceptance Pass

- [ ] Re-profile all baseline scenarios after the refactor.
- [ ] Compare before/after metrics for:
  - [ ] app rerenders per playback minute
  - [ ] `ReaderShell` rerenders per playback minute
  - [ ] HTML anchor-map rebuild count
  - [ ] highlight retries/reapplications
  - [ ] auto-scroll calls
  - [ ] starter thumbnail update bursts
- [ ] Validate that playback remains functionally correct in:
  - [ ] text-only mode
  - [ ] markdown pretty mode
  - [ ] native HTML pretty mode
  - [ ] EPUB native pretty mode
  - [ ] PDF mode if it shares any reader hot paths
- [ ] Confirm no regressions in highlight alignment, auto-centering, or panel state persistence.
- [ ] Confirm speed dial, settings panel, stats panel, and bottom TTS player remain responsive under active playback.

## Recommended Implementation Order

- [ ] Step 1: Add instrumentation and capture a baseline.
- [ ] Step 2: Narrow app/store subscriptions in `App.tsx` and related selectors.
- [ ] Step 3: Extract `ReaderQuickActions` and other non-playback UI from `ReaderShell` churn.
- [ ] Step 4: Cache/precompute pretty HTML anchor-map data so playback no longer rescans the DOM.
- [ ] Step 5: Reduce highlight and mutation-observer work.
- [ ] Step 6: Optimize auto-scroll to minimize forced layout.
- [ ] Step 7: Tackle starter thumbnail batching and list responsiveness.
- [ ] Step 8: Re-profile, tighten regressions, and only then consider deeper backend event-shape changes.

## Acceptance Criteria

- [ ] App-level rerenders are materially reduced during active playback.
- [ ] `ReaderShell` no longer invalidates unrelated UI subtrees on every TTS tick.
- [ ] Pretty-view playback does not trigger repeated full DOM rescans for sync mapping.
- [ ] Highlight and auto-scroll remain smooth on large image-heavy HTML/EPUB content.
- [ ] Quick actions, settings, stats, and the TTS player stay responsive during playback.
- [ ] Starter/calibre list scrolling remains smooth even while thumbnails are hydrating.
- [ ] Profiling evidence shows measurable improvement over baseline, not just subjective perceived improvement.
