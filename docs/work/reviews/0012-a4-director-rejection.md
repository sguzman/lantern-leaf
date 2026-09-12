# Goal 0012 A4 director rejection

Status: **REJECTED BEFORE HUMAN QA; A5 CORRECTION REQUIRED.**

A4 implementation: `c92256fb429aea1d13707a811b0147e4337bbd05`
A4 terminal: `011d221c297cc37cd471ffe53dab86cd5171ef0f`
Green rerun: Windows workflow `34714389917`

## What A4 got right

The production correction is directionally correct and should be preserved:

- `LanternLeafApp` now carries a `FontRegistry` rather than the coarse `fonts_configured` boolean.
- Startup records aliases actually inserted into `egui::FontDefinitions`.
- Global body/heading/monospace styles ask the registry for safe families rather than unconditionally naming `LanternLeafProportionalRegular`, `LanternLeafProportionalBold`, and `LanternLeafMonospaceRegular`.
- Pretty-view family selection consults exact alias availability and falls back to built-in proportional/monospace families or a registered regular face instead of synthesizing a name from enum + global boolean.
- Fallback selection does not rewrite app/book presentation configuration.
- Workspace check/tests, QA preparation, Windows TTS probe, and hosted renderer probe passed on rerun `34714389917`.

## Why A4 is not accepted yet

### 1. Worker branch did not synchronize current director main

The terminal worker branch is not a fast-forward of current director `main`; it is ahead three commits and behind the director A4 documentation/status lineage. The worker report says it synchronized `557904e`, which predates the director A4 contract/status commits.

The next attempt must merge/rebase the actual current `main` into the existing Goal 0012 branch before implementation and keep the current director contract authoritative.

### 2. Required panic-class regression is missing

The A4 director contract deliberately required a deterministic controlled-font regression that installs prepared `FontDefinitions` / styles into an egui `Context` and forces text/layout access where practical. This was not optional ceremony: the real desktop failure was an epaint panic that all prior unit/CI gates had missed.

The new test `missing_optional_font_falls_back_to_bound_builtin_families` only calls `presentation_font_families()` and compares returned `FontFamily` values. It does not install font definitions, set text styles, or force egui/epaint layout. Therefore it cannot prove that the exact panic class `FontFamily::Name(...) is not bound to any fonts` is impossible.

### 3. Controlled-availability matrix is incomplete

The director A4 contract required explicit deterministic coverage for:

- requested Lexend absent while another font is available;
- unrelated aliases present without the requested/global aliases;
- proportional regular available but bold unavailable;
- monospace unavailable;
- per-book unavailable family selection;
- an available family/weight resolving to its real registered alias;
- every named family returned by production selection being present in prepared definitions/registry;
- presentation config remaining unchanged by fallback.

A4 added useful missing/partial helper coverage, but it does not establish the complete matrix above.

## A5 correction

Preserve `c92256f`; do not redesign the production solution unless the stronger tests expose a defect.

A5 must:

1. synchronize the actual latest director `main` into `codex/0012-pretty-presentation-controls-and-inline-images` before implementation;
2. restore the A5 ready contract as authoritative and re-arm the watcher;
3. extract only the minimum production-owned font-preparation/style-resolution seam needed for deterministic tests;
4. create controlled empty/partial/complete registries/definitions independent of host font inventory;
5. install those definitions/styles into an egui `Context` and force body/heading/monospace and representative pretty `LayoutJob` layout so the old epaint panic class is exercised;
6. cover the full missing/partial/available matrix and config-preservation requirement;
7. rerun workspace/Windows gates and terminalize on the same Goal 0012 branch/report lineage.

No human QA is authorized until A5 is director-accepted.
