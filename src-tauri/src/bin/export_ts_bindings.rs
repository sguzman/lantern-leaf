use std::path::Path;

fn main() {
    let out_dir = Path::new("ui/src/generated");
    if let Err(err) = lanternleaf_tauri_lib::export_ts_bindings(out_dir) {
        eprintln!("failed to export TS bindings: {err}");
        std::process::exit(1);
    }
    println!("exported TS bindings to {}", out_dir.display());
}
