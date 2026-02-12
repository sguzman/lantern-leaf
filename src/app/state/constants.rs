use crate::config::{FontFamily, FontWeight};
use iced::widget::scrollable::Id as ScrollId;
use once_cell::sync::Lazy;

/// Limits and defaults for reader controls.
pub(crate) const MAX_HORIZONTAL_MARGIN: u16 = 1000;
pub(crate) const MAX_VERTICAL_MARGIN: u16 = 100;
pub(crate) const MAX_WORD_SPACING: u32 = 5;
pub(crate) const MAX_LETTER_SPACING: u32 = 3;
pub(crate) const MIN_TTS_SPEED: f32 = 0.1;
pub(crate) const MAX_TTS_SPEED: f32 = 3.0;
pub(crate) const MIN_TTS_VOLUME: f32 = 0.0;
pub(crate) const MAX_TTS_VOLUME: f32 = 2.0;
pub(crate) const IMAGE_PREVIEW_HEIGHT_PX: f32 = 240.0;
pub(crate) const IMAGE_LABEL_FONT_SIZE_PX: f32 = 14.0;
pub(crate) const IMAGE_LABEL_LINE_HEIGHT: f32 = 1.0;
pub(crate) const IMAGE_BLOCK_SPACING_PX: f32 = 6.0;
pub(crate) const PAGE_FLOW_SPACING_PX: f32 = 12.0;
pub(crate) const IMAGE_FOOTER_FONT_SIZE_PX: f32 = 13.0;
pub(crate) const IMAGE_FOOTER_LINE_HEIGHT: f32 = 1.0;
pub(crate) static TEXT_SCROLL_ID: Lazy<ScrollId> = Lazy::new(|| ScrollId::new("text-scroll"));
pub(crate) const FONT_FAMILIES: [FontFamily; 13] = [
    FontFamily::Sans,
    FontFamily::Serif,
    FontFamily::Monospace,
    FontFamily::Lexend,
    FontFamily::FiraCode,
    FontFamily::AtkinsonHyperlegible,
    FontFamily::AtkinsonHyperlegibleNext,
    FontFamily::LexicaUltralegible,
    FontFamily::Courier,
    FontFamily::FrankGothic,
    FontFamily::Hermit,
    FontFamily::Hasklug,
    FontFamily::NotoSans,
];
pub(crate) const FONT_WEIGHTS: [FontWeight; 3] =
    [FontWeight::Light, FontWeight::Normal, FontWeight::Bold];
