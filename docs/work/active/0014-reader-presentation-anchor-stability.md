# 0014 — Reader presentation-geometry anchor stability

## Outcome

Make live Presentation geometry changes preserve the reader's semantic viewport position instead of leaving the old raw scroll offset attached to newly reflowed geometry. Highlight identity and TTS follow must remain correct across every geometry-affecting presentation configuration, not merely one hand-picked setting.

This goal also retains the previously queued media max-width/max-height observability work.

## Starting evidence

Goal 0012 correctly added a presentation geometry key and invalidates cached pretty-block heights when geometry changes. The key already covers `content_width`, horizontal/vertical margins, font size, line spacing, word spacing, letter spacing, font family/weight, and the full pretty configuration.

That solves stale-measurement/highlight-window correctness, but the real-desktop pass after Goal 0013 exposed a different failure: changing presentation geometry can leave the `ScrollArea` at the same numeric Y offset even though the document has reflowed. Horizontal margin is especially easy to reproduce because it changes content width and therefore wrapping/heights throughout the document. The result can be a jump to an unrelated semantic location even while canonical highlighting remains correct.

The user explicitly confirmed highlighting is currently fine and scrolling is the broken surface.

Do not reopen Goal 0012. Preserve its accepted presentation/image/TTS behavior and fix the missing viewport-anchor policy here.

## Architectural requirement

Treat geometry invalidation generically. Do **not** patch horizontal margin or media controls individually.

Before a geometry-affecting presentation change is allowed to invalidate/reflow the visible pretty document, retain a stable semantic viewport anchor. Prefer an anchor based on the first visible/near-visible canonical pretty block or canonical sentence plus a relative/fractional offset within that block where practical. After new geometry is measured, restore the viewport from that semantic anchor rather than reusing the old raw document-space Y offset.

Explicit playback follow has precedence over idle-view preservation: if `AutoScrollState` has a legitimate pending canonical TTS follow target, follow that canonical target. Otherwise preserve the user's current semantic reading anchor. Never invent a permanent highlighted-target forcing mode.

## Geometry classes in scope

The solution must work through the common geometry-change path for all settings represented by the production geometry key, including at least:

- horizontal margin / resulting content width;
- vertical margin / viewport inset;
- font family and weight when effective metrics change;
- font size and pretty base-font scale;
- line spacing;
- word spacing and letter spacing;
- paragraph/block spacing;
- heading scales and heading/block spacing;
- list/table geometry owned by the pretty configuration;
- media max width / max height and resulting image-block height;
- any future field added to the same geometry key, unless explicitly documented as non-anchoring.

The implementation should centralize this policy around geometry-key transition/reflow rather than maintain a list of per-widget scroll hacks.

## Authorized passes

### A — diagnose current scroll ownership

- Trace the current `ScrollArea::vertical().id_source("pretty_page")` offset lifecycle through a geometry-key change.
- Prove whether the observed jump is old raw Y surviving new document geometry, an estimated-height transition, an egui scroll-state reset, or a combination.
- Identify the minimum state/seam needed to represent a semantic pretty viewport anchor without moving document work onto the render thread.

### B — generic semantic anchor continuity

- Capture a semantic anchor from the currently visible pretty window before/at geometry transition.
- Invalidate stale measured heights exactly as Goal 0012 requires.
- Restore/reconcile the viewport against the new geometry once enough information is available.
- Avoid visible oscillation, repeated forced scrolling, or fighting ordinary user wheel/drag input.
- Preserve bounded virtualization and overscan.

### C — playback precedence and highlight invariants

- Preserve canonical highlight ownership independent of layout geometry.
- During active TTS, a genuine one-shot canonical follow request may override idle anchor preservation for that transition.
- After follow is consumed, ordinary render-window selection must remain natural; do not permanently force the highlighted block into the window.
- Presentation changes during playback must not make the spoken highlight flash/disappear or send the viewport to a stale unrelated block.

### D — media sizing retained from the original Goal 0014

- Reproduce live media max-width/max-height changes around a currently visible inline image.
- Distinguish non-binding limits from broken wiring/stale geometry.
- A genuinely binding max-width or max-height change must visibly resize the image while preserving aspect ratio.
- The semantic viewport/media anchor must remain stable across that reflow.

### E — production-adjacent regressions

Add deterministic/stateful coverage that changes geometry A -> geometry B while preserving a semantic anchor across multiple positions in a non-uniform document. Cover representative classes rather than only one slider:

- a large horizontal-margin/content-width change that materially changes wrapping;
- a text-metric change (font size or line spacing);
- a block-spacing/heading-scale change;
- a media-height change;
- at least one compound change affecting more than one field;
- idle/user-reading mode and active TTS follow mode.

Tests must prove semantic block/sentence continuity, not merely that numeric scroll offsets remain close.

## Acceptance gates

1. Changing horizontal margin at an arbitrary mid-book location no longer jumps to an unrelated semantic area.
2. The same anchor-continuity policy works for other geometry-affecting presentation settings; the implementation is not horizontal-margin-specific.
3. Geometry-key changes still invalidate stale measured pretty heights before ordinary render-window selection relies on them.
4. Idle presentation editing preserves the user's semantic reading position with bounded visual displacement.
5. During active TTS, canonical pending follow takes precedence correctly and the spoken sentence remains continuously highlighted/visible through the transition.
6. No permanent target forcing or canonical playback ownership redesign is introduced.
7. Binding media max-width/max-height changes visibly resize eligible media with preserved aspect ratio; non-binding limits are explicitly distinguishable.
8. Bounded virtualization, image workers, pretty-build workers, and heavy-work-off-render-thread policy remain intact.
9. Goal 0008/0009 TTS behavior, Goal 0012 presentation/image behavior, and Goal 0013 starter-shell behavior remain green.
10. Workspace validation and Windows CI pass.

## Non-goals

- reopening/redesigning Goal 0012's canonical highlight architecture;
- starter-shell containment (Goal 0013 is closed);
- Caliberate catalog-cover/provider UX (Goal 0010);
- Windows Natural/HD voices;
- PDF work;
- broad aesthetic redesign.

## Repository handoff

Use branch `codex/0014-reader-presentation-anchor-stability`.

Codex should synchronize current director `main`, move this goal `ready -> active`, launch/re-arm the Goal 0014 watcher, execute the full authorized work, run repository and Windows gates, write `docs/work/reports/0014.md`, terminalize only on full success or a true escalation condition, push before signaling terminal state, and restore the shared checkout to `main`.

Do not request human QA during implementation. The director will review the pushed branch first.
