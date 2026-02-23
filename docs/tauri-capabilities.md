# Tauri Capability Policy

This app keeps a single `default` capability scoped to the `main` window in
`src-tauri/capabilities/default.json`.

## Allowed Permission Sets

- `core:path:default` for path operations used by bridge helpers and frontend path APIs.
- `core:event:default` for UI subscriptions and backend event emission (`source-open`,
  `calibre-load`, `session-state`, `reader-state`, `tts-state`, `pdf-transcription`, `log-level`).
- `core:app:default` for app lifecycle operations (safe quit and window close flow).
- `core:window:default` and `core:webview:default` for standard desktop shell behavior.
- `log:default` for plugin logging to webview/stdout.

## Notes

- Filesystem and subprocess operations required for EPUB/PDF/quack-check are executed inside the
  Rust backend and are not exposed as frontend plugin commands.
- Capability changes should be reviewed together with command surface changes in
  `docs/tauri-command-groups.md`.
