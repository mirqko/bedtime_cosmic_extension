
#!/bin/bash
set -e

APP_ID="io.github.mirqko.BedtimeCosmic"
DIR="$(cd -- "$(dirname -- "$0")" && pwd)"

mkdir -p ~/.local/bin
install -Dm755 "$DIR/target/release/bedtime-cosmic" ~/.local/bin/bedtime-cosmic

ICON_DIR="$HOME/.local/share/icons/hicolor/scalable/apps"
mkdir -p "$ICON_DIR"

if [ -f "$DIR/resources/icon_white.svg" ]; then
    install -Dm644 "$DIR/resources/icon_white.svg" \
        "$ICON_DIR/$APP_ID.svg"
fi

mkdir -p ~/.local/share/applications

cat > ~/.local/share/applications/$APP_ID.desktop <<EOF
[Desktop Entry]
Name=Bedtime
Comment=Shutdown timer for COSMIC
Type=Application
Exec=$HOME/.local/bin/bedtime-cosmic
Icon=$APP_ID
Terminal=false
NoDisplay=true
X-CosmicApplet=true
X-CosmicHoverPopup=Auto
EOF

command -v gtk-update-icon-cache >/dev/null &&
    gtk-update-icon-cache -f -t "$HOME/.local/share/icons/hicolor" || true
