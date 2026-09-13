# 0014 — Reader presentation-geometry anchor stability

## State

**A3 correction ready.** Preserve accepted A2 behavior and correct the residual real-desktop instability documented in `docs/work/reviews/0014-a2-real-desktop-rejection.md`.

## Outcome

Live Presentation geometry changes must preserve the reader's semantic viewport position **without visibly flashing through unrelated document areas while reflow settles**. Final semantic location and intermediate visual stability are both part of the contract.

Goal 0014 also owns the already-verified media max-width/max-height behavior; do not regress it.

## Accepted A2 foundation — preserve

A2 materially improved the reader and is not to be rewritten gratuitously:

- geometry-key transitions no longer intentionally reuse stale raw document-space Y as semantic truth;
- canonical sentence identity plus normalized within-sentence position is preferred when available;
- normalized same-block position is the fallback;
- a legitimate pending canonical TTS follow has precedence over idle anchor restoration;
- horizontal/content-width, text metrics, spacing, media sizing, and compound geometry use one centralized transition path;
- binding media max-width and max-height now have automated evidence and real-desktop verification;
- bounded virtualization, pretty/image workers, and heavy-work-off-render-thread policy remain required.

## A3 defect

The A2 real-desktop pass found two remaining failures:

1. some live presentation edits violently jump to an unrelated visual area and then return after recomputation;
2. some edits occasionally settle on the wrong semantic area, including cases where the highlighted canonical sentence had been visibly on screen before the edit. `Jump to highlight` still recovers, so canonical identity remains valid while viewport restoration can be wrong.

These are Goal 0014 defects, not a new cosmetic goal.

## A3 architectural direction

Treat presentation reflow as a **multi-frame transaction** rather than a sequence of independent one-frame estimated restores.

### Stable viewport witness

Maintain enough lightweight render metadata to identify the last stable visible semantic location from actual rendered geometry. Prefer, in order:

- an actually visible canonical sentence/segment witness plus its viewport-local position;
- otherwise an actually visible pretty block plus normalized/local position within its rendered rect;
- only then weaker estimated/raw-offset reconstruction.

If the current highlighted canonical sentence is visibly in the viewport at transaction start, it may be used as the one-shot geometry-edit anchor even when no TTS follow is pending. This is **not** permission to create permanent highlighted-target forcing.

### Reflow transaction ownership

On the first geometry-key change of an edit burst:

- capture the last stable viewport witness once;
- invalidate stale measured geometry as Goal 0012 requires;
- retain that original witness while subsequent geometry keys arrive during the same unsettled edit burst;
- do not recapture the anchor from an intermediate estimated/reflowing frame.

Coalesce rapid slider/control changes into that transaction. Define a deterministic settling condition based on stable geometry plus measured anchor-region evidence rather than an arbitrary long delay.

### Estimates vs measured geometry

Estimates may locate/render the anchor neighborhood and preserve bounded virtualization. They are not sufficient as final semantic authority when cumulative new measurements materially differ.

Once the relevant new geometry is actually rendered/measured, reconcile the scroll position so the stable semantic anchor remains at approximately the same viewport-local position. The user must not observe a distant semantic excursion as the implementation converges.

If egui's immediate-mode lifecycle requires more than one frame, design the transition so intermediate frames remain in the anchor neighborhood. Do not solve this by blocking/heavy work on the render thread.

### Priority / cancellation

- genuine pending canonical TTS follow > idle/edit anchor preservation;
- explicit user wheel/drag/scroll input during an unsettled reflow > stale automatic correction;
- after the transaction settles, automatic ownership ends and ordinary scrolling/render-window behavior resumes;
- no permanent follow, oscillation, or repeated forced scrolling.

## Required diagnosis

Before implementation, trace the production sequence around:

- `ScrollAreaState::load` for `pretty_page`;
- geometry-key invalidation and `pretty_block_heights.clear()`;
- `pretty_block_heights_for(...)` estimates;
- `vertical_scroll_offset(...)` restoration;
- virtualization/render-window selection;
- actual measured-height replacement over subsequent frames;
- repeated settings updates while dragging sliders.

Prove which parts cause the transient out-and-back jerk and occasional wrong final anchor. The director hypothesis in the A2 review is guidance, not a substitute for evidence.

## Required regression coverage

Add deterministic/stateful production-adjacent tests that model multiple frames and ownership transitions, including:

- geometry A -> B with deliberately inaccurate estimates versus final measured heights;
- rapid A -> B -> C -> D changes where all changes retain the original stable semantic witness;
- large mid-book cumulative reflow with non-uniform paragraph/media heights;
- visible highlighted canonical sentence preserved through an idle geometry edit without creating permanent auto-follow;
- active TTS pending-follow precedence;
- user scroll during a pending reflow canceling/superseding stale restoration;
- no intermediate or final render-window selection of an unrelated semantic neighborhood during convergence;
- media max-width/max-height binding and aspect-ratio behavior remains green.

Tests must cover transaction/state behavior, not merely pure anchor helper arithmetic.

## Acceptance gates

1. Large horizontal-margin/content-width edits at an arbitrary mid-book location do not visibly flash to an unrelated semantic area and return.
2. Final location remains in the same semantic reading area after representative font/text-metric, spacing, media, and compound changes.
3. Rapid slider/edit bursts do not compound anchor error by repeatedly capturing unstable intermediate frames.
4. A visibly highlighted sentence remains the one-shot semantic neighborhood when appropriate; `Jump to highlight` is not required to recover from an ordinary presentation edit.
5. Active TTS follow precedence and durable highlight behavior remain correct.
6. User scroll input wins over stale pending correction.
7. Binding image max-width/max-height changes remain visibly functional with preserved aspect ratio.
8. Bounded virtualization and off-render-thread heavy/image/document work remain intact.
9. Workspace validation and Windows CI pass.
10. Final focused real-desktop QA shows only bounded/local movement, not violent unrelated-area excursions or occasional semantic mis-anchors.

## Non-goals

- redesigning Goal 0012 canonical playback/highlight ownership;
- permanent highlighted-target forcing;
- broad reader visual redesign;
- Caliberate cover/provider UX (Goal 0010);
- Windows Natural/HD voices;
- PDF work.

## Repository handoff

Reuse Goal ID `0014` with a fresh Codex correction attempt. Start from current director `main`, read `AGENTS.md`, this ready contract, `docs/work/reviews/0014-a2-real-desktop-rejection.md`, and the A2 report. Preserve valid A2 code. Move this goal `ready -> active`, re-arm the Goal 0014 watcher, run the authorized correction, full repository/Windows gates and CI, append a new A3 report, terminalize only on full success or true escalation, push before signaling completion, and restore the shared checkout to `main`.

Do not request human QA during implementation; director review comes first.
