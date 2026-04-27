# Progress

## Completed (Merged on `main`)

- Local realtime homebase expansion pass completed (Coinbase-first + cross-exchange shadow + capital/equity lab):
  - Config/runtime contracts added:
    - `ui.mode_default`
    - `capital_plan.enabled`, `capital_plan.daily_contribution_usd`, `capital_plan.tiers`
    - `equities.products`, `equities.paper.enabled`, `equities.live.enabled`
    - active `config/config.toml` now includes `[venues.kraken]` and `[venues.gemini]` shadow blocks.
    - new profile: `config/profiles/local_paper_guarded_realtime.toml`
  - Dashboard API surfaces added:
    - `GET /state/ui/mode`
    - `POST /ops/ui/mode`
    - `GET /state/crossvenue/shadow-summary`
    - `GET /state/strategy/downturn-summary`
    - `GET /state/capital/plan`
    - `POST /ops/capital/close-day`
    - `GET /state/capital/ledger`
    - `GET /state/equities/universe`
    - `GET /state/equities/paper-runs`
  - Dashboard UI updates:
    - Basic/Advanced mode toggle with local persistence.
    - New cards for cross-venue summary, downturn summary, capital planner close-day flow, equity universe, and equity paper runs.
    - Advanced diagnostics cards are hidden in Basic mode.
  - Engine/runtime updates:
    - shared state added for UI mode, capital ledger, equity universe, and equity paper runs.
    - equity probe loop added (Coinbase product capability snapshots).
    - capital-ledger async flush loop added (persists close-day ledger rows to SQLite).
  - SQLite schema expanded:
    - `capital_contributions`
    - `capital_reserve_actions`
    - `capital_daily_rollups`
    - `equity_products_snapshots`
    - `equity_paper_runs`
    - `equity_paper_trades`
    - `ui_preferences`
  - Validation status:
    - `cargo check --workspace` passes
    - `cargo test --workspace` passes
    - `cargo run -p pt-cli -- coinbase-smoke --config config/config.toml --timeout-ms 8000` passes
    - local runtime endpoint smoke passes for new state/ops endpoints.

- Route net-edge quality penalties added:
  - Route scoring now applies a dynamic reject-risk penalty (rolling 10m reject ratio from execution events).
  - Route scoring now applies a dynamic latency-decay penalty from Coinbase orderbook freshness vs `execution.stale_book_ms`.
  - New runtime metrics:
    - `routes_reject_ratio_10m`
    - `routes_reject_penalty_bps`
    - `routes_max_coinbase_book_age_ms`
    - `routes_latency_penalty_bps`
  - Validation status:
    - `cargo check --workspace` passes
    - `cargo test --workspace` passes

- Cross-venue route cost model upgraded to venue-specific maker fees:
  - Added `execution.fees.kraken` and `execution.fees.gemini` config support (with defaults + schema support).
  - Extended runtime execution policy with Kraken/Gemini fee schedules.
  - Route scoring now charges per-leg fee by venue prefix (`coinbase:`, `kraken:`, `gemini:`) instead of a single global maker fee.
  - Added route test coverage for venue-fee sensitivity (`venue_specific_fees_change_net_edge`).
  - Validation status:
    - `cargo check --workspace` passes
    - `cargo test --workspace` passes

- Cross-venue route ingestion wiring completed:
  - Added runtime Kraken/Gemini route-book refresh loops in `pt-engine`.
  - Added cross-venue merge in route engine (`coinbase + kraken + gemini`) before opportunity scoring.
  - Normalized route leg identifiers to prefixed format (`<venue>:<base>-<quote>`) for venue-aware filtering/export.
  - Added compact/prefixed pair parsing helpers for symbol normalization (`BTC-USD`, `btcusd`, `kraken:XBTUSD` style inputs).
  - Route parser now supports prefixed products and deterministic edge ordering.
  - Validation status:
    - `cargo check --workspace` passes
    - `cargo test --workspace` passes

- Master Optimization v4 baseline scaffolding pass completed:
  - Added multi-venue config contracts for `venues.kraken` and `venues.gemini` (schema + runtime config parsing/validation).
  - Added runtime/hardware-aware config contracts:
    - `runtime.affinity.*`
    - `runtime.jitter_controls.*`
    - `wallet_intel.*`
    - `benchmark.hotpath.*`
  - Added Linux ultra-tight profile template:
    - `config/profiles/live_linux_ultra_tight.toml`
  - Added host operation scripts:
    - `scripts/linux_tune_baseline.sh` (Linux kernel/network baseline tuning)
    - `scripts/install_homebase_service.sh` (macOS homebase daemon install via launchd)
  - Added dashboard/API surfaces for v4 observability and exports:
    - `GET /state/feed/diagnostics`
    - `GET /state/venues/latency`
    - `GET /state/venues/fill-quality`
    - `GET /state/venues/rejects`
    - `POST /state/routes/export-csv`
    - `GET /state/wallet-intel/coinbase`
    - `GET /state/wallet-intel/polymarket`
    - `GET /state/wallet-intel/leaderboard`
    - `POST /state/wallet-intel/export-csv`
  - Added `pt-core` v4 cross-venue and benchmark data types:
    - `VenueCapability`, `VenueLatencyStats`, `VenueFillQualityStats`
    - `CrossVenueRouteOpportunity`, `WalletIntelSnapshot`, `HotPathBenchmarkReport`
  - Added workspace adapter scaffolds:
    - `crates/pt-kraken`
    - `crates/pt-gemini`

- Dashboard listing-pattern + feed-integrity visibility pass completed:
  - Added `LISTING PATTERN` tab with Coinbase auto-discovered listing cohorts and overlay visualization.
  - Added listing controls for window (`30D/90D/180D`), granularity (`1H/4H/1D`), alignment (`Time Entered/Start All/Calendar`), and scale (`Indexed/Returns`).
  - Added listing overlay CSV export from dashboard (`EXPORT CSV` button) for external analysis.
  - Added listing backend endpoints:
    - `GET /state/listings/candidates`
    - `POST /state/listings/overlay`
    - `GET /state/listings/l2-archive`
  - Added feed/parity visibility endpoints to dashboard runtime loop:
    - `GET /state/feed/health`
    - `GET /state/parity/monitor`
  - Added UI cards:
    - `Feed Health`
    - `Parity Monitor`
  - Parity monitor table now shows gross edge, net edge, implied cost (`gross-net`), minimum required bps, and expected USD for clearer go/no-go math.
  - Added deterministic unit tests for listing helper normalization:
    - `parse_window_preset`
    - `normalize_granularity`
    - `normalize_alignment_mode`
    - `normalize_series_mode`
  - Added deterministic overlay math tests (`overlay_values`, `sample_return`, `summary_returns`) to keep listing-overlay behavior stable without network reliance.
  - Added `EXPORT PARITY CSV` for parity monitor rows.
  - Added server-side parity CSV export endpoint:
    - `POST /state/parity/export-csv`
    - supports `limit` + `include_failures`, writes artifacts to `PT_OUTPUT_DIR` or `data/output`.
  - Hardened Coinbase WS event loop:
    - timeout streak + keepalive ping before reconnect
    - heartbeat-aware reconnect gating to reduce timeout churn
    - feed stale fallback now uses latest of heartbeat or L2 timestamp.
  - Feed health now exposes WS reliability counters:
    - timeout streak
    - read timeouts
    - ping failures
    - heartbeat timeouts
    - remote closes
    - read/connect failures
  - Contract/test/docs sync:
    - `docs/api/dashboard-openapi.yaml` updated with listing/feed/parity schemas.
    - `crates/pt-dashboard/tests/api_contract.rs` updated and passing.
  - Validation status:
    - `cargo check --workspace` passes
    - `cargo test --workspace` passes
    - runtime smoke: `/health`, `/state/feed/health`, `/state/parity/monitor`, `/state/listings/candidates`, `/state/listings/overlay` all return successful responses locally.

- Dashboard trading-ops UX expansion completed:
  - Market selector now renders human-readable pair/bucket labels (not raw ids).
  - Added chart/backtester tab controls with embedded strategy-lab iframe (`127.0.0.1:9090`).
  - Added selected-market granularity aggregation with delta bar chart.
  - Added selected-pair Coinbase orderbook depth panel (bids/asks ladder).
  - Added wallet conversion controls (`preview`, `paper execute`, guarded `live execute`) backed by:
    - `POST /ops/coinbase/convert/preview`
    - `POST /ops/coinbase/convert/execute`
  - Added maker speed-test controls (`paper` + guarded `live`) backed by:
    - `POST /ops/coinbase/maker-test`
  - Validation status:
    - `cargo check --workspace` passes
    - `cargo test --workspace` passes
    - dashboard root serves as HTML (`content-type: text/html`)
    - paper API smoke for convert + maker-test passes locally

- Coinbase runtime blocker patch and verification complete:
  - rustls crypto provider bootstrap added and enforced before live HTTP/WS initialization.
  - Coinbase EC key handling hardened for CDP PEM inputs with SEC1 normalization and SEC1->PKCS8 fallback.
  - Live preflight now validates Coinbase JWT generation and authenticated account probe.
  - `coinbase-smoke` command added and verified passing in read-only mode (REST + WS + fee summary).
  - WS smoke criteria fixed to accept valid subscription/heartbeat/L2 state even with no user-fill activity.
  - Rebalance/order-manager path now uses lifecycle decisions (`submit`, `edit`, `cancel_replace`) with open-order restoration and reconciliation hooks (`get_order`, `list_fills`).

- Rust strategy lab foundation implemented (`crates/pt-strategy-lab`):
  - Coinbase candle ingestion, modular indicator engine, weighted confidence fusion, bull/bear/neutral regime gating.
  - Next-bar fill backtest model with fee/slippage cost accounting.
  - Random-search + walk-forward optimization loop.
  - SQLite persistence for profiles/runs/signals/regimes/paper reports.
  - Axum dashboard/API with endpoints:
    - `GET /lab/state/profile`
    - `POST /lab/profile/save`
    - `POST /lab/profile/load`
    - `POST /lab/backtest/run`
    - `POST /lab/optimize/run`
    - `GET /lab/state/indicators`
    - `GET /lab/state/signals`
    - `GET /lab/state/regime`
    - `GET /lab/state/runs`
  - New CLI commands:
    - `strategy-lab-serve`
    - `strategy-backtest`
    - `strategy-optimize`
    - `strategy-profile-save`
    - `strategy-profile-load`

- Coinbase multi-portfolio auth registry + rotation implementation pass:
  - Profile-based Coinbase auth config added (`venues.coinbase.auth.*`) with env overrides:
    - `COINBASE_AUTH_PROFILE`
    - `COINBASE_CDP_KEY_FILE`
    - `COINBASE_CDP_SECRET_ID`
    - `COINBASE_EXPECTED_KEY_ID`
  - CDP JSON resolver added for `{name, privateKey}` with key-id extraction + live key-id pin checks.
  - AWS Secrets Manager runtime fetch support added for `cdp_secret_id`.
  - Shared auth manager wired into Coinbase REST + WS clients with runtime generation-based WS reconnect on auth swap.
  - Dashboard/operator auth endpoints added:
    - `GET /state/coinbase/auth`
    - `POST /ops/coinbase/auth/reload`
    - `POST /ops/coinbase/auth/switch-profile`
  - CLI auth commands added:
    - `coinbase-auth-status`
    - `coinbase-auth-reload`
    - `coinbase-auth-switch --profile <id>`
  - Portfolio isolation support added:
    - `engine.portfolio_id`
    - sqlite/parquet portfolio-scoped path derivation
    - `portfolio_id` persisted in key runtime tables
  - Auth audit persistence added:
    - `auth_key_events` table with reload/switch/startup events
  - Security hardening:
    - `config/cdp_api_key.json` added to `.gitignore`

- Roadmap v3 core implementation pass completed for Coinbase-first maker stack:
  - Coinbase REST adapter now includes `preview`, `edit`, `batch_cancel`, `get_order`, `list_fills`, `get_product`, `get_product_book`, `get_best_bid_ask`, `get_transaction_summary`.
  - Coinbase WebSocket loop added with `level2`, `user`, `heartbeats`, reconnect handling, timeout detection, and sequence-gap events.
  - In-memory Coinbase orderbook state is now maintained and exposed to dashboard/API.
  - Route engine (`pt-route`) is now wired into runtime with opportunity scoring using dynamic edge profiles.
  - Order manager (`pt-order-manager`) is now wired into rebalance execution with preview-first gating and transition persistence.
  - New SQLite runtime tables are live:
    - `coinbase_l2_events`, `coinbase_user_events`
    - `order_manager_transitions`
    - `route_opportunities`, `route_executions`
    - `fee_tier_snapshots`
  - New dashboard/operator endpoints are live:
    - `GET /state/coinbase/orderbook`
    - `GET /state/routes/opportunities`
    - `GET /state/routes/executions`
    - `GET /state/fees/summary`
    - `POST /ops/profile/pilot-ultra-tight`
    - `POST /ops/unwind/now` (alias)
  - Dashboard UI now visualizes:
    - Coinbase L2 top-of-book
    - Route opportunities and route execution plans
    - Maker/taker fee summary KPI
  - CLI added:
    - `coinbase-ws-status`, `order-manager-status`, `routes-status`
    - `set-edge-profile`
    - `pilot-start --capital ... --profile ultra-tight`
  - Schema and contracts updated:
    - `schemas/config.schema.json` now covers Coinbase WS, pilot risk profile, execution edge profiles, and order-manager config.
    - `docs/api/dashboard-openapi.yaml` updated for new endpoints/types.
    - `docs/data/SCHEMA.md` updated for expanded runtime tables.
  - Validation status:
    - `cargo check --workspace` passes
    - `cargo test --workspace` passes

- Rust workspace baseline for Polymarket + Coinbase hedge engine is in place.
- Local-first safeguards and runbooks are in place (no CI/CD required before local validation).
- Master Plan v2 execution/wallet contracts are now implemented end-to-end:
  - `execution` config block (maker-first policy, vector bands, fee schedules)
  - `wallet` config block (assist/auto modes, target allocations, rebalance controls, approval TTL)
  - `acceptance.replay` config thresholds
- Coinbase wallet-first flow is active in engine runtime:
  - wallet balances + open-order sync loop
  - allocation drift computation
  - rebalance plan generation
  - approval-token-gated execution path
- Real-time execution policy wiring is active:
  - vector-gated quote acceptance
  - maker-first post-only intent handling
  - cancel/replace cooldown + min-rest guards
  - emergency unwind pathway guardrails
- New persistence tables are live in SQLite:
  - `coinbase_balances`, `coinbase_orders`
  - `rebalance_plans`, `rebalance_actions`
  - `execution_events`, `execution_costs`
  - `replay_acceptance_reports`
- Dashboard/ops surface now includes execution + wallet control plane:
  - `/state/execution/orders`, `/state/execution/costs`, `/state/execution/vectors`
  - `/state/coinbase/wallet`, `/state/coinbase/allocations`, `/state/coinbase/rebalance-plan`, `/state/coinbase/orders`
  - `/ops/coinbase/rebalance/approve`, `/ops/coinbase/rebalance/reject`, `/ops/execution/unwind`
- Dashboard UI now renders:
  - execution vectors + emergency unwind control
  - execution cost attribution table
  - coinbase wallet/drift table
  - coinbase open orders table
  - rebalance plan summary + approve/reject controls
- CLI now includes:
  - `wallet-status`, `wallet-plan`, `wallet-approve`
  - `execution-status`
  - `verify-promoted`
  - `report-variants`
- Coinbase strategy lab now supports:
  - backtest
  - overlap (candle-aligned listing analysis)
  - optimize
  - unified dashboard
  - comparative CSV/Markdown report export
- Listing overlap now supports auto-discovery of likely recent Coinbase listings.
- Listing overlap now ranks post-anchor cohorts by impulse and volatility buckets.
- Strategy variants now support a plugin interface:
  - `external_bias_file` (Pine/AI bias series input)
  - `tradingview_webhook_file` (direct TradingView webhook snapshot replay input)
  - `momentum_bias`
  - `rsi_bias`
- Persistent SQLite trade journal now records runs/trades and exposes per-market attribution summaries.
- Strategy-lab promotion tooling now converts selected market/variant into replay NDJSON for Rust replay mode.
- Contract and schema sync completed:
  - `schemas/config.schema.json`
  - `docs/api/dashboard-openapi.yaml`
  - `crates/pt-dashboard/tests/api_contract.rs`
  - `docs/data/SCHEMA.md`
- Validation status:
  - `cargo check --workspace` passes
  - `cargo test --workspace` passes
- Deterministic risk and quote failure-path tests merged in PR `#26`.

## In Progress

- Live Coinbase wallet integration validation on real keys (assist mode, tiny notional, approval workflow).
- WebSocket auth/ops hardening against live subscription constraints per API key scope.
- Route execution promotion from candidate plans to guarded live multi-leg sequencing.
- Canonical next-round checkpoint published in `docs/PHASE1_NEXT_ROUND_2026-04-27.md`.

## Next Queue

1. Implement issue `#9` persistence and restart-reload tests in one small PR.
2. Run the local-first validation ladder on the active Phase 1 stack in a Rust-enabled environment:
   - `cargo fmt --all`
   - `cargo check --workspace`
   - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
   - `cargo test --workspace`
   - `cargo build --workspace`
   - `cargo audit`
   - `./scripts/generate_sbom.sh artifacts`
3. Keep the approval queue read-only while adding backend durability.
4. Continue toward replay/paper repeatability proof with at least three independent positive-cost-modeled runs before any tiny-live recommendation.

## Current Audit (2026-04-27)

- Repo remains in **Phase 1: sandbox trading / paper ROI**.
- Recently merged work reduced safety gaps but did not change the next runtime blocker:
  - `#11`, `#12`, `#13`, `#18`, and `#26` are merged on `main`.
- The next concrete implementation blocker remains issue `#9`: workstation approval-queue state is still memory-only in the Coinbase workstation runtime.
- The safe next slice is backend durability and startup hydration, not more UI expansion.
- Fresh local gate status is still unknown from this control-tower cycle because this workspace did not have a normal authenticated local checkout for the private repo.

## Current audit finding

- The repository is still in Phase 1: sandbox trading / paper ROI.
- The next concrete runtime blocker remains issue `#9`: queue-relevant workstation orders do not survive restart.
- `docs/PHASE1_NEXT_ROUND_2026-04-27.md` is now the canonical checkpoint for the next implementation round.

## Validation ladder

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
- Validate promoted replay artifacts against `pt-cli` replay mode in a Rust-enabled environment.
- Capture at least three independent replay/paper evidence bundles and generate a Phase 1 gate report.
- Wire hosted branch protection/manual approval settings in GitHub.
- Add durable approval-queue persistence behind the existing read-only operator queue surface.
- Run end-stage Rust/frontend validation for the expanded dashboard surface.
