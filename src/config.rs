//! Configuration loading for the EPUB viewer.
//!
//! All user-tunable settings are centralized here and loaded from
//! `conf/config.toml` if present. Any missing or invalid entries fall back to
//! sensible defaults so the UI can still launch.

use serde::Deserialize;
use std::fs;
use std::path::Path;

/// High-level app configuration; deserializable from TOML.
#[derive(Debug, Clone, Deserialize)]
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
    pub justification: Justification,
    #[serde(default)]
    pub word_spacing: u32,
    #[serde(default)]
    pub letter_spacing: u32,
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
            justification: Justification::Left,
            word_spacing: 0,
            letter_spacing: 0,
        }
    }
}

/// Theme mode.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
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

/// Text justification.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Justification {
    Left,
    Center,
    Right,
    Justified,
}

impl Default for Justification {
    fn default() -> Self {
        Justification::Left
    }
}

impl std::fmt::Display for Justification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Justification::Left => "Left",
            Justification::Center => "Center",
            Justification::Right => "Right",
            Justification::Justified => "Justified",
        };
        write!(f, "{}", label)
    }
}

/// Load configuration from the given path, falling back to defaults on error.
pub fn load_config(path: &Path) -> AppConfig {
    let contents = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(_) => return AppConfig::default(),
    };

    toml::from_str::<AppConfig>(&contents).unwrap_or_default()
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
