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

## API Contract Source
- OpenAPI: `docs/api/dashboard-openapi.yaml`
- Config JSON schema: `schemas/config.schema.json`
- TradingView webhook schema: `schemas/tradingview-webhook.schema.json`
- Coinbase strategy lab schema: `schemas/coinbase_strategy_lab.schema.json`
- Prompt bundle export config schema: `schemas/prompt_bundle.schema.json`
