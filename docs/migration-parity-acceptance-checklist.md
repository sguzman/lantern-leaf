# Migration Parity Acceptance Checklist

Use this checklist for explicit pass/fail gating before cutover.

## Starter Flows

- [ ] Open EPUB path succeeds and transitions to reader.
- [ ] Open PDF path succeeds and emits PDF transcription events.
- [ ] Open clipboard text succeeds and creates a reusable source.
- [ ] Recent open/delete behavior is correct and cache-aware.
- [ ] Calibre load/open works at scale without UI lockups.

## Reader Flows

- [ ] Sentence click re-anchors highlight and playback correctly.
- [ ] Page navigation preserves expected highlight semantics.
- [ ] Search query + next/prev behaves identically to prior behavior.
- [ ] Pretty/text-only mode switching is reversible and stable.

## TTS Flows

- [ ] Play/pause/toggle/play-from-page/play-from-highlight all work.
- [ ] Prev/next/repeat sentence controls maintain pause semantics.
- [ ] 3-decimal progress remains stable in TTS and stats displays.

## Layout/UX Rules

- [ ] Top/TTS control bars never render vertically/compressed text.
- [ ] Auto-scroll/auto-center keeps highlight in view across resize and setting changes.
- [ ] Settings/stats panel exclusivity is preserved.

## Runtime/Persistence

- [ ] Close session cancels in-flight jobs and returns to starter.
- [ ] Safe quit performs housekeeping and persistence.
- [ ] Bookmark/config resume fidelity is preserved.
- [ ] Runtime log-level change persists to `conf/config.toml`.

## Test Gates

- [ ] `cargo test`
- [ ] `cargo test -p ebup-viewer-tauri --lib`
- [ ] `cargo check --workspace`
- [ ] `pnpm --dir ui run check`
- [ ] `pnpm --dir ui run lint`
- [ ] `pnpm --dir ui run test`
- [ ] `pnpm --dir ui run test:e2e`
- [ ] `pnpm --dir ui run build`
