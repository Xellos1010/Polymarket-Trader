#!/usr/bin/env bash
set -euo pipefail

OUT="${1:-data/output/prompt_bundle.min.txt}"
CFG="${2:-config/prompt_bundle.json}"
MANIFEST="${3:-data/output/prompt_bundle.manifest.json}"

python3 tools/export_prompt_bundle.py \
  --root . \
  --out "$OUT" \
  --config "$CFG" \
  --include-untracked \
  --manifest "$MANIFEST"

echo "prompt bundle written: $OUT"
echo "manifest written: $MANIFEST"
