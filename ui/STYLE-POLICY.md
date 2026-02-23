# UI Style Policy (MUI + Tailwind)

- Use Material UI components for controls, form fields, cards, dialogs, sliders, and tables.
- Use Tailwind utilities for layout containers, spacing, sizing, and responsive grid/flex behavior.
- Keep component-specific visual tokens in MUI theme values; do not hardcode ad hoc colors in JSX.
- Keep Tailwind preflight enabled and pair it with MUI `CssBaseline`.
- Avoid conflicting global resets beyond Tailwind preflight and MUI baseline.
- For shared visual constants, define the value in the MUI theme first, then reference consistently in UI code.
