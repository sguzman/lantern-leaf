# Bridge Compatibility Policy

This policy defines how the Rust Tauri bridge and the React/TypeScript client evolve without breaking each other.

## Scope

- Rust command handlers in `src-tauri/src/lib.rs`
- Frontend adapter in `ui/src/api/tauri.ts`
- Shared transport DTOs in `src-tauri/src/lib.rs` and generated TypeScript bindings in `ui/src/generated/` (re-exported by `ui/src/types.ts`)
- Bridge events: `source-open`, `calibre-load`, `session-state`, `reader-state`

## Rules

- Commands are append-only by default.
- Existing command names are stable and must not be renamed.
- Existing DTO fields are stable and must not change meaning.
- Required fields must not be removed.
- New fields must be additive and optional from the receiver perspective.
- Command argument renames are treated as breaking changes.
- Error payload shape remains `{ code: string, message: string }`.
- Event channel names are stable.
- `request_id` remains mandatory for all state/progress events.

## Versioning

- Patch release: internal fixes without command/event/DTO contract changes.
- Minor release: additive commands or additive optional fields.
- Major release: any breaking bridge change.

## Required Validation For Contract Changes

- Update Rust DTO definitions and regenerate TS bindings in the same change.
- Regenerate bindings with `pnpm run types:generate`.
- Verify generated bindings are committed with `pnpm run types:check`.
- Add/adjust Rust bridge contract tests in `src-tauri/src/lib.rs`.
- Add/adjust adapter/store tests in `ui/tests`.
- Run:
  - `cargo test -p ebup-viewer-tauri --lib`
  - `cargo check --workspace`
  - `pnpm run types:check`
  - `pnpm --dir ui run check`
  - `pnpm --dir ui run test`
  - `pnpm --dir ui run build`

## Deprecation Procedure

- Mark deprecated commands/fields in docs and adapter comments.
- Keep deprecated surface for at least one minor release.
- Introduce replacement first, then remove only in next major release.
