use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{Emitter, State};
use tauri_plugin_log::{Target, TargetKind, log::LevelFilter};

#[allow(dead_code, unused_imports)]
#[path = "../../src/cache.rs"]
mod cache;
#[allow(dead_code, unused_imports)]
#[path = "../../src/config/mod.rs"]
mod config;

const MAX_RECENT_LIMIT: usize = 512;
const DEFAULT_RECENT_LIMIT: usize = 64;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
enum UiMode {
    Starter,
    Reader,
}

#[derive(Debug, Clone, Serialize)]
struct BootstrapConfig {
    default_font_size: u32,
    default_lines_per_page: usize,
    default_tts_speed: f32,
    default_pause_after_sentence: f32,
}

#[derive(Debug, Clone, Serialize)]
struct BootstrapState {
    app_name: String,
    mode: String,
    config: BootstrapConfig,
}

#[derive(Debug, Clone, Serialize)]
struct SessionState {
    mode: UiMode,
    active_source_path: Option<String>,
    open_in_flight: bool,
}

#[derive(Debug, Clone, Serialize)]
struct RecentBook {
    source_path: String,
    display_title: String,
    thumbnail_path: Option<String>,
    last_opened_unix_secs: u64,
}

#[derive(Debug, Clone, Serialize)]
struct SourceOpenEvent {
    phase: String,
    source_path: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct BridgeError {
    code: String,
    message: String,
}

#[derive(Debug, Clone)]
struct BackendState {
    mode: UiMode,
    active_source_path: Option<PathBuf>,
    open_in_flight: bool,
}

impl Default for BackendState {
    fn default() -> Self {
        Self {
            mode: UiMode::Starter,
            active_source_path: None,
            open_in_flight: false,
        }
    }
}

fn to_session_state(state: &BackendState) -> SessionState {
    SessionState {
        mode: state.mode,
        active_source_path: state
            .active_source_path
            .as_ref()
            .map(|path| path.to_string_lossy().to_string()),
        open_in_flight: state.open_in_flight,
    }
}

fn bridge_error(code: &str, message: impl Into<String>) -> BridgeError {
    BridgeError {
        code: code.to_string(),
        message: message.into(),
    }
}

fn with_open_guard(
    state: &State<'_, Mutex<BackendState>>,
    work: impl FnOnce() -> Result<PathBuf, BridgeError>,
) -> Result<SessionState, BridgeError> {
    {
        let mut guard = state
            .lock()
            .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
        if guard.open_in_flight {
            return Err(bridge_error(
                "operation_conflict",
                "A book open operation is already in progress",
            ));
        }
        guard.open_in_flight = true;
    }

    let outcome = work();

    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    guard.open_in_flight = false;

    match outcome {
        Ok(path) => {
            guard.mode = UiMode::Reader;
            guard.active_source_path = Some(path);
            Ok(to_session_state(&guard))
        }
        Err(err) => Err(err),
    }
}

fn normalize_recent_limit(limit: Option<usize>) -> usize {
    limit.unwrap_or(DEFAULT_RECENT_LIMIT).clamp(1, MAX_RECENT_LIMIT)
}

fn is_supported_source(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase()),
        Some(ext) if ext == "epub" || ext == "pdf" || ext == "txt" || ext == "md" || ext == "markdown"
    )
}

fn resolve_source_path(path: &str) -> Result<PathBuf, BridgeError> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(bridge_error("invalid_input", "Path cannot be empty"));
    }

    let candidate = PathBuf::from(trimmed);
    if !candidate.exists() {
        return Err(bridge_error(
            "not_found",
            format!("Source path does not exist: {trimmed}"),
        ));
    }

    if !candidate.is_file() {
        return Err(bridge_error(
            "invalid_input",
            format!("Source path is not a file: {trimmed}"),
        ));
    }

    if !is_supported_source(&candidate) {
        return Err(bridge_error(
            "unsupported_source",
            format!(
                "Unsupported source type for {} (expected .epub/.pdf/.txt/.md/.markdown)",
                candidate.display()
            ),
        ));
    }

    candidate.canonicalize().map_err(|err| {
        bridge_error(
            "io_error",
            format!("Failed to canonicalize source path {}: {err}", candidate.display()),
        )
    })
}

#[tauri::command]
fn session_get_bootstrap() -> BootstrapState {
    let cfg = config::load_config(Path::new("conf/config.toml"));
    BootstrapState {
        app_name: "ebup-viewer".to_string(),
        mode: "migration".to_string(),
        config: BootstrapConfig {
            default_font_size: cfg.font_size,
            default_lines_per_page: cfg.lines_per_page,
            default_tts_speed: cfg.tts_speed,
            default_pause_after_sentence: cfg.pause_after_sentence,
        },
    }
}

#[tauri::command]
fn session_get_state(state: State<'_, Mutex<BackendState>>) -> Result<SessionState, BridgeError> {
    let guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    Ok(to_session_state(&guard))
}

#[tauri::command]
fn session_return_to_starter(
    state: State<'_, Mutex<BackendState>>,
) -> Result<SessionState, BridgeError> {
    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    guard.mode = UiMode::Starter;
    guard.active_source_path = None;
    guard.open_in_flight = false;
    Ok(to_session_state(&guard))
}

#[tauri::command]
fn recent_list(limit: Option<usize>) -> Vec<RecentBook> {
    cache::list_recent_books(normalize_recent_limit(limit))
        .into_iter()
        .map(|recent| RecentBook {
            source_path: recent.source_path.to_string_lossy().to_string(),
            display_title: recent.display_title,
            thumbnail_path: recent
                .thumbnail_path
                .map(|thumb| thumb.to_string_lossy().to_string()),
            last_opened_unix_secs: recent.last_opened_unix_secs,
        })
        .collect()
}

#[tauri::command]
fn recent_delete(path: String) -> Result<(), BridgeError> {
    let source = PathBuf::from(path.trim());
    if source.as_os_str().is_empty() {
        return Err(bridge_error("invalid_input", "Path cannot be empty"));
    }
    cache::delete_recent_source_and_cache(&source).map_err(|err| bridge_error("io_error", err))
}

#[tauri::command]
fn source_open_path(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
    path: String,
) -> Result<SessionState, BridgeError> {
    let _ = app.emit(
        "source-open",
        SourceOpenEvent {
            phase: "started".to_string(),
            source_path: Some(path.clone()),
            message: None,
        },
    );

    let result = with_open_guard(&state, || {
        let source = resolve_source_path(&path)?;
        cache::remember_source_path(&source);
        Ok(source)
    });

    let event = match &result {
        Ok(session) => SourceOpenEvent {
            phase: "finished".to_string(),
            source_path: session.active_source_path.clone(),
            message: None,
        },
        Err(err) => SourceOpenEvent {
            phase: "failed".to_string(),
            source_path: Some(path),
            message: Some(err.message.clone()),
        },
    };
    let _ = app.emit("source-open", event);
    result
}

#[tauri::command]
fn source_open_clipboard_text(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
    text: String,
) -> Result<SessionState, BridgeError> {
    let _ = app.emit(
        "source-open",
        SourceOpenEvent {
            phase: "started".to_string(),
            source_path: None,
            message: Some("Opening clipboard text".to_string()),
        },
    );

    let result = with_open_guard(&state, || {
        let path = cache::persist_clipboard_text_source(&text)
            .map_err(|err| bridge_error("invalid_input", err))?;
        cache::remember_source_path(&path);
        Ok(path)
    });

    let event = match &result {
        Ok(session) => SourceOpenEvent {
            phase: "finished".to_string(),
            source_path: session.active_source_path.clone(),
            message: None,
        },
        Err(err) => SourceOpenEvent {
            phase: "failed".to_string(),
            source_path: None,
            message: Some(err.message.clone()),
        },
    };
    let _ = app.emit("source-open", event);
    result
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let log_plugin = tauri_plugin_log::Builder::new()
        .level(LevelFilter::Info)
        .target(Target::new(TargetKind::Stdout))
        .target(Target::new(TargetKind::Webview))
        .build();

    tauri::Builder::default()
        .manage(Mutex::new(BackendState::default()))
        .plugin(log_plugin)
        .invoke_handler(tauri::generate_handler![
            session_get_bootstrap,
            session_get_state,
            session_return_to_starter,
            recent_list,
            recent_delete,
            source_open_path,
            source_open_clipboard_text
        ])
        .run(tauri::generate_context!())
        .expect("tauri runtime failed");
}
