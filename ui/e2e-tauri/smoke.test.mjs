import assert from "node:assert/strict";
import { spawn, spawnSync } from "node:child_process";
import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import os from "node:os";
import path from "node:path";
import test from "node:test";
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

function resolveTauriBinary() {
  const binaryName = process.platform === "win32" ? "ebup-viewer-tauri.exe" : "ebup-viewer-tauri";
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

test("tauri runner opens source and exercises core reader controls", async (t) => {
  const timeoutMs = 4 * 60 * 1000;
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
});
