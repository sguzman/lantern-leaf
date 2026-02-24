import DeleteOutlineIcon from "@mui/icons-material/DeleteOutline";
import FolderOpenIcon from "@mui/icons-material/FolderOpen";
import MenuBookIcon from "@mui/icons-material/MenuBook";
import RefreshIcon from "@mui/icons-material/Refresh";
import {
  Button,
  Card,
  CardActions,
  CardContent,
  CircularProgress,
  Divider,
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  Stack,
  TextField,
  Typography
} from "@mui/material";
import { useEffect, useMemo, useState } from "react";

import {
  computeVirtualWindow,
  filterAndSortCalibreBooks,
  type CalibreSort
} from "./calibreList";
import type {
  BootstrapState,
  CalibreBook,
  CalibreLoadEvent,
  PdfTranscriptionEvent,
  RecentBook,
  SourceOpenEvent
} from "../types";

interface StarterShellProps {
  bootstrap: BootstrapState | null;
  recents: RecentBook[];
  calibreBooks: CalibreBook[];
  busy: boolean;
  loadingRecents: boolean;
  loadingCalibre: boolean;
  onOpenPath: (path: string) => Promise<void>;
  onOpenClipboardText: (text: string) => Promise<void>;
  onDeleteRecent: (path: string) => Promise<void>;
  onRefreshRecents: () => Promise<void>;
  onLoadCalibre: (forceRefresh?: boolean) => Promise<void>;
  onOpenCalibreBook: (bookId: number) => Promise<void>;
  onSetRuntimeLogLevel: (level: string) => Promise<void>;
  sourceOpenEvent: SourceOpenEvent | null;
  calibreLoadEvent: CalibreLoadEvent | null;
  pdfTranscriptionEvent: PdfTranscriptionEvent | null;
  runtimeLogLevel: string;
}

export function StarterShell({
  bootstrap,
  recents,
  calibreBooks,
  busy,
  loadingRecents,
  loadingCalibre,
  onOpenPath,
  onOpenClipboardText,
  onDeleteRecent,
  onRefreshRecents,
  onLoadCalibre,
  onOpenCalibreBook,
  onSetRuntimeLogLevel,
  sourceOpenEvent,
  calibreLoadEvent,
  pdfTranscriptionEvent,
  runtimeLogLevel
}: StarterShellProps) {
  const [path, setPath] = useState("");
  const [clipboardError, setClipboardError] = useState<string | null>(null);
  const [calibreSearch, setCalibreSearch] = useState("");
  const [showCalibre, setShowCalibre] = useState(true);
  const [calibreSort, setCalibreSort] = useState<CalibreSort>("title_asc");
  const [calibreScrollTop, setCalibreScrollTop] = useState(0);
  const [logLevelValue, setLogLevelValue] = useState(runtimeLogLevel);

  const calibreRowHeight = 58;
  const calibreViewportHeight = 384;
  const calibreOverscan = 10;

  const summaryText = useMemo(() => {
    if (!bootstrap) {
      return "Loading bootstrap defaults...";
    }
    return `Defaults: font ${bootstrap.config.default_font_size}, lines/page ${bootstrap.config.default_lines_per_page}, TTS speed ${bootstrap.config.default_tts_speed.toFixed(2)}x, pause ${bootstrap.config.default_pause_after_sentence.toFixed(2)}s.`;
  }, [bootstrap]);

  const filteredCalibre = useMemo(() => {
    return filterAndSortCalibreBooks(calibreBooks, calibreSearch, calibreSort);
  }, [calibreBooks, calibreSearch, calibreSort]);

  const virtualWindow = useMemo(() => {
    return computeVirtualWindow(
      filteredCalibre,
      calibreScrollTop,
      calibreRowHeight,
      calibreViewportHeight,
      calibreOverscan
    );
  }, [
    calibreOverscan,
    calibreRowHeight,
    calibreScrollTop,
    calibreViewportHeight,
    filteredCalibre
  ]);

  useEffect(() => {
    setCalibreScrollTop(0);
  }, [calibreSearch, calibreSort, showCalibre]);

  useEffect(() => {
    setLogLevelValue(runtimeLogLevel);
  }, [runtimeLogLevel]);

  const handleOpenPath = async () => {
    await onOpenPath(path);
  };

  const handleClipboardOpen = async () => {
    setClipboardError(null);
    try {
      if (!navigator.clipboard?.readText) {
        throw new Error("Clipboard API is not available in this runtime");
      }
      const text = await navigator.clipboard.readText();
      await onOpenClipboardText(text);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      setClipboardError(message);
    }
  };

  const hasRecents = recents.length > 0;
  const sourceOpenStatus =
    sourceOpenEvent && sourceOpenEvent.phase !== "ready"
      ? `Open #${sourceOpenEvent.request_id}: ${sourceOpenEvent.phase}${
          sourceOpenEvent.source_path ? ` · ${sourceOpenEvent.source_path}` : ""
        }${sourceOpenEvent.message ? ` · ${sourceOpenEvent.message}` : ""}`
      : null;
  const calibreStatus =
    calibreLoadEvent && calibreLoadEvent.phase !== "ready"
      ? `Calibre #${calibreLoadEvent.request_id}: ${calibreLoadEvent.phase}${
          calibreLoadEvent.count !== null ? ` · ${calibreLoadEvent.count.toLocaleString()} books` : ""
        }${calibreLoadEvent.message ? ` · ${calibreLoadEvent.message}` : ""}`
      : null;
  const pdfStatus =
    pdfTranscriptionEvent && pdfTranscriptionEvent.phase !== "ready"
      ? `PDF #${pdfTranscriptionEvent.request_id}: ${pdfTranscriptionEvent.phase}${
          pdfTranscriptionEvent.source_path ? ` · ${pdfTranscriptionEvent.source_path}` : ""
        }${pdfTranscriptionEvent.message ? ` · ${pdfTranscriptionEvent.message}` : ""}`
      : null;

  return (
    <Card className="w-full max-w-7xl rounded-3xl border border-slate-200 shadow-sm">
      <CardContent>
        <Stack spacing={2.5}>
          <Stack direction="row" spacing={1} alignItems="center">
            <MenuBookIcon fontSize="small" />
            <Typography variant="h5" component="h1" fontWeight={700}>
              Welcome
            </Typography>
          </Stack>

          <Typography variant="body1" color="text.secondary">
            React/Tauri migration shell is active. The bridge now supports real source loading,
            reader snapshots, search/navigation, settings/stats, and calibre open flows.
          </Typography>
          <Typography variant="body2" color="text.secondary">
            {summaryText}
          </Typography>
          <Typography variant="caption" color="text.secondary">
            Runtime log level:{" "}
            <span data-testid="starter-runtime-log-level-value">{runtimeLogLevel}</span>
          </Typography>
          <Stack direction={{ xs: "column", md: "row" }} spacing={1} alignItems="center">
            <FormControl size="small" className="md:min-w-44">
              <InputLabel id="runtime-log-level-label">Log Level</InputLabel>
              <Select
                labelId="runtime-log-level-label"
                label="Log Level"
                value={logLevelValue}
                onChange={(event) => setLogLevelValue(String(event.target.value))}
                disabled={busy}
                data-testid="starter-log-level-select"
              >
                <MenuItem value="trace">trace</MenuItem>
                <MenuItem value="debug">debug</MenuItem>
                <MenuItem value="info">info</MenuItem>
                <MenuItem value="warn">warn</MenuItem>
                <MenuItem value="error">error</MenuItem>
              </Select>
            </FormControl>
            <Button
              size="small"
              variant="outlined"
              onClick={() => void onSetRuntimeLogLevel(logLevelValue)}
              disabled={busy || runtimeLogLevel === logLevelValue}
              data-testid="starter-log-level-apply-button"
            >
              Apply Log Level
            </Button>
          </Stack>
          {sourceOpenStatus ? (
            <Typography variant="caption" color="text.secondary" data-testid="starter-open-status">
              {sourceOpenStatus}
            </Typography>
          ) : null}
          {calibreStatus ? (
            <Typography
              variant="caption"
              color="text.secondary"
              data-testid="starter-calibre-status"
            >
              {calibreStatus}
            </Typography>
          ) : null}
          {pdfStatus ? (
            <Typography variant="caption" color="text.secondary" data-testid="starter-pdf-status">
              {pdfStatus}
            </Typography>
          ) : null}

          <Stack direction={{ xs: "column", md: "row" }} spacing={1.5}>
            <TextField
              fullWidth
              size="small"
              label="Open Path (.epub/.pdf/.txt/.md/.markdown)"
              value={path}
              inputProps={{ "data-testid": "starter-open-path-input" }}
              onChange={(event) => setPath(event.target.value)}
              onKeyDown={(event) => {
                if (event.key === "Enter") {
                  void handleOpenPath();
                }
              }}
              disabled={busy}
            />
            <Button
              variant="contained"
              startIcon={<FolderOpenIcon />}
              onClick={() => void handleOpenPath()}
              disabled={busy}
              data-testid="starter-open-path-button"
            >
              Open
            </Button>
            <Button
              variant="outlined"
              onClick={() => void handleClipboardOpen()}
              disabled={busy}
              data-testid="starter-open-clipboard-button"
            >
              Open Clipboard
            </Button>
          </Stack>

          {clipboardError ? (
            <Typography variant="caption" color="error">
              {clipboardError}
            </Typography>
          ) : null}

          <Divider />

          <Stack direction="row" alignItems="center" justifyContent="space-between">
            <Typography variant="h6" component="h2" fontWeight={700}>
              Recent Books
            </Typography>
            <Button
              size="small"
              variant="text"
              startIcon={<RefreshIcon />}
              onClick={() => void onRefreshRecents()}
              disabled={busy || loadingRecents}
            >
              Refresh
            </Button>
          </Stack>

          {loadingRecents ? (
            <Stack direction="row" spacing={1} alignItems="center">
              <CircularProgress size={18} />
              <Typography variant="body2" color="text.secondary">
                Loading recent books...
              </Typography>
            </Stack>
          ) : null}

          {!hasRecents && !loadingRecents ? (
            <Typography variant="body2" color="text.secondary">
              No recent books yet.
            </Typography>
          ) : null}

          {hasRecents ? (
            <div className="grid grid-cols-1 gap-3 md:grid-cols-2">
              {recents.map((recent) => (
                <Card
                  key={recent.source_path}
                  variant="outlined"
                  className="rounded-2xl border-slate-200 shadow-none"
                  data-testid="starter-recent-card"
                  data-recent-path={recent.source_path}
                >
                  <CardContent className="pb-3">
                    <Stack spacing={0.75}>
                      <Typography variant="subtitle1" fontWeight={700} noWrap>
                        {recent.display_title}
                      </Typography>
                      <Typography variant="caption" color="text.secondary" className="truncate">
                        {recent.source_path}
                      </Typography>
                    </Stack>
                  </CardContent>
                  <CardActions className="px-4 pb-4 pt-0">
                    <Button
                      size="small"
                      variant="contained"
                      onClick={() => void onOpenPath(recent.source_path)}
                      disabled={busy}
                      data-testid="starter-recent-open-button"
                      data-recent-path={recent.source_path}
                    >
                      Open
                    </Button>
                    <Button
                      size="small"
                      color="error"
                      variant="outlined"
                      startIcon={<DeleteOutlineIcon />}
                      onClick={() => void onDeleteRecent(recent.source_path)}
                      disabled={busy}
                      data-testid="starter-recent-delete-button"
                      data-recent-path={recent.source_path}
                    >
                      Delete
                    </Button>
                  </CardActions>
                </Card>
              ))}
            </div>
          ) : null}

          <Divider />

          <Stack direction="row" alignItems="center" justifyContent="space-between">
            <Typography variant="h6" component="h2" fontWeight={700}>
              Calibre Library
            </Typography>
            <Stack direction="row" spacing={1}>
              <Button
                size="small"
                variant="outlined"
                onClick={() => setShowCalibre((current) => !current)}
                disabled={busy}
                data-testid="starter-calibre-toggle-button"
              >
                {showCalibre ? "Hide" : "Show"}
              </Button>
              <Button
                size="small"
                variant="outlined"
                onClick={() => void onLoadCalibre(false)}
                disabled={busy || loadingCalibre}
                data-testid="starter-calibre-load-button"
              >
                Load
              </Button>
              <Button
                size="small"
                variant="text"
                startIcon={<RefreshIcon />}
                onClick={() => void onLoadCalibre(true)}
                disabled={busy || loadingCalibre}
                data-testid="starter-calibre-refresh-button"
              >
                Refresh
              </Button>
            </Stack>
          </Stack>

          {showCalibre ? (
            <Stack spacing={1}>
              <Stack direction={{ xs: "column", md: "row" }} spacing={1}>
                <TextField
                  size="small"
                  fullWidth
                  label="Search calibre (title/author/format)"
                  value={calibreSearch}
                  inputProps={{ "data-testid": "starter-calibre-search-input" }}
                  onChange={(event) => setCalibreSearch(event.target.value)}
                  disabled={busy || loadingCalibre}
                />
                <FormControl size="small" className="md:min-w-56">
                  <InputLabel id="calibre-sort-label">Sort</InputLabel>
                  <Select
                    labelId="calibre-sort-label"
                    label="Sort"
                    value={calibreSort}
                    onChange={(event) => setCalibreSort(event.target.value as CalibreSort)}
                    disabled={busy || loadingCalibre}
                  >
                    <MenuItem value="title_asc">Title (A-Z)</MenuItem>
                    <MenuItem value="title_desc">Title (Z-A)</MenuItem>
                    <MenuItem value="author_asc">Author (A-Z)</MenuItem>
                    <MenuItem value="author_desc">Author (Z-A)</MenuItem>
                    <MenuItem value="year_desc">Year (Newest)</MenuItem>
                    <MenuItem value="year_asc">Year (Oldest)</MenuItem>
                    <MenuItem value="id_asc">Book ID (Ascending)</MenuItem>
                    <MenuItem value="id_desc">Book ID (Descending)</MenuItem>
                  </Select>
                </FormControl>
              </Stack>

              {!loadingCalibre && filteredCalibre.length > 0 ? (
                <Typography variant="caption" color="text.secondary">
                  Showing {filteredCalibre.length.toLocaleString()} calibre entries
                </Typography>
              ) : null}
            </Stack>
          ) : null}

          {loadingCalibre ? (
            <Stack direction="row" spacing={1} alignItems="center">
              <CircularProgress size={18} />
              <Typography variant="body2" color="text.secondary">
                Loading calibre books...
              </Typography>
            </Stack>
          ) : null}

          {!loadingCalibre && filteredCalibre.length === 0 ? (
            <Typography variant="body2" color="text.secondary">
              No calibre books loaded yet.
            </Typography>
          ) : null}

          {showCalibre && filteredCalibre.length > 0 ? (
            <div
              className="overflow-y-auto rounded-2xl border border-slate-200"
              style={{ maxHeight: calibreViewportHeight }}
              onScroll={(event) => {
                setCalibreScrollTop(event.currentTarget.scrollTop);
              }}
            >
              <div className="divide-y divide-slate-200">
                {virtualWindow.topSpacerPx > 0 ? (
                  <div style={{ height: virtualWindow.topSpacerPx }} />
                ) : null}
                {virtualWindow.items.map((book) => (
                  <div key={book.id} className="flex items-center justify-between gap-3 px-4 py-2.5">
                    <Stack spacing={0.25} className="min-w-0">
                      <Typography variant="subtitle2" noWrap>
                        {book.title}
                      </Typography>
                      <Typography variant="caption" color="text.secondary" noWrap>
                        {book.authors || "Unknown author"} · {book.extension.toUpperCase()}
                        {book.year ? ` · ${book.year}` : ""}
                      </Typography>
                    </Stack>
                    <Button
                      size="small"
                      variant="contained"
                      onClick={() => void onOpenCalibreBook(book.id)}
                      disabled={busy}
                      data-testid="starter-calibre-open-button"
                      data-book-id={book.id}
                    >
                      Open
                    </Button>
                  </div>
                ))}
                {virtualWindow.bottomSpacerPx > 0 ? (
                  <div style={{ height: virtualWindow.bottomSpacerPx }} />
                ) : null}
              </div>
            </div>
          ) : null}
        </Stack>
      </CardContent>
    </Card>
  );
}
