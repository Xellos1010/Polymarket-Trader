#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/dist}"
VERSION="${2:-$(date +%Y%m%d-%H%M%S)}"
BUNDLE_DIR="$OUT_DIR/polymarket-trader-$VERSION"

mkdir -p "$OUT_DIR"
rm -rf "$BUNDLE_DIR"
mkdir -p "$BUNDLE_DIR/bin" "$BUNDLE_DIR/config" "$BUNDLE_DIR/scripts"

cd "$ROOT_DIR"

echo "[build] cargo build --release -p pt-cli"
cargo build --release -p pt-cli

cp target/release/pt-cli "$BUNDLE_DIR/bin/pt-cli"
cp config/config.example.toml "$BUNDLE_DIR/config/config.toml"
cp scripts/pt-engine.release.service "$BUNDLE_DIR/scripts/pt-engine.service"
cp -r docs "$BUNDLE_DIR/docs"

ARCHIVE="$OUT_DIR/polymarket-trader-$VERSION.tar.gz"
tar -C "$OUT_DIR" -czf "$ARCHIVE" "polymarket-trader-$VERSION"

echo "[build] created $ARCHIVE"
