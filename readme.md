# Bedtime COSMIC

A simple shutdown timer applet for the [COSMIC Desktop Environment](https://system76.com/cosmic).

## Features

- Schedule shutdown in **10, 30 or 60 minutes**.
- Display the remaining time.
- Cancel a scheduled shutdown.
- Cyan icon when a timer is active.
- Lightweight native Rust applet.

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

- Linux
- COSMIC Desktop Environment
- A compatible precompiled release binary

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

## License

See [LICENSE](LICENSE) for details.