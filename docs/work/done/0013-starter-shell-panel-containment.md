# 0013 — Starter shell responsive panel containment

## Outcome

Repair the starter/library shell so Recents, Calibre, Browser Tabs, diagnostics, and adjacent shell regions never visually overlap or bleed across their allocated columns/panels at ordinary desktop widths.

## Starting evidence

Goal 0012 final physical Windows QA passed the reader-presentation surface, but the starter shell still visibly shows Recents / Calibre / Browser Tabs regions bleeding into neighboring space. The user reports the overlap is substantially less severe than before, but it remains plainly visible in the current desktop layout.

This defect is independent of reader presentation/TTS and must not disturb the newly accepted Goal 0012 behavior.

The current starter layout uses a two-column `ui.columns(2)` mode and several unwrapped child rows/groups whose minimum width can exceed the allocated column. Long recent titles/paths, Calibre rows, and Browser Tabs controls can therefore enlarge child geometry beyond the intended column boundary.

## Authorized work

### A — reproduce and measure the real layout failure

- Reproduce overlap at representative central-panel widths, including the width shown by the final Goal 0012 QA screenshot.
- Identify which child rows/groups advertise or consume minimum widths beyond their allocated column rects.
- Add a narrow deterministic layout-policy seam/test surface rather than relying only on visual inspection.

### B — explicit responsive containment

- Base the starter layout decision on the actual available center width after side panels.
- Keep every child group geometrically contained inside the column/panel it owns.
- Long recent titles, filesystem paths, Calibre metadata, and control rows must wrap, elide, or vertically stack rather than invade adjacent columns.
- Do not introduce application-level horizontal overflow merely to preserve a two-column arrangement.

### C — conservative breakpoint / one-column fallback

- Define a readable two-column minimum width from actual content requirements.
- When two readable columns do not fit, use a single-column stack instead of compressed/overlapping pseudo-columns.
- Avoid oscillation or unstable breakpoint behavior during ordinary window resizing.

### D — preserve bounded shell behavior

- Preserve bounded/virtualized Calibre and Recents list behavior.
- Keep Browser Tabs and diagnostics reachable.
- Keep long-running/network/disk/catalog work off the egui/render thread.
- Do not regress Goal 0008/0009 TTS or Goal 0012 reader presentation/image behavior.

## Acceptance gates

1. At representative ordinary desktop widths, Recents, Calibre, Browser Tabs, diagnostics, and adjacent groups remain inside their assigned column/panel rectangles.
2. Long titles/paths/control rows cannot force geometry across a neighboring column boundary.
3. Below the readable two-column breakpoint, the shell uses a stable one-column layout rather than overlap or unreadable compression.
4. Resizing around the breakpoint does not produce visual bleed, runaway horizontal width, or layout thrash.
5. Calibre/Recents bounded list behavior remains intact.
6. Browser Tabs and diagnostics remain reachable and usable.
7. Deterministic layout-policy tests cover representative widths around the breakpoint and long-content cases.
8. Heavy/blocking work remains off the egui/render thread.
9. Goal 0008/0009 TTS and Goal 0012 presentation/image regressions remain green.
10. Repository validation and Windows CI pass.

## Non-goals

- Goal 0012 reader presentation/TTS/highlight redesign;
- Goal 0014 media max-width/max-height viewport anchoring;
- Caliberate catalog-cover loading/provider UX (Goal 0010);
- Windows Natural/HD voices;
- PDF work;
- broad aesthetic redesign of the starter shell.

## Repository handoff

Use branch `codex/0013-starter-shell-panel-containment`.

Codex should synchronize current director `main`, move this goal `ready -> active`, launch/re-arm the Goal 0013 watcher for this execution attempt, implement the full authorized work, run repository and Windows gates, write `docs/work/reports/0013.md`, terminalize only on full success or a true escalation condition, push the terminal branch, signal terminal state, and restore the shared checkout to `main`.

Do not request human QA during implementation. The director will review the pushed branch first and decide whether physical Windows signoff is needed.
