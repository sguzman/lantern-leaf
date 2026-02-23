use crate::{config, epub_loader, normalizer, pagination, text_utils};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const BASE_WPM: f64 = 170.0;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct PanelState {
    pub show_settings: bool,
    pub show_stats: bool,
    pub show_tts: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReaderSettingsView {
    pub theme: config::ThemeMode,
    pub day_highlight: config::HighlightColor,
    pub night_highlight: config::HighlightColor,
    pub font_size: u32,
    pub line_spacing: f32,
    pub margin_horizontal: u16,
    pub margin_vertical: u16,
    pub lines_per_page: usize,
    pub pause_after_sentence: f32,
    pub auto_scroll_tts: bool,
    pub center_spoken_sentence: bool,
    pub tts_speed: f32,
    pub tts_volume: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReaderSettingsPatch {
    pub font_size: Option<u32>,
    pub line_spacing: Option<f32>,
    pub margin_horizontal: Option<u16>,
    pub margin_vertical: Option<u16>,
    pub lines_per_page: Option<usize>,
    pub pause_after_sentence: Option<f32>,
    pub auto_scroll_tts: Option<bool>,
    pub center_spoken_sentence: Option<bool>,
    pub tts_speed: Option<f32>,
    pub tts_volume: Option<f32>,
}

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Clone, Serialize)]
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
    pub stats: ReaderStats,
    pub panels: PanelState,
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
    current_plan_page: Option<usize>,
    current_plan: Option<normalizer::PageNormalization>,
}

impl ReaderSession {
    pub fn load(
        source_path: PathBuf,
        mut config: config::AppConfig,
        normalizer: &normalizer::TextNormalizer,
        bookmark: Option<crate::cache::Bookmark>,
    ) -> Result<Self, String> {
        let loaded = epub_loader::load_book_content(&source_path).map_err(|err| err.to_string())?;
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
        config.lines_per_page = config
            .lines_per_page
            .clamp(pagination::MIN_LINES_PER_PAGE, pagination::MAX_LINES_PER_PAGE);

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
            day_highlight: self.config.day_highlight,
            night_highlight: self.config.night_highlight,
            font_size: self.config.font_size,
            line_spacing: self.config.line_spacing,
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
            stats,
            panels,
        }
    }

    pub fn apply_settings_patch(
        &mut self,
        patch: ReaderSettingsPatch,
        normalizer: &normalizer::TextNormalizer,
    ) {
        let preserve = self.global_display_idx();
        let mut repaginate = false;

        if let Some(font_size) = patch.font_size {
            let clamped = font_size.clamp(pagination::MIN_FONT_SIZE, pagination::MAX_FONT_SIZE);
            if clamped != self.config.font_size {
                self.config.font_size = clamped;
                repaginate = true;
            }
        }
        if let Some(lines) = patch.lines_per_page {
            let clamped = lines.clamp(pagination::MIN_LINES_PER_PAGE, pagination::MAX_LINES_PER_PAGE);
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
        } else {
            self.update_search_matches(normalizer);
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
        let current = self.current_highlight_idx().unwrap_or(0).min(count.saturating_sub(1));
        let next = (current + 1).min(count.saturating_sub(1));
        self.sentence_click(next, normalizer);
    }

    pub fn select_prev_sentence(&mut self, normalizer: &normalizer::TextNormalizer) {
        let count = self.current_sentences(normalizer).len();
        if count == 0 {
            return;
        }
        let current = self.current_highlight_idx().unwrap_or(0).min(count.saturating_sub(1));
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
        let page = bookmark.page.min(self.page_sentence_counts.len().saturating_sub(1));
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

        self.current_plan.clone().unwrap_or(normalizer::PageNormalization {
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
        let page_base: usize = self.page_sentence_counts.iter().take(self.current_page).sum();
        self.highlighted_display_idx.map(|idx| page_base + idx)
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
            + (((page_sentence_count as f64) * progress_fraction).round() as usize)
                .min(page_sentence_count.max(sentence_progress_count).min(sentence_progress_total.max(1)));

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
    ReaderSession::load(source_path, effective_config, normalizer, bookmark)
}

pub fn persist_session_housekeeping(session: &ReaderSession) {
    let bookmark = session.to_bookmark();
    crate::cache::save_bookmark(Path::new(&session.source_path), &bookmark);
    crate::cache::save_epub_config(Path::new(&session.source_path), &session.config);
}
