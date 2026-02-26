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

4. Before switching to live, run:
```bash
cargo run -p pt-cli -- preflight-live --config config/config.toml --timeout-ms 3000
```

5. Strategy visualization/backtest loop (Coinbase first):
```bash
cp config/coinbase_strategy_lab.example.json config/coinbase_strategy_lab.json
python3 tools/coinbase_strategy_lab.py dashboard --config config/coinbase_strategy_lab.json --serve 9090
```

6. If switching to live, set in `config/config.toml`:
- `engine.mode = "live"`
- `venues.polymarket.private_key`
- `venues.coinbase.api_key`
- `venues.coinbase.api_secret`

Or inject secrets via environment variables (preferred):
- `POLYMARKET_PRIVATE_KEY`
- `COINBASE_API_KEY`
- `COINBASE_API_SECRET`
- `COINBASE_PASSPHRASE` (optional)
- `TRADINGVIEW_ENDPOINT_SECRET` (optional)

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
- `GET /state/bias`
- `GET /state/inventory`
- `POST /ops/halt`
- `POST /ops/resume`
- `POST /ops/flatten`

## Schema References

- `docs/api/dashboard-openapi.yaml`
- `schemas/config.schema.json`
- `schemas/tradingview-webhook.schema.json`
