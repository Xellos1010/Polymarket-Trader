#!/usr/bin/env bash
set -euo pipefail

CONFIG_PATH="${1:-config/config.toml}"
TIMEOUT_MS="${2:-3000}"

get_toml_value() {
  local section="$1"
  local key="$2"
  awk -v section="$section" -v key="$key" '
    $0 ~ "^\\[" section "\\]" { in_section=1; next }
    $0 ~ "^\\[" { in_section=0 }
    in_section {
      if ($0 ~ "^[[:space:]]*" key "[[:space:]]*=") {
        split($0, a, "=")
        val=a[2]
        gsub(/^[[:space:]]+|[[:space:]]+$/, "", val)
        gsub(/"/, "", val)
        print val
        exit
      }
    }
  ' "$CONFIG_PATH"
}

compare_le() {
  local value="$1"
  local max="$2"
  awk -v v="$value" -v m="$max" 'BEGIN { exit !(v <= m) }'
}

engine_mode="$(get_toml_value engine mode || true)"

errors=()
if [[ "$engine_mode" != "live" ]]; then
  errors+=("engine.mode must be 'live' for pilot, got '$engine_mode'")
fi

check_limit() {
  local key="$1"
  local max_allowed="$2"
  local value
  value="$(get_toml_value risk "$key" || true)"
  if [[ -z "$value" ]]; then
    errors+=("risk.$key missing")
    return
  fi
  if ! compare_le "$value" "$max_allowed"; then
    errors+=("risk.$key=$value exceeds pilot max $max_allowed")
  fi
}

check_limit daily_loss_limit_pct 0.01
check_limit max_notional_per_market 2.5
check_limit max_total_open_notional 10
check_limit max_markets_quoted_simultaneously 1
check_limit max_unhedged_delta 10

if [[ ${#errors[@]} -gt 0 ]]; then
  echo "pilot config check failed:"
  for e in "${errors[@]}"; do
    echo "- $e"
  done
  exit 1
fi

echo "pilot config limits: OK"

cargo run -p pt-cli -- preflight-live --config "$CONFIG_PATH" --timeout-ms "$TIMEOUT_MS"

echo "tiny live pilot prechecks passed"
echo "next: start engine, monitor dashboard, and keep /ops/halt ready"
