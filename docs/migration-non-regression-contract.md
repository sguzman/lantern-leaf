# Migration Non-Regression Contract

These behaviors are hard requirements for migration parity.

## Highlight And Scroll

- Sentence highlight must track the active playback sentence.
- Sentence click must move highlight and playback anchor to the clicked sentence.
- Auto-scroll must keep highlighted sentence visible.
- Auto-center must center highlighted sentence when enabled.
- Font size, margins, and line spacing changes must not break highlight alignment.

## Layout Stability

- Top controls and TTS control rows must not reflow into vertical/compressed text.
- When width is insufficient, optional controls hide; they do not collapse into vertical rendering.

## Playback Semantics

- Paused playback remains paused after page changes, sentence seeks, and sentence clicks.
- Close session or return to starter cancels in-flight work and leaves no active playback.

## Source Open And Job Lifetimes

- Only one source-open operation may be active at once.
- Duplicate/conflicting opens are rejected or cancelled deterministically.
- PDF transcription events and source-open events must stay request-id correlated.

## Persistence

- Bookmark/config saving must run on close-session and safe-quit paths.
- Recents and source cache compatibility must be preserved across migration builds.

## Precision And Formatting

- TTS progress remains 3-decimal precision in reader UI.
- Pause-after-sentence supports 2-decimal resolution.
