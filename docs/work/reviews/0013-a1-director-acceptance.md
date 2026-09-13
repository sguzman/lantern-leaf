# Goal 0013 A1 — director acceptance

Date: 2026-09-12

## Decision

**ACCEPTED FOR FOCUSED REAL-DESKTOP SIGNOFF.**

The implementation is complete, pushed, CI-green, and suitable for physical validation on the starter/library shell.

## Accepted evidence

- Worker branch synchronized director `main` at `2b7999714e7c93fb3e52846ec06ae16b021102e6` before implementation.
- Production implementation `58ae9de` derives starter layout mode from the actual center `ui.available_width()` and uses a deterministic `1120px` two-column breakpoint with a full one-column fallback below it.
- Recents, Calibre, Browser Tabs, open controls, and diagnostics are rendered through width-bounded groups; long control rows use wrapped layouts; long paths/URLs use bounded truncation; representative prose/title surfaces wrap rather than forcing adjacent-column geometry.
- Existing bounded Calibre row virtualization and shell reachability remain intact. No heavy/blocking work was moved onto the egui/render thread.
- Deterministic starter layout tests cover the breakpoint and a long-content owned-group containment case.
- Goal report records passing workspace tests, check/build, repo-native Windows QA preparation, renderer smoke, and `git diff --check`.
- GitHub Actions Windows baseline run `34725734155` completed successfully for the pushed worker branch.

## Director cleanup

The pushed terminal branch contained both `docs/work/active/0013-starter-shell-panel-containment.md` and the identical `done/` copy despite reporting terminalization. This was lifecycle bookkeeping only; production code and validation were already complete. The stale `active/` copy was removed by the director before integration.

## Remaining gate

Because the original defect is explicitly visual/responsive and was observed on the user's real desktop, one focused physical Windows pass remains appropriate before final closure.

Verify at the ordinary window size that previously showed panel bleed, then resize narrower and wider around the responsive transition:

- Recents / Calibre / Browser Tabs must never overlap;
- below the breakpoint the shell should become a readable one-column stack;
- long titles/paths/control rows must stay inside their owned region;
- Browser Tabs and diagnostics must remain reachable;
- no application-level horizontal overflow or resize thrash should appear.

Goal 0013 should close immediately if that pass is clean.
