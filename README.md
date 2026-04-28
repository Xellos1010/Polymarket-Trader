# Polymarket Trader (Rust)

Rust-first trading workspace with a Coinbase-native workstation, replay/paper/live modes, legacy Polymarket support, hard risk controls, and operator tooling.

## Workspace crates

- `pt-core`: shared config, domain types, metrics, math, errors.
- `pt-market-discovery`: paginated Gamma market discovery and filtering/tiering.
- `pt-polymarket`: Polymarket REST + websocket orderbook client.
- `pt-coinbase`: Coinbase hedge adapter interfaces and implementations.
- `pt-wallet-intel`: wallet discovery, profile/positions/trades ingestion, bias scoring.
- `pt-signal`: wallet + TradingView bias fusion.
- `pt-quote`: quote intent generation and edge gate support.
- `pt-risk`: risk limits and kill-switch state machine.
- `pt-replay`: replay/paper simulation engine.
- `pt-engine`: async orchestration runtime and task graph.
- `pt-dashboard`: health/metrics/risk/ops HTTP endpoints.
- `pt-cli`: command-line entrypoint.

## Quick start (once Rust is installed)

```bash
cp config/config.example.toml config/config.toml
cargo run -p pt-cli -- coinbase up --config config/config.toml --mode paper
```

Open dashboard:

```text
http://127.0.0.1:8080/
```

## Local validation

Use the canonical local-first ladder before merge or deployment decisions:

```bash
./scripts/local_validation_ladder.sh
```

Guide:
- `docs/LOCAL_VALIDATION.md`

## Current recovery status

The current repo cycle is in Phase 0: repo readiness.

Before starting feature work or salvaging payload from older branches:
- check `docs/WORK_STATUS.md` for the operator-readable current stage
- check `docs/WORK_STATUS.json` for the machine-readable current stage
- check `docs/INTEGRATION_BOARD.md` for the ordered consolidation queue
- keep PR `#51` as the only active integration PR for the current cycle
- treat issue `#48` compile recovery as the next code-bearing slice
- keep issue `#9` and deferred branch salvage paused until the Phase 0 validation ladder is green again

## Developer setup

Install local git hooks:

```bash
./scripts/install_git_hooks.sh
```

## Environment variables

Use `.env.example` as the baseline for local or dev shells.

| Variable | Required | Purpose |
|---|---|---|
| `RUST_LOG` | No | Log level/filter for runtime diagnostics |
| `PT_CONFIG_PATH` | No | Optional override path to config TOML |
| `POLYMARKET_PRIVATE_KEY` | Live only | Wallet key material (inject securely) |
| `COINBASE_API_KEY` | Live only | Coinbase API key |
| `COINBASE_API_SECRET` | Live only | Coinbase API secret |
| `COINBASE_PASSPHRASE` | Optional | Coinbase passphrase for key types that require it |
| `TRADINGVIEW_ENDPOINT_SECRET` | Optional | Secret for TradingView webhook auth |

## Operator endpoints

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
- `POST /ops/halt`
- `POST /ops/resume`
- `POST /ops/flatten`
- `POST /ops/coinbase/rebalance/approve`
- `POST /ops/coinbase/rebalance/reject`
- `POST /ops/execution/unwind`

## CLI utilities

- Coinbase workstation:
  - `cargo run -p pt-cli -- coinbase up --config config/config.toml --mode paper`
  - `cargo run -p pt-cli -- coinbase up --config config/config.toml --mode replay`
  - `cargo run -p pt-cli -- coinbase preflight --config config/config.toml --mode live --timeout-ms 3000`
- Live preflight gate:
  - `cargo run -p pt-cli -- preflight-live --config config/config.toml --timeout-ms 3000`
- Wallet or execution operator shortcuts:
  - `cargo run -p pt-cli -- wallet-status`
  - `cargo run -p pt-cli -- wallet-plan`
  - `cargo run -p pt-cli -- wallet-approve --token-id <token_id>`
  - `cargo run -p pt-cli -- execution-status`
- Replay promotion acceptance:
  - `cargo run -p pt-cli -- verify-promoted --artifact data/tuning/promoted_candidate.json --out data/output/replay_acceptance_report.json`
- Journal comparative export:
  - `cargo run -p pt-cli -- report-variants --journal data/strategy_lab/trade_journal.sqlite --out-csv data/output/variant_report.csv --out-md data/output/variant_report.md`
- Maker opportunity scan (live orderbook spread or entry ranking):
  - `cargo run -p pt-cli -- scan-markets --config config/config.toml --limit 60 --top 15`
  - tune assumptions: `cargo run -p pt-cli -- scan-markets --config config/config.toml --adverse-sel-est 0.0025 --hedge-cost-est 0.0008 --gas-amortized-est 0.0003`
- Extract Pine parameters:
  - `cargo run -p pt-cli -- pine-params --path pine-scripts/<script> --out data/tuning/pine_params.json`
- Generate or tune Pine candidates:
  - `cargo run -p pt-cli -- tune-pine --path pine-scripts/<script> --iterations 100 --top-k 10`
- Tune with bundled evaluator:
  - `PT_EVAL_OHLCV=data/ohlcv/btc_1m.csv cargo run -p pt-cli -- tune-pine --path pine-scripts/<script> --iterations 200 --top-k 20 --evaluate-cmd "python3 tools/evaluate_candidate.py --fee-bps 2.0 --slippage-bps 1.0 --fixed-trade-cost 0.00005 --price-col close --timestamp-col ts"`
- Fetch OHLCV CSV for tuning input:
  - `python3 tools/fetch_ohlcv.py --provider coinbase --symbol BTCUSD --interval 1m --limit 300 --out data/ohlcv/btcusd_1m.csv`
  - fallback: `python3 tools/fetch_ohlcv.py --provider kraken --symbol BTCUSD --interval 1m --limit 300 --out data/ohlcv/btcusd_1m.csv`
- Coinbase strategy lab (backtest, overlap, optimize, dashboard):
  - `cp config/coinbase_strategy_lab.example.json config/coinbase_strategy_lab.json`
  - `python3 tools/coinbase_strategy_lab.py dashboard --config config/coinbase_strategy_lab.json`
  - with local server: `python3 tools/coinbase_strategy_lab.py dashboard --config config/coinbase_strategy_lab.json --serve 9090`
  - with listing auto-discovery: `python3 tools/coinbase_strategy_lab.py overlap --config config/coinbase_strategy_lab.json --auto-discovery`
  - disable journal writes: `python3 tools/coinbase_strategy_lab.py backtest --config config/coinbase_strategy_lab.json --disable-journal`
- Promote strategy-lab result to replay input:
  - `./scripts/promote_strategy_lab.sh data/strategy_lab/<report>.json BTC-USD sma_baseline`
- Promote best tuning candidate:
  - `./scripts/promote_candidate.sh data/tuning/pine_tuning_results.json data/tuning/promoted_candidate.json BTC 15m`
- Paper soak (24h default):
  - `./scripts/paper_soak.sh 86400 30 config/config.toml`
- Save session context:
  - `cargo run -p pt-cli -- save-context --out docs/SESSION_CONTEXT.md --note "checkpoint note"`
- Export prompt bundle for external AI iteration:
  - `cp config/prompt_bundle.example.json config/prompt_bundle.json`
  - `./scripts/export_prompt_bundle.sh`

## Schemas and contracts

- OpenAPI spec: `docs/api/dashboard-openapi.yaml`
- Config schema: `schemas/config.schema.json`
- TradingView webhook schema: `schemas/tradingview-webhook.schema.json`
- Coinbase strategy lab config schema: `schemas/coinbase_strategy_lab.schema.json`
- Prompt bundle config schema: `schemas/prompt_bundle.schema.json`
- Runtime data model summary: `docs/data/SCHEMA.md`

## SDLC and architecture docs

- SDLC checklist or status: `docs/SDLC_CHECKLIST.md`
- Architecture overview: `docs/architecture/system-overview.md`
- ADR index: `docs/adr/001-rust-first-polymarket-engine.md`
- Operations runbook: `docs/RUNBOOK.md`
- Tiny live pilot guide: `docs/TINY_LIVE_PILOT.md`
- Contribution guide: `CONTRIBUTING.md`
- Strategy lab guide: `docs/STRATEGY_LAB.md`
- Prompt bundle guide: `docs/PROMPT_BUNDLE.md`
- Progress tracker: `docs/PROGRESS.md`
- Local instructions: `docs/INSTRUCTIONS.md`
- Local validation guide: `docs/LOCAL_VALIDATION.md`

## CI security gates

- Vulnerability scan: `cargo audit`
- SBOM generation: `./scripts/generate_sbom.sh artifacts` (CycloneDX when `cargo-cyclonedx` is installed)

## Notes

- This workspace is built for small-cap validation first (`$10-$50`) with strict safety limits.
- The initial implementation prioritizes plumbing, observability, and risk enforcement.
- See `docs/CONTEXT_PERSISTENCE.md` for resume instructions.
- See `docs/PINE_TUNING.md` for Pine parameter tuning workflow.
