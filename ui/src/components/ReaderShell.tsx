import ArrowBackIcon from "@mui/icons-material/ArrowBack";
import ChevronLeftIcon from "@mui/icons-material/ChevronLeft";
import ChevronRightIcon from "@mui/icons-material/ChevronRight";
import DarkModeOutlinedIcon from "@mui/icons-material/DarkModeOutlined";
import GpsFixedIcon from "@mui/icons-material/GpsFixed";
import LightModeOutlinedIcon from "@mui/icons-material/LightModeOutlined";
import GraphicEqIcon from "@mui/icons-material/GraphicEq";
import SearchIcon from "@mui/icons-material/Search";
import SpeedDialIcon from "@mui/material/SpeedDialIcon";
import TextFieldsIcon from "@mui/icons-material/TextFields";
import TuneIcon from "@mui/icons-material/Tune";
import QueryStatsIcon from "@mui/icons-material/QueryStats";
import {
  Backdrop,
  Box,
  Button,
  Card,
  CardContent,
  Divider,
  Fab,
  FormControl,
  InputLabel,
  MenuItem,
  Paper,
  Select,
  Slider,
  Stack,
  Switch,
  TextField,
  Typography,
} from "@mui/material";
import { useCallback, useEffect, useMemo, useRef, useState, memo } from "react";

import {
  computeReaderTopBarVisibility,
  computeReaderTtsControlVisibility
} from "./layoutPolicies";
import { computeReaderTypographyLayout } from "./readerTypography";
import type {
  FontFamily,
  FontWeight,
  HighlightColor,
  ReaderSettingsPatch,
  ReaderSnapshot,
  ThemeMode,
  TtsStateEvent
} from "../types";

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
  onToggleTheme: () => Promise<void>;
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
  onTtsPrecomputePage: () => Promise<void>;
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
  testId?: string;
  onCommit: (value: number) => Promise<void>;
}

interface ReaderQuickActionsProps {
  busy: boolean;
  isNightTheme: boolean;
  isTextOnly: boolean;
  showSettings: boolean;
  showStats: boolean;
  showTts: boolean;
  onToggleTheme: () => Promise<void>;
  onToggleTextOnly: () => Promise<void>;
  onToggleSettingsPanel: () => Promise<void>;
  onToggleStatsPanel: () => Promise<void>;
  onToggleTtsPanel: () => Promise<void>;
}

const FONT_FAMILY_OPTIONS: Array<{ value: FontFamily; label: string }> = [
  { value: "lexend", label: "Lexend" },
  { value: "sans", label: "Sans" },
  { value: "serif", label: "Serif" },
  { value: "monospace", label: "Monospace" },
  { value: "fira-code", label: "Fira Code" },
  { value: "atkinson-hyperlegible", label: "Atkinson Hyperlegible" },
  { value: "atkinson-hyperlegible-next", label: "Atkinson Hyperlegible Next" },
  { value: "lexica-ultralegible", label: "Lexica Ultralegible" },
  { value: "courier", label: "Courier" },
  { value: "frank-gothic", label: "Frank Gothic" },
  { value: "hermit", label: "Hermit" },
  { value: "hasklug", label: "Hasklug" },
  { value: "noto-sans", label: "Noto Sans" }
];

const FONT_WEIGHT_OPTIONS: Array<{ value: FontWeight; label: string }> = [
  { value: "light", label: "Light" },
  { value: "normal", label: "Normal" },
  { value: "bold", label: "Bold" }
];

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

function normalizeNumber(value: number, min: number, max: number, step: number, decimals: number): number {
  const clamped = clamp(value, min, max);
  if (step <= 0) {
    return Number(clamped.toFixed(decimals));
  }
  const snapped = min + Math.round((clamped - min) / step) * step;
  return Number(clamp(snapped, min, max).toFixed(decimals));
}

function almostEqual(a: number, b: number, decimals: number): boolean {
  const threshold = Math.max(1e-8, Math.pow(10, -decimals) / 2);
  return Math.abs(a - b) <= threshold;
}

function clamp01(value: number): number {
  return Math.min(1, Math.max(0, value));
}

function toHexColor(color: HighlightColor): string {
  const r = Math.round(clamp01(color.r) * 255)
    .toString(16)
    .padStart(2, "0");
  const g = Math.round(clamp01(color.g) * 255)
    .toString(16)
    .padStart(2, "0");
  const b = Math.round(clamp01(color.b) * 255)
    .toString(16)
    .padStart(2, "0");
  return `#${r}${g}${b}`;
}

function withHexColor(current: HighlightColor, hex: string): HighlightColor {
  const normalized = hex.replace("#", "");
  if (!/^[0-9a-fA-F]{6}$/.test(normalized)) {
    return current;
  }
  const r = Number.parseInt(normalized.slice(0, 2), 16) / 255;
  const g = Number.parseInt(normalized.slice(2, 4), 16) / 255;
  const b = Number.parseInt(normalized.slice(4, 6), 16) / 255;
  return {
    r: clamp01(r),
    g: clamp01(g),
    b: clamp01(b),
    a: clamp01(current.a)
  };
}

function withAlpha(current: HighlightColor, alpha: number): HighlightColor {
  return {
    r: clamp01(current.r),
    g: clamp01(current.g),
    b: clamp01(current.b),
    a: clamp01(alpha)
  };
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
  testId,
  onCommit
}: NumericSettingControlProps) {
  const [inputValue, setInputValue] = useState(value.toFixed(decimals));
  const [sliderValue, setSliderValue] = useState(value);
  const [invalid, setInvalid] = useState(false);
  const inputRef = useRef<HTMLInputElement | null>(null);

  useEffect(() => {
    setInputValue(value.toFixed(decimals));
    setSliderValue(value);
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

  const commitNumber = async (candidate: number): Promise<void> => {
    const normalized = normalizeNumber(candidate, min, max, step, decimals);
    if (almostEqual(normalized, value, decimals)) {
      setInputValue(value.toFixed(decimals));
      setSliderValue(value);
      setInvalid(false);
      return;
    }
    setInputValue(normalized.toFixed(decimals));
    setSliderValue(normalized);
    setInvalid(false);
    await onCommit(normalized);
  };

  const commitRaw = async (raw: string): Promise<void> => {
    const parsed = parseValue(raw);
    if (parsed === null) {
      setInvalid(true);
      return;
    }
    await commitNumber(parsed);
  };

  return (
    <Stack spacing={0.75}>
      <Typography variant="caption" fontWeight={700}>
        {label}
      </Typography>
      <Stack direction="row" spacing={1.25} alignItems="center" sx={{ overflow: "visible" }}>
        <Slider
          value={sliderValue}
          min={min}
          max={max}
          step={step}
          onChange={(_, nextValue) => {
            if (typeof nextValue !== "number") {
              return;
            }
            setSliderValue(nextValue);
            setInputValue(nextValue.toFixed(decimals));
            setInvalid(false);
          }}
          onChangeCommitted={(_, nextValue) => {
            if (typeof nextValue !== "number") {
              return;
            }
            void commitNumber(nextValue);
          }}
          sx={{
            flex: 1,
            minWidth: 0,
            px: 1,
            overflow: "visible",
            "& .MuiSlider-thumb": {
              boxShadow: "none"
            }
          }}
        />
        <TextField
          inputRef={inputRef}
          size="small"
          value={inputValue}
          error={invalid}
          onChange={(event) => {
            const raw = event.target.value;
            setInputValue(raw);
            const parsed = parseValue(raw);
            setInvalid(parsed === null);
            if (parsed !== null) {
              setSliderValue(parsed);
            }
          }}
          onBlur={() => void commitRaw(inputValue)}
          onKeyDown={(event) => {
            if (event.key === "Enter") {
              event.preventDefault();
              void commitRaw(inputValue);
            }
            if (event.key === "Escape") {
              event.preventDefault();
              setInputValue(value.toFixed(decimals));
              setSliderValue(value);
              setInvalid(false);
            }
          }}
          onWheel={(event) => {
            if (document.activeElement !== inputRef.current) {
              return;
            }
            event.preventDefault();
            const base = parseValue(inputValue) ?? value;
            const delta = event.deltaY < 0 ? step : -step;
            void commitNumber(base + delta);
          }}
          inputProps={{
            inputMode: "decimal",
            ...(testId ? { "data-testid": `${testId}-input` } : {})
          }}
          sx={{
            width: 98,
            "& .MuiInputBase-input": {
              color: invalid ? "error.main" : undefined
            }
          }}
        />
      </Stack>
    </Stack>
  );
}

const ReaderQuickActions = memo(function ReaderQuickActions({
  busy,
  isNightTheme,
  isTextOnly,
  showSettings,
  showStats,
  showTts,
  onToggleTheme,
  onToggleTextOnly,
  onToggleSettingsPanel,
  onToggleStatsPanel,
  onToggleTtsPanel
}: ReaderQuickActionsProps) {
  const [open, setOpen] = useState(false);

  const actions = useMemo(
    () => [
      {
        key: "theme",
        label: "Day/Night",
        icon: isNightTheme ? <LightModeOutlinedIcon /> : <DarkModeOutlinedIcon />,
        active: false,
        onClick: onToggleTheme
      },
      {
        key: "text",
        label: "Text-only",
        icon: <TextFieldsIcon />,
        active: isTextOnly,
        onClick: onToggleTextOnly
      },
      {
        key: "settings",
        label: "Settings",
        icon: <TuneIcon />,
        active: showSettings,
        onClick: onToggleSettingsPanel
      },
      {
        key: "stats",
        label: "Stats",
        icon: <QueryStatsIcon />,
        active: showStats,
        onClick: onToggleStatsPanel
      },
      {
        key: "tts",
        label: "TTS Controls",
        icon: <GraphicEqIcon />,
        active: showTts,
        onClick: onToggleTtsPanel
      }
    ],
    [
      isNightTheme,
      isTextOnly,
      onToggleSettingsPanel,
      onToggleStatsPanel,
      onToggleTextOnly,
      onToggleTheme,
      onToggleTtsPanel,
      showSettings,
      showStats,
      showTts
    ]
  );

  const close = useCallback(() => setOpen(false), []);

  return (
    <>
      <Backdrop
        open={open}
        onClick={close}
        transitionDuration={0}
        sx={{
          zIndex: (theme) => theme.zIndex.modal,
          bgcolor: "rgba(15, 23, 42, 0.44)"
        }}
      />

      <Box
        sx={{
          position: "fixed",
          top: 16,
          right: 16,
          zIndex: (theme) => theme.zIndex.modal + 1,
          pointerEvents: "none"
        }}
      >
        <Stack direction="column" spacing={1} alignItems="flex-end" sx={{ pointerEvents: "auto" }}>
          {open
            ? actions.map((action) => (
                <Stack key={action.key} direction="row" spacing={1} alignItems="center">
                  <Paper
                    elevation={3}
                    sx={{
                      px: 1.15,
                      py: 0.45,
                      bgcolor: "#ffffff",
                      color: "#0f172a",
                      border: "1px solid #cbd5e1",
                      borderRadius: 1.25
                    }}
                  >
                    <Typography variant="caption" fontWeight={700}>
                      {action.label}
                    </Typography>
                  </Paper>
                  <Fab
                    size="small"
                    color={action.active ? "primary" : "default"}
                    onClick={() => {
                      setOpen(false);
                      void action.onClick();
                    }}
                    disabled={busy}
                    data-testid={`reader-speed-dial-${action.key}`}
                  >
                    {action.icon}
                  </Fab>
                </Stack>
              ))
            : null}

          <Fab
            size="small"
            color="primary"
            onClick={(event) => {
              event.stopPropagation();
              setOpen((current) => !current);
            }}
            data-testid="reader-quick-actions-speed-dial"
          >
            <SpeedDialIcon open={open} />
          </Fab>
        </Stack>
      </Box>
    </>
  );
});

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
  onToggleTheme,
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
  onTtsPrecomputePage,
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

  const searchMatchSet = useMemo(() => new Set(reader.search_matches), [reader.search_matches]);

  const alignHighlightedSentence = useCallback(
    (behavior: ScrollBehavior, force = false) => {
      const idx = reader.highlighted_sentence_idx;
      if (idx === null || idx === undefined) {
        return;
      }
      if (!force && !reader.settings.auto_scroll_tts) {
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

  const jumpToHighlightedSentence = useCallback(() => {
    alignHighlightedSentence("smooth", true);
  }, [alignHighlightedSentence]);

  useEffect(() => {
    const idx = reader.highlighted_sentence_idx;
    if (idx === null || idx === undefined) {
      return;
    }
    if (!reader.settings.auto_scroll_tts) {
      return;
    }
    const frame = requestAnimationFrame(() => {
      requestAnimationFrame(() => {
        alignHighlightedSentence("smooth");
      });
    });
    return () => cancelAnimationFrame(frame);
  }, [
    alignHighlightedSentence,
    reader.current_page,
    reader.highlighted_sentence_idx,
    reader.settings.auto_scroll_tts,
    reader.settings.center_spoken_sentence,
    reader.settings.font_size,
    reader.settings.line_spacing,
    reader.settings.margin_horizontal,
    reader.settings.margin_vertical,
    reader.settings.word_spacing,
    reader.settings.letter_spacing
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
      requestAnimationFrame(() => {
        alignHighlightedSentence("auto");
      });
    };

    const resizeObserver = new ResizeObserver(() => {
      realign();
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
    reader.settings.margin_horizontal,
    reader.settings.margin_vertical,
    reader.settings.word_spacing,
    reader.settings.letter_spacing
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
  const hasHighlightSentence = reader.highlighted_sentence_idx !== null;
  const textModeLabel = reader.text_only_mode ? "Pretty Text" : "Text-only";

  return (
    <Card className="w-full max-w-[1700px] min-h-0 rounded-3xl border border-slate-200 shadow-sm lg:h-full">
      <CardContent className="h-full p-4 md:p-6" sx={{ position: "relative" }}>
        <Stack spacing={2} sx={{ height: "100%", minHeight: 0 }}>
          <Stack
            ref={topBarRef}
            direction="row"
            alignItems="center"
            spacing={1}
            data-testid="reader-topbar"
            sx={{
              flexWrap: "nowrap",
              overflow: "hidden",
              whiteSpace: "nowrap",
              minHeight: 44,
              paddingRight: 0.5,
              flexShrink: 0
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
              data-testid="reader-prev-page-button"
              sx={{ flexShrink: 0 }}
            >
              Prev Page
            </Button>
            <Button
              variant="outlined"
              endIcon={<ChevronRightIcon />}
              onClick={() => void onNextPage()}
              disabled={busy || reader.current_page + 1 >= reader.total_pages}
              data-testid="reader-next-page-button"
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
                  data-testid="reader-prev-sentence-button"
                  sx={{ flexShrink: 0 }}
                >
                  Prev Sentence
                </Button>
                <Button
                  variant="outlined"
                  onClick={() => void onNextSentence()}
                  disabled={busy}
                  data-testid="reader-next-sentence-button"
                  sx={{ flexShrink: 0 }}
                >
                  Next Sentence
                </Button>
              </>
            ) : null}
            {topBarVisibility.showJumpButton ? (
              <Button
                variant="outlined"
                startIcon={<GpsFixedIcon />}
                onClick={() => jumpToHighlightedSentence()}
                disabled={!hasHighlightSentence}
                data-testid="reader-jump-highlight-button"
                sx={{ flexShrink: 0 }}
              >
                Jump to Highlight
              </Button>
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
              sx={{ width: 92, flexShrink: 0 }}
              label="Page"
            />
          </Stack>

          <ReaderQuickActions
            busy={busy}
            isNightTheme={reader.settings.theme === "night"}
            isTextOnly={reader.text_only_mode}
            showSettings={reader.panels.show_settings}
            showStats={reader.panels.show_stats}
            showTts={reader.panels.show_tts}
            onToggleTheme={onToggleTheme}
            onToggleTextOnly={onToggleTextOnly}
            onToggleSettingsPanel={onToggleSettingsPanel}
            onToggleStatsPanel={onToggleStatsPanel}
            onToggleTtsPanel={onToggleTtsPanel}
          />

          <Stack direction="row" spacing={1} alignItems="center" sx={{ flexShrink: 0 }}>
            <SearchIcon fontSize="small" />
            <TextField
              size="small"
              fullWidth
              label="Search (regex supported)"
              value={searchInput}
              data-testid="reader-search-input"
              inputProps={{ "data-reader-search-input": "1" }}
              onChange={(event) => setSearchInput(event.target.value)}
              onKeyDown={(event) => {
                if (event.key === "Enter") {
                  void onSearchQuery(searchInput);
                }
              }}
            />
            <Button
              variant="outlined"
              onClick={() => void onSearchQuery(searchInput)}
              data-testid="reader-search-apply-button"
            >
              Apply
            </Button>
            <Button
              variant="outlined"
              onClick={() => void onSearchPrev()}
              data-testid="reader-search-prev-button"
            >
              Prev
            </Button>
            <Button
              variant="outlined"
              onClick={() => void onSearchNext()}
              data-testid="reader-search-next-button"
            >
              Next
            </Button>
          </Stack>

          <Stack direction={{ xs: "column", lg: "row" }} spacing={2} sx={{ flex: 1, minHeight: 0 }}>
            <div className="min-h-0 flex-1 overflow-hidden rounded-2xl border border-slate-200">
              <div
                ref={sentenceScrollRef}
                className="h-full overflow-y-auto overscroll-contain"
                data-testid="reader-sentence-scroll-container"
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
                    const searchMatch = searchMatchSet.has(idx);
                    return (
                      <button
                        key={`${reader.current_page}:${idx}`}
                        ref={(element) => {
                          sentenceRefs.current[idx] = element;
                        }}
                        type="button"
                        onClick={() => void onSentenceClick(idx)}
                        className="w-full rounded-lg border px-3 py-1.5 text-left transition-colors"
                        data-testid={`reader-sentence-${idx}`}
                        data-highlighted={highlighted ? "1" : "0"}
                        style={{
                          fontSize: `${readerTypography.fontSizePx}px`,
                          lineHeight: readerTypography.lineSpacing,
                          wordSpacing: `${readerTypography.wordSpacingPx}px`,
                          letterSpacing: `${readerTypography.letterSpacingPx}px`,
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
              <div className="w-full min-h-0 shrink-0 rounded-2xl border border-slate-200 p-3 lg:h-full lg:w-[380px]">
                <Stack spacing={1.25} sx={{ height: "100%", minHeight: 0 }}>
                  <Typography variant="subtitle1" fontWeight={700} sx={{ flexShrink: 0 }}>
                    <span data-testid="reader-panel-title">{panelTitle}</span>
                  </Typography>
                  <Divider sx={{ flexShrink: 0 }} />

                  <div
                    style={{
                      overflowY: "auto",
                      minHeight: 0,
                      overscrollBehavior: "contain",
                      paddingRight: 8,
                      scrollbarGutter: "stable"
                    }}
                  >
                    {reader.panels.show_settings ? (
                      <Stack spacing={1.5}>
                        <FormControl size="small">
                          <InputLabel id="setting-font-family-label">Font Family</InputLabel>
                          <Select
                            labelId="setting-font-family-label"
                            label="Font Family"
                            value={reader.settings.font_family}
                            onChange={(event) =>
                              void onApplySettings({
                                font_family: event.target.value as FontFamily
                              })
                            }
                            data-testid="setting-font-family"
                          >
                            {FONT_FAMILY_OPTIONS.map((option) => (
                              <MenuItem key={option.value} value={option.value}>
                                {option.label}
                              </MenuItem>
                            ))}
                          </Select>
                        </FormControl>

                        <FormControl size="small">
                          <InputLabel id="setting-font-weight-label">Font Weight</InputLabel>
                          <Select
                            labelId="setting-font-weight-label"
                            label="Font Weight"
                            value={reader.settings.font_weight}
                            onChange={(event) =>
                              void onApplySettings({
                                font_weight: event.target.value as FontWeight
                              })
                            }
                            data-testid="setting-font-weight"
                          >
                            {FONT_WEIGHT_OPTIONS.map((option) => (
                              <MenuItem key={option.value} value={option.value}>
                                {option.label}
                              </MenuItem>
                            ))}
                          </Select>
                        </FormControl>
                        <FormControl size="small">
                          <InputLabel id="setting-theme-label">Theme</InputLabel>
                          <Select
                            labelId="setting-theme-label"
                            label="Theme"
                            value={reader.settings.theme}
                            onChange={(event) =>
                              void onApplySettings({
                                theme: event.target.value as ThemeMode
                              })
                            }
                            data-testid="setting-theme"
                          >
                            <MenuItem value="day">Day</MenuItem>
                            <MenuItem value="night">Night</MenuItem>
                          </Select>
                        </FormControl>

                        <Stack spacing={1}>
                          <Typography variant="caption" fontWeight={700}>
                            Day Highlight
                          </Typography>
                          <Stack direction="row" spacing={1} alignItems="center">
                            <TextField
                              type="color"
                              size="small"
                              value={toHexColor(reader.settings.day_highlight)}
                              onChange={(event) =>
                                void onApplySettings({
                                  day_highlight: withHexColor(
                                    reader.settings.day_highlight,
                                    event.target.value
                                  )
                                })
                              }
                              inputProps={{ "data-testid": "setting-day-highlight-color" }}
                              sx={{ width: 76 }}
                            />
                            <NumericSettingControl
                              label="Day Highlight Alpha"
                              testId="setting-day-highlight-alpha"
                              value={reader.settings.day_highlight.a}
                              min={0}
                              max={1}
                              step={0.01}
                              decimals={2}
                              onCommit={async (next) => {
                                await onApplySettings({
                                  day_highlight: withAlpha(reader.settings.day_highlight, next)
                                });
                              }}
                            />
                          </Stack>
                        </Stack>

                        <Stack spacing={1}>
                          <Typography variant="caption" fontWeight={700}>
                            Night Highlight
                          </Typography>
                          <Stack direction="row" spacing={1} alignItems="center">
                            <TextField
                              type="color"
                              size="small"
                              value={toHexColor(reader.settings.night_highlight)}
                              onChange={(event) =>
                                void onApplySettings({
                                  night_highlight: withHexColor(
                                    reader.settings.night_highlight,
                                    event.target.value
                                  )
                                })
                              }
                              inputProps={{ "data-testid": "setting-night-highlight-color" }}
                              sx={{ width: 76 }}
                            />
                            <NumericSettingControl
                              label="Night Highlight Alpha"
                              testId="setting-night-highlight-alpha"
                              value={reader.settings.night_highlight.a}
                              min={0}
                              max={1}
                              step={0.01}
                              decimals={2}
                              onCommit={async (next) => {
                                await onApplySettings({
                                  night_highlight: withAlpha(reader.settings.night_highlight, next)
                                });
                              }}
                            />
                          </Stack>
                        </Stack>

                        <NumericSettingControl
                          label="Font Size"
                          testId="setting-font-size"
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
                          testId="setting-lines-per-page"
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
                          testId="setting-horizontal-margin"
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
                          label="Vertical Margin"
                          testId="setting-vertical-margin"
                          value={reader.settings.margin_vertical}
                          min={0}
                          max={240}
                          step={1}
                          decimals={0}
                          onCommit={async (next) => {
                            await onApplySettings({ margin_vertical: Math.round(next) });
                          }}
                        />
                        <NumericSettingControl
                          label="Line Spacing"
                          testId="setting-line-spacing"
                          value={reader.settings.line_spacing}
                          min={0.8}
                          max={3}
                          step={0.05}
                          decimals={2}
                          onCommit={async (next) => {
                            await onApplySettings({ line_spacing: next });
                          }}
                        />
                        <NumericSettingControl
                          label="Word Spacing"
                          testId="setting-word-spacing"
                          value={reader.settings.word_spacing}
                          min={0}
                          max={24}
                          step={1}
                          decimals={0}
                          onCommit={async (next) => {
                            await onApplySettings({ word_spacing: Math.round(next) });
                          }}
                        />
                        <NumericSettingControl
                          label="Letter Spacing"
                          testId="setting-letter-spacing"
                          value={reader.settings.letter_spacing}
                          min={0}
                          max={24}
                          step={1}
                          decimals={0}
                          onCommit={async (next) => {
                            await onApplySettings({ letter_spacing: Math.round(next) });
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
                          Words read to current position: {reader.stats.words_read_up_to_current_position}
                        </Typography>
                        <Typography variant="body2">
                          Sentences read to current position: {reader.stats.sentences_read_up_to_current_position}
                        </Typography>
                      </Stack>
                    ) : null}

                    {reader.panels.show_tts ? (
                      <Stack spacing={1.5}>
                        <Typography variant="caption" fontWeight={700}>
                          <span data-testid="reader-tts-state-summary">
                            State: {reader.tts.state} | Sentence:{" "}
                            {reader.tts.current_sentence_idx !== null
                              ? `${reader.tts.current_sentence_idx + 1}/${Math.max(1, reader.tts.sentence_count)}`
                              : `0/${Math.max(1, reader.tts.sentence_count)}`}
                          </span>
                        </Typography>
                        <Typography variant="caption" color="text.secondary">
                          <span data-testid="reader-tts-progress-label">
                            Progress: {reader.tts.progress_pct.toFixed(3)}%
                          </span>
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
                          data-testid="reader-tts-control-row"
                          sx={{
                            flexWrap: "nowrap",
                            overflowX: "auto",
                            overflowY: "hidden",
                            whiteSpace: "nowrap",
                            minHeight: 36,
                            pb: 0.25,
                            scrollbarWidth: "thin",
                            "& .MuiButton-root": {
                              flexShrink: 0,
                              whiteSpace: "nowrap"
                            }
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
                            <Button
                              variant="outlined"
                              size="small"
                              onClick={() => void onTtsPlay()}
                              data-testid="reader-tts-play-button"
                            >
                              Play
                            </Button>
                          ) : null}
                          {ttsControlVisibility.showPauseButton ? (
                            <Button
                              variant="outlined"
                              size="small"
                              onClick={() => void onTtsPause()}
                              data-testid="reader-tts-pause-button"
                            >
                              Pause
                            </Button>
                          ) : null}
                          {ttsControlVisibility.showPlayPageButton ? (
                            <Button
                              variant="outlined"
                              size="small"
                              onClick={() => void onTtsPlayFromPageStart()}
                              disabled={reader.tts.sentence_count === 0}
                              data-testid="reader-tts-play-page-button"
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
                              data-testid="reader-tts-play-highlight-button"
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
                              data-testid="reader-tts-prev-sentence-button"
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
                              data-testid="reader-tts-next-sentence-button"
                            >
                              Next Sentence
                            </Button>
                          ) : null}
                          <Button
                            variant="outlined"
                            size="small"
                            onClick={() => void onTtsPrecomputePage()}
                            disabled={reader.tts.sentence_count === 0}
                            data-testid="reader-tts-precompute-page-button"
                          >
                            Precompute Page
                          </Button>
                          {ttsControlVisibility.showRepeatButton ? (
                            <Button
                              variant="outlined"
                              size="small"
                              onClick={() => void onTtsRepeatSentence()}
                              disabled={!hasHighlightSentence}
                              data-testid="reader-tts-repeat-button"
                            >
                              Repeat
                            </Button>
                          ) : null}
                        </Stack>
                        <Divider />
                        <NumericSettingControl
                          label="Playback Speed"
                          testId="setting-tts-speed"
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
                          testId="setting-tts-volume"
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
                          testId="setting-pause-after-sentence"
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
                  </div>
                </Stack>
              </div>
            ) : null}
          </Stack>
        </Stack>
      </CardContent>
    </Card>
  );
}
