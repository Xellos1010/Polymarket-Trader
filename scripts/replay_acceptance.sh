#!/usr/bin/env bash
set -euo pipefail

REPLAY="${1:-}"
PROMOTION="${2:-}"
SQLITE_PATH="${3:-}"

if [[ -z "$REPLAY" ]]; then
  echo "usage: $0 <replay.ndjson> [promotion.json] [engine.sqlite]" >&2
  exit 1
fi

CMD=(python3 tools/replay_acceptance.py --replay "$REPLAY")
if [[ -n "$PROMOTION" ]]; then
  CMD+=(--promotion "$PROMOTION")
fi
if [[ -n "$SQLITE_PATH" ]]; then
  CMD+=(--sqlite "$SQLITE_PATH" --min-snapshots 1 --min-risk-events 1)
fi

"${CMD[@]}"
