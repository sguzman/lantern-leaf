//! Legacy binary shim.
//!
//! The desktop GUI now runs through the Tauri entrypoint in `src-tauri`.
//! This binary is retained for two reasons:
//! - It hosts the `--tts-worker` subprocess mode used by the Piper worker pool.
//! - It provides a clear migration message when launched directly.

mod tts_worker;

fn main() {
    if tts_worker::maybe_run_worker() {
        return;
    }
    eprintln!("The iced desktop UI has been decommissioned.");
    eprintln!("Run `pnpm tauri:dev` (or `cargo tauri dev`) to launch ebup-viewer.");
}
