use super::super::messages::{Component, NumericSetting};
use super::super::state::{
    App, MAX_HORIZONTAL_MARGIN, MAX_LETTER_SPACING, MAX_VERTICAL_MARGIN, MAX_WORD_SPACING,
    apply_component,
};
use super::Effect;
use crate::pagination::{MAX_FONT_SIZE, MAX_LINES_PER_PAGE, MIN_FONT_SIZE, MIN_LINES_PER_PAGE};
use std::time::{Duration, Instant};
use tracing::{debug, info};

impl App {
    pub(super) fn handle_font_size_changed(&mut self, size: u32, effects: &mut Vec<Effect>) {
        let clamped = size.clamp(MIN_FONT_SIZE, MAX_FONT_SIZE);
        if clamped != self.config.font_size {
            let old_page = self.reader.current_page;
            let old_sentence_idx = self.tts.current_sentence_idx.unwrap_or(0);
            let active_sentence = self
                .raw_sentences_for_page(old_page)
                .get(old_sentence_idx)
                .cloned()
                .or_else(|| self.raw_sentences_for_page(old_page).into_iter().next());
            let had_tts = self.tts.playback.is_some() || self.tts.is_preparing();
            let was_playing = self
                .tts
                .playback
                .as_ref()
                .map(|p| !p.is_paused())
                .unwrap_or(self.tts.is_playing());

            debug!(
                old = self.config.font_size,
                new = clamped,
                "Font size changed"
            );
            self.config.font_size = clamped;
            self.repaginate();
            self.remap_current_sentence_after_relayout(
                old_page,
                old_sentence_idx,
                active_sentence.as_deref(),
            );
            if had_tts {
                if let Some(sentence_idx) = self.tts.current_sentence_idx {
                    // Invalidate any in-flight work from the old pagination before restart.
                    self.tts.request_id = self.tts.request_id.wrapping_add(1);
                    self.tts.lifecycle = super::super::state::TtsLifecycle::Idle;
                    self.tts.pending_append = false;
                    self.tts.pending_append_batch = None;
                    self.tts.resume_after_prepare = was_playing;
                    effects.push(Effect::StartTts {
                        page: self.reader.current_page,
                        sentence_idx,
                    });
                }
            }
            self.schedule_highlight_snap_after_layout_change(effects);
            effects.push(Effect::SaveConfig);
        }
    }

    pub(super) fn handle_toggle_theme(&mut self, effects: &mut Vec<Effect>) {
        let next = match self.config.theme {
            crate::config::ThemeMode::Night => crate::config::ThemeMode::Day,
            crate::config::ThemeMode::Day => crate::config::ThemeMode::Night,
        };
        info!(
            night_mode = matches!(next, crate::config::ThemeMode::Night),
            "Toggled theme"
        );
        self.config.theme = next;
        effects.push(Effect::SaveConfig);
    }

    pub(super) fn handle_toggle_settings(&mut self, effects: &mut Vec<Effect>) {
        debug!("Toggled settings panel");
        let next = !self.config.show_settings;
        self.config.show_settings = next;
        if next {
            self.show_stats = false;
        } else {
            self.active_numeric_setting = None;
            self.numeric_setting_input.clear();
        }
        self.schedule_highlight_snap_after_layout_change(effects);
        effects.push(Effect::SaveConfig);
    }

    pub(super) fn handle_toggle_stats(&mut self, effects: &mut Vec<Effect>) {
        self.show_stats = !self.show_stats;
        let mut changed_settings_visibility = false;
        if self.show_stats {
            if self.config.show_settings {
                changed_settings_visibility = true;
            }
            self.config.show_settings = false;
            self.active_numeric_setting = None;
            self.numeric_setting_input.clear();
        }
        self.schedule_highlight_snap_after_layout_change(effects);
        if changed_settings_visibility {
            effects.push(Effect::SaveConfig);
        }
    }

    pub(super) fn handle_toggle_text_only(&mut self, effects: &mut Vec<Effect>) {
        self.text_only_mode = !self.text_only_mode;
        debug!(
            enabled = self.text_only_mode,
            "Toggled text-only preview mode"
        );
        self.schedule_highlight_snap_after_layout_change(effects);
    }

    pub(super) fn handle_font_family_changed(
        &mut self,
        family: crate::config::FontFamily,
        effects: &mut Vec<Effect>,
    ) {
        debug!(?family, "Font family changed");
        self.config.font_family = family;
        self.schedule_highlight_snap_after_layout_change(effects);
        effects.push(Effect::SaveConfig);
    }

    pub(super) fn handle_font_weight_changed(
        &mut self,
        weight: crate::config::FontWeight,
        effects: &mut Vec<Effect>,
    ) {
        debug!(?weight, "Font weight changed");
        self.config.font_weight = weight;
        self.schedule_highlight_snap_after_layout_change(effects);
        effects.push(Effect::SaveConfig);
    }

    pub(super) fn handle_line_spacing_changed(&mut self, spacing: f32, effects: &mut Vec<Effect>) {
        self.config.line_spacing = spacing.clamp(0.8, 2.5);
        debug!(
            line_spacing = self.config.line_spacing,
            "Line spacing changed"
        );
        self.schedule_highlight_snap_after_layout_change(effects);
        effects.push(Effect::SaveConfig);
    }

    pub(super) fn handle_margin_horizontal_changed(
        &mut self,
        margin: u16,
        effects: &mut Vec<Effect>,
    ) {
        self.config.margin_horizontal = margin.min(MAX_HORIZONTAL_MARGIN);
        debug!(
            margin_horizontal = self.config.margin_horizontal,
            "Horizontal margin changed"
        );
        self.schedule_highlight_snap_after_layout_change(effects);
        effects.push(Effect::SaveConfig);
    }

    pub(super) fn handle_margin_vertical_changed(
        &mut self,
        margin: u16,
        effects: &mut Vec<Effect>,
    ) {
        self.config.margin_vertical = margin.min(MAX_VERTICAL_MARGIN);
        debug!(
            margin_vertical = self.config.margin_vertical,
            "Vertical margin changed"
        );
        self.schedule_highlight_snap_after_layout_change(effects);
        effects.push(Effect::SaveConfig);
    }

    pub(super) fn handle_word_spacing_changed(&mut self, spacing: u32, effects: &mut Vec<Effect>) {
        self.config.word_spacing = spacing.min(MAX_WORD_SPACING);
        debug!(
            word_spacing = self.config.word_spacing,
            "Word spacing changed"
        );
        self.schedule_highlight_snap_after_layout_change(effects);
        effects.push(Effect::SaveConfig);
    }

    pub(super) fn handle_letter_spacing_changed(
        &mut self,
        spacing: u32,
        effects: &mut Vec<Effect>,
    ) {
        self.config.letter_spacing = spacing.min(MAX_LETTER_SPACING);
        debug!(
            letter_spacing = self.config.letter_spacing,
            "Letter spacing changed"
        );
        self.schedule_highlight_snap_after_layout_change(effects);
        effects.push(Effect::SaveConfig);
    }

    pub(super) fn handle_begin_numeric_setting_edit(&mut self, setting: NumericSetting) {
        self.active_numeric_setting = Some(setting);
        self.numeric_setting_input = self.numeric_setting_value_string(setting);
    }

    pub(super) fn handle_numeric_setting_input_changed(&mut self, value: String) {
        if self.active_numeric_setting.is_some() {
            self.numeric_setting_input = value;
        }
    }

    pub(super) fn handle_commit_numeric_setting_input(&mut self, effects: &mut Vec<Effect>) {
        let Some(setting) = self.active_numeric_setting else {
            return;
        };
        let Some(value) = Self::parse_numeric_setting_value(setting, &self.numeric_setting_input)
        else {
            return;
        };
        self.apply_numeric_setting_value(setting, value, effects);
        self.numeric_setting_input = self.numeric_setting_value_string(setting);
    }

    pub(super) fn handle_cancel_numeric_setting_input(&mut self) {
        self.active_numeric_setting = None;
        self.numeric_setting_input.clear();
    }

    pub(super) fn handle_adjust_numeric_setting_by_wheel(
        &mut self,
        delta: f32,
        effects: &mut Vec<Effect>,
    ) {
        let Some(setting) = self.active_numeric_setting else {
            return;
        };
        if delta.abs() < f32::EPSILON {
            return;
        }

        let steps = if delta.abs() >= 1.0 {
            delta.round() as i32
        } else if delta > 0.0 {
            1
        } else {
            -1
        };
        if steps == 0 {
            return;
        }

        let base = Self::parse_numeric_setting_value(setting, &self.numeric_setting_input)
            .unwrap_or_else(|| self.numeric_setting_value(setting));
        let adjusted = Self::quantize_numeric_setting_value_update(
            setting,
            base + steps as f32 * Self::numeric_setting_step_update(setting),
        );
        self.apply_numeric_setting_value(setting, adjusted, effects);
        self.numeric_setting_input = self.numeric_setting_value_string(setting);
    }

    pub(super) fn handle_day_highlight_changed(
        &mut self,
        component: Component,
        value: f32,
        effects: &mut Vec<Effect>,
    ) {
        self.config.day_highlight = apply_component(self.config.day_highlight, component, value);
        debug!(?component, value, "Day highlight updated");
        effects.push(Effect::SaveConfig);
    }

    pub(super) fn handle_night_highlight_changed(
        &mut self,
        component: Component,
        value: f32,
        effects: &mut Vec<Effect>,
    ) {
        self.config.night_highlight =
            apply_component(self.config.night_highlight, component, value);
        debug!(?component, value, "Night highlight updated");
        effects.push(Effect::SaveConfig);
    }

    pub(super) fn handle_window_resized(
        &mut self,
        width: f32,
        height: f32,
        _effects: &mut Vec<Effect>,
    ) {
        if !width.is_finite() || !height.is_finite() {
            return;
        }
        let width = width.clamp(320.0, 7680.0);
        let height = height.clamp(240.0, 4320.0);

        let changed = (self.config.window_width - width).abs() >= 1.0
            || (self.config.window_height - height).abs() >= 1.0;
        if changed {
            self.config.window_width = width;
            self.config.window_height = height;
            debug!(width, height, "Window size changed");
            self.pending_window_resize = true;
            self.window_geometry_changed_at = Some(Instant::now());
        }
    }

    pub(super) fn handle_window_moved(&mut self, x: f32, y: f32, _effects: &mut Vec<Effect>) {
        if !x.is_finite() || !y.is_finite() {
            return;
        }
        let changed = self
            .config
            .window_pos_x
            .map(|px| (px - x).abs() >= 1.0)
            .unwrap_or(true)
            || self
                .config
                .window_pos_y
                .map(|py| (py - y).abs() >= 1.0)
                .unwrap_or(true);

        if changed {
            self.config.window_pos_x = Some(x);
            self.config.window_pos_y = Some(y);
            debug!(x, y, "Window position changed");
            self.pending_window_move = true;
            self.window_geometry_changed_at = Some(Instant::now());
        }
    }

    pub(super) fn maybe_flush_window_geometry_updates(&mut self, effects: &mut Vec<Effect>) {
        const WINDOW_GEOMETRY_SAVE_DEBOUNCE: Duration = Duration::from_millis(220);
        if !(self.pending_window_resize || self.pending_window_move) {
            return;
        }
        let Some(changed_at) = self.window_geometry_changed_at else {
            return;
        };
        if Instant::now().saturating_duration_since(changed_at) < WINDOW_GEOMETRY_SAVE_DEBOUNCE {
            return;
        }

        if self.pending_window_resize {
            self.schedule_highlight_snap_after_layout_change(effects);
        }
        effects.push(Effect::SaveConfig);
        self.pending_window_resize = false;
        self.pending_window_move = false;
        self.window_geometry_changed_at = None;
    }

    fn schedule_highlight_snap_after_layout_change(&mut self, effects: &mut Vec<Effect>) {
        if !self.config.auto_scroll_tts {
            return;
        }
        let Some(idx) = self.tts.current_sentence_idx else {
            return;
        };
        let sentence_count = self.sentence_count_for_page(self.reader.current_page);
        if sentence_count == 0 {
            return;
        }
        let clamped = idx.min(sentence_count.saturating_sub(1));
        self.tts.current_sentence_idx = Some(clamped);
        self.bookmark.pending_sentence_snap = Some(clamped);
        effects.push(Effect::AutoScrollToCurrent);
        effects.push(Effect::SaveBookmark);
    }

    fn remap_current_sentence_after_relayout(
        &mut self,
        old_page: usize,
        old_sentence_idx: usize,
        active_sentence: Option<&str>,
    ) {
        let Some(target) = active_sentence else {
            return;
        };
        let mut best: Option<(usize, usize, usize)> = None;
        for (page_idx, page_sentences) in self.reader.page_sentences.iter().enumerate() {
            for (sentence_idx, candidate) in page_sentences.iter().enumerate() {
                if candidate == target {
                    let distance = page_idx.abs_diff(old_page) * 10_000
                        + sentence_idx.abs_diff(old_sentence_idx);
                    match best {
                        Some((best_distance, _, _)) if best_distance <= distance => {}
                        _ => best = Some((distance, page_idx, sentence_idx)),
                    }
                }
            }
        }
        if let Some((_, page_idx, sentence_idx)) = best {
            self.reader.current_page = page_idx;
            self.tts.current_sentence_idx = Some(sentence_idx);
            self.tts.last_sentences = self.raw_sentences_for_page(page_idx);
            self.bookmark.pending_sentence_snap = Some(sentence_idx);
        }
    }

    fn apply_numeric_setting_value(
        &mut self,
        setting: NumericSetting,
        value: f32,
        effects: &mut Vec<Effect>,
    ) {
        match setting {
            NumericSetting::LineSpacing => self.handle_line_spacing_changed(value, effects),
            NumericSetting::PauseAfterSentence => {
                self.handle_pause_after_sentence_changed(value, effects);
            }
            NumericSetting::LinesPerPage => {
                self.handle_lines_per_page_changed(value.round() as u32, effects);
            }
            NumericSetting::MarginHorizontal => {
                self.handle_margin_horizontal_changed(value.round() as u16, effects);
            }
            NumericSetting::MarginVertical => {
                self.handle_margin_vertical_changed(value.round() as u16, effects);
            }
            NumericSetting::WordSpacing => {
                self.handle_word_spacing_changed(value.round() as u32, effects);
            }
            NumericSetting::LetterSpacing => {
                self.handle_letter_spacing_changed(value.round() as u32, effects);
            }
        }
    }

    fn numeric_setting_value(&self, setting: NumericSetting) -> f32 {
        match setting {
            NumericSetting::LineSpacing => self.config.line_spacing,
            NumericSetting::PauseAfterSentence => self.config.pause_after_sentence,
            NumericSetting::LinesPerPage => self.config.lines_per_page as f32,
            NumericSetting::MarginHorizontal => self.config.margin_horizontal as f32,
            NumericSetting::MarginVertical => self.config.margin_vertical as f32,
            NumericSetting::WordSpacing => self.config.word_spacing as f32,
            NumericSetting::LetterSpacing => self.config.letter_spacing as f32,
        }
    }

    fn numeric_setting_value_string(&self, setting: NumericSetting) -> String {
        let value = self.numeric_setting_value(setting);
        let decimals = Self::numeric_setting_decimals_update(setting);
        if decimals == 0 {
            format!("{}", value.round() as i32)
        } else {
            format!("{value:.*}", decimals as usize)
        }
    }

    fn parse_numeric_setting_value(setting: NumericSetting, raw: &str) -> Option<f32> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return None;
        }
        let parsed = trimmed.parse::<f32>().ok()?;
        if !parsed.is_finite() {
            return None;
        }
        if Self::numeric_setting_requires_integer_update(setting)
            && (parsed - parsed.round()).abs() > f32::EPSILON
        {
            return None;
        }
        let (min, max) = Self::numeric_setting_bounds_update(setting);
        if parsed < min || parsed > max {
            return None;
        }
        Some(Self::quantize_numeric_setting_value_update(setting, parsed))
    }

    fn numeric_setting_requires_integer_update(setting: NumericSetting) -> bool {
        matches!(
            setting,
            NumericSetting::LinesPerPage
                | NumericSetting::MarginHorizontal
                | NumericSetting::MarginVertical
                | NumericSetting::WordSpacing
                | NumericSetting::LetterSpacing
        )
    }

    fn quantize_numeric_setting_value_update(setting: NumericSetting, value: f32) -> f32 {
        let (min, max) = Self::numeric_setting_bounds_update(setting);
        let step = Self::numeric_setting_step_update(setting);
        let clamped = value.clamp(min, max);
        if step <= f32::EPSILON {
            return clamped;
        }
        let steps = ((clamped - min) / step).round();
        let quantized = min + steps * step;
        let decimals = Self::numeric_setting_decimals_update(setting);
        let factor = 10f32.powi(decimals as i32);
        ((quantized * factor).round() / factor).clamp(min, max)
    }

    fn numeric_setting_bounds_update(setting: NumericSetting) -> (f32, f32) {
        match setting {
            NumericSetting::LineSpacing => (0.8, 2.5),
            NumericSetting::PauseAfterSentence => (0.0, 2.0),
            NumericSetting::LinesPerPage => (MIN_LINES_PER_PAGE as f32, MAX_LINES_PER_PAGE as f32),
            NumericSetting::MarginHorizontal => (0.0, MAX_HORIZONTAL_MARGIN as f32),
            NumericSetting::MarginVertical => (0.0, MAX_VERTICAL_MARGIN as f32),
            NumericSetting::WordSpacing => (0.0, MAX_WORD_SPACING as f32),
            NumericSetting::LetterSpacing => (0.0, MAX_LETTER_SPACING as f32),
        }
    }

    fn numeric_setting_step_update(setting: NumericSetting) -> f32 {
        match setting {
            NumericSetting::LineSpacing => 0.05,
            NumericSetting::PauseAfterSentence => 0.01,
            NumericSetting::LinesPerPage => 1.0,
            NumericSetting::MarginHorizontal => 1.0,
            NumericSetting::MarginVertical => 1.0,
            NumericSetting::WordSpacing => 1.0,
            NumericSetting::LetterSpacing => 1.0,
        }
    }

    fn numeric_setting_decimals_update(setting: NumericSetting) -> u8 {
        match setting {
            NumericSetting::LineSpacing => 2,
            NumericSetting::PauseAfterSentence => 2,
            NumericSetting::LinesPerPage
            | NumericSetting::MarginHorizontal
            | NumericSetting::MarginVertical
            | NumericSetting::WordSpacing
            | NumericSetting::LetterSpacing => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use crate::epub_loader::LoadedBook;
    use std::path::PathBuf;

    fn sample_text(sentence_count: usize) -> String {
        (0..sentence_count)
            .map(|i| {
                format!(
                    "Unique sentence number {i} has enough words to avoid accidental matching collisions."
                )
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn build_test_app(sentence_count: usize) -> App {
        let book = LoadedBook {
            text: sample_text(sentence_count),
            images: Vec::new(),
        };

        let mut config = AppConfig::default();
        config.show_settings = false;
        config.auto_scroll_tts = true;
        config.font_size = 16;
        config.lines_per_page = 16;
        let epub_path = PathBuf::from(format!(
            "/tmp/ebup-appearance-test-{}-{}.epub",
            std::process::id(),
            sentence_count
        ));
        let (mut app, _task) = App::bootstrap(book, config, epub_path, None);
        app.reader.current_page = 0;
        app
    }

    #[test]
    fn font_size_change_preserves_anchor_and_requests_snap() {
        let mut app = build_test_app(180);
        app.tts.current_sentence_idx = Some(4);
        let anchor = app
            .raw_sentences_for_page(app.reader.current_page)
            .get(4)
            .cloned()
            .expect("anchor sentence");

        let mut effects = Vec::new();
        app.handle_font_size_changed(22, &mut effects);

        let mapped = app
            .tts
            .current_sentence_idx
            .and_then(|idx| {
                app.raw_sentences_for_page(app.reader.current_page)
                    .get(idx)
                    .cloned()
            })
            .expect("mapped sentence");
        assert_eq!(anchor, mapped);
        assert_eq!(
            app.bookmark.pending_sentence_snap,
            app.tts.current_sentence_idx
        );
        assert!(
            effects
                .iter()
                .any(|effect| matches!(effect, Effect::AutoScrollToCurrent))
        );
        assert!(
            effects
                .iter()
                .any(|effect| matches!(effect, Effect::SaveBookmark))
        );
        assert!(
            effects
                .iter()
                .any(|effect| matches!(effect, Effect::SaveConfig))
        );
    }

    #[test]
    fn margin_change_requests_snap_when_auto_scroll_enabled() {
        let mut app = build_test_app(120);
        app.tts.current_sentence_idx = Some(6);
        let mut effects = Vec::new();

        app.handle_margin_horizontal_changed(80, &mut effects);

        assert_eq!(app.bookmark.pending_sentence_snap, Some(6));
        assert!(
            effects
                .iter()
                .any(|effect| matches!(effect, Effect::AutoScrollToCurrent))
        );
        assert!(
            effects
                .iter()
                .any(|effect| matches!(effect, Effect::SaveBookmark))
        );
        assert!(
            effects
                .iter()
                .any(|effect| matches!(effect, Effect::SaveConfig))
        );
    }
}
