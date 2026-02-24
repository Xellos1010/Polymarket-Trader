#!/usr/bin/env bash
set -euo pipefail

NOTE="${1:-}"
OUT="${2:-docs/SESSION_CONTEXT.md}"
CFG="${3:-config/config.toml}"

if [[ -n "$NOTE" ]]; then
  cargo run -p pt-cli -- save-context --config "$CFG" --out "$OUT" --note "$NOTE"
else
  cargo run -p pt-cli -- save-context --config "$CFG" --out "$OUT"
fi

echo "context saved to: $OUT"
