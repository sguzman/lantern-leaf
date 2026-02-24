use anyhow::{Context, Result, anyhow};
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tracing::{debug, info, warn};

const DEFAULT_CALIBRE_CONFIG_PATH: &str = "conf/calibre.toml";
const CALIBRE_CACHE_FILE: &str = "calibre-books.toml";
const CALIBRE_CACHE_REV: &str = "calibre-cache-v1";
const CALIBRE_DOWNLOAD_SUBDIR: &str = "calibre-downloads";
const CALIBRE_THUMB_SUBDIR: &str = "calibre-thumbs";
const THUMB_WIDTH: u32 = 68;
const THUMB_HEIGHT: u32 = 100;
const THUMB_PREFETCH_LIMIT: usize = 200;
const THUMB_PREFETCH_BUDGET: Duration = Duration::from_secs(2);
const THUMB_FETCH_TIMEOUT: Duration = Duration::from_millis(350);
const CALIBRE_DB_TIMEOUT_SECS: u64 = 15;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct CalibreConfig {
    pub enabled: bool,
    pub library_path: Option<PathBuf>,
    pub library_url: Option<String>,
    pub state_path: Option<PathBuf>,
    pub content_server: ContentServerConfig,
    pub calibredb_bin: String,
    pub server_urls: Vec<String>,
    pub server_username: Option<String>,
    pub server_password: Option<String>,
    pub allow_local_library_fallback: bool,
    pub allowed_extensions: Vec<String>,
    pub columns: Vec<String>,
    pub list_cache_ttl_secs: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct ContentServerConfig {
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Default for CalibreConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            library_path: None,
            library_url: Some("http://127.0.0.1:8080".to_string()),
            state_path: None,
            content_server: ContentServerConfig::default(),
            calibredb_bin: "calibredb".to_string(),
            server_urls: vec![
                "http://127.0.0.1:8080".to_string(),
                "http://localhost:8080".to_string(),
            ],
            server_username: None,
            server_password: None,
            allow_local_library_fallback: false,
            allowed_extensions: vec![
                "epub".to_string(),
                "pdf".to_string(),
                "md".to_string(),
                "txt".to_string(),
            ],
            columns: vec![
                "title".to_string(),
                "extension".to_string(),
                "author".to_string(),
                "year".to_string(),
                "size".to_string(),
            ],
            list_cache_ttl_secs: 600,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
struct CalibreFile {
    calibre: Option<CalibreConfig>,
    calibred: Option<CalibreConfig>,
}

impl Default for CalibreFile {
    fn default() -> Self {
        Self {
            calibre: Some(CalibreConfig::default()),
            calibred: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CalibreBook {
    pub id: u64,
    pub title: String,
    pub extension: String,
    pub authors: String,
    pub year: Option<i32>,
    pub file_size_bytes: Option<u64>,
    pub path: Option<PathBuf>,
    pub cover_thumbnail: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalibreColumn {
    Title,
    Extension,
    Author,
    Year,
    Size,
}

#[derive(Debug, Deserialize, Serialize)]
struct CachedBookList {
    rev: String,
    generated_unix_secs: u64,
    signature: String,
    books: Vec<CalibreBook>,
}

impl CalibreConfig {
    pub fn load_default() -> Self {
        let Some(path) = resolve_config_path() else {
            return Self::default();
        };
        let Ok(contents) = fs::read_to_string(&path) else {
            return Self::default();
        };
        match toml::from_str::<CalibreFile>(&contents) {
            Ok(file) => file
                .calibre
                .or(file.calibred)
                .unwrap_or_else(CalibreConfig::default),
            Err(err) => {
                warn!(
                    path = %path.display(),
                    "Invalid calibre config TOML; falling back to defaults: {err}"
                );
                CalibreConfig::default()
            }
        }
    }

    pub fn sanitized_extensions(&self) -> Vec<String> {
        let mut out = Vec::new();
        for ext in &self.allowed_extensions {
            let normalized = ext.trim().trim_start_matches('.').to_ascii_lowercase();
            let mapped = match normalized.as_str() {
                "epub" => "epub",
                "pdf" => "pdf",
                "txt" => "txt",
                "md" | "markdown" => "md",
                _ => continue,
            };
            if !out.iter().any(|e| e == mapped) {
                out.push(mapped.to_string());
            }
        }
        if out.is_empty() {
            vec![
                "epub".to_string(),
                "pdf".to_string(),
                "md".to_string(),
                "txt".to_string(),
            ]
        } else {
            out
        }
    }

    pub fn sanitized_columns(&self) -> Vec<CalibreColumn> {
        let mut out = Vec::new();
        for column in &self.columns {
            let normalized = column.trim().to_ascii_lowercase();
            let mapped = match normalized.as_str() {
                "title" => CalibreColumn::Title,
                "ext" | "extension" | "format" => CalibreColumn::Extension,
                "author" | "authors" => CalibreColumn::Author,
                "year" | "pub-year" | "published" => CalibreColumn::Year,
                "size" | "file-size" => CalibreColumn::Size,
                _ => continue,
            };
            if !out.contains(&mapped) {
                out.push(mapped);
            }
        }
        if out.is_empty() {
            vec![
                CalibreColumn::Title,
                CalibreColumn::Extension,
                CalibreColumn::Author,
                CalibreColumn::Year,
                CalibreColumn::Size,
            ]
        } else {
            out
        }
    }
}

fn resolve_config_path() -> Option<PathBuf> {
    if let Some(value) = std::env::var_os("CALIBRE_CONFIG_PATH") {
        let candidate = PathBuf::from(value);
        if candidate.exists() {
            return Some(candidate);
        }
        warn!(
            path = %candidate.display(),
            "CALIBRE_CONFIG_PATH is set but file does not exist; falling back to defaults/search paths"
        );
    }

    let relative = PathBuf::from(DEFAULT_CALIBRE_CONFIG_PATH);
    if relative.exists() {
        return Some(relative);
    }

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let rooted = manifest_dir.join(DEFAULT_CALIBRE_CONFIG_PATH);
    if rooted.exists() {
        return Some(rooted);
    }

    if let Some(parent) = manifest_dir.parent() {
        let rooted_parent = parent.join(DEFAULT_CALIBRE_CONFIG_PATH);
        if rooted_parent.exists() {
            return Some(rooted_parent);
        }
    }

    None
}

pub fn load_books(config: &CalibreConfig, force_refresh: bool) -> Result<Vec<CalibreBook>> {
    if !config.enabled {
        return Ok(Vec::new());
    }

    let started = Instant::now();
    info!(
        force_refresh,
        list_cache_ttl_secs = config.list_cache_ttl_secs,
        thumb_prefetch_limit = THUMB_PREFETCH_LIMIT,
        thumb_prefetch_budget_ms = THUMB_PREFETCH_BUDGET.as_millis(),
        "Starting calibre catalog load"
    );

    let signature = cache_signature(config);
    if !force_refresh {
        if let Some(mut cached) = try_load_cache(config, &signature, false)? {
            info!(book_count = cached.len(), "Using cached calibre catalog");
            let changed = hydrate_book_thumbnails(
                config,
                &mut cached,
                THUMB_PREFETCH_LIMIT,
                THUMB_PREFETCH_BUDGET,
            );
            if changed {
                let _ = write_cache(&signature, &cached);
            }
            info!(
                book_count = cached.len(),
                elapsed_ms = started.elapsed().as_millis(),
                "Finished calibre catalog load from cache"
            );
            return Ok(cached);
        }
        info!("Calibre cache missing/incompatible; fetching from source");
    }

    let mut books = fetch_books(config)?;
    info!(
        book_count = books.len(),
        "Fetched calibre catalog from source"
    );
    let _ = hydrate_book_thumbnails(
        config,
        &mut books,
        THUMB_PREFETCH_LIMIT,
        THUMB_PREFETCH_BUDGET,
    );
    info!(book_count = books.len(), "Writing calibre cache file");
    write_cache(&signature, &books)?;
    info!(book_count = books.len(), "Calibre cache file updated");
    info!(
        book_count = books.len(),
        elapsed_ms = started.elapsed().as_millis(),
        "Finished calibre catalog load"
    );
    Ok(books)
}

pub fn materialize_book_path(config: &CalibreConfig, book: &CalibreBook) -> Result<PathBuf> {
    if let Some(path) = book.path.as_ref().filter(|path| path.exists()) {
        return Ok(path.clone());
    }

    let ext = canonical_extension(&book.extension);
    let cache_root = calibre_download_dir();
    fs::create_dir_all(&cache_root)
        .with_context(|| format!("failed to create {}", cache_root.display()))?;

    let file_name = format!("{}-{}.{}", book.id, short_title_hash(&book.title), ext);
    let target_path = cache_root.join(file_name);
    if target_path.exists() {
        return Ok(target_path);
    }

    let mut last_err = None;
    for target in calibre_targets(config) {
        let tmp_dir = cache_root.join(format!(
            "tmp-{}-{}-{}",
            book.id,
            std::process::id(),
            now_unix_nanos()
        ));
        if let Err(err) = fs::create_dir_all(&tmp_dir) {
            last_err = Some(format!("failed to create {}: {err}", tmp_dir.display()));
            continue;
        }

        let export_result = run_calibredb_export(config, &target, book.id, &ext, &tmp_dir)
            .and_then(|_| {
                find_exported_file(&tmp_dir, &ext).ok_or_else(|| {
                    anyhow!(
                        "export completed but no .{ext} file was found in {}",
                        tmp_dir.display()
                    )
                })
            })
            .and_then(|found| {
                fs::copy(&found, &target_path).with_context(|| {
                    format!(
                        "failed to copy exported file {} -> {}",
                        found.display(),
                        target_path.display()
                    )
                })?;
                Ok(())
            });

        let _ = fs::remove_dir_all(&tmp_dir);

        match export_result {
            Ok(()) => return Ok(target_path),
            Err(err) => last_err = Some(format!("{}: {err}", target.label)),
        }
    }

    Err(anyhow!(
        "failed to materialize book id={} ext={} via calibre targets. {}",
        book.id,
        ext,
        last_err.unwrap_or_else(|| "no export targets succeeded".to_string())
    ))
}

fn fetch_books(config: &CalibreConfig) -> Result<Vec<CalibreBook>> {
    let rows = fetch_rows_from_targets(config)?;
    let allowed_extensions = config.sanitized_extensions();
    let allowed_set: HashSet<String> = allowed_extensions.iter().cloned().collect();
    let library = config.state_path.clone().or(config.library_path.clone());
    let id_dir_index = library
        .as_deref()
        .map(index_library_book_dirs)
        .unwrap_or_default();

    let mut books = Vec::new();
    for row in rows {
        let id = parse_u64_field(&row, "id");
        let Some(id) = id else {
            continue;
        };

        let title = parse_string_field(&row, "title").unwrap_or_else(|| "Untitled".to_string());
        let authors = parse_authors(&row);
        let year = parse_year_field(&row, "pubdate");

        let formats = parse_formats(&row);
        let selected_ext = allowed_extensions
            .iter()
            .find(|ext| formats.iter().any(|f| f == *ext))
            .cloned();
        let Some(selected_ext) = selected_ext else {
            continue;
        };

        let path =
            resolve_book_file_path(library.as_deref(), &id_dir_index, &row, id, &selected_ext);
        let size_from_fs = path
            .as_ref()
            .and_then(|resolved| fs::metadata(resolved).ok().map(|m| m.len()));
        let file_size_bytes = size_from_fs.or_else(|| parse_u64_field(&row, "size"));

        if !allowed_set.contains(&selected_ext) {
            continue;
        }

        books.push(CalibreBook {
            id,
            title,
            extension: selected_ext,
            authors,
            year,
            file_size_bytes,
            cover_thumbnail: None,
            path,
        });
    }

    books.sort_by(|a, b| {
        a.title
            .to_ascii_lowercase()
            .cmp(&b.title.to_ascii_lowercase())
            .then_with(|| a.id.cmp(&b.id))
    });
    Ok(books)
}

fn fetch_rows_from_targets(config: &CalibreConfig) -> Result<Vec<serde_json::Value>> {
    let mut last_err = None;
    for target in calibre_targets(config) {
        info!(target = %target.label, "Attempting calibre target");
        match run_calibredb_list(config, &target) {
            Ok(rows) => {
                info!(target = %target.label, row_count = rows.len(), "Calibre target responded");
                return Ok(rows);
            }
            Err(err) => last_err = Some(format!("{}: {err}", target.label)),
        }
    }
    Err(anyhow!(
        "no server detected (checked configured/default calibre content-server URLs). {}",
        last_err.unwrap_or_else(|| "no targets were available".to_string())
    ))
}

fn run_calibredb_list(
    config: &CalibreConfig,
    target: &CalibreTarget,
) -> Result<Vec<serde_json::Value>> {
    let mut cmd = Command::new(&config.calibredb_bin);
    cmd.arg("--timeout")
        .arg(CALIBRE_DB_TIMEOUT_SECS.to_string());
    cmd.arg("--with-library").arg(&target.with_library);
    if let Some(username) = &target.username {
        cmd.arg("--username").arg(username);
    }
    if let Some(password) = &target.password {
        cmd.arg("--password").arg(password);
    }
    cmd.arg("list")
        .arg("--for-machine")
        .arg("--fields")
        .arg("id,title,authors,pubdate,formats,size");

    let output = cmd
        .output()
        .with_context(|| format!("failed to run calibredb list against {}", target.label))?;
    if !output.status.success() {
        return Err(anyhow!(
            String::from_utf8_lossy(&output.stderr).trim().to_string()
        ));
    }
    let rows: Vec<serde_json::Value> =
        serde_json::from_slice(&output.stdout).with_context(|| {
            format!(
                "failed to parse calibredb JSON output from {}",
                target.label
            )
        })?;
    Ok(rows)
}

fn run_calibredb_export(
    config: &CalibreConfig,
    target: &CalibreTarget,
    book_id: u64,
    extension: &str,
    out_dir: &Path,
) -> Result<()> {
    let mut cmd = Command::new(&config.calibredb_bin);
    cmd.arg("--timeout")
        .arg((CALIBRE_DB_TIMEOUT_SECS * 4).to_string());
    cmd.arg("--with-library").arg(&target.with_library);
    if let Some(username) = &target.username {
        cmd.arg("--username").arg(username);
    }
    if let Some(password) = &target.password {
        cmd.arg("--password").arg(password);
    }
    cmd.arg("export")
        .arg("--single-dir")
        .arg("--dont-write-opf")
        .arg("--dont-save-cover")
        .arg("--dont-save-extra-files")
        .arg("--to-dir")
        .arg(out_dir)
        .arg("--formats")
        .arg(extension)
        .arg(book_id.to_string());

    let output = cmd.output().with_context(|| {
        format!(
            "failed to run calibredb export for id {book_id} against {}",
            target.label
        )
    })?;
    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow!(
            String::from_utf8_lossy(&output.stderr).trim().to_string()
        ))
    }
}

struct CalibreTarget {
    label: String,
    with_library: String,
    username: Option<String>,
    password: Option<String>,
}

fn calibre_targets(config: &CalibreConfig) -> Vec<CalibreTarget> {
    let mut targets = Vec::new();
    if let Some(url) = sanitized_library_url(config) {
        targets.push(CalibreTarget {
            label: format!("server:{url}"),
            with_library: url,
            username: effective_username(config),
            password: effective_password(config),
        });
    }
    for url in sanitized_server_urls(config) {
        if targets.iter().any(|t| t.with_library == url) {
            continue;
        }
        targets.push(CalibreTarget {
            label: format!("server:{url}"),
            with_library: url,
            username: effective_username(config),
            password: effective_password(config),
        });
    }
    if config.allow_local_library_fallback {
        if let Some(path) = config.state_path.as_ref().or(config.library_path.as_ref()) {
            targets.push(CalibreTarget {
                label: format!("local:{}", path.display()),
                with_library: path.to_string_lossy().to_string(),
                username: None,
                password: None,
            });
        }
    }
    targets
}

fn sanitized_server_urls(config: &CalibreConfig) -> Vec<String> {
    let mut urls = Vec::new();
    for raw in &config.server_urls {
        let url = raw.trim().trim_end_matches('/').to_string();
        if url.starts_with("http://") || url.starts_with("https://") {
            if !urls.iter().any(|u| u == &url) {
                urls.push(url);
            }
        }
    }
    if urls.is_empty() {
        vec![
            "http://127.0.0.1:8080".to_string(),
            "http://localhost:8080".to_string(),
        ]
    } else {
        urls
    }
}

fn sanitized_library_url(config: &CalibreConfig) -> Option<String> {
    config
        .library_url
        .as_ref()
        .map(|v| v.trim().to_string())
        .filter(|v| v.starts_with("http://") || v.starts_with("https://"))
}

fn effective_username(config: &CalibreConfig) -> Option<String> {
    config
        .content_server
        .username
        .as_ref()
        .or(config.server_username.as_ref())
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

fn effective_password(config: &CalibreConfig) -> Option<String> {
    config
        .content_server
        .password
        .as_ref()
        .or(config.server_password.as_ref())
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

fn parse_u64_field(row: &serde_json::Value, key: &str) -> Option<u64> {
    row.get(key).and_then(|v| {
        v.as_u64()
            .or_else(|| v.as_i64().map(|n| n.max(0) as u64))
            .or_else(|| v.as_str().and_then(|s| s.trim().parse::<u64>().ok()))
    })
}

fn parse_string_field(row: &serde_json::Value, key: &str) -> Option<String> {
    row.get(key).and_then(|value| {
        value
            .as_str()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    })
}

fn parse_year_field(row: &serde_json::Value, key: &str) -> Option<i32> {
    let raw = parse_string_field(row, key)?;
    let year = raw.chars().take(4).collect::<String>();
    if year.chars().all(|c| c.is_ascii_digit()) {
        year.parse::<i32>().ok()
    } else {
        None
    }
}

fn parse_authors(row: &serde_json::Value) -> String {
    match row.get("authors") {
        Some(serde_json::Value::Array(values)) => {
            let joined = values
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(", ");
            if joined.is_empty() {
                "Unknown".to_string()
            } else {
                joined
            }
        }
        Some(value) => value
            .as_str()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "Unknown".to_string()),
        None => "Unknown".to_string(),
    }
}

fn parse_formats(row: &serde_json::Value) -> Vec<String> {
    let Some(value) = row.get("formats") else {
        return Vec::new();
    };
    match value {
        serde_json::Value::Array(values) => values
            .iter()
            .filter_map(|v| v.as_str())
            .map(normalize_format_value)
            .filter(|s| !s.is_empty())
            .collect(),
        serde_json::Value::String(raw) => raw
            .split(',')
            .map(normalize_format_value)
            .filter(|s| !s.is_empty())
            .collect(),
        _ => Vec::new(),
    }
}

fn normalize_format_value(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    Path::new(trimmed)
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase())
        .or_else(|| Some(trimmed.to_ascii_lowercase()))
        .unwrap_or_default()
}

fn canonical_extension(raw: &str) -> String {
    let normalized = raw.trim().trim_start_matches('.').to_ascii_lowercase();
    if normalized == "markdown" {
        "md".to_string()
    } else {
        normalized
    }
}

fn find_exported_file(dir: &Path, extension: &str) -> Option<PathBuf> {
    let wanted = canonical_extension(extension);
    let entries = fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .map(canonical_extension)?;
        if ext == wanted {
            return Some(path);
        }
    }
    None
}

fn short_title_hash(title: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(title.as_bytes());
    let hash = format!("{:x}", hasher.finalize());
    hash.chars().take(8).collect()
}

fn resolve_book_file_path(
    library: Option<&Path>,
    id_dir_index: &HashMap<u64, PathBuf>,
    row: &serde_json::Value,
    book_id: u64,
    extension: &str,
) -> Option<PathBuf> {
    let rel_dir = parse_string_field(row, "path");
    let base = match (library, rel_dir.as_deref()) {
        (Some(root), Some(rel)) => root.join(rel),
        (Some(_), None) => id_dir_index.get(&book_id)?.clone(),
        _ => return None,
    };
    let entries = fs::read_dir(&base).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_ascii_lowercase())?;
        let normalized_ext = if ext == "markdown" {
            "md"
        } else {
            ext.as_str()
        };
        if normalized_ext == extension {
            return Some(path);
        }
    }
    None
}

fn index_library_book_dirs(root: &Path) -> HashMap<u64, PathBuf> {
    let mut out = HashMap::new();
    collect_book_dirs(root, 0, &mut out);
    out
}

fn collect_book_dirs(dir: &Path, depth: usize, out: &mut HashMap<u64, PathBuf>) {
    if depth > 4 {
        return;
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        if !ft.is_dir() {
            continue;
        }
        if let Some(book_id) = parse_book_id_from_dir_name(&path) {
            out.entry(book_id).or_insert(path.clone());
        }
        collect_book_dirs(&path, depth + 1, out);
    }
}

fn parse_book_id_from_dir_name(path: &Path) -> Option<u64> {
    let name = path.file_name()?.to_str()?;
    let start = name.rfind('(')?;
    if !name.ends_with(')') || start + 1 >= name.len() - 1 {
        return None;
    }
    name[start + 1..name.len() - 1].trim().parse::<u64>().ok()
}

fn hydrate_book_thumbnails(
    config: &CalibreConfig,
    books: &mut [CalibreBook],
    limit: usize,
    budget: Duration,
) -> bool {
    let mut changed = false;
    let started = Instant::now();
    let deadline = started + budget;
    let mut processed = 0usize;
    let mut available = 0usize;
    let prefetch_count = books.len().min(limit);
    for book in books.iter_mut().take(prefetch_count) {
        if started.elapsed() >= budget {
            info!(
                processed,
                available,
                budget_ms = budget.as_millis(),
                "Stopping calibre thumbnail prefetch due to time budget"
            );
            break;
        }
        let current = book.cover_thumbnail.clone();
        let book_started = Instant::now();
        let next = ensure_book_thumbnail(config, book.id, book.path.as_deref(), deadline);
        if next != current {
            book.cover_thumbnail = next;
            changed = true;
        }
        let per_book_ms = book_started.elapsed().as_millis();
        if per_book_ms > 200 {
            info!(
                book_id = book.id,
                elapsed_ms = per_book_ms,
                "Slow thumbnail prefetch item"
            );
        }
        processed += 1;
        if book.cover_thumbnail.is_some() {
            available += 1;
        }
        if processed % 25 == 0 {
            info!(
                processed,
                available,
                elapsed_ms = started.elapsed().as_millis(),
                "Calibre thumbnail prefetch progress"
            );
        }
    }
    info!(
        processed,
        available,
        changed,
        elapsed_ms = started.elapsed().as_millis(),
        "Finished calibre thumbnail prefetch pass"
    );
    changed
}

fn ensure_book_thumbnail(
    config: &CalibreConfig,
    book_id: u64,
    source_path: Option<&Path>,
    deadline: Instant,
) -> Option<PathBuf> {
    let thumb_path = calibre_thumbnail_path(config, book_id);
    if thumb_path.exists() {
        return Some(thumb_path);
    }

    if let Some(dir) = source_path.and_then(Path::parent)
        && let Some(local_cover) = resolve_local_cover_file(dir)
        && let Ok(bytes) = fs::read(&local_cover)
        && write_thumbnail_file(&thumb_path, &bytes).is_ok()
    {
        return Some(thumb_path);
    }

    if let Some(bytes) = fetch_thumbnail_from_server(config, book_id, deadline)
        && write_thumbnail_file(&thumb_path, &bytes).is_ok()
    {
        return Some(thumb_path);
    }

    None
}

fn resolve_local_cover_file(book_dir: &Path) -> Option<PathBuf> {
    for name in ["cover.jpg", "cover.jpeg", "cover.png", "cover.webp"] {
        let candidate = book_dir.join(name);
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}

fn fetch_thumbnail_from_server(
    config: &CalibreConfig,
    book_id: u64,
    deadline: Instant,
) -> Option<Vec<u8>> {
    let remaining = deadline.saturating_duration_since(Instant::now());
    if remaining < Duration::from_millis(40) {
        return None;
    }
    let timeout = remaining.min(THUMB_FETCH_TIMEOUT);
    let client = reqwest::blocking::Client::builder()
        .timeout(timeout)
        .build()
        .ok()?;
    let username = effective_username(config);
    let password = effective_password(config);
    let endpoints = [
        format!("get/thumb/{book_id}"),
        format!("get/cover/{book_id}"),
    ];

    for base in cover_server_urls(config).into_iter().take(1) {
        if Instant::now() >= deadline {
            return None;
        }
        for endpoint in &endpoints {
            if Instant::now() >= deadline {
                return None;
            }
            let url = format!("{base}/{endpoint}");
            let mut request = client.get(&url);
            if let Some(user) = username.as_ref() {
                request = request.basic_auth(user, password.clone());
            }

            let Ok(response) = request.send() else {
                continue;
            };
            if response.status() != StatusCode::OK {
                continue;
            }
            let Ok(bytes) = response.bytes() else {
                continue;
            };
            if bytes.is_empty() {
                continue;
            }
            return Some(bytes.to_vec());
        }
    }

    None
}

fn calibre_thumbnail_path(config: &CalibreConfig, book_id: u64) -> PathBuf {
    let key = thumbnail_scope_key(config);
    calibre_thumb_dir().join(key).join(format!("{book_id}.jpg"))
}

fn thumbnail_scope_key(config: &CalibreConfig) -> String {
    let mut hasher = Sha256::new();
    if let Some(url) = sanitized_library_url(config) {
        hasher.update(url.as_bytes());
    }
    if let Some(path) = config.state_path.as_ref().or(config.library_path.as_ref()) {
        hasher.update(path.to_string_lossy().as_bytes());
    }
    for url in sanitized_server_urls(config) {
        hasher.update(url.as_bytes());
    }
    let digest = format!("{:x}", hasher.finalize());
    digest.chars().take(16).collect()
}

fn cover_server_urls(config: &CalibreConfig) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(raw) = sanitized_library_url(config)
        && let Some(base) = normalize_server_base_url(&raw)
    {
        out.push(base);
    }
    for raw in sanitized_server_urls(config) {
        if let Some(base) = normalize_server_base_url(&raw)
            && !out.iter().any(|known| known == &base)
        {
            out.push(base);
        }
    }
    out
}

fn normalize_server_base_url(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if !(trimmed.starts_with("http://") || trimmed.starts_with("https://")) {
        return None;
    }
    let no_fragment = trimmed.split('#').next()?.split('?').next()?.trim();
    let normalized = no_fragment.trim_end_matches('/').to_string();
    if normalized.is_empty() {
        None
    } else {
        Some(normalized)
    }
}

fn write_thumbnail_file(path: &Path, raw_image: &[u8]) -> Result<()> {
    let image = image::load_from_memory(raw_image).context("decoding thumbnail image")?;
    let thumb = image.resize(THUMB_WIDTH, THUMB_HEIGHT, FilterType::Triangle);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create thumbnail dir {}", parent.display()))?;
    }
    let mut encoded = Vec::new();
    let mut encoder = JpegEncoder::new_with_quality(Cursor::new(&mut encoded), 80);
    encoder
        .encode_image(&thumb)
        .context("encoding thumbnail as jpeg")?;
    fs::write(path, encoded)
        .with_context(|| format!("failed to write thumbnail {}", path.display()))?;
    debug!(path = %path.display(), "cached calibre thumbnail");
    Ok(())
}

fn try_load_cache(
    config: &CalibreConfig,
    signature: &str,
    check_ttl: bool,
) -> Result<Option<Vec<CalibreBook>>> {
    let cache_path = calibre_cache_path();
    let contents = match fs::read_to_string(&cache_path) {
        Ok(contents) => contents,
        Err(_) => return Ok(None),
    };
    let parsed: CachedBookList = match toml::from_str(&contents) {
        Ok(parsed) => parsed,
        Err(_) => return Ok(None),
    };
    if parsed.rev != CALIBRE_CACHE_REV || parsed.signature != signature {
        return Ok(None);
    }

    if check_ttl {
        let now = now_unix_secs();
        if now.saturating_sub(parsed.generated_unix_secs) > config.list_cache_ttl_secs {
            return Ok(None);
        }
    }

    Ok(Some(parsed.books))
}

fn write_cache(signature: &str, books: &[CalibreBook]) -> Result<()> {
    let cache_path = calibre_cache_path();
    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    let payload = CachedBookList {
        rev: CALIBRE_CACHE_REV.to_string(),
        generated_unix_secs: now_unix_secs(),
        signature: signature.to_string(),
        books: books.to_vec(),
    };
    let serialized =
        toml::to_string(&payload).with_context(|| "failed to serialize calibre cache")?;
    fs::write(&cache_path, serialized)
        .with_context(|| format!("failed to write {}", cache_path.display()))?;
    Ok(())
}

fn cache_signature(config: &CalibreConfig) -> String {
    let mut hasher = Sha256::new();
    hasher.update(CALIBRE_CACHE_REV.as_bytes());
    hasher.update(config.calibredb_bin.as_bytes());
    if let Some(url) = sanitized_library_url(config) {
        hasher.update(url.as_bytes());
        hasher.update([0u8]);
    }
    for url in sanitized_server_urls(config) {
        hasher.update(url.as_bytes());
        hasher.update([0u8]);
    }
    if let Some(path) = &config.state_path {
        hasher.update(path.to_string_lossy().as_bytes());
    }
    if let Some(path) = &config.library_path {
        hasher.update(path.to_string_lossy().as_bytes());
    }
    hasher.update([config.allow_local_library_fallback as u8]);
    for ext in config.sanitized_extensions() {
        hasher.update(ext.as_bytes());
        hasher.update([0_u8]);
    }
    format!("{:x}", hasher.finalize())
}

fn calibre_cache_path() -> PathBuf {
    crate::cache::cache_root().join(CALIBRE_CACHE_FILE)
}

fn calibre_download_dir() -> PathBuf {
    crate::cache::cache_root().join(CALIBRE_DOWNLOAD_SUBDIR)
}

fn calibre_thumb_dir() -> PathBuf {
    crate::cache::cache_root().join(CALIBRE_THUMB_SUBDIR)
}

fn now_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn now_unix_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_file(name: &str, extension: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("ebup_viewer_calibre_{name}_{nanos}.{extension}"))
    }

    #[test]
    fn load_default_reads_env_override_file() {
        let key = "CALIBRE_CONFIG_PATH";
        let previous = std::env::var_os(key);
        let path = unique_temp_file("load_default", "toml");
        fs::write(
            &path,
            r#"
[calibre]
enabled = true
server_urls = ["http://0.0.0.0:1"]
allowed_extensions = ["epub", "pdf", "txt"]
"#,
        )
        .expect("write calibre override");

        // SAFETY: test-scoped env mutation; restored before test exits.
        unsafe {
            std::env::set_var(key, &path);
        }
        let config = CalibreConfig::load_default();
        assert!(config.enabled);
        assert_eq!(config.server_urls, vec!["http://0.0.0.0:1".to_string()]);

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

        let _ = fs::remove_file(path);
    }

    #[test]
    fn calibre_paths_follow_cache_root_override() {
        let key = crate::cache::CACHE_DIR_ENV;
        let previous = std::env::var_os(key);
        let override_path = std::env::temp_dir().join(format!(
            "ebup_viewer_calibre_cache_root_{}_{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("clock should be after epoch")
                .as_nanos()
        ));

        // SAFETY: test-scoped env mutation; restored before test exits.
        unsafe {
            std::env::set_var(key, &override_path);
        }

        assert_eq!(calibre_cache_path(), override_path.join(CALIBRE_CACHE_FILE));
        assert_eq!(
            calibre_download_dir(),
            override_path.join(CALIBRE_DOWNLOAD_SUBDIR)
        );
        assert_eq!(
            calibre_thumb_dir(),
            override_path.join(CALIBRE_THUMB_SUBDIR)
        );

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
}
