import ArrowBackIcon from "@mui/icons-material/ArrowBack";
import ChevronLeftIcon from "@mui/icons-material/ChevronLeft";
import ChevronRightIcon from "@mui/icons-material/ChevronRight";
import SearchIcon from "@mui/icons-material/Search";
import TuneIcon from "@mui/icons-material/Tune";
import {
  Button,
  Card,
  CardContent,
  Divider,
  Slider,
  Stack,
  Switch,
  TextField,
  Typography
} from "@mui/material";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";

import {
  computeReaderTopBarVisibility,
  computeReaderTtsControlVisibility
} from "./layoutPolicies";
import { computeReaderTypographyLayout } from "./readerTypography";
import type { ReaderSettingsPatch, ReaderSnapshot, TtsStateEvent } from "../types";

interface ReaderShellProps {
  reader: ReaderSnapshot;
  busy: boolean;
  onCloseSession: () => Promise<void>;
  onPrevPage: () => Promise<void>;
  onNextPage: () => Promise<void>;
  onPrevSentence: () => Promise<void>;
  onNextSentence: () => Promise<void>;
  onSetPage: (page: number) => Promise<void>;
  onSentenceClick: (sentenceIdx: number) => Promise<void>;
  onToggleTextOnly: () => Promise<void>;
  onSearchQuery: (query: string) => Promise<void>;
  onSearchNext: () => Promise<void>;
  onSearchPrev: () => Promise<void>;
  onToggleSettingsPanel: () => Promise<void>;
  onToggleStatsPanel: () => Promise<void>;
  onToggleTtsPanel: () => Promise<void>;
  onTtsPlay: () => Promise<void>;
  onTtsPause: () => Promise<void>;
  onTtsTogglePlayPause: () => Promise<void>;
  onTtsPlayFromPageStart: () => Promise<void>;
  onTtsPlayFromHighlight: () => Promise<void>;
  onTtsSeekNext: () => Promise<void>;
  onTtsSeekPrev: () => Promise<void>;
  onTtsRepeatSentence: () => Promise<void>;
  onApplySettings: (patch: ReaderSettingsPatch) => Promise<void>;
  ttsStateEvent: TtsStateEvent | null;
}

interface NumericSettingControlProps {
  label: string;
  value: number;
  min: number;
  max: number;
  step: number;
  decimals?: number;
  onCommit: (value: number) => Promise<void>;
}

function formatSeconds(seconds: number): string {
  const rounded = Math.max(0, Math.round(seconds));
  const mins = Math.floor(rounded / 60);
  const secs = rounded % 60;
  return `${mins}m ${secs}s`;
}

function formatPercent(value: number): string {
  return `${value.toFixed(3)}%`;
}

function clamp(value: number, min: number, max: number): number {
  return Math.min(max, Math.max(min, value));
}

function scrollSentenceIntoView(
  container: HTMLElement,
  sentence: HTMLElement,
  center: boolean,
  behavior: ScrollBehavior
): void {
  const containerRect = container.getBoundingClientRect();
  const sentenceRect = sentence.getBoundingClientRect();
  const currentTop = container.scrollTop;
  const sentenceTop = sentenceRect.top - containerRect.top + currentTop;
  const sentenceBottom = sentenceTop + sentenceRect.height;
  const viewportTop = currentTop;
  const viewportBottom = viewportTop + container.clientHeight;
  const maxTop = Math.max(0, container.scrollHeight - container.clientHeight);
  const padding = 16;

  let targetTop: number;
  if (center) {
    targetTop = sentenceTop - (container.clientHeight - sentenceRect.height) / 2;
  } else if (sentenceTop < viewportTop + padding) {
    targetTop = sentenceTop - padding;
  } else if (sentenceBottom > viewportBottom - padding) {
    targetTop = sentenceBottom - container.clientHeight + padding;
  } else {
    return;
  }

  const clampedTop = clamp(targetTop, 0, maxTop);
  if (Math.abs(clampedTop - currentTop) < 1) {
    return;
  }
  container.scrollTo({ top: clampedTop, behavior });
}

function NumericSettingControl({
  label,
  value,
  min,
  max,
  step,
  decimals = 2,
  onCommit
}: NumericSettingControlProps) {
  const [inputValue, setInputValue] = useState(value.toFixed(decimals));
  const [invalid, setInvalid] = useState(false);
  const inputRef = useRef<HTMLInputElement | null>(null);

  useEffect(() => {
    setInputValue(value.toFixed(decimals));
    setInvalid(false);
  }, [decimals, value]);

  const parseValue = (raw: string): number | null => {
    const parsed = Number(raw);
    if (!Number.isFinite(parsed)) {
      return null;
    }
    if (parsed < min || parsed > max) {
      return null;
    }
    return parsed;
  };

  const commit = async (raw: string): Promise<void> => {
    const parsed = parseValue(raw);
    if (parsed === null) {
      setInvalid(true);
      return;
    }
    setInvalid(false);
    await onCommit(parsed);
  };

  const sliderValue = Math.min(max, Math.max(min, value));

  return (
    <Stack spacing={0.75}>
      <Typography variant="caption" fontWeight={700}>
        {label}
      </Typography>
      <Stack direction="row" spacing={1.25} alignItems="center">
        <Slider
          value={sliderValue}
          min={min}
          max={max}
          step={step}
          onChange={(_, nextValue) => {
            if (typeof nextValue === "number") {
              void onCommit(nextValue);
            }
          }}
        />
        <TextField
          inputRef={inputRef}
          size="small"
          value={inputValue}
          error={invalid}
          onChange={(event) => {
            setInputValue(event.target.value);
            setInvalid(parseValue(event.target.value) === null);
          }}
          onBlur={() => void commit(inputValue)}
          onKeyDown={(event) => {
            if (event.key === "Enter") {
              event.preventDefault();
              void commit(inputValue);
            }
            if (event.key === "Escape") {
              event.preventDefault();
              setInputValue(value.toFixed(decimals));
              setInvalid(false);
            }
          }}
          onWheel={(event) => {
            if (document.activeElement !== inputRef.current) {
              return;
            }
            event.preventDefault();
            const delta = event.deltaY < 0 ? step : -step;
            const next = Math.min(max, Math.max(min, value + delta));
            void onCommit(next);
          }}
          inputProps={{
            inputMode: "decimal"
          }}
          sx={{
            width: 92,
            "& .MuiInputBase-input": {
              color: invalid ? "error.main" : undefined
            }
          }}
        />
      </Stack>
    </Stack>
  );
}

export function ReaderShell({
  reader,
  busy,
  onCloseSession,
  onPrevPage,
  onNextPage,
  onPrevSentence,
  onNextSentence,
  onSetPage,
  onSentenceClick,
  onToggleTextOnly,
  onSearchQuery,
  onSearchNext,
  onSearchPrev,
  onToggleSettingsPanel,
  onToggleStatsPanel,
  onToggleTtsPanel,
  onTtsPlay,
  onTtsPause,
  onTtsTogglePlayPause,
  onTtsPlayFromPageStart,
  onTtsPlayFromHighlight,
  onTtsSeekNext,
  onTtsSeekPrev,
  onTtsRepeatSentence,
  onApplySettings,
  ttsStateEvent
}: ReaderShellProps) {
  const [pageInput, setPageInput] = useState(String(reader.current_page + 1));
  const [searchInput, setSearchInput] = useState(reader.search_query);
  const sentenceRefs = useRef<Record<number, HTMLButtonElement | null>>({});
  const sentenceScrollRef = useRef<HTMLDivElement | null>(null);
  const topBarRef = useRef<HTMLDivElement | null>(null);
  const ttsControlRowRef = useRef<HTMLDivElement | null>(null);
  const [topBarWidth, setTopBarWidth] = useState(0);
  const [ttsControlRowWidth, setTtsControlRowWidth] = useState(0);

  useEffect(() => {
    const node = topBarRef.current;
    if (!node) {
      return;
    }

    const resizeObserver = new ResizeObserver((entries) => {
      const entry = entries[0];
      if (!entry) {
        return;
      }
      setTopBarWidth(entry.contentRect.width);
    });
    resizeObserver.observe(node);
    setTopBarWidth(node.getBoundingClientRect().width);

    return () => resizeObserver.disconnect();
  }, []);

  useEffect(() => {
    const node = ttsControlRowRef.current;
    if (!node) {
      return;
    }
    const resizeObserver = new ResizeObserver((entries) => {
      const entry = entries[0];
      if (!entry) {
        return;
      }
      setTtsControlRowWidth(entry.contentRect.width);
    });
    resizeObserver.observe(node);
    setTtsControlRowWidth(node.getBoundingClientRect().width);
    return () => resizeObserver.disconnect();
  }, [reader.panels.show_tts]);

  useEffect(() => {
    setPageInput(String(reader.current_page + 1));
  }, [reader.current_page]);

  useEffect(() => {
    setSearchInput(reader.search_query);
  }, [reader.search_query]);

  const alignHighlightedSentence = useCallback(
    (behavior: ScrollBehavior) => {
      const idx = reader.highlighted_sentence_idx;
      if (idx === null || idx === undefined) {
        return;
      }
      if (!reader.settings.auto_scroll_tts) {
        return;
      }
      const container = sentenceScrollRef.current;
      const sentence = sentenceRefs.current[idx];
      if (!container || !sentence) {
        return;
      }
      scrollSentenceIntoView(
        container,
        sentence,
        reader.settings.center_spoken_sentence,
        behavior
      );
    },
    [
      reader.highlighted_sentence_idx,
      reader.settings.auto_scroll_tts,
      reader.settings.center_spoken_sentence
    ]
  );

  useEffect(() => {
    const idx = reader.highlighted_sentence_idx;
    if (idx === null || idx === undefined) {
      return;
    }
    if (!reader.settings.auto_scroll_tts) {
      return;
    }
    const frame = requestAnimationFrame(() => {
      alignHighlightedSentence("smooth");
    });
    return () => cancelAnimationFrame(frame);
  }, [
    alignHighlightedSentence,
    reader.current_page
  ]);

  useEffect(() => {
    if (!reader.settings.auto_scroll_tts) {
      return;
    }
    const container = sentenceScrollRef.current;
    if (!container) {
      return;
    }

    const realign = () => {
      alignHighlightedSentence("auto");
    };
    const resizeObserver = new ResizeObserver(() => {
      requestAnimationFrame(realign);
    });
    resizeObserver.observe(container);
    window.addEventListener("resize", realign);

    return () => {
      resizeObserver.disconnect();
      window.removeEventListener("resize", realign);
    };
  }, [
    alignHighlightedSentence,
    reader.sentences.length,
    reader.settings.auto_scroll_tts,
    reader.settings.font_size,
    reader.settings.line_spacing,
    reader.settings.margin_horizontal
  ]);

  const panelTitle = useMemo(() => {
    if (reader.panels.show_settings) {
      return "Settings";
    }
    if (reader.panels.show_stats) {
      return "Stats";
    }
    if (reader.panels.show_tts) {
      return "TTS Options";
    }
    return null;
  }, [reader.panels.show_settings, reader.panels.show_stats, reader.panels.show_tts]);

  const topBarVisibility = useMemo(
    () => computeReaderTopBarVisibility(topBarWidth),
    [topBarWidth]
  );
  const ttsControlVisibility = useMemo(
    () => computeReaderTtsControlVisibility(ttsControlRowWidth),
    [ttsControlRowWidth]
  );
  const readerTypography = useMemo(
    () => computeReaderTypographyLayout(reader.settings),
    [reader.settings]
  );
  const playbackLabel = reader.tts.state === "playing" ? "Pause" : "Play";
  const hasHighlightSentence = reader.tts.current_sentence_idx !== null;

  return (
    <Card className="w-full max-w-[1700px] rounded-3xl border border-slate-200 shadow-sm">
      <CardContent className="p-4 md:p-6">
        <Stack spacing={2}>
          <Stack
            ref={topBarRef}
            direction="row"
            alignItems="center"
            spacing={1}
            sx={{
              flexWrap: "nowrap",
              overflow: "hidden",
              whiteSpace: "nowrap",
              minHeight: 44
            }}
          >
            <Button
              variant="outlined"
              startIcon={<ArrowBackIcon />}
              onClick={() => void onCloseSession()}
              disabled={busy}
              data-testid="reader-close-session-button"
              sx={{ flexShrink: 0 }}
            >
              Close Session
            </Button>
            <Divider flexItem orientation="vertical" />
            <Button
              variant="outlined"
              startIcon={<ChevronLeftIcon />}
              onClick={() => void onPrevPage()}
              disabled={busy || reader.current_page === 0}
              sx={{ flexShrink: 0 }}
            >
              Prev Page
            </Button>
            <Button
              variant="outlined"
              endIcon={<ChevronRightIcon />}
              onClick={() => void onNextPage()}
              disabled={busy || reader.current_page + 1 >= reader.total_pages}
              sx={{ flexShrink: 0 }}
            >
              Next Page
            </Button>
            {topBarVisibility.showSentenceButtons ? (
              <>
                <Button
                  variant="outlined"
                  onClick={() => void onPrevSentence()}
                  disabled={busy}
                  sx={{ flexShrink: 0 }}
                >
                  Prev Sentence
                </Button>
                <Button
                  variant="outlined"
                  onClick={() => void onNextSentence()}
                  disabled={busy}
                  sx={{ flexShrink: 0 }}
                >
                  Next Sentence
                </Button>
              </>
            ) : null}
            <TextField
              size="small"
              value={pageInput}
              onChange={(event) => setPageInput(event.target.value)}
              onKeyDown={(event) => {
                if (event.key === "Enter") {
                  const parsed = Number(pageInput);
                  if (Number.isFinite(parsed)) {
                    const page = Math.max(1, Math.min(reader.total_pages, Math.floor(parsed)));
                    void onSetPage(page - 1);
                  }
                }
              }}
              sx={{ width: 90, flexShrink: 0 }}
              label="Page"
            />
            {topBarVisibility.showTextModeButton ? (
              <Button
                variant={reader.text_only_mode ? "contained" : "outlined"}
                onClick={() => void onToggleTextOnly()}
                disabled={busy}
                sx={{ flexShrink: 0 }}
              >
                {reader.text_only_mode ? "Pretty Text" : "Text-only"}
              </Button>
            ) : null}
            {topBarVisibility.showSettingsButton ? (
              <Button
                variant={reader.panels.show_settings ? "contained" : "outlined"}
                startIcon={<TuneIcon />}
                onClick={() => void onToggleSettingsPanel()}
                disabled={busy}
                sx={{ flexShrink: 0 }}
              >
                Settings
              </Button>
            ) : null}
            {topBarVisibility.showStatsButton ? (
              <Button
                variant={reader.panels.show_stats ? "contained" : "outlined"}
                onClick={() => void onToggleStatsPanel()}
                disabled={busy}
                sx={{ flexShrink: 0 }}
              >
                Show Stats
              </Button>
            ) : null}
            {topBarVisibility.showTtsButton ? (
              <Button
                variant={reader.panels.show_tts ? "contained" : "outlined"}
                onClick={() => void onToggleTtsPanel()}
                disabled={busy}
                sx={{ flexShrink: 0 }}
              >
                TTS Panel
              </Button>
            ) : null}
          </Stack>

          <Stack direction="row" spacing={1} alignItems="center">
            <SearchIcon fontSize="small" />
            <TextField
              size="small"
              fullWidth
              label="Search (regex supported)"
              value={searchInput}
              inputProps={{ "data-reader-search-input": "1" }}
              onChange={(event) => setSearchInput(event.target.value)}
              onKeyDown={(event) => {
                if (event.key === "Enter") {
                  void onSearchQuery(searchInput);
                }
              }}
            />
            <Button variant="outlined" onClick={() => void onSearchQuery(searchInput)}>
              Apply
            </Button>
            <Button variant="outlined" onClick={() => void onSearchPrev()}>
              Prev
            </Button>
            <Button variant="outlined" onClick={() => void onSearchNext()}>
              Next
            </Button>
          </Stack>

          <Stack direction={{ xs: "column", lg: "row" }} spacing={2}>
            <div className="min-h-[62vh] flex-1 overflow-hidden rounded-2xl border border-slate-200">
              <div
                ref={sentenceScrollRef}
                className="h-full overflow-y-auto"
                style={{
                  paddingLeft: `${readerTypography.horizontalMarginPx}px`,
                  paddingRight: `${readerTypography.horizontalMarginPx}px`,
                  paddingTop: `${readerTypography.verticalMarginPx}px`,
                  paddingBottom: `${readerTypography.verticalMarginPx}px`
                }}
              >
                <Stack spacing={0.75}>
                  {reader.sentences.map((sentence, idx) => {
                    const highlighted = reader.highlighted_sentence_idx === idx;
                    const searchMatch = reader.search_matches.includes(idx);
                    return (
                      <button
                        key={`${reader.current_page}:${idx}`}
                        ref={(element) => {
                          sentenceRefs.current[idx] = element;
                        }}
                        type="button"
                        onClick={() => void onSentenceClick(idx)}
                        className="w-full rounded-lg border px-3 py-1.5 text-left transition-colors"
                        style={{
                          fontSize: `${readerTypography.fontSizePx}px`,
                          lineHeight: readerTypography.lineSpacing,
                          borderColor: highlighted
                            ? "var(--reader-highlight-border)"
                            : searchMatch
                              ? "var(--reader-search-border)"
                              : "transparent",
                          background: highlighted
                            ? "var(--reader-highlight-bg)"
                            : searchMatch
                              ? "var(--reader-search-bg)"
                              : "transparent"
                        }}
                      >
                        {sentence}
                      </button>
                    );
                  })}
                </Stack>
              </div>
            </div>

            {panelTitle ? (
              <div className="w-full shrink-0 rounded-2xl border border-slate-200 p-3 lg:w-[360px]">
                <Stack spacing={1.25}>
                  <Typography variant="subtitle1" fontWeight={700}>
                    {panelTitle}
                  </Typography>
                  <Divider />

                  {reader.panels.show_settings ? (
                    <Stack spacing={1.5}>
                      <NumericSettingControl
                        label="Font Size"
                        value={reader.settings.font_size}
                        min={12}
                        max={36}
                        step={1}
                        decimals={0}
                        onCommit={async (next) => {
                          await onApplySettings({ font_size: Math.round(next) });
                        }}
                      />
                      <NumericSettingControl
                        label="Lines Per Page"
                        value={reader.settings.lines_per_page}
                        min={8}
                        max={1000}
                        step={1}
                        decimals={0}
                        onCommit={async (next) => {
                          await onApplySettings({ lines_per_page: Math.round(next) });
                        }}
                      />
                      <NumericSettingControl
                        label="Horizontal Margin"
                        value={reader.settings.margin_horizontal}
                        min={0}
                        max={600}
                        step={1}
                        decimals={0}
                        onCommit={async (next) => {
                          await onApplySettings({ margin_horizontal: Math.round(next) });
                        }}
                      />
                      <NumericSettingControl
                        label="Line Spacing"
                        value={reader.settings.line_spacing}
                        min={0.8}
                        max={3}
                        step={0.05}
                        decimals={2}
                        onCommit={async (next) => {
                          await onApplySettings({ line_spacing: next });
                        }}
                      />
                      <Stack direction="row" alignItems="center" justifyContent="space-between">
                        <Typography variant="caption" fontWeight={700}>
                          Auto Scroll
                        </Typography>
                        <Switch
                          checked={reader.settings.auto_scroll_tts}
                          onChange={(event) =>
                            void onApplySettings({ auto_scroll_tts: event.target.checked })
                          }
                        />
                      </Stack>
                      <Stack direction="row" alignItems="center" justifyContent="space-between">
                        <Typography variant="caption" fontWeight={700}>
                          Auto Center
                        </Typography>
                        <Switch
                          checked={reader.settings.center_spoken_sentence}
                          onChange={(event) =>
                            void onApplySettings({
                              center_spoken_sentence: event.target.checked
                            })
                          }
                        />
                      </Stack>
                    </Stack>
                  ) : null}

                  {reader.panels.show_stats ? (
                    <Stack spacing={0.8}>
                      <Typography variant="body2">
                        Page index: {reader.stats.page_index} / {reader.stats.total_pages}
                      </Typography>
                      <Typography variant="body2">
                        TTS progress: {reader.stats.tts_progress_pct.toFixed(3)}%
                      </Typography>
                      <Typography variant="body2">
                        Page time remaining: {formatSeconds(reader.stats.page_time_remaining_secs)}
                      </Typography>
                      <Typography variant="body2">
                        Book time remaining: {formatSeconds(reader.stats.book_time_remaining_secs)}
                      </Typography>
                      <Divider />
                      <Typography variant="body2">
                        Words on page: {reader.stats.page_word_count}
                      </Typography>
                      <Typography variant="body2">
                        Sentences on page: {reader.stats.page_sentence_count}
                      </Typography>
                      <Typography variant="body2">
                        Percent at start of page: {formatPercent(reader.stats.page_start_percent)}
                      </Typography>
                      <Typography variant="body2">
                        Percent at end of page: {formatPercent(reader.stats.page_end_percent)}
                      </Typography>
                      <Typography variant="body2">
                        Words read to page start: {reader.stats.words_read_up_to_page_start}
                      </Typography>
                      <Typography variant="body2">
                        Sentences read to page start: {reader.stats.sentences_read_up_to_page_start}
                      </Typography>
                      <Typography variant="body2">
                        Words read to current position:{" "}
                        {reader.stats.words_read_up_to_current_position}
                      </Typography>
                      <Typography variant="body2">
                        Sentences read to current position:{" "}
                        {reader.stats.sentences_read_up_to_current_position}
                      </Typography>
                    </Stack>
                  ) : null}

                  {reader.panels.show_tts ? (
                    <Stack spacing={1.5}>
                      <Typography variant="caption" fontWeight={700}>
                        State: {reader.tts.state} | Sentence:{" "}
                        {reader.tts.current_sentence_idx !== null
                          ? `${reader.tts.current_sentence_idx + 1}/${Math.max(1, reader.tts.sentence_count)}`
                          : `0/${Math.max(1, reader.tts.sentence_count)}`}
                      </Typography>
                      <Typography variant="caption" color="text.secondary">
                        Progress: {reader.tts.progress_pct.toFixed(3)}%
                      </Typography>
                      {ttsStateEvent ? (
                        <Typography variant="caption" color="text.secondary">
                          Last TTS event #{ttsStateEvent.request_id}: {ttsStateEvent.action}
                        </Typography>
                      ) : null}
                      <Stack
                        ref={ttsControlRowRef}
                        direction="row"
                        spacing={1}
                        sx={{
                          flexWrap: "nowrap",
                          overflow: "hidden",
                          whiteSpace: "nowrap",
                          minHeight: 36
                        }}
                      >
                        <Button
                          variant="contained"
                          size="small"
                          onClick={() => void onTtsTogglePlayPause()}
                          data-testid="reader-tts-toggle-button"
                        >
                          {playbackLabel}
                        </Button>
                        {ttsControlVisibility.showPlayButton ? (
                          <Button variant="outlined" size="small" onClick={() => void onTtsPlay()}>
                            Play
                          </Button>
                        ) : null}
                        {ttsControlVisibility.showPauseButton ? (
                          <Button variant="outlined" size="small" onClick={() => void onTtsPause()}>
                            Pause
                          </Button>
                        ) : null}
                        {ttsControlVisibility.showPlayPageButton ? (
                          <Button
                            variant="outlined"
                            size="small"
                            onClick={() => void onTtsPlayFromPageStart()}
                            disabled={reader.tts.sentence_count === 0}
                          >
                            Play Page
                          </Button>
                        ) : null}
                        {ttsControlVisibility.showPlayHighlightButton ? (
                          <Button
                            variant="outlined"
                            size="small"
                            onClick={() => void onTtsPlayFromHighlight()}
                            disabled={!hasHighlightSentence}
                          >
                            Play Highlight
                          </Button>
                        ) : null}
                        {ttsControlVisibility.showPrevSentenceButton ? (
                          <Button
                            variant="outlined"
                            size="small"
                            onClick={() => void onTtsSeekPrev()}
                            disabled={!reader.tts.can_seek_prev}
                          >
                            Prev Sentence
                          </Button>
                        ) : null}
                        {ttsControlVisibility.showNextSentenceButton ? (
                          <Button
                            variant="outlined"
                            size="small"
                            onClick={() => void onTtsSeekNext()}
                            disabled={!reader.tts.can_seek_next}
                          >
                            Next Sentence
                          </Button>
                        ) : null}
                        {ttsControlVisibility.showRepeatButton ? (
                          <Button
                            variant="outlined"
                            size="small"
                            onClick={() => void onTtsRepeatSentence()}
                            disabled={!hasHighlightSentence}
                          >
                            Repeat
                          </Button>
                        ) : null}
                      </Stack>
                      <Divider />
                      <NumericSettingControl
                        label="Playback Speed"
                        value={reader.settings.tts_speed}
                        min={0.25}
                        max={4}
                        step={0.05}
                        decimals={2}
                        onCommit={async (next) => {
                          await onApplySettings({ tts_speed: next });
                        }}
                      />
                      <NumericSettingControl
                        label="Volume"
                        value={reader.settings.tts_volume}
                        min={0}
                        max={2}
                        step={0.05}
                        decimals={2}
                        onCommit={async (next) => {
                          await onApplySettings({ tts_volume: next });
                        }}
                      />
                      <NumericSettingControl
                        label="Pause After Sentence"
                        value={reader.settings.pause_after_sentence}
                        min={0}
                        max={3}
                        step={0.01}
                        decimals={2}
                        onCommit={async (next) => {
                          await onApplySettings({ pause_after_sentence: next });
                        }}
                      />
                    </Stack>
                  ) : null}
                </Stack>
              </div>
            ) : null}
          </Stack>
        </Stack>
      </CardContent>
    </Card>
  );
}
