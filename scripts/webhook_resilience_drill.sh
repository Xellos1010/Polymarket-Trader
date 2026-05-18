#!/usr/bin/env bash
set -euo pipefail

echo "[webhook-drill] running deterministic local webhook fallback/resilience tests"
cargo test -p pt-engine webhook_ -- --nocapture

if [[ -n "${WEBHOOK_DRILL_URL:-}" ]]; then
  if [[ -z "${WEBHOOK_DRILL_SECRET:-}" ]]; then
    echo "[webhook-drill] WEBHOOK_DRILL_URL set but WEBHOOK_DRILL_SECRET missing" >&2
    exit 2
  fi
  echo "[webhook-drill] probing configured webhook URL"
  curl --fail-with-body -sS \
    -X POST "${WEBHOOK_DRILL_URL%/}/tradingview" \
    -H "x-tv-secret: ${WEBHOOK_DRILL_SECRET}" \
    -H "Content-Type: application/json" \
    -d '{"order_action":"buy","contracts":"0.01","ticker":"BTC-USD"}'
  echo
fi

echo "[webhook-drill] complete"
