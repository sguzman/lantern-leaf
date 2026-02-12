use crate::tts::{TtsEngine, TtsPlayback};
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Runtime TTS model (configuration lives in `AppConfig`).
pub struct PendingAppendBatch {
    pub(in crate::app) page: usize,
    pub(in crate::app) request_id: u64,
    pub(in crate::app) start_idx: usize,
    pub(in crate::app) audio_sentences: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TtsLifecycle {
    Idle,
    Preparing {
        page: usize,
        sentence_idx: usize,
        request_id: u64,
    },
    Playing,
    Paused,
}

pub struct TtsState {
    pub(in crate::app) engine: Option<TtsEngine>,
    pub(in crate::app) playback: Option<TtsPlayback>,
    pub(in crate::app) lifecycle: TtsLifecycle,
    pub(in crate::app) pending_append: bool,
    pub(in crate::app) pending_append_batch: Option<PendingAppendBatch>,
    pub(in crate::app) resume_after_prepare: bool,
    pub(in crate::app) last_sentences: Vec<String>,
    pub(in crate::app) current_sentence_idx: Option<usize>,
    pub(in crate::app) sentence_offset: usize,
    pub(in crate::app) track: Vec<(PathBuf, Duration)>,
    pub(in crate::app) started_at: Option<Instant>,
    pub(in crate::app) elapsed: Duration,
    pub(in crate::app) request_id: u64,
    pub(in crate::app) sources_per_sentence: usize,
    pub(in crate::app) total_sources: usize,
    pub(in crate::app) display_to_audio: Vec<Option<usize>>,
    pub(in crate::app) audio_to_display: Vec<usize>,
}

impl TtsState {
    pub(in crate::app) fn new(engine: Option<TtsEngine>) -> Self {
        Self {
            engine,
            playback: None,
            lifecycle: TtsLifecycle::Idle,
            pending_append: false,
            pending_append_batch: None,
            resume_after_prepare: true,
            last_sentences: Vec::new(),
            current_sentence_idx: None,
            sentence_offset: 0,
            track: Vec::new(),
            started_at: None,
            elapsed: Duration::ZERO,
            request_id: 0,
            sources_per_sentence: 1,
            total_sources: 0,
            display_to_audio: Vec::new(),
            audio_to_display: Vec::new(),
        }
    }

    pub(in crate::app) fn is_preparing(&self) -> bool {
        matches!(self.lifecycle, TtsLifecycle::Preparing { .. })
    }

    pub(in crate::app) fn is_playing(&self) -> bool {
        matches!(self.lifecycle, TtsLifecycle::Playing)
    }

    pub(in crate::app) fn preparing_context(&self) -> Option<(usize, usize, u64)> {
        match self.lifecycle {
            TtsLifecycle::Preparing {
                page,
                sentence_idx,
                request_id,
            } => Some((page, sentence_idx, request_id)),
            _ => None,
        }
    }

    pub(in crate::app) fn clear_transient_playback_state(&mut self) {
        self.playback = None;
        self.track.clear();
        self.started_at = None;
        self.elapsed = Duration::ZERO;
        self.sources_per_sentence = 1;
        self.total_sources = 0;
        self.pending_append = false;
        self.pending_append_batch = None;
    }

    pub(in crate::app) fn set_current_sentence_clamped(
        &mut self,
        sentence_idx: usize,
        sentence_count: usize,
    ) {
        if sentence_count == 0 {
            self.current_sentence_idx = None;
        } else {
            self.current_sentence_idx = Some(sentence_idx.min(sentence_count.saturating_sub(1)));
        }
    }

    pub(in crate::app) fn set_mappings_checked(
        &mut self,
        display_to_audio: Vec<Option<usize>>,
        audio_to_display: Vec<usize>,
        audio_sentence_count: usize,
    ) {
        self.display_to_audio = display_to_audio
            .into_iter()
            .map(|mapped| mapped.filter(|idx| *idx < audio_sentence_count))
            .collect();
        self.audio_to_display = audio_to_display
            .into_iter()
            .take(audio_sentence_count)
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::TtsState;

    #[test]
    fn clamps_current_sentence() {
        let mut tts = TtsState::new(None);
        tts.set_current_sentence_clamped(8, 3);
        assert_eq!(tts.current_sentence_idx, Some(2));
    }

    #[test]
    fn checked_mappings_filter_out_of_range_audio() {
        let mut tts = TtsState::new(None);
        tts.set_mappings_checked(vec![Some(0), Some(9), None], vec![2, 1, 0], 2);
        assert_eq!(tts.display_to_audio, vec![Some(0), None, None]);
        assert_eq!(tts.audio_to_display, vec![2, 1]);
    }
}
