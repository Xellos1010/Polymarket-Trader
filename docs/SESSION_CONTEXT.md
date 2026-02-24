# Session Context

Generated at UNIX epoch seconds: `1771891620`

## Note
Added live preflight command, market history dashboard endpoints, and bundled pine evaluator tool.

## Runtime
`rustc`: `rustc 1.93.1 (01f6ddf75 2026-02-11)`
`cargo`: `cargo 1.93.1 (083ac5135 2025-12-15)`

## Core Commands
- Run engine: `cargo run -p pt-cli -- run --config config/config.toml`
- Live preflight: `cargo run -p pt-cli -- preflight-live --config config/config.toml --timeout-ms 3000`
- Dashboard: `http://127.0.0.1:8080/`
- Health: `cargo run -p pt-cli -- status --url http://127.0.0.1:8080/health`
- Market list: `curl -s http://127.0.0.1:8080/state/markets | jq '.[0:5]'`
- Market history: `curl -s "http://127.0.0.1:8080/state/history?limit=120" | jq`
- Extract pine params: `cargo run -p pt-cli -- pine-params --path pine-scripts/<script> --out data/tuning/pine_params.json`
- Tune pine params: `cargo run -p pt-cli -- tune-pine --path pine-scripts/<script> --iterations 100 --evaluate-cmd "python3 tools/evaluate_candidate.py"`

## Live Prerequisites
- Set `engine.mode = "live"` in config.
- Set `venues.polymarket.private_key`.
- Set `venues.coinbase.api_key` and `venues.coinbase.api_secret`.
- Keep hard risk caps enabled for tiny-live rollout.
