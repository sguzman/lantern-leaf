# Migration Baseline Log Scenarios

These are the required baseline logging scenarios and expected signals.

## Scenario 1: EPUB Open

Flow:

1. Launch shell.
2. Open a local `.epub` source.

Expected bridge events:

- `source-open` with `phase=started`
- `session-state` action `source_open_started`
- `reader-state` action `source_open`
- `source-open` with `phase=finished`

## Scenario 2: PDF Open

Flow:

1. Launch shell.
2. Open a local `.pdf` source.

Expected bridge events:

- `source-open` with `phase=started`
- `pdf-transcription` with `phase=started`
- `reader-state` action `source_open`
- `pdf-transcription` with `phase=finished`
- `source-open` with `phase=finished`

## Scenario 3: TTS Batch/Playback Start

Flow:

1. Open reader source.
2. Trigger TTS play.

Expected bridge events:

- `tts-state` action `reader_tts_play`
- `reader-state` action `reader_tts_play`

## Scenario 4: Close During In-Flight Work

Flow:

1. Start source open or long-running operation.
2. Trigger `close session` or `return to starter`.

Expected bridge events:

- `source-open` with `phase=cancelled` for in-flight request.
- `session-state` action `reader_close_session` or `session_return_to_starter`.
- `reader` state cleared in frontend store.

## Scenario 5: Resume From Bookmark

Flow:

1. Open source and move reader state.
2. Close session / safe quit.
3. Re-open same source.

Expected behavior signals:

- Bookmarks/config persisted by backend housekeeping.
- Reader snapshot restores to saved page/sentence context.

## Runtime Log-Level Update

Flow:

1. Change runtime log level in starter panel.

Expected bridge events:

- `log-level` event with updated level.
- `runtimeLogLevel` updated in frontend store.
- `conf/config.toml` receives updated `log_level` value.
