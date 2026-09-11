# 0012 — Pretty reader presentation controls and inline images

## Outcome

Restore first-class native-egui presentation controls for the pretty reader and make embedded EPUB/HTML images render inline at the correct document location, without regressing the now-accepted Goal 0008/0009 TTS, sentence identity, highlight, follow, and large-document performance architecture.

## Starting evidence

Goal 0009 has final real-desktop acceptance: audible Windows TTS, no unsolicited duplicate ordinary line reads, pretty and text-only spoken-sentence highlighting, and viewport follow are working correctly.

After that signoff, the human identified two presentation regressions outside Goal 0009:

1. the current native reader exposes TTS controls but no longer exposes the detailed visual/presentation controls available in older LanternLeaf builds;
2. embedded images are not rendering in real EPUB pretty view.

The current configuration model still contains reader presentation intent, including:

- `font_family`;
- `font_weight`;
- `font_size`;
- `line_spacing`;
- horizontal/vertical margins;
- `word_spacing`;
- `letter_spacing`;
- day/night highlight colors;
- `PrettyUiConfig` controls such as base font scale, heading scales, paragraph/block spacing, and media sizing;
- corresponding optional per-book fields in `BookReaderOverrides`.

The current pretty model also already contains `PrettyBlockKind::Image`, `PrettyImage`, image-size clamping, and image-source resolution. The real-book failure therefore requires tracing the complete source-reference -> extracted asset -> pretty block -> decode/cache -> texture/render path rather than merely adding a new `Image` enum variant.

## Architectural rules

- Native Rust + `eframe`/`egui` remains the production UI.
- **Never do blocking disk I/O, image decoding, or heavy layout/preparation work on the egui/render thread.** The render thread may request work and consume ready results only.
- Preserve A5/A5.1 bounded rendering and lightweight frame state.
- Preserve source-born canonical sentence identity and A8/A8.3 structured-document provenance.
- Presentation changes must not rebuild/restart TTS or mutate canonical sentence identity.
- Use the existing app-config -> per-book override layering from Goal 0009. Do not invent a second presentation-settings store.
- Windows Natural/Narrator/HD voice work is explicitly deferred by the user and is outside this goal.

## Authorized pass A — recover presentation-setting semantics

Audit the current config model, native pretty renderer, current reader settings UI, and relevant historical LanternLeaf implementation behavior.

Create a concrete matrix:

`setting -> app default -> book override -> current renderer consumer -> native UI control -> persistence regression`

At minimum cover:

- font family;
- font weight;
- font size;
- line spacing;
- horizontal margin;
- vertical margin;
- word spacing;
- letter spacing;
- highlight color(s) where currently supported;
- useful `PrettyUiConfig` controls that materially affect rendering, including base font scale, paragraph/block spacing, heading scales, and media sizing.

Do not expose controls that are no-ops. If a field exists but the native renderer does not currently honor it, either wire it through correctly or leave it hidden and document why.

Do not resurrect an old Iced/Tauri/WebView UI. Recover the semantics in native egui.

## Authorized pass B — native Presentation settings UI

Add a clearly discoverable presentation section/panel in the reader distinct from TTS/backend controls.

Suggested organization:

- Typography;
- Layout;
- Pretty formatting/media;
- Highlight/reading-follow appearance where appropriate.

Requirements:

- controls update the native pretty view live;
- changing presentation settings must not restart TTS, change the spoken canonical sentence, or lose highlight/follow state;
- controls use existing `ReaderSettingsPatch`/equivalent command paths and `BookReaderOverrides` persistence;
- untouched books inherit app defaults;
- book-level changes survive close/reopen;
- where practical, provide a clear "Use app defaults" / reset-override action rather than forcing the user to manually rediscover defaults;
- settings panel itself must remain bounded/resizable and must not make the content viewport unusable.

Font family and font weight must affect actual text rendering; do not merely persist values while continuing to use hard-coded proportional regular/bold fonts.

## Authorized pass C — presentation rendering correctness

Make every exposed setting visibly meaningful in native pretty rendering.

Add deterministic policy/render regressions for at least:

- family selection;
- normal/bold weight handling;
- base font size/scale;
- line spacing;
- margins;
- word spacing;
- letter spacing;
- paragraph/block spacing;
- heading scaling.

Preserve rich inline styles such as emphasis, strong, code, links, superscript/subscript, lists, quotes, and tables.

Changing visual settings must preserve the same canonical `PrettySentenceTarget`/highlight ranges and must not alter TTS text or audio planning.

## Authorized pass D — trace and repair embedded image provenance

Trace image identity from actual EPUB/native HTML ingestion to the renderer.

For every source image, preserve enough information to resolve it deterministically:

- original chapter/spine location;
- raw `src` reference;
- normalized EPUB-internal path;
- extracted/local asset path;
- optional alt text;
- source block identity/order.

Handle realistic relative references such as:

- `../Images/foo.jpg`;
- nested chapter/image directories;
- spaces and percent/URL encoding where applicable;
- fragments/query components where meaningful or safely discardable;
- case/path normalization permitted by the EPUB container contract.

Prevent traversal outside the extraction/cache root.

Do not resolve EPUB image references by relying on the process current working directory or bare `PathBuf::exists()` guesses.

If an image cannot be resolved, emit a concise diagnostic and a visible bounded placeholder/alt-text representation instead of silently deleting the block.

## Authorized pass E — lazy off-render-thread image pipeline

The current real-book image failure must be repaired without creating a new egui-stall regression.

Requirements:

- image file read/decode happens off the render thread;
- an image block becoming visible/near-visible may enqueue lightweight work;
- bound worker count / in-flight decode work;
- cache successful decoded image data and bounded texture state;
- cache negative results without making transient failures permanent;
- request repaint when a decode completes;
- UI/render thread performs only lightweight ready-result consumption / texture upload needed by egui;
- do not decode all images at book open;
- do not decode all images in a large book because one image becomes visible;
- bound memory with a sensible LRU/eviction or equivalent policy;
- preserve aspect ratio;
- honor configured pretty max width/height/media sizing;
- unsupported/corrupt formats produce a placeholder and diagnostic rather than crashing the reader.

The hard software-philosophy rule applies: **no heavy image work on the GUI/render thread.**

## Authorized pass F — real EPUB image fixture

Add a project-owned valid EPUB fixture that exercises the actual production ingestion and pretty pipeline.

Include at least:

- multiple spine chapters;
- PNG and JPEG assets;
- nested asset directories;
- a filename containing a space or encoded character;
- a relative `../Images/...` reference;
- text before and after an image;
- at least one inline image inside a text-bearing source block if the source model supports it;
- alt text;
- ordinary rich text around the image.

Test the production chain:

`EPUB bytes -> extraction/source provenance -> StructuredDocument -> ReaderImageRef/pretty projection -> image work queue -> decode result -> render sizing policy`

Assert:

1. image blocks occur in the correct source order;
2. normalized references resolve to the intended extracted local asset;
3. no path escapes the extraction root;
4. PNG/JPEG dimensions decode correctly;
5. aspect ratio survives max-width/max-height clamping;
6. text/canonical sentence IDs immediately before/after the image are unchanged;
7. inline-image splitting does not corrupt source-block/canonical highlight provenance;
8. missing image produces intentional placeholder state;
9. image decode work is not executed by the egui frame/render function.

## Authorized pass G — per-book presentation persistence

Use the Goal 0009 layered configuration model.

Regression scenario:

1. App config has presentation defaults.
2. New Book A inherits them.
3. Change Book A font/spacing/layout/pretty settings.
4. Close/reopen Book A and verify overrides return.
5. Open untouched Book B and verify it still inherits current app defaults.
6. Reset Book A presentation overrides and verify it returns to app defaults.
7. Voice/backend overrides and TTS behavior remain unaffected by presentation changes.

Do not freeze a whole historical `AppConfig` into a book file.

## Performance / regression gates

Preserve all accepted behavior from Goals 0008/0009:

- fast/snappy large EPUB open;
- bounded pretty rendering;
- sentence-accurate pretty highlight/follow;
- working text-only highlight/follow;
- first-sample-driven TTS identity;
- no duplicate ordinary line playback;
- Windows voice inheritance/per-book override behavior;
- Close book / Safe Quit lifecycle;
- transactional Piper failure recovery;
- one canonical ReaderSession / lightweight hot paths.

Add a large pretty-document regression proving presentation controls and lazy images do not restore whole-book layout/decode work.

## Acceptance gates

1. Native reader exposes a useful discoverable Presentation settings surface separate from TTS controls.
2. Font family, font weight, font size, line spacing, margins, word spacing, and letter spacing demonstrably affect native pretty rendering or are explicitly withheld with evidence that the renderer cannot yet support them; no exposed no-op controls are accepted.
3. Useful `PrettyUiConfig` controls such as paragraph/block/heading/media sizing are live and persist through existing app/book config layering.
4. Per-book presentation changes survive reopen; untouched books inherit app defaults; reset-to-app-default behavior is covered.
5. Embedded EPUB images render inline at the correct document position in the real native pretty pipeline.
6. Relative/nested/encoded EPUB image references resolve safely and deterministically to extracted assets.
7. Image disk read/decode work is lazy, bounded, and off the egui/render thread.
8. Image failures render intentional placeholder/alt state and useful diagnostics rather than disappearing or crashing.
9. Image sizing preserves aspect ratio and honors configured media limits.
10. Real EPUB fixture proves image provenance/order plus canonical sentence/highlight identity before and after media.
11. Goal 0008/0009 TTS, pretty/text-only synchronization, voice configuration, and large-document responsiveness remain green.
12. `cargo check --workspace`, repository-policy tests, `git diff --check`, repo-native Windows QA preparation, Windows TTS probe, and hosted renderer probe pass.
13. No human QA during implementation. Human verification occurs only after director acceptance.

## Explicit non-goals

Do not expand Goal 0012 into:

- Caliberate catalog-cover work from Goal 0010;
- Windows Natural/Narrator/HD voices from deferred Goal 0011;
- PDF visual/TTS work;
- Piper model downloading/catalog management;
- a broad library UI redesign;
- WebView/Tauri fallback.

## Repository handoff

Create/continue branch `codex/0012-pretty-presentation-controls-and-inline-images` and report `docs/work/reports/0012.md`.

This is a new macro-goal. Synchronize current director state from `main`, move this goal `ready -> active`, arm the Goal 0012 watcher, implement every authorized pass, validate every acceptance gate, terminalize `done` or `blocked`, push, signal terminal state, and restore the shared checkout to `main` without merging.

## Human verification after director acceptance

Use the same real EPUB(s) that already prove TTS/highlight sync. Verify presentation controls live, persistence across reopen, and actual inline images. TTS only needs a regression sanity check; do not reopen its architecture unless evidence shows a regression.
