export interface BootstrapState {
  app_name: string;
  mode: string;
  config: BootstrapConfig;
}

export type UiMode = "starter" | "reader";

export interface BootstrapConfig {
  default_font_size: number;
  default_lines_per_page: number;
  default_tts_speed: number;
  default_pause_after_sentence: number;
}

export interface SessionState {
  mode: UiMode;
  active_source_path: string | null;
  open_in_flight: boolean;
}

export interface RecentBook {
  source_path: string;
  display_title: string;
  thumbnail_path: string | null;
  last_opened_unix_secs: number;
}

export interface BridgeError {
  code: string;
  message: string;
}

export interface SourceOpenEvent {
  phase: string;
  source_path: string | null;
  message: string | null;
}
