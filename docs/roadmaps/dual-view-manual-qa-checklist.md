# Dual View Manual QA Checklist

- [ ] EPUB opens with Pretty Text markdown rendering and Text-only fallback behavior is not used.
- [ ] HTML opens with Pretty Text markdown rendering and sentence highlight/auto-scroll remains correct.
- [ ] DOCX opens with Pretty Text markdown rendering and TTS plays from canonical Text-only sentence pipeline.
- [ ] PDF (structured) opens with Pretty Text markdown rendering plus stable TTS sentence progression.
- [ ] PDF (scan/raw) opens without markdown and shows non-blocking fallback indicator.
- [ ] Switching Pretty Text <-> Text-only does not reset current TTS highlight/sentence position.
- [ ] Jump to highlighted sentence works in both views.
- [ ] Search next/prev works in both views and highlights remain aligned.
- [ ] Close session and reopen restores bookmark sentence/page as expected.
- [ ] Recent delete succeeds after reading and does not fail on transient cache races.
- [ ] Logs contain markdown availability summary and mapping fallback telemetry under active reading.
