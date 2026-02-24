#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <host> [user] [ssh_key] [remote_dir]" >&2
  exit 1
fi

HOST="$1"
USER_NAME="${2:-ubuntu}"
SSH_KEY="${3:-}"
REMOTE_DIR="${4:-/opt/Polymarket-Trader}"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="$ROOT_DIR/dist"

if [[ -n "$SSH_KEY" ]]; then
  SSH_OPTS=(-i "$SSH_KEY")
else
  SSH_OPTS=()
fi

"$ROOT_DIR/scripts/build_release_bundle.sh" "$DIST_DIR"
ARCHIVE="$(ls -1t "$DIST_DIR"/polymarket-trader-*.tar.gz | head -n1)"
ARCHIVE_BASENAME="$(basename "$ARCHIVE")"

scp "${SSH_OPTS[@]}" "$ARCHIVE" "$USER_NAME@$HOST:/tmp/$ARCHIVE_BASENAME"

ssh "${SSH_OPTS[@]}" "$USER_NAME@$HOST" bash -s <<REMOTE
set -euo pipefail

sudo mkdir -p "$REMOTE_DIR"
sudo chown -R "$USER_NAME":"$USER_NAME" "$REMOTE_DIR"

TMP_DIR="/tmp/polymarket-trader-deploy"
rm -rf "\$TMP_DIR"
mkdir -p "\$TMP_DIR"

tar -C "\$TMP_DIR" -xzf "/tmp/$ARCHIVE_BASENAME"
NEW_DIR="\$(find \"\$TMP_DIR\" -maxdepth 1 -type d -name 'polymarket-trader-*' | head -n1)"

cp -r "\$NEW_DIR"/* "$REMOTE_DIR/"

sudo cp "$REMOTE_DIR/scripts/pt-engine.service" /etc/systemd/system/pt-engine.service
sudo systemctl daemon-reload
sudo systemctl enable pt-engine
sudo systemctl restart pt-engine

sleep 3
curl -fsS http://127.0.0.1:8080/healthz >/dev/null
curl -fsS http://127.0.0.1:8080/ready >/dev/null

echo "deploy ok"
REMOTE

echo "Deployment completed to $USER_NAME@$HOST:$REMOTE_DIR"
