import type { ReaderSettingsView } from "../types";

const MIN_FONT_SIZE_PX = 12;
const MAX_FONT_SIZE_PX = 36;
const MIN_LINE_SPACING = 0.8;
const MAX_LINE_SPACING = 3.0;
const MIN_HORIZONTAL_MARGIN_PX = 0;
const MAX_HORIZONTAL_MARGIN_PX = 600;
const MIN_VERTICAL_MARGIN_PX = 0;
const MAX_VERTICAL_MARGIN_PX = 240;

export interface ReaderTypographyLayout {
  fontSizePx: number;
  lineSpacing: number;
  horizontalMarginPx: number;
  verticalMarginPx: number;
}

function clamp(value: number, min: number, max: number): number {
  if (!Number.isFinite(value)) {
    return min;
  }
  return Math.min(max, Math.max(min, value));
}

export function computeReaderTypographyLayout(settings: ReaderSettingsView): ReaderTypographyLayout {
  return {
    fontSizePx: clamp(settings.font_size, MIN_FONT_SIZE_PX, MAX_FONT_SIZE_PX),
    lineSpacing: clamp(settings.line_spacing, MIN_LINE_SPACING, MAX_LINE_SPACING),
    horizontalMarginPx: clamp(
      settings.margin_horizontal,
      MIN_HORIZONTAL_MARGIN_PX,
      MAX_HORIZONTAL_MARGIN_PX
    ),
    verticalMarginPx: clamp(
      settings.margin_vertical,
      MIN_VERTICAL_MARGIN_PX,
      MAX_VERTICAL_MARGIN_PX
    )
  };
}
