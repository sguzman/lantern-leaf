import assert from "node:assert/strict";
import { spawn, spawnSync } from "node:child_process";
import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { Builder, By, Capabilities } from "selenium-webdriver";

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
  const sourcePath = path.resolve(tmpDir, "tauri-e2e-source.txt");
  writeFileSync(
    sourcePath,
    "This is a tauri e2e source file. It exists so the real runtime can open it.",
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

  const settingsButton = await waitForElement(
    driver,
    By.xpath("//button[normalize-space()='Settings']")
  );
  await settingsButton.click();

  const ttsToggle = await waitForElement(driver, By.css("[data-testid='reader-tts-toggle-button']"));
  const beforeText = await ttsToggle.getText();
  await ttsToggle.click();
  const afterPlayText = await ttsToggle.getText();
  assert.notEqual(afterPlayText, beforeText, "playback button label should toggle");
  await ttsToggle.click();
  const afterPauseText = await ttsToggle.getText();
  assert.equal(afterPauseText, beforeText, "playback button label should return to initial state");

  await closeSessionButton.click();
  await waitForElement(driver, By.xpath("//h1[normalize-space()='Welcome']"), 20000);
});
