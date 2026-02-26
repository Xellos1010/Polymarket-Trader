# Session Context

Generated at UNIX epoch seconds: `1772072894`

## Note
Implemented next-priority strategy-lab upgrades:
- listing overlap auto-discovery for recent Coinbase markets
- strategy plugin variants (external Pine/AI bias file + momentum + RSI)
- persistent SQLite trade journal and per-market attribution summary
- strategy-lab promotion pipeline to replay NDJSON (`tools/promote_strategy_lab.py`, `scripts/promote_strategy_lab.sh`)
- progress and operator instructions docs (`docs/PROGRESS.md`, `docs/INSTRUCTIONS.md`)

## Runtime
`rustc`: `rustc 1.93.1 (01f6ddf75 2026-02-11)`
`cargo`: `cargo 1.93.1 (083ac5135 2025-12-15)`

## Core Commands
- Run engine: `cargo run -p pt-cli -- run --config config/config.toml`
- Live preflight: `cargo run -p pt-cli -- preflight-live --config config/config.toml --timeout-ms 3000`
- Dashboard: `http://127.0.0.1:8080/`
- Health: `cargo run -p pt-cli -- status --url http://127.0.0.1:8080/health`

### Strategy Lab
- Generate dashboard: `python3 tools/coinbase_strategy_lab.py dashboard --config config/coinbase_strategy_lab.json --serve 9090`
- Overlap with auto-discovery: `python3 tools/coinbase_strategy_lab.py overlap --config config/coinbase_strategy_lab.json --auto-discovery`
- Backtest without journal writes: `python3 tools/coinbase_strategy_lab.py backtest --config config/coinbase_strategy_lab.json --disable-journal`

### Promotion / Replay
- Promote lab result: `./scripts/promote_strategy_lab.sh data/strategy_lab/<report>.json BTC-USD sma_baseline`
- Apply replay config:
  - `engine.mode = "replay"`
  - `engine.replay_path = "data/replay/strategy_lab_promoted.ndjson"`

### Pine Tuning
- Extract pine params: `cargo run -p pt-cli -- pine-params --path pine-scripts/<script> --out data/tuning/pine_params.json`
- Tune pine params: `cargo run -p pt-cli -- tune-pine --path pine-scripts/<script> --iterations 100 --evaluate-cmd "python3 tools/evaluate_candidate.py"`
- Promote tuning candidate: `./scripts/promote_candidate.sh data/tuning/pine_tuning_results.json data/tuning/promoted_candidate.json BTC 15m`

## Live Prerequisites
- Set `engine.mode = "live"` in config.
- Set `venues.polymarket.private_key` or `POLYMARKET_PRIVATE_KEY`.
- Set `venues.coinbase.api_key`/`api_secret` or `COINBASE_API_KEY`/`COINBASE_API_SECRET`.
- Keep hard risk caps enabled for tiny-live rollout.
