import ArrowBackIcon from "@mui/icons-material/ArrowBack";
import {
  Button,
  Card,
  CardActions,
  CardContent,
  Divider,
  Stack,
  Typography
} from "@mui/material";

import type { SessionState } from "../types";

interface ReaderShellProps {
  session: SessionState;
  busy: boolean;
  onBack: () => Promise<void>;
}

export function ReaderShell({ session, busy, onBack }: ReaderShellProps) {
  return (
    <Card className="w-full max-w-5xl rounded-3xl border border-slate-200 shadow-sm">
      <CardContent>
        <Stack spacing={2}>
          <Typography variant="h5" component="h2" fontWeight={700}>
            Reader Surface (Migration)
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Source selected successfully. The full reader parity port (rendering/highlight/TTS
            controls/stats/settings/search) is the next migration phase.
          </Typography>
          <Divider />
          <Stack spacing={0.5}>
            <Typography variant="body2" color="text.secondary">
              Active source
            </Typography>
            <Typography variant="body2" fontFamily="monospace">
              {session.active_source_path ?? "None"}
            </Typography>
          </Stack>
        </Stack>
      </CardContent>
      <CardActions className="px-6 pb-6">
        <Button
          variant="outlined"
          startIcon={<ArrowBackIcon />}
          onClick={() => void onBack()}
          disabled={busy}
        >
          Close Session
        </Button>
      </CardActions>
    </Card>
  );
}
