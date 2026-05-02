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

3a. Wallet/execution checks: use the dashboard and `GET /api/v1/orders` (see `docs/api/dashboard-openapi.yaml` and `README.md`). Historical `pt-cli wallet-*` helpers are not in the current CLI surface.

4. Before switching to live, run:
```bash
cargo run -p pt-cli -- preflight-live --config config/config.toml --timeout-ms 3000
```

5. Strategy visualization/backtest loop (canonical driver: `tools/coinbase_strategy_lab.py`):
```bash
cp config/coinbase_strategy_lab.example.json config/coinbase_strategy_lab.json
python3 tools/coinbase_strategy_lab.py backtest --config config/coinbase_strategy_lab.json
python3 tools/coinbase_strategy_lab.py optimize --config config/coinbase_strategy_lab.json
python3 tools/coinbase_strategy_lab.py dashboard --config config/coinbase_strategy_lab.json --serve 9090
```

The Rust library `pt-strategy-lab` still exposes `/lab/*` HTTP handlers for embedding/tests; there is no `pt-cli strategy-lab-serve` today. See `docs/STRATEGY_LAB.md`.

5a. Promote a lab run into replay-mode input:
```bash
./scripts/promote_strategy_lab.sh data/strategy_lab/<report>.json BTC-USD sma_baseline
```

5b. Verify replay + promotion artifacts:
```bash
./scripts/replay_acceptance.sh data/replay/strategy_lab_promoted.ndjson data/tuning/promoted_candidate.json data/output/engine.sqlite
```
Adjust paths to your replay file, optional promotion JSON, and optional SQLite evidence path.

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

## Useful endpoints

Canonical list and request bodies: **`docs/api/dashboard-openapi.yaml`**. Shipped today from `pt-dashboard` include `GET /`, health/metrics, `GET /state/*` snapshots listed in that spec, workstation `GET|POST /api/v1/*`, and `POST /ops/halt|resume|flatten`.

## Coinbase local validation

Before first live startup:
```bash
cargo run -p pt-cli -- preflight-live --timeout-ms 5000
cargo run -p pt-cli -- coinbase preflight --mode live --timeout-ms 8000
```

## Schema references

- `docs/api/dashboard-openapi.yaml`
- `schemas/config.schema.json`
- `schemas/tradingview-webhook.schema.json`
- `schemas/coinbase_strategy_lab.schema.json`
- `schemas/strategy_lab.schema.json`
- `schemas/prompt_bundle.schema.json`

## Operating Notes

- Current progress tracker: `docs/PROGRESS.md`
- Active local instructions: `docs/INSTRUCTIONS.md`
