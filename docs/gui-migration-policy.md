# GUI Migration Branch Policy

## Branching

- Use `gui-migration/*` branches for all Tauri/React migration work.
- `main` now ships the Tauri/React shell; iced UI modules are decommissioned.
- Scope each PR to one roadmap slice (for example: `phase-3-session-bridge`, `phase-7-starter-port`).

## Merge Gates

- `cargo check` must pass for the existing Rust app.
- `cargo check --manifest-path src-tauri/Cargo.toml` must pass for the bridge.
- `pnpm --dir ui run check` and `pnpm --dir ui run build` must pass for frontend.
- Roadmap checkboxes touched by the PR must be updated in `GUI-ROADMAP.md`.
- Any decommission PR must explicitly prove Piper/TTS behavior parity is preserved.

## Rollback Plan

- If migration regressions are found, keep Tauri as default and revert only the offending migration PRs.
- Revert only the migration PR that introduced the regression; do not revert unrelated fixes.
- Keep source/cache/config formats backward compatible throughout migration.
- Do not reintroduce parallel GUI frameworks unless a new migration plan is approved.

## Non-Negotiable Product Constraints

- Piper-based TTS is a core product feature and must remain in the final shipped app.
- “UI decommission” means framework-specific UI removal only (iced widgets/view/update wiring), not TTS engine/runtime removal.
