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

## 4) Validate in Rust Strategy Lab

Backtest:

```bash
cargo run -p pt-cli -- strategy-backtest --product BTC-USD --granularity-sec 300 --limit 600 --out data/output/strategy_backtest_report.json
```

Optimize/walk-forward:

```bash
cargo run -p pt-cli -- strategy-optimize --product BTC-USD --granularity-sec 300 --limit 600 --iterations 200 --walk-forward-splits 4 --out data/output/strategy_optimize_report.json
```

Dashboard:

```bash
cargo run -p pt-cli -- strategy-lab-serve --bind 127.0.0.1:9090 --db data/strategy_lab/strategy_lab.sqlite
```

## 5) Promotion Gate

```bash
cargo run -p pt-cli -- verify-promoted --artifact data/tuning/promoted_candidate.json --out data/output/replay_acceptance_report.json
```

Only move to tiny live after replay + paper checks pass.
