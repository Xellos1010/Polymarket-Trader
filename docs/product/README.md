# Product Planning Bootstrap

This folder holds the smallest planning surface needed to move the repository through Phase 1 sandbox trading safely.

## Purpose

Use these files to queue small, reviewable tasks that improve replay validation, paper-mode repeatability, risk evidence, and operator visibility without changing runtime behavior.

## Files

- `IMPLEMENTATION_WORK_ORDERS.md` - PR-sized work orders for the next execution rounds.
- `FEATURE_TRACKER.json` - machine-readable tracker for the current Phase 1 queue.

## Operating boundaries

1. Replay and paper evidence stay ahead of product expansion.
2. These docs do not enable live trading, deployment, or risk-cap changes.
3. New planning artifacts should stay focused on Phase 0 and Phase 1 until replay and paper gates pass repeatedly.

## Current focus

- Run the local validation ladder in a Rust-enabled environment.
- Verify promoted strategy-lab outputs in `pt-cli` replay mode.
- Complete repeatable paper-soak evidence with risk counters and attribution.
- Add fixture-backed dashboard coverage only where it improves operator visibility for those gates.
