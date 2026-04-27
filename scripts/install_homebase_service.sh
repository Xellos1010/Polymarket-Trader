#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SERVICE_NAME="com.pt.homebase"
PLIST_PATH="$HOME/Library/LaunchAgents/${SERVICE_NAME}.plist"
CONFIG_PATH="${1:-$ROOT_DIR/config/config.toml}"

mkdir -p "$HOME/Library/LaunchAgents"

cat > "$PLIST_PATH" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>${SERVICE_NAME}</string>
  <key>ProgramArguments</key>
  <array>
    <string>${ROOT_DIR}/target/release/pt-cli</string>
    <string>run-homebase</string>
    <string>--config</string>
    <string>${CONFIG_PATH}</string>
  </array>
  <key>WorkingDirectory</key>
  <string>${ROOT_DIR}</string>
  <key>RunAtLoad</key>
  <true/>
  <key>KeepAlive</key>
  <true/>
  <key>StandardOutPath</key>
  <string>${ROOT_DIR}/data/output/homebase.stdout.log</string>
  <key>StandardErrorPath</key>
  <string>${ROOT_DIR}/data/output/homebase.stderr.log</string>
</dict>
</plist>
PLIST

launchctl unload "$PLIST_PATH" >/dev/null 2>&1 || true
launchctl load "$PLIST_PATH"

echo "Installed homebase service: $SERVICE_NAME"
echo "Status: launchctl list | rg '$SERVICE_NAME'"
echo "Logs: $ROOT_DIR/data/output/homebase.stdout.log"
