# Goal 0012 A6 — Director review

## Decision

**REJECTED BEFORE HUMAN QA; A7 BOUNDED CORRECTION REQUIRED.**

A6 contains substantial accepted progress and must be preserved, but it does not yet satisfy the blockquote geometry acceptance gate and its durable-highlight regression does not prove the post-follow lifecycle required by the A6 contract.

## Accepted A6 work to preserve

Implementation `6dcf9815ef87303caa3a3421bb8cc9e832f6b8ea` is cleanly based on director `main` and Windows workflow `34721716717` passed both native-workspace and hosted-renderer jobs.

The following A6 work is accepted in direction and should not be redesigned unless the A7 proof exposes a concrete defect:

- horizontal margin now uses literal symmetric viewport inset semantics and no hidden 720-px text-column cap;
- vertical margin is owned outside the scroll document as a viewport/frame inset;
- presentation geometry changes clear measured pretty-block heights through a geometry key;
- the settings/presentation panel body is vertically scrollable;
- word and letter spacing use the production LayoutJob path and have real-egui width evidence;
- tables/TOCs receive readable minimum column widths and horizontal overflow;
- font availability/effective fallback is visible without mutating configured intent;
- media width/height sizing regressions are present;
- A3 async wakeups, A4/A5 font safety, inline images, and Goal 0008/0009 TTS semantics remain preserved.

## Blocker 1 — blockquote rule still paints against the wrong rectangle

The physical A5 failure was not merely that the blockquote rule was dark/thick; its geometry was owned by an unbounded child-UI rectangle.

A6 makes the rule thinner/lighter and adds a restrained tint/indent, but production still does this inside `Frame::show` before the quote label establishes its final measured response:

```rust
let rect = ui.max_rect();
ui.painter().line_segment(
    [rect.left_top(), rect.left_bottom()],
    Stroke::new(BLOCKQUOTE_RULE_WIDTH, border_color),
);
response = Some(ui.add(Label::new(job).wrap(true)));
```

`ui.max_rect()` is the child UI's available maximum rectangle, not the measured quote block response rectangle. Therefore the left rule can still extend substantially beyond the actual quote content. This fails A6 requirement H: any rule/background treatment must be constrained to the actual quote block rect.

A7 must paint the rule from geometry obtained after/with actual quote layout (for example the final frame/inner response rect or an explicit allocated quote rect), not from an unconstrained pre-layout `ui.max_rect()`.

## Blocker 2 — durable-highlight regression does not prove the requested lifecycle

A6 production correctly invalidates measured heights when the presentation geometry key changes. That is likely the principal fix for the physical one-frame-highlight symptom.

However the required regression was specifically stateful: seed geometry A measurements, change to geometry B, consume the one-shot follow, then prove the active canonical highlighted block remains rendered while it is still in the viewport, across 48+ boundaries.

The added `changed_geometry_keeps_pretty_follow_window_bounded_for_64_boundaries` test uses one constant height vector, manually positions the viewport at each target, and calls `pretty_render_window(..., target=None)`. It does not seed/reuse A measurements, transition to B, exercise follow request -> record/consume -> subsequent normal selection, or connect to the production condition:

```rust
follow_requested.then_some(highlight_block_idx).flatten()
```

This is insufficient evidence after the exact physical regression was a highlight that flashed during follow and vanished afterward.

A7 does **not** need to redesign canonical playback or force the highlighted target forever. It must add a production-adjacent stateful regression that proves the post-follow behavior. If the stronger regression exposes a real render-window bug, apply the smallest render-only correction while preserving user-driven scroll freedom.

## A7 scope

A7 is deliberately narrow:

1. preserve A6 implementation `6dcf981`;
2. bind blockquote rule/tint geometry to the final measured quote block rectangle;
3. add a deterministic long-quote egui/layout regression proving the rule cannot exceed the quote block rect or bleed into adjacent content;
4. add a stateful geometry-A -> geometry-B -> follow -> consume -> normal-window regression over at least 48 canonical boundaries;
5. change production highlight/window logic only if that stronger test exposes an actual defect;
6. rerun all Goal 0012, Goal 0008/0009, repository, Windows TTS, renderer, and Windows CI gates.

No human QA is authorized until A7 is terminal, CI-green, and director-accepted.
