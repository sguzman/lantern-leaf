# Tauri E2E Runner

This project now has a dedicated Tauri-native smoke E2E path in addition to browser-only Playwright tests.

## Command

- `pnpm --dir ui run test:e2e:tauri`

## What It Verifies

- Builds the Tauri app in debug/no-bundle mode.
- Launches `tauri-driver`.
- Starts a WebDriver session with browser name `wry`.
- Opens a real local text source.
- Opens a generated EPUB fixture source and verifies reader transition.
- Verifies reader open/close flow.
- Verifies starter recent-entry delete behavior for the opened source.
- Verifies clipboard-open flow in runtime (stubbed clipboard API) and reader text render.
- Verifies PDF source-open and pdf-transcription terminal event lifecycle (including diagnostics on failure paths).
- Verifies text-only/pretty toggle behavior.
- Verifies reader search apply/next/prev updates highlighted sentence selection.
- Verifies settings/stats/TTS panel exclusivity.
- Verifies TTS controls (toggle/play/pause/play page/play highlight/seek/repeat) and paused-state invariants.
- Verifies sentence navigation updates TTS sentence position.
- Verifies paused-state invariants are preserved across next-page and next-sentence controls.
- Verifies topbar and TTS control rows stay single-line (no vertical expansion) under a narrower window width.
- Verifies TTS toggle label transitions in the reader.

## Local Prerequisites

- Rust toolchain
- `cargo install tauri-driver --locked`
- Linux packages:
  - `libgtk-3-dev`
  - `libwebkit2gtk-4.1-dev`
  - `libayatana-appindicator3-dev`
  - `librsvg2-dev`
  - `webkit2gtk-driver`
  - `xvfb` (for headless CI-style execution)

## CI

The `gui-migration` workflow includes a `tauri-e2e` job that runs this smoke path under `xvfb`.
