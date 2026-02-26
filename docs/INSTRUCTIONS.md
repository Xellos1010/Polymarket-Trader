# Instructions

## Local-First Rule

Do not run CI/CD or deployment automation until these local checks pass:

1. `cargo check --workspace`
2. `cargo test --workspace`
3. strategy-lab validation (`backtest`, `overlap`, `optimize`, `dashboard`)
4. replay/paper verification for any promoted candidate

## Core Local Workflow

1. Copy configs:
```bash
cp config/config.example.toml config/config.toml
cp config/coinbase_strategy_lab.example.json config/coinbase_strategy_lab.json
cp config/prompt_bundle.example.json config/prompt_bundle.json
```

2. Run strategy dashboard:
```bash
python3 tools/coinbase_strategy_lab.py dashboard --config config/coinbase_strategy_lab.json --serve 9090
```

3. Promote selected result to replay artifact:
```bash
./scripts/promote_strategy_lab.sh data/strategy_lab/<dashboard-or-backtest>.json BTC-USD sma_baseline
```

4. Apply replay settings in `config/config.toml`:
- `engine.mode = "replay"`
- `engine.replay_path = "data/replay/strategy_lab_promoted.ndjson"`

5. Run engine locally:
```bash
cargo run -p pt-cli -- run --config config/config.toml
```

## Strategy Variant Plugins

Configured in `backtest.variants[*].plugins`:

- `external_bias_file`: load `{idx,bias}` or `{ts_ms,bias}` series from JSON/CSV.
- `momentum_bias`: tanh-scaled lookback return.
- `rsi_bias`: directional bias from RSI extremes.

Use `bias_gain` per variant to control plugin influence.

## Context Persistence

- Save session checkpoint:
```bash
./scripts/save_context.sh "note" docs/SESSION_CONTEXT.md config/config.toml
```

- Export external AI bundle:
```bash
./scripts/export_prompt_bundle.sh
```
