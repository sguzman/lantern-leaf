import { describe, expect, it } from "vitest";

import {
  highlightBorder,
  mapFontFamily,
  mapFontWeight,
  toCssRgba
} from "../src/theme/mapping";

describe("theme mapping", () => {
  it("maps highlight colors to stable rgba/border css strings", () => {
    const dayHighlight = { r: 0.2, g: 0.4, b: 0.7, a: 0.15 };
    const nightHighlight = { r: 0.8, g: 0.8, b: 0.5, a: 0.2 };

    expect(toCssRgba(dayHighlight)).toBe("rgba(51, 102, 179, 0.150)");
    expect(highlightBorder(dayHighlight)).toBe("rgb(102, 140, 198)");
    expect(toCssRgba(nightHighlight)).toBe("rgba(204, 204, 128, 0.200)");
    expect(highlightBorder(nightHighlight)).toBe("rgb(217, 217, 159)");
  });

  it("maps configured font families and weights with safe defaults", () => {
    expect(mapFontFamily("lexend")).toContain("Lexend");
    expect(mapFontFamily("noto-sans")).toContain("Noto Sans");
    expect(mapFontFamily("serif")).toContain("Noto Serif");
    expect(mapFontFamily("unknown-family")).toContain("Lexend");

    expect(mapFontWeight("light")).toBe(300);
    expect(mapFontWeight("normal")).toBe(400);
    expect(mapFontWeight("bold")).toBe(700);
    expect(mapFontWeight("unexpected")).toBe(400);
  });
});
