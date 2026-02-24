# Polymarket Trader (Rust)

Rust-first autonomous trading workspace for Polymarket crypto markets with Coinbase spot hedge support, hard risk controls, replay/paper/live modes, and operator dashboard.

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
cargo run -p pt-cli -- run --config config/config.toml
```

Open dashboard:

```text
http://127.0.0.1:8080/
```

## Operator endpoints

- `GET /` dashboard UI
- `GET /health`
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

## CLI utilities

- Live preflight gate:
  - `cargo run -p pt-cli -- preflight-live --config config/config.toml --timeout-ms 3000`
- Extract Pine parameters:
  - `cargo run -p pt-cli -- pine-params --path pine-scripts/<script> --out data/tuning/pine_params.json`
- Generate/tune Pine candidates:
  - `cargo run -p pt-cli -- tune-pine --path pine-scripts/<script> --iterations 100 --top-k 10`
- Tune with bundled evaluator:
  - `cargo run -p pt-cli -- tune-pine --path pine-scripts/<script> --iterations 200 --top-k 20 --evaluate-cmd "python3 tools/evaluate_candidate.py"`
- Save session context:
  - `cargo run -p pt-cli -- save-context --out docs/SESSION_CONTEXT.md --note "checkpoint note"`

## Notes

- This workspace is built for small-cap validation first (`$10-$50`) with strict safety limits.
- The initial implementation prioritizes plumbing, observability, and risk enforcement.
- See `docs/CONTEXT_PERSISTENCE.md` for resume instructions.
- See `docs/PINE_TUNING.md` for Pine parameter tuning workflow.
