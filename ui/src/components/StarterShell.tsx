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

import type { BootstrapState, CalibreBook, RecentBook } from "../types";

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
  onOpenCalibreBook
}: StarterShellProps) {
  const [path, setPath] = useState("");
  const [clipboardError, setClipboardError] = useState<string | null>(null);
  const [calibreSearch, setCalibreSearch] = useState("");
  const [showCalibre, setShowCalibre] = useState(true);
  const [calibreSort, setCalibreSort] = useState("title_asc");
  const [calibreScrollTop, setCalibreScrollTop] = useState(0);

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
    const query = calibreSearch.trim().toLowerCase();
    const filtered = calibreBooks.filter((book) => {
      if (!query) {
        return true;
      }
      return (
        book.title.toLowerCase().includes(query) ||
        book.authors.toLowerCase().includes(query) ||
        book.extension.toLowerCase().includes(query)
      );
    });

    const sorted = [...filtered];
    sorted.sort((left, right) => {
      switch (calibreSort) {
        case "title_desc":
          return right.title.localeCompare(left.title);
        case "author_asc":
          return left.authors.localeCompare(right.authors);
        case "author_desc":
          return right.authors.localeCompare(left.authors);
        case "year_desc":
          return (right.year ?? 0) - (left.year ?? 0);
        case "year_asc":
          return (left.year ?? 0) - (right.year ?? 0);
        case "id_asc":
          return left.id - right.id;
        case "id_desc":
          return right.id - left.id;
        case "title_asc":
        default:
          return left.title.localeCompare(right.title);
      }
    });
    return sorted;
  }, [calibreBooks, calibreSearch, calibreSort]);

  const virtualWindow = useMemo(() => {
    const totalCount = filteredCalibre.length;
    if (totalCount === 0) {
      return {
        items: [] as CalibreBook[],
        topSpacerPx: 0,
        bottomSpacerPx: 0
      };
    }
    const start = Math.max(0, Math.floor(calibreScrollTop / calibreRowHeight) - calibreOverscan);
    const maxVisible = Math.ceil(calibreViewportHeight / calibreRowHeight) + calibreOverscan * 2;
    const end = Math.min(totalCount, start + maxVisible);
    return {
      items: filteredCalibre.slice(start, end),
      topSpacerPx: start * calibreRowHeight,
      bottomSpacerPx: Math.max(0, (totalCount - end) * calibreRowHeight)
    };
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

          <Stack direction={{ xs: "column", md: "row" }} spacing={1.5}>
            <TextField
              fullWidth
              size="small"
              label="Open Path (.epub/.pdf/.txt/.md/.markdown)"
              value={path}
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
            >
              Open
            </Button>
            <Button variant="outlined" onClick={() => void handleClipboardOpen()} disabled={busy}>
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
              >
                {showCalibre ? "Hide" : "Show"}
              </Button>
              <Button
                size="small"
                variant="outlined"
                onClick={() => void onLoadCalibre(false)}
                disabled={busy || loadingCalibre}
              >
                Load
              </Button>
              <Button
                size="small"
                variant="text"
                startIcon={<RefreshIcon />}
                onClick={() => void onLoadCalibre(true)}
                disabled={busy || loadingCalibre}
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
                  onChange={(event) => setCalibreSearch(event.target.value)}
                  disabled={busy || loadingCalibre}
                />
                <FormControl size="small" className="md:min-w-56">
                  <InputLabel id="calibre-sort-label">Sort</InputLabel>
                  <Select
                    labelId="calibre-sort-label"
                    label="Sort"
                    value={calibreSort}
                    onChange={(event) => setCalibreSort(event.target.value)}
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
