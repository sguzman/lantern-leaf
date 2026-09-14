use std::{path::PathBuf, sync::Arc, time::Instant};

use lanternleaf_app::contracts::{
    PdfEmbeddedTextEvent, PdfEmbeddedTextPreparedEvent, ReaderSnapshot,
};
use lanternleaf_app::pipeline::{
    AppCommand, AppEvent, DispatchPlan, PersistenceOutcome, PersistenceTrigger, PlannedEffect,
    ReaderCommand, RuntimeEffect,
};
use lanternleaf_core::cache::PdfSentencePageHint;
use tracing::trace;

use super::{LanternLeafApp, LifecycleSignal, StatusLogEntry};
use crate::shell::NotificationLevel;

pub(crate) fn starter_startup_commands() -> Vec<AppCommand> {
    vec![
        AppCommand::Bootstrap,
        AppCommand::RefreshRecents { limit: None },
        AppCommand::LoadCalibreBooks {
            force_refresh: false,
        },
    ]
}

fn refresh_recents_after_persistence(event: &AppEvent) -> bool {
    matches!(
        event,
        AppEvent::PersistenceFlushed {
            trigger: PersistenceTrigger::SourceOpen,
            outcome: PersistenceOutcome::Completed,
            ..
        }
    )
}

impl LanternLeafApp {
    pub(crate) fn execute_startup_commands(&mut self) {
        for command in starter_startup_commands() {
            self.execute_command(command);
        }
    }

    pub(crate) fn execute_command(&mut self, command: AppCommand) {
        if let AppCommand::EnsureCalibreThumbnail { id } = command {
            self.execute_calibre_thumbnail(id);
            return;
        }
        if matches!(command, AppCommand::CloseReaderSession) {
            self.begin_close_reader();
            return;
        }
        if matches!(command, AppCommand::SafeQuit) {
            self.begin_safe_quit();
            return;
        }
        let state_snapshot = self.runtime.state_snapshot();
        let reader_snapshot = state_snapshot.reader_document.snapshot.as_deref();
        self.maybe_record_audio_command(&command, reader_snapshot);
        self.apply_persistence_trigger(&command, reader_snapshot);
        let is_tts_command = matches!(
            &command,
            AppCommand::Reader(ReaderCommand::Session(session_command))
                if lanternleaf_app::tts_runtime::TtsCommand::from_session_command(session_command).is_some()
        );
        let plan = self.runtime.plan_command(command.clone());
        self.apply_local_events(&plan);
        self.log_plan(&plan);
        self.last_plan = Some(plan);
        if !is_tts_command {
            if let Some(plan) = &self.last_plan {
                self.dispatch_effects(plan);
            }
        }
        if is_tts_command {
            self.apply_tts_command_if_needed(&command);
        }
    }

    pub(crate) fn execute_calibre_thumbnail(&mut self, book_id: u64) {
        const MAX_IN_FLIGHT: usize = 4;
        if self.calibre_cover_ownership.is_pending(book_id)
            || self.calibre_cover_ownership.in_flight() >= MAX_IN_FLIGHT
        {
            trace!(book_id, "Coalescing bounded Calibre cover request");
            return;
        }
        let plan = self
            .runtime
            .plan_command(AppCommand::EnsureCalibreThumbnail { id: book_id });
        if !self
            .calibre_cover_ownership
            .claim(book_id, plan.request_id, MAX_IN_FLIGHT)
        {
            return;
        }
        self.apply_local_events(&plan);
        self.log_plan(&plan);
        self.last_plan = Some(plan);
        if let Some(plan) = &self.last_plan {
            self.dispatch_effects(plan);
        }
    }

    fn begin_close_reader(&mut self) {
        self.tts_runtime
            .apply_command(lanternleaf_app::tts_runtime::TtsCommand::Stop);
        self.lifecycle.begin_close_book();
        self.show_reader_confirm_modal = false;
        self.last_reader_source_for_persistence = None;
        self.queue_persistence_flush(PersistenceTrigger::SessionClose);
        self.push_status("Closing book after persistence completes".to_string());
    }

    fn begin_safe_quit(&mut self) {
        self.tts_runtime
            .apply_command(lanternleaf_app::tts_runtime::TtsCommand::Stop);
        self.lifecycle.begin_safe_quit();
        self.queue_persistence_flush(PersistenceTrigger::SafeQuit);
        self.push_status("Safe quit waiting for persistence completion".to_string());
    }

    pub(crate) fn execute_reader_command(&mut self, command: ReaderCommand) {
        self.execute_command(AppCommand::Reader(command));
    }

    fn apply_local_events(&mut self, plan: &DispatchPlan) {
        for event in &plan.local_events {
            self.runtime.apply_event(event.clone());
        }
    }

    fn dispatch_effects(&self, plan: &DispatchPlan) {
        for effect in &plan.effects {
            self.effect_dispatcher.dispatch(effect.clone());
        }
    }

    fn apply_tts_command_if_needed(&mut self, command: &AppCommand) {
        let AppCommand::Reader(ReaderCommand::Session(session_command)) = command else {
            return;
        };
        let Some(tts_command) =
            lanternleaf_app::tts_runtime::TtsCommand::from_session_command(session_command)
        else {
            return;
        };
        trace!(
            tts_command = tts_command.label(),
            action = session_command.action(),
            "Dispatching TTS command to egui runtime"
        );
        let _ = self.tts_runtime.submit_command(tts_command);
    }

    fn apply_persistence_trigger(
        &mut self,
        command: &AppCommand,
        _snapshot: Option<&ReaderSnapshot>,
    ) {
        let (trigger, description) = match command {
            AppCommand::Reader(_) => (Some(PersistenceTrigger::ReaderCommand), "reader_command"),
            AppCommand::SetRuntimeLogLevel { .. } => (
                Some(PersistenceTrigger::RuntimeConfigChange),
                "runtime_config",
            ),
            AppCommand::SafeQuit | AppCommand::FlushPersistence { .. } => (None, ""),
            _ => (None, ""),
        };
        let Some(trigger) = trigger else {
            return;
        };
        self.record_persistence_event(trigger, description);
        self.queue_persistence_flush(trigger);
    }

    pub(crate) fn queue_persistence_flush(&self, trigger: PersistenceTrigger) {
        let request_id = self.runtime.next_request_id();
        trace!(
            request_id,
            trigger = ?trigger,
            "Queued persistence flush effect"
        );
        self.effect_dispatcher.dispatch(PlannedEffect {
            request_id,
            effect: RuntimeEffect::FlushPersistence { trigger },
        });
    }

    pub(crate) fn update_persistence_lifecycle(&mut self, snapshot: Option<&ReaderSnapshot>) {
        if !self.persistence_logged {
            self.persistence.on_startup();
            self.push_status("Persistence: startup".to_string());
            self.persistence_logged = true;
        }

        match snapshot {
            Some(snapshot) => {
                if self
                    .last_reader_source
                    .as_deref()
                    .map(|path| path != snapshot.source_path)
                    .unwrap_or(true)
                {
                    self.record_persistence_status("source_open", &snapshot.source_path);
                    self.queue_persistence_flush(PersistenceTrigger::SourceOpen);
                    self.last_reader_source = Some(snapshot.source_path.clone());
                }
                self.last_reader_source_for_persistence = Some(snapshot.source_path.clone());
            }
            None => {
                if let Some(last_source) = self.last_reader_source_for_persistence.take() {
                    self.record_persistence_status("session_close", &last_source);
                    self.queue_persistence_flush(PersistenceTrigger::SessionClose);
                }
                self.last_reader_source = None;
            }
        }
    }

    fn record_persistence_status(&mut self, label: &str, source_path: &str) {
        self.push_status(format!("Persistence: {label} ({source_path})"));
    }

    pub(crate) fn handle_effect_events(&mut self) {
        for mut event in self.effect_dispatcher.drain_events() {
            trace!(event = ?event, "Applying effect event");
            if let AppEvent::PdfEmbeddedTextCompleted(raw) = &mut event {
                let page_texts = std::mem::take(&mut raw.page_texts);
                self.prepare_pdf_embedded_text_event(PdfEmbeddedTextEvent {
                    request_id: raw.request_id,
                    source_path: raw.source_path.clone(),
                    generation: raw.generation,
                    revision: raw.revision,
                    page_count: raw.page_count,
                    page_texts,
                    worker_thread: raw.worker_thread.clone(),
                    terminal: raw.terminal.clone(),
                    accepted: raw.accepted,
                    degraded_reason: raw.degraded_reason.clone(),
                });
            }
            if matches!(&event, AppEvent::PdfEmbeddedTextPrepared(_)) {
                if let AppEvent::PdfEmbeddedTextPrepared(prepared) = &mut event {
                    self.apply_prepared_pdf_embedded_text_event(prepared);
                }
            }
            match &event {
                _ if refresh_recents_after_persistence(&event) => {
                    trace!("Refreshing starter Recents after successful source persistence");
                    self.execute_command(AppCommand::RefreshRecents { limit: Some(30) });
                }
                AppEvent::PersistenceFlushed {
                    trigger: PersistenceTrigger::SessionClose,
                    outcome: PersistenceOutcome::Completed | PersistenceOutcome::SkippedNoSession,
                    ..
                } => {
                    if self.lifecycle.persistence_completed(
                        PersistenceTrigger::SessionClose,
                        PersistenceOutcome::Completed,
                    ) == LifecycleSignal::ClearReader
                    {
                        self.effect_dispatcher.dispatch(PlannedEffect {
                            request_id: self.runtime.next_request_id(),
                            effect: RuntimeEffect::CloseReaderSession,
                        });
                    }
                }
                AppEvent::PersistenceFlushed {
                    trigger: PersistenceTrigger::SafeQuit,
                    outcome: PersistenceOutcome::Completed | PersistenceOutcome::SkippedNoSession,
                    ..
                } => {
                    let _ = self.lifecycle.persistence_completed(
                        PersistenceTrigger::SafeQuit,
                        PersistenceOutcome::Completed,
                    );
                }
                AppEvent::PersistenceFlushed {
                    trigger: PersistenceTrigger::SessionClose,
                    outcome: PersistenceOutcome::Failed,
                    ..
                } => {
                    let _ = self.lifecycle.persistence_completed(
                        PersistenceTrigger::SessionClose,
                        PersistenceOutcome::Failed,
                    );
                    self.show_reader_confirm_modal = true;
                    self.push_status("Book close canceled because persistence failed".to_string());
                }
                AppEvent::PersistenceFlushed {
                    trigger: PersistenceTrigger::SafeQuit,
                    outcome: PersistenceOutcome::Failed,
                    ..
                } => {
                    let _ = self.lifecycle.persistence_completed(
                        PersistenceTrigger::SafeQuit,
                        PersistenceOutcome::Failed,
                    );
                    self.show_safe_quit_modal = true;
                    self.push_status("Safe quit canceled because persistence failed".to_string());
                }
                AppEvent::PdfEmbeddedTextCompleted(_) => {}
                AppEvent::PdfEmbeddedTextPrepared(_) => {}
                _ => {}
            }
            self.runtime.apply_event(event);
        }
    }

    fn prepare_pdf_embedded_text_event(&mut self, event: PdfEmbeddedTextEvent) {
        let source_path = PathBuf::from(&event.source_path);
        let current_source = self.current_pdf_path.as_ref();
        let current_generation = {
            #[cfg(not(target_arch = "wasm32"))]
            {
                self.pdf_generation
            }
            #[cfg(target_arch = "wasm32")]
            {
                0
            }
        };
        if current_source != Some(&source_path)
            || event.generation != current_generation
            || !event.accepted
        {
            if let Some(reason) = &event.degraded_reason {
                self.push_status(format!("Native PDF text remains visual-only: {reason}"));
            }
            trace!(
                source = %event.source_path,
                generation = event.generation,
                current_generation,
                "Discarded stale or untrusted native PDF text event"
            );
            return;
        }
        let expected_page_count = self
            .runtime
            .state_snapshot()
            .reader_document
            .snapshot
            .as_ref()
            .filter(|snapshot| snapshot.pretty_kind == lanternleaf_app::contracts::PrettyKind::Pdf)
            .map(|snapshot| snapshot.total_pages);
        if expected_page_count != Some(event.page_count)
            || event.page_texts.len() != event.page_count
        {
            self.push_status("Native PDF text rejected because page coverage changed".to_string());
            return;
        }
        let (search_query, search_allowed) = self
            .effect_session
            .lock()
            .ok()
            .and_then(|session| {
                session.as_ref().map(|session| {
                    (
                        session.search_query_for_preparation().to_string(),
                        session.pdf_search_allowed_for_preparation(),
                    )
                })
        })
            .unwrap_or_default();
        let cache_service = Arc::clone(&self.cache_service);
        let event_tx = self.effect_dispatcher.event_tx();
        std::thread::spawn(move || {
            let preparation_thread = format!("{:?}", std::thread::current().id());
            let prepared = match lanternleaf_core::session::ReaderSession::prepare_pdf_embedded_text(
                event.page_texts.clone(),
                &search_query,
                search_allowed,
            ) {
                Ok(prepared) => prepared,
                Err(error) => {
                    let _ =
                        event_tx.send(AppEvent::PdfEmbeddedTextCompleted(PdfEmbeddedTextEvent {
                            accepted: false,
                            degraded_reason: Some(error),
                            ..event
                        }));
                    return;
                }
            };
            let cache_artifact = lanternleaf_core::cache::PdfRenderPrecomputedState {
                version: super::PDF_NATIVE_TEXT_CACHE_VERSION,
                extraction_revision: super::PDF_NATIVE_TEXT_EXTRACTION_REVISION.to_string(),
                source_identity: super::pdf_source_identity(&source_path),
                page_texts: prepared.pages.clone(),
                sentence_page_hints: prepared
                    .sentence_page_hints
                    .iter()
                    .copied()
                    .map(|page_idx| PdfSentencePageHint {
                        page_idx: Some(page_idx),
                    })
                    .collect(),
                source: event.source_path.clone(),
            };
            // Durable cache IO is intentionally on this worker, never on egui.
            cache_service.persist_pdf_render_precomputed_state(&source_path, &cache_artifact);
            let _ = event_tx.send(AppEvent::PdfEmbeddedTextPrepared(
                PdfEmbeddedTextPreparedEvent {
                    request_id: event.request_id,
                    source_path: event.source_path,
                    generation: event.generation,
                    revision: event.revision,
                    page_count: event.page_count,
                    worker_thread: event.worker_thread,
                    preparation_thread,
                    prepared,
                    cache_artifact,
                },
            ));
        });
    }

    fn apply_prepared_pdf_embedded_text_event(&mut self, event: &mut PdfEmbeddedTextPreparedEvent) {
        let source_path = PathBuf::from(&event.source_path);
        let current_source = self.current_pdf_path.as_ref();
        let current_generation = {
            #[cfg(not(target_arch = "wasm32"))]
            {
                self.pdf_generation
            }
            #[cfg(target_arch = "wasm32")]
            {
                0
            }
        };
        if current_source != Some(&source_path)
            || event.generation != current_generation
            || event.revision > self.pdf_page_metadata_revision
            || event.prepared.pages.len() != event.page_count
        {
            trace!(
                source = %event.source_path,
                generation = event.generation,
                current_generation,
                "Discarded stale or incomplete prepared native PDF text event"
            );
            return;
        }
        let normalizer = lanternleaf_core::normalizer::TextNormalizer::load_default();
        let prepared = std::mem::take(&mut event.prepared);
        let mut adoption_error = None;
        let adopted = if let Ok(mut session) = self.effect_session.lock() {
            if let Some(session) = session.as_mut() {
                match session.apply_prepared_pdf_embedded_text(prepared) {
                    Ok(()) => {
                        let panels = self
                            .runtime
                            .state_snapshot()
                            .session
                            .session
                            .map(|state| state.panels)
                            .unwrap_or_default();
                        let snapshot = session.snapshot(panels, &normalizer);
                        self.runtime.apply_event(AppEvent::ReaderUpdated(
                            lanternleaf_app::contracts::ReaderStateEvent {
                                request_id: event.request_id,
                                action: "pdf_embedded_text_adopted".to_string(),
                                reader: snapshot,
                            },
                        ));
                        true
                    }
                    Err(error) => {
                        adoption_error = Some(error);
                        false
                    }
                }
            } else {
                false
            }
        } else {
            false
        };
        if let Some(error) = adoption_error {
            self.push_status(format!("Native PDF text adoption failed: {error}"));
        }
        if adopted {
            self.push_status(format!(
                "Native PDF embedded text accepted ({} pages; worker {}; prepared {})",
                event.page_count, event.worker_thread, event.preparation_thread
            ));
        }
    }

    fn log_plan(&mut self, plan: &DispatchPlan) {
        let entry = format!("Planned {} ({})", plan.action, plan.effects.len());
        self.push_status(entry);
    }

    pub(crate) fn push_status(&mut self, message: String) {
        let message_lower = message.to_lowercase();
        let level = if message_lower.contains("error") || message_lower.contains("failed") {
            NotificationLevel::Error
        } else if message_lower.contains("warn") {
            NotificationLevel::Warn
        } else {
            NotificationLevel::Info
        };
        self.status_log.push(StatusLogEntry {
            timestamp: Instant::now(),
            message,
        });
        if let Some(entry) = self.status_log.last() {
            self.shell_state
                .record_notification(level, entry.message.clone());
        }
        if self.status_log.len() > 8 {
            self.status_log.remove(0);
        }
    }
}

#[cfg(test)]
mod startup_tests {
    use super::*;

    #[test]
    fn starter_startup_loads_caliberate_catalog_without_manual_refresh() {
        let commands = starter_startup_commands();
        assert!(matches!(commands.first(), Some(AppCommand::Bootstrap)));
        assert!(
            commands
                .iter()
                .any(|command| matches!(command, AppCommand::RefreshRecents { limit: None }))
        );
        assert!(commands.iter().any(|command| matches!(
            command,
            AppCommand::LoadCalibreBooks {
                force_refresh: false
            }
        )));
    }

    #[test]
    fn recents_refresh_is_triggered_only_after_successful_source_persistence() {
        assert!(refresh_recents_after_persistence(
            &AppEvent::PersistenceFlushed {
                request_id: 7,
                trigger: PersistenceTrigger::SourceOpen,
                outcome: PersistenceOutcome::Completed,
            }
        ));
        assert!(!refresh_recents_after_persistence(
            &AppEvent::PersistenceFlushed {
                request_id: 7,
                trigger: PersistenceTrigger::SourceOpen,
                outcome: PersistenceOutcome::Failed,
            }
        ));
        assert!(!refresh_recents_after_persistence(
            &AppEvent::PersistenceFlushed {
                request_id: 7,
                trigger: PersistenceTrigger::SessionClose,
                outcome: PersistenceOutcome::Completed,
            }
        ));
    }
}
