import { describe, expect, it } from "vitest";

import { buildHtmlSentenceAnchorMap, normalizeSyncText } from "../src/components/htmlSync";

describe("normalizeSyncText", () => {
  it("removes punctuation noise and normalizes spacing", () => {
    expect(normalizeSyncText(" Hello,   world! [42] ")).toBe("hello world 42");
  });
});

describe("buildHtmlSentenceAnchorMap", () => {
  it("maps matching sentences to their corresponding anchors", () => {
    const out = buildHtmlSentenceAnchorMap(
      ["Intro", "First paragraph text goes here", "Second paragraph text goes here"],
      ["First paragraph text goes here.", "Second paragraph text goes here."],
      [1, 2]
    );

    expect(out.map).toEqual([1, 2]);
    expect(out.diagnostics.confidentMatches).toBe(2);
    expect(out.diagnostics.fallbackMatches).toBe(0);
  });

  it("drifts conservatively toward hint anchors when text match confidence is low", () => {
    const out = buildHtmlSentenceAnchorMap(
      ["contents", "chapter one", "chapter one body", "chapter two body"],
      ["Completely different text A", "Completely different text B", "Completely different text C"],
      [0, 2, 3]
    );

    expect(out.map).toEqual([0, 1, 2]);
    expect(out.diagnostics.fallbackMatches).toBe(3);
  });

  it("caps suspicious multi-anchor leaps unless the text evidence is exact", () => {
    const out = buildHtmlSentenceAnchorMap(
      [
        "preface",
        "chapter 1",
        "small bridge",
        "detour",
        "very far anchor partial opening of the sentence for matching"
      ],
      [
        "chapter 1",
        "very far anchor partial opening of the sentence for matching but the rest diverges into unrelated material and keeps going"
      ],
      [1, 4]
    );

    expect(out.map[0]).toBe(1);
    expect(out.map[1]).toBe(1);
    expect(out.diagnostics.cappedLeaps).toBe(1);
  });

  it("returns empty output when there are no anchors or no sentences", () => {
    expect(buildHtmlSentenceAnchorMap([], ["one"], [0]).map).toEqual([]);
    expect(buildHtmlSentenceAnchorMap(["one"], [], [0]).map).toEqual([]);
  });
});
