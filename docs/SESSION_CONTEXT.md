# Session Context

Generated at UNIX epoch seconds: `1771907558`

## Note
Implemented backlog items #2-#10: OpenAPI contract tests, git hooks, CI audit+SBOM, env secret overrides, deployment automation, soak tooling, OHLCV fetch + evaluator cost model, candidate promotion flow, and tiny live pilot guard.

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

- Promote tuning candidate: `./scripts/promote_candidate.sh data/tuning/pine_tuning_results.json data/tuning/promoted_candidate.json BTC 15m`
- Paper soak: `./scripts/paper_soak.sh 3600 30 config/config.toml`
- Tiny live pilot checks: `./scripts/tiny_live_pilot.sh config/config.toml 3000`
- Install git hooks: `./scripts/install_git_hooks.sh`

## Live Prerequisites
- Set `engine.mode = "live"` in config.
- Set `venues.polymarket.private_key` or `POLYMARKET_PRIVATE_KEY`.
- Set `venues.coinbase.api_key`/`api_secret` or `COINBASE_API_KEY`/`COINBASE_API_SECRET`.
- Keep hard risk caps enabled for tiny-live rollout.
