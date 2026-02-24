import type { HighlightColor } from "../types";

function clamp01(value: number): number {
  return Math.max(0, Math.min(1, value));
}

export function toCssRgba(color: HighlightColor): string {
  const r = Math.round(color.r * 255);
  const g = Math.round(color.g * 255);
  const b = Math.round(color.b * 255);
  return `rgba(${r}, ${g}, ${b}, ${color.a.toFixed(3)})`;
}

export function highlightBorder(color: HighlightColor): string {
  const factor = 0.25;
  const r = Math.round(clamp01(color.r + (1 - color.r) * factor) * 255);
  const g = Math.round(clamp01(color.g + (1 - color.g) * factor) * 255);
  const b = Math.round(clamp01(color.b + (1 - color.b) * factor) * 255);
  return `rgb(${r}, ${g}, ${b})`;
}

export function mapFontFamily(value: string | undefined): string {
  switch ((value ?? "").toLowerCase()) {
    case "serif":
      return "Noto Serif, Georgia, serif";
    case "monospace":
      return "Fira Code, Consolas, monospace";
    case "fira-code":
      return "Fira Code, Consolas, monospace";
    case "atkinson-hyperlegible":
      return "Atkinson Hyperlegible, Noto Sans, sans-serif";
    case "atkinson-hyperlegible-next":
      return "Atkinson Hyperlegible Next, Noto Sans, sans-serif";
    case "lexica-ultralegible":
      return "Lexica Ultralegible, Noto Sans, sans-serif";
    case "courier":
      return "Courier New, Courier, monospace";
    case "frank-gothic":
      return "Franklin Gothic Medium, Arial Narrow, sans-serif";
    case "hermit":
      return "Hermit, Fira Code, monospace";
    case "hasklug":
      return "Hasklug, Fira Code, monospace";
    case "noto-sans":
      return "Noto Sans, Segoe UI, sans-serif";
    case "sans":
      return "Noto Sans, Segoe UI, sans-serif";
    case "lexend":
    default:
      return "Lexend, Noto Sans, Segoe UI, sans-serif";
  }
}

export function mapFontWeight(value: string | undefined): number {
  switch ((value ?? "").toLowerCase()) {
    case "light":
      return 300;
    case "bold":
      return 700;
    case "normal":
    default:
      return 400;
  }
}
