# GUI Migration Parity Report

This report compares core workflows between the legacy iced UI path and the Tauri/React migration shell.

## Scope

- Startup and starter screen operations.
- Source open flows (EPUB, PDF via quack-check, clipboard text).
- Reader navigation/search/settings/stats interactions.
- TTS playback controls and sentence highlighting semantics.
- Cancellation and shutdown behavior for in-flight work.

## Workflow Parity Matrix

| Workflow | iced status | Tauri/React status | Evidence |
| --- | --- | --- | --- |
| Open local source path (`.epub/.pdf/.txt/.md`) | Supported | Supported | Bridge commands `source_open_path`, `source_open_clipboard_text` in `src-tauri/src/lib.rs`; adapter tests in `ui/tests/tauriApi.test.ts` |
| Open EPUB path in real Tauri runtime | Supported | Supported | Tauri-runner smoke opens a generated EPUB fixture and asserts reader transition in `ui/e2e-tauri/smoke.test.mjs` |
| Open clipboard text source | Supported | Supported | `source_open_clipboard_text` in `src-tauri/src/lib.rs`; starter UI action in `ui/src/components/StarterShell.tsx` |
| Recent list + delete recent/cache | Supported | Supported | Commands `recent_list`, `recent_delete` in `src-tauri/src/lib.rs`; starter UI cards/actions in `ui/src/components/StarterShell.tsx` |
| Starter clipboard + recent-delete in real Tauri runtime | Supported | Supported | Runtime smoke flow covers clipboard-open and recent-delete actions in `ui/e2e-tauri/smoke.test.mjs` |
| PDF transcription event lifecycle visibility in real runtime | Supported | Supported | Runtime smoke asserts terminal `pdf-transcription` + `source-open` marker events (including diagnostics on failure) in `ui/e2e-tauri/smoke.test.mjs` |
| Calibre load/open | Supported | Supported | Commands `calibre_load_books`, `calibre_open_book`; virtualization helpers in `ui/src/components/calibreList.ts` and tests in `ui/tests/calibreList.test.ts` |
| Calibre load terminal lifecycle visibility in real runtime | Supported | Supported | Runtime smoke asserts terminal `calibre-load` marker events and diagnostics in `ui/e2e-tauri/smoke.test.mjs` |
| Reader page navigation/search | Supported | Supported | Reader commands in `src-tauri/src/lib.rs`; UI integration in `ui/src/components/ReaderShell.tsx` |
| Reader search next/prev in real Tauri runtime | Supported | Supported | Runtime smoke exercises search apply/next/prev and highlighted sentence transitions in `ui/e2e-tauri/smoke.test.mjs` |
| Sentence click to move highlight/TTS anchor | Supported | Supported | Mapping/state in `src-tauri/src/session.rs`; store behavior tests in `ui/tests/appStore.test.ts` |
| Pause semantics across page/sentence navigation | Supported | Supported | Session tests in `src-tauri/src/session.rs` (`paused_state_*`, `sentence_click_keeps_paused_state`) |
| Pause semantics in real Tauri runtime | Supported | Supported | Tauri-runner scenario asserts paused state is preserved across `Next Page` and `Next Sentence` in `ui/e2e-tauri/smoke.test.mjs` |
| Settings/stats/TTS panel exclusivity and text-mode toggle | Supported | Supported | Reader panel/text-mode controls in `ui/src/components/ReaderShell.tsx`; Tauri-runner coverage in `ui/e2e-tauri/smoke.test.mjs` |
| Source-open cancellation on close/return | Supported | Supported | Cancellation plumbing in `src-tauri/src/lib.rs` (`active_open_request` / `active_open_source_path`) and event handling test in `ui/tests/appStore.test.ts` |
| Responsive no-vertical-collapse top controls | Supported | Supported | Policy helpers in `ui/src/components/layoutPolicies.ts`; tests in `ui/tests/layoutPolicies.test.ts` |
| Narrow-width topbar/TTS row no-vertical expansion in real runtime | Supported | Supported | Tauri-runner assertions against `reader-topbar` / `reader-tts-control-row` heights in `ui/e2e-tauri/smoke.test.mjs` |
| TTS progress precision (3 decimals) | Supported | Supported | TTS display formatting in `ui/src/components/ReaderShell.tsx` |
| Full TTS control surface in real runtime | Supported | Supported | Runtime smoke covers toggle/play/pause/play-from-page/play-from-highlight and seek/repeat pause semantics in `ui/e2e-tauri/smoke.test.mjs` |
| Runtime log-level updates | Supported | Supported | Command `logging_set_level` and event `log-level` in `src-tauri/src/lib.rs`; starter controls in `ui/src/components/StarterShell.tsx` |
| Runtime log-level persistence to config | Supported | Supported | Config persistence helper + override-path coverage in `src-tauri/src/lib.rs` tests (`persist_base_config_writes_updated_log_level`, `app_config_path_uses_override_env_when_present`) |
| Reader shutdown housekeeping persistence | Supported | Supported | `cleanup_for_shutdown_persists_active_reader_housekeeping` test in `src-tauri/src/lib.rs` |
| Bridge progress/state events for TTS and PDF transcription | Supported | Supported | Events `tts-state` / `pdf-transcription` in `src-tauri/src/lib.rs`; store subscriptions in `ui/src/store/appStore.ts` |
| Stale async event rejection | Supported | Supported | Request-id monotonic guards for source/calibre/tts/pdf/log events in `ui/src/store/slices/jobsSlice.ts`; coverage in `ui/tests/appStore.test.ts` |
| Bookmark/config cache compatibility | Supported | Supported | Cache roundtrip/legacy tests in `src/cache.rs` (`bookmark_roundtrip_*`, `load_bookmark_defaults_scroll_*`, `epub_config_roundtrip_*`) |
| Zustand domain slices | Supported | Supported | Physical slices in `ui/src/store/slices/` and selector projections in `ui/src/store/selectors.ts` |
| PDF fallback robustness (quack-check native/split fallback) | Supported | Supported | Pipeline fallback tests in `src/quack_check/pipeline.rs` |

## Current Gaps

- Full PDF success in runtime smoke is still environment-dependent on quack-check/docling dependencies; smoke currently gates terminal event diagnostics.
- Full calibre-open success in runtime smoke is still environment-dependent on reachable calibre backends; smoke currently gates terminal event diagnostics.

## Validation Snapshot

Latest migration verification run includes:

- `pnpm --dir ui run check`
- `pnpm --dir ui run lint`
- `pnpm --dir ui run test`
- `pnpm --dir ui run test:e2e`
- `pnpm --dir ui run test:e2e:tauri`
- `pnpm --dir ui run build`
- `pnpm --dir ui run audit:bundle`
- `cargo test -p ebup-viewer-tauri --lib`
- `cargo test`
- `cargo check --workspace`

Reference baseline docs:

- `docs/migration-feature-inventory.md`
- `docs/migration-non-regression-contract.md`
- `docs/migration-baseline-metrics.md`
- `docs/migration-baseline-log-scenarios.md`
- `docs/migration-parity-acceptance-checklist.md`
