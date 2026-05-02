#!/usr/bin/env bash
set -euo pipefail

for cmd in rustc cargo jq git; do
  if command -v "$cmd" >/dev/null 2>&1; then
    echo "[ok] $cmd: $($cmd --version 2>/dev/null | head -n1)"
  else
    echo "[missing] $cmd"
  fi
done

for cmd in node aws docker; do
  if command -v "$cmd" >/dev/null 2>&1; then
    echo "[optional-ok] $cmd: $($cmd --version 2>/dev/null | head -n1)"
  else
    echo "[optional-missing] $cmd"
  fi
done
