# Bedtime COSMIC

A simple Rust applet for the COSMIC Desktop Environment, providing a panel menu to schedule system shutdowns

<img width="364" height="285" alt="Screenshot_2026-11-02_21-43-11" src="https://github.com/user-attachments/assets/1e0eb25a-78bf-47f2-a9d4-610228906fd8" />

<img width="364" height="285" alt="Screenshot_2026-11-02_21-43-18" src="https://github.com/user-attachments/assets/77f68214-3487-4878-8003-64cd9af01b85" />

Bedtime uses systemctl poweroff to shut down the system. On most systemd-based desktop distributions, this should work without requiring a password when run from an active local user session

## Features

- Schedule shutdown in **10, 30 or 60 minutes**.
- Display the remaining time.
- Cancel a scheduled shutdown.

## Installation

1. Download the latest release.
2. Extract the archive.
3. Open a terminal in the extracted directory.
4. Run:

```bash
chmod +x install.sh
./install.sh
```

No Rust or Cargo installation required.

Then open **Settings → Desktop → Applets**, find Bedtime and add it to your panel.

## Requirements

- COSMIC Desktop Environment

## Building from source

```bash
cargo build --release
```

## Uninstallation

Remove the installed files:

```bash
rm ~/.local/bin/bedtime-cosmic
rm ~/.local/share/applications/io.github.mirqko.BedtimeCosmic.desktop
rm ~/.local/share/icons/hicolor/scalable/apps/io.github.mirqko.BedtimeCosmic.svg
```
