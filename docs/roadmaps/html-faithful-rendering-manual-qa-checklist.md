# HTML Faithful Rendering Manual QA Checklist

## Rendering Fidelity
- [ ] Open a representative `.html` source with headings, paragraphs, lists, tables, figures, captions, and links.
- [ ] Confirm Pretty Text mode renders the source as native HTML rather than markdown-like fallback text.
- [ ] Confirm readable structure is preserved:
- [ ] headings
- [ ] paragraphs
- [ ] lists
- [ ] tables
- [ ] figures/captions
- [ ] inline emphasis
- [ ] Confirm source CSS is respected where safe and does not distort the surrounding app UI.

## Text-only and TTS Ownership
- [ ] Switch to Text-only mode and confirm only clean extracted text is shown.
- [ ] Confirm Text-only mode does not show raw HTML tags, CSS, script text, or layout boilerplate.
- [ ] Start TTS from Text-only mode and confirm playback follows the Text-only sentence order.
- [ ] Toggle between Pretty Text and Text-only during playback and confirm the spoken position remains stable.

## Link and Asset Behavior
- [ ] Confirm internal anchor links scroll to the correct in-document location.
- [ ] Confirm external links open safely without breaking the reader state.
- [ ] Confirm inline images load correctly in Pretty Text mode.
- [ ] Confirm missing or broken images fail gracefully without collapsing the whole reader view.

## Sync and Highlight Behavior
- [ ] Start TTS in Text-only mode, switch to Pretty Text mode, and confirm the current spoken unit is highlighted.
- [ ] Confirm highlight remains visible throughout playback and does not disappear unexpectedly.
- [ ] Confirm highlight/scroll does not jump backward or several sections ahead.
- [ ] Confirm auto-scroll only repositions when playback moves to a new mapped HTML anchor or page.
- [ ] Confirm `Jump to Highlight` lands at the current mapped location.

## Resume, Cache, and Recovery
- [ ] Close and reopen the same HTML source and confirm the reader resumes at the previous position.
- [ ] Confirm cached Pretty Text and Text-only artifacts reopen without corruption.
- [ ] Delete the recent item and confirm cache artifacts are removed cleanly.
- [ ] Reopen the source after delete and confirm artifacts rebuild successfully.

## Stress Cases
- [ ] Test an HTML document with a table of contents and repeated headings.
- [ ] Test an HTML document with heavy images and long captions.
- [ ] Test an HTML document with footnotes/internal references.
- [ ] Test a long HTML document and confirm playback sync remains stable over time.
