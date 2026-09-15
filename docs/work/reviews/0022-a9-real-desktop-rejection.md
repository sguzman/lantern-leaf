# Goal 0022 A9 — real-desktop rejection

## Decision

**REJECTED IN PHYSICAL WINDOWS QA. Goal 0022 remains open.**

A9 materially improved the native PDF text/TTS path, but the real desktop exposed severe interaction/state defects that automated coverage did not capture. Do not treat A9 as physically accepted.

## What remained good

Physical Windows QA preserved important accepted evidence:

- violent PDF scrollbar dragging remains very responsive;
- transient `Rendering page N` placeholders still disappear quickly and are not considered a blocker;
- native PDF pages render quickly;
- trusted PDF Text-only works and appears page-aligned;
- natural PDF TTS **did cross a native page boundary** during this run, so A9 improved the A7 page-continuation failure;
- repeated TTS Next mostly advances correctly;
- rapid PDF visual/Text-only toggling converges correctly;
- PDF close/open remains snappy;
- a newly opened representative EPUB survived aggressive Pretty/Text-only stress without the A7 presentation nuke.

Goal 0019/0020 visual responsiveness therefore remains intact.

## Blocker 1 — TTS speed control has an asynchronous snap-back race and causes severe layout churn

Physical behavior:

- dragging/changing TTS speed violently moves reader UI/layout;
- the displayed slider value can snap back to the old `2.5` value while the user is manipulating it;
- repeated settings/status churn makes ordinary TTS control feel destructive.

Source review confirms the same ownership mistake previously fixed for Search: `render_tts_widget()` reconstructs `tts_speed` and `tts_volume` from the asynchronously acknowledged `ReaderSnapshot` every frame and emits `ApplySettings` on every slider `changed()` frame.

The visible slider state must be synchronously app-owned during interaction, with stale acknowledgement protection and bounded/coalesced commit semantics. Settings/status traffic must not change the central reader viewport geometry.

## Blocker 2 — reader command/status diagnostics are allowed to reflow the product UI

Physical behavior:

- TTS speed manipulation and repeated Next/Previous visibly move the layout;
- the long changing command/status text appears to participate in this instability.

Current UI renders transient command/status diagnostics as ordinary layout content, including a bottom status bar and `Last command` content in the center. Product interaction must not shift the reading viewport merely because a diagnostic string changed length or because many commands were issued.

A10 must provide fixed/bounded diagnostic presentation: fixed-height/clipped/truncated/overlay behavior is acceptable; dynamic product-layout reflow is not.

## Blocker 3 — Search now accepts text but has no usable navigation UX

Physical behavior:

- Search panel now remains open;
- typed text is accepted;
- match count updates;
- pressing Enter does nothing;
- the user cannot tell where the matches are or navigate them from the Search UI.

Source review confirms `render_search_panel()` currently renders the editor, match count, and `Focus search` only. It does not expose `SearchNext`/`SearchPrev`, Enter/Shift+Enter semantics, current match identity, page provenance, or an excerpt.

A10 must make Search operational rather than merely computational.

For native PDF visual mode, exact in-page rectangle highlighting is still out of scope until sentence geometry exists. Search must nevertheless jump native page ownership and present enough selected-match information (`match X/Y`, native page, excerpt) that the user can tell a search action occurred. Text-only may select/follow the actual canonical row.

## Blocker 4 — burst Previous can repeat instead of moving monotonically backward

Physical behavior:

- spamming TTS Next worked well;
- spamming TTS Previous eventually repeated rather than continuing backward.

A10 must add production-shaped burst navigation coverage through the actual TTS runtime/control worker. Each accepted Previous command must move one canonical sentence backward until the true document start; stale first-sample/runtime events must not reassert the sentence being left.

The same invariant should hold for burst Next until document end.

## Blocker 5 — precise PDF click-to-position is absent and the coarse alternative is not discoverable

Clicking rendered PDF text does not reposition TTS. This is expected at the architecture level because Goal 0022 deliberately has no native sentence geometry; exact click-on-text requires the subsequent geometry goal.

However, the product currently gives the user no obvious way to reposition TTS while staying in visual PDF mode. A10 must make a **coarse native-page-level** action explicit and easy to discover, e.g. `Play from current PDF page` / `Start TTS at visible page`, using current viewport/native-page ownership. Do not fake sentence precision.

Exact click-to-sentence, visual spoken highlight, and PDF visual auto-follow remain out of Goal 0022 scope.

## Blocker 6 — previously damaged EPUB presentation can reappear on reopen

Physical behavior:

- the EPUB damaged during the A7 Text-only race reopened in the same visibly truncated/broken presentation state;
- playing TTS and repeatedly moving Next appeared to recover the presentation;
- a different EPUB no longer reproduces the stress-toggle nuke under A9.

This means the A8/A9 desired-state correction stopped the original easy reproduction, but stale/bad presentation state from the earlier failure can still survive or be resurrected through persistence/cache/session restoration.

A10 must diagnose the actual durable/in-memory ownership path instead of assuming the cause. Reopen a session after stress and verify that transient presentation failure cannot be persisted as authoritative document content or restored as a truncated Pretty document. If a stale derived artifact is detected, invalidate/rebuild it safely rather than preserving damage.

## Non-blocking observations / queued separately

### Caliberate materialized PDF identity in Recents

The real materialized PDF source is stored under the QA cache, e.g. `.../calibre-downloads/caliberate/725-13e8b7a0.pdf`. That is expected source materialization, not durable page-raster caching.

But Recents displays the hash-like materialized filename (`725-13e8b7a0`), no useful book title, and no cover. Source review confirms Recents currently infers title from the materialized file stem and independently infers thumbnails instead of preserving Caliberate provider identity.

This is a real UX defect but is catalog/materialization provenance work rather than Goal 0022 text/TTS correctness. It is queued separately as Goal 0024 so it is not lost.

### `Rendering page N`

Still visible during violent PDF movement, but physically judged fast/transient enough not to block.

### PDF Text-only transition latency

Visual -> Text-only remains slightly slower than the reverse transition. Record as measurable polish unless it becomes severe.

### PDF spoken visual highlight / auto-follow

Still absent and still expected. Exact native sentence geometry has not been implemented yet.

## A10 acceptance requirements

A10 must preserve all A9 architecture that survived QA while correcting the six blockers above. No human QA is authorized until A10 passes director source/CI review.
