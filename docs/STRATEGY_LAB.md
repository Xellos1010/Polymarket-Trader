# Strategy Lab (Rust-First)

Primary implementation is now Rust (`crates/pt-strategy-lab`) with Axum dashboard and CLI integration.

## Rust Commands

Serve dashboard/API:

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

1. Tune in Rust strategy lab.
2. Save best profile/version.
3. Produce promoted candidate and replay artifacts.
4. Verify with `pt-cli verify-promoted`.
5. Run paper soak before tiny live.
