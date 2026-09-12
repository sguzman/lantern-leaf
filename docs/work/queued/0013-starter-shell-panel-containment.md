# 0013 — Starter shell responsive panel containment

## Outcome

Repair the starter/library shell so Recents, Calibre, Browser Tabs, diagnostics, and adjacent shell regions never visually overlap or bleed across their allocated columns/panels at ordinary desktop widths.

## Starting evidence

During Goal 0012 physical Windows QA, the starter shell visibly rendered the Recents group and Browser Tabs group across one another. This is independent of the reader-presentation/TTS correction and should not prolong Goal 0012 beyond its own reader acceptance surface.

Current starter layout uses a two-column `ui.columns(2)` mode and several unwrapped child rows/groups whose minimum width can exceed the allocated column. Long recent titles/paths and Browser Tabs controls can therefore enlarge child geometry beyond the intended column boundary.

## Scope

When promoted:

- reproduce the observed overlap at representative physical window/central-panel widths;
- define explicit responsive column containment based on the actual center width after side panels;
- ensure child groups cannot expand across adjacent column rects;
- use wrapping/vertical stacking for long title/path/control rows;
- choose a conservative single-column fallback when two readable columns do not fit;
- preserve bounded/virtualized Calibre/Recents lists;
- keep Browser Tabs and diagnostics reachable;
- no horizontal application-level overflow caused by one long path/title/control row;
- add deterministic layout-policy tests/seams for representative widths around the column breakpoint.

## Non-goals

Do not mix this with Goal 0012 TTS/highlight/presentation geometry, Caliberate catalog-cover loading, Natural voices, or PDF work.

## Priority

Queued behind the currently reopened Goal 0012 A6 reader-presentation correction. Re-evaluate against Goal 0010 after Goal 0012 closes.
