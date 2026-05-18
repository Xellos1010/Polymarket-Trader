#!/usr/bin/env bash
# Codespace / devcontainer bootstrap — sandbox configs only; no live credentials.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

log() { printf '\n[devcontainer] %s\n' "$*"; }

log "Rust toolchain components"
rustup component add rustfmt clippy

log "System packages (validation ladder)"
export DEBIAN_FRONTEND=noninteractive
sudo apt-get update -qq
sudo apt-get install -y -qq jq curl python3 python3-venv

log "Node / pnpm (Corepack)"
corepack enable
corepack prepare pnpm@10.11.0 --activate

log "pnpm install (workspace root)"
pnpm install

log "Optional security / SBOM tooling"
if ! command -v cargo-audit >/dev/null 2>&1; then
  cargo install cargo-audit --locked
fi
if ! command -v cargo-cyclonedx >/dev/null 2>&1; then
  cargo install cargo-cyclonedx --locked || true
fi

bootstrap_if_missing() {
  local dest="$1"
  local example="$2"
  if [[ ! -f "$dest" && -f "$example" ]]; then
    cp "$example" "$dest"
    log "bootstrapped $dest from $example"
  fi
}

log "Sandbox config bootstrap (examples only)"
bootstrap_if_missing "config/config.toml" "config/config.example.toml"
bootstrap_if_missing "config/coinbase_strategy_lab.json" "config/coinbase_strategy_lab.example.json"
bootstrap_if_missing "config/prompt_bundle.json" "config/prompt_bundle.example.json"

log "Warm cargo metadata (speeds first IDE check)"
cargo fetch --locked 2>/dev/null || cargo fetch

log "Done. Quick checks:"
echo "  pnpm exec nx show projects"
echo "  pnpm verify          # fmt, check, clippy, test, build"
echo "  pnpm exec nx run polymarket-trader:dev   # backend :8080 + Vite :5173"
echo "Secrets: use Codespaces secrets or a local .env (never commit keys)."
