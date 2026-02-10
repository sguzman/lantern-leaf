use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT_CALIBRE_CONFIG_PATH: &str = "conf/calibre.toml";
const CALIBRE_CACHE_PATH: &str = ".cache/calibre-books.toml";
const CALIBRE_CACHE_REV: &str = "calibre-cache-v1";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct CalibreConfig {
    pub enabled: bool,
    pub library_path: Option<PathBuf>,
    pub calibredb_bin: String,
    pub allowed_extensions: Vec<String>,
    pub columns: Vec<String>,
    pub list_cache_ttl_secs: u64,
}

impl Default for CalibreConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            library_path: None,
            calibredb_bin: "calibredb".to_string(),
            allowed_extensions: vec!["epub".to_string(), "md".to_string(), "txt".to_string()],
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
    calibre: CalibreConfig,
}

impl Default for CalibreFile {
    fn default() -> Self {
        Self {
            calibre: CalibreConfig::default(),
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
    pub path: PathBuf,
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
        let path = PathBuf::from(DEFAULT_CALIBRE_CONFIG_PATH);
        let Ok(contents) = fs::read_to_string(&path) else {
            return Self::default();
        };
        toml::from_str::<CalibreFile>(&contents)
            .map(|file| file.calibre)
            .unwrap_or_default()
    }

    pub fn sanitized_extensions(&self) -> Vec<String> {
        let mut out = Vec::new();
        for ext in &self.allowed_extensions {
            let normalized = ext.trim().trim_start_matches('.').to_ascii_lowercase();
            let mapped = match normalized.as_str() {
                "epub" => "epub",
                "txt" => "txt",
                "md" | "markdown" => "md",
                _ => continue,
            };
            if !out.iter().any(|e| e == mapped) {
                out.push(mapped.to_string());
            }
        }
        if out.is_empty() {
            vec!["epub".to_string(), "md".to_string(), "txt".to_string()]
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

pub fn load_books(config: &CalibreConfig, force_refresh: bool) -> Result<Vec<CalibreBook>> {
    if !config.enabled {
        return Ok(Vec::new());
    }

    let library = config
        .library_path
        .clone()
        .ok_or_else(|| anyhow!("calibre.library_path is not set"))?;
    if !library.exists() {
        return Err(anyhow!(
            "calibre library path does not exist: {}",
            library.display()
        ));
    }

    let signature = cache_signature(config, &library);
    if !force_refresh {
        if let Some(cached) = try_load_cache(config, &signature)? {
            return Ok(cached);
        }
    }

    let books = fetch_books(config, &library)?;
    write_cache(&signature, &books)?;
    Ok(books)
}

fn fetch_books(config: &CalibreConfig, library: &Path) -> Result<Vec<CalibreBook>> {
    let output = Command::new(&config.calibredb_bin)
        .arg("--with-library")
        .arg(library)
        .arg("list")
        .arg("--for-machine")
        .arg("--fields")
        .arg("id,title,authors,pubdate,formats,size,path")
        .output()
        .with_context(|| "failed to run calibredb list command")?;

    if !output.status.success() {
        return Err(anyhow!(
            "calibredb list failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    let rows: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout)
        .with_context(|| "failed to parse calibredb JSON output")?;
    let allowed_extensions = config.sanitized_extensions();
    let allowed_set: HashSet<String> = allowed_extensions.iter().cloned().collect();

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

        let Some(path) = resolve_book_file_path(library, &row, &selected_ext) else {
            continue;
        };
        let size_from_fs = fs::metadata(&path).ok().map(|m| m.len());
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

fn resolve_book_file_path(
    library: &Path,
    row: &serde_json::Value,
    extension: &str,
) -> Option<PathBuf> {
    let rel_dir = parse_string_field(row, "path")?;
    let base = library.join(rel_dir);
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

fn try_load_cache(config: &CalibreConfig, signature: &str) -> Result<Option<Vec<CalibreBook>>> {
    let cache_path = PathBuf::from(CALIBRE_CACHE_PATH);
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

    let now = now_unix_secs();
    if now.saturating_sub(parsed.generated_unix_secs) > config.list_cache_ttl_secs {
        return Ok(None);
    }

    Ok(Some(parsed.books))
}

fn write_cache(signature: &str, books: &[CalibreBook]) -> Result<()> {
    let cache_path = PathBuf::from(CALIBRE_CACHE_PATH);
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

fn cache_signature(config: &CalibreConfig, library: &Path) -> String {
    let mut hasher = Sha256::new();
    hasher.update(CALIBRE_CACHE_REV.as_bytes());
    hasher.update(library.to_string_lossy().as_bytes());
    hasher.update(config.calibredb_bin.as_bytes());
    for ext in config.sanitized_extensions() {
        hasher.update(ext.as_bytes());
        hasher.update([0_u8]);
    }
    format!("{:x}", hasher.finalize())
}

fn now_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
