#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FRONTEND_DIR="$ROOT_DIR/crates/pt-dashboard/frontend"
LOCAL_DIR="$ROOT_DIR/.local"
BACKEND_LOG="$LOCAL_DIR/dev-backend.log"
FRONTEND_LOG="$LOCAL_DIR/dev-frontend.log"

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "missing required command: $1" >&2
    exit 1
  fi
}

require_path() {
  if [ ! -e "$1" ]; then
    echo "missing required path: $1" >&2
    exit 1
  fi
}

cleanup() {
  if [ -n "${BACKEND_PID:-}" ] && kill -0 "$BACKEND_PID" >/dev/null 2>&1; then
    kill "$BACKEND_PID" >/dev/null 2>&1 || true
  fi
  if [ -n "${FRONTEND_PID:-}" ] && kill -0 "$FRONTEND_PID" >/dev/null 2>&1; then
    kill "$FRONTEND_PID" >/dev/null 2>&1 || true
  fi
}

require_cmd cargo
require_cmd pnpm
require_path "$ROOT_DIR/config/config.toml"
require_path "$FRONTEND_DIR/node_modules"

mkdir -p "$LOCAL_DIR"

trap cleanup EXIT INT TERM

echo "Starting backend on http://127.0.0.1:8080"
(
  cd "$ROOT_DIR"
  cargo run -p pt-cli -- run --config config/config.toml
) 2>&1 | tee "$BACKEND_LOG" &
BACKEND_PID=$!

echo "Starting frontend on http://127.0.0.1:5173"
(
  cd "$FRONTEND_DIR"
  pnpm run dev -- --host 0.0.0.0 --port 5173
) 2>&1 | tee "$FRONTEND_LOG" &
FRONTEND_PID=$!

echo "Logs:"
echo "  backend  -> $BACKEND_LOG"
echo "  frontend -> $FRONTEND_LOG"

while kill -0 "$BACKEND_PID" >/dev/null 2>&1 && kill -0 "$FRONTEND_PID" >/dev/null 2>&1; do
  sleep 1
done

if ! kill -0 "$BACKEND_PID" >/dev/null 2>&1; then
  wait "$BACKEND_PID"
else
  wait "$FRONTEND_PID"
fi
