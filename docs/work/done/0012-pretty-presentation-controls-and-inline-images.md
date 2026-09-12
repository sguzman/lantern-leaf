# 0012 — A6 correction: real-desktop pretty geometry and presentation-panel usability

## Outcome

Preserve the accepted Goal 0012 presentation/image/font-safety work while fixing the concrete pretty-layout and settings-panel defects exposed by the first successful physical Windows presentation pass.

This is not a new macro-goal. Continue the existing Goal 0012 branch/report lineage.

## Starting evidence

A5 implementation `46d70b34d2a560373e831471f24b58d17b1fe8bc`, terminal `e2938b2cf543df237beb79f83e5159e4e598b4cd`, and Windows CI `34718069167` were director-accepted for physical QA.

The physical pass then verified:

- LanternLeaf starts successfully on the machine/config that previously crashed;
- inline/cover EPUB imagery appears in pretty view;
- the Presentation section is visible;
- font size, base font scale, paragraph spacing, block spacing, H1 scale, and H2 scale visibly work;
- TTS spoken identity and follow/scroll remain synchronized.

The same pass exposed reader-layout failures documented in `docs/work/reviews/0012-a5-real-desktop-rejection.md`.

## Preserve prior accepted work

Preserve unless a direct correction requires the smallest local change:

- Goal 0008/0009 canonical TTS identity, highlight/follow ownership, text-only semantics, and no ordinary duplicate-line playback;
- A3 async pretty/image completion repaint wakeups;
- A4/A5 exact `FontRegistry` safety and deterministic missing-font fallback;
- Presentation app-default -> per-book persistence/reset;
- EPUB image provenance/reference normalization and inline image rendering;
- bounded pretty virtualization and off-render-thread pretty/image heavy work;
- no Natural/Narrator/HD voice work;
- no Caliberate catalog-cover work;
- no PDF work.

## Mandatory first step

Before implementation:

1. fetch current director `main`;
2. synchronize it into `codex/0012-pretty-presentation-controls-and-inline-images`;
3. make this A6 contract and `0012-a5-real-desktop-rejection.md` authoritative;
4. re-arm the Goal 0012 watcher;
5. append Attempt A6 to `docs/work/reports/0012.md` only after the branch is synchronized.

## A — make horizontal margin literal and remove hidden geometry

Current pretty layout conflates horizontal margin with an implicit centered 720-px max-width column.

Correct semantics:

- `margin_horizontal = 0` means no large LanternLeaf-imposed reading gutter beyond ordinary widget/frame padding;
- increasing horizontal margin creates a visible, approximately symmetric left/right inset in the pretty viewport;
- horizontal margin must respond monotonically across its UI range;
- do not silently cap the text column at 720 px under the name of "horizontal margin";
- if a separate maximum text-column width is still desirable, expose it as a separate explicit presentation policy/control rather than hiding it inside margin arithmetic;
- UI slider range should match the core-supported useful range instead of becoming inert on wide windows.

Add a deterministic geometry helper/test proving representative viewport widths produce different content widths/left insets for 0, medium, and high margins.

## B — make vertical margin a viewport inset, not document length

Current implementation inserts `vertical_margin` inside the scroll document at both ends.

Correct semantics:

- vertical margin reduces/insets the visible reading viewport at top and bottom;
- it must not merely prepend/append blank scrollable document space;
- changing vertical margin must not push the scrollbar/document bottom farther away by the same margin amount;
- top and bottom inset should be symmetric unless constrained by the actual available viewport;
- the scrollable content extent should be driven by document blocks, not artificial top/bottom padding.

Prefer a viewport/frame inset or equivalent layout ownership outside the scroll content.

## C — presentation geometry must invalidate virtualization measurements

Any setting that changes block geometry must invalidate or version the measured-height/prefix-sum cache used by pretty virtualization.

At minimum account for:

- font family/weight when metrics can differ;
- font size;
- base font scale;
- line spacing;
- word spacing;
- letter spacing;
- paragraph spacing;
- block/list spacing;
- heading scales;
- table cell geometry;
- media sizing settings when they change rendered block height/width;
- horizontal content width/margin changes.

Do not rebuild the semantic document or perform heavy work merely because a slider moves. The goal is lightweight render-geometry invalidation/re-measurement, not reparsing.

Required regression shape:

1. render/seed measured block heights for presentation geometry A;
2. change one or more geometry-affecting settings to geometry B;
3. prove stale A measurements are not reused as authoritative B prefix sums;
4. prove the active canonical highlighted block remains present/rendered after the follow request is consumed;
5. run 48+ canonical TTS boundaries under production pretty-window selection while geometry B is active and verify highlight visibility/follow ownership does not become a one-frame flash.

Do not change canonical playback ownership to fix a render-window bug.

## D — restore durable pretty highlight visibility

Physical symptom: spoken highlight appears briefly, then disappears, while speech identity and scrolling remain synchronized.

A6 must make the current canonical sentence visibly highlighted for the duration of that sentence whenever its pretty target is visible/renderable.

Acceptance:

- first-sample boundary changes canonical sentence identity;
- highlight becomes visible on the corresponding pretty sentence;
- after the auto-follow request is recorded/consumed, the same sentence remains highlighted until the next canonical boundary;
- virtualization may not evict the currently highlighted target solely because the one-shot follow request is no longer pending;
- if the target is genuinely outside the viewport after user-driven scrolling, preserve current user-control policy; do not introduce an aggressive permanent scroll lock.

## E — settings/presentation side panel must be scrollable

The expanded left SidePanel currently clips controls below the physical window.

Correct behavior:

- the side-panel body containing Settings/Stats/Search/TTS/diagnostics must have bounded vertical scrolling when content exceeds available height;
- top panel toggles should remain usable;
- every Presentation control, including controls below highlight appearance, must be reachable with wheel/thumb scrolling at the tested desktop height;
- no panel content may paint outside its allocated panel rect;
- avoid nested-scroll traps where ordinary wheel scrolling becomes impossible.

Add a small production seam/layout test where practical, but the code path itself must clearly own a `ScrollArea` or equivalent bounded body.

## F — word and letter spacing need real layout proof

The settings patch path currently stores/applies `word_spacing` and `letter_spacing`, and the renderer assigns `extra_letter_spacing`. Physical QA nevertheless found both controls visually ineffective.

Do not declare success from value plumbing alone.

Add deterministic egui layout tests that:

- layout identical representative text at zero and nonzero letter spacing and prove measurable width/glyph-position change;
- layout identical representative multi-word text at zero and nonzero word spacing and prove measurable inter-word/layout change;
- exercise the production `spans_to_job*` path, not a separate toy formatter;
- preserve mixed bold/italic/code spans.

If the current `extra_letter_spacing` implementation does not produce the required effect in the real egui version, replace it with the smallest reliable production technique.

## G — readable pretty tables / TOC

The current plain `Grid` may squeeze columns until text wraps nearly character-by-character.

Implement a bounded readable-width policy:

- derive column count from the table;
- provide a sensible minimum readable cell/column width;
- use available reader width when the table fits;
- when a table cannot fit without destructive squeezing, prefer horizontal table scrolling/overflow to character-level collapse;
- keep row striping/header emphasis/borders bounded;
- do not let a wide table expand the overall application/panel geometry or bleed outside the pretty viewport.

Add a deterministic fixture equivalent to a two-column book TOC with headers like `CHAPTER` / `PAGE`, Roman-numeral rows, long chapter titles, and page links. Assert the layout policy does not reduce the text column to near-character width.

## H — restrain blockquote presentation without losing semantics

Preserve blockquote/source quote semantics, but replace the visually dominant black vertical stripe with a bounded, subtle quote treatment.

Requirements:

- quote style remains distinguishable from ordinary paragraphs;
- any left rule/indent/background is constrained to the actual quote block rect;
- very long blockquotes must not create a dominant full-page black line;
- nested/long quoted text remains readable;
- the quote treatment must not interfere with sentence highlight rendering.

## I — font availability must be visible in the UI

A4/A5 safe fallback behavior is accepted and must remain.

Improve the Presentation font selector so a user can distinguish configured intent from effective availability:

- unavailable optional families must not look like a silent no-op;
- either annotate choices as unavailable/fallback, disable unavailable choices while preserving persisted intent, or display an explicit `Effective font: ...` / fallback indicator;
- do not reintroduce runtime font discovery in ordinary frames; consume the already prepared `FontRegistry`;
- do not mutate persisted `font_family` merely because the current machine lacks it.

## J — verify media sizing controls

Physical QA confirmed image rendering but did not conclusively verify media max width/height controls.

Add production regressions proving changing `image_max_width_pct` and `image_max_height_px` measurably changes `clamp_image_size()` / rendered target sizing while preserving aspect ratio and cache bounds.

No additional human media-specific pass is required beyond the final A6 desktop pass unless automation reveals a new defect.

## Validation

Run:

- focused pretty geometry/cache invalidation tests;
- persistent-highlight/follow regressions including 48+ boundaries;
- word/letter spacing real-egui layout tests;
- table width policy/TOC fixture tests;
- settings-panel containment tests/seams where practical;
- blockquote bounded-style test/seam where practical;
- media sizing regressions;
- A4/A5 font-safety layout matrix;
- A3 async worker wakeup regressions;
- Goal 0008/0009 TTS/canonical regressions;
- `cargo test --workspace -- --test-threads=1`;
- `cargo check --workspace`;
- `git diff --check`;
- repo-native Windows QA preparation;
- Windows TTS probe;
- hosted renderer probe;
- Windows CI.

## Acceptance gates

1. No hidden hard-coded 720-px column controls horizontal margin behavior.
2. Horizontal margin changes pretty viewport/content inset visibly and monotonically.
3. Vertical margin insets the viewing viewport rather than extending document scroll length.
4. Geometry-affecting presentation changes cannot leave stale measured-height/prefix caches authoritative.
5. Current spoken sentence remains visibly highlighted after the one-shot follow event is consumed.
6. 48+ production pretty-boundary transitions preserve highlight/follow under changed presentation geometry.
7. Presentation/sidebar controls are vertically scrollable and reachable at bounded desktop height.
8. Word spacing and letter spacing produce measurable real-egui layout changes through production formatting.
9. Realistic TOC/table content remains readable; overflow scrolls rather than collapsing to character-width columns.
10. Blockquotes remain semantically distinct without a dominant unbounded black rule.
11. Font fallback safety remains intact and the UI communicates unavailable/effective fallback state.
12. Media max width/height controls have regression evidence.
13. Inline EPUB images remain functional while TTS is idle.
14. Goal 0008/0009 TTS playback/canonical identity/text-only behavior remain green.
15. Heavy pretty/image/font discovery work remains off the render thread.
16. Repository/Windows gates pass.
17. No human QA until director accepts A6.

## Explicit non-goals

Do not broaden A6 into:

- starter-shell multi-column redesign beyond any shared SidePanel containment helper strictly required by E;
- Caliberate catalog covers;
- Natural/Narrator/HD voices;
- PDF rendering;
- Piper catalog/model management;
- a new document-layout engine.

The separate starter-shell overlap observed in the same physical session is tracked outside Goal 0012.

## Repository handoff

Continue branch `codex/0012-pretty-presentation-controls-and-inline-images` and existing report `docs/work/reports/0012.md`.

Move this goal `ready -> active`, append **Attempt A6**, implement only this correction, validate, terminalize `done` or `blocked`, push, signal terminal state, and restore shared checkout to `main` without merging.

## Human verification after director acceptance

One focused Windows pass should verify:

- no giant baked-in left gutter at zero/low horizontal margin;
- horizontal and vertical margin semantics feel literal;
- panel scrolling reaches all Presentation controls;
- word/letter spacing visibly changes text;
- TOC/table remains readable;
- blockquote styling is restrained;
- unavailable font behavior is understandable;
- inline images still render while idle;
- TTS pretty highlight remains continuously visible for each spoken sentence and follows correctly.
