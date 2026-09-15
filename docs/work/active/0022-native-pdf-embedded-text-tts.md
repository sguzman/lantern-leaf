# 0022 — Native PDF embedded-text / TTS trustworthy path — A10

This is the A10 correction continuation of Goal 0022 after A9 real-desktop rejection. Preserve all accepted A9 native Rust/eframe/egui architecture and physical behavior. Continue the existing `codex/0022-native-pdf-embedded-text-tts` branch and `docs/work/reports/0022.md` lineage.

## Authorized corrections

1. Make TTS Speed and Volume app-owned synchronous drafts. Sliders must follow the hand immediately; stale acknowledgements must not overwrite newer drafts; source/session changes resynchronize intentionally; matching acknowledgements settle cleanly; commits are bounded/coalesced, duplicate same-target commits are no-ops, and final canonical settings persist. Add delayed/stale acknowledgement, rapid-drag, same-target, and final-value tests.
2. Keep transient command/status diagnostics out of document geometry. Use a fixed/bounded presentation with bounded visible count and width; short versus very long diagnostics, command churn, slider dragging, and seek spam must preserve the reader viewport rect/height.
3. Make Search navigable: Previous/Next buttons, Enter/Shift+Enter navigation, selected `X / Y`, native-page provenance, bounded excerpt, explicit empty/no-match states, deterministic selection reconciliation, persistent open state, visual-mode authoritative PDF page jumps, Text-only canonical-row selection/follow, later-page PDF search, and no egui full-document search.
4. Repair real runtime burst TTS seeking for Next and Previous across PDF page boundaries and empty pages, with monotonic canonical identity, EPUB parity, and stale first-sample rejection under canceled/replaced generations. Use the simulated production runtime harness, not only session helpers.
5. Add an obvious quick-control action to play from the current authoritative PDF page, selecting the first accepted sentence there or the next non-empty page. Do not fake PDF sentence geometry; preserve Text-only row click-to-play.
6. Diagnose and repair persisted EPUB close/reopen presentation corruption. Reproduce through actual persisted/cache state, reject incomplete derived artifacts, rebuild safely, preserve valid bookmarks/canonical position, render complete Pretty content immediately without TTS/Next healing, and retain the new-EPUB stress-toggle regression. Do not delete healthy caches globally.

## Non-goals

Do not implement PDF sentence rectangles, visual spoken highlight, PDF visual auto-follow, exact rendered-text click-to-sentence, Quack-check, Python, Docling, OCR, or hostile/mixed recovery. Goal 0024 Caliberate Recents identity remains separate.

## Validation and handoff

Run focused tests for every correction and all A8/A9 regressions, then `cargo test --workspace -- --test-threads=1`, `cargo check --workspace`, `cargo build --workspace`, `git diff --check`, repo-native Windows QA preparation, and fresh hosted native-workspace plus hosted-renderer-probe. Update the Goal 0022 report with A10 evidence. Move `ready -> active -> done`, push before signaling, re-arm the watcher with `-Rearm`, restore the shared checkout to `main`, and do not request human QA.
