# Goal 0012 A5 — Director acceptance for focused real-desktop QA

## Decision

**ACCEPTED FOR FOCUSED REAL-DESKTOP QA.**

Goal 0012 is not finally closed until the physical Windows pass succeeds.

## Accepted implementation

A4 production fix `c92256fb429aea1d13707a811b0147e4337bbd05` replaces the unsafe coarse `fonts_configured` assumption with exact registered-alias tracking. Global egui styles and pretty/per-book font selection use named LanternLeaf aliases only when those aliases are actually registered; missing optional regular/bold/monospace families fall back deterministically to a registered safe alias or egui built-in proportional/monospace families without rewriting configured app/book font intent.

A5 implementation `46d70b34d2a560373e831471f24b58d17b1fe8bc` adds the missing real-layout proof. Controlled registries are installed into an egui `Context`; Body, Heading, and Monospace styles are laid out through epaint, and a representative pretty `LayoutJob` containing regular, bold, and bold-code spans is forced through the production `presentation_font_families()` path. Coverage includes missing Lexend with unrelated aliases, missing bold/monospace, unavailable per-book family fallback, available registered aliases, named-family binding invariants, and preservation of configured family/weight intent.

## Validation

Worker report records successful local workspace tests/checks, `git diff --check`, repo-native Windows QA preparation, Windows synthesis/TTS probes, and renderer smoke evidence.

Authoritative Windows workflow `34718069167` on A5 implementation `46d70b34d2a560373e831471f24b58d17b1fe8bc` passed both `native-workspace` and `hosted-renderer-probe`, including workspace check/build/test, watcher-policy, staged QA/synthesis, Windows TTS probe, and renderer capability probe.

Worker terminal commit: `e2938b2cf543df237beb79f83e5159e4e598b4cd`.

## Human verification

Run the normal repo-native Windows QA path on current `main`.

The first gate is startup on the same machine/config that previously crashed. If startup succeeds, verify:

1. Open a real EPUB and confirm the Presentation section is present.
2. Change font family/weight/size and a few spacing/margin controls; pretty rendering should update without crash.
3. Select at least one font family that is unavailable on the machine if possible; LanternLeaf must remain stable and fall back safely.
4. Close/reopen the book and confirm explicit presentation choices persist; `Use app presentation defaults` restores presentation defaults without disturbing TTS settings.
5. With TTS stopped, open/scroll through an image-bearing EPUB and confirm inline images appear without unrelated input being required to wake the UI.
6. Run a short TTS sanity check in pretty view and confirm spoken-sentence highlight/follow remains synchronized.

If startup or any required presentation/image behavior fails, do not repeat the same QA before a code correction. Reopen Goal 0012 with the exact failing observation.
