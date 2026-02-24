# Context Persistence Guide

Use this to resume quickly after interruptions.

## Save Current Session

```bash
cargo run -p pt-cli -- save-context --out docs/SESSION_CONTEXT.md --note "what changed + next step"
```

This writes:
- runtime versions (`rustc`, `cargo`)
- core run commands
- live prerequisites

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

5. If switching to live, set in `config/config.toml`:
- `engine.mode = "live"`
- `venues.polymarket.private_key`
- `venues.coinbase.api_key`
- `venues.coinbase.api_secret`

## Useful Endpoints

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
