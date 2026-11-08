# Flappy Dusk — Rust desktop app for Windows

Flappy Dusk is packaged as a Rust desktop application with Tauri. The existing HTML game is kept as the frontend and displayed in the Windows WebView2 runtime. The Windows installer is built as an NSIS `.exe`.

The app opens on the flight settings screen. Choose the flight options and click **Go Fly** to enter fullscreen gameplay. The in-game gear opens flight settings, and **Save & Quit** saves your progress before returning to the home screen. The FPS counter reports rendered frames per second.

The desktop app creates a `saves` folder in its per-user application data directory and stores progress in `progress.json`. Website progress remains in encrypted browser storage; use the recovery-code controls on the website to back up or transfer it.

The app, installer, uninstaller, and optional desktop shortcut use `flappybird-logo.png`. The Windows installer uses Flappy Dusk colors and artwork, includes the standard install-location and Start Menu steps, and lets you create a desktop shortcut from its final screen.

## Requirements

- Rust with the MSVC toolchain
- Node.js and npm
- Microsoft C++ Build Tools
- Microsoft Edge WebView2 Runtime
- NSIS, available on `PATH`, to build the installer

## Run and build

1. Install the requirements above.
2. From this folder, run `npm install`.
3. Run `npm run dev` to launch the game in the Tauri desktop window. The app creates its local saves folder when it starts.
4. Run `npm run build:win` to create the Windows installer.

The installer is written to `src-tauri\target\release\bundle\nsis\`. It installs for the current Windows user.
