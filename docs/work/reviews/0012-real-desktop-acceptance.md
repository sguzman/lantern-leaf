# Goal 0012 — real-desktop acceptance

Date: 2026-09-12

## Decision

**FINALLY ACCEPTED / CLOSED.**

The focused physical Windows signoff after A7 passed the required Goal 0012 reader-presentation acceptance surface.

## Human QA evidence

The user reported that the presentation controls now work and specifically confirmed the previously failing visual behavior is repaired. The focused pass is accepted for:

- literal horizontal margin behavior with the prior giant baked-in gutter gone;
- vertical margin behaving as a visible reading-area inset rather than scroll-document padding;
- a scrollable settings panel with the full Presentation/TTS surface reachable;
- visibly working word spacing and letter spacing;
- readable TOC/table presentation instead of pathological character-by-character compression;
- restrained blockquote styling without the prior giant vertical rule;
- visible font fallback/unavailable-font behavior;
- inline images remaining present;
- continuously visible spoken-sentence highlighting/follow during TTS rather than flash-and-disappear behavior.

The user described the resulting visual work as a substantial improvement and was satisfied with the reader presentation.

## Non-blocking residual observation

The user could not physically verify the **media max width / max height** controls. Changing those controls caused a violent/unexpected scroll jump and the visible image did not appear to change size during that attempt.

This does **not** reopen Goal 0012 because:

- A6 already contains automated aspect-ratio/max-size sizing evidence;
- the focused human acceptance gate defined in the A7 director review required inline images to remain functional, not a second proof of the sizing algorithm;
- the newly observed problem is specifically interactive viewport anchoring / visible control effect while media geometry changes.

Preserve this as a separate follow-up rather than obscuring the successful Goal 0012 signoff.

## Separate starter-shell issue

The starter/library view still shows some Recents / Calibre / Browser Tabs panel bleed at the tested width, though substantially less severe than before. This remains Goal 0013 and is not a Goal 0012 regression.

## Result

Goal 0012 is closed with automated, director, CI, and real-desktop acceptance. The media-control scroll-anchor observation is queued separately for later work; starter-shell containment proceeds independently under Goal 0013.
