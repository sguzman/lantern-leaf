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
  Stack,
  TextField,
  Typography
} from "@mui/material";
import { useMemo, useState } from "react";

import type { BootstrapState, RecentBook } from "../types";

interface StarterShellProps {
  bootstrap: BootstrapState | null;
  recents: RecentBook[];
  busy: boolean;
  loadingRecents: boolean;
  onOpenPath: (path: string) => Promise<void>;
  onOpenClipboardText: (text: string) => Promise<void>;
  onDeleteRecent: (path: string) => Promise<void>;
  onRefreshRecents: () => Promise<void>;
}

export function StarterShell({
  bootstrap,
  recents,
  busy,
  loadingRecents,
  onOpenPath,
  onOpenClipboardText,
  onDeleteRecent,
  onRefreshRecents
}: StarterShellProps) {
  const [path, setPath] = useState("");
  const [clipboardError, setClipboardError] = useState<string | null>(null);

  const summaryText = useMemo(() => {
    if (!bootstrap) {
      return "Loading bootstrap defaults...";
    }
    return `Defaults: font ${bootstrap.config.default_font_size}, lines/page ${bootstrap.config.default_lines_per_page}, TTS speed ${bootstrap.config.default_tts_speed.toFixed(2)}x, pause ${bootstrap.config.default_pause_after_sentence.toFixed(2)}s.`;
  }, [bootstrap]);

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
    <Card className="w-full max-w-6xl rounded-3xl border border-slate-200 shadow-sm">
      <CardContent>
        <Stack spacing={2.5}>
          <Stack direction="row" spacing={1} alignItems="center">
            <MenuBookIcon fontSize="small" />
            <Typography variant="h5" component="h1" fontWeight={700}>
              Welcome
            </Typography>
          </Stack>

          <Typography variant="body1" color="text.secondary">
            React/Tauri migration shell is active. Existing Rust core behavior stays as the source
            of truth while UI parity is being ported.
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
        </Stack>
      </CardContent>
    </Card>
  );
}
