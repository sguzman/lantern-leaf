# Goal 0021 — PDF fit/manual zoom transition polish

## Status

**QUEUED — MINOR PDF UX POLISH**

Goal 0020 is physically accepted and closed. This goal captures one bounded real-desktop residual and must not preempt Gate 4 unless it becomes materially disruptive.

## Observed physical behavior

Fit width and Fit page themselves work. Reset/100% and manual `+`/`-` zoom also work across the tested 25%–400% range.

Residual: after entering Fit width or Fit page, pressing `+` or `-` exits fit mode but resumes stepping from the previously remembered manual zoom level rather than using the current effective fit scale as the transition baseline.

Example semantic expectation:

- user enters Fit page and the effective scale is about 72%;
- pressing `+` should enter manual mode at the next supported manual zoom step above ~72%, not jump back to an unrelated old manual scale;
- pressing `-` should analogously choose the next supported step below the effective fit scale.

## Direction

When a manual zoom step is requested while in a fit mode:

1. compute the current effective fit scale from the same authoritative production fit calculation used for presentation/render specification;
2. choose the next supported manual zoom level relative to that effective scale in the requested direction;
3. transition to manual mode at that level;
4. preserve the existing semantic viewport witness/focal anchoring and render-spec generation/replan behavior;
5. do not mutate the remembered manual level merely by entering or resizing within fit mode before an explicit manual transition.

Reset/100% remains an explicit manual 100% action.

## Acceptance

- Fit width -> `+` and `-` transition smoothly from the current effective fit scale.
- Fit page -> `+` and `-` transition smoothly from the current effective fit scale.
- fit scale changes caused by resize are respected when the later manual step occurs.
- no stale render-spec or blank-viewport regression.
- focal anchoring remains stable.
- 25%–400% manual zoom, Fit width, Fit page, Reset/100%, continuous scrolling, PDF page-domain navigation, and EPUB/TTS regressions remain green.

## Non-goals

Do not expand into PDF TTS/highlighting/OCR/Quack-check integration or broader toolbar redesign.
