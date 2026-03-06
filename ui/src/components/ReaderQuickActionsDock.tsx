import GraphicEqIcon from "@mui/icons-material/GraphicEq";
import QueryStatsIcon from "@mui/icons-material/QueryStats";
import SpeedDialIcon from "@mui/material/SpeedDialIcon";
import TextFieldsIcon from "@mui/icons-material/TextFields";
import TuneIcon from "@mui/icons-material/Tune";
import { Box, ClickAwayListener, Fab, Paper, Stack, Typography } from "@mui/material";
import { memo, useCallback, useMemo, useState } from "react";

import { useReaderQuickActionsState } from "../store/selectors";

export const ReaderQuickActionsDock = memo(function ReaderQuickActionsDock() {
  const {
    busy,
    isTextOnly,
    showSettings,
    showStats,
    showTts,
    onToggleTextOnly,
    onToggleSettingsPanel,
    onToggleStatsPanel,
    onToggleTtsPanel
  } = useReaderQuickActionsState();
  const [open, setOpen] = useState(false);

  const actions = useMemo(
    () => [
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
      isTextOnly,
      onToggleSettingsPanel,
      onToggleStatsPanel,
      onToggleTextOnly,
      onToggleTtsPanel,
      showSettings,
      showStats,
      showTts
    ]
  );

  const close = useCallback(() => setOpen(false), []);

  return (
    <ClickAwayListener onClickAway={close}>
      <Box
        sx={{
          position: "fixed",
          top: 16,
          right: 16,
          zIndex: (theme) => theme.zIndex.modal + 1
        }}
      >
        <Stack direction="column" spacing={1} alignItems="flex-end">
          <Fab
            size="small"
            color="primary"
            onClick={(event) => {
              event.stopPropagation();
              setOpen((current) => !current);
            }}
            sx={{
              transform: open ? "rotate(90deg)" : "rotate(0deg)",
              transition: "transform 170ms cubic-bezier(0.2, 0, 0, 1)"
            }}
            data-testid="reader-quick-actions-speed-dial"
          >
            <SpeedDialIcon open={open} />
          </Fab>

          <Stack
            spacing={1}
            alignItems="flex-end"
            sx={{
              maxHeight: open ? 280 : 0,
              opacity: open ? 1 : 0,
              overflow: "hidden",
              pointerEvents: open ? "auto" : "none",
              transition:
                "max-height 220ms cubic-bezier(0.2, 0, 0, 1), opacity 160ms ease-out"
            }}
          >
            {actions.map((action, index) => (
              <Stack
                key={action.key}
                direction="row"
                spacing={1}
                alignItems="center"
                sx={{
                  transform: open ? "translateY(0) scale(1)" : "translateY(-8px) scale(0.95)",
                  opacity: open ? 1 : 0,
                  transition: `transform 180ms cubic-bezier(0.2, 0, 0, 1) ${index * 28}ms, opacity 140ms ease-out ${index * 28}ms`
                }}
              >
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
            ))}
          </Stack>
        </Stack>
      </Box>
    </ClickAwayListener>
  );
});
