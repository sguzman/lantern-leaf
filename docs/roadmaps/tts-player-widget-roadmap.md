# TTS Player Widget Roadmap

## Goal

Replace the current right-side TTS controls placement with a dedicated audio-player-style control bar anchored at the bottom of the text scroll pane. The widget should only appear when TTS controls are enabled, and should behave consistently in both text-only and pretty-text reading modes.

## Product Requirements

- [ ] Move the TTS control widget out of the right-side stats/settings column.
- [ ] Render the TTS control widget at the bottom of the active text scroll pane.
- [ ] Show the widget only when TTS controls are enabled.
- [ ] Hide the widget entirely when TTS controls are disabled.
- [ ] Present controls in this exact left-to-right order:
  - [ ] Previous page
  - [ ] Previous sentence
  - [ ] Play/Pause
  - [ ] Next sentence
  - [ ] Next page
- [ ] Make the Play/Pause button visually larger and more prominent than the other controls.
- [ ] Keep the widget usable in both text-only and pretty-text modes.

## UX and Layout

- [ ] Anchor the player widget to the bottom of the reading pane rather than the global app frame.
- [ ] Ensure the widget does not overlap or obscure the active reading content.
- [ ] Add bottom padding or layout reservation so the final lines of text remain readable above the widget.
- [ ] Keep the widget stable while playback state changes so it does not shift surrounding layout.
- [ ] Preserve responsive behavior for narrow widths and mobile-sized windows.
- [ ] Maintain clear visual separation from the text content while still feeling integrated with the reader.
- [ ] Use a player-like visual hierarchy so the central Play/Pause action is dominant.

## Interaction Model

- [ ] Wire `Previous page` to the existing page-back behavior for paged content.
- [ ] Wire `Next page` to the existing page-forward behavior for paged content.
- [ ] Wire `Previous sentence` to the existing sentence-back behavior.
- [ ] Wire `Next sentence` to the existing sentence-forward behavior.
- [ ] Wire `Play/Pause` to the existing playback toggle behavior.
- [ ] Keep button disabled states accurate when an action is unavailable.
- [ ] Preserve keyboard and focus behavior for all controls.
- [ ] Ensure the widget updates immediately when playback state changes from external actions or hotkeys.

## Reader Integration

- [ ] Render the widget inside the reader shell so it follows the currently active reading pane.
- [ ] Ensure the widget works correctly whether the right-side panel is showing stats or settings.
- [ ] Ensure swapping stats/settings does not affect widget position, state, or scroll.
- [ ] Ensure changing between text-only and pretty-text keeps the widget present and functional.
- [ ] Ensure widget visibility is driven by the same TTS-controls-enabled source of truth used elsewhere in the app.

## State and Performance

- [ ] Avoid introducing extra rerender churn into the reading pane during playback.
- [ ] Keep the control bar component isolated from high-frequency reader updates where possible.
- [ ] Ensure button hover, ripple, and open/close transitions remain responsive during active playback.
- [ ] Do not regress the existing lag improvements in the reader or speed-dial interactions.

## Styling and Accessibility

- [ ] Use consistent iconography for page and sentence navigation.
- [ ] Differentiate page navigation from sentence navigation clearly.
- [ ] Make the larger Play/Pause button visually obvious without overwhelming the layout.
- [ ] Preserve accessible labels, tooltips, and focus indicators.
- [ ] Ensure sufficient contrast and click targets for all controls.
- [ ] Ensure the widget remains usable with reduced motion preferences.

## Testing

- [ ] Add component-level tests for visible/hidden behavior based on the TTS-controls-enabled flag.
- [ ] Add tests for button order and disabled states.
- [ ] Add tests ensuring the Play/Pause button uses the prominent styling variant.
- [ ] Add integration coverage for text-only mode.
- [ ] Add integration coverage for pretty-text mode.
- [ ] Verify keyboard navigation and focus order.
- [ ] Verify the widget does not cover the last readable lines in the scroll pane.

## Manual Verification

- [ ] Confirm the widget appears at the bottom of the reading pane when TTS controls are enabled.
- [ ] Confirm the widget disappears entirely when TTS controls are disabled.
- [ ] Confirm button order matches the required sequence exactly.
- [ ] Confirm Play/Pause is visibly larger than the other buttons.
- [ ] Confirm all five controls work during live playback.
- [ ] Confirm the widget behaves correctly in text-only view.
- [ ] Confirm the widget behaves correctly in pretty-text view.
- [ ] Confirm toggling stats/settings does not move or reset the widget.
- [ ] Confirm no new lag or layout instability is introduced during playback.

## Acceptance Criteria

- [ ] The TTS controls no longer render beneath the right-side stats/settings pane.
- [ ] A dedicated player-style widget renders at the bottom of the text scroll pane.
- [ ] The widget appears only when TTS controls are enabled.
- [ ] The widget contains exactly five controls in the required order.
- [ ] The Play/Pause control is the dominant visual action.
- [ ] The widget works in both text-only and pretty-text modes without scroll regressions or noticeable lag.
