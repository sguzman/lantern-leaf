import { describe, expect, it } from "vitest";

import { computeReaderTypographyLayout } from "../src/components/readerTypography";
import type { ReaderSettingsView } from "../src/types";

function makeSettings(overrides: Partial<ReaderSettingsView> = {}): ReaderSettingsView {
  return {
    theme: "day",
    font_family: "lexend",
    font_weight: "bold",
    day_highlight: { r: 0.2, g: 0.4, b: 0.7, a: 0.15 },
    night_highlight: { r: 0.8, g: 0.8, b: 0.5, a: 0.2 },
    font_size: 22,
    line_spacing: 1.2,
    word_spacing: 0,
    letter_spacing: 0,
    margin_horizontal: 100,
    margin_vertical: 12,
    lines_per_page: 700,
    pause_after_sentence: 0.06,
    auto_scroll_tts: true,
    center_spoken_sentence: true,
    tts_speed: 2.5,
    tts_volume: 1,
    ...overrides
  };
}

describe("computeReaderTypographyLayout", () => {
  it("preserves current defaults unchanged", () => {
    const layout = computeReaderTypographyLayout(makeSettings());
    expect(layout).toEqual({
      fontSizePx: 22,
      lineSpacing: 1.2,
      horizontalMarginPx: 100,
      verticalMarginPx: 12,
      wordSpacingPx: 0,
      letterSpacingPx: 0
    });
  });

  it("clamps out-of-range values into stable bounds", () => {
    const layout = computeReaderTypographyLayout(
      makeSettings({
        font_size: 200,
        line_spacing: 9,
        word_spacing: -4,
        letter_spacing: 99,
        margin_horizontal: 9999,
        margin_vertical: -50
      })
    );
    expect(layout).toEqual({
      fontSizePx: 36,
      lineSpacing: 3,
      horizontalMarginPx: 600,
      verticalMarginPx: 0,
      wordSpacingPx: 0,
      letterSpacingPx: 24
    });
  });

  it("defensively handles non-finite input values", () => {
    const layout = computeReaderTypographyLayout(
      makeSettings({
        font_size: Number.NaN,
        line_spacing: Number.POSITIVE_INFINITY,
        word_spacing: Number.NaN,
        letter_spacing: Number.POSITIVE_INFINITY
      })
    );
    expect(layout.fontSizePx).toBe(12);
    expect(layout.lineSpacing).toBe(0.8);
    expect(layout.wordSpacingPx).toBe(0);
    expect(layout.letterSpacingPx).toBe(0);
  });
});
