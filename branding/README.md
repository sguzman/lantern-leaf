# LanternLeaf Branding

## Identity

- App name: `LanternLeaf`
- Mascot name: `Pipwick`
- Mascot concept: anthropomorphic chibi candle scholar with glasses reading books

## Source Assets

- Mascot artwork: `branding/mascot.png`
- Color palette and CSS variables: `branding/colors.css`
- Naming notes: `branding/project-name.md`
- Mascot notes: `branding/mascot.md`

## Generated Favicon Assets

- `branding/favicon/favicon-16x16.png`
- `branding/favicon/favicon-32x32.png`
- `branding/favicon/favicon-48x48.png`
- `branding/favicon/favicon.ico`
- `branding/favicon/apple-touch-icon.png`
- `branding/favicon/android-chrome-192x192.png`
- `branding/favicon/android-chrome-512x512.png`

## Runtime Integration

- Web favicon + manifest links are configured in `ui/index.html`.
- Web static assets are copied to `ui/public/`.
- Desktop/taskbar icon assets are generated under `src-tauri/icons/` and wired in `src-tauri/tauri.conf.json`.
