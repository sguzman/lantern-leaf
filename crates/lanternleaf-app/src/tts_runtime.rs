use std::{
    collections::VecDeque,
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
        mpsc,
    },
    thread,
    time::{Duration, Instant},
};

use lanternleaf_core::{cache, cancellation, config, normalizer, session, tts};
use tracing::{info, trace, warn};

const TTS_PROGRESS_POLL_INTERVAL: Duration = Duration::from_millis(8);
const TTS_PREPARE_SENTENCE_WINDOW: usize = 8;
const TTS_TARGET_PLAY_FROM_PAGE_MS: u64 = 1500;
const TTS_TARGET_PLAY_FROM_HIGHLIGHT_MS: u64 = 600;
const TTS_TARGET_NEXT_SENTENCE_MS: u64 = 180;
const TTS_TARGET_WARM_CACHE_MS: u64 = 250;
const TTS_TARGET_COLD_CACHE_MS: u64 = 2000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TtsRuntimeMode {
    Real,
    Simulated,
}

/// Deterministic boundary source used by simulated runtime tests. It models
/// actual first-sample delivery without making wall-clock duration the driver.
#[derive(Debug, Default)]
pub struct SimulatedBoundaryDriver {
    pending: Mutex<VecDeque<tts::TtsSentenceBoundary>>,
    output: Mutex<Option<mpsc::Sender<tts::TtsSentenceBoundary>>>,
    paused: AtomicBool,
}

impl SimulatedBoundaryDriver {
    pub fn pause(&self) {
        self.paused.store(true, Ordering::SeqCst);
    }

    pub fn resume(&self) {
        self.paused.store(false, Ordering::SeqCst);
    }

    pub fn emit_next(&self) -> bool {
        if self.paused.load(Ordering::SeqCst) {
            return false;
        }
        let boundary = self.pending.lock().ok().and_then(|mut queue| queue.pop_front());
        let Some(boundary) = boundary else { return false };
        self.output
            .lock()
            .ok()
            .and_then(|guard| guard.as_ref().cloned())
            .is_some_and(|sender| sender.send(boundary).is_ok())
    }

    fn install(
        &self,
        output: mpsc::Sender<tts::TtsSentenceBoundary>,
        boundaries: Vec<tts::TtsSentenceBoundary>,
    ) {
        if let Ok(mut guard) = self.output.lock() {
            *guard = Some(output);
        }
        if let Ok(mut queue) = self.pending.lock() {
            *queue = boundaries.into_iter().collect();
        }
    }
}

#[derive(Debug, Clone)]
pub enum TtsCommand {
    Play,
    Pause,
    TogglePlayPause,
    PlayFromPageStart,
    PlayFromHighlight,
    SeekNext,
    SeekPrev,
    RepeatSentence,
    Stop,
    ApplySettings { patch: session::ReaderSettingsPatch },
}

impl TtsCommand {
    pub fn from_session_command(command: &session::SessionCommand) -> Option<Self> {
        match command {
            session::SessionCommand::TtsPlay => Some(Self::Play),
            session::SessionCommand::TtsPause => Some(Self::Pause),
            session::SessionCommand::TtsTogglePlayPause => Some(Self::TogglePlayPause),
            session::SessionCommand::TtsPlayFromPageStart => Some(Self::PlayFromPageStart),
            session::SessionCommand::TtsPlayFromHighlight => Some(Self::PlayFromHighlight),
            session::SessionCommand::TtsSeekNext => Some(Self::SeekNext),
            session::SessionCommand::TtsSeekPrev => Some(Self::SeekPrev),
            session::SessionCommand::TtsRepeatSentence => Some(Self::RepeatSentence),
            session::SessionCommand::TtsStop => Some(Self::Stop),
            session::SessionCommand::ApplySettings { patch } => {
                if patch_has_tts_fields(patch) {
                    Some(Self::ApplySettings {
                        patch: patch.clone(),
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn to_session_command(&self) -> session::SessionCommand {
        match self {
            Self::Play => session::SessionCommand::TtsPlay,
            Self::Pause => session::SessionCommand::TtsPause,
            Self::TogglePlayPause => session::SessionCommand::TtsTogglePlayPause,
            Self::PlayFromPageStart => session::SessionCommand::TtsPlayFromPageStart,
            Self::PlayFromHighlight => session::SessionCommand::TtsPlayFromHighlight,
            Self::SeekNext => session::SessionCommand::TtsSeekNext,
            Self::SeekPrev => session::SessionCommand::TtsSeekPrev,
            Self::RepeatSentence => session::SessionCommand::TtsRepeatSentence,
            Self::Stop => session::SessionCommand::TtsStop,
            Self::ApplySettings { patch } => session::SessionCommand::ApplySettings {
                patch: patch.clone(),
            },
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Play => "tts.play",
            Self::Pause => "tts.pause",
            Self::TogglePlayPause => "tts.toggle_play_pause",
            Self::PlayFromPageStart => "tts.play_page_start",
            Self::PlayFromHighlight => "tts.play_from_highlight",
            Self::SeekNext => "tts.seek_next",
            Self::SeekPrev => "tts.seek_prev",
            Self::RepeatSentence => "tts.repeat_sentence",
            Self::Stop => "tts.stop",
            Self::ApplySettings { .. } => "tts.apply_settings",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TtsRuntimeEventKind {
    StateChanged,
    Progress,
    SentenceStarted,
    Queued,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct TtsRuntimeEvent {
    pub request_id: u64,
    pub action: String,
    pub kind: TtsRuntimeEventKind,
    /// TTS events carry playback deltas only. The immutable reader document remains owned by the
    /// app session and is never cloned into the high-frequency event stream.
    pub snapshot: Option<session::ReaderSnapshot>,
    pub playback: Option<crate::contracts::ReaderPlaybackState>,
    pub tts: Option<session::ReaderTtsView>,
    pub message: Option<String>,
    pub cursor: Option<TtsCursor>,
}

#[derive(Debug, Clone)]
pub struct TtsPlaybackSnapshot {
    pub state: session::TtsPlaybackState,
    pub current_sentence_idx: Option<usize>,
    pub total_sentences: usize,
    pub progress_pct: f64,
    pub speed: f32,
    pub volume: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct TtsCursor {
    pub audio_idx: Option<usize>,
    pub display_idx: Option<usize>,
    pub page: usize,
}

#[derive(Debug)]
struct TtsRequestRuntime {
    request_id: u64,
    cancel_token: cancellation::CancellationToken,
    pause_requested: Arc<AtomicBool>,
}

impl TtsRequestRuntime {
    fn set_paused(&self, paused: bool) {
        self.pause_requested.store(paused, Ordering::SeqCst);
    }
}

#[derive(Debug, Clone)]
struct TtsPlaybackPlan {
    source_path: PathBuf,
    page: usize,
    sentences: Vec<String>,
    display_ids: Vec<usize>,
    start_idx: usize,
    pause_after: Duration,
    speed: f32,
    volume: f32,
    threads: usize,
    progress_log_interval: Duration,
    model_path: PathBuf,
    espeak_path: PathBuf,
    backend: config::TtsBackend,
    windows_voice_id: Option<String>,
}

#[derive(Default)]
struct TtsEventBatcher {
    bucket: Vec<TtsRuntimeEvent>,
}

impl TtsEventBatcher {
    fn collect(&mut self, rx: &mpsc::Receiver<TtsRuntimeEvent>) {
        while let Ok(event) = rx.try_recv() {
            if matches!(
                event.kind,
                TtsRuntimeEventKind::Progress | TtsRuntimeEventKind::StateChanged
            ) {
                if let Some(existing) = self
                    .bucket
                    .iter_mut()
                    .find(|entry| entry.request_id == event.request_id && entry.kind == event.kind)
                {
                    *existing = event;
                    continue;
                }
            }
            self.bucket.push(event);
        }
    }

    fn drain(&mut self) -> Vec<TtsRuntimeEvent> {
        std::mem::take(&mut self.bucket)
    }
}

#[derive(Clone)]
pub struct TtsRuntime {
    mode: TtsRuntimeMode,
    simulated_boundary_driver: Option<Arc<SimulatedBoundaryDriver>>,
    normalizer: normalizer::TextNormalizer,
    panels: Arc<Mutex<session::PanelState>>,
    session: Arc<Mutex<Option<session::ReaderSession>>>,
    request: Arc<Mutex<Option<TtsRequestRuntime>>>,
    next_request_id: Arc<AtomicU64>,
    last_command: Arc<Mutex<Option<String>>>,
    event_tx: mpsc::Sender<TtsRuntimeEvent>,
    event_rx: Arc<Mutex<mpsc::Receiver<TtsRuntimeEvent>>>,
    event_batcher: Arc<Mutex<TtsEventBatcher>>,
    command_tx: mpsc::Sender<TtsCommand>,
    command_in_flight: Arc<AtomicBool>,
}

impl TtsRuntime {
    pub fn new(normalizer: normalizer::TextNormalizer) -> Self {
        Self::new_with_mode(normalizer, TtsRuntimeMode::Real)
    }

    pub fn new_with_mode(normalizer: normalizer::TextNormalizer, mode: TtsRuntimeMode) -> Self {
        Self::new_with_session(normalizer, mode, Arc::new(Mutex::new(None)))
    }

    pub fn new_with_session(
        normalizer: normalizer::TextNormalizer,
        mode: TtsRuntimeMode,
        session: Arc<Mutex<Option<session::ReaderSession>>>,
    ) -> Self {
        let (event_tx, event_rx) = mpsc::channel();
        let (command_tx, command_rx) = mpsc::channel();
        let runtime = Self {
            mode,
            simulated_boundary_driver: (mode == TtsRuntimeMode::Simulated)
                .then(|| Arc::new(SimulatedBoundaryDriver::default())),
            normalizer,
            panels: Arc::new(Mutex::new(session::PanelState::default())),
            session,
            request: Arc::new(Mutex::new(None)),
            next_request_id: Arc::new(AtomicU64::new(1)),
            last_command: Arc::new(Mutex::new(None)),
            event_tx,
            event_rx: Arc::new(Mutex::new(event_rx)),
            event_batcher: Arc::new(Mutex::new(TtsEventBatcher::default())),
            command_tx,
            command_in_flight: Arc::new(AtomicBool::new(false)),
        };
        let worker = runtime.clone();
        thread::Builder::new()
            .name("lanternleaf-tts-control".to_string())
            .spawn(move || {
                while let Ok(command) = command_rx.recv() {
                    let started = Instant::now();
                    trace!(tts_command = command.label(), "TTS control worker started");
                    let _ = worker.apply_command(command);
                    trace!(
                        elapsed_ms = started.elapsed().as_millis(),
                        "TTS control worker finished"
                    );
                    worker.command_in_flight.store(false, Ordering::Release);
                }
            })
            .expect("failed to spawn TTS control worker");
        runtime
    }

    pub fn simulated_boundary_driver(&self) -> Option<Arc<SimulatedBoundaryDriver>> {
        self.simulated_boundary_driver.clone()
    }

    /// Queue a TTS command for the control worker. This is the egui-facing API and performs no
    /// planning, cache IO, snapshot construction, engine creation, or synthesis on the caller.
    pub fn submit_command(&self, command: TtsCommand) -> bool {
        let started = Instant::now();
        let label = command.label().to_string();
        self.command_in_flight.store(true, Ordering::Release);
        let submitted = self.command_tx.send(command).is_ok();
        if !submitted {
            self.command_in_flight.store(false, Ordering::Release);
        }
        trace!(command = %label, elapsed_us = started.elapsed().as_micros(), submitted, "Submitted TTS control command");
        submitted
    }

    pub fn set_session(&self, session: Option<session::ReaderSession>) {
        if session.is_none() {
            self.cancel_request();
        }
        if let Ok(mut guard) = self.session.lock() {
            *guard = session;
        }
    }

    #[cfg(test)]
    fn reset_snapshot_construction_count(&self) {
        if let Ok(guard) = self.session.lock() {
            if let Some(reader) = guard.as_ref() {
                reader.reset_snapshot_construction_count();
            }
        }
    }

    #[cfg(test)]
    fn snapshot_construction_count(&self) -> usize {
        self.session
            .lock()
            .ok()
            .and_then(|guard| {
                guard
                    .as_ref()
                    .map(session::ReaderSession::snapshot_construction_count)
            })
            .unwrap_or(0)
    }

    pub fn set_panels(&self, panels: session::PanelState) {
        if let Ok(mut guard) = self.panels.lock() {
            *guard = panels;
        }
    }

    pub fn needs_repaint(&self) -> bool {
        if self
            .request
            .lock()
            .ok()
            .is_some_and(|guard| guard.is_some())
        {
            return true;
        }
        if self.command_in_flight.load(Ordering::Acquire) {
            return true;
        }
        self.session
            .lock()
            .ok()
            .and_then(|guard| guard.as_ref().map(|reader| reader.tts_state()))
            .is_some_and(|state| state != session::TtsPlaybackState::Idle)
    }

    pub fn snapshot(&self) -> Option<session::ReaderSnapshot> {
        let Ok(mut session_guard) = self.session.lock() else {
            return None;
        };
        let reader = session_guard.as_mut()?;
        let panels = panels_snapshot(&self.panels);
        Some(reader.snapshot(panels, &self.normalizer))
    }

    pub fn apply_command(&self, command: TtsCommand) -> Option<session::ReaderPlaybackView> {
        trace!(tts_command = command.label(), "Applying TTS command");
        if let Ok(mut guard) = self.last_command.lock() {
            *guard = Some(command.label().to_string());
        }
        let mut should_sync_tts = true;
        match command {
            TtsCommand::Play
            | TtsCommand::PlayFromPageStart
            | TtsCommand::PlayFromHighlight
            | TtsCommand::RepeatSentence
            | TtsCommand::SeekNext
            | TtsCommand::SeekPrev => {
                if let Some(driver) = self.simulated_boundary_driver.as_ref() {
                    driver.resume();
                }
                if matches!(command, TtsCommand::Play) {
                    should_sync_tts = !self.maybe_resume_playback();
                }
            }
            TtsCommand::Pause => {
                if let Some(driver) = self.simulated_boundary_driver.as_ref() {
                    driver.pause();
                }
                self.pause_playback();
                should_sync_tts = false;
            }
            TtsCommand::TogglePlayPause => {
                should_sync_tts = !self.maybe_toggle_playback();
            }
            _ => {}
        }

        let command_for_session = command.to_session_command();
        let action = command_for_session.action();
        let sync_after_command = should_sync_tts_after_reader_command(&command_for_session);
        let request_id = self.next_request_id.fetch_add(1, Ordering::SeqCst);
        let playback = {
            let mut guard = self.session.lock().ok()?;
            let reader = match guard.as_mut() {
                Some(reader) => reader,
                None => {
                    warn!(
                        request_id,
                        action, "Ignoring TTS command because no reader session is active"
                    );
                    self.emit_event(TtsRuntimeEvent {
                        request_id,
                        action: action.to_string(),
                        kind: TtsRuntimeEventKind::Failed,
                        snapshot: None,
                        playback: None,
                        tts: None,
                        message: Some("no reader session".to_string()),
                        cursor: None,
                    });
                    return None;
                }
            };
            reader
                .apply_command_lightweight(command_for_session, &self.normalizer)
                .playback
        };

        let cursor = cursor_from_playback(&playback);
        self.emit_event(TtsRuntimeEvent {
            request_id,
            action: action.to_string(),
            kind: TtsRuntimeEventKind::StateChanged,
            snapshot: None,
            playback: Some(reader_playback_state_from_view(&playback)),
            tts: Some(playback.tts.clone()),
            message: None,
            cursor,
        });

        if should_sync_tts && sync_after_command {
            self.sync_tts_runtime_after_reader_change();
        }

        Some(playback)
    }

    pub fn collect_events(&self) -> Vec<TtsRuntimeEvent> {
        let rx_guard = match self.event_rx.lock() {
            Ok(guard) => guard,
            Err(_) => return Vec::new(),
        };
        let mut batcher = match self.event_batcher.lock() {
            Ok(guard) => guard,
            Err(_) => return Vec::new(),
        };
        batcher.collect(&rx_guard);
        batcher.drain()
    }

    fn emit_event(&self, event: TtsRuntimeEvent) {
        let _ = self.event_tx.send(event);
    }

    fn cancel_request(&self) {
        if let Ok(mut guard) = self.request.lock() {
            if let Some(runtime) = guard.take() {
                runtime.cancel_token.cancel();
                self.emit_event(TtsRuntimeEvent {
                    request_id: runtime.request_id,
                    action: "reader_tts_runtime_cancelled".to_string(),
                    kind: TtsRuntimeEventKind::Cancelled,
                    snapshot: None,
                    playback: None,
                    tts: None,
                    message: None,
                    cursor: None,
                });
            }
        }
    }

    fn pause_playback(&self) {
        let behavior = self
            .session
            .lock()
            .ok()
            .and_then(|guard| {
                guard
                    .as_ref()
                    .map(|reader| reader.config.tts_pause_resume_behavior)
            })
            .unwrap_or_default();
        match behavior {
            config::TtsPauseResumeBehavior::RestartSentence => self.cancel_request(),
            _ => {
                if let Ok(guard) = self.request.lock() {
                    if let Some(runtime) = guard.as_ref() {
                        runtime.set_paused(true);
                    }
                }
            }
        }
    }

    fn maybe_resume_playback(&self) -> bool {
        let (behavior, paused, can_resume) = {
            let Ok(mut guard) = self.session.lock() else {
                return false;
            };
            let reader = match guard.as_mut() {
                Some(reader) => reader,
                None => return false,
            };
            let state = reader.playback_view(&self.normalizer).tts.state;
            (
                reader.config.tts_pause_resume_behavior,
                state == session::TtsPlaybackState::Paused,
                self.request
                    .lock()
                    .ok()
                    .and_then(|guard| guard.as_ref().map(|_| ()))
                    .is_some(),
            )
        };

        if behavior == config::TtsPauseResumeBehavior::ResumeFromPausePoint && paused && can_resume
        {
            if let Ok(guard) = self.request.lock() {
                if let Some(runtime) = guard.as_ref() {
                    runtime.set_paused(false);
                    return true;
                }
            }
        }
        false
    }

    fn maybe_toggle_playback(&self) -> bool {
        let (behavior, tts_state, can_resume) = {
            let Ok(mut guard) = self.session.lock() else {
                return false;
            };
            let reader = match guard.as_mut() {
                Some(reader) => reader,
                None => return false,
            };
            let state = reader.playback_view(&self.normalizer).tts.state;
            (
                reader.config.tts_pause_resume_behavior,
                state,
                self.request
                    .lock()
                    .ok()
                    .and_then(|guard| guard.as_ref().map(|_| ()))
                    .is_some(),
            )
        };

        match tts_state {
            session::TtsPlaybackState::Playing => {
                if behavior == config::TtsPauseResumeBehavior::RestartSentence {
                    self.cancel_request();
                } else if let Ok(guard) = self.request.lock() {
                    if let Some(runtime) = guard.as_ref() {
                        runtime.set_paused(true);
                    }
                }
                true
            }
            session::TtsPlaybackState::Paused => {
                if behavior == config::TtsPauseResumeBehavior::ResumeFromPausePoint && can_resume {
                    if let Ok(guard) = self.request.lock() {
                        if let Some(runtime) = guard.as_ref() {
                            runtime.set_paused(false);
                            return true;
                        }
                    }
                }
                false
            }
            session::TtsPlaybackState::Idle => false,
        }
    }

    fn sync_tts_runtime_after_reader_change(&self) {
        let plan = self.build_tts_playback_plan();
        if plan.is_none() {
            self.cancel_request();
            return;
        }
        let plan = plan.expect("plan exists");
        self.cancel_request();
        let request_id = self.next_request_id.fetch_add(1, Ordering::SeqCst);
        let cancel_token = cancellation::CancellationToken::new();
        let pause_requested = Arc::new(AtomicBool::new(false));
        {
            let mut guard = match self.request.lock() {
                Ok(guard) => guard,
                Err(_) => return,
            };
            *guard = Some(TtsRequestRuntime {
                request_id,
                cancel_token: cancel_token.clone(),
                pause_requested: pause_requested.clone(),
            });
        }

        info!(
            request_id,
            page = plan.page + 1,
            sentence_idx = plan.start_idx,
            sentence_count = plan.sentences.len(),
            target_play_from_page_ms = TTS_TARGET_PLAY_FROM_PAGE_MS,
            target_play_from_highlight_ms = TTS_TARGET_PLAY_FROM_HIGHLIGHT_MS,
            target_next_sentence_ms = TTS_TARGET_NEXT_SENTENCE_MS,
            target_warm_cache_ms = TTS_TARGET_WARM_CACHE_MS,
            target_cold_cache_ms = TTS_TARGET_COLD_CACHE_MS,
            "Starting TTS runtime playback job"
        );

        let ctx = TtsRuntimeContext {
            mode: self.mode,
            simulated_boundary_driver: self.simulated_boundary_driver.clone(),
            normalizer: self.normalizer.clone(),
            session: self.session.clone(),
            request: self.request.clone(),
            last_command: self.last_command.clone(),
            event_tx: self.event_tx.clone(),
        };

        thread::spawn(move || {
            run_tts_runtime_loop(ctx, request_id, cancel_token, pause_requested);
        });
    }

    fn build_tts_playback_plan(&self) -> Option<TtsPlaybackPlan> {
        let mut guard = self.session.lock().ok()?;
        let reader = guard.as_mut()?;
        let playback = reader.playback_view(&self.normalizer);
        if playback.tts.state != session::TtsPlaybackState::Playing {
            return None;
        }
        let (audio_sentences, start_idx) = reader.current_tts_audio_slice(&self.normalizer);
        let display_ids = reader.current_tts_audio_display_ids(&self.normalizer);
        if audio_sentences.is_empty() {
            return None;
        }
        trace!(
            source = %reader.source_path.display(),
            page = playback.current_page + 1,
            start_idx,
            sentence_count = audio_sentences.len(),
            tts_payload_source = "tts_text",
            "Built TTS playback plan from canonical tts_text payload"
        );
        Some(TtsPlaybackPlan {
            source_path: reader.source_path.clone(),
            page: playback.current_page,
            sentences: audio_sentences,
            display_ids,
            start_idx,
            pause_after: Duration::from_secs_f64(reader.config.pause_after_sentence.max(0.0) as f64),
            speed: reader.config.tts_speed,
            volume: reader.config.tts_volume,
            threads: reader.config.tts_threads.max(1),
            progress_log_interval: Duration::from_secs_f64(
                reader.config.tts_progress_log_interval_secs.max(0.1) as f64,
            ),
            model_path: PathBuf::from(reader.config.tts_model_path.clone()),
            espeak_path: PathBuf::from(reader.config.tts_espeak_path.clone()),
            backend: reader.config.tts_backend,
            windows_voice_id: reader.config.windows_voice_id.clone(),
        })
    }
}

#[derive(Clone)]
struct TtsRuntimeContext {
    mode: TtsRuntimeMode,
    simulated_boundary_driver: Option<Arc<SimulatedBoundaryDriver>>,
    normalizer: normalizer::TextNormalizer,
    session: Arc<Mutex<Option<session::ReaderSession>>>,
    request: Arc<Mutex<Option<TtsRequestRuntime>>>,
    last_command: Arc<Mutex<Option<String>>>,
    event_tx: mpsc::Sender<TtsRuntimeEvent>,
}

fn run_tts_runtime_loop(
    ctx: TtsRuntimeContext,
    runtime_request_id: u64,
    cancel_token: cancellation::CancellationToken,
    pause_requested: Arc<AtomicBool>,
) {
    struct PrefetchedBatch {
        source_path: PathBuf,
        page: usize,
        start_idx: usize,
        prepared: Vec<PreparedSentence>,
    }

    struct PendingPrefetch {
        source_path: PathBuf,
        page: usize,
        start_idx: usize,
        handle: thread::JoinHandle<Result<Vec<PreparedSentence>, String>>,
    }

    let mut engine: Option<tts::TtsEngine> = None;
    let runtime_started_at = Instant::now();
    let mut playback_started_at: Option<Instant> = None;
    let mut ready_prefetch: Option<PrefetchedBatch> = None;

    loop {
        if cancel_token.is_cancelled() {
            break;
        }

        let Some(plan) = collect_tts_playback_plan(&ctx, runtime_request_id) else {
            break;
        };
        if plan.start_idx >= plan.sentences.len() {
            break;
        }

        if ctx.mode == TtsRuntimeMode::Real && engine.is_none() {
            let built_engine = match tts::TtsEngine::new(
                plan.model_path.clone(),
                plan.espeak_path.clone(),
                plan.backend,
                plan.windows_voice_id.clone(),
            ) {
                Ok(engine) => engine,
                Err(err) => {
                    transition_tts_runtime_to_paused(
                        &ctx,
                        runtime_request_id,
                        "reader_tts_runtime_error",
                        &format!("Failed to initialize {:?} TTS backend: {err}", plan.backend),
                    );
                    break;
                }
            };
            engine = Some(built_engine);
        }

        let chunk_end = (plan.start_idx + TTS_PREPARE_SENTENCE_WINDOW).min(plan.sentences.len());
        let prepared = if let Some(prefetched) = ready_prefetch.take() {
            if prefetched.source_path == plan.source_path
                && prefetched.page == plan.page
                && prefetched.start_idx == plan.start_idx
            {
                prefetched.prepared
            } else {
                prepare_tts_batch(&ctx, &plan, plan.start_idx, chunk_end, engine.as_ref())
                    .unwrap_or_else(|err| {
                        transition_tts_runtime_to_paused(
                            &ctx,
                            runtime_request_id,
                            "reader_tts_runtime_error",
                            &format!("Failed to prepare TTS audio batch: {err}"),
                        );
                        Vec::new()
                    })
            }
        } else {
            prepare_tts_batch(&ctx, &plan, plan.start_idx, chunk_end, engine.as_ref())
                .unwrap_or_else(|err| {
                    transition_tts_runtime_to_paused(
                        &ctx,
                        runtime_request_id,
                        "reader_tts_runtime_error",
                        &format!("Failed to prepare TTS audio batch: {err}"),
                    );
                    Vec::new()
                })
        };

        if cancel_token.is_cancelled() {
            break;
        }
        if prepared.is_empty() {
            transition_tts_runtime_to_paused(
                &ctx,
                runtime_request_id,
                "reader_tts_runtime_stopped",
                "Prepared TTS batch was empty",
            );
            break;
        }

        let next_chunk_start = chunk_end;
        let pending_prefetch = if next_chunk_start < plan.sentences.len() {
            let next_chunk_end =
                (next_chunk_start + TTS_PREPARE_SENTENCE_WINDOW).min(plan.sentences.len());
            let next_engine = engine.as_ref().cloned();
            let next_ctx = ctx.clone();
            let next_plan = plan.clone();
            Some(PendingPrefetch {
                source_path: plan.source_path.clone(),
                page: plan.page,
                start_idx: next_chunk_start,
                handle: thread::spawn(move || {
                    prepare_tts_batch(
                        &next_ctx,
                        &next_plan,
                        next_chunk_start,
                        next_chunk_end,
                        next_engine.as_ref(),
                    )
                    .map_err(|err| err.to_string())
                }),
            })
        } else {
            None
        };

        let (boundary_tx, boundary_rx) = mpsc::channel::<tts::TtsSentenceBoundary>();
        let playback = match build_playback(
            &ctx,
            &plan,
            &prepared,
            engine.as_ref(),
            boundary_tx,
        ) {
            Ok(playback) => playback,
            Err(err) => {
                if cancel_token.is_cancelled() {
                    break;
                }
                transition_tts_runtime_to_paused(
                    &ctx,
                    runtime_request_id,
                    "reader_tts_runtime_error",
                    &format!("Failed to start {:?} TTS playback: {err}", plan.backend),
                );
                break;
            }
        };

        if playback_started_at.is_none() {
            playback_started_at = Some(Instant::now());
            log_tts_playback_latency(
                &ctx,
                runtime_request_id,
                runtime_started_at,
                playback_started_at,
            );
        }
        emit_queued_event(&ctx, runtime_request_id, &plan, &prepared);

        let mut continue_playback = true;
        let mut boundaries_seen = 0usize;
        while boundaries_seen < prepared.len() {
            if cancel_token.is_cancelled() {
                playback.stop();
                clear_tts_request_if_current(&ctx, runtime_request_id);
                emit_terminal_event(
                    &ctx,
                    runtime_request_id,
                    TtsRuntimeEventKind::Cancelled,
                    "reader_tts_runtime_cancelled",
                    None,
                );
                return;
            }

            if pause_requested.load(Ordering::SeqCst) {
                if !playback.is_paused() {
                    playback.pause();
                }
                thread::sleep(TTS_PROGRESS_POLL_INTERVAL);
                continue;
            }

            if playback.is_paused() {
                playback.play();
            }
            match boundary_rx.recv_timeout(TTS_PROGRESS_POLL_INTERVAL) {
                Ok(boundary) => {
                    playback.boundary_consumed();
                    if let Some(playback_view) =
                        apply_tts_audio_boundary(&ctx, runtime_request_id, boundary)
                    {
                        boundaries_seen = boundaries_seen.saturating_add(1);
                        emit_playback_event(
                            &ctx,
                            runtime_request_id,
                            "reader_tts_sentence_started",
                            playback_view,
                            TtsRuntimeEventKind::SentenceStarted,
                            None,
                        );
                    }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    continue_playback = false;
                    break;
                }
            }
        }

        loop {
            if cancel_token.is_cancelled() {
                playback.stop();
                clear_tts_request_if_current(&ctx, runtime_request_id);
                emit_terminal_event(
                    &ctx,
                    runtime_request_id,
                    TtsRuntimeEventKind::Cancelled,
                    "reader_tts_runtime_cancelled",
                    None,
                );
                return;
            }

            if pause_requested.load(Ordering::SeqCst) {
                if !playback.is_paused() {
                    playback.pause();
                }
                thread::sleep(TTS_PROGRESS_POLL_INTERVAL);
                continue;
            }

            if playback.is_paused() {
                playback.play();
            }

            if playback.queued_sources() == 0 {
                break;
            }

            thread::sleep(TTS_PROGRESS_POLL_INTERVAL);
        }

        playback.stop();

        if !continue_playback {
            break;
        }

        if let Some(pending) = pending_prefetch {
            match pending.handle.join() {
                Ok(Ok(prepared)) => {
                    ready_prefetch = Some(PrefetchedBatch {
                        source_path: pending.source_path,
                        page: pending.page,
                        start_idx: pending.start_idx,
                        prepared,
                    });
                }
                Ok(Err(err)) => {
                    warn!(
                        runtime_request_id,
                        page = pending.page + 1,
                        sentence_idx = pending.start_idx,
                        error = %err,
                        "Failed to prefetch next TTS batch; runtime will fall back to inline prepare"
                    );
                }
                Err(_) => {
                    warn!(
                        runtime_request_id,
                        page = pending.page + 1,
                        sentence_idx = pending.start_idx,
                        "TTS prefetch worker panicked; runtime will fall back to inline prepare"
                    );
                }
            }
        }
    }

    clear_tts_request_if_current(&ctx, runtime_request_id);
    let kind = if cancel_token.is_cancelled() {
        TtsRuntimeEventKind::Cancelled
    } else {
        TtsRuntimeEventKind::Completed
    };
    let action = if kind == TtsRuntimeEventKind::Cancelled {
        "reader_tts_runtime_cancelled"
    } else {
        "reader_tts_runtime_complete"
    };
    emit_terminal_event(&ctx, runtime_request_id, kind, action, None);
}

fn collect_tts_playback_plan(
    ctx: &TtsRuntimeContext,
    runtime_request_id: u64,
) -> Option<TtsPlaybackPlan> {
    let mut guard = ctx.session.lock().ok()?;
    let Some(reader) = guard.as_mut() else {
        return None;
    };
    let current_request_id = ctx
        .request
        .lock()
        .ok()
        .and_then(|guard| guard.as_ref().map(|runtime| runtime.request_id));
    if current_request_id != Some(runtime_request_id) {
        return None;
    }
    let playback = reader.playback_view(&ctx.normalizer);
    if playback.tts.state != session::TtsPlaybackState::Playing {
        return None;
    }
    let (audio_sentences, start_idx) = reader.current_tts_audio_slice(&ctx.normalizer);
    let display_ids = reader.current_tts_audio_display_ids(&ctx.normalizer);
    if audio_sentences.is_empty() {
        return None;
    }
    Some(TtsPlaybackPlan {
        source_path: reader.source_path.clone(),
        page: playback.current_page,
        sentences: audio_sentences,
        display_ids,
        start_idx,
        pause_after: Duration::from_secs_f64(reader.config.pause_after_sentence.max(0.0) as f64),
        speed: reader.config.tts_speed,
        volume: reader.config.tts_volume,
        threads: reader.config.tts_threads.max(1),
        progress_log_interval: Duration::from_secs_f64(
            reader.config.tts_progress_log_interval_secs.max(0.1) as f64,
        ),
        model_path: PathBuf::from(reader.config.tts_model_path.clone()),
        espeak_path: PathBuf::from(reader.config.tts_espeak_path.clone()),
        backend: reader.config.tts_backend,
        windows_voice_id: reader.config.windows_voice_id.clone(),
    })
}

fn clear_tts_request_if_current(ctx: &TtsRuntimeContext, runtime_request_id: u64) {
    if let Ok(mut guard) = ctx.request.lock() {
        let current_request_id = guard.as_ref().map(|runtime| runtime.request_id);
        if current_request_id == Some(runtime_request_id) {
            *guard = None;
        }
    }
}

fn transition_tts_runtime_to_paused(
    ctx: &TtsRuntimeContext,
    runtime_request_id: u64,
    action: &str,
    message: &str,
) {
    let event_payload = {
        let mut guard = match ctx.session.lock() {
            Ok(guard) => guard,
            Err(_) => return,
        };
        let current_request_id = ctx
            .request
            .lock()
            .ok()
            .and_then(|guard| guard.as_ref().map(|runtime| runtime.request_id));
        if current_request_id != Some(runtime_request_id) {
            return;
        }
        let reader = match guard.as_mut() {
            Some(reader) => reader,
            None => return,
        };
        let delta =
            reader.apply_command_lightweight(session::SessionCommand::TtsPause, &ctx.normalizer);
        persist_reader_progress(reader, "tts_runtime_pause");
        Some((delta.playback, reader.source_path.clone()))
    };

    if let Some((playback, source_path)) = event_payload {
        warn!(
            runtime_request_id,
            source = %source_path.display(),
            error = %message,
            "TTS runtime transitioned to paused"
        );
        emit_playback_event(
            ctx,
            runtime_request_id,
            action,
            playback,
            TtsRuntimeEventKind::Failed,
            Some(message.to_string()),
        );
    }
}

fn apply_tts_audio_boundary(
    ctx: &TtsRuntimeContext,
    runtime_request_id: u64,
    boundary: tts::TtsSentenceBoundary,
) -> Option<session::ReaderPlaybackView> {
    let event_payload = {
        let mut guard = ctx.session.lock().ok()?;
        let current_request_id = ctx
            .request
            .lock()
            .ok()
            .and_then(|guard| guard.as_ref().map(|runtime| runtime.request_id));
        if current_request_id != Some(runtime_request_id) {
            return None;
        }
        let reader = guard.as_mut()?;
        let delta = reader.apply_tts_sentence_boundary(
            &ctx.normalizer,
            boundary.audio_idx,
            boundary.canonical_display_id,
        )?;
        persist_reader_progress(reader, "tts_runtime_sentence_started");
        Some(delta.playback)
    };
    event_payload
}

fn persist_reader_progress(reader: &mut session::ReaderSession, reason: &'static str) {
    let bookmark = reader.to_bookmark();
    let source_path = reader.source_path.clone();
    trace!(
        path = %reader.source_path.display(),
        page = reader.current_page + 1,
        reason,
        "Persisting active reader progress"
    );
    let _ = cache::save_bookmark(source_path.as_path(), &bookmark);
}

#[derive(Debug, Clone)]
struct PreparedSentence {
    path: Option<PathBuf>,
    duration: Duration,
    canonical_display_id: usize,
}

fn prepare_tts_batch(
    ctx: &TtsRuntimeContext,
    plan: &TtsPlaybackPlan,
    chunk_start: usize,
    chunk_end: usize,
    engine: Option<&tts::TtsEngine>,
) -> Result<Vec<PreparedSentence>, String> {
    let chunk_sentences = plan.sentences[chunk_start..chunk_end].to_vec();
    trace!(
        page = plan.page + 1,
        chunk_start,
        chunk_end,
        mode = ?ctx.mode,
        "Preparing TTS batch"
    );
    match ctx.mode {
        TtsRuntimeMode::Simulated => Ok(chunk_sentences
            .iter()
            .enumerate()
            .map(|(offset, sentence)| PreparedSentence {
                path: None,
                duration: simulated_sentence_duration(sentence, plan.speed),
                canonical_display_id: plan
                    .display_ids
                    .get(chunk_start + offset)
                    .copied()
                    .unwrap_or(chunk_start + offset),
            })
            .collect()),
        TtsRuntimeMode::Real => {
            let engine = engine.ok_or_else(|| "TTS engine missing".to_string())?;
            let cache_root = cache::hash_dir(&plan.source_path).join("tts");
            let prepared = engine
                .prepare_batch(
                    cache_root,
                    chunk_sentences,
                    0,
                    plan.threads,
                    plan.progress_log_interval,
                )
                .map_err(|err| err.to_string())?;
            trace!(
                page = plan.page + 1,
                prepared = prepared.len(),
                "Prepared TTS batch"
            );
            Ok(prepared
                .into_iter()
                .enumerate()
                .map(|(offset, (path, duration))| PreparedSentence {
                    path: Some(path),
                    duration,
                    canonical_display_id: plan
                        .display_ids
                        .get(chunk_start + offset)
                        .copied()
                        .unwrap_or(chunk_start + offset),
                })
                .collect())
        }
    }
}

struct PlaybackHandle {
    kind: PlaybackKind,
    sentence_durations: Vec<Duration>,
}

enum PlaybackKind {
    Real(tts::TtsPlayback),
    Simulated {
        paused: Arc<AtomicBool>,
        queued: Arc<AtomicUsize>,
    },
}

impl PlaybackHandle {
    fn boundary_consumed(&self) {
        if let PlaybackKind::Simulated { queued, .. } = &self.kind {
            queued.fetch_sub(1, Ordering::SeqCst);
        }
    }

    fn pause(&self) {
        match &self.kind {
            PlaybackKind::Real(playback) => playback.pause(),
            PlaybackKind::Simulated { paused, .. } => {
                paused.store(true, Ordering::SeqCst);
            }
        }
    }

    fn play(&self) {
        match &self.kind {
            PlaybackKind::Real(playback) => playback.play(),
            PlaybackKind::Simulated { paused, .. } => {
                paused.store(false, Ordering::SeqCst);
            }
        }
    }

    fn is_paused(&self) -> bool {
        match &self.kind {
            PlaybackKind::Real(playback) => playback.is_paused(),
            PlaybackKind::Simulated { paused, .. } => paused.load(Ordering::SeqCst),
        }
    }

    fn stop(self) {
        match self.kind {
            PlaybackKind::Real(playback) => playback.stop(),
            PlaybackKind::Simulated { queued, .. } => {
                queued.store(0, Ordering::SeqCst);
            }
        }
    }

    fn queued_sources(&self) -> usize {
        match &self.kind {
            PlaybackKind::Real(playback) => playback.queued_sources(),
            PlaybackKind::Simulated { queued, .. } => queued.load(Ordering::SeqCst),
        }
    }
}

fn build_playback(
    ctx: &TtsRuntimeContext,
    plan: &TtsPlaybackPlan,
    prepared: &[PreparedSentence],
    engine: Option<&tts::TtsEngine>,
    boundary_tx: mpsc::Sender<tts::TtsSentenceBoundary>,
) -> Result<PlaybackHandle, String> {
    match ctx.mode {
        TtsRuntimeMode::Simulated => {
            let sentence_durations = prepared
                .iter()
                .map(|item| item.duration)
                .collect::<Vec<_>>();
            let queued = Arc::new(AtomicUsize::new(sentence_durations.len()));
            let boundaries = prepared
                .iter()
                .enumerate()
                .map(|(index, item)| tts::TtsSentenceBoundary {
                    audio_idx: plan.start_idx.saturating_add(index),
                    canonical_display_id: item.canonical_display_id,
                })
                .collect::<Vec<_>>();
            if let Some(driver) = ctx.simulated_boundary_driver.as_ref() {
                driver.install(boundary_tx, boundaries);
            } else {
                for boundary in boundaries {
                    let _ = boundary_tx.send(boundary);
                }
            }
            Ok(PlaybackHandle {
                kind: PlaybackKind::Simulated {
                    paused: Arc::new(AtomicBool::new(false)),
                    queued,
                },
                sentence_durations,
            })
        }
        TtsRuntimeMode::Real => {
            let engine = engine.ok_or_else(|| "TTS engine missing".to_string())?;
            let files: Vec<PathBuf> = prepared
                .iter()
                .filter_map(|item| item.path.clone())
                .collect();
            let canonical_display_ids = prepared
                .iter()
                .map(|item| item.canonical_display_id)
                .collect::<Vec<_>>();
            let plan_start_idx = plan.start_idx;
            let marker = Arc::new(move |boundary: tts::TtsSentenceBoundary| {
                let _ = boundary_tx.send(tts::TtsSentenceBoundary {
                    audio_idx: plan_start_idx.saturating_add(boundary.audio_idx),
                    canonical_display_id: boundary.canonical_display_id,
                });
            });
            let playback = engine
                .play_files_with_sentence_ids(
                    &files,
                    &canonical_display_ids,
                    plan.pause_after,
                    plan.speed,
                    plan.volume,
                    false,
                    Some(marker),
                )
                .map_err(|err| err.to_string())?;
            let sentence_durations = playback.sentence_durations().to_vec();
            Ok(PlaybackHandle {
                kind: PlaybackKind::Real(playback),
                sentence_durations,
            })
        }
    }
}

fn emit_playback_event(
    ctx: &TtsRuntimeContext,
    request_id: u64,
    action: &str,
    playback: session::ReaderPlaybackView,
    kind: TtsRuntimeEventKind,
    message: Option<String>,
) {
    let cursor = cursor_from_playback(&playback);
    let event = TtsRuntimeEvent {
        request_id,
        action: action.to_string(),
        kind,
        snapshot: None,
        playback: Some(reader_playback_state_from_view(&playback)),
        tts: Some(playback.tts.clone()),
        message,
        cursor,
    };
    let _ = ctx.event_tx.send(event);
}

fn emit_terminal_event(
    ctx: &TtsRuntimeContext,
    request_id: u64,
    kind: TtsRuntimeEventKind,
    action: &str,
    message: Option<String>,
) {
    let event = TtsRuntimeEvent {
        request_id,
        action: action.to_string(),
        kind,
        snapshot: None,
        playback: None,
        tts: None,
        message,
        cursor: None,
    };
    let _ = ctx.event_tx.send(event);
}

fn emit_queued_event(
    ctx: &TtsRuntimeContext,
    request_id: u64,
    plan: &TtsPlaybackPlan,
    prepared: &[PreparedSentence],
) {
    let event = TtsRuntimeEvent {
        request_id,
        action: "reader_tts_runtime_queue".to_string(),
        kind: TtsRuntimeEventKind::Queued,
        snapshot: None,
        playback: None,
        tts: None,
        message: Some(format!(
            "queued {} sentences for page {}",
            prepared.len(),
            plan.page + 1
        )),
        cursor: None,
    };
    let _ = ctx.event_tx.send(event);
}

fn cursor_from_playback(snapshot: &session::ReaderPlaybackView) -> Option<TtsCursor> {
    Some(TtsCursor {
        audio_idx: snapshot.tts.current_sentence_idx,
        display_idx: snapshot.highlighted_sentence_idx,
        page: snapshot.current_page,
    })
}

fn reader_playback_state_from_view(
    reader: &session::ReaderPlaybackView,
) -> crate::contracts::ReaderPlaybackState {
    crate::contracts::ReaderPlaybackState {
        source_path: reader.source_path.clone(),
        current_page: reader.current_page,
        highlighted_sentence_idx: reader.highlighted_sentence_idx,
        tts: reader.tts.clone(),
        stats: reader.stats.clone(),
        updated_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64,
    }
}

fn should_sync_tts_after_reader_command(command: &session::SessionCommand) -> bool {
    match command {
        session::SessionCommand::GetSnapshot => false,
        session::SessionCommand::ApplySettings { patch } => {
            patch.font_size.is_some()
                || patch.lines_per_page.is_some()
                || patch.pause_after_sentence.is_some()
                || patch.tts_speed.is_some()
                || patch.tts_volume.is_some()
                || patch.tts_backend.is_some()
                || patch.windows_voice_id.is_some()
        }
        _ => true,
    }
}

fn patch_has_tts_fields(patch: &session::ReaderSettingsPatch) -> bool {
    patch.tts_speed.is_some()
        || patch.tts_volume.is_some()
        || patch.pause_after_sentence.is_some()
        || patch.auto_scroll_tts.is_some()
        || patch.tts_backend.is_some()
        || patch.windows_voice_id.is_some()
}

fn panels_snapshot(panels: &Mutex<session::PanelState>) -> session::PanelState {
    panels.lock().map(|guard| *guard).unwrap_or_default()
}

fn log_tts_playback_latency(
    ctx: &TtsRuntimeContext,
    request_id: u64,
    runtime_started_at: Instant,
    playback_started_at: Option<Instant>,
) {
    let elapsed_ms = playback_started_at
        .unwrap_or(runtime_started_at)
        .saturating_duration_since(runtime_started_at)
        .as_millis();
    let command = ctx
        .last_command
        .lock()
        .ok()
        .and_then(|guard| guard.clone())
        .unwrap_or_else(|| "tts.unknown".to_string());
    trace!(
        request_id,
        tts_command = %command,
        elapsed_ms,
        target_play_from_page_ms = TTS_TARGET_PLAY_FROM_PAGE_MS,
        target_play_from_highlight_ms = TTS_TARGET_PLAY_FROM_HIGHLIGHT_MS,
        target_warm_cache_ms = TTS_TARGET_WARM_CACHE_MS,
        target_cold_cache_ms = TTS_TARGET_COLD_CACHE_MS,
        "TTS playback latency recorded"
    );
}

fn simulated_sentence_duration(sentence: &str, speed: f32) -> Duration {
    let words = sentence.split_whitespace().count().max(1) as f32;
    let base_wpm = 180.0;
    let effective_wpm = (base_wpm * speed.max(0.25)).max(60.0);
    let seconds = words / (effective_wpm / 60.0);
    Duration::from_secs_f32(seconds.max(0.15))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sentence_started_events_are_not_coalesced_with_progress() {
        let (tx, rx) = mpsc::channel();
        for kind in [
            TtsRuntimeEventKind::SentenceStarted,
            TtsRuntimeEventKind::SentenceStarted,
            TtsRuntimeEventKind::Progress,
            TtsRuntimeEventKind::Progress,
        ] {
            tx.send(TtsRuntimeEvent {
                request_id: 7,
                action: "test".to_string(),
                kind,
                snapshot: None,
                playback: None,
                tts: None,
                message: None,
                cursor: None,
            })
            .unwrap();
        }
        let mut batcher = TtsEventBatcher::default();
        batcher.collect(&rx);
        let events = batcher.drain();
        assert_eq!(
            events
                .iter()
                .filter(|event| event.kind == TtsRuntimeEventKind::SentenceStarted)
                .count(),
            2
        );
        assert_eq!(
            events
                .iter()
                .filter(|event| event.kind == TtsRuntimeEventKind::Progress)
                .count(),
            1
        );
    }

    fn build_test_session(page_sentences: &[&[&str]]) -> session::ReaderSession {
        let pages: Vec<String> = page_sentences
            .iter()
            .map(|sentences| sentences.join(" "))
            .collect();
        let raw_page_sentences: Vec<Vec<String>> = page_sentences
            .iter()
            .map(|sentences| sentences.iter().map(|s| s.to_string()).collect())
            .collect();

        session::ReaderSession::from_pages_for_test(
            PathBuf::from("/tmp/test.epub"),
            "test.epub".to_string(),
            pages,
            raw_page_sentences,
        )
    }

    #[test]
    fn tts_command_updates_snapshot_and_state() {
        let normalizer = normalizer::TextNormalizer::default();
        let runtime = TtsRuntime::new_with_mode(normalizer, TtsRuntimeMode::Simulated);
        runtime.set_session(Some(build_test_session(&[&["A.", "B."]])));

        let snapshot = runtime.apply_command(TtsCommand::Play).expect("snapshot");
        assert_eq!(snapshot.tts.state, session::TtsPlaybackState::Playing);
    }

    #[test]
    fn large_session_tts_submission_is_bounded_and_worker_owned() {
        let normalizer = normalizer::TextNormalizer::default();
        let runtime = TtsRuntime::new_with_mode(normalizer, TtsRuntimeMode::Simulated);
        let sentences: Vec<String> = (0..10_488)
            .map(|idx| format!("Sentence {idx} is deterministic."))
            .collect();
        let page = sentences.join(" ");
        runtime.set_session(Some(session::ReaderSession::from_pages_for_test(
            PathBuf::from("/tmp/large-test.epub"),
            "large-test.epub".to_string(),
            vec![page],
            vec![sentences],
        )));
        runtime.reset_snapshot_construction_count();

        let started = Instant::now();
        assert!(runtime.submit_command(TtsCommand::PlayFromHighlight));
        assert!(
            started.elapsed() < Duration::from_millis(50),
            "egui-facing TTS submission took {:?}",
            started.elapsed()
        );

        let deadline = Instant::now() + Duration::from_secs(2);
        let mut saw_state = false;
        while Instant::now() < deadline {
            let events = runtime.collect_events();
            assert!(events.iter().all(|event| event.snapshot.is_none()));
            saw_state |= events
                .iter()
                .any(|event| event.kind == TtsRuntimeEventKind::StateChanged);
            if saw_state {
                break;
            }
            thread::sleep(Duration::from_millis(5));
        }
        assert!(
            saw_state,
            "TTS control worker did not begin the large session"
        );
        for _ in 0..100 {
            assert!(runtime.submit_command(TtsCommand::SeekNext));
        }
        let deadline = Instant::now() + Duration::from_secs(2);
        while Instant::now() < deadline {
            let _ = runtime.collect_events();
            if runtime.snapshot_construction_count() != 0 {
                break;
            }
            thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(
            runtime.snapshot_construction_count(),
            0,
            "TTS worker constructed a full ReaderSnapshot in its hot path"
        );
    }

    #[test]
    fn canonical_session_cursor_is_shared_between_reader_and_tts_paths() {
        let normalizer = normalizer::TextNormalizer::default();
        let shared = Arc::new(Mutex::new(Some(build_test_session(&[&["A.", "B.", "C."]]))));
        let runtime = TtsRuntime::new_with_session(
            normalizer.clone(),
            TtsRuntimeMode::Simulated,
            Arc::clone(&shared),
        );
        {
            let mut guard = shared.lock().expect("session lock");
            guard.as_mut().expect("session").apply_command(
                session::SessionCommand::SentenceClick { sentence_idx: 2 },
                session::PanelState::default(),
                &normalizer,
            );
        }
        let view = runtime
            .apply_command(TtsCommand::PlayFromHighlight)
            .expect("playback view");
        assert_eq!(view.highlighted_sentence_idx, Some(2));
        assert_eq!(view.tts.current_sentence_idx, Some(2));

        let _ = runtime.apply_command(TtsCommand::SeekPrev);
        let mut guard = shared.lock().expect("session lock");
        let projected = guard.as_mut().expect("session").playback_view(&normalizer);
        assert_eq!(projected.tts.current_sentence_idx, Some(1));
    }

    #[test]
    fn tts_runtime_emits_progress_events() {
        let normalizer = normalizer::TextNormalizer::default();
        let runtime = TtsRuntime::new_with_mode(normalizer, TtsRuntimeMode::Simulated);
        let driver = runtime
            .simulated_boundary_driver()
            .expect("simulated runtime exposes its boundary driver");
        runtime.set_session(Some(build_test_session(&[&["A.", "B.", "C."]])));

        let _ = runtime.apply_command(TtsCommand::Play);
        let started = Instant::now();
        let mut saw_progress = false;
        while started.elapsed() < Duration::from_millis(300) {
            let _ = driver.emit_next();
            let events = runtime.collect_events();
            if events.iter().any(|event| {
                event.kind == TtsRuntimeEventKind::Progress
                    || event.kind == TtsRuntimeEventKind::SentenceStarted
            }) {
                saw_progress = true;
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
        assert!(saw_progress, "expected at least one progress event");
    }

    #[test]
    fn controlled_runtime_boundaries_preserve_identity_across_windows_and_controls() {
        let normalizer = normalizer::TextNormalizer::default();
        let runtime = TtsRuntime::new_with_mode(normalizer, TtsRuntimeMode::Simulated);
        let driver = runtime
            .simulated_boundary_driver()
            .expect("simulated runtime exposes its boundary driver");
        let sentences: Vec<String> = (0..136)
            .map(|idx| format!("Boundary sentence {idx} is source-born."))
            .collect();
        runtime.set_session(Some(session::ReaderSession::from_pages_for_test(
            PathBuf::from("/tmp/boundary-identity.epub"),
            "boundary-identity.epub".to_string(),
            vec![sentences.join(" ")],
            vec![sentences.clone()],
        )));

        let wait_for_boundary = |runtime: &TtsRuntime,
                                 driver: &SimulatedBoundaryDriver|
         -> Option<usize> {
            let deadline = Instant::now() + Duration::from_secs(2);
            let mut emitted = false;
            while Instant::now() < deadline {
                if !emitted {
                    emitted = driver.emit_next();
                }
                if let Some(event) = runtime
                    .collect_events()
                    .into_iter()
                    .find(|event| event.kind == TtsRuntimeEventKind::SentenceStarted)
                {
                    return event.cursor.and_then(|cursor| cursor.display_idx);
                }
                thread::yield_now();
            }
            None
        };

        assert!(runtime.submit_command(TtsCommand::Play));
        let first = wait_for_boundary(&runtime, &driver).expect("first boundary");

        assert!(runtime.submit_command(TtsCommand::Pause));
        let pause_deadline = Instant::now() + Duration::from_millis(300);
        while Instant::now() < pause_deadline {
            let _ = runtime.collect_events();
            thread::yield_now();
        }
        assert!(!driver.emit_next(), "paused simulated audio must not emit a boundary");
        assert_eq!(
            runtime
                .snapshot()
                .and_then(|snapshot| snapshot.highlighted_sentence_idx),
            Some(first)
        );

        assert!(runtime.submit_command(TtsCommand::Play));
        assert_eq!(wait_for_boundary(&runtime, &driver), Some(first + 1));

        assert!(runtime.submit_command(TtsCommand::RepeatSentence));
        let repeat_deadline = Instant::now() + Duration::from_millis(300);
        while Instant::now() < repeat_deadline {
            let _ = runtime.collect_events();
            if runtime
                .snapshot()
                .and_then(|snapshot| snapshot.highlighted_sentence_idx)
                == Some(first + 1)
            {
                break;
            }
            thread::yield_now();
        }
        assert_eq!(
            runtime
                .snapshot()
                .and_then(|snapshot| snapshot.highlighted_sentence_idx),
            Some(first + 1)
        );

        assert!(runtime.submit_command(TtsCommand::SeekNext));
        let next_deadline = Instant::now() + Duration::from_millis(300);
        while Instant::now() < next_deadline {
            let _ = runtime.collect_events();
            thread::yield_now();
        }
        assert!(runtime.submit_command(TtsCommand::SeekPrev));

        let window_runtime = TtsRuntime::new_with_mode(
            normalizer::TextNormalizer::default(),
            TtsRuntimeMode::Simulated,
        );
        let window_driver = window_runtime
            .simulated_boundary_driver()
            .expect("window runtime exposes boundary driver");
        window_runtime.set_session(Some(session::ReaderSession::from_pages_for_test(
            PathBuf::from("/tmp/boundary-window.epub"),
            "boundary-window.epub".to_string(),
            vec![sentences.join(" ")],
            vec![sentences.clone()],
        )));
        assert!(window_runtime.submit_command(TtsCommand::Play));
        let mut observed = Vec::new();
        while observed.len() < 131 {
            let Some(idx) = wait_for_boundary(&window_runtime, &window_driver) else {
                break;
            };
            observed.push(idx);
        }
        assert!(observed.len() >= 131, "expected 128+ runtime boundaries");
        assert!(
            observed.iter().any(|idx| *idx >= 64),
            "boundary identities did not cross the second window: max={:?}",
            observed.iter().max()
        );
    }

    #[test]
    fn tts_runtime_cancels_on_clear_session() {
        let normalizer = normalizer::TextNormalizer::default();
        let runtime = TtsRuntime::new_with_mode(normalizer, TtsRuntimeMode::Simulated);
        runtime.set_session(Some(build_test_session(&[&["A.", "B."]])));

        let _ = runtime.apply_command(TtsCommand::Play);
        runtime.set_session(None);
        thread::sleep(Duration::from_millis(10));
        let events = runtime.collect_events();
        assert!(
            events
                .iter()
                .any(|event| event.kind == TtsRuntimeEventKind::Completed
                    || event.kind == TtsRuntimeEventKind::Cancelled)
        );
    }

    #[test]
    fn backend_and_voice_settings_are_active_resync_changes() {
        assert!(should_sync_tts_after_reader_command(
            &session::SessionCommand::ApplySettings {
                patch: session::ReaderSettingsPatch {
                    tts_backend: Some(config::TtsBackend::Windows),
                    ..Default::default()
                }
            }
        ));
        assert!(should_sync_tts_after_reader_command(
            &session::SessionCommand::ApplySettings {
                patch: session::ReaderSettingsPatch {
                    windows_voice_id: Some("voice-id".to_string()),
                    ..Default::default()
                }
            }
        ));
        assert!(!should_sync_tts_after_reader_command(
            &session::SessionCommand::ApplySettings {
                patch: session::ReaderSettingsPatch {
                    theme: Some(config::ThemeMode::Day),
                    ..Default::default()
                }
            }
        ));
    }

    #[test]
    fn active_backend_switch_preserves_cursor_and_replaces_request() {
        let normalizer = normalizer::TextNormalizer::default();
        let runtime = TtsRuntime::new_with_mode(normalizer, TtsRuntimeMode::Simulated);
        runtime.set_session(Some(build_test_session(&[&["A.", "B."]])));
        let started = runtime
            .apply_command(TtsCommand::Play)
            .expect("play snapshot");
        let before_idx = started.tts.current_sentence_idx;
        let _ = runtime.collect_events();

        let after = runtime
            .apply_command(TtsCommand::ApplySettings {
                patch: session::ReaderSettingsPatch {
                    tts_backend: Some(config::TtsBackend::Windows),
                    ..Default::default()
                },
            })
            .expect("backend patch snapshot");
        assert_eq!(after.tts.current_sentence_idx, before_idx);
        assert_eq!(after.settings.tts_backend, config::TtsBackend::Windows);
        assert!(
            runtime
                .collect_events()
                .iter()
                .any(|event| event.kind == TtsRuntimeEventKind::Cancelled)
        );
    }

    #[test]
    fn active_voice_switch_preserves_cursor_and_replaces_request() {
        let normalizer = normalizer::TextNormalizer::default();
        let runtime = TtsRuntime::new_with_mode(normalizer, TtsRuntimeMode::Simulated);
        runtime.set_session(Some(build_test_session(&[&["A.", "B."]])));
        let started = runtime
            .apply_command(TtsCommand::Play)
            .expect("play snapshot");
        let before_idx = started.tts.current_sentence_idx;
        let _ = runtime.collect_events();

        let after = runtime
            .apply_command(TtsCommand::ApplySettings {
                patch: session::ReaderSettingsPatch {
                    windows_voice_id: Some("voice-id".to_string()),
                    ..Default::default()
                },
            })
            .expect("voice patch snapshot");
        assert_eq!(after.tts.current_sentence_idx, before_idx);
        assert_eq!(after.settings.windows_voice_id.as_deref(), Some("voice-id"));
        assert!(
            runtime
                .collect_events()
                .iter()
                .any(|event| event.kind == TtsRuntimeEventKind::Cancelled)
        );
    }
}
