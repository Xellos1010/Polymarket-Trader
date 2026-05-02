#!/usr/bin/env bash
set -euo pipefail

REPORT="${1:-}"
if [[ -z "$REPORT" ]]; then
  echo "usage: $0 <strategy_lab_report_json> [market] [variant] [out_replay] [out_promotion]" >&2
  exit 1
fi

MARKET="${2:-}"
VARIANT="${3:-}"
OUT_REPLAY="${4:-data/replay/strategy_lab_promoted.ndjson}"
OUT_PROMOTION="${5:-data/tuning/strategy_lab_promoted.json}"

CMD=(python3 tools/promote_strategy_lab.py --report "$REPORT" --out-replay "$OUT_REPLAY" --out-promotion "$OUT_PROMOTION")
if [[ -n "$MARKET" ]]; then
  CMD+=(--market "$MARKET")
fi
if [[ -n "$VARIANT" ]]; then
  CMD+=(--variant "$VARIANT")
fi

"${CMD[@]}"

echo "promoted replay written to: $OUT_REPLAY"
echo "promotion artifact written to: $OUT_PROMOTION"
cat "$OUT_PROMOTION"
