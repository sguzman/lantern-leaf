import { expect, test } from "@playwright/test";

test.describe("perf baseline", () => {
  test("captures baseline timings for startup, open, page switch, tts, and resize", async ({ page }) => {
    const startupStart = Date.now();
    await page.goto("/");
    await expect(page.getByRole("heading", { name: "Welcome" })).toBeVisible();
    const startupMs = Date.now() - startupStart;

    const openStart = Date.now();
    await page.getByTestId("starter-open-path-input").fill("/tmp/mock.epub");
    await page.getByTestId("starter-open-path-button").click();
    await expect(page.getByTestId("reader-close-session-button")).toBeVisible();
    const openMs = Date.now() - openStart;

    const pageSwitchStart = Date.now();
    await page.getByRole("button", { name: "Next Page" }).click();
    await expect(page.getByLabel("Page")).toHaveValue("2");
    const pageSwitchMs = Date.now() - pageSwitchStart;

    await page.getByRole("button", { name: "Settings" }).click();
    const ttsStart = Date.now();
    const ttsToggle = page.getByTestId("reader-tts-toggle-button");
    await ttsToggle.click();
    await expect(ttsToggle).toHaveText("Pause");
    const ttsStartMs = Date.now() - ttsStart;

    const resizeStart = Date.now();
    await page.setViewportSize({ width: 1180, height: 860 });
    await expect(page.getByTestId("reader-close-session-button")).toBeVisible();
    const resizeMs = Date.now() - resizeStart;

    console.info(
      `[perf-baseline] startup_ms=${startupMs} open_ms=${openMs} page_switch_ms=${pageSwitchMs} tts_start_ms=${ttsStartMs} resize_ms=${resizeMs}`
    );

    expect(startupMs).toBeLessThan(5000);
    expect(openMs).toBeLessThan(5000);
    expect(pageSwitchMs).toBeLessThan(2500);
    expect(ttsStartMs).toBeLessThan(2500);
    expect(resizeMs).toBeLessThan(2500);
  });
});
