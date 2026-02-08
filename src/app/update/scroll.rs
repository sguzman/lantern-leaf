use super::Effect;
use super::super::state::App;
use crate::cache::{Bookmark, save_bookmark};
use crate::text_utils::split_sentences;
use iced::widget::scrollable::RelativeOffset;
use tracing::info;

impl App {
    pub(super) fn handle_scrolled(
        &mut self,
        offset: RelativeOffset,
        viewport_height: f32,
        content_height: f32,
        effects: &mut Vec<Effect>,
    ) {
        let sanitized = Self::sanitize_offset(offset);
        self.bookmark.viewport_fraction = if viewport_height.is_finite()
            && content_height.is_finite()
            && content_height > 0.0
        {
            (viewport_height / content_height).clamp(0.05, 1.0)
        } else {
            0.25
        };
        if sanitized != self.bookmark.last_scroll_offset {
            self.bookmark.last_scroll_offset = sanitized;
            effects.push(Effect::SaveBookmark);
        }
    }

    pub(super) fn handle_jump_to_current_audio(&mut self, effects: &mut Vec<Effect>) {
        if let Some(idx) = self.tts.current_sentence_idx {
            let total = self.tts.last_sentences.len();
            if let Some(offset) = self.scroll_offset_for_sentence(idx, total) {
                info!(
                    idx,
                    fraction = offset.y,
                    "Jumping to current audio sentence (scroll only)"
                );
                effects.push(Effect::ScrollTo(offset));
                effects.push(Effect::SaveBookmark);
            }
        }
    }

    pub(super) fn persist_bookmark(&self) {
        let sentences = self.current_sentences();

        let sentence_idx = self
            .tts
            .current_sentence_idx
            .filter(|idx| *idx < sentences.len())
            .or_else(|| {
                if sentences.is_empty() {
                    None
                } else {
                    let frac = Self::sanitize_offset(self.bookmark.last_scroll_offset).y;
                    let idx = (frac * (sentences.len().saturating_sub(1) as f32)).round() as usize;
                    Some(idx.min(sentences.len().saturating_sub(1)))
                }
            });
        let sentence_text = sentence_idx.and_then(|idx| sentences.get(idx).cloned());
        let scroll_y = Self::sanitize_offset(self.bookmark.last_scroll_offset).y;

        let bookmark = Bookmark {
            page: self.reader.current_page,
            sentence_idx,
            sentence_text,
            scroll_y,
        };

        save_bookmark(&self.epub_path, &bookmark);
    }

    pub(super) fn sanitize_offset(offset: RelativeOffset) -> RelativeOffset {
        let clamp = |v: f32| {
            if v.is_finite() {
                v.clamp(0.0, 1.0)
            } else {
                0.0
            }
        };
        RelativeOffset {
            x: clamp(offset.x),
            y: clamp(offset.y),
        }
    }

    fn current_sentences(&self) -> Vec<String> {
        split_sentences(
            self.reader
                .pages
                .get(self.reader.current_page)
                .map(String::as_str)
                .unwrap_or("")
                .to_string(),
        )
    }

    pub(crate) fn scroll_offset_for_sentence(
        &self,
        sentence_idx: usize,
        total_sentences: usize,
    ) -> Option<RelativeOffset> {
        if total_sentences == 0 {
            return None;
        }

        let base = self
            .sentence_progress_for_page(sentence_idx, total_sentences)
            .unwrap_or_else(|| {
                let clamped_idx = sentence_idx.min(total_sentences.saturating_sub(1)) as f32;
                let denom = total_sentences.saturating_sub(1).max(1) as f32;
                (clamped_idx / denom).clamp(0.0, 1.0)
            });

        let viewport_fraction = self.estimated_viewport_fraction();
        let y = if self.config.center_spoken_sentence {
            // Center mode keeps the active sentence near middle of the viewport.
            (base - 0.5 * viewport_fraction).clamp(0.0, 1.0)
        } else {
            // Track mode follows the sentence without trying to center it.
            // Keep a small lead so the highlight does not sit on the very top edge.
            (base - 0.15 * viewport_fraction).clamp(0.0, 1.0)
        };

        Some(RelativeOffset { x: 0.0, y })
    }

    fn sentence_progress_for_page(
        &self,
        sentence_idx: usize,
        total_sentences: usize,
    ) -> Option<f32> {
        let page = self
            .reader
            .pages
            .get(self.reader.current_page)
            .map(String::as_str)?;
        let sentences = if self.tts.last_sentences.len() == total_sentences
            && !self.tts.last_sentences.is_empty()
        {
            self.tts.last_sentences.clone()
        } else {
            split_sentences(page.to_string())
        };
        if sentences.is_empty() {
            return None;
        }

        let idx = sentence_idx.min(sentences.len().saturating_sub(1));
        let sentence_lengths: Vec<usize> = sentences
            .iter()
            .map(|s| s.chars().count().max(1))
            .collect();
        let total_weight: usize = sentence_lengths.iter().sum();
        if total_weight == 0 {
            return None;
        }

        let before_weight: usize = sentence_lengths.iter().take(idx).sum();
        let anchor_weight = before_weight + sentence_lengths[idx] / 2;
        Some((anchor_weight as f32 / total_weight as f32).clamp(0.0, 1.0))
    }

    fn estimated_viewport_fraction(&self) -> f32 {
        if self.bookmark.viewport_fraction.is_finite() && self.bookmark.viewport_fraction > 0.0 {
            return self.bookmark.viewport_fraction.clamp(0.08, 0.45);
        }
        0.25
    }
}
