# 0012 — A5 correction: synchronize font fix and prove real egui layout safety

## Outcome

Preserve the successful Goal 0012 presentation/image/async-wakeup implementation and the A4 production font-registry correction while closing the two remaining director blockers: the worker branch must synchronize the actual current director `main`, and the missing-font fix must be proven against the real egui/epaint layout path rather than resolver arithmetic alone.

## Starting evidence

Real desktop QA after A3 crashed before functional testing with:

`FontFamily::Name("LanternLeafProportionalRegular") is not bound to any fonts`

A4 implementation `c92256fb429aea1d13707a811b0147e4337bbd05` replaces the coarse `fonts_configured` boolean with exact alias tracking and safe built-in fallbacks. Windows workflow rerun `34714389917` is green.

Director review is recorded in `docs/work/reviews/0012-a4-director-rejection.md`.

## Preserve prior work

Preserve all accepted A1–A3 work and preserve the A4 production solution unless stronger testing exposes a real defect:

- separate Presentation controls and app/book persistence/reset;
- canonical sentence/highlight/follow identity;
- EPUB image provenance/reference resolution;
- bounded off-render-thread pretty preparation and image decode;
- A3 completion-triggered repaint wakeups;
- A4 `FontRegistry` exact-alias tracking and safe proportional/monospace fallback policy;
- accepted Goal 0008/0009 TTS behavior.

Natural/Narrator/HD voices, Caliberate catalog-cover work, and PDF remain out of scope.

## Mandatory first step — synchronize actual director main

Before implementation:

1. fetch the latest `main`;
2. merge or rebase that exact director state into `codex/0012-pretty-presentation-controls-and-inline-images` while preserving `c92256f`;
3. resolve lifecycle files so this A5 ready contract and `0012-a4-director-rejection.md` are authoritative;
4. re-arm the Goal 0012 watcher;
5. only then modify tests/seams.

Do not terminalize another branch that is behind director `main`.

## A — production-owned deterministic font-preparation seam

Extract only the minimum seam needed to test production behavior with controlled availability. It may construct/return prepared `FontDefinitions`, a `FontRegistry`, and resolved text styles/families from synthetic empty/partial/complete alias availability.

Do not move system font discovery into egui render frames. No new per-frame filesystem/fontdb work.

## B — force the exact egui/epaint layout path

At least one deterministic regression must create an egui `Context`, install controlled prepared font definitions/styles, and force text layout using the same production-resolved body/heading/monospace families. The test must fail if a returned `FontFamily::Name` is unbound.

Also force at least one representative pretty `LayoutJob` through egui layout using `presentation_font_families()` (or its production replacement) under missing/partial availability.

Do not satisfy this with string/name assertions alone.

## C — complete controlled-availability matrix

Required deterministic cases, independent of host system fonts:

1. requested `Lexend` absent while another alias/face exists -> startup/global body layout is safe;
2. unrelated alias exists but global proportional aliases do not -> old `inserted_any` false-positive cannot occur;
3. proportional regular exists but proportional bold does not -> body and heading/bold layout are both safe;
4. monospace aliases absent -> monospace/code layout is safe;
5. per-book requested family absent -> pretty layout falls back safely;
6. requested available family/weight -> actual registered LanternLeaf alias is selected;
7. every `FontFamily::Name` returned by production selection is present in the prepared definitions/registry;
8. fallback does not mutate the configured app/book `font_family` or `font_weight` values.

A compact table-driven test is encouraged if it uses the production resolver and real egui layout seam.

## D — validation

Run:

- focused font-registry/layout regressions;
- existing Goal 0012 presentation/image/wakeup regressions;
- Goal 0008/0009 TTS/canonical regressions;
- `cargo test --workspace -- --test-threads=1`;
- `cargo check --workspace`;
- `git diff --check`;
- repo-native Windows QA preparation;
- Windows TTS probe;
- hosted renderer probe;
- Windows CI.

If an unrelated known nondeterministic test flakes, rerun the identical implementation commit and document both runs; do not weaken A5 font gates.

## Acceptance gates

1. Worker branch contains the actual latest director `main` before implementation.
2. A4 production registry/fallback behavior is preserved or minimally corrected if real layout testing exposes a defect.
3. Controlled missing/partial-font tests force egui text layout and no panic occurs.
4. Representative pretty `LayoutJob` layout is safe under missing/per-book-unavailable font selection.
5. All named families returned by production selection are guaranteed bound.
6. Missing bold and monospace faces degrade safely.
7. Configured app/book font intent remains unchanged by fallback.
8. No new render-thread font discovery/I/O is introduced.
9. Existing Goal 0012 and Goal 0008/0009 regressions remain green.
10. Repository/Windows gates pass.
11. No human QA until director accepts A5.

## Repository handoff

Continue branch `codex/0012-pretty-presentation-controls-and-inline-images` and `docs/work/reports/0012.md`.

Append **Attempt A5**, move this goal `ready -> active`, implement only this bounded correction, validate, terminalize `done` or `blocked`, push, signal terminal state, and restore the shared checkout to `main` without merging.

## Human verification after director acceptance

One focused Windows pass will first verify that the same machine/config that produced the A3 font panic now starts LanternLeaf normally. Only after startup succeeds should QA continue to presentation controls/persistence, idle inline images, and brief TTS synchronization sanity.
