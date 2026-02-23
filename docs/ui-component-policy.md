# UI Component Policy (MUI + Tailwind)

This policy keeps the migrated UI consistent and prevents style drift.

## Responsibility Split

- Use Material UI for:
  - Inputs, buttons, sliders, switches, dialogs, cards, alerts, tables.
  - Theme tokens and component-level interaction behavior.
- Use Tailwind for:
  - Layout structure (grid/flex/spacing/alignment/sizing).
  - Responsive placement and container utilities.

## Styling Rules

- Do not replace MUI controls with ad-hoc HTML controls unless there is a measured performance reason.
- Keep visual tokens sourced from the MUI theme and synced CSS variables.
- Prefer Tailwind utility classes on layout wrappers, not deep inside MUI internals.
- Avoid overlapping reset behavior; keep `CssBaseline` as the base reset.
- Avoid one-off inline color constants; use theme palette or CSS variables.

## Top-Bar and Reader Constraints

- Top controls must never vertically compress text/buttons.
- If horizontal space is insufficient, hide optional controls instead of wrapping vertically.
- Preserve fixed-height control rows for top bar and TTS control regions.

## Accessibility

- Preserve keyboard shortcuts from config.
- Keep focus-visible states for interactive controls.
- Keep semantic labels on search/settings/stats/TTS controls.

## Validation Checklist

- `pnpm --dir ui run lint`
- `pnpm --dir ui run check`
- `pnpm --dir ui run test`
- `pnpm --dir ui run build`
