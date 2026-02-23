import { describe, expect, it } from "vitest";

import {
  computeReaderTopBarVisibility,
  computeReaderTtsControlVisibility
} from "../src/components/layoutPolicies";

describe("reader layout policies", () => {
  it("applies top-bar visibility thresholds in priority order", () => {
    const tight = computeReaderTopBarVisibility(700);
    const medium = computeReaderTopBarVisibility(1100);
    const wide = computeReaderTopBarVisibility(1600);

    expect(tight.showSentenceButtons).toBe(false);
    expect(tight.showTextModeButton).toBe(false);
    expect(tight.showSettingsButton).toBe(false);
    expect(tight.showStatsButton).toBe(false);
    expect(tight.showTtsButton).toBe(false);

    expect(medium.showSentenceButtons).toBe(true);
    expect(medium.showTextModeButton).toBe(true);
    expect(medium.showSettingsButton).toBe(true);
    expect(medium.showStatsButton).toBe(false);
    expect(medium.showTtsButton).toBe(false);

    expect(wide.showSentenceButtons).toBe(true);
    expect(wide.showTextModeButton).toBe(true);
    expect(wide.showSettingsButton).toBe(true);
    expect(wide.showStatsButton).toBe(true);
    expect(wide.showTtsButton).toBe(true);
  });

  it("keeps a minimal non-wrapping TTS control set at narrow widths", () => {
    const narrow = computeReaderTtsControlVisibility(120);
    const medium = computeReaderTtsControlVisibility(420);
    const wide = computeReaderTtsControlVisibility(760);

    expect(narrow.showPlayButton).toBe(false);
    expect(narrow.showPauseButton).toBe(false);
    expect(narrow.showPlayPageButton).toBe(false);
    expect(narrow.showPlayHighlightButton).toBe(false);
    expect(narrow.showPrevSentenceButton).toBe(false);
    expect(narrow.showNextSentenceButton).toBe(false);
    expect(narrow.showRepeatButton).toBe(false);

    expect(medium.showPlayPageButton).toBe(true);
    expect(medium.showPlayHighlightButton).toBe(true);
    expect(medium.showPrevSentenceButton).toBe(true);
    expect(medium.showNextSentenceButton).toBe(false);
    expect(medium.showPlayButton).toBe(false);
    expect(medium.showPauseButton).toBe(false);

    expect(wide.showPlayButton).toBe(true);
    expect(wide.showPauseButton).toBe(true);
    expect(wide.showPlayPageButton).toBe(true);
    expect(wide.showPlayHighlightButton).toBe(true);
    expect(wide.showPrevSentenceButton).toBe(true);
    expect(wide.showNextSentenceButton).toBe(true);
    expect(wide.showRepeatButton).toBe(true);
  });
});
