use crate::{
    cancellation::CancellationToken, config, epub_loader, normalizer, pagination, text_utils,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use ts_rs::TS;

const BASE_WPM: f64 = 170.0;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, TS)]
#[ts(export)]
pub struct PanelState {
    pub show_settings: bool,
    pub show_stats: bool,
    pub show_tts: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum TtsPlaybackState {
    #[default]
    Idle,
    Playing,
    Paused,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ReaderSettingsView {
    pub theme: config::ThemeMode,
    pub font_family: config::FontFamily,
    pub font_weight: config::FontWeight,
    pub day_highlight: config::HighlightColor,
    pub night_highlight: config::HighlightColor,
    pub font_size: u32,
    pub line_spacing: f32,
    pub word_spacing: u32,
    pub letter_spacing: u32,
    pub margin_horizontal: u16,
    pub margin_vertical: u16,
    pub lines_per_page: usize,
    pub pause_after_sentence: f32,
    pub auto_scroll_tts: bool,
    pub center_spoken_sentence: bool,
    pub tts_speed: f32,
    pub tts_volume: f32,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ReaderTtsView {
    pub state: TtsPlaybackState,
    pub current_sentence_idx: Option<usize>,
    pub sentence_count: usize,
    pub can_seek_prev: bool,
    pub can_seek_next: bool,
    pub progress_pct: f64,
}

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export)]
pub struct ReaderSettingsPatch {
    #[ts(optional)]
    pub theme: Option<config::ThemeMode>,
    #[ts(optional)]
    pub day_highlight: Option<config::HighlightColor>,
    #[ts(optional)]
    pub night_highlight: Option<config::HighlightColor>,
    #[ts(optional)]
    pub font_family: Option<config::FontFamily>,
    #[ts(optional)]
    pub font_weight: Option<config::FontWeight>,
    #[ts(optional)]
    pub font_size: Option<u32>,
    #[ts(optional)]
    pub line_spacing: Option<f32>,
    #[ts(optional)]
    pub word_spacing: Option<u32>,
    #[ts(optional)]
    pub letter_spacing: Option<u32>,
    #[ts(optional)]
    pub margin_horizontal: Option<u16>,
    #[ts(optional)]
    pub margin_vertical: Option<u16>,
    #[ts(optional)]
    pub lines_per_page: Option<usize>,
    #[ts(optional)]
    pub pause_after_sentence: Option<f32>,
    #[ts(optional)]
    pub auto_scroll_tts: Option<bool>,
    #[ts(optional)]
    pub center_spoken_sentence: Option<bool>,
    #[ts(optional)]
    pub tts_speed: Option<f32>,
    #[ts(optional)]
    pub tts_volume: Option<f32>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ReaderStats {
    pub page_index: usize,
    pub total_pages: usize,
    pub tts_progress_pct: f64,
    pub page_time_remaining_secs: f64,
    pub book_time_remaining_secs: f64,
    pub page_word_count: usize,
    pub page_sentence_count: usize,
    pub page_start_percent: f64,
    pub page_end_percent: f64,
    pub words_read_up_to_page_start: usize,
    pub sentences_read_up_to_page_start: usize,
    pub words_read_up_to_page_end: usize,
    pub sentences_read_up_to_page_end: usize,
    pub words_read_up_to_current_position: usize,
    pub sentences_read_up_to_current_position: usize,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct ReaderSnapshot {
    pub source_path: String,
    pub source_name: String,
    pub current_page: usize,
    pub total_pages: usize,
    pub text_only_mode: bool,
    pub page_text: String,
    pub sentences: Vec<String>,
    pub highlighted_sentence_idx: Option<usize>,
    pub search_query: String,
    pub search_matches: Vec<usize>,
    pub selected_search_match: Option<usize>,
    pub settings: ReaderSettingsView,
    pub tts: ReaderTtsView,
    pub stats: ReaderStats,
    pub panels: PanelState,
}

#[derive(Debug, Clone)]
pub enum SessionCommand {
    GetSnapshot,
    NextPage,
    PrevPage,
    SetPage { page: usize },
    SentenceClick { sentence_idx: usize },
    NextSentence,
    PrevSentence,
    ToggleTextOnly,
    ApplySettings { patch: ReaderSettingsPatch },
    SearchSetQuery { query: String },
    SearchNext,
    SearchPrev,
    TtsPlay,
    TtsPause,
    TtsTogglePlayPause,
    TtsPlayFromPageStart,
    TtsPlayFromHighlight,
    TtsSeekNext,
    TtsSeekPrev,
    TtsRepeatSentence,
    TtsStop,
}

impl SessionCommand {
    pub fn action(&self) -> &'static str {
        match self {
            Self::GetSnapshot => "reader_get_snapshot",
            Self::NextPage => "reader_next_page",
            Self::PrevPage => "reader_prev_page",
            Self::SetPage { .. } => "reader_set_page",
            Self::SentenceClick { .. } => "reader_sentence_click",
            Self::NextSentence => "reader_next_sentence",
            Self::PrevSentence => "reader_prev_sentence",
            Self::ToggleTextOnly => "reader_toggle_text_only",
            Self::ApplySettings { .. } => "reader_apply_settings",
            Self::SearchSetQuery { .. } => "reader_search_set_query",
            Self::SearchNext => "reader_search_next",
            Self::SearchPrev => "reader_search_prev",
            Self::TtsPlay => "reader_tts_play",
            Self::TtsPause => "reader_tts_pause",
            Self::TtsTogglePlayPause => "reader_tts_toggle_play_pause",
            Self::TtsPlayFromPageStart => "reader_tts_play_from_page_start",
            Self::TtsPlayFromHighlight => "reader_tts_play_from_highlight",
            Self::TtsSeekNext => "reader_tts_seek_next",
            Self::TtsSeekPrev => "reader_tts_seek_prev",
            Self::TtsRepeatSentence => "reader_tts_repeat_sentence",
            Self::TtsStop => "reader_tts_stop",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SessionEvent {
    pub action: &'static str,
    pub snapshot: ReaderSnapshot,
}

#[derive(Debug, Clone)]
pub struct ReaderSession {
    pub source_path: PathBuf,
    source_name: String,
    full_text: String,
    pub config: config::AppConfig,
    pages: Vec<String>,
    raw_page_sentences: Vec<Vec<String>>,
    page_word_counts: Vec<usize>,
    page_sentence_counts: Vec<usize>,
    pub current_page: usize,
    highlighted_display_idx: Option<usize>,
    highlighted_audio_idx: Option<usize>,
    pub text_only_mode: bool,
    search_query: String,
    search_matches: Vec<usize>,
    selected_search_match: Option<usize>,
    tts_state: TtsPlaybackState,
    current_plan_page: Option<usize>,
    current_plan: Option<normalizer::PageNormalization>,
}

impl ReaderSession {
    pub fn load(
        source_path: PathBuf,
        config: config::AppConfig,
        normalizer: &normalizer::TextNormalizer,
        bookmark: Option<crate::cache::Bookmark>,
    ) -> Result<Self, String> {
        Self::load_with_cancel(source_path, config, normalizer, bookmark, None)
    }

    pub fn load_with_cancel(
        source_path: PathBuf,
        mut config: config::AppConfig,
        normalizer: &normalizer::TextNormalizer,
        bookmark: Option<crate::cache::Bookmark>,
        cancel: Option<&CancellationToken>,
    ) -> Result<Self, String> {
        let loaded = epub_loader::load_book_content_with_cancel(&source_path, cancel)
            .map_err(|err| err.to_string())?;
        let source_name = source_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("book")
            .to_string();

        // Cache-based overrides are loaded before calling this constructor; keep
        // runtime defaults in bounds regardless of config source.
        config.font_size = config
            .font_size
            .clamp(pagination::MIN_FONT_SIZE, pagination::MAX_FONT_SIZE);
        config.lines_per_page = config.lines_per_page.clamp(
            pagination::MIN_LINES_PER_PAGE,
            pagination::MAX_LINES_PER_PAGE,
        );

        let mut session = Self {
            source_path,
            source_name,
            full_text: loaded.text,
            config,
            pages: Vec::new(),
            raw_page_sentences: Vec::new(),
            page_word_counts: Vec::new(),
            page_sentence_counts: Vec::new(),
            current_page: 0,
            highlighted_display_idx: None,
            highlighted_audio_idx: None,
            text_only_mode: false,
            search_query: String::new(),
            search_matches: Vec::new(),
            selected_search_match: None,
            tts_state: TtsPlaybackState::Idle,
            current_plan_page: None,
            current_plan: None,
        };

        let restore_global_idx = bookmark
            .as_ref()
            .and_then(|bookmark| session.global_idx_for_bookmark(bookmark));
        if let Some(bookmark) = bookmark {
            session.current_page = bookmark.page;
        }
        session.repaginate(normalizer, restore_global_idx);
        if session.highlighted_display_idx.is_none() {
            session.highlighted_display_idx = Some(0).filter(|_| session.current_display_len() > 0);
        }
        Ok(session)
    }

    pub fn source_path_str(&self) -> String {
        self.source_path.to_string_lossy().to_string()
    }

    pub fn settings_view(&self) -> ReaderSettingsView {
        ReaderSettingsView {
            theme: self.config.theme,
            font_family: self.config.font_family,
            font_weight: self.config.font_weight,
            day_highlight: self.config.day_highlight,
            night_highlight: self.config.night_highlight,
            font_size: self.config.font_size,
            line_spacing: self.config.line_spacing,
            word_spacing: self.config.word_spacing,
            letter_spacing: self.config.letter_spacing,
            margin_horizontal: self.config.margin_horizontal,
            margin_vertical: self.config.margin_vertical,
            lines_per_page: self.config.lines_per_page,
            pause_after_sentence: self.config.pause_after_sentence,
            auto_scroll_tts: self.config.auto_scroll_tts,
            center_spoken_sentence: self.config.center_spoken_sentence,
            tts_speed: self.config.tts_speed,
            tts_volume: self.config.tts_volume,
        }
    }

    pub fn snapshot(
        &mut self,
        panels: PanelState,
        normalizer: &normalizer::TextNormalizer,
    ) -> ReaderSnapshot {
        let sentences = self.current_sentences(normalizer);
        let highlighted_sentence_idx = self.current_highlight_idx();
        let stats = self.stats(normalizer);
        let tts = self.tts_view(normalizer, stats.tts_progress_pct);
        ReaderSnapshot {
            source_path: self.source_path_str(),
            source_name: self.source_name.clone(),
            current_page: self.current_page,
            total_pages: self.pages.len(),
            text_only_mode: self.text_only_mode,
            page_text: self
                .pages
                .get(self.current_page)
                .cloned()
                .unwrap_or_else(String::new),
            sentences,
            highlighted_sentence_idx,
            search_query: self.search_query.clone(),
            search_matches: self.search_matches.clone(),
            selected_search_match: self.selected_search_match,
            settings: self.settings_view(),
            tts,
            stats,
            panels,
        }
    }

    pub fn apply_command(
        &mut self,
        command: SessionCommand,
        panels: PanelState,
        normalizer: &normalizer::TextNormalizer,
    ) -> SessionEvent {
        let action = command.action();
        match command {
            SessionCommand::GetSnapshot => {}
            SessionCommand::NextPage => self.next_page(normalizer),
            SessionCommand::PrevPage => self.prev_page(normalizer),
            SessionCommand::SetPage { page } => self.set_page(page, normalizer),
            SessionCommand::SentenceClick { sentence_idx } => {
                self.sentence_click(sentence_idx, normalizer)
            }
            SessionCommand::NextSentence => self.select_next_sentence(normalizer),
            SessionCommand::PrevSentence => self.select_prev_sentence(normalizer),
            SessionCommand::ToggleTextOnly => self.toggle_text_only(normalizer),
            SessionCommand::ApplySettings { patch } => self.apply_settings_patch(patch, normalizer),
            SessionCommand::SearchSetQuery { query } => self.set_search_query(query, normalizer),
            SessionCommand::SearchNext => self.search_next(normalizer),
            SessionCommand::SearchPrev => self.search_prev(normalizer),
            SessionCommand::TtsPlay => self.tts_play(normalizer),
            SessionCommand::TtsPause => self.tts_pause(),
            SessionCommand::TtsTogglePlayPause => self.tts_toggle_play_pause(normalizer),
            SessionCommand::TtsPlayFromPageStart => self.tts_play_from_page_start(normalizer),
            SessionCommand::TtsPlayFromHighlight => self.tts_play_from_highlight(normalizer),
            SessionCommand::TtsSeekNext => self.tts_seek_next(normalizer),
            SessionCommand::TtsSeekPrev => self.tts_seek_prev(normalizer),
            SessionCommand::TtsRepeatSentence => self.tts_repeat_current_sentence(normalizer),
            SessionCommand::TtsStop => self.tts_stop(),
        }
        SessionEvent {
            action,
            snapshot: self.snapshot(panels, normalizer),
        }
    }

    pub fn apply_settings_patch(
        &mut self,
        patch: ReaderSettingsPatch,
        normalizer: &normalizer::TextNormalizer,
    ) {
        let preserve = self.global_display_idx();
        let mut repaginate = false;

        if let Some(theme) = patch.theme {
            self.config.theme = theme;
        }
        if let Some(day_highlight) = patch.day_highlight {
            self.config.day_highlight = config::HighlightColor {
                r: day_highlight.r.clamp(0.0, 1.0),
                g: day_highlight.g.clamp(0.0, 1.0),
                b: day_highlight.b.clamp(0.0, 1.0),
                a: day_highlight.a.clamp(0.0, 1.0),
            };
        }
        if let Some(night_highlight) = patch.night_highlight {
            self.config.night_highlight = config::HighlightColor {
                r: night_highlight.r.clamp(0.0, 1.0),
                g: night_highlight.g.clamp(0.0, 1.0),
                b: night_highlight.b.clamp(0.0, 1.0),
                a: night_highlight.a.clamp(0.0, 1.0),
            };
        }
        if let Some(font_family) = patch.font_family {
            self.config.font_family = font_family;
        }
        if let Some(font_weight) = patch.font_weight {
            self.config.font_weight = font_weight;
        }
        if let Some(font_size) = patch.font_size {
            let clamped = font_size.clamp(pagination::MIN_FONT_SIZE, pagination::MAX_FONT_SIZE);
            if clamped != self.config.font_size {
                self.config.font_size = clamped;
                repaginate = true;
            }
        }
        if let Some(lines) = patch.lines_per_page {
            let clamped = lines.clamp(
                pagination::MIN_LINES_PER_PAGE,
                pagination::MAX_LINES_PER_PAGE,
            );
            if clamped != self.config.lines_per_page {
                self.config.lines_per_page = clamped;
                repaginate = true;
            }
        }
        if let Some(margin_horizontal) = patch.margin_horizontal {
            self.config.margin_horizontal = margin_horizontal.clamp(0, 600);
        }
        if let Some(margin_vertical) = patch.margin_vertical {
            self.config.margin_vertical = margin_vertical.clamp(0, 240);
        }
        if let Some(line_spacing) = patch.line_spacing {
            self.config.line_spacing = line_spacing.clamp(0.8, 3.0);
        }
        if let Some(word_spacing) = patch.word_spacing {
            self.config.word_spacing = word_spacing.clamp(0, 24);
        }
        if let Some(letter_spacing) = patch.letter_spacing {
            self.config.letter_spacing = letter_spacing.clamp(0, 24);
        }
        if let Some(pause) = patch.pause_after_sentence {
            let rounded = ((pause.clamp(0.0, 3.0) * 100.0).round()) / 100.0;
            self.config.pause_after_sentence = rounded;
        }
        if let Some(auto_scroll_tts) = patch.auto_scroll_tts {
            self.config.auto_scroll_tts = auto_scroll_tts;
        }
        if let Some(center_spoken_sentence) = patch.center_spoken_sentence {
            self.config.center_spoken_sentence = center_spoken_sentence;
        }
        if let Some(tts_speed) = patch.tts_speed {
            self.config.tts_speed = tts_speed.clamp(0.25, 4.0);
        }
        if let Some(tts_volume) = patch.tts_volume {
            self.config.tts_volume = tts_volume.clamp(0.0, 2.0);
        }

        if repaginate {
            self.repaginate(normalizer, preserve);
        }
    }

    pub fn set_search_query(&mut self, query: String, normalizer: &normalizer::TextNormalizer) {
        self.search_query = query;
        self.update_search_matches(normalizer);
        self.apply_selected_match_as_highlight(normalizer);
    }

    pub fn search_next(&mut self, normalizer: &normalizer::TextNormalizer) {
        if self.search_matches.is_empty() {
            self.selected_search_match = None;
            return;
        }
        self.selected_search_match = Some(match self.selected_search_match {
            Some(current) => (current + 1) % self.search_matches.len(),
            None => 0,
        });
        self.apply_selected_match_as_highlight(normalizer);
    }

    pub fn search_prev(&mut self, normalizer: &normalizer::TextNormalizer) {
        if self.search_matches.is_empty() {
            self.selected_search_match = None;
            return;
        }
        self.selected_search_match = Some(match self.selected_search_match {
            Some(0) | None => self.search_matches.len().saturating_sub(1),
            Some(current) => current.saturating_sub(1),
        });
        self.apply_selected_match_as_highlight(normalizer);
    }

    pub fn next_page(&mut self, normalizer: &normalizer::TextNormalizer) {
        if self.current_page + 1 >= self.pages.len() {
            return;
        }
        self.current_page += 1;
        self.highlighted_display_idx = Some(0).filter(|_| self.current_display_len() > 0);
        self.highlighted_audio_idx = None;
        self.current_plan_page = None;
        self.current_plan = None;
        if self.text_only_mode {
            self.highlighted_audio_idx = self
                .highlighted_display_idx
                .and_then(|idx| self.map_display_to_audio_idx(normalizer, idx));
        }
        self.update_search_matches(normalizer);
    }

    pub fn prev_page(&mut self, normalizer: &normalizer::TextNormalizer) {
        if self.current_page == 0 {
            return;
        }
        self.current_page = self.current_page.saturating_sub(1);
        self.highlighted_display_idx = Some(0).filter(|_| self.current_display_len() > 0);
        self.highlighted_audio_idx = None;
        self.current_plan_page = None;
        self.current_plan = None;
        if self.text_only_mode {
            self.highlighted_audio_idx = self
                .highlighted_display_idx
                .and_then(|idx| self.map_display_to_audio_idx(normalizer, idx));
        }
        self.update_search_matches(normalizer);
    }

    pub fn set_page(&mut self, page: usize, normalizer: &normalizer::TextNormalizer) {
        if self.pages.is_empty() {
            self.current_page = 0;
            return;
        }
        self.current_page = page.min(self.pages.len().saturating_sub(1));
        self.highlighted_display_idx = Some(0).filter(|_| self.current_display_len() > 0);
        self.highlighted_audio_idx = None;
        self.current_plan_page = None;
        self.current_plan = None;
        if self.text_only_mode {
            self.highlighted_audio_idx = self
                .highlighted_display_idx
                .and_then(|idx| self.map_display_to_audio_idx(normalizer, idx));
        }
        self.update_search_matches(normalizer);
    }

    pub fn sentence_click(&mut self, sentence_idx: usize, normalizer: &normalizer::TextNormalizer) {
        if self.text_only_mode {
            let plan = self.ensure_current_plan(normalizer);
            if sentence_idx >= plan.audio_sentences.len() {
                return;
            }
            self.highlighted_audio_idx = Some(sentence_idx);
            self.highlighted_display_idx = self.map_audio_to_display_idx(normalizer, sentence_idx);
            return;
        }

        if sentence_idx >= self.current_display_len() {
            return;
        }
        self.highlighted_display_idx = Some(sentence_idx);
        self.highlighted_audio_idx = self.map_display_to_audio_idx(normalizer, sentence_idx);
    }

    pub fn select_next_sentence(&mut self, normalizer: &normalizer::TextNormalizer) {
        let count = self.current_sentences(normalizer).len();
        if count == 0 {
            return;
        }
        let current = self
            .current_highlight_idx()
            .unwrap_or(0)
            .min(count.saturating_sub(1));
        let next = (current + 1).min(count.saturating_sub(1));
        self.sentence_click(next, normalizer);
    }

    pub fn select_prev_sentence(&mut self, normalizer: &normalizer::TextNormalizer) {
        let count = self.current_sentences(normalizer).len();
        if count == 0 {
            return;
        }
        let current = self
            .current_highlight_idx()
            .unwrap_or(0)
            .min(count.saturating_sub(1));
        let prev = current.saturating_sub(1);
        self.sentence_click(prev, normalizer);
    }

    pub fn toggle_text_only(&mut self, normalizer: &normalizer::TextNormalizer) {
        self.text_only_mode = !self.text_only_mode;
        if self.text_only_mode {
            let display_idx = self.highlighted_display_idx.unwrap_or(0);
            self.highlighted_audio_idx = self.map_display_to_audio_idx(normalizer, display_idx);
        } else if let Some(audio_idx) = self.highlighted_audio_idx {
            self.highlighted_display_idx = self.map_audio_to_display_idx(normalizer, audio_idx);
        }
        self.update_search_matches(normalizer);
    }

    pub fn tts_play(&mut self, normalizer: &normalizer::TextNormalizer) {
        let count = self.current_sentences(normalizer).len();
        if count == 0 {
            self.tts_state = TtsPlaybackState::Idle;
            return;
        }
        if self.current_highlight_idx().is_none() {
            self.sentence_click(0, normalizer);
        }
        self.tts_state = TtsPlaybackState::Playing;
    }

    pub fn tts_pause(&mut self) {
        if self.tts_state == TtsPlaybackState::Playing {
            self.tts_state = TtsPlaybackState::Paused;
        }
    }

    pub fn tts_toggle_play_pause(&mut self, normalizer: &normalizer::TextNormalizer) {
        if self.tts_state == TtsPlaybackState::Playing {
            self.tts_pause();
        } else {
            self.tts_play(normalizer);
        }
    }

    pub fn tts_play_from_page_start(&mut self, normalizer: &normalizer::TextNormalizer) {
        let count = self.current_sentences(normalizer).len();
        if count == 0 {
            self.tts_state = TtsPlaybackState::Idle;
            return;
        }
        self.sentence_click(0, normalizer);
        self.tts_state = TtsPlaybackState::Playing;
    }

    pub fn tts_play_from_highlight(&mut self, normalizer: &normalizer::TextNormalizer) {
        if self.current_highlight_idx().is_none() {
            self.tts_play_from_page_start(normalizer);
            return;
        }
        self.tts_state = TtsPlaybackState::Playing;
    }

    pub fn tts_seek_next(&mut self, normalizer: &normalizer::TextNormalizer) {
        if self.move_highlight_relative(1, normalizer) {
            return;
        }
        // Keep playback paused/idle at the end; don't auto-reset to start.
        if self.tts_state == TtsPlaybackState::Playing {
            self.tts_state = TtsPlaybackState::Paused;
        }
    }

    pub fn tts_seek_prev(&mut self, normalizer: &normalizer::TextNormalizer) {
        let _ = self.move_highlight_relative(-1, normalizer);
    }

    pub fn tts_repeat_current_sentence(&mut self, normalizer: &normalizer::TextNormalizer) {
        if self.current_highlight_idx().is_none() {
            self.tts_play_from_page_start(normalizer);
        }
    }

    pub fn tts_stop(&mut self) {
        self.tts_state = TtsPlaybackState::Idle;
    }

    pub fn to_bookmark(&self) -> crate::cache::Bookmark {
        crate::cache::Bookmark {
            page: self.current_page,
            sentence_idx: self.current_highlight_idx(),
            sentence_text: None,
            scroll_y: 0.0,
        }
    }

    fn repaginate(
        &mut self,
        normalizer: &normalizer::TextNormalizer,
        preserve_global_idx: Option<usize>,
    ) {
        self.pages = pagination::paginate(
            &self.full_text,
            self.config.font_size,
            self.config.lines_per_page,
        );
        if self.pages.is_empty() {
            self.pages.push(String::new());
        }
        self.raw_page_sentences = self
            .pages
            .iter()
            .map(|page| text_utils::split_sentences(page))
            .collect();
        self.page_word_counts = self
            .pages
            .iter()
            .map(|page| page.split_whitespace().count())
            .collect();
        self.page_sentence_counts = self.raw_page_sentences.iter().map(Vec::len).collect();

        self.current_page = self.current_page.min(self.pages.len().saturating_sub(1));
        self.current_plan_page = None;
        self.current_plan = None;

        if let Some(global_idx) = preserve_global_idx {
            let (page, idx) = self.page_idx_for_global_sentence(global_idx);
            self.current_page = page;
            self.highlighted_display_idx = Some(idx);
        } else {
            self.highlighted_display_idx = Some(0).filter(|_| self.current_display_len() > 0);
        }

        self.highlighted_audio_idx = None;
        if self.text_only_mode {
            self.highlighted_audio_idx = self
                .highlighted_display_idx
                .and_then(|idx| self.map_display_to_audio_idx(normalizer, idx));
        }
        self.update_search_matches(normalizer);
    }

    fn global_idx_for_bookmark(&self, bookmark: &crate::cache::Bookmark) -> Option<usize> {
        let sentence_idx = bookmark.sentence_idx?;
        let page = bookmark
            .page
            .min(self.page_sentence_counts.len().saturating_sub(1));
        let base: usize = self.page_sentence_counts.iter().take(page).sum();
        Some(base + sentence_idx)
    }

    fn page_idx_for_global_sentence(&self, global_idx: usize) -> (usize, usize) {
        if self.page_sentence_counts.is_empty() {
            return (0, 0);
        }
        let mut remaining = global_idx;
        for (page_idx, count) in self.page_sentence_counts.iter().copied().enumerate() {
            if count == 0 {
                continue;
            }
            if remaining < count {
                return (page_idx, remaining);
            }
            remaining = remaining.saturating_sub(count);
        }
        let last_page = self.page_sentence_counts.len().saturating_sub(1);
        let last_idx = self.page_sentence_counts[last_page].saturating_sub(1);
        (last_page, last_idx)
    }

    fn current_display_len(&self) -> usize {
        self.raw_page_sentences
            .get(self.current_page)
            .map(Vec::len)
            .unwrap_or(0)
    }

    fn ensure_current_plan(
        &mut self,
        normalizer: &normalizer::TextNormalizer,
    ) -> normalizer::PageNormalization {
        let needs_refresh = self.current_plan_page != Some(self.current_page);
        if needs_refresh {
            let display = self
                .raw_page_sentences
                .get(self.current_page)
                .cloned()
                .unwrap_or_default();
            let plan = normalizer.plan_page_cached(&self.source_path, self.current_page, &display);
            self.current_plan_page = Some(self.current_page);
            self.current_plan = Some(plan);
        }

        self.current_plan
            .clone()
            .unwrap_or(normalizer::PageNormalization {
                audio_sentences: Vec::new(),
                display_to_audio: Vec::new(),
                audio_to_display: Vec::new(),
            })
    }

    fn map_display_to_audio_idx(
        &mut self,
        normalizer: &normalizer::TextNormalizer,
        display_idx: usize,
    ) -> Option<usize> {
        let plan = self.ensure_current_plan(normalizer);
        if plan.display_to_audio.is_empty() {
            return None;
        }
        let clamped = display_idx.min(plan.display_to_audio.len().saturating_sub(1));
        plan.display_to_audio
            .iter()
            .skip(clamped)
            .find_map(|mapped| *mapped)
            .or_else(|| {
                plan.display_to_audio
                    .iter()
                    .take(clamped + 1)
                    .rev()
                    .find_map(|mapped| *mapped)
            })
    }

    fn map_audio_to_display_idx(
        &mut self,
        normalizer: &normalizer::TextNormalizer,
        audio_idx: usize,
    ) -> Option<usize> {
        let plan = self.ensure_current_plan(normalizer);
        plan.audio_to_display.get(audio_idx).copied()
    }

    fn current_sentences(&mut self, normalizer: &normalizer::TextNormalizer) -> Vec<String> {
        if self.text_only_mode {
            return self.ensure_current_plan(normalizer).audio_sentences;
        }
        self.raw_page_sentences
            .get(self.current_page)
            .cloned()
            .unwrap_or_default()
    }

    fn current_highlight_idx(&self) -> Option<usize> {
        if self.text_only_mode {
            self.highlighted_audio_idx
        } else {
            self.highlighted_display_idx
        }
    }

    fn global_display_idx(&self) -> Option<usize> {
        let page_base: usize = self
            .page_sentence_counts
            .iter()
            .take(self.current_page)
            .sum();
        self.highlighted_display_idx.map(|idx| page_base + idx)
    }

    fn tts_view(
        &mut self,
        normalizer: &normalizer::TextNormalizer,
        progress_pct: f64,
    ) -> ReaderTtsView {
        let sentence_count = self.current_sentences(normalizer).len();
        let current_sentence_idx = self.current_highlight_idx();
        let can_seek_prev = if let Some(idx) = current_sentence_idx {
            idx > 0 || self.has_sentence_before_current_page()
        } else {
            self.has_sentence_before_current_page()
        };
        let can_seek_next = if let Some(idx) = current_sentence_idx {
            idx + 1 < sentence_count || self.has_sentence_after_current_page()
        } else {
            sentence_count > 0 || self.has_sentence_after_current_page()
        };
        ReaderTtsView {
            state: self.tts_state,
            current_sentence_idx,
            sentence_count,
            can_seek_prev,
            can_seek_next,
            progress_pct: (progress_pct * 1000.0).round() / 1000.0,
        }
    }

    fn has_sentence_before_current_page(&self) -> bool {
        self.page_sentence_counts
            .iter()
            .take(self.current_page)
            .any(|count| *count > 0)
    }

    fn has_sentence_after_current_page(&self) -> bool {
        self.page_sentence_counts
            .iter()
            .skip(self.current_page.saturating_add(1))
            .any(|count| *count > 0)
    }

    fn move_highlight_relative(
        &mut self,
        delta: isize,
        normalizer: &normalizer::TextNormalizer,
    ) -> bool {
        if delta == 0 {
            return self.current_highlight_idx().is_some();
        }

        let count = self.current_sentences(normalizer).len();
        if count == 0 {
            if delta > 0 {
                return self.move_to_adjacent_page_with_sentences(1, normalizer);
            }
            return self.move_to_adjacent_page_with_sentences(-1, normalizer);
        }

        let current = self
            .current_highlight_idx()
            .unwrap_or(0)
            .min(count.saturating_sub(1));
        if delta > 0 {
            let next = current.saturating_add(delta as usize);
            if next < count {
                self.sentence_click(next, normalizer);
                return true;
            }
            if self.move_to_adjacent_page_with_sentences(1, normalizer) {
                self.sentence_click(0, normalizer);
                return true;
            }
            return false;
        }

        let back = delta.unsigned_abs();
        if current >= back {
            self.sentence_click(current - back, normalizer);
            return true;
        }
        if self.move_to_adjacent_page_with_sentences(-1, normalizer) {
            let new_count = self.current_sentences(normalizer).len();
            if new_count > 0 {
                self.sentence_click(new_count - 1, normalizer);
                return true;
            }
        }
        false
    }

    fn move_to_adjacent_page_with_sentences(
        &mut self,
        direction: isize,
        normalizer: &normalizer::TextNormalizer,
    ) -> bool {
        if direction == 0 || self.pages.is_empty() {
            return false;
        }
        let mut page = self.current_page as isize + direction;
        while page >= 0 && (page as usize) < self.pages.len() {
            let idx = page as usize;
            if self.page_sentence_counts.get(idx).copied().unwrap_or(0) > 0 {
                self.set_page(idx, normalizer);
                return true;
            }
            page += direction;
        }
        false
    }

    fn update_search_matches(&mut self, normalizer: &normalizer::TextNormalizer) {
        self.search_matches.clear();
        self.selected_search_match = None;
        let query = self.search_query.trim().to_string();
        if query.is_empty() {
            return;
        }

        let sentences = self.current_sentences(normalizer);
        let regex = Regex::new(&query).ok();
        let query_lower = query.to_ascii_lowercase();
        for (idx, sentence) in sentences.iter().enumerate() {
            let matched = if let Some(regex) = &regex {
                regex.is_match(sentence)
            } else {
                sentence.to_ascii_lowercase().contains(&query_lower)
            };
            if matched {
                self.search_matches.push(idx);
            }
        }
        if !self.search_matches.is_empty() {
            self.selected_search_match = Some(0);
        }
    }

    fn apply_selected_match_as_highlight(&mut self, normalizer: &normalizer::TextNormalizer) {
        let Some(selected_idx) = self.selected_search_match else {
            return;
        };
        let Some(sentence_idx) = self.search_matches.get(selected_idx).copied() else {
            return;
        };
        self.sentence_click(sentence_idx, normalizer);
    }

    fn stats(&mut self, normalizer: &normalizer::TextNormalizer) -> ReaderStats {
        let page_word_count = self
            .page_word_counts
            .get(self.current_page)
            .copied()
            .unwrap_or_default();
        let page_sentence_count = self
            .page_sentence_counts
            .get(self.current_page)
            .copied()
            .unwrap_or_default();
        let words_before_page: usize = self.page_word_counts.iter().take(self.current_page).sum();
        let sentences_before_page: usize = self
            .page_sentence_counts
            .iter()
            .take(self.current_page)
            .sum();
        let words_up_to_page_end = words_before_page + page_word_count;
        let sentences_up_to_page_end = sentences_before_page + page_sentence_count;
        let total_words = self.page_word_counts.iter().sum::<usize>().max(1);

        let (progress_fraction, sentence_progress_count, sentence_progress_total) =
            if self.text_only_mode {
                let plan = self.ensure_current_plan(normalizer);
                let count = plan.audio_sentences.len();
                let idx = self.highlighted_audio_idx.unwrap_or(0);
                let clamped_idx = idx.min(count.saturating_sub(1));
                let fraction = if count == 0 {
                    0.0
                } else {
                    (clamped_idx + 1) as f64 / count as f64
                };
                (fraction, clamped_idx + 1, count)
            } else {
                let count = page_sentence_count;
                let idx = self.highlighted_display_idx.unwrap_or(0);
                let clamped_idx = idx.min(count.saturating_sub(1));
                let fraction = if count == 0 {
                    0.0
                } else {
                    (clamped_idx + 1) as f64 / count as f64
                };
                (fraction, clamped_idx + 1, count)
            };

        let tts_progress_pct = progress_fraction * 100.0;
        let words_up_to_current_position =
            words_before_page + ((page_word_count as f64) * progress_fraction).round() as usize;
        let sentences_up_to_current_position = sentences_before_page
            + (((page_sentence_count as f64) * progress_fraction).round() as usize).min(
                page_sentence_count
                    .max(sentence_progress_count)
                    .min(sentence_progress_total.max(1)),
            );

        let effective_wpm = (BASE_WPM * self.config.tts_speed as f64).max(40.0);
        let page_total_secs = (page_word_count as f64 / effective_wpm) * 60.0;
        let page_time_remaining_secs = page_total_secs * (1.0 - progress_fraction);
        let book_total_secs = (total_words as f64 / effective_wpm) * 60.0;
        let global_word_progress =
            (words_up_to_current_position as f64 / total_words as f64).clamp(0.0, 1.0);
        let book_time_remaining_secs = book_total_secs * (1.0 - global_word_progress);

        let page_start_percent = (words_before_page as f64 / total_words as f64) * 100.0;
        let page_end_percent = (words_up_to_page_end as f64 / total_words as f64) * 100.0;

        ReaderStats {
            page_index: self.current_page + 1,
            total_pages: self.pages.len(),
            tts_progress_pct,
            page_time_remaining_secs,
            book_time_remaining_secs,
            page_word_count,
            page_sentence_count,
            page_start_percent,
            page_end_percent,
            words_read_up_to_page_start: words_before_page,
            sentences_read_up_to_page_start: sentences_before_page,
            words_read_up_to_page_end: words_up_to_page_end,
            sentences_read_up_to_page_end: sentences_up_to_page_end,
            words_read_up_to_current_position: words_up_to_current_position,
            sentences_read_up_to_current_position: sentences_up_to_current_position,
        }
    }
}

pub fn load_session_for_source(
    source_path: PathBuf,
    base_config: &config::AppConfig,
    normalizer: &normalizer::TextNormalizer,
) -> Result<ReaderSession, String> {
    load_session_for_source_with_cancel(source_path, base_config, normalizer, None)
}

pub fn load_session_for_source_with_cancel(
    source_path: PathBuf,
    base_config: &config::AppConfig,
    normalizer: &normalizer::TextNormalizer,
    cancel: Option<&CancellationToken>,
) -> Result<ReaderSession, String> {
    let mut effective_config = base_config.clone();
    if let Some(mut overrides) = crate::cache::load_epub_config(&source_path) {
        overrides.log_level = base_config.log_level;
        overrides.tts_threads = base_config.tts_threads;
        overrides.tts_progress_log_interval_secs = base_config.tts_progress_log_interval_secs;
        overrides.key_toggle_play_pause = base_config.key_toggle_play_pause.clone();
        overrides.key_safe_quit = base_config.key_safe_quit.clone();
        overrides.key_next_sentence = base_config.key_next_sentence.clone();
        overrides.key_prev_sentence = base_config.key_prev_sentence.clone();
        overrides.key_repeat_sentence = base_config.key_repeat_sentence.clone();
        overrides.key_toggle_search = base_config.key_toggle_search.clone();
        overrides.key_toggle_settings = base_config.key_toggle_settings.clone();
        overrides.key_toggle_stats = base_config.key_toggle_stats.clone();
        overrides.key_toggle_tts = base_config.key_toggle_tts.clone();
        effective_config = overrides;
    }
    let bookmark = crate::cache::load_bookmark(&source_path);
    ReaderSession::load_with_cancel(source_path, effective_config, normalizer, bookmark, cancel)
}

pub fn persist_session_housekeeping(session: &ReaderSession) {
    let bookmark = session.to_bookmark();
    crate::cache::save_bookmark(Path::new(&session.source_path), &bookmark);
    crate::cache::save_epub_config(Path::new(&session.source_path), &session.config);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_session(page_sentences: &[&[&str]]) -> ReaderSession {
        let pages: Vec<String> = page_sentences
            .iter()
            .map(|sentences| sentences.join(" "))
            .collect();
        let raw_page_sentences: Vec<Vec<String>> = page_sentences
            .iter()
            .map(|sentences| {
                sentences
                    .iter()
                    .map(|sentence| sentence.to_string())
                    .collect()
            })
            .collect();
        let page_word_counts: Vec<usize> = pages
            .iter()
            .map(|page| page.split_whitespace().count())
            .collect();
        let page_sentence_counts: Vec<usize> = raw_page_sentences.iter().map(Vec::len).collect();

        ReaderSession {
            source_path: PathBuf::from("/tmp/test.epub"),
            source_name: "test.epub".to_string(),
            full_text: pages.join("\n\n"),
            config: config::AppConfig::default(),
            pages,
            raw_page_sentences,
            page_word_counts,
            page_sentence_counts,
            current_page: 0,
            highlighted_display_idx: Some(0),
            highlighted_audio_idx: None,
            text_only_mode: false,
            search_query: String::new(),
            search_matches: Vec::new(),
            selected_search_match: None,
            tts_state: TtsPlaybackState::Paused,
            current_plan_page: None,
            current_plan: None,
        }
    }

    #[test]
    fn paused_state_is_preserved_when_changing_pages() {
        let normalizer = normalizer::TextNormalizer::default();
        let mut session = build_test_session(&[&["A.", "B."], &["C.", "D."]]);
        session.current_page = 0;
        session.highlighted_display_idx = Some(1);
        session.tts_state = TtsPlaybackState::Paused;

        session.next_page(&normalizer);

        assert_eq!(session.current_page, 1);
        assert_eq!(session.current_highlight_idx(), Some(0));
        assert_eq!(session.tts_state, TtsPlaybackState::Paused);
    }

    #[test]
    fn paused_state_is_preserved_when_seeking_next_sentence() {
        let normalizer = normalizer::TextNormalizer::default();
        let mut session = build_test_session(&[&["A.", "B.", "C."]]);
        session.highlighted_display_idx = Some(0);
        session.tts_state = TtsPlaybackState::Paused;

        session.tts_seek_next(&normalizer);

        assert_eq!(session.current_highlight_idx(), Some(1));
        assert_eq!(session.tts_state, TtsPlaybackState::Paused);
    }

    #[test]
    fn paused_state_is_preserved_when_seeking_prev_across_page_boundary() {
        let normalizer = normalizer::TextNormalizer::default();
        let mut session = build_test_session(&[&["A."], &["B."]]);
        session.current_page = 1;
        session.highlighted_display_idx = Some(0);
        session.tts_state = TtsPlaybackState::Paused;

        session.tts_seek_prev(&normalizer);

        assert_eq!(session.current_page, 0);
        assert_eq!(session.current_highlight_idx(), Some(0));
        assert_eq!(session.tts_state, TtsPlaybackState::Paused);
    }

    #[test]
    fn sentence_click_keeps_paused_state() {
        let normalizer = normalizer::TextNormalizer::default();
        let mut session = build_test_session(&[&["A.", "B.", "C."]]);
        session.highlighted_display_idx = Some(0);
        session.tts_state = TtsPlaybackState::Paused;

        session.sentence_click(2, &normalizer);

        assert_eq!(session.current_highlight_idx(), Some(2));
        assert_eq!(session.tts_state, TtsPlaybackState::Paused);
    }

    #[test]
    fn text_only_sentence_click_uses_audio_index_mapping() {
        let normalizer = normalizer::TextNormalizer::default();
        let mut session = build_test_session(&[&[
            r#"In the word lists of Cheshire, Derbyshire, Lancashire and Yorkshire we find the following terms, all of which took root in the Delaware Valley: abide as in cannot abide it, all out for entirely, apple-pie order to mean very good order, bamboozle for deceive, black and white for writing, blather for empty talk, boggle for take fright, brat for child, budge for move, burying for funeral, by golly as an expletive, by gum for another expletive."#,
        ]]);
        session.tts_state = TtsPlaybackState::Paused;
        session.toggle_text_only(&normalizer);
        let audio_count = session.current_sentences(&normalizer).len();
        assert!(
            audio_count > 1,
            "expected long sentence to split into multiple audio chunks"
        );

        let target_audio_idx = audio_count - 1;
        session.sentence_click(target_audio_idx, &normalizer);

        assert_eq!(session.highlighted_audio_idx, Some(target_audio_idx));
        assert_eq!(session.highlighted_display_idx, Some(0));
        assert_eq!(session.current_highlight_idx(), Some(target_audio_idx));
        assert_eq!(session.tts_state, TtsPlaybackState::Paused);
    }

    #[test]
    fn apply_settings_patch_clamps_pause_speed_and_volume() {
        let normalizer = normalizer::TextNormalizer::default();
        let mut session = build_test_session(&[&["A.", "B."]]);

        session.apply_settings_patch(
            ReaderSettingsPatch {
                theme: None,
                day_highlight: None,
                night_highlight: None,
                font_family: None,
                font_weight: None,
                font_size: None,
                line_spacing: None,
                word_spacing: None,
                letter_spacing: None,
                margin_horizontal: None,
                margin_vertical: None,
                lines_per_page: None,
                pause_after_sentence: Some(0.056),
                auto_scroll_tts: None,
                center_spoken_sentence: None,
                tts_speed: Some(4.9),
                tts_volume: Some(-1.0),
            },
            &normalizer,
        );

        assert!((session.config.pause_after_sentence - 0.06).abs() < f32::EPSILON);
        assert!((session.config.tts_speed - 4.0).abs() < f32::EPSILON);
        assert!((session.config.tts_volume - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn tts_stop_forces_idle_state() {
        let normalizer = normalizer::TextNormalizer::default();
        let mut session = build_test_session(&[&["A.", "B."]]);
        session.tts_play(&normalizer);
        assert_eq!(session.tts_state, TtsPlaybackState::Playing);

        session.tts_stop();

        assert_eq!(session.tts_state, TtsPlaybackState::Idle);
    }

    #[test]
    fn session_command_dispatch_emits_expected_action_and_snapshot() {
        let normalizer = normalizer::TextNormalizer::default();
        let mut session = build_test_session(&[&["A.", "B."], &["C.", "D."]]);

        let event =
            session.apply_command(SessionCommand::NextPage, PanelState::default(), &normalizer);

        assert_eq!(event.action, "reader_next_page");
        assert_eq!(event.snapshot.current_page, 1);
        assert_eq!(event.snapshot.highlighted_sentence_idx, Some(0));
    }

    #[test]
    fn session_command_dispatch_preserves_paused_tts_state() {
        let normalizer = normalizer::TextNormalizer::default();
        let mut session = build_test_session(&[&["A.", "B."], &["C.", "D."]]);
        session.tts_state = TtsPlaybackState::Paused;

        let event =
            session.apply_command(SessionCommand::NextPage, PanelState::default(), &normalizer);

        assert_eq!(session.tts_state, TtsPlaybackState::Paused);
        assert_eq!(event.snapshot.tts.state, TtsPlaybackState::Paused);
    }

    #[test]
    fn session_command_dispatch_applies_settings_patch_with_rounding() {
        let normalizer = normalizer::TextNormalizer::default();
        let mut session = build_test_session(&[&["A.", "B."]]);

        let event = session.apply_command(
            SessionCommand::ApplySettings {
                patch: ReaderSettingsPatch {
                    theme: None,
                    day_highlight: None,
                    night_highlight: None,
                    font_family: None,
                    font_weight: None,
                    font_size: None,
                    line_spacing: None,
                    word_spacing: None,
                    letter_spacing: None,
                    margin_horizontal: None,
                    margin_vertical: None,
                    lines_per_page: None,
                    pause_after_sentence: Some(0.056),
                    auto_scroll_tts: None,
                    center_spoken_sentence: None,
                    tts_speed: Some(2.5),
                    tts_volume: Some(1.3),
                },
            },
            PanelState::default(),
            &normalizer,
        );

        assert_eq!(event.action, "reader_apply_settings");
        assert!((event.snapshot.settings.pause_after_sentence - 0.06).abs() < f32::EPSILON);
        assert!((event.snapshot.settings.tts_speed - 2.5).abs() < f32::EPSILON);
        assert!((event.snapshot.settings.tts_volume - 1.3).abs() < f32::EPSILON);
    }
}
