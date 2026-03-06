# TTS Player Widget Roadmap

## Goal

Replace the current right-side TTS controls placement with a dedicated audio-player-style control bar anchored at the bottom of the text scroll pane. The widget should only appear when TTS controls are enabled, and should behave consistently in both text-only and pretty-text reading modes.

## Product Requirements

- [x] Move the TTS control widget out of the right-side stats/settings column.
- [x] Render the TTS control widget at the bottom of the active text scroll pane.
- [x] Show the widget only when TTS controls are enabled.
- [x] Hide the widget entirely when TTS controls are disabled.
- [x] Present controls in this exact left-to-right order:
  - [x] Previous page
  - [x] Previous sentence
  - [x] Play/Pause
  - [x] Next sentence
  - [x] Next page
- [x] Make the Play/Pause button visually larger and more prominent than the other controls.
- [x] Keep the widget usable in both text-only and pretty-text modes.

## UX and Layout

- [x] Anchor the player widget to the bottom of the reading pane rather than the global app frame.
- [x] Ensure the widget does not overlap or obscure the active reading content.
- [x] Add bottom padding or layout reservation so the final lines of text remain readable above the widget.
- [x] Keep the widget stable while playback state changes so it does not shift surrounding layout.
- [x] Preserve responsive behavior for narrow widths and mobile-sized windows.
- [x] Maintain clear visual separation from the text content while still feeling integrated with the reader.
- [x] Use a player-like visual hierarchy so the central Play/Pause action is dominant.

## Interaction Model

- [x] Wire `Previous page` to the existing page-back behavior for paged content.
- [x] Wire `Next page` to the existing page-forward behavior for paged content.
- [x] Wire `Previous sentence` to the existing sentence-back behavior.
- [x] Wire `Next sentence` to the existing sentence-forward behavior.
- [x] Wire `Play/Pause` to the existing playback toggle behavior.
- [x] Keep button disabled states accurate when an action is unavailable.
- [x] Preserve keyboard and focus behavior for all controls.
- [x] Ensure the widget updates immediately when playback state changes from external actions or hotkeys.

## Reader Integration

- [x] Render the widget inside the reader shell so it follows the currently active reading pane.
- [x] Ensure the widget works correctly whether the right-side panel is showing stats or settings.
- [x] Ensure swapping stats/settings does not affect widget position, state, or scroll.
- [x] Ensure changing between text-only and pretty-text keeps the widget present and functional.
- [x] Ensure widget visibility is driven by the same TTS-controls-enabled source of truth used elsewhere in the app.

## State and Performance

- [x] Avoid introducing extra rerender churn into the reading pane during playback.
- [x] Keep the control bar component isolated from high-frequency reader updates where possible.
- [x] Ensure button hover, ripple, and open/close transitions remain responsive during active playback.
- [x] Do not regress the existing lag improvements in the reader or speed-dial interactions.

## Styling and Accessibility

- [x] Use consistent iconography for page and sentence navigation.
- [x] Differentiate page navigation from sentence navigation clearly.
- [x] Make the larger Play/Pause button visually obvious without overwhelming the layout.
- [x] Preserve accessible labels, tooltips, and focus indicators.
- [x] Ensure sufficient contrast and click targets for all controls.
- [x] Ensure the widget remains usable with reduced motion preferences.

## Testing

- [x] Add component-level tests for visible/hidden behavior based on the TTS-controls-enabled flag.
- [x] Add tests for button order and disabled states.
- [x] Add tests ensuring the Play/Pause button uses the prominent styling variant.
- [x] Add integration coverage for text-only mode.
- [x] Add integration coverage for pretty-text mode.
- [x] Verify keyboard navigation and focus order.
- [x] Verify the widget does not cover the last readable lines in the scroll pane.

## Manual Verification

- [x] Confirm the widget appears at the bottom of the reading pane when TTS controls are enabled.
- [x] Confirm the widget disappears entirely when TTS controls are disabled.
- [x] Confirm button order matches the required sequence exactly.
- [x] Confirm Play/Pause is visibly larger than the other buttons.
- [x] Confirm all five controls work during live playback.
- [x] Confirm the widget behaves correctly in text-only view.
- [x] Confirm the widget behaves correctly in pretty-text view.
- [x] Confirm toggling stats/settings does not move or reset the widget.
- [x] Confirm no new lag or layout instability is introduced during playback.

## Acceptance Criteria

- [x] The TTS controls no longer render beneath the right-side stats/settings pane.
- [x] A dedicated player-style widget renders at the bottom of the text scroll pane.
- [x] The widget appears only when TTS controls are enabled.
- [x] The widget contains exactly five controls in the required order.
- [x] The Play/Pause control is the dominant visual action.
- [x] The widget works in both text-only and pretty-text modes without scroll regressions or noticeable lag.
