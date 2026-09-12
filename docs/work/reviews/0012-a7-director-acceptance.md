# Goal 0012 A7 — director acceptance

Date: 2026-09-12

## Decision

**ACCEPTED FOR FOCUSED REAL-DESKTOP SIGNOFF.**

A7 closes the two director blockers from A6 without widening the correction or changing canonical playback ownership.

## Accepted evidence

- Worker branch synchronized director `main` at `1aac4585e695ec0cc10248e59daec91ca629ae1c` and preserved A6 implementation `6dcf9815ef87303caa3a3421bb8cc9e832f6b8ea`.
- A7 implementation `6fd324869ce6cca0c858c3ee32fabe627efba47b` changes blockquote decoration ownership from pre-layout child `ui.max_rect()` to the final measured `Frame::show` response rectangle.
- The blockquote regression lays out ordinary neighboring paragraphs around a long wrapping quote and proves the rule endpoints equal the measured quote frame while remaining between neighboring content.
- The stateful highlight regression invalidates geometry-A measurements, establishes variable geometry B, drives `AutoScrollState` request -> decision -> record/consume, and then performs a subsequent normal `pretty_render_window(..., None)` selection across 64 canonical boundaries. The active target remains naturally present after the one-shot forced-follow lifecycle has ended.
- Production canonical playback/highlight ownership was not changed and no permanent highlighted-target forcing was introduced.
- A6 presentation improvements remain preserved: literal horizontal margins, viewport-owned vertical insets, geometry invalidation, scrollable settings panel, real-egui word/letter spacing, readable table overflow, font availability/effective-fallback UX, media sizing, inline images, and off-render-thread heavy work.
- Windows baseline run `34723376577` passed `native-workspace` and `hosted-renderer-probe`, including workspace check/build/test, repository QA preparation, Windows TTS, watcher-policy, and hosted renderer capability gates.

## Integration

Terminal branch head `762452ada592b7b906b0537319fd4b3060ad1c34` was fast-forwarded into `main` after director review.

## Remaining gate

Goal 0012 is **not finally closed** until one focused physical Windows pass verifies the A6/A7 presentation behavior on the user's actual EPUBs. The next human pass should specifically verify:

- zero/low horizontal margin no longer leaves the hidden large gutter;
- horizontal and vertical margins behave literally;
- the settings/presentation panel scrolls to all controls;
- word and letter spacing visibly change text;
- TOC/table content remains readable instead of character-wrapping into narrow columns;
- blockquotes retain restrained bounded styling without a page-length rule;
- unavailable fonts visibly communicate fallback state;
- inline images still render while TTS is idle;
- the spoken pretty highlight remains continuously visible for the current sentence after follow/scroll occurs, instead of flashing once and disappearing.

The separate starter-shell overlap remains Goal 0013 and is not part of Goal 0012 signoff.
