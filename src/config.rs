//! Configuration loading for the EPUB viewer.
//!
//! All user-tunable settings are centralized here and loaded from
//! `conf/config.toml` if present. Any missing or invalid entries fall back to
//! sensible defaults so the UI can still launch.

use serde::Deserialize;
use std::fs;
use std::path::Path;
use tracing::{debug, info, warn};

/// High-level app configuration; deserializable from TOML.
#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct AppConfig {
    #[serde(default)]
    pub theme: ThemeMode,
    #[serde(default = "default_font_size")]
    pub font_size: u32,
    #[serde(default = "default_line_spacing")]
    pub line_spacing: f32,
    #[serde(default = "default_margin")]
    pub margin_horizontal: u16,
    #[serde(default = "default_margin")]
    pub margin_vertical: u16,
    #[serde(default)]
    pub font_family: FontFamily,
    #[serde(default)]
    pub font_weight: FontWeight,
    #[serde(default)]
    pub word_spacing: u32,
    #[serde(default)]
    pub letter_spacing: u32,
    #[serde(default = "default_tts_model")]
    pub tts_model_path: String,
    #[serde(default = "default_tts_speed")]
    pub tts_speed: f32,
    #[serde(default = "default_tts_espeak_path")]
    pub tts_espeak_path: String,
    #[serde(default = "default_tts_threads")]
    pub tts_threads: usize,
    #[serde(default = "default_show_tts")]
    pub show_tts: bool,
    #[serde(default = "default_show_settings")]
    pub show_settings: bool,
    #[serde(default = "default_day_highlight")]
    pub day_highlight: HighlightColor,
    #[serde(default = "default_night_highlight")]
    pub night_highlight: HighlightColor,
    #[serde(default = "default_log_level")]
    pub log_level: LogLevel,
    #[serde(default = "default_lines_per_page")]
    pub lines_per_page: usize,
    #[serde(default = "default_pause_after_sentence")]
    pub pause_after_sentence: f32,
    #[serde(default = "default_auto_scroll_tts")]
    pub auto_scroll_tts: bool,
    #[serde(default = "default_center_spoken_sentence")]
    pub center_spoken_sentence: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum ConfigInput {
    Tables(ConfigTables),
    Flat(AppConfig),
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            theme: ThemeMode::Night,
            font_size: default_font_size(),
            line_spacing: default_line_spacing(),
            margin_horizontal: default_margin(),
            margin_vertical: default_margin(),
            font_family: FontFamily::Sans,
            font_weight: FontWeight::Normal,
            word_spacing: 0,
            letter_spacing: 0,
            tts_model_path: default_tts_model(),
            tts_speed: default_tts_speed(),
            tts_espeak_path: default_tts_espeak_path(),
            tts_threads: default_tts_threads(),
            show_tts: default_show_tts(),
            show_settings: default_show_settings(),
            day_highlight: default_day_highlight(),
            night_highlight: default_night_highlight(),
            log_level: default_log_level(),
            lines_per_page: default_lines_per_page(),
            pause_after_sentence: default_pause_after_sentence(),
            auto_scroll_tts: default_auto_scroll_tts(),
            center_spoken_sentence: default_center_spoken_sentence(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
struct ConfigTables {
    #[serde(default)]
    appearance: AppearanceConfig,
    #[serde(default)]
    reading_behavior: ReadingBehaviorConfig,
    #[serde(default)]
    ui: UiConfig,
    #[serde(default)]
    logging: LoggingConfig,
    #[serde(default)]
    tts: TtsConfig,
}

impl From<ConfigTables> for AppConfig {
    fn from(tables: ConfigTables) -> Self {
        AppConfig {
            theme: tables.appearance.theme,
            font_family: tables.appearance.font_family,
            font_weight: tables.appearance.font_weight,
            font_size: tables.appearance.font_size,
            line_spacing: tables.appearance.line_spacing,
            word_spacing: tables.appearance.word_spacing,
            letter_spacing: tables.appearance.letter_spacing,
            lines_per_page: tables.appearance.lines_per_page,
            margin_horizontal: tables.appearance.margin_horizontal,
            margin_vertical: tables.appearance.margin_vertical,
            day_highlight: tables.appearance.day_highlight,
            night_highlight: tables.appearance.night_highlight,
            pause_after_sentence: tables.reading_behavior.pause_after_sentence,
            auto_scroll_tts: tables.reading_behavior.auto_scroll_tts,
            center_spoken_sentence: tables.reading_behavior.center_spoken_sentence,
            show_tts: tables.ui.show_tts,
            show_settings: tables.ui.show_settings,
            log_level: tables.logging.log_level,
            tts_model_path: tables.tts.tts_model_path,
            tts_espeak_path: tables.tts.tts_espeak_path,
            tts_speed: tables.tts.tts_speed,
            tts_threads: tables.tts.tts_threads,
        }
    }
}

impl From<&AppConfig> for ConfigTables {
    fn from(config: &AppConfig) -> Self {
        ConfigTables {
            appearance: AppearanceConfig {
                theme: config.theme,
                font_family: config.font_family,
                font_weight: config.font_weight,
                font_size: config.font_size,
                line_spacing: config.line_spacing,
                word_spacing: config.word_spacing,
                letter_spacing: config.letter_spacing,
                lines_per_page: config.lines_per_page,
                margin_horizontal: config.margin_horizontal,
                margin_vertical: config.margin_vertical,
                day_highlight: config.day_highlight,
                night_highlight: config.night_highlight,
            },
            reading_behavior: ReadingBehaviorConfig {
                pause_after_sentence: config.pause_after_sentence,
                auto_scroll_tts: config.auto_scroll_tts,
                center_spoken_sentence: config.center_spoken_sentence,
            },
            ui: UiConfig {
                show_tts: config.show_tts,
                show_settings: config.show_settings,
            },
            logging: LoggingConfig {
                log_level: config.log_level,
            },
            tts: TtsConfig {
                tts_model_path: config.tts_model_path.clone(),
                tts_espeak_path: config.tts_espeak_path.clone(),
                tts_speed: config.tts_speed,
                tts_threads: config.tts_threads,
            },
        }
    }
}

/// Theme mode.
#[derive(Debug, Clone, Copy, Deserialize, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ThemeMode {
    Day,
    Night,
}

impl Default for ThemeMode {
    fn default() -> Self {
        ThemeMode::Night
    }
}

impl std::fmt::Display for ThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            ThemeMode::Day => "Day",
            ThemeMode::Night => "Night",
        };
        write!(f, "{}", label)
    }
}

/// Font family options.
#[derive(Debug, Clone, Copy, Deserialize, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum FontFamily {
    Sans,
    Serif,
    Monospace,
    Lexend,
    FiraCode,
    AtkinsonHyperlegible,
    AtkinsonHyperlegibleNext,
    LexicaUltralegible,
    Courier,
    FrankGothic,
    Hermit,
    Hasklug,
    NotoSans,
}

impl Default for FontFamily {
    fn default() -> Self {
        FontFamily::Sans
    }
}

impl std::fmt::Display for FontFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            FontFamily::Sans => "Sans",
            FontFamily::Serif => "Serif",
            FontFamily::Monospace => "Monospace",
            FontFamily::Lexend => "Lexend",
            FontFamily::FiraCode => "Fira Code",
            FontFamily::AtkinsonHyperlegible => "Atkinson Hyperlegible",
            FontFamily::AtkinsonHyperlegibleNext => "Atkinson Hyperlegible Next",
            FontFamily::LexicaUltralegible => "Lexica Ultralegible",
            FontFamily::Courier => "Courier",
            FontFamily::FrankGothic => "Frank Gothic",
            FontFamily::Hermit => "Hermit",
            FontFamily::Hasklug => "Hasklug",
            FontFamily::NotoSans => "Noto Sans",
        };
        write!(f, "{}", label)
    }
}

/// Font weight options.
#[derive(Debug, Clone, Copy, Deserialize, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum FontWeight {
    Light,
    Normal,
    Bold,
}

impl Default for FontWeight {
    fn default() -> Self {
        FontWeight::Normal
    }
}

impl std::fmt::Display for FontWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            FontWeight::Light => "Light",
            FontWeight::Normal => "Normal",
            FontWeight::Bold => "Bold",
        };
        write!(f, "{}", label)
    }
}

/// Load configuration from the given path, falling back to defaults on error.
pub fn load_config(path: &Path) -> AppConfig {
    let contents = match fs::read_to_string(path) {
        Ok(data) => {
            info!(path = %path.display(), "Loaded base config");
            data
        }
        Err(err) => {
            warn!(
                path = %path.display(),
                "Falling back to default config: {err}"
            );
            return AppConfig::default();
        }
    };

    match parse_config(&contents) {
        Ok(cfg) => {
            debug!("Parsed configuration from disk");
            cfg
        }
        Err(err) => {
            warn!(path = %path.display(), "Invalid config TOML: {err}");
            AppConfig::default()
        }
    }
}

pub fn parse_config(contents: &str) -> Result<AppConfig, toml::de::Error> {
    let cfg = toml::from_str::<ConfigInput>(contents)?;
    Ok(match cfg {
        ConfigInput::Tables(tables) => tables.into(),
        ConfigInput::Flat(flat) => flat,
    })
}

pub fn serialize_config(config: &AppConfig) -> Result<String, toml::ser::Error> {
    toml::to_string(&ConfigTables::from(config))
}

fn default_font_size() -> u32 {
    16
}

fn default_line_spacing() -> f32 {
    1.2
}

fn default_margin() -> u16 {
    12
}

fn default_tts_model() -> String {
    "/usr/share/piper-voices/en/en_US/ryan/high/en_US-ryan-high.onnx".to_string()
}

fn default_tts_speed() -> f32 {
    2.5
}

fn default_tts_espeak_path() -> String {
    "/usr/share".to_string()
}

fn default_tts_threads() -> usize {
    16
}

fn default_show_tts() -> bool {
    true
}

fn default_show_settings() -> bool {
    true
}

fn default_day_highlight() -> HighlightColor {
    HighlightColor {
        r: 0.2,
        g: 0.4,
        b: 0.7,
        a: 0.15,
    }
}

fn default_night_highlight() -> HighlightColor {
    HighlightColor {
        r: 0.8,
        g: 0.8,
        b: 0.5,
        a: 0.2,
    }
}

fn default_log_level() -> LogLevel {
    LogLevel::Debug
}

fn default_lines_per_page() -> usize {
    28
}

fn default_pause_after_sentence() -> f32 {
    0.2
}

fn default_auto_scroll_tts() -> bool {
    false
}

fn default_center_spoken_sentence() -> bool {
    true
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct HighlightColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
struct AppearanceConfig {
    #[serde(default)]
    theme: ThemeMode,
    #[serde(default)]
    font_family: FontFamily,
    #[serde(default)]
    font_weight: FontWeight,
    #[serde(default = "default_font_size")]
    font_size: u32,
    #[serde(default = "default_line_spacing")]
    line_spacing: f32,
    #[serde(default)]
    word_spacing: u32,
    #[serde(default)]
    letter_spacing: u32,
    #[serde(default = "default_lines_per_page")]
    lines_per_page: usize,
    #[serde(default = "default_margin")]
    margin_horizontal: u16,
    #[serde(default = "default_margin")]
    margin_vertical: u16,
    #[serde(default = "default_day_highlight")]
    day_highlight: HighlightColor,
    #[serde(default = "default_night_highlight")]
    night_highlight: HighlightColor,
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        AppearanceConfig {
            theme: ThemeMode::default(),
            font_family: FontFamily::default(),
            font_weight: FontWeight::default(),
            font_size: default_font_size(),
            line_spacing: default_line_spacing(),
            word_spacing: 0,
            letter_spacing: 0,
            lines_per_page: default_lines_per_page(),
            margin_horizontal: default_margin(),
            margin_vertical: default_margin(),
            day_highlight: default_day_highlight(),
            night_highlight: default_night_highlight(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
struct ReadingBehaviorConfig {
    #[serde(default = "default_pause_after_sentence")]
    pause_after_sentence: f32,
    #[serde(default = "default_auto_scroll_tts")]
    auto_scroll_tts: bool,
    #[serde(default = "default_center_spoken_sentence")]
    center_spoken_sentence: bool,
}

impl Default for ReadingBehaviorConfig {
    fn default() -> Self {
        ReadingBehaviorConfig {
            pause_after_sentence: default_pause_after_sentence(),
            auto_scroll_tts: default_auto_scroll_tts(),
            center_spoken_sentence: default_center_spoken_sentence(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
struct UiConfig {
    #[serde(default = "default_show_tts")]
    show_tts: bool,
    #[serde(default = "default_show_settings")]
    show_settings: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        UiConfig {
            show_tts: default_show_tts(),
            show_settings: default_show_settings(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
struct LoggingConfig {
    #[serde(default = "default_log_level")]
    log_level: LogLevel,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        LoggingConfig {
            log_level: default_log_level(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
struct TtsConfig {
    #[serde(default = "default_tts_model")]
    tts_model_path: String,
    #[serde(default = "default_tts_espeak_path")]
    tts_espeak_path: String,
    #[serde(default = "default_tts_speed")]
    tts_speed: f32,
    #[serde(default = "default_tts_threads")]
    tts_threads: usize,
}

impl Default for TtsConfig {
    fn default() -> Self {
        TtsConfig {
            tts_model_path: default_tts_model(),
            tts_espeak_path: default_tts_espeak_path(),
            tts_speed: default_tts_speed(),
            tts_threads: default_tts_threads(),
        }
    }
}

/// Supported logging verbosity levels.
#[derive(Debug, Clone, Copy, Deserialize, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Debug
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        };
        write!(f, "{}", label)
    }
}

impl LogLevel {
    pub fn as_filter_str(self) -> &'static str {
        match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        }
    }
}
