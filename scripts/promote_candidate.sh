#!/usr/bin/env bash
set -euo pipefail

REPORT="${1:-data/tuning/pine_tuning_results.json}"
OUT="${2:-data/tuning/promoted_candidate.json}"
ASSET="${3:-BTC}"
HORIZON="${4:-15m}"

python3 tools/promote_candidate.py \
  --report "$REPORT" \
  --out "$OUT" \
  --asset "$ASSET" \
  --horizon "$HORIZON"

echo "promoted candidate written to: $OUT"
cat "$OUT"
