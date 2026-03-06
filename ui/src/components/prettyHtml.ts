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

export function renderNativePrettyHtml(
  html: string,
  imageCandidates: Array<{ rawPath: string; src: string }>
): string {
  const container = document.createElement("div");
  container.innerHTML = html;
  let baseUrl: string | null =
    container.querySelector<HTMLElement>("[data-ll-base-url]")?.dataset.llBaseUrl ?? null;
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
            .replace(/\bhtml\b/g, scope)
            .replace(/\bbody\b/g, scope)
            .replace(/\:root\b/g, scope);
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
  const sanitizeNode = (node: Node): void => {
    if (node.nodeType === Node.ELEMENT_NODE) {
      const element = node as Element;
      const tag = element.tagName.toLowerCase();
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
        element.setAttribute("style", rewritten);
      }
      if (tag === "img") {
        const src = (element.getAttribute("src") ?? "").trim();
        const resolved = resolveResourceTarget(src) ?? "";
        if (resolved) {
          element.setAttribute("src", resolved);
        } else {
          element.remove();
          return;
        }
        if (!element.hasAttribute("loading")) {
          element.setAttribute("loading", "lazy");
        }
      } else if (tag === "source") {
        const src = (element.getAttribute("src") ?? "").trim();
        const srcset = (element.getAttribute("srcset") ?? "").trim();
        if (src) {
          const resolved = resolveResourceTarget(src);
          if (resolved) {
            element.setAttribute("src", resolved);
          } else {
            element.removeAttribute("src");
          }
        }
        if (srcset) {
          element.setAttribute("srcset", rewriteSrcset(srcset));
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
      if (anchorTags.has(tag)) {
        element.setAttribute("data-ll-html-anchor", String(anchorIndex));
        anchorIndex += 1;
      }
    }
    const children = [...node.childNodes];
    for (const child of children) {
      sanitizeNode(child);
    }
  };
  sanitizeNode(container);
  return container.innerHTML;
}
