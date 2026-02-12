use iced::widget::scrollable::RelativeOffset;

/// Bookmark and scroll tracking model.
pub struct BookmarkState {
    pub(in crate::app) last_scroll_offset: RelativeOffset,
    pub(in crate::app) viewport_fraction: f32,
    pub(in crate::app) viewport_width: f32,
    pub(in crate::app) viewport_height: f32,
    pub(in crate::app) content_width: f32,
    pub(in crate::app) content_height: f32,
    pub(in crate::app) pending_sentence_snap: Option<usize>,
}

pub struct TextOnlyPreview {
    pub(in crate::app) page: usize,
    pub(in crate::app) audio_sentences: Vec<String>,
    pub(in crate::app) display_to_audio: Vec<Option<usize>>,
    pub(in crate::app) audio_to_display: Vec<usize>,
}
