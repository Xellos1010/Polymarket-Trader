# Data Schemas

## Runtime Tables (SQLite)

### `market_snapshots`
- `ts_ms` INTEGER NOT NULL
- `market_id` TEXT NOT NULL
- `token_id` TEXT NOT NULL
- `bid` REAL NOT NULL
- `ask` REAL NOT NULL
- `spread` REAL NOT NULL
- `liquidity` REAL NOT NULL

### `execution_reports`
- `ts_ms` INTEGER NOT NULL
- `portfolio_id` TEXT NULL
- `venue` TEXT NOT NULL
- `order_id` TEXT NOT NULL
- `market_id` TEXT NULL
- `side` TEXT NOT NULL
- `status` TEXT NOT NULL
- `filled_qty` REAL NOT NULL
- `avg_px` REAL NOT NULL
- `details` TEXT NULL

### `risk_events`
- `ts_ms` INTEGER NOT NULL
- `payload` TEXT NOT NULL (`RiskState` JSON)

### `coinbase_balances`
- `ts_ms` INTEGER NOT NULL
- `portfolio_id` TEXT NULL
- `venue` TEXT NOT NULL
- `account_id` TEXT NOT NULL
- `asset` TEXT NOT NULL
- `available` REAL NOT NULL
- `hold` REAL NOT NULL
- `usd_value` REAL NOT NULL

### `coinbase_orders`
- `ts_ms` INTEGER NOT NULL
- `portfolio_id` TEXT NULL
- `order_id` TEXT NOT NULL
- `product_id` TEXT NOT NULL
- `side` TEXT NOT NULL
- `status` TEXT NOT NULL
- `order_type` TEXT NOT NULL
- `average_filled_price` REAL NOT NULL
- `filled_size` REAL NOT NULL

### `rebalance_plans`
- `ts_ms` INTEGER NOT NULL
- `portfolio_id` TEXT NULL
- `plan_id` TEXT NOT NULL
- `status` TEXT NOT NULL
- `payload` TEXT NOT NULL (`RebalancePlan` JSON)

### `rebalance_actions`
- `ts_ms` INTEGER NOT NULL
- `portfolio_id` TEXT NULL
- `plan_id` TEXT NOT NULL
- `action` TEXT NOT NULL
- `payload` TEXT NOT NULL

### `execution_events`
- `ts_ms` INTEGER NOT NULL
- `portfolio_id` TEXT NULL
- `order_id` TEXT NOT NULL
- `venue` TEXT NOT NULL
- `market_id` TEXT NULL
- `product_id` TEXT NULL
- `state` TEXT NOT NULL
- `side` TEXT NOT NULL
- `price` REAL NOT NULL
- `qty` REAL NOT NULL
- `details` TEXT NULL

### `execution_costs`
- `ts_ms` INTEGER NOT NULL
- `portfolio_id` TEXT NULL
- `execution_id` TEXT NOT NULL
- `venue` TEXT NOT NULL
- `market_id` TEXT NULL
- `side` TEXT NOT NULL
- `qty` REAL NOT NULL
- `avg_px` REAL NOT NULL
- `reference_px` REAL NOT NULL
- `fee_bps` REAL NOT NULL
- `fee_est` REAL NOT NULL
- `slippage_bps` REAL NOT NULL
- `slippage_est` REAL NOT NULL
- `rebate_bps_est` REAL NOT NULL
- `rebate_est` REAL NOT NULL
- `effective_edge` REAL NOT NULL
- `strategy_class` TEXT NULL
- `route_id` TEXT NULL

### `replay_acceptance_reports`
- `ts_ms` INTEGER NOT NULL
- `artifact_path` TEXT NOT NULL
- `passed` INTEGER NOT NULL
- `fail_reasons` TEXT NOT NULL
- `total_reports` INTEGER NOT NULL
- `reject_error_rate` REAL NOT NULL
- `max_unhedged_delta` REAL NOT NULL
- `killswitch` TEXT NOT NULL
- `effective_fee_bps_avg` REAL NOT NULL
- `payload` TEXT NOT NULL (`ReplayAcceptanceReport` JSON)

### `coinbase_l2_events`
- `ts_ms` INTEGER NOT NULL
- `portfolio_id` TEXT NULL
- `product_id` TEXT NOT NULL
- `sequence_num` INTEGER NOT NULL
- `side` TEXT NOT NULL
- `price_level` REAL NOT NULL
- `new_quantity` REAL NOT NULL
- `event_time_ms` INTEGER NOT NULL

### `coinbase_user_events`
- `ts_ms` INTEGER NOT NULL
- `portfolio_id` TEXT NULL
- `order_id` TEXT NOT NULL
- `product_id` TEXT NOT NULL
- `status` TEXT NOT NULL
- `side` TEXT NOT NULL
- `post_only` INTEGER NOT NULL
- `avg_price` REAL NOT NULL
- `filled_qty` REAL NOT NULL
- `total_fees` REAL NOT NULL
- `payload` TEXT NOT NULL

### `order_manager_transitions`
- `ts_ms` INTEGER NOT NULL
- `portfolio_id` TEXT NULL
- `market_id` TEXT NULL
- `order_id` TEXT NULL
- `action` TEXT NOT NULL
- `reason` TEXT NOT NULL
- `target_price` REAL NOT NULL
- `target_size` REAL NOT NULL

### `route_opportunities`
- `ts_ms` INTEGER NOT NULL
- `portfolio_id` TEXT NULL
- `route_id` TEXT NOT NULL
- `strategy_class` TEXT NOT NULL
- `gross_edge_bps` REAL NOT NULL
- `expected_net_bps` REAL NOT NULL
- `expected_usd_profit` REAL NOT NULL
- `capital_required_usd` REAL NOT NULL
- `payload` TEXT NOT NULL

### `route_executions`
- `ts_ms` INTEGER NOT NULL
- `portfolio_id` TEXT NULL
- `route_id` TEXT NOT NULL
- `approved` INTEGER NOT NULL
- `reason` TEXT NULL
- `payload` TEXT NOT NULL

### `fee_tier_snapshots`
- `ts_ms` INTEGER NOT NULL
- `portfolio_id` TEXT NULL
- `maker_fee_rate` TEXT NULL
- `taker_fee_rate` TEXT NULL
- `total_fees` REAL NOT NULL
- `payload` TEXT NOT NULL

### `auth_key_events`
- `ts_ms` INTEGER NOT NULL
- `portfolio_id` TEXT NOT NULL
- `venue` TEXT NOT NULL
- `action` TEXT NOT NULL
- `profile_id` TEXT NULL
- `key_id_suffix` TEXT NULL
- `source` TEXT NULL
- `status` TEXT NOT NULL (`ok|error`)
- `reason` TEXT NOT NULL

## Strategy Lab Journal (SQLite)

Default file: `data/strategy_lab/trade_journal.sqlite`

### `lab_runs`
- `run_id` TEXT PRIMARY KEY
- `created_ts_ms` INTEGER NOT NULL
- `mode` TEXT NOT NULL
- `provider` TEXT NOT NULL
- `granularity_sec` INTEGER NOT NULL
- `config_json` TEXT NOT NULL

### `market_results`
- `run_id` TEXT NOT NULL
- `market` TEXT NOT NULL
- `variant` TEXT NOT NULL
- `total_return` REAL NOT NULL
- `max_drawdown` REAL NOT NULL
- `sharpe_like` REAL NOT NULL
- `trades` INTEGER NOT NULL
- `bars` INTEGER NOT NULL
- `pnl_abs` REAL NOT NULL
- `final_equity` REAL NOT NULL
- `created_ts_ms` INTEGER NOT NULL

### `trade_fills`
- `run_id` TEXT NOT NULL
- `market` TEXT NOT NULL
- `variant` TEXT NOT NULL
- `bar_idx` INTEGER NOT NULL
- `ts_ms` INTEGER NOT NULL
- `side` TEXT NOT NULL
- `price` REAL NOT NULL
- `delta` REAL NOT NULL
- `target_position` REAL NOT NULL
- `bias` REAL NOT NULL

## Rust Strategy Lab Store (SQLite)

Default file: `data/strategy_lab/strategy_lab.sqlite`

### `strategy_profiles`
- `profile_id` TEXT NOT NULL UNIQUE
- `name` TEXT NOT NULL
- `latest_version` INTEGER NOT NULL
- `updated_ts_ms` INTEGER NOT NULL

### `strategy_profile_versions`
- `profile_id` TEXT NOT NULL
- `version` INTEGER NOT NULL
- `created_ts_ms` INTEGER NOT NULL
- `note` TEXT NULL
- `payload` TEXT NOT NULL (`StrategyProfile` JSON)

### `strategy_runs`
- `run_id` TEXT NOT NULL UNIQUE
- `profile_id` TEXT NOT NULL
- `product_id` TEXT NOT NULL
- `ts_ms` INTEGER NOT NULL
- `total_return_pct` REAL NOT NULL
- `max_drawdown_pct` REAL NOT NULL
- `trades` INTEGER NOT NULL
- `win_rate` REAL NOT NULL
- `pnl` REAL NOT NULL
- `payload` TEXT NOT NULL (`StrategyRunReport` JSON)

### `indicator_series`
- `run_id` TEXT NOT NULL
- `ts_ms` INTEGER NOT NULL
- `indicator_name` TEXT NOT NULL
- `bias` REAL NOT NULL
- `confidence` REAL NOT NULL
- `regime` TEXT NOT NULL
- `payload` TEXT NOT NULL (`IndicatorSignal` JSON)

### `signal_series`
- `run_id` TEXT NOT NULL
- `ts_ms` INTEGER NOT NULL
- `score` REAL NOT NULL
- `action` TEXT NOT NULL
- `confluence` INTEGER NOT NULL
- `regime` TEXT NOT NULL
- `payload` TEXT NOT NULL (`FusionDecision` JSON)

### `regime_series`
- `run_id` TEXT NOT NULL
- `ts_ms` INTEGER NOT NULL
- `regime` TEXT NOT NULL

### `paper_endpoint_reports`
- `ts_ms` INTEGER NOT NULL
- `profile_id` TEXT NOT NULL
- `simulated_orders` INTEGER NOT NULL
- `simulated_edits` INTEGER NOT NULL
- `simulated_cancel_replace` INTEGER NOT NULL
- `estimated_reject_rate` REAL NOT NULL
- `notes` TEXT NOT NULL
- `payload` TEXT NOT NULL (`PaperEndpointReport` JSON)

## API Contract Source
- OpenAPI: `docs/api/dashboard-openapi.yaml`
- Config JSON schema: `schemas/config.schema.json`
- TradingView webhook schema: `schemas/tradingview-webhook.schema.json`
- Coinbase strategy lab schema: `schemas/coinbase_strategy_lab.schema.json`
- Rust strategy lab schema: `schemas/strategy_lab.schema.json`
- Prompt bundle export config schema: `schemas/prompt_bundle.schema.json`
