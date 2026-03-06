# Browser Tabs Import Manual QA Checklist

- [ ] Confirm `browsr` health shows online and extension connected in the starter screen.
- [ ] Confirm multiple browser windows appear in the `Browser Tabs` section when available.
- [ ] Confirm window filtering narrows the visible tab list correctly.
- [ ] Confirm tab search filters by title and URL without affecting other starter sections.
- [ ] Import a tab with readable article HTML and verify:
- [ ] Pretty Text renders the captured HTML.
- [ ] Text-only renders the captured plain text.
- [ ] Reader title uses the tab title rather than `browser-tab.lltab`.
- [ ] Relative links open against the original page origin.
- [ ] Relative images load against the original page origin.
- [ ] Start TTS in Text-only, switch to Pretty Text, and verify highlight/scroll stay aligned.
- [ ] Reopen the imported tab from Recents and verify cached snapshot reopen works with the original title.
- [ ] Use `Refresh Tab` from the reader quick actions on an imported tab and verify the refreshed snapshot opens cleanly.
- [ ] Verify browser-tab refresh preserves the same recent entry and does not create duplicate browser-tab entries for the same live tab.
- [ ] Delete an imported browser-tab recent entry and verify all cached browser-tab artifacts are removed.
- [ ] Stop `browsr` or disconnect the extension and verify the starter UI shows a clear failure state rather than hanging.
