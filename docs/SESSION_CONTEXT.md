# Session Context

Generated at UNIX epoch seconds: `1772172705`

## Note
Patched Coinbase blockers (rustls provider + SEC1 key handling), passed coinbase-smoke, added Rust strategy-lab crate + CLI + dashboard, and added wallet conversion + maker speed-test controls in main dashboard.
Added listing-pattern dashboard tab with recent-listing overlays, plus feed health and parity monitor surfaces.
Added listing overlay CSV export and enhanced parity monitor math columns (gross/net/cost/min gate/usd).
Added parity CSV export and Coinbase WS timeout/reconnect hardening (keepalive ping + timeout streak gate).
Added server-side parity CSV export API (`/state/parity/export-csv`) and expanded feed-health counters for WS timeout/ping/connect diagnostics.
Added Master Optimization v4 scaffolding:
- Multi-venue config contracts (`venues.kraken`, `venues.gemini`) + runtime/hardware tuning config blocks.
- New endpoints: `/state/feed/diagnostics`, `/state/venues/*`, `/state/routes/export-csv`, `/state/wallet-intel/*`.
- New profile: `config/profiles/live_linux_ultra_tight.toml`.
- New scripts: `scripts/linux_tune_baseline.sh`, `scripts/install_homebase_service.sh`.

## Runtime
`rustc`: `rustc 1.93.1 (01f6ddf75 2026-02-11)`
`cargo`: `cargo 1.93.1 (083ac5135 2025-12-15)`

## Core Commands
- Run engine: `cargo run -p pt-cli -- run --config config/config.toml`
- Live preflight: `cargo run -p pt-cli -- preflight-live --config config/config.toml --timeout-ms 3000`
- Dashboard: `http://127.0.0.1:8080/`
- Health: `cargo run -p pt-cli -- status --url http://127.0.0.1:8080/health`
- Wallet status: `cargo run -p pt-cli -- wallet-status`
- Wallet plan: `cargo run -p pt-cli -- wallet-plan`
- Wallet approve: `cargo run -p pt-cli -- wallet-approve --token-id <token_id>`
- Execution status: `cargo run -p pt-cli -- execution-status`
- Coinbase WS status: `cargo run -p pt-cli -- coinbase-ws-status`
- Coinbase auth status: `cargo run -p pt-cli -- coinbase-auth-status`
- Coinbase auth reload: `cargo run -p pt-cli -- coinbase-auth-reload`
- Coinbase auth switch: `cargo run -p pt-cli -- coinbase-auth-switch --profile primary`
- Order manager status: `cargo run -p pt-cli -- order-manager-status`
- Routes status: `cargo run -p pt-cli -- routes-status`
- Set edge profile: `cargo run -p pt-cli -- set-edge-profile --strategy maker_mm_spot --min-bps 8`
- Pilot start: `cargo run -p pt-cli -- pilot-start --capital 20 --profile ultra-tight --timeout-ms 3000`
- Market list: `curl -s http://127.0.0.1:8080/state/markets | jq '.[0:5]'`
- Market history: `curl -s "http://127.0.0.1:8080/state/history?limit=120" | jq`
- Coinbase orderbook: `curl -s http://127.0.0.1:8080/state/coinbase/orderbook | jq '.[0:5]'`
- Route opportunities: `curl -s http://127.0.0.1:8080/state/routes/opportunities | jq '.[0:5]'`
- Feed health: `curl -s http://127.0.0.1:8080/state/feed/health | jq`
- Feed diagnostics: `curl -s http://127.0.0.1:8080/state/feed/diagnostics | jq`
- Parity monitor: `curl -s http://127.0.0.1:8080/state/parity/monitor | jq '.rows[0:5]'`
- Venue latency: `curl -s http://127.0.0.1:8080/state/venues/latency | jq`
- Venue fill quality: `curl -s http://127.0.0.1:8080/state/venues/fill-quality | jq`
- Wallet intel coinbase: `curl -s http://127.0.0.1:8080/state/wallet-intel/coinbase | jq '.[0:20]'`
- Wallet intel polymarket: `curl -s http://127.0.0.1:8080/state/wallet-intel/polymarket | jq '.[0:20]'`
- Wallet intel leaderboard: `curl -s http://127.0.0.1:8080/state/wallet-intel/leaderboard | jq '.[0:20]'`
- Routes CSV export: `curl -s -X POST http://127.0.0.1:8080/state/routes/export-csv -H 'content-type: application/json' -d '{"limit":500,"min_expected_net_bps":0}' | jq`
- Wallet intel CSV export: `curl -s -X POST http://127.0.0.1:8080/state/wallet-intel/export-csv -H 'content-type: application/json' -d '{"source":"all","limit":5000}' | jq`
- Listing candidates: `curl -s "http://127.0.0.1:8080/state/listings/candidates?window=90d&granularity_sec=14400" | jq '.candidates[0:10]'`
- Listing overlay: `curl -s -X POST http://127.0.0.1:8080/state/listings/overlay -H 'content-type: application/json' -d '{"window_preset":"90d","granularity_sec":14400,"alignment_mode":"entry_aligned","normalization":"indexed","product_ids":["BTC-USD","ETH-USD"]}' | jq`
- Coinbase smoke: `cargo run -p pt-cli -- coinbase-smoke --timeout-ms 8000`
- Strategy lab serve: `cargo run -p pt-cli -- strategy-lab-serve --bind 127.0.0.1:9090 --db data/strategy_lab/strategy_lab.sqlite`
- Convert preview: `curl -s -X POST http://127.0.0.1:8080/ops/coinbase/convert/preview -H 'content-type: application/json' -d '{"from_asset":"BTC","to_asset":"USD","amount":0.0001,"live":false}' | jq`
- Convert execute (paper): `curl -s -X POST http://127.0.0.1:8080/ops/coinbase/convert/execute -H 'content-type: application/json' -d '{"from_asset":"BTC","to_asset":"USD","amount":0.0001,"live":false}' | jq`
- Maker speed test (paper): `curl -s -X POST http://127.0.0.1:8080/ops/coinbase/maker-test -H 'content-type: application/json' -d '{"product_id":"BTC-USD","side":"buy","base_size":0.0001,"live":false}' | jq`
- Strategy backtest: `cargo run -p pt-cli -- strategy-backtest --product BTC-USD --granularity-sec 300 --limit 600 --out data/output/strategy_backtest_report.json`
- Strategy optimize: `cargo run -p pt-cli -- strategy-optimize --product BTC-USD --granularity-sec 300 --limit 600 --iterations 200 --walk-forward-splits 4 --out data/output/strategy_optimize_report.json`
- Strategy profile load: `cargo run -p pt-cli -- strategy-profile-load --profile-id default --out data/output/strategy_profile_default.json`
- Strategy profile save: `cargo run -p pt-cli -- strategy-profile-save --path data/output/strategy_profile_default.json --note \"manual update\"`
- Extract pine params: `cargo run -p pt-cli -- pine-params --path pine-scripts/<script> --out data/tuning/pine_params.json`
- Tune pine params: `cargo run -p pt-cli -- tune-pine --path pine-scripts/<script> --iterations 100 --evaluate-cmd "python3 tools/evaluate_candidate.py"`

- Promote tuning candidate: `./scripts/promote_candidate.sh data/tuning/pine_tuning_results.json data/tuning/promoted_candidate.json BTC 15m`
- Verify promoted artifact: `cargo run -p pt-cli -- verify-promoted --artifact data/tuning/promoted_candidate.json --out data/output/replay_acceptance_report.json`
- Report variants: `cargo run -p pt-cli -- report-variants --journal data/strategy_lab/trade_journal.sqlite --out-csv data/output/variant_report.csv --out-md data/output/variant_report.md`
- Paper soak: `./scripts/paper_soak.sh 3600 30 config/config.toml`
- Tiny live pilot checks: `./scripts/tiny_live_pilot.sh config/config.toml 3000`
- Linux host tune baseline: `sudo ./scripts/linux_tune_baseline.sh`
- Install macOS homebase service: `./scripts/install_homebase_service.sh config/config.toml`
- Install git hooks: `./scripts/install_git_hooks.sh`

## Live Prerequisites
- Set `engine.mode = "live"` in config.
- Set `venues.polymarket.private_key` or `POLYMARKET_PRIVATE_KEY`.
- Coinbase auth: legacy (`venues.coinbase.api_key/api_secret`) OR profile (`venues.coinbase.auth.active_profile` + `cdp_key_file|cdp_secret_id`).
- Env overrides: `COINBASE_AUTH_PROFILE`, `COINBASE_CDP_KEY_FILE`, `COINBASE_CDP_SECRET_ID`, `COINBASE_EXPECTED_KEY_ID`.
- Keep hard risk caps enabled for tiny-live rollout.
- Live convert confirm phrase: `I_UNDERSTAND_LIVE_CONVERT`.
- Live maker-test confirm phrase: `I_UNDERSTAND_LIVE_MAKER_TEST`.
