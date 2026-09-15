# LanternLeaf QA Evidence Ledger

This ledger records the physical observations that materially changed project truth. Automated tests belong in goal reports; this file exists for **human-visible runtime evidence** that shaped architecture or acceptance.

## Windows / native baseline

The restart established Windows as a real product platform. Build success alone was not considered sufficient; local launch, audible TTS, and interactive native behavior became acceptance gates.

## Ordinary Windows TTS

Physically accepted behavior from the early TTS gates:

- speech is audible through the ordinary Windows backend;
- Play/Pause works;
- ordinary installed voice switching works;
- the actually audible sentence drives canonical cursor/highlight state;
- Zira works as the portable ordinary default;
- per-book voice persistence works;
- invalid Piper transitions can be rejected without poisoning the session;
- Close Book and Safe Quit work.

This is the baseline. Windows Natural/HD voices are not part of it and remain deferred.

## Native EPUB reader responsiveness

An early native EPUB presentation build was physically unusable despite being egui-based:

- hovering buttons could take roughly 2–3 seconds to register;
- the reader felt catastrophically laggy;
- attempting playback during that period contributed to instability/crash concern.

A later correction made the view substantially snappier. This incident is why "native" is never accepted as synonymous with "responsive".

## Goal 0012 — pretty presentation controls

Physical QA verified the repaired presentation system, including:

- hidden-width/gutter correction;
- settings panel usability;
- font/word/letter/line/paragraph/heading spacing effects;
- tables/TOCs/blockquotes becoming readable;
- inline imagery;
- media max width/height behavior;
- TTS highlight continuing to function after virtualization/presentation changes.

## Goal 0014 — reflow anchor stability

Physical QA confirmed the severe violent-jump failure was gone.

Residual behavior under extreme cumulative text-metric edits was downgraded to minor polish rather than keeping the goal open.

## Goal 0010 / Caliberate covers

Real desktop evidence:

- covers appear in the starter before opening books;
- scrolling causes lazy cover population;
- EPUB opens remain very fast;
- TTS/settings continue to work;
- cover retrieval does not require EPUB materialization.

## Goal 0016 — real library continuity

Real Caliberate library size: roughly 105,570 books.

Physical observations:

- catalog becomes usable progressively rather than waiting for the full library;
- loaded/total state is visible;
- Recents refresh in the same session;
- durable Recents survive restart;
- a first open after QA state reset may take about 5–7 seconds;
- warm immediate reopen remains fast.

The 5–7 second case was accepted as cold-cache behavior, not treated as a regression.

## Goal 0019 — native PDF visual baseline

Real Caliberate PDF used for acceptance: book 725, 638 pages.

Physical observations:

- actual native PDF pages render;
- page ownership reports `Page N / 638`;
- Previous/Next work;
- repeated/abusive PDF interaction does not collapse the reader;
- representative EPUB behavior remains intact.

The user explicitly wanted continuous scrolling rather than a one-page-at-a-time reader, which directly motivated Goal 0020.

## Goal 0020 — continuous PDF viewport

Physical acceptance on the 638-page PDF:

- continuous wheel scrolling works;
- portions of adjacent pages are visible at page seams;
- violent/fast scrolling is extremely responsive;
- page count/current-page ownership keeps up;
- Previous/Next behave well as continuous-stack jumps;
- Fit Width works;
- Fit Page works;
- Reset/100% works;
- manual zoom works from 25% through 400%;
- horizontal access at high zoom works;
- zoom approximately preserves reading position;
- EPUB TTS and visual controls still work.

Residual observed defect:

After Fit Width/Fit Page, +/- remembers the old manual zoom rather than stepping from the current effective fit value. Queued as Goal 0021; not severe enough to reject Goal 0020.

## Goal 0022 A7 — first physical native PDF embedded-text/TTS QA

A7 was the first Goal 0022 attempt allowed onto the real desktop after six source-level rejections.

### What remained good

On the real 638-page-class PDF path:

- violent scrolling remained excellent;
- dragging the scrollbar remained responsive;
- transient `Rendering page N` text appeared but disappeared quickly;
- this placeholder behavior was judged minor/not problematic;
- Text-only worked;
- Text-only appeared to correspond sensibly to native PDF pages;
- Previous/Next mostly worked;
- visual PDF -> Text-only -> visual PDF switching kept page identity sane;
- PDF close/reopen was very snappy;
- EPUB initially still opened and basic features worked.

This evidence is crucial: Goal 0022's new text machinery did **not** destroy the accepted Goal 0020 visual foundation.

### PDF TTS physical failures

Observed:

- starting TTS near the beginning/title could read the title and then die;
- TTS appeared to read only the current native page in some locations;
- later in the book it sometimes crossed a page boundary successfully;
- therefore page continuation was inconsistent rather than universally absent;
- pressing Next sometimes repeated instead of advancing;
- page-number state could flicker rapidly around the failed transition.

Interpretation later confirmed in source review:

The runtime could know more canonical text existed while rebuilding a local bounded plan without first moving authoritative ReaderSession page ownership to the next non-empty native page.

### PDF visual highlight/follow

No native PDF pretty-surface spoken highlight or scroll synchronization was visible.

This was **expected**, not a regression. Exact native sentence geometry was deliberately out of Goal 0022 scope.

### Search physical failure

Opening Search caused the panel to appear and immediately disappear, making it impossible to type.

This led to separation of:

- persistent Search panel visibility;
- one-shot focus acquisition.

Subsequent source review of A8 found that even this split was insufficient unless actual editor focus also controls shortcut suppression and the visible text draft is synchronous.

### EPUB Text-only stress failure

The most severe A7 physical regression:

- repeatedly toggling Text-only/Pretty on an otherwise healthy EPUB eventually broke presentation;
- only a small fragment / a couple paragraphs remained rendered;
- large parts of the book appeared missing.

This was traced to relative asynchronous `ToggleTextOnly` commands racing with stale snapshots/optimistic UI state.

Architectural correction:

`SetTextOnly { enabled: bool }`

with latest-target authority and same-target no-op semantics.

## QA infrastructure observation

Repeated `qa.ps1` runs inside one PowerShell process can eventually make Visual Studio environment setup fail with:

`The input line is too long.`

A fresh PowerShell works.

Until Goal 0018 is complete, physical QA should be started from a fresh PowerShell.

## Cache/materialization observations

For PDFs, fast reopen does not imply durable raster files should appear in the filesystem cache.

Observed/accepted distinction:

- Caliberate may materialize source PDFs under LanternLeaf's QA cache hierarchy;
- direct filesystem PDFs need not be duplicated;
- native raster textures are bounded/ephemeral in memory;
- native text cache is a different artifact class.

## How to use this ledger

When machine evidence and this ledger disagree:

1. determine whether the claim is visual/audio/interaction-sensitive;
2. reproduce on current main if necessary;
3. do not erase prior physical evidence merely because a synthetic test passes;
4. if behavior is intentionally changed, record the new physical evidence and why the old expectation was superseded.
