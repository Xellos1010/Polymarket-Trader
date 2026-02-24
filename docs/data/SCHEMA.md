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

## API Contract Source
- OpenAPI: `docs/api/dashboard-openapi.yaml`
- Config JSON schema: `schemas/config.schema.json`
- TradingView webhook schema: `schemas/tradingview-webhook.schema.json`
