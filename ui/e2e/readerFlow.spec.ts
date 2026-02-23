import { expect, test } from "@playwright/test";

test.describe("reader flow", () => {
  test("opens a source and exercises core reader/tts controls", async ({ page }) => {
    await page.goto("/");

    await page.getByTestId("starter-open-path-input").fill("/tmp/mock.epub");
    await page.getByTestId("starter-open-path-button").click();

    await expect(page.getByTestId("reader-close-session-button")).toBeVisible();

    // Hide settings so the TTS panel can be displayed.
    await page.getByRole("button", { name: "Settings" }).click();
    await expect(page.getByTestId("reader-tts-toggle-button")).toBeVisible();

    const ttsToggle = page.getByTestId("reader-tts-toggle-button");
    await expect(ttsToggle).toHaveText("Play");
    await ttsToggle.click();
    await expect(ttsToggle).toHaveText("Pause");
    await ttsToggle.click();
    await expect(ttsToggle).toHaveText("Play");

    await page.getByTestId("reader-close-session-button").click();
    await expect(page.getByRole("heading", { name: "Welcome" })).toBeVisible();
  });
});
