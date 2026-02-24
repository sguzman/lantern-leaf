# Tauri E2E Runner

This project now has a dedicated Tauri-native smoke E2E path in addition to browser-only Playwright tests.

## Command

- `pnpm --dir ui run test:e2e:tauri`
- `pnpm --dir ui run test:e2e:tauri:soak -- --iterations 3`

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
- Verifies calibre load terminal lifecycle event (`finished`/`failed`) and diagnostics in runtime.
- Verifies text-only/pretty toggle behavior.
- Verifies reader search apply/next/prev updates highlighted sentence selection.
- Verifies settings/stats/TTS panel exclusivity.
- Verifies TTS controls (toggle/play/pause/play page/play highlight/seek/repeat) and paused-state invariants.
- Verifies sentence navigation updates TTS sentence position.
- Verifies paused-state invariants are preserved across next-page and next-sentence controls.
- Verifies topbar and TTS control rows stay single-line (no vertical expansion) under a narrower window width.
- Verifies TTS toggle label transitions in the reader.
- In soak mode, repeats the full runtime smoke for `N` iterations and writes a report to `tmp/tauri-soak-report.json`.

## Soak Report

- Default report path: `tmp/tauri-soak-report.json`
- Override report path: `TAURI_SOAK_REPORT=/path/to/report.json`
- Override iteration count: `TAURI_SOAK_ITERATIONS=5`
- Report fields include: pass/fail counts, per-iteration wall duration, extracted smoke-test duration, and aggregate metrics (`avg/min/max/p95`).

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

The `gui-migration` workflow includes:

- `tauri-e2e`: always-on runtime smoke under `xvfb`.
- `tauri-e2e-soak`: manual `workflow_dispatch` job for repeated runtime soak runs, with `tmp/tauri-soak-report.json` uploaded as an artifact.
