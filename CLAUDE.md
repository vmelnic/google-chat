# Google Chat Desktop — Tauri v2 Wrapper

## Build & Run

```sh
npm install
npm run tauri dev      # development
npm run tauri build    # production (outputs .app and .dmg)
```

## Architecture

- **Frontend**: Minimal `src/index.html` loading page shown briefly before navigating to `https://chat.google.com`
- **Backend**: `src-tauri/src/lib.rs` — Rust Tauri app with tray icon, close-to-tray, dock reopen
- **Config**: `src-tauri/tauri.conf.json` — window settings, Chrome user-agent override, CSP rules

## Key Decisions

- User-agent is spoofed as Chrome because Google Chat blocks non-Chrome webviews
- Window close is intercepted to hide-to-tray instead of quitting; quit is via tray menu or Cmd+Q
- `RunEvent::Reopen` handles dock icon clicks to restore hidden windows
- Icons were generated from SVG using a Swift converter (`svg2png`)

## Tauri Plugins

- `tauri-plugin-notification` — native notifications
- `tauri-plugin-autostart` — launch at login
- `tauri-plugin-global-shortcut` — keyboard shortcuts

## Known Limitations

- macOS 26 Liquid Glass title bar is content-adaptive (samples pixel brightness beneath it) — no API exists to force it dark when the web content header is light
