# Session Context

Generated at UNIX epoch seconds: `1772072894`

## Note
Implemented next-priority strategy-lab upgrades:
- listing overlap auto-discovery for recent Coinbase markets
- strategy plugin variants (external Pine/AI bias file + momentum + RSI)
- persistent SQLite trade journal and per-market attribution summary
- strategy-lab promotion pipeline to replay NDJSON (`tools/promote_strategy_lab.py`, `scripts/promote_strategy_lab.sh`)
- progress and operator instructions docs (`docs/PROGRESS.md`, `docs/INSTRUCTIONS.md`)

## 2026-04-26 Next Round Checkpoint
- reviewed current `main`, open PRs #11, #12, #13, and #14, plus issues #9 and #10
- confirmed the repo is still in Phase 1 and the next implementation blocker is restart-safe approval-queue persistence
- queued the next focused coding slice in `docs/PHASE1_NEXT_ROUND_2026-04-26.md`
- recommended merge/order sequence:
  - PR #12 evidence gate
  - PR #13 read-only approval queue API
  - PR #11 frontend fixture safety net
  - PR #14 close or supersede after implementation work is active
  - next new code PR: issue #9 SQLite-backed workstation order persistence and startup hydration
- parallel analysis results agreed on the narrowest safe path:
  - persist workstation order lifecycle state using `storage.sqlite_path`
  - hydrate persisted orders into runtime startup state
  - keep execution authority unchanged and avoid new mutating approval APIs
  - validate with create/update/reload coverage before any merge/deploy decision

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
