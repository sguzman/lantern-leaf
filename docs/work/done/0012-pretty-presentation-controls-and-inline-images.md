# 0012 — A7 correction: bounded blockquote geometry and post-follow highlight proof

## Outcome

Preserve Goal 0012 A6 implementation `6dcf9815ef87303caa3a3421bb8cc9e832f6b8ea` and close the two remaining director blockers before another physical Windows pass:

1. blockquote decoration must be constrained to the actual measured quote block rectangle;
2. durable pretty highlight after one-shot follow consumption must be proven with the stateful geometry-change lifecycle requested by A6.

This is not a new macro-goal. Continue the existing Goal 0012 branch/report lineage.

## Starting evidence

A6 implementation `6dcf9815ef87303caa3a3421bb8cc9e832f6b8ea` and Windows workflow `34721716717` passed automated validation. Director review is recorded in `docs/work/reviews/0012-a6-director-rejection.md`.

A6 work to preserve:

- literal horizontal margin semantics with no hidden 720-px cap;
- vertical margin as viewport/frame inset rather than scroll-document padding;
- presentation geometry-key invalidation of measured pretty-block heights;
- vertically scrollable settings/presentation panel body;
- real-egui word/letter-spacing behavior;
- readable table/TOC widths with horizontal overflow;
- visible font availability/effective fallback state;
- media sizing regressions;
- inline EPUB imagery, A3 async wakeups, A4/A5 font safety, and Goal 0008/0009 TTS/canonical behavior.

## Mandatory first step

Before implementation:

1. fetch current director `main`;
2. synchronize it into `codex/0012-pretty-presentation-controls-and-inline-images` while preserving `6dcf981`;
3. make this A7 contract and `docs/work/reviews/0012-a6-director-rejection.md` authoritative;
4. re-arm the Goal 0012 watcher;
5. append Attempt A7 to `docs/work/reports/0012.md` only after synchronization.

## A — bind blockquote decoration to measured block geometry

A6 still paints the quote rule from `ui.max_rect()` inside the frame before the quote label establishes final measured geometry. That can produce a rule extending beyond the actual quote content.

Correct behavior:

- preserve blockquote/source-quote semantics;
- keep the A6 restrained tint/indent/rule style or improve it minimally;
- paint any left rule only from a rectangle whose top/bottom are derived from the final measured quote/frame/inner response;
- do not use an unconstrained pre-layout `ui.max_rect()` as quote-rule height authority;
- a short quote gets a short rule;
- a long quote gets a rule matching that quote block only, not the rest of the viewport/page;
- adjacent paragraphs/headings must never inherit or sit behind the quote rule;
- sentence highlight background inside a quote remains visible and bounded.

Preferred implementation shapes include painting after `Frame::show` from the returned response/inner-response geometry, or allocating an explicit quote rect that exactly owns the quote layout. Keep this local to rendering; do not change document semantics.

### Required regression

Use a real egui context/layout seam with at least:

- ordinary paragraph before quote;
- multi-line/long blockquote;
- ordinary paragraph after quote.

Prove the quote-decoration vertical extent is contained within the measured quote block rect and does not overlap the neighboring paragraphs. A pure constant assertion on rule width is insufficient.

## B — prove durable highlight after follow consumption under changed geometry

A6 likely fixed the physical flash/disappear symptom by invalidating stale measured heights, but its 64-boundary test does not exercise the lifecycle that failed on desktop.

Required deterministic regression shape:

1. construct representative bounded pretty blocks / height authority for geometry A;
2. seed at least some measured A heights as the renderer would after a frame;
3. change one or more geometry-affecting settings to geometry B and prove A measurements are invalidated/not authoritative;
4. for a canonical target, model/request the one-shot follow and select/render the target window;
5. record/consume that follow as production `AutoScrollState` does;
6. on the subsequent frame, run normal production pretty-window selection with no forced follow target while the viewport remains at the followed location;
7. prove the same active highlighted target remains inside the render window and therefore can stay visibly highlighted until the next canonical boundary;
8. repeat over at least 48 canonical boundaries / representative targets under geometry B.

The regression must connect the production geometry invalidation/window-selection/follow lifecycle. Do not satisfy this with a constant-height helper loop that manually begins every viewport at the target without exercising follow state.

### Production correction policy

Do **not** change canonical playback ownership.

Do **not** force the highlighted block into the render window forever: user-driven scrolling away from the spoken sentence must remain possible.

If the stronger stateful regression passes with current production logic, keep production highlight/window code unchanged. If it fails, make the smallest render-only correction necessary to retain the target after follow consumption while it is still naturally in/near the followed viewport.

## C — preserve all A6 accepted improvements

Re-run focused regressions proving:

- horizontal margin is literal/monotonic;
- vertical margin does not extend document length;
- settings panel body scrolls;
- word and letter spacing measurably affect real egui layout;
- TOC/table columns remain readable with bounded horizontal overflow;
- font unavailable/effective-fallback UI remains correct;
- image max width/height preserve aspect ratio;
- inline images still use bounded off-render-thread decode and completion wakeups;
- A4/A5 font fallback never returns unbound named families.

No broad redesign is authorized.

## Validation

Run:

- focused blockquote measured-rect regression;
- stateful geometry A -> B -> follow -> consume -> normal-window 48+ boundary regression;
- all A6 geometry/spacing/table/panel/font/media regressions;
- A3 async worker wakeup regressions;
- Goal 0008/0009 TTS/canonical/text-only regressions;
- `cargo test --workspace -- --test-threads=1`;
- `cargo check --workspace`;
- `cargo build --workspace`;
- `git diff --check`;
- repo-native Windows QA preparation;
- Windows TTS probes;
- hosted renderer probe;
- Windows CI.

If an unrelated known nondeterministic test flakes, rerun the identical implementation commit and document both runs rather than weakening A7 gates.

## Acceptance gates

1. Worker branch contains current director `main` before A7 implementation and preserves A6 `6dcf981`.
2. No blockquote rule uses unconstrained pre-layout `ui.max_rect()` height ownership.
3. Real egui regression proves quote decoration stays inside final measured quote block geometry and does not bleed into neighboring content.
4. A6 horizontal/vertical margin semantics remain intact.
5. A6 geometry invalidation remains intact.
6. Stateful geometry-A -> geometry-B regression proves stale measurements are not reused.
7. Follow is requested and consumed through production-adjacent state, then the next normal render-window selection still contains the active highlighted target while the viewport remains at the followed location.
8. The post-follow proof covers at least 48 canonical boundaries/targets.
9. User-driven scrolling is not replaced by permanent target forcing.
10. A6 panel scrolling, spacing, table/TOC, font fallback UI, and media sizing regressions remain green.
11. Inline images and async wakeups remain green.
12. Goal 0008/0009 TTS/canonical/text-only regressions remain green.
13. Heavy work remains off the egui/render thread.
14. Repository/Windows gates and Windows CI pass.
15. No human QA until director accepts A7.

## Explicit non-goals

Do not broaden A7 into starter-shell Goal 0013, Caliberate catalog-cover Goal 0010, Natural/Narrator/HD voices, PDF work, Piper management, or a new document-layout engine.

## Repository handoff

Continue branch `codex/0012-pretty-presentation-controls-and-inline-images` and report `docs/work/reports/0012.md`.

Move this goal `ready -> active`, append **Attempt A7**, implement only this bounded correction, validate, terminalize `done` or `blocked`, push, signal terminal state, and restore the shared checkout to `main` without merging.

## Human verification after director acceptance

If A7 is accepted, the next physical Windows pass should verify the already-defined Goal 0012 reader surface: literal margins, scrollable Presentation controls, visible word/letter spacing, readable TOC/table, restrained bounded blockquotes, understandable font fallback, idle inline images, and continuously visible synchronized TTS highlight/follow.
