# Google Chat Desktop

Native macOS desktop wrapper for Google Chat built with [Tauri v2](https://tauri.app/). Uses the system WebView (WKWebView) instead of bundling Chromium.

## Features

- Native macOS window wrapping `chat.google.com`
- System tray icon with Hide/Quit menu
- Click tray or dock icon to restore window
- Close-to-tray (app stays running when window is closed)
- Launch at login (via autostart plugin)
- Global keyboard shortcuts (via global-shortcut plugin)
- Notification support (via notification plugin)

## Prerequisites

- [Rust](https://rustup.rs/)
- [Node.js](https://nodejs.org/) (v18+)

## Development

```sh
npm install
npm run tauri dev
```

## Build

```sh
npm run tauri build
```

Outputs:
- `src-tauri/target/release/bundle/macos/Google Chat.app`
- `src-tauri/target/release/bundle/dmg/Google Chat_0.1.0_aarch64.dmg`

## CLAUDE.md

See [CLAUDE.md](CLAUDE.md) for AI assistant context.
