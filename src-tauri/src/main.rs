#[path = "../../src/tts_worker.rs"]
mod tts_worker;

fn main() {
    if tts_worker::maybe_run_worker() {
        return;
    }
    ebup_viewer_tauri_lib::run();
}
