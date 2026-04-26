# Strategy Lab (Rust-First)

Primary implementation is now Rust (`crates/pt-strategy-lab`) with Axum dashboard and CLI integration.

## Rust Commands

Serve dashboard/API:
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
cargo run -p pt-cli -- strategy-lab-serve --bind 127.0.0.1:9090 --db data/strategy_lab/strategy_lab.sqlite
```

Run backtest (next-bar fill model):

```bash
cargo run -p pt-cli -- strategy-backtest \
  --product BTC-USD \
  --granularity-sec 300 \
  --limit 600 \
  --out data/output/strategy_backtest_report.json
```

Run optimizer (random search + walk-forward):

```bash
cargo run -p pt-cli -- strategy-optimize \
  --product BTC-USD \
  --granularity-sec 300 \
  --limit 600 \
  --iterations 200 \
  --walk-forward-splits 4 \
  --out data/output/strategy_optimize_report.json
```

Load/save strategy profile versions:

```bash
cargo run -p pt-cli -- strategy-profile-load --profile-id default --out data/output/strategy_profile_default.json
cargo run -p pt-cli -- strategy-profile-save --path data/output/strategy_profile_default.json --note "manual tuning"
```

## Indicator Stack

Implemented indicator modules:
- MA regime (`EMA/SMA/WMA/HMA/DEMA/TEMA/VWMA/RMA/ZLEMA`)
- RSI
- Fibonacci BB (VWMA basis + fib multiplier)
- Ichimoku (conversion/base/span B + displacement)
- MACD
- ADX
- ATR
- Volume pressure/spike
- VWAP deviation
- StochRSI

Fusion model:
- weighted confidence score in `[-1,1]`
- buy threshold default `+0.60`
- sell threshold default `-0.60`
- minimum confluence default `2`
- bull/bear/neutral regime gating

## Rust API Endpoints

- `GET /lab/state/profile`
- `POST /lab/profile/save`
- `POST /lab/profile/load`
- `POST /lab/backtest/run`
- `POST /lab/optimize/run`
- `GET /lab/state/indicators`
- `GET /lab/state/signals`
- `GET /lab/state/regime`
- `GET /lab/state/runs`

## SQLite Store

Default: `data/strategy_lab/strategy_lab.sqlite`

Key tables:
- `strategy_profiles`
- `strategy_profile_versions`
- `strategy_runs`
- `indicator_series`
- `signal_series`
- `regime_series`
- `paper_endpoint_reports`

## Python Fallback

Legacy Python lab remains available for overlap experiments and cross-checking:

```bash
python3 tools/coinbase_strategy_lab.py dashboard --config config/coinbase_strategy_lab.json --serve 9090
```

## Promotion Flow
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

1. Tune in Rust strategy lab.
2. Save best profile/version.
3. Produce promoted candidate and replay artifacts.
4. Verify with `pt-cli verify-promoted`.
5. Run paper soak before tiny live.
