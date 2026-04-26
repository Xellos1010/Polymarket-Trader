# Implementation Work Orders

## WO-001: Run the Rust validation ladder and record evidence

- Goal: establish current Phase 0 readiness in a Rust-enabled environment before any deployment or live-readiness work.
- Scope:
  - run `cargo fmt --all`
  - run `cargo check --workspace`
  - run `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - run `cargo test --workspace`
  - run `cargo build --workspace`
  - run `cargo audit`
  - generate SBOM artifacts
- Acceptance:
  - command results are captured
  - failures become small follow-up issues or PRs
  - no config or credential changes are required

## WO-002: Verify promoted strategy-lab output in replay mode

- Goal: prove the promotion path from strategy-lab output into replay is deterministic and reviewable.
- Scope:
  - generate or reuse promoted replay NDJSON
  - run replay acceptance checks
  - run `pt-cli` replay against the promoted artifact
  - capture execution, risk, and attribution evidence
- Acceptance:
  - replay runs complete without policy surprises
  - evidence includes fills, halts, and cost attribution when available
  - at least one reusable fixture path is documented for follow-up automation

## WO-003: Complete repeatable paper-soak evidence

- Goal: demonstrate paper-mode behavior is positive after modeled costs and remains inside hard risk limits.
- Scope:
  - run paper soak for the configured duration
  - capture market-level and run-level attribution
  - verify daily loss, market notional, total open notional, and unhedged delta remain inside limits
  - repeat across at least three independent runs
- Acceptance:
  - net simulated PnL remains positive after modeled costs
  - no unexpected auto-halt occurs
  - repeatability is shown across multiple runs rather than a single good window

## WO-004: Add fixture-backed dashboard visibility for sandbox gates

- Goal: improve operator visibility for replay and paper evidence without implying unsupported backend functionality.
- Scope:
  - add fixture-safe UI or tests only where they support current API surfaces
  - cover replay status, paper evidence summaries, or risk counters already present or explicitly mocked
  - avoid speculative workspaces that imply live governance or unsupported endpoints
- Acceptance:
  - frontend changes stay isolated and test-backed
  - no new backend contracts are implied without matching implementation
  - work is suitable for a separate frontend-only PR
