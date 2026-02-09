use super::super::messages::Message;
use super::super::state::{App, TEXT_SCROLL_ID};
use super::Effect;
use iced::Event;
use iced::event;
use iced::keyboard::{self, Key, Modifiers, key};
use iced::time;
use iced::window;
use iced::{Subscription, Task};
use std::time::Duration;

impl App {
    pub fn subscription(app: &App) -> Subscription<Message> {
        let mut subscriptions: Vec<Subscription<Message>> = vec![
            iced::window::resize_events().map(|(_id, size)| Message::WindowResized {
                width: size.width,
                height: size.height,
            }),
            event::listen_with(runtime_event_to_message),
        ];

        if app.tts.running {
            subscriptions.push(time::every(Duration::from_millis(50)).map(Message::Tick));
        }

        Subscription::batch(subscriptions)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let effects = self.reduce(message);
        if effects.is_empty() {
            Task::none()
        } else {
            Task::batch(effects.into_iter().map(|effect| self.run_effect(effect)))
        }
    }

    fn reduce(&mut self, message: Message) -> Vec<Effect> {
        let mut effects = Vec::new();

        match message {
            Message::NextPage => self.handle_next_page(&mut effects),
            Message::PreviousPage => self.handle_previous_page(&mut effects),
            Message::FontSizeChanged(size) => self.handle_font_size_changed(size, &mut effects),
            Message::ToggleTheme => self.handle_toggle_theme(&mut effects),
            Message::ToggleSettings => self.handle_toggle_settings(&mut effects),
            Message::ToggleTextOnly => self.handle_toggle_text_only(&mut effects),
            Message::FontFamilyChanged(family) => {
                self.handle_font_family_changed(family, &mut effects);
            }
            Message::FontWeightChanged(weight) => {
                self.handle_font_weight_changed(weight, &mut effects);
            }
            Message::LineSpacingChanged(spacing) => {
                self.handle_line_spacing_changed(spacing, &mut effects);
            }
            Message::MarginHorizontalChanged(margin) => {
                self.handle_margin_horizontal_changed(margin, &mut effects);
            }
            Message::MarginVerticalChanged(margin) => {
                self.handle_margin_vertical_changed(margin, &mut effects);
            }
            Message::WordSpacingChanged(spacing) => {
                self.handle_word_spacing_changed(spacing, &mut effects);
            }
            Message::LetterSpacingChanged(spacing) => {
                self.handle_letter_spacing_changed(spacing, &mut effects);
            }
            Message::LinesPerPageChanged(lines) => {
                self.handle_lines_per_page_changed(lines, &mut effects);
            }
            Message::DayHighlightChanged(component, value) => {
                self.handle_day_highlight_changed(component, value, &mut effects);
            }
            Message::PauseAfterSentenceChanged(pause) => {
                self.handle_pause_after_sentence_changed(pause, &mut effects);
            }
            Message::NightHighlightChanged(component, value) => {
                self.handle_night_highlight_changed(component, value, &mut effects);
            }
            Message::AutoScrollTtsChanged(enabled) => {
                self.handle_auto_scroll_tts_changed(enabled, &mut effects);
            }
            Message::CenterSpokenSentenceChanged(centered) => {
                self.handle_center_spoken_sentence_changed(centered, &mut effects);
            }
            Message::ToggleTtsControls => self.handle_toggle_tts_controls(&mut effects),
            Message::JumpToCurrentAudio => self.handle_jump_to_current_audio(&mut effects),
            Message::TogglePlayPause => self.handle_toggle_play_pause(&mut effects),
            Message::RepeatCurrentSentence => self.handle_repeat_current_sentence(&mut effects),
            Message::SafeQuit => effects.push(Effect::QuitSafely),
            Message::Play => self.handle_play(&mut effects),
            Message::PlayFromPageStart => self.handle_play_from_page_start(&mut effects),
            Message::PlayFromCursor(idx) => self.handle_play_from_cursor(idx, &mut effects),
            Message::Pause => self.handle_pause(&mut effects),
            Message::SetTtsSpeed(speed) => self.handle_set_tts_speed(speed, &mut effects),
            Message::SetTtsVolume(volume) => self.handle_set_tts_volume(volume, &mut effects),
            Message::SeekForward => self.handle_seek_forward(&mut effects),
            Message::SeekBackward => self.handle_seek_backward(&mut effects),
            Message::SentenceClicked(idx) => self.handle_sentence_clicked(idx, &mut effects),
            Message::WindowResized { width, height } => {
                self.handle_window_resized(width, height, &mut effects);
            }
            Message::WindowMoved { x, y } => {
                self.handle_window_moved(x, y, &mut effects);
            }
            Message::KeyPressed { key, modifiers } => {
                if let Some(shortcut) = self.shortcut_message_for_key(key, modifiers) {
                    effects.extend(self.reduce(shortcut));
                }
            }
            Message::Scrolled {
                offset,
                viewport_width,
                viewport_height,
                content_width,
                content_height,
            } => self.handle_scrolled(
                offset,
                viewport_width,
                viewport_height,
                content_width,
                content_height,
                &mut effects,
            ),
            Message::TtsPrepared {
                page,
                start_idx,
                request_id,
                files,
            } => self.handle_tts_prepared(page, start_idx, request_id, files, &mut effects),
            Message::Tick(now) => self.handle_tick(now, &mut effects),
        }

        if self.text_only_mode {
            self.ensure_text_only_preview_for_page(self.reader.current_page);
        }

        effects
    }

    fn run_effect(&mut self, effect: Effect) -> Task<Message> {
        match effect {
            Effect::SaveConfig => {
                self.save_epub_config();
                Task::none()
            }
            Effect::SaveBookmark => {
                self.persist_bookmark();
                Task::none()
            }
            Effect::StartTts { page, sentence_idx } => self.start_playback_from(page, sentence_idx),
            Effect::StopTts => {
                self.stop_playback();
                Task::none()
            }
            Effect::ScrollTo(offset) => {
                self.bookmark.last_scroll_offset = offset;
                iced::widget::scrollable::snap_to(TEXT_SCROLL_ID.clone(), offset)
            }
            Effect::AutoScrollToCurrent => {
                if !self.config.auto_scroll_tts {
                    return Task::none();
                }
                if let Some(idx) = self.tts.current_sentence_idx {
                    if let Some(offset) = self.scroll_offset_for_sentence(idx) {
                        self.bookmark.last_scroll_offset = offset;
                        return iced::widget::scrollable::snap_to(TEXT_SCROLL_ID.clone(), offset);
                    }
                }
                Task::none()
            }
            Effect::QuitSafely => {
                self.save_epub_config();
                self.persist_bookmark();
                self.stop_playback();
                iced::exit()
            }
        }
    }

    fn shortcut_message_for_key(&self, key: Key, modifiers: Modifiers) -> Option<Message> {
        if modifiers.control() || modifiers.alt() || modifiers.logo() {
            return None;
        }

        let pressed = match key.as_ref() {
            Key::Named(key::Named::Space) => "space".to_string(),
            Key::Character(ch) => ch.to_ascii_lowercase(),
            _ => return None,
        };

        let toggle_play_pause =
            Self::normalize_shortcut_token(&self.config.key_toggle_play_pause, "space");
        let safe_quit = Self::normalize_shortcut_token(&self.config.key_safe_quit, "q");
        let next_sentence = Self::normalize_shortcut_token(&self.config.key_next_sentence, "f");
        let prev_sentence = Self::normalize_shortcut_token(&self.config.key_prev_sentence, "s");
        let repeat_sentence = Self::normalize_shortcut_token(&self.config.key_repeat_sentence, "r");

        if pressed == toggle_play_pause {
            Some(Message::TogglePlayPause)
        } else if pressed == safe_quit {
            Some(Message::SafeQuit)
        } else if pressed == next_sentence {
            Some(Message::SeekForward)
        } else if pressed == prev_sentence {
            Some(Message::SeekBackward)
        } else if pressed == repeat_sentence {
            Some(Message::RepeatCurrentSentence)
        } else {
            None
        }
    }

    fn normalize_shortcut_token(raw: &str, fallback: &str) -> String {
        let normalized = raw.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "" => fallback.to_string(),
            "spacebar" => "space".to_string(),
            _ => normalized,
        }
    }
}

fn runtime_event_to_message(
    event: Event,
    status: event::Status,
    _window_id: window::Id,
) -> Option<Message> {
    if status == event::Status::Captured {
        return None;
    }
    match event {
        Event::Window(iced::window::Event::Moved(position)) => Some(Message::WindowMoved {
            x: position.x,
            y: position.y,
        }),
        Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
            Some(Message::KeyPressed { key, modifiers })
        }
        _ => None,
    }
}
