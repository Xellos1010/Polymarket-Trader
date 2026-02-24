#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if ! command -v git >/dev/null 2>&1; then
  echo 'git is required' >&2
  exit 1
fi

cd "$ROOT_DIR"
git config core.hooksPath .githooks

echo 'git hooks installed:'
echo '  - pre-commit: fmt + clippy'
echo '  - pre-push: test suite'
