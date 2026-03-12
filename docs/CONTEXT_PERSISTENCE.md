# Context Persistence Guide

Use this to resume quickly after interruptions.

## Local-First Release Rule

Do not run CI/CD or deployment pipeline automation until local verification passes:
1. `cargo check --workspace`
2. local `paper` run
3. strategy-lab/backtest validation
4. tiny live pilot guard (`scripts/tiny_live_pilot.sh`)

## Save Current Session

```bash
cargo run -p pt-cli -- save-context --out docs/SESSION_CONTEXT.md --note "what changed + next step"
```

This writes:
- runtime versions (`rustc`, `cargo`)
- core run commands
- live prerequisites

For external AI handoff context, export a prompt bundle:

```bash
cp config/prompt_bundle.example.json config/prompt_bundle.json
./scripts/export_prompt_bundle.sh
```

## Resume Workflow

1. Validate build:
```bash
cargo check --workspace
```

2. Start paper mode:
```bash
cargo run -p pt-cli -- run --config config/config.toml
```

3. Open dashboard:
```text
http://127.0.0.1:8080/
```

3a. Wallet/execution quick checks:
```bash
cargo run -p pt-cli -- wallet-status
cargo run -p pt-cli -- wallet-plan
cargo run -p pt-cli -- execution-status
cargo run -p pt-cli -- coinbase-auth-status
```

4. Before switching to live, run:
```bash
cargo run -p pt-cli -- preflight-live --config config/config.toml --timeout-ms 3000
```

5. Strategy visualization/backtest loop (Coinbase first, Rust strategy lab):
```bash
cargo run -p pt-cli -- strategy-lab-serve --bind 127.0.0.1:9090 --db data/strategy_lab/strategy_lab.sqlite
```

5a. Rust strategy backtest/optimize flow:
```bash
cargo run -p pt-cli -- strategy-backtest --product BTC-USD --granularity-sec 300 --limit 600 --out data/output/strategy_backtest_report.json
cargo run -p pt-cli -- strategy-optimize --product BTC-USD --granularity-sec 300 --limit 600 --iterations 200 --walk-forward-splits 4 --out data/output/strategy_optimize_report.json
```

5b. Optional Python fallback:
```bash
cp config/coinbase_strategy_lab.example.json config/coinbase_strategy_lab.json
python3 tools/coinbase_strategy_lab.py dashboard --config config/coinbase_strategy_lab.json --serve 9090
```

5c. Promote a lab run into replay-mode input:
```bash
./scripts/promote_strategy_lab.sh data/strategy_lab/<report>.json BTC-USD sma_baseline
```

5d. Verify promoted artifact against acceptance thresholds:
```bash
cargo run -p pt-cli -- verify-promoted --artifact data/tuning/promoted_candidate.json --out data/output/replay_acceptance_report.json
```

6. If switching to live, set in `config/config.toml`:
- `engine.mode = "live"`
- `engine.portfolio_id = "<portfolio>"`
- `venues.polymarket.private_key`
- Coinbase auth strategy:
  - Legacy: `venues.coinbase.api_key` + `venues.coinbase.api_secret`
  - Profile: `venues.coinbase.auth.active_profile` + `venues.coinbase.auth.profiles.<id>.cdp_key_file|cdp_secret_id`

Or inject secrets via environment variables (preferred):
- `POLYMARKET_PRIVATE_KEY`
- `COINBASE_API_KEY`
- `COINBASE_API_SECRET`
- `COINBASE_AUTH_PROFILE`
- `COINBASE_CDP_KEY_FILE`
- `COINBASE_CDP_SECRET_ID`
- `COINBASE_EXPECTED_KEY_ID`
- `COINBASE_PASSPHRASE` (optional)
- `TRADINGVIEW_ENDPOINT_SECRET` (optional)

For Linux ultra-tight pilot baseline, review:
- `config/profiles/live_linux_ultra_tight.toml`
- `scripts/linux_tune_baseline.sh`

For macOS homebase boot persistence:
- `scripts/install_homebase_service.sh`

## Useful Endpoints

- `GET /` dashboard UI
- `GET /health`
- `GET /healthz`
- `GET /ready`
- `GET /metrics`
- `GET /state/risk`
- `GET /state/books`
- `GET /state/markets`
- `GET /state/history?market_id=<id>&limit=360`
- `GET /state/executions`
- `GET /state/execution/orders`
- `GET /state/execution/costs`
- `GET /state/execution/vectors`
- `GET /state/bias`
- `GET /state/inventory`
- `GET /state/coinbase/wallet`
- `GET /state/coinbase/allocations`
- `GET /state/coinbase/rebalance-plan`
- `GET /state/coinbase/orders`
- `GET /state/coinbase/orderbook`
- `GET /state/coinbase/auth`
- `GET /state/routes/opportunities`
- `GET /state/routes/executions`
- `GET /state/fees/summary`
- `GET /state/feed/health`
- `GET /state/feed/diagnostics`
- `GET /state/ui/mode`
- `POST /ops/ui/mode`
- `GET /state/crossvenue/shadow-summary`
- `GET /state/strategy/downturn-summary`
- `GET /state/capital/plan`
- `POST /ops/capital/close-day`
- `GET /state/capital/ledger`
- `GET /state/equities/universe`
- `GET /state/equities/paper-runs`
- `GET /state/parity/monitor`
- `POST /state/parity/export-csv`
- `GET /state/venues/latency`
- `GET /state/venues/fill-quality`
- `GET /state/venues/rejects`
- `POST /state/routes/export-csv`
- `GET /state/wallet-intel/coinbase`
- `GET /state/wallet-intel/polymarket`
- `GET /state/wallet-intel/leaderboard`
- `POST /state/wallet-intel/export-csv`
- `GET /state/listings/candidates`
- `POST /state/listings/overlay`
- `GET /state/listings/l2-archive`
- `POST /ops/halt`
- `POST /ops/resume`
- `POST /ops/flatten`
- `POST /ops/coinbase/rebalance/approve`
- `POST /ops/coinbase/rebalance/reject`
- `POST /ops/coinbase/auth/reload`
- `POST /ops/coinbase/auth/switch-profile`
- `POST /ops/execution/unwind`

## Coinbase Local Validation

Run this before first live startup:
```bash
cargo run -p pt-cli -- preflight-live --timeout-ms 5000
cargo run -p pt-cli -- coinbase-smoke --timeout-ms 8000
```

## Schema References

- `docs/api/dashboard-openapi.yaml`
- `schemas/config.schema.json`
- `schemas/tradingview-webhook.schema.json`
- `schemas/coinbase_strategy_lab.schema.json`
- `schemas/prompt_bundle.schema.json`

## Operating Notes

- Current progress tracker: `docs/PROGRESS.md`
- Active local instructions: `docs/INSTRUCTIONS.md`
