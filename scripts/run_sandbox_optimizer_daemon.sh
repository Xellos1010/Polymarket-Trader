#!/usr/bin/env bash
set -euo pipefail

CONFIG="${1:-config/sandbox_optimizer_cycle.json}"
INTERVAL_SECS="${2:-3600}"

python3 tools/sandbox_optimizer_daemon.py --config "$CONFIG" --interval-secs "$INTERVAL_SECS"
