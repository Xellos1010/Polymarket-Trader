# Instructions

## Current Cycle Override

Before starting feature work, salvaging payload from older branches, or making a merge-readiness claim, review these files in order:
- `docs/WORK_STATUS.md`
- `docs/WORK_STATUS.json`
- `docs/archive/program-history-2026/INTEGRATION_BOARD.md` (historical)
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

### Run strategy lab (Python driver, recommended)
```bash
python3 tools/coinbase_strategy_lab.py backtest --config config/coinbase_strategy_lab.json
python3 tools/coinbase_strategy_lab.py optimize --config config/coinbase_strategy_lab.json
python3 tools/coinbase_strategy_lab.py dashboard --config config/coinbase_strategy_lab.json --serve 9090
```

The Rust crate `pt-strategy-lab` implements `/lab/*` HTTP handlers for tests and embedding; there is no `pt-cli strategy-lab-serve` today.

### Run Coinbase workstation in paper mode
```bash
cargo run -p pt-cli -- coinbase up --config config/config.toml --mode paper
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

### Full engine + ops dashboard
```bash
cargo run -p pt-cli -- run --config config/config.toml
```

### Open dashboards
- Engine + ops dashboard: `http://127.0.0.1:8080` (bind from `[ops].dashboard_bind` in `config/config.toml`)
- Strategy lab static bundle (when using `--serve`): `http://127.0.0.1:9090` (or the port you pass to `coinbase_strategy_lab.py dashboard --serve`)

### Dashboard UI

The bundled React UI under `crates/pt-dashboard/frontend` tracks workstation state (markets, orders, risk, strategy-lab imports). Treat **`docs/api/dashboard-openapi.yaml`** as the source of truth for HTTP paths and bodies exposed by `pt-dashboard` today.

## Operator checks (current `pt-cli`)

```bash
cargo run -p pt-cli -- status --url http://127.0.0.1:8080/health
cargo run -p pt-cli -- preflight-live --timeout-ms 3000
```

## Replay promotion verification

```bash
./scripts/replay_acceptance.sh data/replay/strategy_lab_promoted.ndjson data/tuning/promoted_candidate.json data/output/engine.sqlite
```

## Live and guarded flows

Pilot and smoke flows are documented in **`docs/TINY_LIVE_PILOT.md`**, **`docs/RUNBOOK.md`**, and **`scripts/tiny_live_pilot.sh`**. Several historical `pt-cli` subcommands (`wallet-*`, `coinbase-smoke`, `pilot-start`, `run-homebase`, `verify-promoted`, …) are **not** in the current CLI; prefer dashboard/API plus the scripts above.

## Dashboard HTTP smoke checks

```bash
curl -fsS http://127.0.0.1:8080/health
curl -fsS http://127.0.0.1:8080/metrics | head
curl -fsS http://127.0.0.1:8080/state/risk | jq
curl -fsS http://127.0.0.1:8080/api/v1/products | jq '.[0:3]'
```

## Linux Ultra-Tight Profile and Host Tuning

```bash
cat config/profiles/live_linux_ultra_tight.toml
sudo ./scripts/linux_tune_baseline.sh
```

## macOS Homebase Service Install

```bash
./scripts/install_homebase_service.sh config/config.toml
launchctl list | grep 'com.pt.homebase'
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

## Execution and workstation HTTP surfaces

Canonical paths live in **`docs/api/dashboard-openapi.yaml`** and the `README.md` operator summary. Older docs listed many `/state/*` and `/ops/*` routes that are not present on the current `pt-dashboard` router; use the OpenAPI file plus `crates/pt-dashboard/src/lib.rs` when auditing what is actually bound.
