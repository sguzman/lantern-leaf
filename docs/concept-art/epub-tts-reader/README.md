# EPUB + TTS Reader Concept

Status: **visual/product reference, not an implementation contract**

This folder preserves a LanternLeaf UI concept that should not be allowed to disappear into chat history. The current GUI is free to differ mechanically while the implementation is still being recovered, but this concept captures a strong target for what the product should *feel like* when EPUB reading, text-to-speech, navigation, and customization are treated as one coherent activity.

## Artifacts

- [`leafline-epub-tts-reader-concept.webp`](leafline-epub-tts-reader-concept.webp) — a visually faithful repository preview of the concept art, downsampled from the attached 1448×1086 source to 400×300 for durable archival through the repository connector.
- [`furnishing.toml`](furnishing.toml) — a semantic TOML-like furnishing of the pictured UI. It describes regions, controls, document semantics, presentation, and live TTS state; it is intentionally more like an editable intermediate representation than a pixel-perfect GUI schema.

The source image was generated during a ChatGPT UI-authoring experiment and reattached by the user on 2026-09-16 so the concept could be archived with the project. The repository image is a preview copy; the TOML and this document preserve the higher-level design intent independently of raster resolution.

## Product intention

The central idea is the same one already expressed in LanternLeaf's project philosophy: **reading and listening are one activity**. TTS is not a utility panel bolted onto an ebook reader. The spoken sentence exists inside the document, the visual highlight follows it, navigation and bookmarks share the same reading identity, and the player remains present without displacing the book.

The concept should therefore be read as a product north star for four things working together:

1. **Document context** — library, current book, chapter list, progress, highlights, notes, and bookmarks.
2. **The reading surface** — typography-first EPUB presentation with a visible current spoken range.
3. **Reading control** — a dense but orderly inspector for typography, spacing, width, colors, modes, annotations, and TTS behavior.
4. **Continuous speech transport** — persistent play/pause, skip, rate, voice, volume, waveform/progress, and current narration context.

## Spatial concept

The desktop shell is intentionally stable and legible:

- **Left rail (~260 px):** application navigation, current-book identity, progress, and table of contents.
- **Center (`1fr`):** the book. This is the dominant surface and should remain visually calmer than the surrounding controls.
- **Right inspector (~330 px):** heavy reader customization. Controls are dense, grouped, and immediately understandable rather than hidden behind many modal dialogs.
- **Bottom TTS surface (~96 px):** persistent playback transport spanning the reading session.

The composition is asymmetric on purpose. The book occupies the calm center while operational complexity is pushed to the edges.

## Interaction ideas worth preserving

### TTS is projected into the document

The current sentence is softly filled with the accent color rather than represented only by an external progress bar. Playback should be able to auto-follow the sentence and preserve semantic identity across scrolling and relayout.

The highlight is a synchronization signal, not a selection metaphor. It should remain visually gentle enough that long listening sessions do not turn the page into a flashing control surface.

### The player is persistent

Speech transport belongs at the bottom of the reading window, not in a temporary popup. The user should be able to change speed, voice, position, and volume without losing the current paragraph or opening a separate mode.

### Customization is a first-class product surface

The right inspector deliberately exposes more controls than a minimalist reader normally would. LanternLeaf should make the reading surface *owned by the user*:

- font family and size;
- line height and paragraph spacing;
- page margins and column width;
- text alignment;
- dyslexia-friendly type option;
- theme presets;
- brightness and contrast;
- scroll vs paged reading;
- highlight palette;
- annotation visibility;
- TTS voice, rate, pronunciation, and follow behavior.

The intent is not gratuitous settings density. The point is to make long-form reading physically and cognitively adjustable without requiring CSS, browser extensions, or another application.

### Navigation remains book-shaped

The left rail retains a cover, author, chapter count, progress, and table of contents. LanternLeaf can support many source formats internally, but the reading experience should still make the source feel like a coherent work rather than a generic text buffer.

## Visual language

The concept uses a dark, restrained, premium reading palette. It should feel calmer than a media dashboard and warmer than a developer tool.

### Core palette

| Role | Reference | Intent |
| --- | --- | --- |
| Application background | `#09100F` | near-black green; removes glare without becoming flat black |
| Reading surface | `#101716` | slightly raised dark teal/charcoal |
| Raised surface | `#141D1B` | side panels and controls |
| Hover/active dark surface | `#1B2824` | subtle depth without bright chrome |
| Primary text | `#F4F4EF` | warm off-white UI text |
| Reading text | `#F2ECE1` | warmer cream for long-form serif reading |
| Muted text | `#909A94` | restrained metadata |
| Accent | `#62E89C` | jade/mint; active controls and playback state |
| Accent soft | `#254F3D` | selection/highlight support |
| TTS highlight | `#315E4A` | soft spoken-sentence fill |
| Border | `#26322E` | quiet panel separation |

The jade accent should be used sparingly. The page remains mostly dark neutral + warm text; green communicates *active state* rather than becoming decorative wallpaper.

## Typography

The concept separates application typography from reading typography:

- **UI:** a clean sans serif in the Inter family/style.
- **Book:** a literary serif in the Literata family/style.
- **Chapter titles:** the reading serif, larger and slightly heavier.
- **Metadata and section labels:** compact sans-serif with increased tracking.

This split is important. The controls should feel like software; the center should feel like a book.

## Theme presets shown in the concept

The inspector includes four illustrative starting points:

- **Default** — dark neutral, warm cream text, jade accent.
- **Sepia** — warm paper-like surface.
- **Night** — lower-luminance near-black reading.
- **Ocean** — cooler blue-toned dark surface.

These are conceptual presets, not a required final set.

## TOML furnishing model

`furnishing.toml` intentionally distinguishes four ontological layers:

- **application shell** — sidebar, toolbar, inspector, and player;
- **document semantics** — chapter, heading, paragraph, quote;
- **presentation projection** — typography, margins, width, colors, flow mode;
- **live interaction state** — current TTS block/sentence, progress, voice, and active highlighting.

That separation matters for future UI-authoring experiments. A concept image could be translated into semantic TOML, edited structurally, and rendered back into concept art without requiring the TOML to become a literal CSS clone or an egui widget dump.

The furnishing is therefore a **design representation**, not a parser commitment. If LanternLeaf later adopts declarative UI configuration, its production schema should be designed deliberately rather than accidentally inheriting this experiment.

## What should survive implementation changes

The exact card radii, pixel measurements, icons, and fictional content are disposable. The durable ideas are:

- TTS and visual reading share one session model;
- the spoken sentence is visible in the document;
- playback remains immediately available;
- the book stays visually dominant;
- customization is powerful and inspectable;
- navigation preserves the identity and structure of the work;
- visual polish should communicate calm control rather than browser/tooling complexity.

If a future GUI looks materially different but preserves those properties, it can still be faithful to this concept.
