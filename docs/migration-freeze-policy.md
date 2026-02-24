# Migration Freeze Policy

During migration parity work:

- New user-facing features are frozen unless they unblock parity or fix migration regressions.
- Priority order is: correctness regressions, parity gaps, test coverage gaps, then net-new features.
- Any exception must include:
  - Explicit migration-blocking rationale.
  - Added regression tests for changed behavior.
  - Roadmap checkbox updates for traceability.

## Active Exception Categories

- Bridge contract hardening (events, error taxonomy, log-level controls).
- Test harness additions required for parity proof (unit/bridge/E2E).
- Runtime safety controls (safe quit, task cancellation, capability hardening).
