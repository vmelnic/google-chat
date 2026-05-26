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

## Releasing

Releases are built automatically via GitHub Actions for macOS (ARM + Intel), Windows, and Linux.

1. Update the version in `package.json` and `src-tauri/tauri.conf.json`
2. Commit and tag:
   ```sh
   git add -A && git commit -m "bump version to 0.2.0"
   git tag v0.2.0
   git push origin main --tags
   ```
3. The workflow builds all platforms and creates a **draft** GitHub release with:
   - macOS `.dmg` (Apple Silicon + Intel)
   - Windows `.msi` and `.exe`
   - Linux `.deb` and `.AppImage`
4. Review the draft release on GitHub and publish it

## CLAUDE.md

See [CLAUDE.md](CLAUDE.md) for AI assistant context.
