import assert from "node:assert/strict";
import { spawn, spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { createHash } from "node:crypto";
import { Builder, By, Capabilities, Key } from "selenium-webdriver";

const __dirname = fileURLToPath(new URL(".", import.meta.url));
const repoRoot = path.resolve(__dirname, "..", "..");
const tauriDriverPath = path.resolve(
  os.homedir(),
  ".cargo",
  "bin",
  process.platform === "win32" ? "tauri-driver.exe" : "tauri-driver"
);

const webdriverServer = "http://127.0.0.1:4444/";
const PANDOC_PIPELINE_REV = "pandoc-clean-v1";
const QUACK_CHECK_PIPELINE_REV = "quack-check-pdf-v2";
const DEFAULT_QUACK_TEXT_FILENAME = "transcript.txt";
const CALIBRE_CACHE_REV = "calibre-cache-v1";

function cacheRootFromEnv() {
  const override = process.env.LANTERNLEAF_CACHE_DIR;
  if (typeof override === "string" && override.trim().length > 0) {
    return path.resolve(override.trim());
  }
  return path.resolve(repoRoot, ".cache");
}

function runOrThrow(command, args, cwd) {
  const result = spawnSync(command, args, {
    cwd,
    stdio: "inherit",
    shell: process.platform === "win32"
  });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(" ")} failed with status ${result.status}`);
  }
}

function sha256Hex(payload) {
  return createHash("sha256").update(payload).digest("hex");
}

function sourceCacheHash(sourcePath) {
  return sha256Hex(readFileSync(sourcePath));
}

function quackTextFilenameFromConfig(rawConfig) {
  const section = rawConfig.match(/\[output\]([\s\S]*?)(?:\n\[|$)/m);
  if (!section) {
    return DEFAULT_QUACK_TEXT_FILENAME;
  }
  const match = section[1].match(/^\s*text_filename\s*=\s*"([^"]+)"/m);
  if (!match || !match[1]?.trim()) {
    return DEFAULT_QUACK_TEXT_FILENAME;
  }
  return match[1].trim();
}

function seedPandocCacheForSource(sourcePath, text) {
  const sourceDigest = sourceCacheHash(sourcePath);
  const sourceStats = statSync(sourcePath);
  const filterPath = path.resolve(repoRoot, "conf", "pandoc", "strip-nontext.lua");
  const filterSha256 = sha256Hex(readFileSync(filterPath));

  const meta = [
    `source_len = ${sourceStats.size}`,
    `source_modified_unix_secs = ${Math.floor(sourceStats.mtimeMs / 1000)}`,
    `pipeline_rev = "${PANDOC_PIPELINE_REV}"`,
    `filter_sha256 = "${filterSha256}"`,
    ""
  ].join("\n");

  const cacheDir = path.join(cacheRootFromEnv(), sourceDigest);
  mkdirSync(cacheDir, { recursive: true });
  writeFileSync(path.join(cacheDir, "source-plain.txt"), text, "utf8");
  writeFileSync(path.join(cacheDir, "source-plain.meta.toml"), meta, "utf8");
}

function seedPdfCacheForSource(sourcePath, text) {
  const sourceDigest = sourceCacheHash(sourcePath);
  const sourceStats = statSync(sourcePath);
  const quackConfigPath = path.resolve(repoRoot, "conf", "quack-check.toml");
  const quackConfig = readFileSync(quackConfigPath, "utf8");
  const quackConfigSha256 = sha256Hex(quackConfig);
  const quackTextFilename = quackTextFilenameFromConfig(quackConfig);

  const meta = [
    `source_len = ${sourceStats.size}`,
    `source_modified_unix_secs = ${Math.floor(sourceStats.mtimeMs / 1000)}`,
    `pipeline_rev = "${QUACK_CHECK_PIPELINE_REV}"`,
    `quack_config_sha256 = "${quackConfigSha256}"`,
    `quack_text_filename = "${quackTextFilename}"`,
    ""
  ].join("\n");

  const cacheDir = path.join(cacheRootFromEnv(), sourceDigest, "pdf");
  mkdirSync(cacheDir, { recursive: true });
  writeFileSync(path.join(cacheDir, "source-plain.txt"), text, "utf8");
  writeFileSync(path.join(cacheDir, "source-plain.meta.toml"), meta, "utf8");
}

function createEpubFixture(sourcePath, marker) {
  const pythonScript = [
    "import sys, zipfile",
    "epub_path = sys.argv[1]",
    "marker = sys.argv[2]",
    "container_xml = '''<?xml version=\"1.0\" encoding=\"UTF-8\"?>",
    "<container version=\"1.0\" xmlns=\"urn:oasis:names:tc:opendocument:xmlns:container\">",
    "  <rootfiles>",
    "    <rootfile full-path=\"content.opf\" media-type=\"application/oebps-package+xml\"/>",
    "  </rootfiles>",
    "</container>'''",
    "content_opf = f'''<?xml version=\"1.0\" encoding=\"UTF-8\"?>",
    "<package xmlns=\"http://www.idpf.org/2007/opf\" unique-identifier=\"BookId\" version=\"2.0\">",
    "  <metadata xmlns:dc=\"http://purl.org/dc/elements/1.1/\">",
    "    <dc:title>Runtime Fixture</dc:title>",
    "    <dc:language>en</dc:language>",
    "    <dc:identifier id=\"BookId\">runtime-fixture-id</dc:identifier>",
    "  </metadata>",
    "  <manifest>",
    "    <item id=\"chapter\" href=\"ch1.xhtml\" media-type=\"application/xhtml+xml\"/>",
    "  </manifest>",
    "  <spine>",
    "    <itemref idref=\"chapter\"/>",
    "  </spine>",
    "</package>'''",
    "chapter = f'''<?xml version=\"1.0\" encoding=\"UTF-8\"?>",
    "<html xmlns=\"http://www.w3.org/1999/xhtml\"><body>",
    "<p>{marker} first sentence. {marker} second sentence. {marker} third sentence.</p>",
    "</body></html>'''",
    "with zipfile.ZipFile(epub_path, \"w\") as archive:",
    "    archive.writestr(\"mimetype\", \"application/epub+zip\", compress_type=zipfile.ZIP_STORED)",
    "    archive.writestr(\"META-INF/container.xml\", container_xml)",
    "    archive.writestr(\"content.opf\", content_opf)",
    "    archive.writestr(\"ch1.xhtml\", chapter)"
  ].join("\n");
  const result = spawnSync("python3", ["-c", pythonScript, sourcePath, marker], {
    cwd: repoRoot,
    encoding: "utf8"
  });
  if (result.status === 0) {
    return;
  }

  // Fallback path for environments without python3: use a synthetic .epub plus seeded cache.
  writeFileSync(sourcePath, `synthetic epub fixture ${Date.now()}`, "utf8");
  seedPandocCacheForSource(
    sourcePath,
    `${marker} first sentence. ${marker} second sentence. ${marker} third sentence.`
  );
}

function normalizeCalibreExt(raw) {
  const normalized = String(raw ?? "").trim().replace(/^\./, "").toLowerCase();
  if (normalized === "markdown") {
    return "md";
  }
  return normalized;
}

function sanitizeCalibreExts(exts) {
  const out = [];
  for (const raw of exts ?? []) {
    const normalized = normalizeCalibreExt(raw);
    const mapped =
      normalized === "epub" || normalized === "pdf" || normalized === "txt" || normalized === "md"
        ? normalized
        : null;
    if (mapped && !out.includes(mapped)) {
      out.push(mapped);
    }
  }
  return out.length > 0 ? out : ["epub", "pdf", "md", "txt"];
}

function sanitizeCalibreServerUrls(urls) {
  const out = [];
  for (const raw of urls ?? []) {
    const normalized = String(raw ?? "").trim().replace(/\/+$/, "");
    if (!normalized.startsWith("http://") && !normalized.startsWith("https://")) {
      continue;
    }
    if (!out.includes(normalized)) {
      out.push(normalized);
    }
  }
  return out.length > 0 ? out : ["http://127.0.0.1:8080", "http://localhost:8080"];
}

function sanitizeCalibreLibraryUrl(raw) {
  const normalized = String(raw ?? "").trim();
  if (!normalized.startsWith("http://") && !normalized.startsWith("https://")) {
    return null;
  }
  return normalized;
}

function toTomlString(value) {
  return JSON.stringify(String(value));
}

function buildCalibreCacheSignature(config) {
  const hash = createHash("sha256");
  hash.update(CALIBRE_CACHE_REV);
  hash.update(String(config.calibredb_bin ?? "calibredb"));

  const libraryUrl = sanitizeCalibreLibraryUrl(config.library_url);
  if (libraryUrl) {
    hash.update(libraryUrl);
    hash.update(Buffer.from([0]));
  }
  for (const url of sanitizeCalibreServerUrls(config.server_urls)) {
    hash.update(url);
    hash.update(Buffer.from([0]));
  }
  if (config.state_path) {
    hash.update(String(config.state_path));
  }
  if (config.library_path) {
    hash.update(String(config.library_path));
  }
  hash.update(Buffer.from([config.allow_local_library_fallback ? 1 : 0]));
  for (const ext of sanitizeCalibreExts(config.allowed_extensions)) {
    hash.update(ext);
    hash.update(Buffer.from([0]));
  }
  return hash.digest("hex");
}

function writeCalibreConfigFixture(configPath, config) {
  const lines = [
    "[calibre]",
    `enabled = ${config.enabled ? "true" : "false"}`,
    `library_url = ${toTomlString(config.library_url ?? "")}`,
    `state_path = ${toTomlString(config.state_path ?? "")}`,
    `calibredb_bin = ${toTomlString(config.calibredb_bin ?? "calibredb")}`,
    `server_urls = [${(config.server_urls ?? []).map((value) => toTomlString(value)).join(", ")}]`,
    `server_username = ${toTomlString(config.server_username ?? "")}`,
    `server_password = ${toTomlString(config.server_password ?? "")}`,
    `allow_local_library_fallback = ${config.allow_local_library_fallback ? "true" : "false"}`,
    `allowed_extensions = [${(config.allowed_extensions ?? []).map((value) => toTomlString(value)).join(", ")}]`,
    `columns = [${(config.columns ?? []).map((value) => toTomlString(value)).join(", ")}]`,
    `list_cache_ttl_secs = ${Math.max(0, Math.trunc(config.list_cache_ttl_secs ?? 600))}`,
    "",
    "[calibre.content_server]",
    `username = ${toTomlString(config.content_username ?? "")}`,
    `password = ${toTomlString(config.content_password ?? "")}`,
    ""
  ];
  writeFileSync(configPath, lines.join("\n"), "utf8");
}

function writeCalibreCacheFixture(cachePaths, signature, books) {
  const lines = [
    `rev = ${toTomlString(CALIBRE_CACHE_REV)}`,
    `generated_unix_secs = ${Math.floor(Date.now() / 1000)}`,
    `signature = ${toTomlString(signature)}`,
    ""
  ];

  for (const book of books) {
    lines.push("[[books]]");
    lines.push(`id = ${Math.max(0, Math.trunc(book.id))}`);
    lines.push(`title = ${toTomlString(book.title)}`);
    lines.push(`extension = ${toTomlString(book.extension)}`);
    lines.push(`authors = ${toTomlString(book.authors)}`);
    if (book.year !== null && book.year !== undefined) {
      lines.push(`year = ${Math.trunc(book.year)}`);
    }
    if (book.file_size_bytes !== null && book.file_size_bytes !== undefined) {
      lines.push(`file_size_bytes = ${Math.max(0, Math.trunc(book.file_size_bytes))}`);
    }
    if (book.path) {
      lines.push(`path = ${toTomlString(book.path)}`);
    }
    if (book.cover_thumbnail) {
      lines.push(`cover_thumbnail = ${toTomlString(book.cover_thumbnail)}`);
    }
    lines.push("");
  }

  for (const cachePath of cachePaths) {
    mkdirSync(path.dirname(cachePath), { recursive: true });
    writeFileSync(cachePath, lines.join("\n"), "utf8");
  }
}

function resolveTauriBinary() {
  const binaryName = process.platform === "win32" ? "lanternleaf-tauri.exe" : "lanternleaf-tauri";
  const candidates = [
    path.resolve(repoRoot, "src-tauri", "target", "debug", binaryName),
    path.resolve(repoRoot, "target", "debug", binaryName)
  ];

  for (const candidate of candidates) {
    if (existsSync(candidate)) {
      return candidate;
    }
  }
  throw new Error(
    `Unable to find Tauri debug binary (${binaryName}); checked: ${candidates.join(", ")}`
  );
}

async function delay(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function createDriver(applicationPath) {
  const capabilities = new Capabilities();
  capabilities.setBrowserName("wry");
  capabilities.set("tauri:options", {
    application: applicationPath
  });

  const lastErrors = [];
  for (let attempt = 1; attempt <= 40; attempt += 1) {
    try {
      return await new Builder()
        .withCapabilities(capabilities)
        .usingServer(webdriverServer)
        .build();
    } catch (error) {
      lastErrors.push(String(error));
      await delay(250);
    }
  }
  throw new Error(
    `Failed to establish webdriver session after retries: ${lastErrors.slice(-3).join(" | ")}`
  );
}

async function waitForElement(driver, locator, timeoutMs = 20000) {
  const startedAt = Date.now();
  while (Date.now() - startedAt <= timeoutMs) {
    try {
      const element = await driver.findElement(locator);
      if (await element.isDisplayed()) {
        return element;
      }
    } catch {
      // keep polling
    }
    await delay(200);
  }
  throw new Error(`Timed out waiting for element ${locator.toString()} after ${timeoutMs}ms`);
}

async function findVisibleElement(driver, locator, timeoutMs = 2500) {
  try {
    return await waitForElement(driver, locator, timeoutMs);
  } catch {
    return null;
  }
}

async function waitForNoElement(driver, locator, timeoutMs = 10000) {
  const startedAt = Date.now();
  while (Date.now() - startedAt <= timeoutMs) {
    try {
      const elements = await driver.findElements(locator);
      if (elements.length === 0) {
        return;
      }
      let visibleCount = 0;
      for (const element of elements) {
        if (await element.isDisplayed()) {
          visibleCount += 1;
        }
      }
      if (visibleCount === 0) {
        return;
      }
    } catch {
      return;
    }
    await delay(200);
  }
  throw new Error(`Element ${locator.toString()} remained present after ${timeoutMs}ms`);
}

function xpathLiteral(value) {
  if (!value.includes("'")) {
    return `'${value}'`;
  }
  if (!value.includes('"')) {
    return `"${value}"`;
  }
  const parts = value.split("'");
  return `concat(${parts.map((part, idx) => `${idx > 0 ? `,"'",` : ""}'${part}'`).join("")})`;
}

async function pickAvailableSentenceIndex(driver, preferredIdx = 14) {
  const sentenceIdx = await driver.executeScript((preferred) => {
    const indices = Array.from(
      document.querySelectorAll("button[data-testid^='reader-sentence-']")
    )
      .map((node) => {
        const raw = node.getAttribute("data-testid") ?? "";
        const suffix = raw.replace("reader-sentence-", "");
        const parsed = Number.parseInt(suffix, 10);
        return Number.isFinite(parsed) ? parsed : null;
      })
      .filter((value) => value !== null)
      .sort((left, right) => left - right);

    if (indices.length === 0) {
      return null;
    }
    if (indices.includes(preferred)) {
      return preferred;
    }
    return indices[Math.floor(indices.length / 2)];
  }, preferredIdx);

  if (sentenceIdx === null || sentenceIdx === undefined) {
    throw new Error("No sentence buttons were rendered in reader view");
  }
  return Number(sentenceIdx);
}

async function waitForText(driver, locator, expectedText, timeoutMs = 10000) {
  const startedAt = Date.now();
  while (Date.now() - startedAt <= timeoutMs) {
    try {
      const value = (await driver.findElement(locator).getText()).trim();
      if (value === expectedText) {
        return value;
      }
    } catch {
      // keep polling
    }
    await delay(200);
  }
  throw new Error(`Timed out waiting for ${locator.toString()} text "${expectedText}"`);
}

async function waitForInputValue(driver, locator, expectedValue, timeoutMs = 10000) {
  const startedAt = Date.now();
  while (Date.now() - startedAt <= timeoutMs) {
    try {
      const value = await driver.findElement(locator).getAttribute("value");
      if (String(value) === expectedValue) {
        return value;
      }
    } catch {
      // keep polling
    }
    await delay(200);
  }
  throw new Error(`Timed out waiting for ${locator.toString()} value "${expectedValue}"`);
}

async function waitForMarkerAttribute(driver, testId, attribute, predicate, timeoutMs = 10000) {
  const startedAt = Date.now();
  while (Date.now() - startedAt <= timeoutMs) {
    const value = await driver.executeScript(
      (markerTestId, markerAttr) => {
        const node = document.querySelector(`[data-testid='${markerTestId}']`);
        if (!node) {
          return null;
        }
        return node.getAttribute(markerAttr);
      },
      testId,
      attribute
    );
    if (value !== null && predicate(String(value))) {
      return String(value);
    }
    await delay(200);
  }
  throw new Error(
    `Timed out waiting for marker ${testId} attribute ${attribute} to satisfy predicate`
  );
}

async function setNumericSetting(driver, baseTestId, nextValue) {
  const locator = By.css(`[data-testid='${baseTestId}-input']`);
  const input = await waitForElement(driver, locator);
  await input.click();
  await input.sendKeys(Key.chord(Key.CONTROL, "a"));
  await input.sendKeys(String(nextValue));
  await input.sendKeys(Key.ENTER);
  const renderedValue = await waitForInputValue(driver, locator, String(nextValue), 2500).catch(
    async () => {
      const startedAt = Date.now();
      while (Date.now() - startedAt <= 10000) {
        const value = await driver.findElement(locator).getAttribute("value");
        const asNumber = Number(value);
        if (Number.isFinite(asNumber) && Math.abs(asNumber - Number(nextValue)) < 0.011) {
          return value;
        }
        await delay(200);
      }
      throw new Error(
        `Timed out waiting for ${locator.toString()} numeric value close to ${nextValue}`
      );
    }
  );
  return renderedValue;
}

async function assertHighlightedSentenceVisible(driver, timeoutMs = 15000) {
  const startedAt = Date.now();
  while (Date.now() - startedAt <= timeoutMs) {
    const result = await driver.executeScript(() => {
      const container = document.querySelector("[data-testid='reader-sentence-scroll-container']");
      if (!container) {
        return { ready: false, visible: false };
      }
      const highlighted = container.querySelector("[data-highlighted='1']");
      if (!highlighted) {
        return { ready: false, visible: false };
      }
      const containerRect = container.getBoundingClientRect();
      const highlightedRect = highlighted.getBoundingClientRect();
      const visible =
        highlightedRect.top >= containerRect.top && highlightedRect.bottom <= containerRect.bottom;
      return {
        ready: true,
        visible
      };
    });
    if (result.ready && result.visible) {
      return;
    }
    await delay(200);
  }
  throw new Error("Highlighted sentence is not visible inside the sentence scroll container");
}

async function maybeSetWindowRect(driver, width, height) {
  try {
    await driver.manage().window().setRect({ width, height });
    await delay(300);
  } catch {
    // Some platforms/drivers do not support setRect; continue without failing.
  }
}

async function assertElementHeightAtMost(driver, testId, maxHeight, timeoutMs = 10000) {
  const startedAt = Date.now();
  while (Date.now() - startedAt <= timeoutMs) {
    const result = await driver.executeScript(
      (currentTestId) => {
        const node = document.querySelector(`[data-testid='${currentTestId}']`);
        if (!node) {
          return { ready: false, height: 0 };
        }
        const rect = node.getBoundingClientRect();
        return { ready: true, height: rect.height };
      },
      testId
    );
    if (result.ready && result.height > 0 && result.height <= maxHeight) {
      return result.height;
    }
    await delay(200);
  }
  throw new Error(`Element ${testId} exceeded max height ${maxHeight}px`);
}

function parseSentencePosition(summaryText) {
  const match = summaryText.match(/Sentence:\s*(\d+)\s*\/\s*(\d+)/);
  if (!match) {
    throw new Error(`Unable to parse sentence position from summary: ${summaryText}`);
  }
  return {
    current: Number(match[1]),
    total: Number(match[2])
  };
}

function parsePlaybackState(summaryText) {
  const match = summaryText.match(/State:\s*([a-zA-Z]+)/);
  if (!match) {
    throw new Error(`Unable to parse playback state from summary: ${summaryText}`);
  }
  return match[1].toLowerCase();
}

async function waitForPlaybackState(driver, expectedState, timeoutMs = 10000) {
  const summaryLocator = By.css("[data-testid='reader-tts-state-summary']");
  const startedAt = Date.now();
  while (Date.now() - startedAt <= timeoutMs) {
    try {
      const text = await driver.findElement(summaryLocator).getText();
      if (parsePlaybackState(text) === expectedState.toLowerCase()) {
        return text;
      }
    } catch {
      // keep polling
    }
    await delay(200);
  }
  throw new Error(`Timed out waiting for playback state "${expectedState}"`);
}

async function highlightedSentenceIndex(driver) {
  const rawValue = await driver.executeScript(() => {
    const active = document.querySelector(
      "button[data-testid^='reader-sentence-'][data-highlighted='1']"
    );
    if (!active) {
      return null;
    }
    const testId = active.getAttribute("data-testid") ?? "";
    const match = testId.match(/reader-sentence-(\d+)/);
    if (!match) {
      return null;
    }
    return Number.parseInt(match[1], 10);
  });
  if (rawValue === null || rawValue === undefined) {
    return null;
  }
  return Number(rawValue);
}

async function waitForHighlightedSentenceIndex(driver, predicate, timeoutMs = 10000) {
  const startedAt = Date.now();
  while (Date.now() - startedAt <= timeoutMs) {
    const idx = await highlightedSentenceIndex(driver);
    if (idx !== null && predicate(idx)) {
      return idx;
    }
    await delay(200);
  }
  throw new Error("Timed out waiting for highlighted sentence index predicate");
}

test("tauri runner opens source and exercises core reader controls", async (t) => {
  const timeoutMs = 7 * 60 * 1000;
  if (typeof t.setTimeout === "function") {
    t.setTimeout(timeoutMs);
  }

  if (!existsSync(tauriDriverPath)) {
    throw new Error(
      `tauri-driver is missing at ${tauriDriverPath}. Install it with: cargo install tauri-driver --locked`
    );
  }

  const tmpDir = path.resolve(repoRoot, "tmp");
  mkdirSync(tmpDir, { recursive: true });
  const uniqueRunId = Date.now();
  const runtimeCacheRoot = path.resolve(tmpDir, `tauri-e2e-cache-${uniqueRunId}`);
  const previousCacheDirEnv = process.env.LANTERNLEAF_CACHE_DIR;
  const previousCalibreConfigPathEnv = process.env.CALIBRE_CONFIG_PATH;
  process.env.LANTERNLEAF_CACHE_DIR = runtimeCacheRoot;
  rmSync(runtimeCacheRoot, { recursive: true, force: true });
  mkdirSync(runtimeCacheRoot, { recursive: true });
  const sourceFileName = `tauri-e2e-source-${uniqueRunId}.txt`;
  const sourcePath = path.resolve(tmpDir, sourceFileName);
  writeFileSync(
    sourcePath,
    Array.from(
      { length: 120 },
      (_, idx) =>
        `This is tauri sentence number ${idx + 1} for end-to-end coverage and deterministic pagination with repeated terms alpha beta gamma delta epsilon zeta eta theta.`
    ).join("\n"),
    "utf8"
  );
  const epubFileName = `tauri-e2e-source-${uniqueRunId}.epub`;
  const epubPath = path.resolve(tmpDir, epubFileName);
  const epubMarker = `epub-cache-marker-${uniqueRunId}`;
  createEpubFixture(epubPath, epubMarker);
  const pdfFileName = `tauri-e2e-source-${uniqueRunId}.pdf`;
  const pdfPath = path.resolve(tmpDir, pdfFileName);
  const pdfMarker = `pdf-cache-marker-${uniqueRunId}`;
  writeFileSync(
    pdfPath,
    [
      "%PDF-1.4",
      "% synthetic cached fixture",
      "1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj",
      "2 0 obj << /Type /Pages /Count 1 /Kids [3 0 R] >> endobj",
      "3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 300 300] >> endobj",
      "xref",
      "0 4",
      "0000000000 65535 f ",
      "0000000010 00000 n ",
      "0000000063 00000 n ",
      "0000000125 00000 n ",
      "trailer << /Size 4 /Root 1 0 R >>",
      "startxref",
      "186",
      "%%EOF"
    ].join("\n"),
    "utf8"
  );
  seedPdfCacheForSource(
    pdfPath,
    `${pdfMarker} first sentence. ${pdfMarker} second sentence. ${pdfMarker} third sentence.`
  );
  const calibreSourceFileName = `tauri-e2e-calibre-source-${uniqueRunId}.txt`;
  const calibreSourcePath = path.resolve(tmpDir, calibreSourceFileName);
  const calibreMarker = `calibre-runtime-marker-${uniqueRunId}`;
  writeFileSync(
    calibreSourcePath,
    `${calibreMarker} first sentence. ${calibreMarker} second sentence. ${calibreMarker} third sentence.`,
    "utf8"
  );
  const calibreConfigPath = path.resolve(tmpDir, `tauri-e2e-calibre-${uniqueRunId}.toml`);
  const calibreCachePaths = [path.join(runtimeCacheRoot, "calibre-books.toml")];
  const calibreTargetBookId = 910_001;
  const calibreTargetTitle = `Runtime Calibre Target ${uniqueRunId}`;
  const calibreConfig = {
    enabled: true,
    library_url: "",
    state_path: "",
    calibredb_bin: "calibredb",
    server_urls: ["http://0.0.0.0:1"],
    server_username: "",
    server_password: "",
    content_username: "",
    content_password: "",
    allow_local_library_fallback: false,
    allowed_extensions: ["epub", "pdf", "md", "txt"],
    columns: ["title", "extension", "author", "year", "size"],
    list_cache_ttl_secs: 3600
  };
  writeCalibreConfigFixture(calibreConfigPath, calibreConfig);
  process.env.CALIBRE_CONFIG_PATH = calibreConfigPath;
  const calibreSignature = buildCalibreCacheSignature(calibreConfig);
  const calibreBooks = [];
  for (let idx = 0; idx < 1_500; idx += 1) {
    const id = idx + 1;
    calibreBooks.push({
      id,
      title: `Runtime Calibre Book ${String(id).padStart(5, "0")}`,
      extension: "txt",
      authors: `Author ${String((id % 337) + 1).padStart(3, "0")}`,
      year: 1900 + (id % 120),
      file_size_bytes: 1024 + idx
    });
  }
  calibreBooks.push({
    id: calibreTargetBookId,
    title: calibreTargetTitle,
    extension: "txt",
    authors: "Runtime Author",
    year: 2024,
    file_size_bytes: 4096,
    path: calibreSourcePath
  });
  writeCalibreCacheFixture(calibreCachePaths, calibreSignature, calibreBooks);

  runOrThrow("pnpm", ["tauri", "build", "--debug", "--no-bundle"], repoRoot);
  const application = resolveTauriBinary();

  process.env.TAURI_WEBVIEW_AUTOMATION = "true";
  process.env.GDK_BACKEND = "x11";
  process.env.WEBKIT_DISABLE_DMABUF_RENDERER = "1";
  delete process.env.WAYLAND_DISPLAY;
  const tauriDriver = spawn(tauriDriverPath, [], {
    stdio: [null, process.stdout, process.stderr]
  });

  let driver;
  t.after(async () => {
    try {
      if (driver) {
        await driver.quit();
      }
    } finally {
      tauriDriver.kill();
      if (previousCacheDirEnv === undefined) {
        delete process.env.LANTERNLEAF_CACHE_DIR;
      } else {
        process.env.LANTERNLEAF_CACHE_DIR = previousCacheDirEnv;
      }
      if (previousCalibreConfigPathEnv === undefined) {
        delete process.env.CALIBRE_CONFIG_PATH;
      } else {
        process.env.CALIBRE_CONFIG_PATH = previousCalibreConfigPathEnv;
      }
      rmSync(runtimeCacheRoot, { recursive: true, force: true });
      rmSync(calibreConfigPath, { force: true });
      rmSync(calibreSourcePath, { force: true });
    }
  });

  driver = await createDriver(application);

  await waitForElement(driver, By.xpath("//h1[normalize-space()='Welcome']"), 30000);

  const pathInput = await waitForElement(
    driver,
    By.css("[data-testid='starter-open-path-input']")
  );
  await pathInput.clear();
  await pathInput.sendKeys(sourcePath);

  const openButton = await waitForElement(
    driver,
    By.css("[data-testid='starter-open-path-button']")
  );
  await openButton.click();

  const closeSessionButton = await waitForElement(
    driver,
    By.css("[data-testid='reader-close-session-button']"),
    30000
  );
  assert.ok(await closeSessionButton.isDisplayed(), "reader session should open");
  await waitForMarkerAttribute(driver, "app-session-mode", "data-mode", (value) => value === "reader");
  await waitForMarkerAttribute(
    driver,
    "app-last-source-open-event",
    "data-phase",
    (value) => value === "finished"
  );
  await waitForMarkerAttribute(
    driver,
    "app-last-source-open-event",
    "data-source-path",
    (value) => value.endsWith(sourceFileName)
  );
  await maybeSetWindowRect(driver, 1680, 980);

  const textModeToggle = await waitForElement(
    driver,
    By.css("[data-testid='reader-toggle-text-mode-button']")
  );
  const textModeBefore = await textModeToggle.getText();
  assert.equal(textModeBefore.toLowerCase(), "text-only");
  await textModeToggle.click();
  assert.equal((await textModeToggle.getText()).toLowerCase(), "pretty text");
  await textModeToggle.click();
  assert.equal((await textModeToggle.getText()).toLowerCase(), "text-only");

  const statsToggle = await waitForElement(driver, By.css("[data-testid='reader-toggle-stats-button']"));
  await statsToggle.click();
  const panelTitleLocator = By.css("[data-testid='reader-panel-title']");
  await waitForElement(driver, panelTitleLocator);
  await waitForText(driver, panelTitleLocator, "Stats");

  const settingsToggle = await waitForElement(
    driver,
    By.css("[data-testid='reader-toggle-settings-button']")
  );
  await settingsToggle.click();
  await waitForText(driver, panelTitleLocator, "Settings");

  const targetSentenceIdx = await pickAvailableSentenceIndex(driver, 14);
  const targetSentence = await waitForElement(
    driver,
    By.css(`[data-testid='reader-sentence-${targetSentenceIdx}']`)
  );
  await targetSentence.click();
  await assertHighlightedSentenceVisible(driver);

  const searchInput = await waitForElement(driver, By.css("input[data-reader-search-input='1']"));
  await searchInput.click();
  await searchInput.sendKeys(Key.chord(Key.CONTROL, "a"));
  await searchInput.sendKeys("theta");
  const searchApplyButton = await waitForElement(
    driver,
    By.css("[data-testid='reader-search-apply-button']")
  );
  await searchApplyButton.click();
  const searchStartIdx = await waitForHighlightedSentenceIndex(driver, () => true);
  const searchNextButton = await waitForElement(
    driver,
    By.css("[data-testid='reader-search-next-button']")
  );
  await searchNextButton.click();
  const searchNextIdx = await waitForHighlightedSentenceIndex(
    driver,
    (idx) => idx !== searchStartIdx
  );
  const searchPrevButton = await waitForElement(
    driver,
    By.css("[data-testid='reader-search-prev-button']")
  );
  await searchPrevButton.click();
  const searchPrevIdx = await waitForHighlightedSentenceIndex(driver, () => true);
  assert.equal(
    searchPrevIdx,
    searchStartIdx,
    `search prev should return to starting match (start ${searchStartIdx}, next ${searchNextIdx}, prev ${searchPrevIdx})`
  );

  await setNumericSetting(driver, "setting-font-size", 30);
  await assertHighlightedSentenceVisible(driver);

  await setNumericSetting(driver, "setting-horizontal-margin", 180);
  await assertHighlightedSentenceVisible(driver);

  await setNumericSetting(driver, "setting-line-spacing", 1.8);
  await assertHighlightedSentenceVisible(driver);
  await setNumericSetting(driver, "setting-lines-per-page", 10);
  await assertHighlightedSentenceVisible(driver);

  const ttsPanelToggle = await waitForElement(
    driver,
    By.css("[data-testid='reader-toggle-tts-panel-button']")
  );
  await ttsPanelToggle.click();
  try {
    await waitForText(driver, panelTitleLocator, "TTS Options", 3000);
  } catch {
    // If settings and TTS are both toggled, hide settings then re-toggle TTS.
    await settingsToggle.click();
    await ttsPanelToggle.click();
    await waitForText(driver, panelTitleLocator, "TTS Options", 6000);
  }

  const ttsProgressLabel = await waitForElement(
    driver,
    By.css("[data-testid='reader-tts-progress-label']")
  );
  assert.match(
    await ttsProgressLabel.getText(),
    /^Progress:\s*\d+\.\d{3}%$/,
    "TTS progress should render with 3 decimal places"
  );

  const pauseButton = await findVisibleElement(
    driver,
    By.css("[data-testid='reader-tts-pause-button']")
  );
  const playButton = await findVisibleElement(driver, By.css("[data-testid='reader-tts-play-button']"));
  if (playButton && pauseButton) {
    await playButton.click();
    await waitForPlaybackState(driver, "playing");
    await pauseButton.click();
    await waitForPlaybackState(driver, "paused");
  }

  const playPageButton = await findVisibleElement(
    driver,
    By.css("[data-testid='reader-tts-play-page-button']")
  );
  if (playPageButton && pauseButton) {
    await playPageButton.click();
    await waitForPlaybackState(driver, "playing");
    await pauseButton.click();
    await waitForPlaybackState(driver, "paused");
  }

  const playHighlightButton = await findVisibleElement(
    driver,
    By.css("[data-testid='reader-tts-play-highlight-button']")
  );
  if (playHighlightButton && pauseButton) {
    await playHighlightButton.click();
    await waitForPlaybackState(driver, "playing");
    await pauseButton.click();
    await waitForPlaybackState(driver, "paused");
  }

  const ttsSeekPrevButton = await findVisibleElement(
    driver,
    By.css("[data-testid='reader-tts-prev-sentence-button']")
  );
  const ttsSeekNextButton = await findVisibleElement(
    driver,
    By.css("[data-testid='reader-tts-next-sentence-button']")
  );
  const ttsRepeatButton = await findVisibleElement(
    driver,
    By.css("[data-testid='reader-tts-repeat-button']")
  );
  if (ttsSeekNextButton) {
    const ttsSeekNextDisabled = await ttsSeekNextButton.getAttribute("disabled");
    if (ttsSeekNextDisabled === null) {
      await ttsSeekNextButton.click();
      await waitForPlaybackState(driver, "paused");
    }
  }
  if (ttsSeekPrevButton) {
    const ttsSeekPrevDisabled = await ttsSeekPrevButton.getAttribute("disabled");
    if (ttsSeekPrevDisabled === null) {
      await ttsSeekPrevButton.click();
      await waitForPlaybackState(driver, "paused");
    }
  }
  if (ttsRepeatButton) {
    const ttsRepeatDisabled = await ttsRepeatButton.getAttribute("disabled");
    if (ttsRepeatDisabled === null) {
      await ttsRepeatButton.click();
      await waitForPlaybackState(driver, "paused");
    }
  }

  await maybeSetWindowRect(driver, 1080, 860);
  await assertElementHeightAtMost(driver, "reader-topbar", 64);
  await assertElementHeightAtMost(driver, "reader-tts-control-row", 56);

  const ttsSummary = await waitForElement(driver, By.css("[data-testid='reader-tts-state-summary']"));
  const prevSentenceButton = await waitForElement(
    driver,
    By.css("[data-testid='reader-prev-sentence-button']")
  );
  const nextSentenceButton = await waitForElement(
    driver,
    By.css("[data-testid='reader-next-sentence-button']")
  );
  for (let attempt = 0; attempt < 6; attempt += 1) {
    const currentPosition = parseSentencePosition(await ttsSummary.getText());
    if (currentPosition.current <= 1) {
      break;
    }
    await prevSentenceButton.click();
    await delay(120);
  }
  const beforePosition = parseSentencePosition(await ttsSummary.getText());
  if (beforePosition.current < beforePosition.total) {
    await nextSentenceButton.click();
    const afterPosition = parseSentencePosition(await ttsSummary.getText());
    assert.ok(
      afterPosition.current > beforePosition.current,
      `expected sentence position to move forward (before ${beforePosition.current}, after ${afterPosition.current})`
    );
    assert.ok(afterPosition.total >= beforePosition.total, "sentence total should remain stable");
  } else {
    const nextDisabled = await nextSentenceButton.getAttribute("disabled");
    assert.notEqual(
      nextDisabled,
      null,
      "next sentence should be disabled when no forward sentence is available"
    );
  }

  const ttsToggle = await waitForElement(driver, By.css("[data-testid='reader-tts-toggle-button']"));
  const beforeText = await ttsToggle.getText();
  await ttsToggle.click();
  await waitForPlaybackState(driver, "playing");
  const afterPlayText = await ttsToggle.getText();
  assert.notEqual(afterPlayText, beforeText, "playback button label should toggle");
  await ttsToggle.click();
  await waitForPlaybackState(driver, "paused");
  const afterPauseText = await ttsToggle.getText();
  assert.equal(afterPauseText, beforeText, "playback button label should return to initial state");

  const nextPageButton = await waitForElement(driver, By.css("[data-testid='reader-next-page-button']"));
  const nextPageDisabled = await nextPageButton.getAttribute("disabled");
  assert.equal(nextPageDisabled, null, "next page should be enabled for multi-page fixture");
  await nextPageButton.click();
  await waitForPlaybackState(driver, "paused");

  await nextSentenceButton.click();
  await waitForPlaybackState(driver, "paused");

  await closeSessionButton.click();
  await waitForElement(driver, By.xpath("//h1[normalize-space()='Welcome']"), 20000);
  await waitForMarkerAttribute(
    driver,
    "app-session-mode",
    "data-mode",
    (value) => value === "starter"
  );

  const sourcePathAttr = xpathLiteral(sourcePath);
  const recentCardLocator = By.xpath(
    `//*[@data-testid='starter-recent-card' and @data-recent-path=${sourcePathAttr}]`
  );
  const recentCard = await waitForElement(driver, recentCardLocator, 20000);
  const recentDelete = await recentCard.findElement(
    By.css("[data-testid='starter-recent-delete-button']")
  );
  await recentDelete.click();
  await waitForNoElement(driver, recentCardLocator, 15000);

  const clipboardMarker = `clipboard-tauri-runtime-${uniqueRunId}`;
  const clipboardText = `${clipboardMarker} first sentence. ${clipboardMarker} second sentence.`;
  await driver.executeScript(
    (text) => {
      Object.defineProperty(window.navigator, "clipboard", {
        configurable: true,
        value: {
          readText: async () => text
        }
      });
    },
    clipboardText
  );
  const openClipboardButton = await waitForElement(
    driver,
    By.css("[data-testid='starter-open-clipboard-button']")
  );
  await openClipboardButton.click();
  await waitForElement(driver, By.css("[data-testid='reader-close-session-button']"), 30000);
  const clipboardMarkerLiteral = xpathLiteral(clipboardMarker);
  await waitForElement(
    driver,
    By.xpath(
      `//button[starts-with(@data-testid,'reader-sentence-') and contains(normalize-space(), ${clipboardMarkerLiteral})]`
    ),
    20000
  );

  const closeSessionAfterClipboard = await waitForElement(
    driver,
    By.css("[data-testid='reader-close-session-button']")
  );
  await closeSessionAfterClipboard.click();
  await waitForElement(driver, By.xpath("//h1[normalize-space()='Welcome']"), 20000);

  const pathInputAfterClipboard = await waitForElement(
    driver,
    By.css("[data-testid='starter-open-path-input']")
  );
  await pathInputAfterClipboard.clear();
  await pathInputAfterClipboard.sendKeys(epubPath);
  const openButtonAfterClipboard = await waitForElement(
    driver,
    By.css("[data-testid='starter-open-path-button']")
  );
  await openButtonAfterClipboard.click();
  const epubPhase = await waitForMarkerAttribute(
    driver,
    "app-last-source-open-event",
    "data-phase",
    (value) => value === "finished" || value === "failed" || value === "cancelled",
    30000
  );
  if (epubPhase !== "finished") {
    const openMessage = await waitForMarkerAttribute(
      driver,
      "app-last-source-open-event",
      "data-message",
      () => true,
      5000
    );
    throw new Error(`EPUB open did not finish successfully (phase=${epubPhase}): ${openMessage}`);
  }
  await waitForElement(driver, By.css("[data-testid='reader-close-session-button']"), 30000);
  await waitForMarkerAttribute(
    driver,
    "app-last-source-open-event",
    "data-phase",
    (value) => value === "finished"
  );
  await waitForMarkerAttribute(
    driver,
    "app-last-source-open-event",
    "data-source-path",
    (value) => value.endsWith(epubFileName)
  );
  const epubMarkerLiteral = xpathLiteral(epubMarker);
  await waitForElement(
    driver,
    By.xpath(
      `//button[starts-with(@data-testid,'reader-sentence-') and contains(normalize-space(), ${epubMarkerLiteral})]`
    ),
    20000
  );
  const closeSessionAfterEpub = await waitForElement(
    driver,
    By.css("[data-testid='reader-close-session-button']")
  );
  await closeSessionAfterEpub.click();
  await waitForElement(driver, By.xpath("//h1[normalize-space()='Welcome']"), 20000);

  const pathInputAfterEpub = await waitForElement(
    driver,
    By.css("[data-testid='starter-open-path-input']")
  );
  await pathInputAfterEpub.clear();
  await pathInputAfterEpub.sendKeys(pdfPath);
  const openButtonAfterEpub = await waitForElement(
    driver,
    By.css("[data-testid='starter-open-path-button']")
  );
  await openButtonAfterEpub.click();
  const pdfSourcePhase = await waitForMarkerAttribute(
    driver,
    "app-last-source-open-event",
    "data-phase",
    (value) => value === "finished" || value === "failed" || value === "cancelled",
    30000
  );
  await waitForMarkerAttribute(
    driver,
    "app-last-source-open-event",
    "data-source-path",
    (value) => value.endsWith(pdfFileName)
  );
  const pdfEventPhase = await waitForMarkerAttribute(
    driver,
    "app-last-pdf-event",
    "data-phase",
    (value) => value === "finished" || value === "failed" || value === "cancelled"
  );
  await waitForMarkerAttribute(
    driver,
    "app-last-pdf-event",
    "data-source-path",
    (value) => value.endsWith(pdfFileName)
  );
  if (pdfSourcePhase !== "finished" || pdfEventPhase !== "finished") {
    const sourceMessage = await waitForMarkerAttribute(
      driver,
      "app-last-source-open-event",
      "data-message",
      () => true,
      5000
    );
    const pdfMessage = await waitForMarkerAttribute(
      driver,
      "app-last-pdf-event",
      "data-message",
      () => true,
      5000
    );
    throw new Error(
      `PDF open should finish successfully (source=${pdfSourcePhase}, pdf=${pdfEventPhase}) source_message=${sourceMessage} pdf_message=${pdfMessage}`
    );
  }
  await waitForElement(driver, By.css("[data-testid='reader-close-session-button']"), 30000);
  const pdfMarkerLiteral = xpathLiteral(pdfMarker);
  await waitForElement(
    driver,
    By.xpath(
      `//button[starts-with(@data-testid,'reader-sentence-') and contains(normalize-space(), ${pdfMarkerLiteral})]`
    ),
    20000
  );
  const closeSessionAfterPdf = await waitForElement(
    driver,
    By.css("[data-testid='reader-close-session-button']")
  );
  await closeSessionAfterPdf.click();
  await waitForElement(driver, By.xpath("//h1[normalize-space()='Welcome']"), 20000);

  const calibreLoadButton = await waitForElement(
    driver,
    By.css("[data-testid='starter-calibre-load-button']")
  );
  await calibreLoadButton.click();
  const calibrePhase = await waitForMarkerAttribute(
    driver,
    "app-last-calibre-event",
    "data-phase",
    (value) => value === "finished" || value === "failed",
    30000
  );
  if (calibrePhase !== "finished") {
    const calibreMessage = await waitForMarkerAttribute(
      driver,
      "app-last-calibre-event",
      "data-message",
      () => true,
      5000
    );
    throw new Error(`Calibre load should finish successfully: ${calibreMessage}`);
  }
  const calibreCount = Number(
    await waitForMarkerAttribute(driver, "app-last-calibre-event", "data-count", () => true, 5000)
  );
  assert.ok(
    Number.isFinite(calibreCount) && calibreCount >= 1500,
    `expected calibre load count >= 1500, got ${calibreCount}`
  );

  const calibreSearchInput = await waitForElement(
    driver,
    By.css("input[data-testid='starter-calibre-search-input']")
  );
  await calibreSearchInput.click();
  await calibreSearchInput.sendKeys(Key.chord(Key.CONTROL, "a"));
  await calibreSearchInput.sendKeys(`Target ${uniqueRunId}`);

  const calibreOpenButton = await waitForElement(
    driver,
    By.css(`[data-testid='starter-calibre-open-button'][data-book-id='${calibreTargetBookId}']`),
    20000
  );
  await calibreOpenButton.click();
  await waitForElement(driver, By.css("[data-testid='reader-close-session-button']"), 30000);
  await waitForMarkerAttribute(
    driver,
    "app-last-source-open-event",
    "data-phase",
    (value) => value === "finished"
  );
  await waitForMarkerAttribute(
    driver,
    "app-last-source-open-event",
    "data-source-path",
    (value) => value.endsWith(calibreSourceFileName)
  );
  const calibreMarkerLiteral = xpathLiteral(calibreMarker);
  await waitForElement(
    driver,
    By.xpath(
      `//button[starts-with(@data-testid,'reader-sentence-') and contains(normalize-space(), ${calibreMarkerLiteral})]`
    ),
    20000
  );
  const closeSessionAfterCalibre = await waitForElement(
    driver,
    By.css("[data-testid='reader-close-session-button']")
  );
  await closeSessionAfterCalibre.click();
  await waitForElement(driver, By.xpath("//h1[normalize-space()='Welcome']"), 20000);
});
