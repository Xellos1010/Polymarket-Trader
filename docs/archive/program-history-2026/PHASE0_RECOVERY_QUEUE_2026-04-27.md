# Phase 0 Compile Recovery Queue (2026-04-27)

## Phase
Phase 0: repo readiness

## Why this queue exists
Re-auditing the open PR stack, issue `#9`, recent CI runs, and the current `main` branch shows the repo is blocked before the next Phase 1 approval-queue runtime slice can safely continue.

Grounded findings:
- `crates/pt-cli/Cargo.toml` still contains a duplicate `chrono.workspace = true` entry.
- CI currently fails before meaningful Phase 1 validation because repo integrity is broken earlier in the ladder.
- Several Rust files on `main` contain duplicate imports, duplicate type blocks, or partially merged sections that prevent a reliable fmt/check/test run.
- A blanket restore from baseline commit `7da0bd8ba608f0f57e2edc83b7bf1f73cff955b1` would remove substantial newer intended work in multiple files.

## Safest next action
Merge the smallest blocker first, then recover compile integrity in narrow slices.
