# Instructions

## Local-First Rule

Do not run CI/CD or deployment automation until the canonical local validation ladder passes:

```bash
./scripts/local_validation_ladder.sh
```

Reference guide:
- `docs/LOCAL_VALIDATION.md`

The minimum ladder includes:
1. `cargo fmt --all`
2. `cargo check --workspace`
3. `cargo clippy --workspace --all-targets --all-features -- -D warnings`
4. `cargo test --workspace`
5. `cargo build --workspace`
6. `cargo audit`
7. `./scripts/generate_sbom.sh artifacts`
8. strategy-lab validation (`backtest`, `overlap`, `optimize`)
9. runtime smoke and paper verification in sandbox mode only

## Core Local Workflow

1. Copy configs:
```bash
cp config/config.example.toml config/config.toml
cp config/coinbase_strategy_lab.example.json config/coinbase_strategy_lab.json
cp config/prompt_bundle.example.json config/prompt_bundle.json
```

2. Run Coinbase workstation:
```bash
cargo run -p pt-cli -- coinbase up --config config/config.toml --mode paper
```

3. Run strategy dashboard when you need offline backtest or optimization output:
```bash
python3 tools/coinbase_strategy_lab.py dashboard --config config/coinbase_strategy_lab.json --serve 9090
```

4. Promote selected result to replay artifact:
```bash
./scripts/promote_strategy_lab.sh data/strategy_lab/<dashboard-or-backtest>.json BTC-USD sma_baseline
```

5. Apply replay settings in `config/config.toml`:
- `engine.mode = "replay"`
- `engine.replay_path = "data/replay/strategy_lab_promoted.ndjson"`

6. Run Coinbase workstation in replay mode:
```bash
cargo run -p pt-cli -- coinbase up --config config/config.toml --mode replay
```

## Strategy Variant Plugins

Configured in `backtest.variants[*].plugins`:

- `external_bias_file`: load `{idx,bias}` or `{ts_ms,bias}` series from JSON or CSV.
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
