# Migration Baseline Metrics

Baseline captured from migration shell E2E run on 2026-02-24 (local dev environment, mock adapter path).

Command:

```bash
pnpm --dir ui run test:e2e
```

Observed perf sample (`ui/e2e/perfBaseline.spec.ts`):

- Startup to Welcome visible: `725 ms`
- Source open (starter -> reader transition): `117 ms`
- Page switch latency (Next Page -> page input update): `70 ms`
- TTS start control latency (toggle -> Pause label): `51 ms`
- Resize responsiveness (viewport change -> controls stable): `7 ms`

## Notes

- These values are WebView-shell migration baselines under mock data; they are used for regression detection during ongoing migration work.
- Final parity signoff requires additional baselines against full Tauri runtime and real source content.

## Tauri Runtime Soak Baseline

Runtime soak captured on `2026-02-24` from:

```bash
pnpm --dir ui run test:e2e:tauri:soak -- --iterations 3
```

From `tmp/tauri-soak-report.json`:

- Iterations: `3`
- Pass/fail: `3/0`
- Average smoke duration: `50101.590 ms`
- Min smoke duration: `49390.887 ms`
- Max smoke duration: `50578.461 ms`
- p95 smoke duration: `50578.461 ms`
