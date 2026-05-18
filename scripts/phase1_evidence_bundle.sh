#!/usr/bin/env bash
set -euo pipefail

SCHEMA_VERSION=1

usage() {
  cat >&2 <<'EOF'
usage: ./scripts/phase1_evidence_bundle.sh \
  --bundle-dir <dir> \
  --run-label <label> \
  --replay <replay.ndjson> \
  [--promotion <promotion.json>] \
  [--sqlite <engine.sqlite>] \
  [--paper-soak <paper-soak.json>] \
  [--metrics <metrics.json>]
EOF
  exit 1
}

BUNDLE_DIR=""
RUN_LABEL=""
REPLAY_PATH=""
PROMOTION_PATH=""
SQLITE_PATH=""
PAPER_SOAK_PATH=""
METRICS_PATH=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --bundle-dir)
      BUNDLE_DIR="${2:-}"
      shift 2
      ;;
    --run-label)
      RUN_LABEL="${2:-}"
      shift 2
      ;;
    --replay)
      REPLAY_PATH="${2:-}"
      shift 2
      ;;
    --promotion)
      PROMOTION_PATH="${2:-}"
      shift 2
      ;;
    --sqlite)
      SQLITE_PATH="${2:-}"
      shift 2
      ;;
    --paper-soak)
      PAPER_SOAK_PATH="${2:-}"
      shift 2
      ;;
    --metrics)
      METRICS_PATH="${2:-}"
      shift 2
      ;;
    *)
      usage
      ;;
  esac
done

if [[ -z "$BUNDLE_DIR" || -z "$RUN_LABEL" || -z "$REPLAY_PATH" ]]; then
  usage
fi

mkdir -p "$BUNDLE_DIR"
RUN_DIR="$BUNDLE_DIR/$RUN_LABEL"
mkdir -p "$RUN_DIR"

REPLAY_CMD=(python3 tools/replay_acceptance.py --replay "$REPLAY_PATH" --out "$RUN_DIR/replay_acceptance.json")
if [[ -n "$PROMOTION_PATH" ]]; then
  REPLAY_CMD+=(--promotion "$PROMOTION_PATH")
fi
if [[ -n "$SQLITE_PATH" ]]; then
  REPLAY_CMD+=(--sqlite "$SQLITE_PATH" --min-snapshots 1 --min-risk-events 1)
fi

"${REPLAY_CMD[@]}"

if [[ -n "$PAPER_SOAK_PATH" ]]; then
  cp "$PAPER_SOAK_PATH" "$RUN_DIR/paper_soak.json"
fi

if [[ -n "$METRICS_PATH" ]]; then
  cp "$METRICS_PATH" "$RUN_DIR/metrics.json"
fi

RUN_DIR_ENV="$RUN_DIR" \
RUN_LABEL_ENV="$RUN_LABEL" \
REPLAY_PATH_ENV="$REPLAY_PATH" \
PROMOTION_PATH_ENV="$PROMOTION_PATH" \
SQLITE_PATH_ENV="$SQLITE_PATH" \
SCHEMA_VERSION_ENV="$SCHEMA_VERSION" \
python3 - <<'PY'
import datetime as dt
import json
import os
from pathlib import Path

run_dir = Path(os.environ["RUN_DIR_ENV"])
manifest = {
    "schema_version": int(os.environ["SCHEMA_VERSION_ENV"]),
    "phase": "Phase 1",
    "run_label": os.environ["RUN_LABEL_ENV"],
    "generated_at": dt.datetime.now(tz=dt.timezone.utc).isoformat(),
    "artifacts": {
        "replay_source": os.environ["REPLAY_PATH_ENV"],
        "replay_acceptance": str(run_dir / "replay_acceptance.json"),
        "promotion_source": os.environ["PROMOTION_PATH_ENV"] or None,
        "sqlite_source": os.environ["SQLITE_PATH_ENV"] or None,
        "paper_soak": str(run_dir / "paper_soak.json") if (run_dir / "paper_soak.json").exists() else None,
        "metrics": str(run_dir / "metrics.json") if (run_dir / "metrics.json").exists() else None,
    },
}
(run_dir / "manifest.json").write_text(json.dumps(manifest, indent=2, sort_keys=True), encoding="utf-8")
print(json.dumps(manifest, indent=2, sort_keys=True))
PY

echo "phase1 evidence bundle created: $RUN_DIR"
