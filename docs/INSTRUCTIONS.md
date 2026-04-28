# Instructions

## Current Cycle Override

Before starting feature work, salvaging payload from older branches, or making a merge-readiness claim, review these files in order:
- `docs/WORK_STATUS.md`
- `docs/INTEGRATION_BOARD.md`
- `docs/SESSION_CONTEXT.md`

Current grounded state for this cycle:
- the repo is in Phase 0: repo readiness
- PR `#51` is the only active integration PR
- issue `#48` compile recovery is the next code-bearing slice
- issue `#9` and deferred branch salvage remain paused until the Phase 0 validation ladder is green again

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

### Prepare configs
```bash
cp config/config.example.toml config/config.toml
cp config/coinbase_strategy_lab.example.json config/coinbase_strategy_lab.json
cp config/prompt_bundle.example.json config/prompt_bundle.json
```

### Run strategy dashboard (Rust, recommended)
```bash
cargo run -p pt-cli -- strategy-lab-serve --bind 127.0.0.1:9090 --db data/strategy_lab/strategy_lab.sqlite
```

### Run Coinbase workstation in paper mode
```bash
cargo run -p pt-cli -- coinbase up --config config/config.toml --mode paper
```

### Optional Python dashboard fallback
```bash
python3 tools/coinbase_strategy_lab.py dashboard --config config/coinbase_strategy_lab.json --serve 9090
```

### Rust strategy-lab local CLI workflow
```bash
cargo run -p pt-cli -- strategy-backtest --product BTC-USD --granularity-sec 300 --limit 600 --out data/output/strategy_backtest_report.json
cargo run -p pt-cli -- strategy-optimize --product BTC-USD --granularity-sec 300 --limit 600 --iterations 200 --walk-forward-splits 4 --out data/output/strategy_optimize_report.json
cargo run -p pt-cli -- strategy-profile-load --profile-id default --out data/output/strategy_profile_default.json
cargo run -p pt-cli -- strategy-profile-save --path data/output/strategy_profile_default.json --note "manual tuning update"
```

### Promote selected result to replay artifact
```bash
./scripts/promote_strategy_lab.sh data/strategy_lab/<dashboard-or-backtest>.json BTC-USD sma_baseline
```

### Apply replay settings in `config/config.toml`
- `engine.mode = "replay"`
- `engine.replay_path = "data/replay/strategy_lab_promoted.ndjson"`

### Run Coinbase workstation in replay mode
```bash
cargo run -p pt-cli -- coinbase up --config config/config.toml --mode replay
```

### Split runtime modes (recommended)
```bash
# control-plane homebase (dashboard, analytics, wallet intel)
cargo run -p pt-cli -- run-homebase --config config/config.toml

# data-plane execution only (no dashboard)
cargo run -p pt-cli -- run-exec --config config/config.toml
```

### Open dashboards
- Engine + ops dashboard: `http://127.0.0.1:8080`
- Strategy lab/backtester: `http://127.0.0.1:9090`

### Dashboard controls available on a healthy build
- Market dropdown now shows pair/bucket label instead of raw market id.
- `View` selector supports `Basic` and `Advanced` mode (persisted in browser localStorage and mirrored to `/ops/ui/mode`).
- `CHART` / `BACKTESTER` tabs switch between live market charting and embedded strategy lab.
- `LISTING PATTERN` tab overlays recently listed Coinbase products with configurable window, alignment, and normalization.
- `EXPORT CSV` exports the current overlay series as row-wise CSV (`product_id,label,source,anchor_time,index,ts,value`).
- `Parity Monitor` includes `EXPORT PARITY CSV` for route gate and audit snapshots.
- Granularity selector controls aggregated delta-bar rendering for selected market history.
- `Selected Pair Orderbook Depth` shows bid and ask ladders for the selected market's mapped Coinbase product.
- `Wallet Convert (Maker-First)` supports preview, paper execute, and guarded live execute.
- `Maker Orderbook Speed Test` supports paper and guarded live timing probes.
- Realtime summary panels include cross-exchange shadow, downturn strategy, capital planner, equity universe, and equity paper runs.

## Wallet-First Operator Checks

```bash
cargo run -p pt-cli -- wallet-status
cargo run -p pt-cli -- wallet-plan
cargo run -p pt-cli -- execution-status
cargo run -p pt-cli -- coinbase-ws-status
cargo run -p pt-cli -- coinbase-auth-status
cargo run -p pt-cli -- routes-status
cargo run -p pt-cli -- order-manager-status
```

If a rebalance plan is pending approval:
```bash
cargo run -p pt-cli -- wallet-approve --token-id <token_id>
```

## Replay Promotion Verification

```bash
cargo run -p pt-cli -- verify-promoted --artifact data/tuning/promoted_candidate.json --out data/output/replay_acceptance_report.json
```

## Strategy-Lab Comparative Export

```bash
cargo run -p pt-cli -- report-variants --journal data/strategy_lab/trade_journal.sqlite --out-csv data/output/variant_report.csv --out-md data/output/variant_report.md
```

## Edge Profile Tuning

```bash
cargo run -p pt-cli -- set-edge-profile --strategy maker_mm_spot --min-bps 8
cargo run -p pt-cli -- set-edge-profile --strategy conversion_cycle --min-bps 100
```

## Live and Auth Guarded Commands

These commands remain gated behind successful Phase 0 and Phase 1 evidence. Do not treat them as the current next step while compile recovery is still open.

Tiny live pilot start:
```bash
cargo run -p pt-cli -- pilot-start --capital 20 --profile ultra-tight --timeout-ms 3000
```

Coinbase authenticated smoke test (read-only by default):
```bash
cargo run -p pt-cli -- coinbase-smoke --timeout-ms 8000
```

Coinbase guarded write smoke (tiny post-only create/edit/cancel):
```bash
cargo run -p pt-cli -- coinbase-smoke --timeout-ms 8000 --write-test --confirm I_UNDERSTAND_POST_ONLY_TEST_ORDERS
```

Coinbase auth hot reload and profile switch:
```bash
cargo run -p pt-cli -- coinbase-auth-reload
cargo run -p pt-cli -- coinbase-auth-switch --profile primary
```

## Listing, Feed, and Parity API Quick Checks

```bash
curl -s http://127.0.0.1:8080/state/feed/health | jq
curl -s http://127.0.0.1:8080/state/feed/diagnostics | jq
curl -s http://127.0.0.1:8080/state/ui/mode | jq
curl -s -X POST http://127.0.0.1:8080/ops/ui/mode -H 'content-type: application/json' -d '{"mode":"advanced"}' | jq
curl -s http://127.0.0.1:8080/state/crossvenue/shadow-summary | jq
curl -s http://127.0.0.1:8080/state/strategy/downturn-summary | jq
curl -s http://127.0.0.1:8080/state/capital/plan | jq
curl -s -X POST http://127.0.0.1:8080/ops/capital/close-day -H 'content-type: application/json' -d '{"contribution_usd":10,"realized_pnl_usd":1.0,"approve":true}' | jq
curl -s http://127.0.0.1:8080/state/capital/ledger | jq '.[0:5]'
curl -s http://127.0.0.1:8080/state/equities/universe | jq '.[0:10]'
curl -s http://127.0.0.1:8080/state/equities/paper-runs | jq '.[0:10]'
curl -s http://127.0.0.1:8080/state/parity/monitor | jq '.rows[0:5]'
curl -s -X POST http://127.0.0.1:8080/state/parity/export-csv -H 'content-type: application/json' -d '{"limit":500,"include_failures":true}' | jq
curl -s http://127.0.0.1:8080/state/venues/latency | jq
curl -s http://127.0.0.1:8080/state/venues/fill-quality | jq
curl -s http://127.0.0.1:8080/state/wallet-intel/coinbase | jq '.[0:20]'
curl -s -X POST http://127.0.0.1:8080/state/routes/export-csv -H 'content-type: application/json' -d '{"limit":500,"min_expected_net_bps":0}' | jq
curl -s -X POST http://127.0.0.1:8080/state/wallet-intel/export-csv -H 'content-type: application/json' -d '{"source":"all","limit":5000}' | jq
curl -s "http://127.0.0.1:8080/state/listings/candidates?window=90d&granularity_sec=14400" | jq '.candidates[0:10]'
curl -s -X POST http://127.0.0.1:8080/state/listings/overlay -H 'content-type: application/json' -d '{"window_preset":"90d","granularity_sec":14400,"alignment_mode":"entry_aligned","normalization":"indexed","product_ids":["BTC-USD","ETH-USD"]}' | jq
```

## Linux Ultra-Tight Profile and Host Tuning

```bash
cat config/profiles/live_linux_ultra_tight.toml
sudo ./scripts/linux_tune_baseline.sh
```

## macOS Homebase Service Install

```bash
./scripts/install_homebase_service.sh config/config.toml
launchctl list | rg 'com.pt.homebase'
```

## Strategy Variant Plugins

Configured in `backtest.variants[*].plugins`:
- `external_bias_file`: load `{idx,bias}` or `{ts_ms,bias}` series from JSON or CSV.
- `momentum_bias`: tanh-scaled lookback return.
- `rsi_bias`: directional bias from RSI extremes.

Use `bias_gain` per variant to control plugin influence.

## Context Persistence

Save session checkpoint:
```bash
./scripts/save_context.sh "note" docs/SESSION_CONTEXT.md config/config.toml
```

Export external AI bundle:
```bash
./scripts/export_prompt_bundle.sh
```

## Execution and Wallet API Surfaces

- `GET /state/execution/orders`
- `GET /state/execution/costs`
- `GET /state/execution/vectors`
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
- `POST /ops/ui/mode` (`{"mode":"basic|advanced"}`)
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
- `GET /state/listings/candidates`
- `POST /state/listings/overlay`
- `GET /state/listings/l2-archive`
- `POST /state/routes/export-csv`
- `GET /state/wallet-intel/coinbase`
- `GET /state/wallet-intel/polymarket`
- `GET /state/wallet-intel/leaderboard`
- `POST /state/wallet-intel/export-csv`
- `POST /ops/coinbase/rebalance/approve` (`{"token_id":"..."}`)
- `POST /ops/coinbase/rebalance/reject`
- `POST /ops/coinbase/auth/reload`
- `POST /ops/coinbase/auth/switch-profile` (`{"profile_id":"primary"}`)
- `POST /ops/coinbase/convert/preview`
- `POST /ops/coinbase/convert/execute`
- `POST /ops/coinbase/maker-test`
- `POST /ops/execution/unwind`
- `POST /ops/unwind/now`
- `POST /ops/profile/pilot-ultra-tight`
