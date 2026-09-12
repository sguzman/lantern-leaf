# 0012 — A4 correction: safe font registry and missing-font fallback

## Outcome

Preserve the successful Goal 0012 presentation-controls, inline-image, and async-wakeup implementation while fixing the real-desktop startup crash caused by unsafe named-font alias assumptions. Missing optional fonts must degrade to deterministic safe fallbacks; they must never panic LanternLeaf startup or pretty rendering.

## Starting evidence

A3 implementation `dccc5b999aa7732d1f71a248d8595cf5bde4a40d`, terminal `ae411ffaadebebb23f137b40728dfb4dbc944048`, and Windows CI `34655821185` were director-accepted for desktop QA.

The first real Windows QA compiled successfully, launched LanternLeaf, logged `Configured egui font families via fontdb requested_family=Lexend mono_family=Fira Code`, then immediately panicked in epaint because `FontFamily::Name("LanternLeafProportionalRegular")` was not bound to any fonts. The process exited with code 101 before functional reader QA.

Director diagnosis is recorded in `docs/work/reviews/0012-a3-real-desktop-rejection.md`.

## Preserve prior accepted work

Do not redesign Goal 0012. Preserve:

- the separate Presentation settings surface;
- live font family/weight/size, spacing, margins, highlight, heading/base/paragraph/block/media controls;
- app-default -> per-book presentation persistence and presentation-only reset;
- source-born canonical sentence/highlight/follow identity;
- safe EPUB image provenance/reference resolution and generated EPUB fixture coverage;
- bounded off-render-thread pretty construction and image decode;
- bounded image request/cache behavior and visible placeholders;
- A3 worker-completion repaint wakeups for pretty-build and image success/failure;
- all accepted Goal 0008/0009 TTS behavior.

Natural/Narrator/HD Windows voices remain deferred and out of scope. Caliberate catalog covers and PDF work remain out of scope.

## Blocking defect

`setup_egui_fonts()` currently treats `inserted_any` as proof that all named aliases later used by global styles exist. That is false: unrelated family aliases can set `inserted_any = true` while the configured proportional or monospace alias is absent. The function then installs global `TextStyle` entries that reference unbound named families and egui panics on layout.

Pretty rendering has the same class of defect: `presentation_font_families()` manufactures named aliases based on a single `fonts_configured` boolean rather than exact alias availability.

## Authorized A4 correction

### A — production-owned font registry / availability contract

Replace the coarse global boolean assumption with a small production-owned font registry/availability model created during startup font preparation.

The model must be able to answer, at minimum:

- whether a named LanternLeaf alias is bound;
- which proportional regular/bold aliases are safe for global chrome;
- which monospace alias is safe;
- which family/weight alias is safe for a requested presentation family;
- what deterministic fallback should be used when a requested alias is unavailable.

The exact type is flexible. Do not add font discovery/file I/O to ordinary egui frames.

### B — guaranteed-safe global styles

When installing global egui `TextStyle` values:

- use a LanternLeaf named alias only if that exact alias is bound;
- otherwise use a deterministic bound fallback or egui built-in `FontFamily::Proportional` / `Monospace`;
- no global style may ever reference an unbound named family.

A missing configured family must not prevent LanternLeaf from starting.

### C — guaranteed-safe pretty font selection

`presentation_font_families()` or its replacement must never synthesize an alias merely from enum/slug + a global configured flag.

For every requested family/weight:

- return the exact registered alias when present;
- otherwise return a safe bound fallback/built-in family;
- preserve configured intent in app/book settings rather than rewriting configuration because a font is absent on this machine;
- avoid per-frame warning spam. If diagnostics are emitted, do so at startup or on explicit settings change/first fallback observation.

Regular/bold/monospace/code paths must each remain safe when one or more requested faces are missing.

### D — deterministic controlled-availability tests

Do not depend on CI machine font inventory. Extract a narrow resolver/preparation seam that can be fed controlled/synthetic availability.

Required regressions:

1. requested `Lexend` absent, another font available -> font setup/fallback is valid and forcing an egui text layout does not panic;
2. partial registration where unrelated aliases exist -> the old `inserted_any` false-positive cannot produce an unbound global alias;
3. proportional regular available but bold unavailable -> both body and heading/bold rendering resolve safely;
4. requested monospace unavailable -> code/monospace rendering resolves safely;
5. per-book selection of an unavailable supported family resolves to a safe fallback and layout does not panic;
6. available family/weight still resolves to its real registered LanternLeaf alias;
7. every named family returned by the production font-selection helper is present in the prepared registry/definitions;
8. presentation config values remain unchanged by fallback.

Where practical, create an egui `Context`, install the prepared `FontDefinitions`/styles, and force text/layout access so the regression catches the exact epaint panic class rather than testing strings only.

### E — preserve startup/render performance discipline

Startup font discovery may remain startup-owned, but no new filesystem/fontdb work may occur in ordinary render frames or hot pretty-render loops. Rendering consumes prepared registry state only.

## Acceptance gates

1. The real crash class `FontFamily::Name(...) is not bound to any fonts` is impossible for LanternLeaf-generated aliases under controlled missing/partial font availability.
2. LanternLeaf can start with configured `Lexend` absent without panic.
3. Missing bold/monospace/per-book family variants degrade safely.
4. Existing available font family/weight switching still changes pretty rendering.
5. No fallback silently rewrites app/book presentation configuration.
6. Existing Goal 0012 presentation persistence/reset, inline images, image async wakeups, bounded rendering, and off-render-thread discipline remain green.
7. Existing Goal 0008/0009 TTS/highlight/follow regressions remain green.
8. `cargo test --workspace -- --test-threads=1`, `cargo check --workspace`, `git diff --check`, repo-native Windows QA preparation, Windows TTS probe, hosted renderer probe, and Windows CI pass.
9. No human QA until director accepts A4.

## Repository handoff

Continue the existing branch `codex/0012-pretty-presentation-controls-and-inline-images` and report `docs/work/reports/0012.md`.

This is Attempt A4 of the same Goal 0012. Synchronize current `main` before implementation, move this goal `ready -> active`, re-arm the Goal 0012 watcher, preserve A1–A3, implement only this bounded font-safety correction, append Attempt A4 to the report, validate, terminalize `done` or `blocked`, push, signal terminal state, and restore the shared checkout to `main` without merging.

## Human verification after director acceptance

One focused Windows pass will first confirm LanternLeaf starts on the same machine/config that crashed. Only after startup succeeds should the human verify presentation controls/persistence, idle inline images, and a brief TTS synchronization sanity check.
