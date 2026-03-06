function normalizeImageTarget(raw: string): string {
  const cleaned = raw
    .trim()
    .replace(/^<|>$/g, "")
    .split("#")[0]
    .split("?")[0]
    .replace(/\\/g, "/");
  let decoded = cleaned;
  try {
    decoded = decodeURIComponent(cleaned);
  } catch {
    decoded = cleaned;
  }
  const parts: string[] = [];
  for (const part of decoded.split("/")) {
    if (!part || part === ".") {
      continue;
    }
    if (part === "..") {
      parts.pop();
      continue;
    }
    parts.push(part);
  }
  return parts.join("/").toLowerCase();
}

function imageBaseName(raw: string): string {
  const normalized = normalizeImageTarget(raw);
  const parts = normalized.split("/");
  return parts[parts.length - 1] ?? normalized;
}

function resolveRelativeUrl(raw: string, baseUrl: string | null): string | null {
  const trimmed = raw.trim();
  if (!trimmed || !baseUrl) {
    return null;
  }
  if (
    trimmed.startsWith("http://") ||
    trimmed.startsWith("https://") ||
    trimmed.startsWith("data:") ||
    trimmed.startsWith("asset:") ||
    trimmed.startsWith("#")
  ) {
    return trimmed;
  }
  try {
    return new URL(trimmed, baseUrl).toString();
  } catch {
    return null;
  }
}

function rewriteCssUrls(
  rawCss: string,
  baseUrl: string | null,
  resolveTarget: (target: string) => string | null
): string {
  return rawCss.replace(/url\(([^)]+)\)/gi, (_match, rawTarget) => {
    const unwrapped = String(rawTarget ?? "")
      .trim()
      .replace(/^['"]|['"]$/g, "");
    if (!unwrapped || unwrapped.startsWith("data:") || unwrapped.startsWith("#")) {
      return `url(${unwrapped})`;
    }
    const resolved = resolveTarget(unwrapped) ?? resolveRelativeUrl(unwrapped, baseUrl) ?? unwrapped;
    return `url("${resolved}")`;
  });
}

function focusBrowserTabContent(container: HTMLDivElement): void {
  const browserWrapper = container.querySelector<HTMLElement>("[data-ll-browser-tab='1']");
  if (!browserWrapper) {
    return;
  }
  if (container.querySelector("[data-ll-browser-tab-focused='1']")) {
    return;
  }
  const htmlRoot = container.querySelector("html");
  const bodyRoot = container.querySelector("body");
  const headRoot = container.querySelector("head");
  const candidateSelectors = [
    ".mw-parser-output",
    ".mw-body-content",
    "main article",
    "main#content",
    "article",
    "[role='main']",
    "main",
    "#content",
    ".entry-content",
    ".post-content",
    ".article-content"
  ];
  let chosen: HTMLElement | null = null;
  let fallback: HTMLElement | null = null;
  let fallbackTextLen = 0;
  for (const selector of candidateSelectors) {
    const candidate = container.querySelector<HTMLElement>(selector);
    if (!candidate) {
      continue;
    }
    const textLen = (candidate.textContent ?? "").trim().length;
    if (textLen >= 600) {
      chosen = candidate;
      break;
    }
    if (textLen > fallbackTextLen) {
      fallback = candidate;
      fallbackTextLen = textLen;
    }
  }
  chosen ??= fallback;
  if (!chosen) {
    return;
  }
  const chosenTextLen = (chosen.textContent ?? "").trim().length;
  const refinedChild = Array.from(chosen.children)
    .filter((child): child is HTMLElement =>
      child instanceof HTMLElement &&
      ["section", "main", "article", "div"].includes(child.tagName.toLowerCase())
    )
    .map((child) => ({
      child,
      textLen: (child.textContent ?? "").trim().length
    }))
    .sort((left, right) => right.textLen - left.textLen)[0];
  if (
    refinedChild &&
    refinedChild.textLen >= 400 &&
    refinedChild.textLen * 2 >= chosenTextLen
  ) {
    chosen = refinedChild.child;
  }

  const focused = document.createElement("div");
  focused.dataset.llBaseUrl = browserWrapper.dataset.llBaseUrl ?? "";
  focused.dataset.llBrowserTab = "1";

  const title = container.querySelector("title")?.textContent?.trim();
  if (title) {
    const heading = document.createElement("h1");
    heading.textContent = title;
    focused.appendChild(heading);
  }

  const readingRoot = document.createElement("div");
  const classes = [
    "ll-browser-tab-root",
    ...(htmlRoot?.className?.split(/\s+/) ?? []),
    ...(bodyRoot?.className?.split(/\s+/) ?? [])
  ].filter(Boolean);
  readingRoot.className = Array.from(new Set(classes)).join(" ");
  const inlineStyle = [htmlRoot?.getAttribute("style"), bodyRoot?.getAttribute("style")]
    .filter(Boolean)
    .join("; ");
  if (inlineStyle.trim()) {
    readingRoot.setAttribute("style", inlineStyle);
  }

  const styles = headRoot ? Array.from(headRoot.querySelectorAll("style")) : [];
  for (const style of styles) {
    readingRoot.appendChild(style.cloneNode(true));
  }
  readingRoot.appendChild(chosen.cloneNode(true));
  focused.appendChild(readingRoot);

  browserWrapper.replaceWith(focused);
}

export function renderNativePrettyHtml(
  html: string,
  imageCandidates: Array<{ rawPath: string; src: string }>
): string {
  const container = document.createElement("div");
  container.innerHTML = html;
  focusBrowserTabContent(container);
  let baseUrl: string | null =
    container.querySelector<HTMLElement>("[data-ll-base-url]")?.dataset.llBaseUrl ?? null;
  const scopedRoot = container.querySelector("[data-ll-browser-tab='1']")
    ? ".reader-native-html-content .ll-browser-tab-root"
    : ".reader-native-html-content";
  const wrappers = Array.from(container.querySelectorAll<HTMLElement>("[data-ll-base-url]"));
  for (const wrapper of wrappers) {
    while (wrapper.firstChild) {
      wrapper.parentNode?.insertBefore(wrapper.firstChild, wrapper);
    }
    wrapper.remove();
  }
  const allowTags = new Set([
    "html",
    "head",
    "body",
    "a",
    "article",
    "aside",
    "b",
    "blockquote",
    "br",
    "code",
    "div",
    "em",
    "figcaption",
    "figure",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "header",
    "hr",
    "i",
    "img",
    "svg",
    "image",
    "g",
    "defs",
    "symbol",
    "use",
    "path",
    "picture",
    "rect",
    "circle",
    "ellipse",
    "line",
    "polyline",
    "polygon",
    "li",
    "main",
    "nav",
    "ol",
    "p",
    "pre",
    "s",
    "section",
    "small",
    "span",
    "strong",
    "source",
    "sub",
    "sup",
    "table",
    "tbody",
    "td",
    "th",
    "thead",
    "tr",
    "u",
    "ul",
    "style",
    "link",
  ]);
  const allowedGlobalAttrs = new Set([
    "id",
    "title",
    "lang",
    "dir",
    "role",
    "aria-label",
    "class",
    "style",
    "xmlns",
    "xmlns:xlink",
  ]);
  const allowedPerTagAttrs = new Map<string, Set<string>>([
    ["a", new Set(["href"])],
    ["img", new Set(["src", "alt", "width", "height", "loading"])],
    ["picture", new Set([])],
    ["source", new Set(["src", "srcset", "type", "media"])],
    ["td", new Set(["colspan", "rowspan"])],
    ["th", new Set(["colspan", "rowspan"])],
    ["style", new Set(["type", "media"])],
    [
      "svg",
      new Set([
        "viewbox",
        "width",
        "height",
        "preserveaspectratio",
        "version",
        "xmlns",
        "xmlns:xlink",
      ]),
    ],
    [
      "image",
      new Set(["href", "xlink:href", "width", "height", "x", "y", "preserveaspectratio"]),
    ],
    ["use", new Set(["href", "xlink:href"])],
  ]);

  const scopeCssToNativeContainer = (rawCss: string): string => {
    const scope = ".reader-native-html-content";
    let css = rawCss.replace(/@import[^;]+;/gi, "");
    css = css.replace(/@page\s*\{[\s\S]*?\}/gi, "");
    css = css.replace(/(^|})\s*([^@}{][^{]+)\{/g, (_m, sep, selectorGroup) => {
      const rewritten = String(selectorGroup)
        .split(",")
        .map((selector) => {
          const trimmed = selector.trim();
          if (!trimmed) {
            return trimmed;
          }
          if (
            trimmed.startsWith(scope) ||
            trimmed.startsWith("@") ||
            trimmed.startsWith("from") ||
            trimmed.startsWith("to") ||
            /\d+%\s*$/.test(trimmed)
          ) {
            return trimmed;
          }
          const normalized = trimmed
            .replace(/\bhtml\b/g, scopedRoot)
            .replace(/\bbody\b/g, scopedRoot)
            .replace(/\:root\b/g, scopedRoot);
          if (normalized.includes(scope)) {
            return normalized;
          }
          return `${scope} ${normalized}`;
        })
        .join(", ");
      return `${sep} ${rewritten}{`;
    });
    return css;
  };
  const unusedImages = [...imageCandidates];
  const resolveImageTarget = (target: string): string | null => {
    const normalizedTarget = normalizeImageTarget(target);
    if (!normalizedTarget) {
      return null;
    }
    const targetBaseName = imageBaseName(normalizedTarget);
    const matched = unusedImages.find((candidate) => {
      const candidateNormalized = normalizeImageTarget(candidate.rawPath);
      const candidateBaseName = imageBaseName(candidateNormalized);
      return (
        candidateNormalized === normalizedTarget ||
        candidateNormalized.endsWith(`/${normalizedTarget}`) ||
        candidateBaseName === targetBaseName ||
        candidateBaseName.endsWith(`-${targetBaseName}`)
      );
    });
    if (matched) {
      return matched.src;
    }
    if (
      normalizedTarget.startsWith("http://") ||
      normalizedTarget.startsWith("https://") ||
      normalizedTarget.startsWith("data:") ||
      normalizedTarget.startsWith("asset:")
    ) {
      return target;
    }
    return null;
  };
  const resolveResourceTarget = (target: string): string | null => {
    const direct = resolveImageTarget(target);
    if (direct) {
      return direct;
    }
    const absolute = resolveRelativeUrl(target, baseUrl);
    if (absolute) {
      return resolveImageTarget(absolute) ?? absolute;
    }
    return isSafeScheme(target) ? target : null;
  };
  const isSafeScheme = (value: string): boolean =>
    value.startsWith("http://") ||
    value.startsWith("https://") ||
    value.startsWith("data:") ||
    value.startsWith("asset:") ||
    value.startsWith("#");
  const toInternalAnchor = (raw: string): string | null => {
    if (raw.startsWith("#")) {
      return raw;
    }
    const hashIdx = raw.indexOf("#");
    if (hashIdx >= 0) {
      const fragment = raw.slice(hashIdx);
      return fragment.startsWith("#") ? fragment : null;
    }
    return null;
  };
  const rewriteSrcset = (raw: string): string => {
    return raw
      .split(",")
      .map((part) => {
        const trimmed = part.trim();
        if (!trimmed) {
          return "";
        }
        const [target, descriptor] = trimmed.split(/\s+/, 2);
        const resolved = resolveResourceTarget(target) ?? target;
        return descriptor ? `${resolved} ${descriptor}` : resolved;
      })
      .filter((value) => value.length > 0)
      .join(", ");
  };

  let anchorIndex = 0;
  const anchorTags = new Set([
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "p",
    "li",
    "blockquote",
    "pre",
  ]);
  const anchorableBlockTags = new Set([
    ...anchorTags,
    "div",
    "section",
  ]);
  const measureNodeText = (node: Element): number =>
    (node.textContent ?? "").replace(/\s+/g, " ").trim().length;
  const isLeafBlockAnchor = (element: Element): boolean => {
    const tag = element.tagName.toLowerCase();
    if (!["div", "section"].includes(tag)) {
      return false;
    }
    const textLen = measureNodeText(element);
    if (textLen < 48) {
      return false;
    }
    const directBlockChildren = Array.from(element.children).filter((child) => {
      const childTag = child.tagName.toLowerCase();
      if (!anchorableBlockTags.has(childTag) && childTag !== "article") {
        return false;
      }
      return measureNodeText(child) >= 24;
    });
    return directBlockChildren.length === 0;
  };
  const sanitizeNode = (node: Node): void => {
    if (node.nodeType === Node.ELEMENT_NODE) {
      const element = node as Element;
      const tag = element.tagName.toLowerCase();
      const rawImgSrc = tag === "img"
        ? (
            element.getAttribute("src") ??
            element.getAttribute("data-src") ??
            element.getAttribute("data-lazy-src") ??
            element.getAttribute("data-original") ??
            ""
          ).trim()
        : "";
      const rawImgSrcset = tag === "img"
        ? (
            element.getAttribute("srcset") ??
            element.getAttribute("data-srcset") ??
            element.getAttribute("data-lazy-srcset") ??
            ""
          ).trim()
        : "";
      const rawSourceSrc = tag === "source"
        ? (
            element.getAttribute("src") ??
            element.getAttribute("data-src") ??
            ""
          ).trim()
        : "";
      const rawSourceSrcset = tag === "source"
        ? (
            element.getAttribute("srcset") ??
            element.getAttribute("data-srcset") ??
            ""
          ).trim()
        : "";
      if (!allowTags.has(tag)) {
        if (tag === "script" || tag === "iframe" || tag === "object" || tag === "embed") {
          element.remove();
          return;
        }
        const parent = element.parentNode;
        if (!parent) {
          element.remove();
          return;
        }
        while (element.firstChild) {
          parent.insertBefore(element.firstChild, element);
        }
        parent.removeChild(element);
        return;
      }
      const allowAttrs = allowedPerTagAttrs.get(tag) ?? new Set<string>();
      const attrs = [...element.attributes];
      for (const attr of attrs) {
        const name = attr.name.toLowerCase();
        if (name.startsWith("on")) {
          element.removeAttribute(attr.name);
          continue;
        }
        if (allowedGlobalAttrs.has(name) || name.startsWith("aria-") || allowAttrs.has(name)) {
          continue;
        }
        element.removeAttribute(attr.name);
      }
      if (element.hasAttribute("style")) {
        const rewritten = rewriteCssUrls(
          element.getAttribute("style") ?? "",
          baseUrl,
          resolveResourceTarget
        );
        if (tag === "img") {
          element.setAttribute(
            "style",
            rewritten.replace(/(?:^|;)\s*min-width\s*:[^;]+;?/gi, ";")
          );
        } else {
          element.setAttribute("style", rewritten);
        }
      }
      if (tag === "img") {
        const resolved = resolveResourceTarget(rawImgSrc) ?? "";
        if (resolved) {
          element.setAttribute("src", resolved);
        } else {
          element.remove();
          return;
        }
        if (rawImgSrcset) {
          element.setAttribute("srcset", rewriteSrcset(rawImgSrcset));
        }
        if (!element.hasAttribute("loading")) {
          element.setAttribute("loading", "lazy");
        }
      } else if (tag === "source") {
        if (rawSourceSrc) {
          const resolved = resolveResourceTarget(rawSourceSrc);
          if (resolved) {
            element.setAttribute("src", resolved);
          } else {
            element.removeAttribute("src");
          }
        }
        if (rawSourceSrcset) {
          element.setAttribute("srcset", rewriteSrcset(rawSourceSrcset));
        }
      } else if (tag === "image") {
        const href =
          (element.getAttribute("href") ?? "").trim() ||
          (element.getAttribute("xlink:href") ?? "").trim();
        const resolved = resolveResourceTarget(href) ?? "";
        if (resolved) {
          element.setAttribute("href", resolved);
          element.setAttribute("xlink:href", resolved);
        } else {
          element.remove();
          return;
        }
      } else if (tag === "style") {
        const rawCss = element.textContent ?? "";
        element.textContent = rewriteCssUrls(
          scopeCssToNativeContainer(rawCss),
          baseUrl,
          resolveResourceTarget
        );
      } else if (tag === "link") {
        // Remove external/relative stylesheet links to avoid global style bleed.
        element.remove();
        return;
      } else if (tag === "a") {
        const href = (element.getAttribute("href") ?? "").trim();
        const resolvedImage = resolveImageTarget(href);
        const internal = toInternalAnchor(href);
        let resolved = "";
        if (resolvedImage) {
          resolved = resolvedImage;
        } else if (internal) {
          resolved = internal;
        } else if (resolveRelativeUrl(href, baseUrl)) {
          resolved = resolveRelativeUrl(href, baseUrl) ?? "";
        } else if (isSafeScheme(href)) {
          resolved = href;
        }
        if (!resolved) {
          element.removeAttribute("href");
        } else {
          element.setAttribute("href", resolved);
          if (resolved.startsWith("http://") || resolved.startsWith("https://")) {
            element.setAttribute("target", "_blank");
            element.setAttribute("rel", "noreferrer");
          }
        }
      }
      const children = [...node.childNodes];
      for (const child of children) {
        sanitizeNode(child);
      }
      if (anchorTags.has(tag) || isLeafBlockAnchor(element)) {
        element.setAttribute("data-ll-html-anchor", String(anchorIndex));
        anchorIndex += 1;
      }
    }
  };
  sanitizeNode(container);
  return container.innerHTML;
}
