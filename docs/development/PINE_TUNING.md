# Pine Tuning + Rust Strategy Lab

Use Pine parameter extraction/tuning for candidate generation, then validate in the Rust strategy lab before replay/paper promotion.

## 1) Extract Pine Parameters

```bash
cargo run -p pt-cli -- pine-params \
  --path pine-scripts/V1-RSI-Fibonacci-BB-Ichimoku-Pyramid-Date-Range-Enabled-v6 \
  --out data/tuning/pine_params.json
```

## 2) Generate Candidate Sets

```bash
cargo run -p pt-cli -- tune-pine \
  --path pine-scripts/V1-RSI-Fibonacci-BB-Ichimoku-Pyramid-Date-Range-Enabled-v6 \
  --iterations 300 \
  --top-k 25 \
  --evaluate-cmd "python3 tools/evaluate_candidate.py" \
  --out data/tuning/pine_tuning_results.json
```

## 3) Promote Candidate

```bash
./scripts/promote_candidate.sh \
  data/tuning/pine_tuning_results.json \
  data/tuning/promoted_candidate.json \
  BTC 15m
```

## 4) Validate in strategy lab (Python driver)

Backtest, optimize, and optional local static server (same commands as `docs/LOCAL_VALIDATION.md`):

```bash
cp config/coinbase_strategy_lab.example.json config/coinbase_strategy_lab.json
python3 tools/coinbase_strategy_lab.py backtest --config config/coinbase_strategy_lab.json
python3 tools/coinbase_strategy_lab.py optimize --config config/coinbase_strategy_lab.json
python3 tools/coinbase_strategy_lab.py dashboard --config config/coinbase_strategy_lab.json --serve 9090
```

The Rust crate `pt-strategy-lab` hosts `/lab/*` HTTP handlers for tests and embedding; there is no `pt-cli strategy-lab-serve` today.

## 5) Promotion gate

```bash
./scripts/replay_acceptance.sh data/replay/strategy_lab_promoted.ndjson data/tuning/promoted_candidate.json data/output/engine.sqlite
```

Use your real replay path, optional promotion JSON, and optional SQLite evidence path (see comments in `scripts/replay_acceptance.sh`).

Only move to tiny live after replay + paper checks pass.
