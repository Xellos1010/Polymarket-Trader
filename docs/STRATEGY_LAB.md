# Coinbase Strategy Lab

Local-first visualization, strategy modification, and backtesting for Coinbase markets.

## Files

- Tool: `tools/coinbase_strategy_lab.py`
- Promotion tool: `tools/promote_strategy_lab.py`
- Replay acceptance tool: `tools/replay_acceptance.py`
- Promotion wrapper: `scripts/promote_strategy_lab.sh`
- Replay acceptance wrapper: `scripts/replay_acceptance.sh`
- Config example: `config/coinbase_strategy_lab.example.json`
- Config schema: `schemas/coinbase_strategy_lab.schema.json`
- Output folder: `data/strategy_lab/`
- Journal DB (default): `data/strategy_lab/trade_journal.sqlite`

## Setup

```bash
cp config/coinbase_strategy_lab.example.json config/coinbase_strategy_lab.json
```

## Modes

### 1) Backtest

```bash
python3 tools/coinbase_strategy_lab.py backtest --config config/coinbase_strategy_lab.json
```

### 2) Listing overlap (candle aligned)

```bash
python3 tools/coinbase_strategy_lab.py overlap --config config/coinbase_strategy_lab.json
```

The overlap chart aligns each asset series at its anchor candle (`anchor_time`) and compares forward candle movement (`+1`, `+3`, `+10`) independent of wall-clock timestamps.

### 3) Optimization

```bash
python3 tools/coinbase_strategy_lab.py optimize --config config/coinbase_strategy_lab.json
```

Objective:

`score = avg_return - drawdown_penalty*avg_drawdown - turnover_penalty*trade_rate`

### 4) Unified dashboard

```bash
python3 tools/coinbase_strategy_lab.py dashboard --config config/coinbase_strategy_lab.json
```

Serve locally:

```bash
python3 tools/coinbase_strategy_lab.py dashboard --config config/coinbase_strategy_lab.json --serve 9090
```

## Strategy Variants and Plugins

Backtest variants support baseline and plugin-driven comparisons side-by-side:

- `external_bias_file` (for Pine/AI generated bias series)
- `momentum_bias`
- `rsi_bias`

Each variant defines:

- `name`
- `bias_gain`
- `plugins[]`

Use `bias_gain` to control how strongly plugin bias shifts SMA baseline positioning.

## Listing Theory Auto-Discovery

`overlap.auto_discovery` can auto-select recent Coinbase listings by scanning products and identifying assets whose first candle inside a rolling discovery window appears after the window start threshold.

Useful fields:

- `lookback_days`
- `discovery_granularity_sec`
- `max_products_scan`
- `max_results`
- `quote_currencies`

## Persistent Journal and Attribution

The lab writes run and trade results into SQLite when `journal.enabled=true`:

- `lab_runs`
- `market_results`
- `trade_fills`

Dashboard/backtest pages include aggregated per-market/per-variant attribution from this journal.

## Fast Overrides

```bash
python3 tools/coinbase_strategy_lab.py backtest \
  --config config/coinbase_strategy_lab.json \
  --markets BTC-USD,ETH-USD,SOL-USD,XRP-USD \
  --short-window 7 \
  --long-window 34 \
  --fee-bps 6 \
  --slippage-bps 2 \
  --limit 500
```

Disable journal writes for quick dry-runs:

```bash
python3 tools/coinbase_strategy_lab.py backtest --config config/coinbase_strategy_lab.json --disable-journal
```

## Promote To Rust Replay

Generate replay artifact from a strategy-lab report:

```bash
./scripts/promote_strategy_lab.sh data/strategy_lab/<dashboard-or-backtest>.json BTC-USD sma_baseline
```

Outputs:

- `data/replay/strategy_lab_promoted.ndjson`
- `data/tuning/strategy_lab_promoted.json`

Then set in `config/config.toml`:

- `engine.mode = "replay"`
- `engine.replay_path = "data/replay/strategy_lab_promoted.ndjson"`

## Replay Acceptance

After promotion, validate the replay artifact before or after running the Rust engine:

```bash
./scripts/replay_acceptance.sh data/replay/strategy_lab_promoted.ndjson data/tuning/strategy_lab_promoted.json
```

The validator checks that replay frames are parseable, chronologically ordered, bounded, and internally consistent. It verifies snapshot fields such as market id, token id, bid, ask, spread, liquidity, timestamp, and replay bias.

Optional runtime evidence can be checked from the engine SQLite store after a replay run:

```bash
python3 tools/replay_acceptance.py \
  --replay data/replay/strategy_lab_promoted.ndjson \
  --promotion data/tuning/strategy_lab_promoted.json \
  --sqlite data/output/pt.sqlite \
  --min-risk-events 1 \
  --min-snapshots 1
```

Optional dashboard evidence can be checked while the local dashboard is running:

```bash
python3 tools/replay_acceptance.py \
  --replay data/replay/strategy_lab_promoted.ndjson \
  --dashboard-url http://127.0.0.1:8080
```

## Pine + AI Fine-Tuning Loop

Use strategy-lab for market-level verification and the Pine pipeline for parameter search:

1. Extract Pine params: `pt-cli pine-params`
2. Run tuning: `pt-cli tune-pine`
3. Score candidates: `tools/evaluate_candidate.py`
4. Feed bias series into `external_bias_file` plugin
5. Compare variant performance in dashboard/backtest

Reference: `docs/PINE_TUNING.md`
