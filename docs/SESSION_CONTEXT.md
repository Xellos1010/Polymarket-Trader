# Session Context

Generated at UNIX epoch seconds: `1772420340`

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
Added cross-venue route ingestion in `pt-engine`:
- Dedicated Kraken/Gemini top-of-book loops feeding route books.
- Route merge now consumes `coinbase + kraken + gemini` with venue-prefixed leg IDs.
- `pt-route` now supports prefixed products (`coinbase:BTC-USD`) and deterministic product iteration for stable route ordering.
Added venue-specific route fee modeling:
- `execution.fees.kraken` and `execution.fees.gemini` are now supported with defaults.
- Route net-edge now applies maker fee by leg venue instead of a single shared maker fee.
Added route quality penalties:
- Dynamic reject-risk penalty from rolling 10m execution reject ratio.
- Dynamic latency-decay penalty from Coinbase L2 freshness relative to `execution.stale_book_ms`.
Added local realtime homebase expansion:
- Basic/Advanced dashboard mode (`/state/ui/mode`, `/ops/ui/mode`).
- Cross-exchange shadow summary (`/state/crossvenue/shadow-summary`).
- Downturn strategy summary (`/state/strategy/downturn-summary`).
- Capital planner + close-day workflow (`/state/capital/plan`, `/ops/capital/close-day`, `/state/capital/ledger`).
- Equity capability probe + paper run surfaces (`/state/equities/universe`, `/state/equities/paper-runs`).
- Added config blocks: `ui`, `capital_plan`, `equities`.
- Added SQLite runtime tables for capital/equity persistence.

## 2026-04-26 Next Round Checkpoint
- reviewed current `main`, open PRs #11, #12, #13, and #14, plus issues #9 and #10
- confirmed the repo is still in Phase 1 and the next implementation blocker is restart-safe approval-queue persistence
- queued the next focused coding slice in `docs/PHASE1_NEXT_ROUND_2026-04-26.md`
- recommended merge/order sequence:
  - PR #12 evidence gate
  - PR #13 read-only approval queue API
  - PR #11 frontend fixture safety net
  - PR #14 close or supersede after implementation work is active
  - next new code PR: issue #9 SQLite-backed workstation order persistence and startup hydration
- parallel analysis results agreed on the narrowest safe path:
  - persist workstation order lifecycle state using `storage.sqlite_path`
  - hydrate persisted orders into runtime startup state
  - keep execution authority unchanged and avoid new mutating approval APIs
  - validate with create/update/reload coverage before any merge/deploy decision

## Runtime
`rustc`: `rustc 1.93.1 (01f6ddf75 2026-02-11)`
`cargo`: `cargo 1.93.1 (083ac5135 2025-12-15)`

## Core Commands
- Run engine: `cargo run -p pt-cli -- run --config config/config.toml`
- Run homebase mode: `cargo run -p pt-cli -- run-homebase --config config/config.toml`
- Run exec-only mode: `cargo run -p pt-cli -- run-exec --config config/config.toml`
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
- Route opportunities (venue filtered): `curl -s "http://127.0.0.1:8080/state/routes/opportunities?venue_set=coinbase,kraken,gemini" | jq '.[0:5]'`
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

## 2026-04-26 Phase 1 checkpoint

### Current repo queue
- PR #11: fixture-backed dashboard frontend tests for the current API surface.
- PR #12: hardened Phase 1 evidence bundle and 3-run gate workflow.
- PR #13: read-only `/api/v1/approval-queue` operator API derived from queue-relevant order states.
- Issue #9 remains the next clean blocker after those slices: durable approval-queue persistence and restart recovery.

### Current safest next action
- Persist workstation order state through `storage.sqlite_path` and hydrate it on Coinbase workstation startup.
- Keep read models in `/api/v1/orders` and, when merged, `/api/v1/approval-queue` strictly read-only.
- Do not add approval, execute, or autonomous live-routing endpoints.

### Acceptance criteria for the next code slice
- `draft`, `cancel_requested`, `open`, `filled`, `canceled`, and `rejected` workstation orders survive restart.
- Reload restores deterministic queue state before the dashboard serves traffic.
- Queue visibility remains operator-facing only and does not change execution authority.
- Tests cover create, update, and reload behavior.

### Validation ladder still required before merge or deploy
1. `cargo fmt --all`
2. `cargo check --workspace`
3. `cargo clippy --workspace --all-targets --all-features -- -D warnings`
4. `cargo test --workspace`
5. `cargo build --workspace`
6. `cargo audit`
7. `./scripts/generate_sbom.sh artifacts`
8. `python3 tools/coinbase_strategy_lab.py backtest --config config/coinbase_strategy_lab.json`
9. `python3 tools/coinbase_strategy_lab.py overlap --config config/coinbase_strategy_lab.json --auto-discovery`
10. `python3 tools/coinbase_strategy_lab.py optimize --config config/coinbase_strategy_lab.json`
11. `cargo run -p pt-cli -- run --config config/config.toml`
12. `./scripts/paper_soak.sh 86400 30 config/config.toml`

### Known unknowns
- This workspace still does not provide a local Rust checkout and full validation path, so gate status is not yet freshly re-proven here.
- Do not treat open PRs or docs planning as evidence that Phase 1 ROI gates have passed.
- Live convert confirm phrase: `I_UNDERSTAND_LIVE_CONVERT`.
- Live maker-test confirm phrase: `I_UNDERSTAND_LIVE_MAKER_TEST`.
