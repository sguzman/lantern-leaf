// @vitest-environment jsdom
import { describe, expect, it } from "vitest";

import { renderNativePrettyHtml } from "../src/components/prettyHtml";

describe("renderNativePrettyHtml", () => {
  it("sanitizes unsafe markup and rewrites image/link targets", () => {
    const html = `
      <section>
        <h1 onclick="evil()">Title</h1>
        <p style="color:red">Body <a href="https://example.com/path">external</a></p>
        <img src="OPS/images/cover.jpg" />
        <script>alert('xss')</script>
      </section>
    `;
    const out = renderNativePrettyHtml(html, [
      {
        rawPath: "images/img-0001-aabbccddeeff-cover.jpg",
        src: "asset:/cache/images/img-0001-aabbccddeeff-cover.jpg",
      },
    ]);
    expect(out).toContain('data-ll-html-anchor="0"');
    expect(out).toContain('target="_blank"');
    expect(out).toContain('rel="noreferrer"');
    expect(out).toContain('src="asset:/cache/images/img-0001-aabbccddeeff-cover.jpg"');
    expect(out).not.toContain("<script");
    expect(out).not.toContain("onclick=");
    expect(out).toContain('style="color:red"');
  });

  it("preserves internal anchors and table/footnote-like content", () => {
    const html = `
      <article>
        <p id="fnref1"><a href="#fn1">[1]</a></p>
        <table><tbody><tr><td>row</td></tr></tbody></table>
        <p id="fn1">Footnote body</p>
      </article>
    `;
    const out = renderNativePrettyHtml(html, []);
    expect(out).toContain('href="#fn1"');
    expect(out).toContain("<table>");
    expect(out).toContain("Footnote body");
    expect(out).toContain('data-ll-html-anchor="0"');
  });

  it("does not transform markdown-style link/image syntax into HTML tags", () => {
    const html = `<p>Raw markdown [link](doc.md) and ![img](cover.png)</p>`;
    const out = renderNativePrettyHtml(html, []);
    expect(out).toContain("[link](doc.md)");
    expect(out).toContain("![img](cover.png)");
    expect(out).not.toContain('href="doc.md"');
    expect(out).not.toContain("<img");
  });
});
