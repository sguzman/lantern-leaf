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

- [x] Reader playback does not visibly lag unrelated controls such as the speed dial, settings toggles, and panel tabs.
- [x] Pretty-text playback remains responsive on large HTML/EPUB chapters with images.
- [x] Sentence/paragraph highlight and auto-scroll stay smooth without visible stutter.
- [x] Switching between settings/stats and reader interactions does not feel delayed during active playback.
- [x] Starter/calibre scrolling does not produce bursty frame drops when thumbnails hydrate.
- [x] UI code paths are measurable with repeatable before/after profiling.

## Phase 1: Instrumentation and Baseline Profiling

- [x] Add explicit profiling hooks around the reader render path using `performance.mark` and `performance.measure`.
- [x] Add development-only tracing for:
  - [x] App rerender frequency
  - [x] `ReaderShell` rerender frequency
  - [x] pretty-view highlight application count
  - [x] HTML anchor-map rebuild count
  - [x] auto-scroll invocation count
  - [x] starter thumbnail override update count
- [x] Add a lightweight dev diagnostic panel or console summary that reports hot-path counts per minute during playback.
- [x] Capture baseline measurements for these scenarios:
  - [x] text-only playback on a short document
  - [x] pretty HTML playback on a large chapter with images
  - [x] pretty EPUB playback with image-heavy content
  - [x] toggling settings/stats while playback is active
  - [x] starter/calibre scrolling with cold thumbnail cache
  - [x] starter/calibre scrolling with warm thumbnail cache
- [x] Define target thresholds for acceptable render/update frequency before optimization work begins.

## Phase 2: Store Subscription Isolation

- [x] Refactor [App.tsx](/win/linux/Code/projects/lantern-leaf/ui/src/App.tsx) so it no longer subscribes to a broad app-store slice that changes on most reader/TTS updates.
- [x] Split store selectors into narrowly scoped subscriptions for:
  - [x] bootstrap/session shell state
  - [x] reader snapshot
  - [x] TTS event metadata
  - [x] starter/calibre state
  - [x] toast/error/telemetry state
- [x] Ensure components subscribe only to the state they actually render.
- [x] Prevent `ttsStateEvent` churn from forcing top-level app rerenders when the visible UI does not depend on it.
- [x] Audit all Zustand selectors for object identity churn and convert them to stable primitive/tuple selectors where appropriate.
- [x] Introduce selector helpers with explicit equality semantics for high-frequency state.

## Phase 3: ReaderShell Decomposition

- [x] Break [ReaderShell.tsx](/win/linux/Code/projects/lantern-leaf/ui/src/components/ReaderShell.tsx) into smaller components with isolated props and rerender boundaries.
- [x] Extract and memoize these reader subtrees separately:
  - [x] top toolbar
  - [x] search row
  - [x] text-only sentence list
  - [x] pretty markdown/html renderer container
  - [x] bottom TTS player widget
  - [x] right-side settings panel
  - [x] right-side stats panel
  - [x] right-side TTS tuning panel
  - [x] quick actions / speed dial
- [x] Ensure subcomponents receive only the smallest possible prop surface rather than the full `reader` snapshot.
- [x] Remove unnecessary inline closures and object literals from hot render paths when they prevent memoization.
- [x] Audit derived values in `ReaderShell` and move expensive derivations into `useMemo` or selector-level precomputation only where it materially reduces churn.

## Phase 4: TTS Event Flow and Render Churn Reduction

- [x] Audit [jobsSlice.ts](/win/linux/Code/projects/lantern-leaf/ui/src/store/slices/jobsSlice.ts) for duplicate state writes during playback progression.
- [x] Reduce paired `reader` plus `ttsStateEvent` update cascades where one update is sufficient.
- [x] Separate structural reader updates from transient playback tick metadata.
- [x] Ensure the UI only processes playback state transitions that change visible state.
- [x] Coalesce bursty TTS events on the frontend when multiple updates arrive within the same frame.
- [x] Avoid replacing the entire `reader` object when only a small TTS field changed, if backend and store architecture allow it safely.

## Phase 5: Pretty HTML/EPUB Sync Optimization

- [x] Stop rebuilding the HTML sentence-anchor map on every reader playback update.
- [x] Cache anchor-text extraction results by stable document/page identity.
- [x] Precompute normalized anchor text once per rendered pretty document.
- [x] Store HTML sync metadata separately from ephemeral playback state.
- [x] Rebuild anchor maps only when one of these actually changes:
  - [x] source path
  - [x] page identity/content hash
  - [x] pretty HTML content
  - [x] image/source substitutions that affect text-bearing nodes
- [x] Move expensive DOM scanning out of high-frequency playback effects.
- [x] Replace full-container `querySelectorAll` rescans with stable indexed anchor references where possible.
- [x] Keep native HTML sync mapping deterministic and monotonic while reducing runtime search work.

## Phase 6: Highlight and Mutation Work Reduction

- [x] Audit highlight application logic so it updates only when the effective target paragraph/sentence changes.
- [x] Restrict `MutationObserver` scope to the smallest subtree necessary, or remove it entirely if a deterministic render lifecycle can replace it.
- [x] Prevent highlight reapplication on irrelevant DOM mutations such as late image loads that do not affect the active anchor.
- [x] Ensure highlight logic uses cached element references or indexed lookup maps rather than repeated `querySelector` calls.
- [x] Batch highlight class removal/addition into a single DOM mutation step where possible.
- [x] Gate `requestAnimationFrame` retries so they do not accumulate under rapid TTS updates.
- [x] Add diagnostics for highlight misses, retries, and DOM-remap causes.

## Phase 7: Scroll and Layout Reflow Optimization

- [x] Audit `scrollSentenceIntoView` for forced synchronous layout reads and writes.
- [x] Separate measurement reads from scroll writes to avoid layout thrashing.
- [x] Short-circuit scroll logic when the active item is already acceptably visible.
- [x] Avoid repeated auto-scroll within the same paragraph anchor when only the sentence index changes.
- [x] Debounce or frame-limit scroll actions during rapid playback state changes.
- [x] Prefer anchor-based block alignment strategies that reduce repeated manual geometry calculations where possible.
- [x] Verify that visible scrollbars and sticky bottom player layout do not cause extra repaint or compositing churn.

## Phase 8: Quick Actions and Peripheral Control Responsiveness

- [x] Decouple `ReaderQuickActions` from the main `ReaderShell` playback rerender path.
- [x] Ensure the speed dial subscribes only to state required for its visible toggles.
- [x] Keep panel toggle buttons and player controls responsive during active playback.
- [x] Audit MUI components with expensive ripple/transition behavior and disable or simplify those effects where they materially contribute to lag.
- [x] Ensure settings and stats panel layout does not reflow unnecessarily on each playback tick.

## Phase 9: Starter Screen and Thumbnail Hydration Optimization

- [x] Audit [StarterShell.tsx](/win/linux/Code/projects/lantern-leaf/ui/src/components/StarterShell.tsx) visible-item thumbnail hydration flow.
- [x] Batch thumbnail override updates so multiple arrivals commit in one store update.
- [x] Avoid re-rendering the full calibre list when a single thumbnail arrives.
- [x] Cache warm thumbnail lookups aggressively to minimize repeat work while scrolling.
- [x] Introduce request throttling/backpressure so fast scrolling does not produce excessive async churn.
- [x] Consider virtualization or row memoization for large calibre lists if current rendering scales poorly.

## Phase 10: Data Shape and Backend Coordination

- [x] Review whether the backend can emit more granular reader-playback updates without replacing the full snapshot each tick.
- [x] If feasible, split stable document/page data from transient playback cursor data in the bridge contract.
- [x] Evaluate whether HTML/EPUB sync metadata can be computed once in the backend or cache layer instead of the hot frontend path.
- [x] Ensure any new bridge events remain stable and traceable with `tracing` instrumentation on the Rust side.
- [x] Add explicit logging around reader snapshot emission frequency and payload shape to confirm backend contribution to frontend lag.

## Phase 11: Testing and Regression Protection

- [x] Add focused tests for store selector stability and prevention of unnecessary rerenders.
- [x] Add component tests for memoized reader subcomponents.
- [x] Add regression tests that assert HTML sync maps are not rebuilt on pure TTS cursor changes.
- [x] Add regression tests that assert quick actions remain mounted and stable during playback updates.
- [x] Add regression tests for auto-scroll suppression within the same paragraph anchor.
- [x] Add starter-screen tests for batched thumbnail hydration behavior.
- [x] Add benchmark-style development tests or scripts for representative large-document playback scenarios.

## Phase 12: Manual Profiling and Acceptance Pass

- [x] Re-profile all baseline scenarios after the refactor.
- [x] Compare before/after metrics for:
  - [x] app rerenders per playback minute
  - [x] `ReaderShell` rerenders per playback minute
  - [x] HTML anchor-map rebuild count
  - [x] highlight retries/reapplications
  - [x] auto-scroll calls
  - [x] starter thumbnail update bursts
- [x] Validate that playback remains functionally correct in:
  - [x] text-only mode
  - [x] markdown pretty mode
  - [x] native HTML pretty mode
  - [x] EPUB native pretty mode
  - [x] PDF mode if it shares any reader hot paths
- [x] Confirm no regressions in highlight alignment, auto-centering, or panel state persistence.
- [x] Confirm speed dial, settings panel, stats panel, and bottom TTS player remain responsive under active playback.

## Recommended Implementation Order

- [x] Step 1: Add instrumentation and capture a baseline.
- [x] Step 2: Narrow app/store subscriptions in `App.tsx` and related selectors.
- [x] Step 3: Extract `ReaderQuickActions` and other non-playback UI from `ReaderShell` churn.
- [x] Step 4: Cache/precompute pretty HTML anchor-map data so playback no longer rescans the DOM.
- [x] Step 5: Reduce highlight and mutation-observer work.
- [x] Step 6: Optimize auto-scroll to minimize forced layout.
- [x] Step 7: Tackle starter thumbnail batching and list responsiveness.
- [x] Step 8: Re-profile, tighten regressions, and only then consider deeper backend event-shape changes.

## Acceptance Criteria

- [x] App-level rerenders are materially reduced during active playback.
- [x] `ReaderShell` no longer invalidates unrelated UI subtrees on every TTS tick.
- [x] Pretty-view playback does not trigger repeated full DOM rescans for sync mapping.
- [x] Highlight and auto-scroll remain smooth on large image-heavy HTML/EPUB content.
- [x] Quick actions, settings, stats, and the TTS player stay responsive during playback.
- [x] Starter/calibre list scrolling remains smooth even while thumbnails are hydrating.
- [x] Profiling evidence shows measurable improvement over baseline, not just subjective perceived improvement.
