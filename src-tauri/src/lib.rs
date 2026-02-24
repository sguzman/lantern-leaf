use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{Emitter, Manager, State};
use tauri_plugin_log::{Target, TargetKind, log::LevelFilter};
use tracing::{info, warn};
use ts_rs::TS;

#[allow(dead_code, unused_imports)]
#[path = "../../src/cache.rs"]
mod cache;
#[allow(dead_code, unused_imports)]
#[path = "../../src/calibre.rs"]
mod calibre;
#[allow(dead_code, unused_imports)]
#[path = "../../src/config/mod.rs"]
mod config;
#[allow(dead_code, unused_imports)]
#[path = "../../src/epub_loader.rs"]
mod epub_loader;
#[allow(dead_code, unused_imports)]
#[path = "../../src/normalizer.rs"]
mod normalizer;
#[allow(dead_code, unused_imports)]
#[path = "../../src/pagination.rs"]
mod pagination;
#[allow(dead_code, unused_imports)]
#[path = "../../src/quack_check/mod.rs"]
mod quack_check;
#[allow(dead_code, unused_imports)]
#[path = "../../src/text_utils.rs"]
mod text_utils;

mod session;

const MAX_RECENT_LIMIT: usize = 512;
const DEFAULT_RECENT_LIMIT: usize = 64;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
enum UiMode {
    Starter,
    Reader,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
struct BootstrapConfig {
    theme: config::ThemeMode,
    font_family: config::FontFamily,
    font_weight: config::FontWeight,
    day_highlight: config::HighlightColor,
    night_highlight: config::HighlightColor,
    log_level: String,
    default_font_size: u32,
    default_lines_per_page: usize,
    default_tts_speed: f32,
    default_pause_after_sentence: f32,
    key_toggle_play_pause: String,
    key_next_sentence: String,
    key_prev_sentence: String,
    key_repeat_sentence: String,
    key_toggle_search: String,
    key_safe_quit: String,
    key_toggle_settings: String,
    key_toggle_stats: String,
    key_toggle_tts: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
struct BootstrapState {
    app_name: String,
    mode: String,
    config: BootstrapConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
struct SessionState {
    mode: UiMode,
    active_source_path: Option<String>,
    open_in_flight: bool,
    panels: session::PanelState,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
struct OpenSourceResult {
    session: SessionState,
    reader: session::ReaderSnapshot,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
struct RecentBook {
    source_path: String,
    display_title: String,
    thumbnail_path: Option<String>,
    #[ts(type = "number")]
    last_opened_unix_secs: u64,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
struct CalibreBookDto {
    #[ts(type = "number")]
    id: u64,
    title: String,
    extension: String,
    authors: String,
    year: Option<i32>,
    #[ts(type = "number | null")]
    file_size_bytes: Option<u64>,
    source_path: Option<String>,
    cover_thumbnail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
struct SourceOpenEvent {
    #[ts(type = "number")]
    request_id: u64,
    phase: String,
    source_path: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
struct CalibreLoadEvent {
    #[ts(type = "number")]
    request_id: u64,
    phase: String,
    count: Option<usize>,
    message: Option<String>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
struct TtsStateEvent {
    #[ts(type = "number")]
    request_id: u64,
    action: String,
    tts: session::ReaderTtsView,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
struct PdfTranscriptionEvent {
    #[ts(type = "number")]
    request_id: u64,
    phase: String,
    source_path: String,
    message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
struct LogLevelEvent {
    #[ts(type = "number")]
    request_id: u64,
    level: String,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
struct SessionStateEvent {
    #[ts(type = "number")]
    request_id: u64,
    action: String,
    session: SessionState,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
struct ReaderStateEvent {
    #[ts(type = "number")]
    request_id: u64,
    action: String,
    reader: session::ReaderSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
struct BridgeError {
    code: String,
    message: String,
}

#[derive(Debug)]
struct BackendState {
    mode: UiMode,
    active_source_path: Option<PathBuf>,
    active_open_source_path: Option<PathBuf>,
    open_in_flight: bool,
    active_open_request: Option<u64>,
    next_request_id: u64,
    panels: session::PanelState,
    base_config: config::AppConfig,
    normalizer: normalizer::TextNormalizer,
    reader: Option<session::ReaderSession>,
    calibre_config: calibre::CalibreConfig,
    calibre_books: Vec<calibre::CalibreBook>,
}

impl BackendState {
    fn new() -> Self {
        let config_path = app_config_path();
        let base_config = config::load_config(&config_path);
        let panels = session::PanelState {
            show_settings: base_config.show_settings,
            show_stats: false,
            show_tts: base_config.show_tts,
        };
        Self {
            mode: UiMode::Starter,
            active_source_path: None,
            active_open_source_path: None,
            open_in_flight: false,
            active_open_request: None,
            next_request_id: 1,
            panels,
            base_config,
            normalizer: normalizer::TextNormalizer::load_default(),
            reader: None,
            calibre_config: calibre::CalibreConfig::load_default(),
            calibre_books: Vec::new(),
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
        panels: state.panels,
    }
}

fn bridge_error(code: &str, message: impl Into<String>) -> BridgeError {
    BridgeError {
        code: code.to_string(),
        message: message.into(),
    }
}

fn app_config_path() -> PathBuf {
    std::env::var_os("EBUP_VIEWER_CONFIG_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("conf/config.toml"))
}

fn persist_base_config(config: &config::AppConfig, path: &Path) -> Result<(), BridgeError> {
    let serialized = config::serialize_config(config).map_err(|err| {
        bridge_error(
            "config_serialize_failed",
            format!("Failed to serialize config for update: {err}"),
        )
    })?;
    fs::write(path, serialized).map_err(|err| {
        bridge_error(
            "io_error",
            format!("Failed to persist config to {}: {err}", path.display()),
        )
    })
}

fn parse_log_level_label(label: &str) -> Option<config::LogLevel> {
    match label.trim().to_ascii_lowercase().as_str() {
        "trace" => Some(config::LogLevel::Trace),
        "debug" => Some(config::LogLevel::Debug),
        "info" => Some(config::LogLevel::Info),
        "warn" | "warning" => Some(config::LogLevel::Warn),
        "error" => Some(config::LogLevel::Error),
        _ => None,
    }
}

fn log_level_to_filter(level: config::LogLevel) -> LevelFilter {
    match level {
        config::LogLevel::Trace => LevelFilter::Trace,
        config::LogLevel::Debug => LevelFilter::Debug,
        config::LogLevel::Info => LevelFilter::Info,
        config::LogLevel::Warn => LevelFilter::Warn,
        config::LogLevel::Error => LevelFilter::Error,
    }
}

fn normalize_recent_limit(limit: Option<usize>) -> usize {
    limit
        .unwrap_or(DEFAULT_RECENT_LIMIT)
        .clamp(1, MAX_RECENT_LIMIT)
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
            format!(
                "Failed to canonicalize source path {}: {err}",
                candidate.display()
            ),
        )
    })
}

fn map_calibre_book(book: calibre::CalibreBook) -> CalibreBookDto {
    CalibreBookDto {
        id: book.id,
        title: book.title,
        extension: book.extension,
        authors: book.authors,
        year: book.year,
        file_size_bytes: book.file_size_bytes,
        source_path: book.path.map(|path| path.to_string_lossy().to_string()),
        cover_thumbnail: book
            .cover_thumbnail
            .map(|path| path.to_string_lossy().to_string()),
    }
}

fn export_single_type<T: TS + 'static>(out_dir: &Path) -> Result<(), String> {
    T::export_all_to(out_dir).map_err(|err| err.to_string())
}

pub fn export_ts_bindings(out_dir: &Path) -> Result<(), String> {
    fs::create_dir_all(out_dir)
        .map_err(|err| format!("Failed to create {}: {err}", out_dir.display()))?;

    for entry in fs::read_dir(out_dir)
        .map_err(|err| format!("Failed to list {}: {err}", out_dir.display()))?
    {
        let entry = entry.map_err(|err| format!("Failed to read entry: {err}"))?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("ts") {
            fs::remove_file(&path)
                .map_err(|err| format!("Failed to remove {}: {err}", path.display()))?;
        }
    }

    export_single_type::<UiMode>(out_dir)?;
    export_single_type::<BootstrapConfig>(out_dir)?;
    export_single_type::<BootstrapState>(out_dir)?;
    export_single_type::<SessionState>(out_dir)?;
    export_single_type::<OpenSourceResult>(out_dir)?;
    export_single_type::<RecentBook>(out_dir)?;
    export_single_type::<CalibreBookDto>(out_dir)?;
    export_single_type::<SourceOpenEvent>(out_dir)?;
    export_single_type::<CalibreLoadEvent>(out_dir)?;
    export_single_type::<TtsStateEvent>(out_dir)?;
    export_single_type::<PdfTranscriptionEvent>(out_dir)?;
    export_single_type::<LogLevelEvent>(out_dir)?;
    export_single_type::<SessionStateEvent>(out_dir)?;
    export_single_type::<ReaderStateEvent>(out_dir)?;
    export_single_type::<BridgeError>(out_dir)?;
    export_single_type::<session::PanelState>(out_dir)?;
    export_single_type::<session::ReaderSettingsView>(out_dir)?;
    export_single_type::<session::ReaderTtsView>(out_dir)?;
    export_single_type::<session::ReaderSettingsPatch>(out_dir)?;
    export_single_type::<session::ReaderStats>(out_dir)?;
    export_single_type::<session::ReaderSnapshot>(out_dir)?;
    export_single_type::<session::TtsPlaybackState>(out_dir)?;
    export_single_type::<config::ThemeMode>(out_dir)?;
    export_single_type::<config::FontFamily>(out_dir)?;
    export_single_type::<config::FontWeight>(out_dir)?;
    export_single_type::<config::HighlightColor>(out_dir)?;

    let index_content = r#"export type { UiMode } from "./UiMode";
export type { BootstrapConfig } from "./BootstrapConfig";
export type { BootstrapState } from "./BootstrapState";
export type { SessionState } from "./SessionState";
export type { OpenSourceResult } from "./OpenSourceResult";
export type { RecentBook } from "./RecentBook";
export type { CalibreBookDto } from "./CalibreBookDto";
export type { SourceOpenEvent } from "./SourceOpenEvent";
export type { CalibreLoadEvent } from "./CalibreLoadEvent";
export type { TtsStateEvent } from "./TtsStateEvent";
export type { PdfTranscriptionEvent } from "./PdfTranscriptionEvent";
export type { LogLevelEvent } from "./LogLevelEvent";
export type { SessionStateEvent } from "./SessionStateEvent";
export type { ReaderStateEvent } from "./ReaderStateEvent";
export type { BridgeError } from "./BridgeError";
export type { PanelState } from "./PanelState";
export type { ReaderSettingsView } from "./ReaderSettingsView";
export type { ReaderTtsView } from "./ReaderTtsView";
export type { ReaderSettingsPatch } from "./ReaderSettingsPatch";
export type { ReaderStats } from "./ReaderStats";
export type { ReaderSnapshot } from "./ReaderSnapshot";
export type { TtsPlaybackState } from "./TtsPlaybackState";
export type { ThemeMode } from "./ThemeMode";
export type { FontFamily } from "./FontFamily";
export type { FontWeight } from "./FontWeight";
export type { HighlightColor } from "./HighlightColor";
"#;

    fs::write(out_dir.join("index.ts"), index_content).map_err(|err| {
        format!(
            "Failed to write {}: {err}",
            out_dir.join("index.ts").display()
        )
    })?;

    Ok(())
}

fn persist_active_reader(state: &mut BackendState) {
    if let Some(reader) = &state.reader {
        session::persist_session_housekeeping(reader);
    }
}

fn cleanup_for_shutdown(state: &mut BackendState) -> Option<u64> {
    let cancelled_open_request = if state.open_in_flight {
        state.active_open_request
    } else {
        None
    };
    if let Some(reader) = state.reader.as_mut() {
        reader.tts_stop();
    }
    persist_active_reader(state);
    state.reader = None;
    state.mode = UiMode::Starter;
    state.active_source_path = None;
    state.active_open_source_path = None;
    state.open_in_flight = false;
    state.active_open_request = None;
    cancelled_open_request
}

fn finalize_shutdown_from_mutex(state: &Mutex<BackendState>) {
    match state.lock() {
        Ok(mut guard) => {
            let _ = cleanup_for_shutdown(&mut guard);
        }
        Err(_) => warn!("Skipping shutdown housekeeping: backend state lock poisoned"),
    }
}

fn allocate_request_id(state: &mut BackendState) -> u64 {
    let request_id = state.next_request_id;
    state.next_request_id = state.next_request_id.wrapping_add(1).max(1);
    request_id
}

fn begin_open_request(state: &mut BackendState, source_path: &Path) -> Result<u64, BridgeError> {
    if state.open_in_flight {
        return Err(bridge_error(
            "operation_conflict",
            "A book open operation is already in progress",
        ));
    }
    let request_id = allocate_request_id(state);
    state.open_in_flight = true;
    state.active_open_request = Some(request_id);
    state.active_open_source_path = Some(source_path.to_path_buf());
    Ok(request_id)
}

fn emit_session_state(
    app: &tauri::AppHandle,
    request_id: u64,
    action: &str,
    session: &SessionState,
) {
    let _ = app.emit(
        "session-state",
        SessionStateEvent {
            request_id,
            action: action.to_string(),
            session: session.clone(),
        },
    );
}

fn emit_reader_state(
    app: &tauri::AppHandle,
    request_id: u64,
    action: &str,
    reader: &session::ReaderSnapshot,
) {
    let _ = app.emit(
        "reader-state",
        ReaderStateEvent {
            request_id,
            action: action.to_string(),
            reader: reader.clone(),
        },
    );
}

fn emit_tts_state(
    app: &tauri::AppHandle,
    request_id: u64,
    action: &str,
    tts: &session::ReaderTtsView,
) {
    let _ = app.emit(
        "tts-state",
        TtsStateEvent {
            request_id,
            action: action.to_string(),
            tts: tts.clone(),
        },
    );
}

fn apply_reader_update<F>(
    app: &tauri::AppHandle,
    state: &State<'_, Mutex<BackendState>>,
    action: &str,
    update: F,
) -> Result<session::ReaderSnapshot, BridgeError>
where
    F: FnOnce(&mut session::ReaderSession, &normalizer::TextNormalizer),
{
    let (snapshot, request_id) = {
        let mut guard = state
            .lock()
            .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
        let normalizer = guard.normalizer.clone();
        let panels = guard.panels;
        let request_id = allocate_request_id(&mut guard);
        let reader = guard
            .reader
            .as_mut()
            .ok_or_else(|| bridge_error("no_reader", "No active reader session"))?;
        update(reader, &normalizer);
        (reader.snapshot(panels, &normalizer), request_id)
    };
    emit_reader_state(app, request_id, action, &snapshot);
    emit_tts_state(app, request_id, action, &snapshot.tts);
    Ok(snapshot)
}

fn apply_panel_toggle<F>(
    app: &tauri::AppHandle,
    state: &State<'_, Mutex<BackendState>>,
    action: &str,
    toggle: F,
) -> Result<SessionState, BridgeError>
where
    F: FnOnce(&mut session::PanelState),
{
    let (session, reader_snapshot, request_id) = {
        let mut guard = state
            .lock()
            .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
        let request_id = allocate_request_id(&mut guard);
        toggle(&mut guard.panels);

        let session = to_session_state(&guard);
        let normalizer = guard.normalizer.clone();
        let panels = guard.panels;
        let reader_snapshot = guard
            .reader
            .as_mut()
            .map(|reader| reader.snapshot(panels, &normalizer));
        (session, reader_snapshot, request_id)
    };

    emit_session_state(app, request_id, action, &session);
    if let Some(snapshot) = &reader_snapshot {
        emit_reader_state(app, request_id, action, snapshot);
        emit_tts_state(app, request_id, action, &snapshot.tts);
    }
    Ok(session)
}

async fn open_resolved_source(
    app: &tauri::AppHandle,
    state: &State<'_, Mutex<BackendState>>,
    source_path: PathBuf,
) -> Result<OpenSourceResult, BridgeError> {
    let (request_id, started_session): (u64, SessionState) = {
        let mut guard = state
            .lock()
            .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
        let request_id = begin_open_request(&mut guard, &source_path)?;
        let started_session = to_session_state(&guard);
        (request_id, started_session)
    };

    emit_session_state(app, request_id, "source_open_started", &started_session);

    let source_is_pdf = source_path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("pdf"))
        .unwrap_or(false);

    info!(
        request_id,
        path = %source_path.display(),
        "Starting source open request"
    );

    let _ = app.emit(
        "source-open",
        SourceOpenEvent {
            request_id,
            phase: "started".to_string(),
            source_path: Some(source_path.to_string_lossy().to_string()),
            message: None,
        },
    );

    if source_is_pdf {
        let _ = app.emit(
            "pdf-transcription",
            PdfTranscriptionEvent {
                request_id,
                phase: "started".to_string(),
                source_path: source_path.to_string_lossy().to_string(),
                message: None,
            },
        );
    }

    cache::remember_source_path(&source_path);

    let (base_config, normalizer) = {
        let guard = state
            .lock()
            .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
        (guard.base_config.clone(), guard.normalizer.clone())
    };

    let source_path_for_task = source_path.clone();
    let normalizer_for_task = normalizer.clone();
    let reader_result = tauri::async_runtime::spawn_blocking(move || {
        session::load_session_for_source(source_path_for_task, &base_config, &normalizer_for_task)
    })
    .await;

    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    if guard.active_open_request != Some(request_id) {
        let should_emit_cancelled = guard.open_in_flight || guard.active_open_source_path.is_some();
        drop(guard);
        if should_emit_cancelled {
            let _ = app.emit(
                "source-open",
                SourceOpenEvent {
                    request_id,
                    phase: "cancelled".to_string(),
                    source_path: Some(source_path.to_string_lossy().to_string()),
                    message: Some("Source open request was superseded or cancelled".to_string()),
                },
            );
            if source_is_pdf {
                let _ = app.emit(
                    "pdf-transcription",
                    PdfTranscriptionEvent {
                        request_id,
                        phase: "cancelled".to_string(),
                        source_path: source_path.to_string_lossy().to_string(),
                        message: Some(
                            "PDF transcription cancelled by request supersession".to_string(),
                        ),
                    },
                );
            }
        }
        info!(
            request_id,
            path = %source_path.display(),
            "Discarded stale source open completion"
        );
        return Err(bridge_error(
            "open_cancelled",
            "Source open request was superseded or cancelled",
        ));
    }
    guard.open_in_flight = false;
    guard.active_open_request = None;
    guard.active_open_source_path = None;
    let reader_result = match reader_result {
        Ok(result) => result,
        Err(err) => {
            let session = to_session_state(&guard);
            drop(guard);
            emit_session_state(app, request_id, "source_open_failed", &session);
            let message = format!("Failed to join load task: {err}");
            warn!(
                request_id,
                path = %source_path.display(),
                error = %message,
                "Source open request task failed"
            );
            let _ = app.emit(
                "source-open",
                SourceOpenEvent {
                    request_id,
                    phase: "failed".to_string(),
                    source_path: Some(source_path.to_string_lossy().to_string()),
                    message: Some(message.clone()),
                },
            );
            if source_is_pdf {
                let _ = app.emit(
                    "pdf-transcription",
                    PdfTranscriptionEvent {
                        request_id,
                        phase: "failed".to_string(),
                        source_path: source_path.to_string_lossy().to_string(),
                        message: Some(message.clone()),
                    },
                );
            }
            return Err(bridge_error("task_join_error", message));
        }
    };

    match reader_result {
        Ok(mut reader) => {
            let reader_panels = session::PanelState {
                show_settings: reader.config.show_settings,
                show_stats: false,
                show_tts: reader.config.show_tts,
            };
            guard.panels = reader_panels;
            let snapshot = reader.snapshot(reader_panels, &normalizer);

            guard.mode = UiMode::Reader;
            guard.active_source_path = Some(source_path.clone());
            guard.reader = Some(reader);
            let session = to_session_state(&guard);
            let result = OpenSourceResult {
                session: session.clone(),
                reader: snapshot.clone(),
            };

            drop(guard);
            emit_session_state(app, request_id, "source_open", &session);
            emit_reader_state(app, request_id, "source_open", &snapshot);
            emit_tts_state(app, request_id, "source_open", &snapshot.tts);

            let _ = app.emit(
                "source-open",
                SourceOpenEvent {
                    request_id,
                    phase: "finished".to_string(),
                    source_path: Some(source_path.to_string_lossy().to_string()),
                    message: None,
                },
            );
            if source_is_pdf {
                let _ = app.emit(
                    "pdf-transcription",
                    PdfTranscriptionEvent {
                        request_id,
                        phase: "finished".to_string(),
                        source_path: source_path.to_string_lossy().to_string(),
                        message: None,
                    },
                );
            }
            info!(
                request_id,
                path = %source_path.display(),
                page = snapshot.current_page + 1,
                total_pages = snapshot.total_pages,
                "Completed source open request"
            );
            Ok(result)
        }
        Err(err) => {
            let session = to_session_state(&guard);
            drop(guard);
            emit_session_state(app, request_id, "source_open_failed", &session);
            warn!(
                request_id,
                path = %source_path.display(),
                error = %err,
                "Source open request failed"
            );
            let _ = app.emit(
                "source-open",
                SourceOpenEvent {
                    request_id,
                    phase: "failed".to_string(),
                    source_path: Some(source_path.to_string_lossy().to_string()),
                    message: Some(err.clone()),
                },
            );
            if source_is_pdf {
                let _ = app.emit(
                    "pdf-transcription",
                    PdfTranscriptionEvent {
                        request_id,
                        phase: "failed".to_string(),
                        source_path: source_path.to_string_lossy().to_string(),
                        message: Some(err.clone()),
                    },
                );
            }
            Err(bridge_error("open_failed", err))
        }
    }
}

#[tauri::command]
fn session_get_bootstrap(
    state: State<'_, Mutex<BackendState>>,
) -> Result<BootstrapState, BridgeError> {
    let guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    Ok(BootstrapState {
        app_name: "ebup-viewer".to_string(),
        mode: "migration".to_string(),
        config: BootstrapConfig {
            theme: guard.base_config.theme,
            font_family: guard.base_config.font_family,
            font_weight: guard.base_config.font_weight,
            day_highlight: guard.base_config.day_highlight,
            night_highlight: guard.base_config.night_highlight,
            log_level: guard.base_config.log_level.as_filter_str().to_string(),
            default_font_size: guard.base_config.font_size,
            default_lines_per_page: guard.base_config.lines_per_page,
            default_tts_speed: guard.base_config.tts_speed,
            default_pause_after_sentence: guard.base_config.pause_after_sentence,
            key_toggle_play_pause: guard.base_config.key_toggle_play_pause.clone(),
            key_next_sentence: guard.base_config.key_next_sentence.clone(),
            key_prev_sentence: guard.base_config.key_prev_sentence.clone(),
            key_repeat_sentence: guard.base_config.key_repeat_sentence.clone(),
            key_toggle_search: guard.base_config.key_toggle_search.clone(),
            key_safe_quit: guard.base_config.key_safe_quit.clone(),
            key_toggle_settings: guard.base_config.key_toggle_settings.clone(),
            key_toggle_stats: guard.base_config.key_toggle_stats.clone(),
            key_toggle_tts: guard.base_config.key_toggle_tts.clone(),
        },
    })
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
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<SessionState, BridgeError> {
    let (session, request_id, cancelled_request, cancelled_source_path) = {
        let mut guard = state
            .lock()
            .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
        let request_id = allocate_request_id(&mut guard);
        let cancelled_request = if guard.open_in_flight {
            guard.active_open_request
        } else {
            None
        };
        let cancelled_source_path = guard
            .active_open_source_path
            .as_ref()
            .map(|path| path.to_string_lossy().to_string());
        let _ = cleanup_for_shutdown(&mut guard);
        (
            to_session_state(&guard),
            request_id,
            cancelled_request,
            cancelled_source_path,
        )
    };
    emit_session_state(&app, request_id, "session_return_to_starter", &session);
    if let Some(cancelled_request) = cancelled_request {
        let _ = app.emit(
            "source-open",
            SourceOpenEvent {
                request_id: cancelled_request,
                phase: "cancelled".to_string(),
                source_path: cancelled_source_path,
                message: Some("Source open request cancelled by return-to-starter".to_string()),
            },
        );
    }
    Ok(session)
}

#[tauri::command]
fn panel_toggle_settings(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<SessionState, BridgeError> {
    apply_panel_toggle(&app, &state, "panel_toggle_settings", |panels| {
        panels.show_settings = !panels.show_settings;
        if panels.show_settings {
            panels.show_stats = false;
        }
    })
}

#[tauri::command]
fn panel_toggle_stats(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<SessionState, BridgeError> {
    apply_panel_toggle(&app, &state, "panel_toggle_stats", |panels| {
        panels.show_stats = !panels.show_stats;
        if panels.show_stats {
            panels.show_settings = false;
        }
    })
}

#[tauri::command]
fn panel_toggle_tts(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<SessionState, BridgeError> {
    apply_panel_toggle(&app, &state, "panel_toggle_tts", |panels| {
        panels.show_tts = !panels.show_tts;
    })
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
async fn source_open_path(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
    path: String,
) -> Result<OpenSourceResult, BridgeError> {
    let source = resolve_source_path(&path)?;
    open_resolved_source(&app, &state, source).await
}

#[tauri::command]
async fn source_open_clipboard_text(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
    text: String,
) -> Result<OpenSourceResult, BridgeError> {
    let trimmed = text.trim().to_string();
    if trimmed.is_empty() {
        return Err(bridge_error("invalid_input", "clipboard text is empty"));
    }
    let path = cache::persist_clipboard_text_source(&trimmed)
        .map_err(|err| bridge_error("invalid_input", err))?;
    open_resolved_source(&app, &state, path).await
}

#[tauri::command]
fn reader_get_snapshot(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    apply_reader_update(&app, &state, "reader_get_snapshot", |_, _| {})
}

#[tauri::command]
fn reader_next_page(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    apply_reader_update(&app, &state, "reader_next_page", |reader, normalizer| {
        reader.next_page(normalizer);
    })
}

#[tauri::command]
fn reader_prev_page(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    apply_reader_update(&app, &state, "reader_prev_page", |reader, normalizer| {
        reader.prev_page(normalizer);
    })
}

#[tauri::command]
fn reader_set_page(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
    page: usize,
) -> Result<session::ReaderSnapshot, BridgeError> {
    apply_reader_update(&app, &state, "reader_set_page", |reader, normalizer| {
        reader.set_page(page, normalizer);
    })
}

#[tauri::command]
fn reader_sentence_click(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
    sentence_idx: usize,
) -> Result<session::ReaderSnapshot, BridgeError> {
    apply_reader_update(
        &app,
        &state,
        "reader_sentence_click",
        |reader, normalizer| {
            reader.sentence_click(sentence_idx, normalizer);
        },
    )
}

#[tauri::command]
fn reader_next_sentence(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    apply_reader_update(
        &app,
        &state,
        "reader_next_sentence",
        |reader, normalizer| {
            reader.select_next_sentence(normalizer);
        },
    )
}

#[tauri::command]
fn reader_prev_sentence(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    apply_reader_update(
        &app,
        &state,
        "reader_prev_sentence",
        |reader, normalizer| {
            reader.select_prev_sentence(normalizer);
        },
    )
}

#[tauri::command]
fn reader_toggle_text_only(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    apply_reader_update(
        &app,
        &state,
        "reader_toggle_text_only",
        |reader, normalizer| {
            reader.toggle_text_only(normalizer);
        },
    )
}

#[tauri::command]
fn reader_apply_settings(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
    patch: session::ReaderSettingsPatch,
) -> Result<session::ReaderSnapshot, BridgeError> {
    apply_reader_update(
        &app,
        &state,
        "reader_apply_settings",
        |reader, normalizer| {
            reader.apply_settings_patch(patch, normalizer);
        },
    )
}

#[tauri::command]
fn reader_search_set_query(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
    query: String,
) -> Result<session::ReaderSnapshot, BridgeError> {
    apply_reader_update(
        &app,
        &state,
        "reader_search_set_query",
        |reader, normalizer| {
            reader.set_search_query(query, normalizer);
        },
    )
}

#[tauri::command]
fn reader_search_next(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    apply_reader_update(&app, &state, "reader_search_next", |reader, normalizer| {
        reader.search_next(normalizer);
    })
}

#[tauri::command]
fn reader_search_prev(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    apply_reader_update(&app, &state, "reader_search_prev", |reader, normalizer| {
        reader.search_prev(normalizer);
    })
}

#[tauri::command]
fn reader_tts_play(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    apply_reader_update(&app, &state, "reader_tts_play", |reader, normalizer| {
        reader.tts_play(normalizer);
    })
}

#[tauri::command]
fn reader_tts_pause(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    apply_reader_update(&app, &state, "reader_tts_pause", |reader, _| {
        reader.tts_pause();
    })
}

#[tauri::command]
fn reader_tts_toggle_play_pause(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    apply_reader_update(
        &app,
        &state,
        "reader_tts_toggle_play_pause",
        |reader, normalizer| {
            reader.tts_toggle_play_pause(normalizer);
        },
    )
}

#[tauri::command]
fn reader_tts_play_from_page_start(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    apply_reader_update(
        &app,
        &state,
        "reader_tts_play_from_page_start",
        |reader, normalizer| {
            reader.tts_play_from_page_start(normalizer);
        },
    )
}

#[tauri::command]
fn reader_tts_play_from_highlight(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    apply_reader_update(
        &app,
        &state,
        "reader_tts_play_from_highlight",
        |reader, normalizer| {
            reader.tts_play_from_highlight(normalizer);
        },
    )
}

#[tauri::command]
fn reader_tts_seek_next(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    apply_reader_update(
        &app,
        &state,
        "reader_tts_seek_next",
        |reader, normalizer| {
            reader.tts_seek_next(normalizer);
        },
    )
}

#[tauri::command]
fn reader_tts_seek_prev(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    apply_reader_update(
        &app,
        &state,
        "reader_tts_seek_prev",
        |reader, normalizer| {
            reader.tts_seek_prev(normalizer);
        },
    )
}

#[tauri::command]
fn reader_tts_repeat_sentence(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    apply_reader_update(
        &app,
        &state,
        "reader_tts_repeat_sentence",
        |reader, normalizer| {
            reader.tts_repeat_current_sentence(normalizer);
        },
    )
}

#[tauri::command]
fn reader_close_session(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<SessionState, BridgeError> {
    let (session, request_id, cancelled_request, cancelled_source_path) = {
        let mut guard = state
            .lock()
            .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
        let request_id = allocate_request_id(&mut guard);
        let cancelled_request = if guard.open_in_flight {
            guard.active_open_request
        } else {
            None
        };
        let cancelled_source_path = guard
            .active_open_source_path
            .as_ref()
            .map(|path| path.to_string_lossy().to_string());
        let _ = cleanup_for_shutdown(&mut guard);
        (
            to_session_state(&guard),
            request_id,
            cancelled_request,
            cancelled_source_path,
        )
    };
    emit_session_state(&app, request_id, "reader_close_session", &session);
    if let Some(cancelled_request) = cancelled_request {
        let _ = app.emit(
            "source-open",
            SourceOpenEvent {
                request_id: cancelled_request,
                phase: "cancelled".to_string(),
                source_path: cancelled_source_path,
                message: Some("Source open request cancelled by session close".to_string()),
            },
        );
    }
    Ok(session)
}

#[tauri::command]
fn app_safe_quit(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
) -> Result<(), BridgeError> {
    finalize_shutdown_from_mutex(state.inner());
    app.exit(0);
    Ok(())
}

#[tauri::command]
fn logging_set_level(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
    level: String,
) -> Result<String, BridgeError> {
    let parsed = parse_log_level_label(&level).ok_or_else(|| {
        bridge_error(
            "invalid_input",
            format!("Unsupported log level '{level}'. Use trace/debug/info/warn/error."),
        )
    })?;

    let request_id = {
        let mut guard = state
            .lock()
            .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
        let request_id = allocate_request_id(&mut guard);
        guard.base_config.log_level = parsed;
        let config_path = app_config_path();
        persist_base_config(&guard.base_config, &config_path)?;
        request_id
    };

    tauri_plugin_log::log::set_max_level(log_level_to_filter(parsed));
    let level_label = parsed.as_filter_str().to_string();
    let _ = app.emit(
        "log-level",
        LogLevelEvent {
            request_id,
            level: level_label.clone(),
        },
    );
    info!(request_id, level = %level_label, "Updated runtime log level");
    Ok(level_label)
}

#[tauri::command]
async fn calibre_load_books(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
    force_refresh: Option<bool>,
) -> Result<Vec<CalibreBookDto>, BridgeError> {
    let force_refresh = force_refresh.unwrap_or(false);
    let (config, request_id) = {
        let mut guard = state
            .lock()
            .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
        let request_id = allocate_request_id(&mut guard);
        (guard.calibre_config.clone(), request_id)
    };

    info!(request_id, force_refresh, "Starting calibre load request");

    let _ = app.emit(
        "calibre-load",
        CalibreLoadEvent {
            request_id,
            phase: "started".to_string(),
            count: None,
            message: None,
        },
    );

    let books_result =
        tauri::async_runtime::spawn_blocking(move || calibre::load_books(&config, force_refresh))
            .await
            .map_err(|err| {
                bridge_error(
                    "task_join_error",
                    format!("Failed to join calibre task: {err}"),
                )
            })
            .and_then(|result| {
                result.map_err(|err| bridge_error("calibre_load_failed", err.to_string()))
            });

    let books = match books_result {
        Ok(books) => books,
        Err(err) => {
            warn!(
                request_id,
                force_refresh,
                error = %err.message,
                "Calibre load request failed"
            );
            let _ = app.emit(
                "calibre-load",
                CalibreLoadEvent {
                    request_id,
                    phase: "failed".to_string(),
                    count: None,
                    message: Some(err.message.clone()),
                },
            );
            return Err(err);
        }
    };

    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    guard.calibre_books = books.clone();

    let _ = app.emit(
        "calibre-load",
        CalibreLoadEvent {
            request_id,
            phase: "finished".to_string(),
            count: Some(books.len()),
            message: None,
        },
    );
    info!(
        request_id,
        force_refresh,
        count = books.len(),
        "Completed calibre load request"
    );

    Ok(books.into_iter().map(map_calibre_book).collect())
}

#[tauri::command]
async fn calibre_open_book(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
    book_id: u64,
) -> Result<OpenSourceResult, BridgeError> {
    let (book, calibre_config) = {
        let guard = state
            .lock()
            .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
        let book = guard
            .calibre_books
            .iter()
            .find(|book| book.id == book_id)
            .cloned()
            .ok_or_else(|| {
                bridge_error("not_found", format!("Unknown calibre book id={book_id}"))
            })?;
        (book, guard.calibre_config.clone())
    };

    let path = tauri::async_runtime::spawn_blocking(move || {
        calibre::materialize_book_path(&calibre_config, &book)
    })
    .await
    .map_err(|err| {
        bridge_error(
            "task_join_error",
            format!("Failed to join calibre-open task: {err}"),
        )
    })?
    .map_err(|err| bridge_error("calibre_open_failed", err.to_string()))?;

    open_resolved_source(&app, &state, path).await
}

#[cfg(target_os = "linux")]
fn configure_linux_display_backend() {
    let wayland_display = std::env::var("WAYLAND_DISPLAY").ok();
    let xdg_session_type = std::env::var("XDG_SESSION_TYPE")
        .ok()
        .map(|value| value.to_ascii_lowercase());
    let x_display = std::env::var("DISPLAY").ok();
    let wayland_available = wayland_display.is_some()
        || matches!(
            xdg_session_type.clone(),
            Some(value) if value == "wayland"
        );
    let allow_x11 = matches!(
        std::env::var("EBUP_VIEWER_ALLOW_X11")
            .ok()
            .map(|value| value.to_ascii_lowercase()),
        Some(value) if value == "1" || value == "true" || value == "yes"
    );

    if !wayland_available || allow_x11 {
        info!(
            wayland_display = ?wayland_display,
            xdg_session_type = ?xdg_session_type,
            x_display = ?x_display,
            allow_x11,
            "Skipping Wayland-first backend override"
        );
        return;
    }

    let current_gdk_backend = std::env::var("GDK_BACKEND")
        .ok()
        .map(|value| value.to_ascii_lowercase());
    let current_winit_backend = std::env::var("WINIT_UNIX_BACKEND").ok();
    let prefer_x11_first = x_display.is_some() && wayland_display.is_some();
    let desired_gdk_backend = if prefer_x11_first {
        "x11,wayland"
    } else {
        "wayland,x11"
    };

    // Prefer Wayland but include X11 fallback so startup does not hard-fail when Wayland is present
    // but runtime-incompatible on this machine/session.
    if current_gdk_backend.as_deref() != Some(desired_gdk_backend) {
        // SAFETY: startup-time process env initialization before Tauri runtime threads start.
        unsafe {
            std::env::set_var("GDK_BACKEND", desired_gdk_backend);
        }
    }
    if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none() {
        // SAFETY: startup-time process env initialization before Tauri runtime threads start.
        unsafe {
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        }
    }

    info!(
        wayland_display = ?wayland_display,
        xdg_session_type = ?xdg_session_type,
        x_display = ?x_display,
        gdk_backend = desired_gdk_backend,
        winit_backend = ?current_winit_backend,
        webkit_disable_dmabuf_renderer = ?std::env::var("WEBKIT_DISABLE_DMABUF_RENDERER").ok(),
        "Configured Linux display backend defaults with safe fallback ordering"
    );
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(target_os = "linux")]
    configure_linux_display_backend();

    let startup_config = config::load_config(&app_config_path());
    let log_plugin = tauri_plugin_log::Builder::new()
        .level(log_level_to_filter(startup_config.log_level))
        .target(Target::new(TargetKind::Stdout))
        .target(Target::new(TargetKind::Webview))
        .build();

    info!("Starting ebup-viewer tauri bridge");
    let builder = tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();
            if let Err(err) = ctrlc::set_handler(move || {
                info!("Received Ctrl+C; running safe shutdown housekeeping");
                let state = app_handle.state::<Mutex<BackendState>>();
                finalize_shutdown_from_mutex(state.inner());
                app_handle.exit(130);
            }) {
                warn!("Failed to install Ctrl+C signal handler: {err}");
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            if matches!(event, tauri::WindowEvent::CloseRequested { .. }) {
                let state = window.app_handle().state::<Mutex<BackendState>>();
                finalize_shutdown_from_mutex(state.inner());
            }
        })
        .manage(Mutex::new(BackendState::new()))
        .plugin(log_plugin)
        .invoke_handler(tauri::generate_handler![
            session_get_bootstrap,
            session_get_state,
            session_return_to_starter,
            panel_toggle_settings,
            panel_toggle_stats,
            panel_toggle_tts,
            recent_list,
            recent_delete,
            source_open_path,
            source_open_clipboard_text,
            reader_get_snapshot,
            reader_next_page,
            reader_prev_page,
            reader_set_page,
            reader_sentence_click,
            reader_next_sentence,
            reader_prev_sentence,
            reader_toggle_text_only,
            reader_apply_settings,
            reader_search_set_query,
            reader_search_next,
            reader_search_prev,
            reader_tts_play,
            reader_tts_pause,
            reader_tts_toggle_play_pause,
            reader_tts_play_from_page_start,
            reader_tts_play_from_highlight,
            reader_tts_seek_next,
            reader_tts_seek_prev,
            reader_tts_repeat_sentence,
            reader_close_session,
            app_safe_quit,
            logging_set_level,
            calibre_load_books,
            calibre_open_book
        ]);

    if let Err(err) = builder.run(tauri::generate_context!()) {
        warn!("tauri runtime failed: {err}");
        panic!("tauri runtime failed: {err}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_file(name: &str, extension: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("ebup_viewer_test_{name}_{nanos}.{extension}"))
    }

    #[test]
    fn bootstrap_state_roundtrips_json_contract() {
        let state = BootstrapState {
            app_name: "ebup-viewer".to_string(),
            mode: "migration".to_string(),
            config: BootstrapConfig {
                theme: config::ThemeMode::Day,
                font_family: config::FontFamily::Lexend,
                font_weight: config::FontWeight::Bold,
                day_highlight: config::HighlightColor {
                    r: 0.2,
                    g: 0.4,
                    b: 0.7,
                    a: 0.15,
                },
                night_highlight: config::HighlightColor {
                    r: 0.8,
                    g: 0.8,
                    b: 0.5,
                    a: 0.2,
                },
                log_level: "debug".to_string(),
                default_font_size: 22,
                default_lines_per_page: 700,
                default_tts_speed: 2.5,
                default_pause_after_sentence: 0.06,
                key_toggle_play_pause: "space".to_string(),
                key_next_sentence: "f".to_string(),
                key_prev_sentence: "s".to_string(),
                key_repeat_sentence: "r".to_string(),
                key_toggle_search: "ctrl+f".to_string(),
                key_safe_quit: "q".to_string(),
                key_toggle_settings: "ctrl+t".to_string(),
                key_toggle_stats: "ctrl+g".to_string(),
                key_toggle_tts: "ctrl+y".to_string(),
            },
        };

        let json = serde_json::to_string(&state).expect("serialize bootstrap");
        let decoded: BootstrapState = serde_json::from_str(&json).expect("deserialize bootstrap");
        assert_eq!(decoded.config.default_font_size, 22);
        assert_eq!(decoded.config.theme, config::ThemeMode::Day);
        assert_eq!(decoded.config.key_toggle_tts, "ctrl+y");
    }

    #[test]
    fn session_state_roundtrips_json_contract() {
        let state = SessionState {
            mode: UiMode::Reader,
            active_source_path: Some("/tmp/book.epub".to_string()),
            open_in_flight: false,
            panels: session::PanelState {
                show_settings: true,
                show_stats: false,
                show_tts: true,
            },
        };
        let json = serde_json::to_string(&state).expect("serialize session");
        let decoded: SessionState = serde_json::from_str(&json).expect("deserialize session");
        assert!(matches!(decoded.mode, UiMode::Reader));
        assert_eq!(
            decoded.active_source_path.as_deref(),
            Some("/tmp/book.epub")
        );
        assert!(decoded.panels.show_tts);
    }

    #[test]
    fn event_contracts_include_request_ids() {
        let source = SourceOpenEvent {
            request_id: 42,
            phase: "started".to_string(),
            source_path: Some("/tmp/book.epub".to_string()),
            message: None,
        };
        let source_json = serde_json::to_value(source).expect("serialize source event");
        assert_eq!(
            source_json.get("request_id").and_then(|v| v.as_u64()),
            Some(42)
        );

        let calibre = CalibreLoadEvent {
            request_id: 43,
            phase: "finished".to_string(),
            count: Some(123),
            message: None,
        };
        let calibre_json = serde_json::to_value(calibre).expect("serialize calibre event");
        assert_eq!(
            calibre_json.get("request_id").and_then(|v| v.as_u64()),
            Some(43)
        );
        assert_eq!(
            calibre_json.get("count").and_then(|v| v.as_u64()),
            Some(123)
        );

        let session_event = SessionStateEvent {
            request_id: 44,
            action: "reader_close_session".to_string(),
            session: SessionState {
                mode: UiMode::Starter,
                active_source_path: None,
                open_in_flight: false,
                panels: session::PanelState {
                    show_settings: true,
                    show_stats: false,
                    show_tts: true,
                },
            },
        };
        let session_json = serde_json::to_value(session_event).expect("serialize session event");
        assert_eq!(
            session_json.get("request_id").and_then(|v| v.as_u64()),
            Some(44)
        );

        let reader_event = ReaderStateEvent {
            request_id: 45,
            action: "reader_next_page".to_string(),
            reader: session::ReaderSnapshot {
                source_path: "/tmp/book.epub".to_string(),
                source_name: "book.epub".to_string(),
                current_page: 0,
                total_pages: 1,
                text_only_mode: false,
                page_text: "hello".to_string(),
                sentences: vec!["hello".to_string()],
                highlighted_sentence_idx: Some(0),
                search_query: String::new(),
                search_matches: vec![],
                selected_search_match: None,
                settings: session::ReaderSettingsView {
                    theme: config::ThemeMode::Day,
                    day_highlight: config::HighlightColor {
                        r: 0.2,
                        g: 0.4,
                        b: 0.7,
                        a: 0.15,
                    },
                    night_highlight: config::HighlightColor {
                        r: 0.8,
                        g: 0.8,
                        b: 0.5,
                        a: 0.2,
                    },
                    font_size: 22,
                    line_spacing: 1.2,
                    margin_horizontal: 100,
                    margin_vertical: 12,
                    lines_per_page: 700,
                    pause_after_sentence: 0.06,
                    auto_scroll_tts: false,
                    center_spoken_sentence: true,
                    tts_speed: 2.5,
                    tts_volume: 1.0,
                },
                tts: session::ReaderTtsView {
                    state: session::TtsPlaybackState::Idle,
                    current_sentence_idx: Some(0),
                    sentence_count: 1,
                    can_seek_prev: false,
                    can_seek_next: false,
                    progress_pct: 0.0,
                },
                stats: session::ReaderStats {
                    page_index: 1,
                    total_pages: 1,
                    tts_progress_pct: 0.0,
                    page_time_remaining_secs: 0.0,
                    book_time_remaining_secs: 0.0,
                    page_word_count: 1,
                    page_sentence_count: 1,
                    page_start_percent: 0.0,
                    page_end_percent: 100.0,
                    words_read_up_to_page_start: 0,
                    sentences_read_up_to_page_start: 0,
                    words_read_up_to_page_end: 1,
                    sentences_read_up_to_page_end: 1,
                    words_read_up_to_current_position: 1,
                    sentences_read_up_to_current_position: 1,
                },
                panels: session::PanelState {
                    show_settings: true,
                    show_stats: false,
                    show_tts: true,
                },
            },
        };
        let reader_json = serde_json::to_value(reader_event).expect("serialize reader event");
        assert_eq!(
            reader_json.get("request_id").and_then(|v| v.as_u64()),
            Some(45)
        );
    }

    #[test]
    fn normalize_recent_limit_clamps_to_expected_bounds() {
        assert_eq!(normalize_recent_limit(None), DEFAULT_RECENT_LIMIT);
        assert_eq!(normalize_recent_limit(Some(0)), 1);
        assert_eq!(normalize_recent_limit(Some(1)), 1);
        assert_eq!(
            normalize_recent_limit(Some(MAX_RECENT_LIMIT + 123)),
            MAX_RECENT_LIMIT
        );
    }

    #[test]
    fn supported_source_extensions_match_contract() {
        assert!(is_supported_source(Path::new("/tmp/book.epub")));
        assert!(is_supported_source(Path::new("/tmp/book.PDF")));
        assert!(is_supported_source(Path::new("/tmp/book.txt")));
        assert!(is_supported_source(Path::new("/tmp/book.md")));
        assert!(is_supported_source(Path::new("/tmp/book.markdown")));
        assert!(!is_supported_source(Path::new("/tmp/book.docx")));
    }

    #[test]
    fn resolve_source_path_returns_expected_error_codes() {
        let empty = resolve_source_path("   ").expect_err("empty input must fail");
        assert_eq!(empty.code, "invalid_input");

        let missing = resolve_source_path("/tmp/this/path/does/not/exist.epub")
            .expect_err("missing source must fail");
        assert_eq!(missing.code, "not_found");

        let unsupported = unique_temp_file("unsupported", "docx");
        fs::write(&unsupported, "hello world").expect("write temp file");
        let err = resolve_source_path(unsupported.to_string_lossy().as_ref())
            .expect_err("unsupported extension must fail");
        assert_eq!(err.code, "unsupported_source");
        let _ = fs::remove_file(unsupported);
    }

    #[test]
    fn parse_log_level_label_accepts_supported_values() {
        assert_eq!(
            parse_log_level_label("trace"),
            Some(config::LogLevel::Trace)
        );
        assert_eq!(
            parse_log_level_label("DEBUG"),
            Some(config::LogLevel::Debug)
        );
        assert_eq!(parse_log_level_label("info"), Some(config::LogLevel::Info));
        assert_eq!(
            parse_log_level_label("warning"),
            Some(config::LogLevel::Warn)
        );
        assert_eq!(parse_log_level_label("warn"), Some(config::LogLevel::Warn));
        assert_eq!(
            parse_log_level_label("error"),
            Some(config::LogLevel::Error)
        );
        assert_eq!(parse_log_level_label("verbose"), None);
    }

    #[test]
    fn app_config_path_uses_override_env_when_present() {
        let key = "EBUP_VIEWER_CONFIG_PATH";
        let previous = std::env::var_os(key);
        let override_path = unique_temp_file("config_override_path", "toml");
        // SAFETY: test-scoped env mutation; restored before test exits.
        unsafe {
            std::env::set_var(key, &override_path);
        }
        assert_eq!(app_config_path(), override_path);
        match previous {
            Some(value) => {
                // SAFETY: test-scoped env mutation restore.
                unsafe {
                    std::env::set_var(key, value);
                }
            }
            None => {
                // SAFETY: test-scoped env mutation restore.
                unsafe {
                    std::env::remove_var(key);
                }
            }
        }
    }

    #[test]
    fn persist_base_config_writes_updated_log_level() {
        let path = unique_temp_file("persist_base_config", "toml");
        let mut cfg = config::AppConfig::default();
        cfg.log_level = config::LogLevel::Warn;
        persist_base_config(&cfg, &path).expect("persist config");

        let loaded = config::load_config(&path);
        assert_eq!(loaded.log_level, config::LogLevel::Warn);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn cleanup_for_shutdown_clears_inflight_open_request() {
        let mut state = BackendState::new();
        state.mode = UiMode::Reader;
        state.active_source_path = Some(PathBuf::from("/tmp/active.epub"));
        state.active_open_source_path = Some(PathBuf::from("/tmp/opening.pdf"));
        state.open_in_flight = true;
        state.active_open_request = Some(77);

        let cancelled = cleanup_for_shutdown(&mut state);

        assert_eq!(cancelled, Some(77));
        assert!(matches!(state.mode, UiMode::Starter));
        assert!(state.active_source_path.is_none());
        assert!(state.active_open_source_path.is_none());
        assert!(!state.open_in_flight);
        assert!(state.active_open_request.is_none());
        assert!(state.reader.is_none());
    }

    #[test]
    fn cleanup_for_shutdown_without_inflight_open_returns_none() {
        let mut state = BackendState::new();
        state.mode = UiMode::Reader;
        state.active_source_path = Some(PathBuf::from("/tmp/active.epub"));
        state.open_in_flight = false;
        state.active_open_request = None;

        let cancelled = cleanup_for_shutdown(&mut state);

        assert_eq!(cancelled, None);
        assert!(matches!(state.mode, UiMode::Starter));
        assert!(state.active_source_path.is_none());
        assert!(!state.open_in_flight);
        assert!(state.active_open_request.is_none());
    }

    #[test]
    fn begin_open_request_rejects_duplicates_and_tracks_path() {
        let mut state = BackendState::new();
        let first_source = PathBuf::from("/tmp/first.epub");
        let second_source = PathBuf::from("/tmp/second.pdf");

        let request_id = begin_open_request(&mut state, &first_source).expect("first open request");
        assert_eq!(request_id, 1);
        assert!(state.open_in_flight);
        assert_eq!(state.active_open_request, Some(1));
        assert_eq!(
            state.active_open_source_path.as_deref(),
            Some(first_source.as_path())
        );

        let duplicate =
            begin_open_request(&mut state, &second_source).expect_err("duplicate open should fail");
        assert_eq!(duplicate.code, "operation_conflict");
        assert_eq!(state.active_open_request, Some(1));
        assert_eq!(
            state.active_open_source_path.as_deref(),
            Some(first_source.as_path())
        );
    }
}
