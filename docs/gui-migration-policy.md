# GUI Migration Branch Policy

## Branching

- Use `gui-migration/*` branches for all Tauri/React migration work.
- Keep the `main` branch shipping the current Rust/iced app until parity signoff.
- Scope each PR to one roadmap slice (for example: `phase-3-session-bridge`, `phase-7-starter-port`).

## Merge Gates

- `cargo check` must pass for the existing Rust app.
- `cargo check --manifest-path src-tauri/Cargo.toml` must pass for the bridge.
- `pnpm --dir ui run check` and `pnpm --dir ui run build` must pass for frontend.
- Roadmap checkboxes touched by the PR must be updated in `GUI-ROADMAP.md`.

## Rollback Plan

- If migration regressions are found, keep iced as default and disable Tauri launch scripts.
- Revert only the migration PR that introduced the regression; do not revert unrelated fixes.
- Keep source/cache/config formats backward compatible throughout migration.
- Do not remove iced modules until Phase 14 parity signoff is complete.
