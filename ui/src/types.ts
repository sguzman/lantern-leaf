export type UiMode = "starter" | "reader";

export interface PanelState {
  show_settings: boolean;
  show_stats: boolean;
  show_tts: boolean;
}

export type ThemeMode = "day" | "night";

export interface HighlightColor {
  r: number;
  g: number;
  b: number;
  a: number;
}

export interface BootstrapConfig {
  theme: ThemeMode;
  font_family: string;
  font_weight: string;
  day_highlight: HighlightColor;
  night_highlight: HighlightColor;
  default_font_size: number;
  default_lines_per_page: number;
  default_tts_speed: number;
  default_pause_after_sentence: number;
  key_toggle_play_pause: string;
  key_next_sentence: string;
  key_prev_sentence: string;
  key_repeat_sentence: string;
  key_toggle_search: string;
  key_safe_quit: string;
  key_toggle_settings: string;
  key_toggle_stats: string;
  key_toggle_tts: string;
}

export interface BootstrapState {
  app_name: string;
  mode: string;
  config: BootstrapConfig;
}

export interface SessionState {
  mode: UiMode;
  active_source_path: string | null;
  open_in_flight: boolean;
  panels: PanelState;
}

export interface ReaderSettingsView {
  theme: ThemeMode;
  day_highlight: HighlightColor;
  night_highlight: HighlightColor;
  font_size: number;
  line_spacing: number;
  margin_horizontal: number;
  margin_vertical: number;
  lines_per_page: number;
  pause_after_sentence: number;
  auto_scroll_tts: boolean;
  center_spoken_sentence: boolean;
  tts_speed: number;
  tts_volume: number;
}

export interface ReaderStats {
  page_index: number;
  total_pages: number;
  tts_progress_pct: number;
  page_time_remaining_secs: number;
  book_time_remaining_secs: number;
  page_word_count: number;
  page_sentence_count: number;
  page_start_percent: number;
  page_end_percent: number;
  words_read_up_to_page_start: number;
  sentences_read_up_to_page_start: number;
  words_read_up_to_page_end: number;
  sentences_read_up_to_page_end: number;
  words_read_up_to_current_position: number;
  sentences_read_up_to_current_position: number;
}

export type TtsPlaybackState = "idle" | "playing" | "paused";

export interface ReaderTtsView {
  state: TtsPlaybackState;
  current_sentence_idx: number | null;
  sentence_count: number;
  can_seek_prev: boolean;
  can_seek_next: boolean;
  progress_pct: number;
}

export interface ReaderSnapshot {
  source_path: string;
  source_name: string;
  current_page: number;
  total_pages: number;
  text_only_mode: boolean;
  page_text: string;
  sentences: string[];
  highlighted_sentence_idx: number | null;
  search_query: string;
  search_matches: number[];
  selected_search_match: number | null;
  settings: ReaderSettingsView;
  tts: ReaderTtsView;
  stats: ReaderStats;
  panels: PanelState;
}

export interface OpenSourceResult {
  session: SessionState;
  reader: ReaderSnapshot;
}

export interface RecentBook {
  source_path: string;
  display_title: string;
  thumbnail_path: string | null;
  last_opened_unix_secs: number;
}

export interface CalibreBook {
  id: number;
  title: string;
  extension: string;
  authors: string;
  year: number | null;
  file_size_bytes: number | null;
  source_path: string | null;
  cover_thumbnail: string | null;
}

export interface BridgeError {
  code: string;
  message: string;
}

export interface SourceOpenEvent {
  request_id: number;
  phase: string;
  source_path: string | null;
  message: string | null;
}

export interface CalibreLoadEvent {
  request_id: number;
  phase: string;
  count: number | null;
  message: string | null;
}

export interface SessionStateEvent {
  request_id: number;
  action: string;
  session: SessionState;
}

export interface ReaderStateEvent {
  request_id: number;
  action: string;
  reader: ReaderSnapshot;
}

export interface ReaderSettingsPatch {
  font_size?: number;
  line_spacing?: number;
  margin_horizontal?: number;
  margin_vertical?: number;
  lines_per_page?: number;
  pause_after_sentence?: number;
  auto_scroll_tts?: boolean;
  center_spoken_sentence?: boolean;
  tts_speed?: number;
  tts_volume?: number;
}
