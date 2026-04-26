#!/usr/bin/env bash
set -euo pipefail

CONFIG="${1:-config/sandbox_optimizer_cycle.json}"

python3 tools/sandbox_optimizer_cycle.py --config "$CONFIG"
