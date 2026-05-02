#!/usr/bin/env bash
set -euo pipefail

DURATION_SECS="${1:-86400}"
INTERVAL_SECS="${2:-30}"
CONFIG_PATH="${3:-config/config.toml}"
OUT_DIR="${4:-data/soak}"
STARTUP_TIMEOUT_SECS="${5:-120}"

mkdir -p "$OUT_DIR"
TS="$(date +%Y%m%d-%H%M%S)"
LOG_FILE="$OUT_DIR/paper-soak-$TS.log"
REPORT_FILE="$OUT_DIR/paper-soak-$TS.json"

start_epoch="$(date +%s)"
end_epoch="$((start_epoch + DURATION_SECS))"

if [[ -x "target/debug/pt-cli" ]]; then
  ENGINE_CMD=(target/debug/pt-cli run --config "$CONFIG_PATH")
else
  ENGINE_CMD=(cargo run -p pt-cli -- run --config "$CONFIG_PATH")
fi

"${ENGINE_CMD[@]}" >"$LOG_FILE" 2>&1 &
ENGINE_PID=$!

cleanup() {
  kill "$ENGINE_PID" >/dev/null 2>&1 || true
  wait "$ENGINE_PID" >/dev/null 2>&1 || true
}
trap cleanup EXIT

samples=0
failed_probes=0
halt_count=0
max_abs_delta=0
last_kill_switch="unknown"

startup_deadline="$((start_epoch + STARTUP_TIMEOUT_SECS))"
until curl -fsS http://127.0.0.1:8080/health >/dev/null 2>&1; do
  if [[ "$(date +%s)" -ge "$startup_deadline" ]]; then
    echo "engine did not become healthy within startup timeout (${STARTUP_TIMEOUT_SECS}s)" >&2
    exit 1
  fi
  sleep 2
done

while [[ "$(date +%s)" -lt "$end_epoch" ]]; do
  health_json="$(curl -fsS http://127.0.0.1:8080/health || true)"
  risk_json="$(curl -fsS http://127.0.0.1:8080/state/risk || true)"

  if [[ -z "$health_json" || -z "$risk_json" ]]; then
    failed_probes=$((failed_probes + 1))
    sleep "$INTERVAL_SECS"
    continue
  fi

  samples=$((samples + 1))

  if command -v jq >/dev/null 2>&1; then
    kill_switch="$(echo "$health_json" | jq -r '.kill_switch // "unknown"')"
    delta_raw="$(echo "$risk_json" | jq -r '.unhedged_delta // 0')"
  else
    kill_switch="unknown"
    delta_raw="0"
  fi

  last_kill_switch="$kill_switch"
  if [[ "$kill_switch" != "Running" ]]; then
    halt_count=$((halt_count + 1))
  fi

  abs_delta="$(python3 - <<PY
v=float("$delta_raw")
print(abs(v))
PY
)"

  max_abs_delta="$(python3 - <<PY
a=float("$max_abs_delta")
b=float("$abs_delta")
print(a if a>b else b)
PY
)"

  sleep "$INTERVAL_SECS"
done

pass=true
reason="ok"
if [[ "$failed_probes" -gt 0 ]]; then
  pass=false
  reason="failed_probes"
elif [[ "$halt_count" -gt 0 ]]; then
  pass=false
  reason="kill_switch_not_running"
fi

cat > "$REPORT_FILE" <<JSON
{
  "start_epoch": $start_epoch,
  "duration_secs": $DURATION_SECS,
  "interval_secs": $INTERVAL_SECS,
  "samples": $samples,
  "failed_probes": $failed_probes,
  "halt_count": $halt_count,
  "max_abs_unhedged_delta": $max_abs_delta,
  "last_kill_switch": "$last_kill_switch",
  "pass": $pass,
  "reason": "$reason",
  "log_file": "$LOG_FILE"
}
JSON

echo "soak report: $REPORT_FILE"
cat "$REPORT_FILE"

if [[ "$pass" != "true" ]]; then
  exit 1
fi
