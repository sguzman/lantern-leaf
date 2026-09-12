# Goal 0012 A3 real-desktop rejection

Status: **REJECTED AT REAL-DESKTOP STARTUP; A4 CORRECTION REQUIRED BEFORE FUNCTIONAL QA**

A3 implementation: `dccc5b999aa7732d1f71a248d8595cf5bde4a40d`
Worker terminal: `ae411ffaadebebb23f137b40728dfb4dbc944048`
A3 Windows CI: `34655821185` — green

## Real-desktop evidence

The first focused Windows QA after director acceptance did not reach the reader UI. LanternLeaf compiled and launched, logged:

`Configured egui font families via fontdb requested_family=Lexend mono_family=Fira Code`

and then immediately panicked in egui/epaint:

`FontFamily::Name("LanternLeafProportionalRegular") is not bound to any fonts`

The process exited with code 101. This is a Goal 0012 startup regression introduced by the presentation-font registration path, not a user setup requirement.

## Root cause in the accepted A3 code

`setup_egui_fonts()` uses one global `inserted_any` flag. The supported-family preload loop can successfully register one or more unrelated/fallback family aliases and set `inserted_any = true` even when the configured proportional family cannot be resolved/read.

After `inserted_any` becomes true, the function unconditionally installs global egui `TextStyle` entries using:

- `LanternLeafProportionalRegular`;
- `LanternLeafProportionalBold`;
- `LanternLeafMonospaceRegular`.

Those aliases are only inserted when their specific source face resolves and `face_bytes()` succeeds. Therefore `inserted_any == true` does not imply that the aliases subsequently referenced by global styles exist. egui correctly panics when layout asks for an unbound named family.

There is a second instance of the same unsafe assumption in pretty rendering: `presentation_font_families()` manufactures `LanternLeafFamily{slug}{weight}` names whenever the single `fonts_configured` boolean is true. It does not prove that the selected family/weight alias is actually present. A missing optional font can therefore also become a later book-render panic even if startup survives.

## Required A4 correction

Preserve all substantive Goal 0012 A1–A3 work. Fix font registration/fallback as one bounded correction.

1. Replace the coarse `fonts_configured: bool` safety assumption with production-owned font availability/registry semantics sufficient to answer whether each named alias is actually bound.
2. Never install a global egui `TextStyle` using a named LanternLeaf alias unless that alias is guaranteed bound to at least one font-data entry.
3. Never return a named presentation family/weight alias unless that exact alias is bound.
4. Missing configured/app/book fonts must fall back deterministically to a safe available family or egui built-in family. Missing optional fonts must never panic startup or book rendering.
5. Preserve the user's configured family intent. A temporary fallback should not silently rewrite app/book configuration merely because a font is unavailable on one machine.
6. Emit concise diagnostics when a requested family/weight falls back, without spamming every frame.
7. Preserve live presentation switching and per-book persistence/reset. Switching to an unavailable family must remain responsive and safe.
8. Do not add runtime filesystem/font discovery work to ordinary egui frames. Font discovery/registration remains startup-owned; rendering consumes prepared registry state only.

## Deterministic regressions required

Do not rely on whatever fonts happen to be installed on CI. Add a narrow production test seam around font resolution/registration so tests can control availability.

At minimum prove:

1. configured `Lexend` absent while some other system/synthetic font is available -> setup succeeds safely or falls back, no unbound global style alias, and forcing text layout does not panic;
2. configured family absent with partial font registration -> the old `inserted_any` false-positive cannot occur;
3. requested proportional regular present but bold absent -> normal and heading/bold paths both resolve safely;
4. requested monospace face absent -> code/monospace rendering falls back safely;
5. a per-book switch to an unavailable supported family returns only a bound/fallback family and does not panic layout;
6. an available supported family/weight still selects its real registered alias;
7. every LanternLeaf named family returned by production font-selection helpers exists in the prepared `FontDefinitions`/registry.

## Validation gates

- all existing Goal 0012 presentation/image/async-wakeup tests remain green;
- all Goal 0008/0009 TTS/canonical sync tests remain green;
- `cargo test --workspace -- --test-threads=1` passes;
- `cargo check --workspace` passes;
- `git diff --check` passes;
- repo-native Windows QA preparation passes;
- Windows TTS probe and hosted renderer probe pass;
- Windows baseline CI passes.

No human QA until A4 is director-reviewed and accepted.
