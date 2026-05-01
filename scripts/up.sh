#!/usr/bin/env bash
set -euo pipefail

mkdir -p ./.local

echo "Starting pt-engine..."
cargo run -p pt-cli -- run --config config/config.toml &> ./.local/pt-engine.log &
PID=$!
echo $PID > ./.local/pt-engine.pid
echo "pt-engine started with PID $PID. Log available at .local/pt-engine.log"
