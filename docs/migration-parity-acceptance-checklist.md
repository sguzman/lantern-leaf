# Migration Parity Acceptance Checklist

Use this checklist for explicit pass/fail gating before cutover.

Last automated verification run: `2026-02-24` (local + Tauri runtime smoke).

## Starter Flows

- [x] Open EPUB path succeeds and transitions to reader.
- [x] Open PDF path succeeds and emits PDF transcription events.
- [x] Open PDF path emits terminal transcription events (`finished`/`failed`/`cancelled`) with diagnostics.
- [x] Open clipboard text succeeds and creates a reusable source.
- [x] Recent open/delete behavior is correct and cache-aware.
- [x] Calibre load/open works at scale without UI lockups.
- [x] Calibre load emits terminal lifecycle (`finished`/`failed`) with diagnostics in runtime smoke.

## Reader Flows

- [x] Sentence click re-anchors highlight and playback correctly.
- [x] Page navigation preserves expected highlight semantics.
- [x] Search query + next/prev behaves identically to prior behavior.
- [x] Pretty/text-only mode switching is reversible and stable.

## TTS Flows

- [x] Play/pause/toggle/play-from-page/play-from-highlight all work.
- [x] Prev/next/repeat sentence controls maintain pause semantics.
- [x] 3-decimal progress remains stable in TTS and stats displays.

## Layout/UX Rules

- [x] Top/TTS control bars never render vertically/compressed text.
- [x] Auto-scroll/auto-center keeps highlight in view across resize and setting changes.
- [x] Settings/stats panel exclusivity is preserved.

## Runtime/Persistence

- [x] Close session cancels in-flight jobs and returns to starter.
- [x] Safe quit performs housekeeping and persistence.
- [x] Bookmark/config resume fidelity is preserved.
- [x] Runtime log-level change persists to `conf/config.toml`.

## Test Gates

- [x] `cargo test`
- [x] `cargo test -p ebup-viewer-tauri --lib`
- [x] `cargo check --workspace`
- [x] `pnpm --dir ui run check`
- [x] `pnpm --dir ui run lint`
- [x] `pnpm --dir ui run test`
- [x] `pnpm --dir ui run test:e2e`
- [x] `pnpm --dir ui run test:e2e:tauri`
- [x] `pnpm --dir ui run build`
- [x] `pnpm --dir ui run audit:bundle`
- [x] `pnpm run types:check`
