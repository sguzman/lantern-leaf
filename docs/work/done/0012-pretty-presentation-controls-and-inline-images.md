# 0012 — A4 correction: deterministic missing-font fallback

## Outcome

Preserve the accepted A1–A3 presentation, inline-image, and async-wakeup implementation while ensuring optional font availability cannot produce an unbound egui `FontFamily::Name`.

## Authorized scope

- Synchronize current director `main` into the existing Goal 0012 worker branch before implementation.
- Preserve all accepted presentation controls, persistence/reset, image provenance/rendering, async worker repaint wakeups, and Goal 0008/0009 regressions.
- Track the font aliases actually registered in `FontDefinitions`.
- Resolve configured presentation families through that registry and use deterministic built-in proportional/monospace fallbacks when an optional family or weight is unavailable.
- Never rewrite persisted app/book font intent as a side effect of fallback.
- Add deterministic controlled-font-availability regressions covering missing and partially registered families.
- Run repository and Windows gates, including workspace tests/check, repo-native QA preparation, Windows TTS probe, and hosted renderer probe.

## Non-goals

Do not redesign presentation controls, add font-download/catalog behavior, alter TTS, modify PDF behavior, or request human QA during implementation.

## Handoff

Continue `codex/0012-pretty-presentation-controls-and-inline-images` and append Attempt A4 to `docs/work/reports/0012.md`. Terminalize only after the A4 acceptance gates pass, then push, signal, and restore the shared checkout to `main`.
