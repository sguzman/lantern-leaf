# Goal 0012 A5 — Real-desktop rejection after startup success

## Decision

**REJECTED AS FINAL SIGNOFF; A6 REQUIRED.**

A5 fixed the startup font panic. LanternLeaf now launches on the same physical Windows machine/config, inline EPUB imagery is visibly rendering in pretty view, the Presentation section is present, and the existing TTS playback/scroll identity remains synchronized. Goal 0012 is nevertheless not complete because the first usable physical presentation pass exposed several concrete native-egui layout defects.

## Physical Windows observations

The user verified:

- startup now succeeds;
- cover/inline imagery is visible in pretty view;
- the expanded Presentation controls are visible;
- font size, base font scale, paragraph spacing, block spacing, heading-1 scale, and heading-2 scale visibly work;
- TTS remains spoken-sentence synchronized and continues to scroll/follow.

The same pass exposed:

1. a large built-in left gap in pretty view even before useful horizontal-margin control;
2. horizontal margin slider appears ineffective;
3. vertical margin changes document scroll extent rather than the viewing viewport, with bottom padding pushed off-screen;
4. word spacing and letter spacing did not produce a visible effect in physical QA;
5. the Presentation/settings sidebar is taller than the window and cannot be scrolled, making controls unreachable;
6. some selectable font families visibly fall back while the UI gives no availability/effective-font indication;
7. quoted passages render with a visually dominant vertical rule; long quotations can produce an objectionably long stripe;
8. a real EPUB table of contents collapses columns until headings/cells wrap at nearly character granularity;
9. after the presentation work, spoken highlight now appears briefly and then disappears even though TTS identity remains synchronized and follow/scroll continues;
10. the starter screen can visibly bleed adjacent multi-column groups into one another at the tested window geometry.

Media max width/height was not conclusively verified in this pass. Heading-3 behavior was not conclusively distinguishable from source content lacking an H3.

## Director code diagnosis

### Horizontal margin is conflated with a hidden 720 px text-column cap

`render_pretty_page()` currently computes:

```text
max_width = (available_width - horizontal_margin * 2).clamp(240, 720)
margin = (available_width - max_width) / 2
```

At sufficiently wide windows, both margin 0 and larger slider values can clamp to the same 720-px content column. This creates a large implicit centered gutter and makes the horizontal-margin control appear dead. Horizontal margin must own explicit viewport/content inset semantics; a hidden max-width policy must not masquerade as the margin control.

### Vertical margin is document padding, not viewport inset

The renderer currently calls `ui.add_space(vertical_margin)` inside the scroll content before and after the pretty document. This lengthens the scrollable book instead of shrinking/insetting the visible reading viewport. The physical observation exactly matches the implementation.

### Presentation geometry does not invalidate cached block heights

`pretty_block_heights` stores measured response heights and `pretty_block_heights_for()` reuses them independent of font size, family/weight metrics, base scale, line spacing, word/letter spacing, paragraph/block spacing, margins, and table/media geometry.

This is a likely explanation for the new highlight symptom: on a fresh TTS boundary, a pending follow request forces the highlighted target block into the bounded render window, so the highlight appears. After the follow request is consumed, viewport-based virtualization can use stale prefix heights and stop rendering the actual highlighted block while canonical playback identity and scroll state continue correctly. A6 must prove or disprove this with production-adjacent geometry/follow regressions; do not weaken canonical TTS ownership.

### Sidebar has no enclosing scroll container

The left `SidePanel` renders Settings, Stats, Search, TTS, status diagnostics, and anchor diagnostics sequentially. There is no vertical `ScrollArea` around the panel body. Once Presentation expanded, controls can simply extend beyond the window.

### Blockquote styling is semantically plausible but visually over-dominant

Quoted blocks are intentionally styled with a left rule and weak text. The rule is a fixed 3-px stroke using active-widget fill and runs for the quote block. Preserve quote semantics, but use a restrained bounded quote treatment so source documents containing long blockquotes do not dominate the page with a black stripe.

### Table renderer has no readable-width or overflow policy

Pretty tables currently use an egui `Grid` with wrapping labels but no minimum readable column width and no horizontal overflow fallback. Narrow/heterogeneous cells can therefore collapse to character-level wrapping. A6 needs a deterministic table-width policy that prefers readable columns and horizontal scrolling/overflow over destructive squeezing.

### Word/letter spacing path is wired but physical behavior failed

The core patch fields are persisted/applied, and the renderer feeds them into `TextFormat.extra_letter_spacing`. Do not assume the feature works merely because the values reach the render code. A6 must add a real egui layout regression proving changed values measurably alter glyph/word layout, then correct the implementation if that proof currently fails.

### Font fallback is safe but opaque

A4/A5 intentionally preserve configured font intent and fall back safely when an optional family is unavailable. That is correct. The Presentation UI should, however, expose availability/effective fallback rather than making a non-installed family look silently broken.

### Starter multi-column containment is a separate shell issue

The starter view uses `ui.columns(2)` and child groups/rows that can expand beyond their allocated column. The physical screenshot shows Recents and Browser Tabs group borders/content overlapping. This is outside the reader-presentation acceptance core and should be tracked separately rather than silently folded into TTS/presentation semantics.

## Preserve accepted behavior

A6 must preserve:

- Goal 0008/0009 TTS identity, playback, no-duplicate-line behavior, text-only behavior, and canonical highlight/follow ownership;
- A3 async worker completion wakeups;
- A4/A5 exact font-registry safety and missing-font fallbacks;
- app-default -> per-book presentation persistence/reset;
- EPUB image provenance and inline image rendering;
- bounded off-render-thread pretty/image work.

Do not ask the user to repeat this same QA before a code correction exists.
