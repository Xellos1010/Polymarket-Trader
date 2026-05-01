#!/usr/bin/env bash
set -euo pipefail

if [ -f ./.local/pt-engine.pid ]; then
  PID=$(cat ./.local/pt-engine.pid)
  echo "Stopping pt-engine with PID $PID..."
  if kill "$PID"; then
    echo "pt-engine stopped."
    rm ./.local/pt-engine.pid
  else
    echo "Failed to stop pt-engine with PID $PID. It may have already stopped."
    rm ./.local/pt-engine.pid
  fi
else
  echo "pt-engine is not running (no PID file found)."
fi
