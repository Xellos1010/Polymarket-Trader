#!/usr/bin/env bash
set -euo pipefail

RUN_FRONTEND="${RUN_FRONTEND:-0}"
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

log_stage() {
  printf '\n[local-validation] %s\n' "$1"
}

run_stage() {
  local label="$1"
  shift
  log_stage "$label"
  "$@"
}

require_file() {
  local path="$1"
  local source_path="$2"
  if [[ ! -f "$path" && -f "$source_path" ]]; then
    cp "$source_path" "$path"
    printf '[local-validation] bootstrapped %s from %s\n' "$path" "$source_path"
  fi
}

require_file "config/config.toml" "config/config.example.toml"
require_file "config/coinbase_strategy_lab.json" "config/coinbase_strategy_lab.example.json"
require_file "config/prompt_bundle.json" "config/prompt_bundle.example.json"

run_stage "cargo fmt" cargo fmt --all
run_stage "cargo check" cargo check --workspace
run_stage "cargo clippy" cargo clippy --workspace --all-targets --all-features -- -D warnings
run_stage "cargo test" cargo test --workspace
run_stage "cargo build" cargo build --workspace
run_stage "cargo audit" cargo audit
run_stage "generate sbom" ./scripts/generate_sbom.sh artifacts
run_stage "strategy lab backtest" python3 tools/coinbase_strategy_lab.py backtest --config config/coinbase_strategy_lab.json
run_stage "strategy lab overlap" python3 tools/coinbase_strategy_lab.py overlap --config config/coinbase_strategy_lab.json --auto-discovery
run_stage "strategy lab optimize" python3 tools/coinbase_strategy_lab.py optimize --config config/coinbase_strategy_lab.json

if [[ "$RUN_FRONTEND" == "1" ]]; then
  log_stage "frontend install and test"
  pushd crates/pt-dashboard/frontend >/dev/null
  npm install
  npm test
  npm run build
  popd >/dev/null
fi

run_stage "runtime smoke" cargo run -p pt-cli -- run --config config/config.toml
run_stage "paper soak" ./scripts/paper_soak.sh 86400 30 config/config.toml

log_stage "completed"
