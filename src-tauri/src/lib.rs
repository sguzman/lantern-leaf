use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{Emitter, State};
use tauri_plugin_log::{Target, TargetKind, log::LevelFilter};
use tracing::{info, warn};

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
    key_next_sentence: String,
    key_prev_sentence: String,
    key_toggle_search: String,
    key_safe_quit: String,
    key_toggle_settings: String,
    key_toggle_stats: String,
    key_toggle_tts: String,
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
    panels: session::PanelState,
}

#[derive(Debug, Clone, Serialize)]
struct OpenSourceResult {
    session: SessionState,
    reader: session::ReaderSnapshot,
}

#[derive(Debug, Clone, Serialize)]
struct RecentBook {
    source_path: String,
    display_title: String,
    thumbnail_path: Option<String>,
    last_opened_unix_secs: u64,
}

#[derive(Debug, Clone, Serialize)]
struct CalibreBookDto {
    id: u64,
    title: String,
    extension: String,
    authors: String,
    year: Option<i32>,
    file_size_bytes: Option<u64>,
    source_path: Option<String>,
    cover_thumbnail: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct SourceOpenEvent {
    phase: String,
    source_path: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct CalibreLoadEvent {
    phase: String,
    count: Option<usize>,
    message: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct BridgeError {
    code: String,
    message: String,
}

#[derive(Debug)]
struct BackendState {
    mode: UiMode,
    active_source_path: Option<PathBuf>,
    open_in_flight: bool,
    panels: session::PanelState,
    base_config: config::AppConfig,
    normalizer: normalizer::TextNormalizer,
    reader: Option<session::ReaderSession>,
    calibre_config: calibre::CalibreConfig,
    calibre_books: Vec<calibre::CalibreBook>,
}

impl BackendState {
    fn new() -> Self {
        let base_config = config::load_config(Path::new("conf/config.toml"));
        let panels = session::PanelState {
            show_settings: base_config.show_settings,
            show_stats: false,
            show_tts: base_config.show_tts,
        };
        Self {
            mode: UiMode::Starter,
            active_source_path: None,
            open_in_flight: false,
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

fn persist_active_reader(state: &mut BackendState) {
    if let Some(reader) = &state.reader {
        session::persist_session_housekeeping(reader);
    }
}

async fn open_resolved_source(
    app: &tauri::AppHandle,
    state: &State<'_, Mutex<BackendState>>,
    source_path: PathBuf,
) -> Result<OpenSourceResult, BridgeError> {
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

    let _ = app.emit(
        "source-open",
        SourceOpenEvent {
            phase: "started".to_string(),
            source_path: Some(source_path.to_string_lossy().to_string()),
            message: None,
        },
    );

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
    .await
    .map_err(|err| bridge_error("task_join_error", format!("Failed to join load task: {err}")))?;

    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    guard.open_in_flight = false;

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

            let _ = app.emit(
                "source-open",
                SourceOpenEvent {
                    phase: "finished".to_string(),
                    source_path: Some(source_path.to_string_lossy().to_string()),
                    message: None,
                },
            );
            Ok(OpenSourceResult {
                session,
                reader: snapshot,
            })
        }
        Err(err) => {
            let _ = app.emit(
                "source-open",
                SourceOpenEvent {
                    phase: "failed".to_string(),
                    source_path: Some(source_path.to_string_lossy().to_string()),
                    message: Some(err.clone()),
                },
            );
            Err(bridge_error("open_failed", err))
        }
    }
}

#[tauri::command]
fn session_get_bootstrap(state: State<'_, Mutex<BackendState>>) -> Result<BootstrapState, BridgeError> {
    let guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    Ok(BootstrapState {
        app_name: "ebup-viewer".to_string(),
        mode: "migration".to_string(),
        config: BootstrapConfig {
            default_font_size: guard.base_config.font_size,
            default_lines_per_page: guard.base_config.lines_per_page,
            default_tts_speed: guard.base_config.tts_speed,
            default_pause_after_sentence: guard.base_config.pause_after_sentence,
            key_next_sentence: guard.base_config.key_next_sentence.clone(),
            key_prev_sentence: guard.base_config.key_prev_sentence.clone(),
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
    state: State<'_, Mutex<BackendState>>,
) -> Result<SessionState, BridgeError> {
    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;

    persist_active_reader(&mut guard);
    guard.reader = None;
    guard.mode = UiMode::Starter;
    guard.active_source_path = None;
    guard.open_in_flight = false;
    Ok(to_session_state(&guard))
}

#[tauri::command]
fn panel_toggle_settings(
    state: State<'_, Mutex<BackendState>>,
) -> Result<SessionState, BridgeError> {
    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    guard.panels.show_settings = !guard.panels.show_settings;
    if guard.panels.show_settings {
        guard.panels.show_stats = false;
    }
    Ok(to_session_state(&guard))
}

#[tauri::command]
fn panel_toggle_stats(state: State<'_, Mutex<BackendState>>) -> Result<SessionState, BridgeError> {
    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    guard.panels.show_stats = !guard.panels.show_stats;
    if guard.panels.show_stats {
        guard.panels.show_settings = false;
    }
    Ok(to_session_state(&guard))
}

#[tauri::command]
fn panel_toggle_tts(state: State<'_, Mutex<BackendState>>) -> Result<SessionState, BridgeError> {
    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    guard.panels.show_tts = !guard.panels.show_tts;
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
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    let normalizer = guard.normalizer.clone();
    let panels = guard.panels;
    let reader = guard
        .reader
        .as_mut()
        .ok_or_else(|| bridge_error("no_reader", "No active reader session"))?;
    Ok(reader.snapshot(panels, &normalizer))
}

#[tauri::command]
fn reader_next_page(
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    let normalizer = guard.normalizer.clone();
    let panels = guard.panels;
    let reader = guard
        .reader
        .as_mut()
        .ok_or_else(|| bridge_error("no_reader", "No active reader session"))?;
    reader.next_page(&normalizer);
    Ok(reader.snapshot(panels, &normalizer))
}

#[tauri::command]
fn reader_prev_page(
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    let normalizer = guard.normalizer.clone();
    let panels = guard.panels;
    let reader = guard
        .reader
        .as_mut()
        .ok_or_else(|| bridge_error("no_reader", "No active reader session"))?;
    reader.prev_page(&normalizer);
    Ok(reader.snapshot(panels, &normalizer))
}

#[tauri::command]
fn reader_set_page(
    state: State<'_, Mutex<BackendState>>,
    page: usize,
) -> Result<session::ReaderSnapshot, BridgeError> {
    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    let normalizer = guard.normalizer.clone();
    let panels = guard.panels;
    let reader = guard
        .reader
        .as_mut()
        .ok_or_else(|| bridge_error("no_reader", "No active reader session"))?;
    reader.set_page(page, &normalizer);
    Ok(reader.snapshot(panels, &normalizer))
}

#[tauri::command]
fn reader_sentence_click(
    state: State<'_, Mutex<BackendState>>,
    sentence_idx: usize,
) -> Result<session::ReaderSnapshot, BridgeError> {
    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    let normalizer = guard.normalizer.clone();
    let panels = guard.panels;
    let reader = guard
        .reader
        .as_mut()
        .ok_or_else(|| bridge_error("no_reader", "No active reader session"))?;
    reader.sentence_click(sentence_idx, &normalizer);
    Ok(reader.snapshot(panels, &normalizer))
}

#[tauri::command]
fn reader_next_sentence(
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    let normalizer = guard.normalizer.clone();
    let panels = guard.panels;
    let reader = guard
        .reader
        .as_mut()
        .ok_or_else(|| bridge_error("no_reader", "No active reader session"))?;
    reader.select_next_sentence(&normalizer);
    Ok(reader.snapshot(panels, &normalizer))
}

#[tauri::command]
fn reader_prev_sentence(
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    let normalizer = guard.normalizer.clone();
    let panels = guard.panels;
    let reader = guard
        .reader
        .as_mut()
        .ok_or_else(|| bridge_error("no_reader", "No active reader session"))?;
    reader.select_prev_sentence(&normalizer);
    Ok(reader.snapshot(panels, &normalizer))
}

#[tauri::command]
fn reader_toggle_text_only(
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    let normalizer = guard.normalizer.clone();
    let panels = guard.panels;
    let reader = guard
        .reader
        .as_mut()
        .ok_or_else(|| bridge_error("no_reader", "No active reader session"))?;
    reader.toggle_text_only(&normalizer);
    Ok(reader.snapshot(panels, &normalizer))
}

#[tauri::command]
fn reader_apply_settings(
    state: State<'_, Mutex<BackendState>>,
    patch: session::ReaderSettingsPatch,
) -> Result<session::ReaderSnapshot, BridgeError> {
    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    let normalizer = guard.normalizer.clone();
    let panels = guard.panels;
    let reader = guard
        .reader
        .as_mut()
        .ok_or_else(|| bridge_error("no_reader", "No active reader session"))?;
    reader.apply_settings_patch(patch, &normalizer);
    Ok(reader.snapshot(panels, &normalizer))
}

#[tauri::command]
fn reader_search_set_query(
    state: State<'_, Mutex<BackendState>>,
    query: String,
) -> Result<session::ReaderSnapshot, BridgeError> {
    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    let normalizer = guard.normalizer.clone();
    let panels = guard.panels;
    let reader = guard
        .reader
        .as_mut()
        .ok_or_else(|| bridge_error("no_reader", "No active reader session"))?;
    reader.set_search_query(query, &normalizer);
    Ok(reader.snapshot(panels, &normalizer))
}

#[tauri::command]
fn reader_search_next(
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    let normalizer = guard.normalizer.clone();
    let panels = guard.panels;
    let reader = guard
        .reader
        .as_mut()
        .ok_or_else(|| bridge_error("no_reader", "No active reader session"))?;
    reader.search_next(&normalizer);
    Ok(reader.snapshot(panels, &normalizer))
}

#[tauri::command]
fn reader_search_prev(
    state: State<'_, Mutex<BackendState>>,
) -> Result<session::ReaderSnapshot, BridgeError> {
    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    let normalizer = guard.normalizer.clone();
    let panels = guard.panels;
    let reader = guard
        .reader
        .as_mut()
        .ok_or_else(|| bridge_error("no_reader", "No active reader session"))?;
    reader.search_prev(&normalizer);
    Ok(reader.snapshot(panels, &normalizer))
}

#[tauri::command]
fn reader_close_session(
    state: State<'_, Mutex<BackendState>>,
) -> Result<SessionState, BridgeError> {
    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    persist_active_reader(&mut guard);
    guard.reader = None;
    guard.mode = UiMode::Starter;
    guard.active_source_path = None;
    Ok(to_session_state(&guard))
}

#[tauri::command]
async fn calibre_load_books(
    app: tauri::AppHandle,
    state: State<'_, Mutex<BackendState>>,
    force_refresh: Option<bool>,
) -> Result<Vec<CalibreBookDto>, BridgeError> {
    let force_refresh = force_refresh.unwrap_or(false);
    let config = {
        let guard = state
            .lock()
            .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
        guard.calibre_config.clone()
    };

    let _ = app.emit(
        "calibre-load",
        CalibreLoadEvent {
            phase: "started".to_string(),
            count: None,
            message: None,
        },
    );

    let books = tauri::async_runtime::spawn_blocking(move || calibre::load_books(&config, force_refresh))
        .await
        .map_err(|err| bridge_error("task_join_error", format!("Failed to join calibre task: {err}")))?
        .map_err(|err| bridge_error("calibre_load_failed", err.to_string()))?;

    let mut guard = state
        .lock()
        .map_err(|_| bridge_error("lock_poisoned", "Backend state lock poisoned"))?;
    guard.calibre_books = books.clone();

    let _ = app.emit(
        "calibre-load",
        CalibreLoadEvent {
            phase: "finished".to_string(),
            count: Some(books.len()),
            message: None,
        },
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
            .ok_or_else(|| bridge_error("not_found", format!("Unknown calibre book id={book_id}")))?;
        (book, guard.calibre_config.clone())
    };

    let path = tauri::async_runtime::spawn_blocking(move || {
        calibre::materialize_book_path(&calibre_config, &book)
    })
    .await
    .map_err(|err| bridge_error("task_join_error", format!("Failed to join calibre-open task: {err}")))?
    .map_err(|err| bridge_error("calibre_open_failed", err.to_string()))?;

    open_resolved_source(&app, &state, path).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let log_plugin = tauri_plugin_log::Builder::new()
        .level(LevelFilter::Info)
        .target(Target::new(TargetKind::Stdout))
        .target(Target::new(TargetKind::Webview))
        .build();

    info!("Starting ebup-viewer tauri bridge");
    let builder = tauri::Builder::default()
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
            reader_close_session,
            calibre_load_books,
            calibre_open_book
        ]);

    if let Err(err) = builder.run(tauri::generate_context!()) {
        warn!("tauri runtime failed: {err}");
        panic!("tauri runtime failed: {err}");
    }
}
