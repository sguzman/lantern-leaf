# Tauri Command Groups

This groups the current bridge surface into stable domains so frontend and backend changes stay organized.

## Session

- `session_get_bootstrap`
- `session_get_state`
- `session_return_to_starter`
- `app_safe_quit`

## Panels

- `panel_toggle_settings`
- `panel_toggle_stats`
- `panel_toggle_tts`

## Source Open

- `source_open_path`
- `source_open_clipboard_text`
- events:
  - `source-open`
  - `session-state`
  - `reader-state`

## Recent

- `recent_list`
- `recent_delete`

## Reader Navigation And Search

- `reader_get_snapshot`
- `reader_next_page`
- `reader_prev_page`
- `reader_set_page`
- `reader_sentence_click`
- `reader_next_sentence`
- `reader_prev_sentence`
- `reader_toggle_text_only`
- `reader_search_set_query`
- `reader_search_next`
- `reader_search_prev`

## Reader Settings

- `reader_apply_settings`

## Reader TTS

- `reader_tts_play`
- `reader_tts_pause`
- `reader_tts_toggle_play_pause`
- `reader_tts_play_from_page_start`
- `reader_tts_play_from_highlight`
- `reader_tts_seek_next`
- `reader_tts_seek_prev`
- `reader_tts_repeat_sentence`
- `reader_close_session`

## Calibre

- `calibre_load_books`
- `calibre_open_book`
- event:
  - `calibre-load`
