# Coinbase Sandbox ROI Flow

## Purpose

Document the current Coinbase-first sandbox flow as it actually exists today, the seams that already work, and the gaps that still block a cleaner paper ROI loop.

This document is intentionally local-first and sandbox-only. It does not authorize live trading.

## Current phase

Phase 1 preparation: sandbox trading and paper ROI readiness.

## Current flow

### 1. Strategy research and tuning

Current research tooling already available:

- `tools/coinbase_strategy_lab.py backtest`
- `tools/coinbase_strategy_lab.py overlap`
- `tools/coinbase_strategy_lab.py optimize`
- `tools/coinbase_strategy_lab.py dashboard`

Current outputs already available:

- JSON backtest or dashboard reports
- local HTML dashboards for visual review
- optimization rankings
- persistent journal data in `data/strategy_lab/trade_journal.sqlite`

### 2. Promotion into replay artifacts

Current promotion path:

- `scripts/promote_strategy_lab.sh`
- `tools/promote_strategy_lab.py`

Current promotion outputs:

- replay NDJSON frames for the Rust engine
- promotion JSON artifact with selected market and variant
- suggested replay-mode config patch

### 3. Replay acceptance checks

Current replay evidence tooling:

- `tools/replay_acceptance.py`
- `scripts/replay_acceptance.sh`
- `pt-cli verify-promoted`

Current evidence surfaces:

- replay frame validity
- promotion metadata consistency
- optional SQLite runtime evidence
- optional dashboard endpoint evidence

### 4. Coinbase workstation runtime

Current runtime entrypoint:

- `cargo run -p pt-cli -- coinbase up --config config/config.toml --mode paper`

Current workstation capabilities already visible in code:

- product refresh loop
- scanner loop
- order loop
- live arming and disarm guards
- strategy-lab import endpoint
- paper and replay mode resolution

### 5. TradingView-style source seam

Current source-adapter capability already present:

- `crates/pt-signal/src/lib.rs`
- `parse_tradingview_bias`
- `SignalFusionEngine`
- `schemas/tradingview-webhook.schema.json`

This is already a real source seam, but it is currently a bias-ingestion seam, not a full execution-strategy environment.

## What is already connected

### Connected today

- strategy lab can produce visual backtests and optimization outputs
- strategy lab can promote selected variants into replay artifacts
- replay acceptance tooling can validate promoted artifacts
- Coinbase workstation can import strategy-lab summaries through `/api/v1/strategy-lab/import`
- Coinbase workstation scanner logic can see imported summaries

### Important implementation detail

The current import-to-runtime effect is limited.

In `crates/pt-cli/src/coinbase.rs`, imported strategy-lab summaries currently influence runtime behavior through a shallow bias bump:

- `plugin_signal_for_product(...)`
- imported summaries add a small incremental plugin bias when `best_variants` matches the product

This means imports are not yet a full strategy handoff contract. They are currently a lightweight hint into the Coinbase scanner and strategy scoring path.

## What is not yet cleanly connected

### Gap 1. Strategy-lab import is not yet a strong runtime strategy contract

Current behavior suggests that imported strategy-lab results:

- are visible in the dashboard and product detail surfaces
- slightly affect plugin scoring
- do not yet appear to select a concrete execution profile, sizing profile, or variant-specific decision rule for paper mode

### Gap 2. Replay and paper evidence are adjacent, not unified

Replay promotion and replay acceptance are already present, but the paper workstation loop is still a separate runtime path.

That means ROI evidence still needs a clear operator story for:

- what was tuned
- what was promoted
- what was replay-accepted
- what was then run in paper mode
- how the paper outcome is tied back to the promoted strategy input

### Gap 3. TradingView is a source seam, not yet a strategy environment

The repo already supports TradingView-style bias ingestion and Pine tuning workflows, but it does not yet provide a single unified environment where:

- strategy definition
- visual backtest review
- AI parameter optimization
- stress testing
- paper execution handoff

all operate through one shared contract.

## Best current operator-safe workflow

Until the handoff seam is strengthened, the safest Coinbase sandbox workflow is:

1. tune and review candidates in strategy lab
2. promote a selected market and variant to replay artifacts
3. run replay acceptance checks
4. run the Coinbase workstation in paper mode
5. import the strategy-lab summary for operator visibility
6. treat paper results as a separate evidence layer that still requires attribution and reconciliation

## Recommended first implementation target

The best first Coinbase code-bearing slice should make strategy-lab output materially affect Coinbase paper behavior in a bounded way.

### Preferred target

Define a stronger handoff contract between strategy-lab output and Coinbase paper execution.

Expected shape:

- selected market
- selected variant
- expected score or edge inputs
- optional sizing hints
- traceable import metadata
- clear runtime visibility of which promoted artifact is active

### Why this target first

- it builds on code that already exists
- it improves traceability for sandbox ROI claims
- it avoids premature live-mode work
- it supports later TradingView-style strategy inputs through the same seam

## Acceptance criteria for that future slice

1. A promoted or imported strategy artifact can be traced to paper runtime behavior.
2. Paper-mode operator surfaces show which strategy artifact is active.
3. ROI evidence can name the strategy-lab source artifact that informed the paper run.
4. The slice remains sandbox and paper only.

## Validation expectations for that future slice

- `cargo check --workspace`
- `cargo test --workspace`
- strategy-lab backtest command
- promotion command
- replay acceptance command
- Coinbase paper runtime smoke check
- dashboard or SQLite evidence showing imported artifact visibility

## Guardrails

- no live-mode activation
- no live credentials
- no risk-cap increases
- no claims that backtest results guarantee paper or live profitability
