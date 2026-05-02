# System Architecture

```mermaid
flowchart TD
A[Market Discovery] --> B[Universe Filter]
B --> C[Orderbook Ingestion]
C --> D[Signal Fusion<br/>Wallet + TradingView]
D --> E[Quote Engine]
E --> F[Risk Engine + Kill Switch]
F --> G[Polymarket Execution]
G --> H[Coinbase Hedge]
G --> I[PnL + Cost Attribution]
H --> I
I --> J[Dashboard + Metrics + Storage]
J --> B
```

## Runtime Topology
- Single process (`pt-engine` via `pt-cli run`, or the `pt-cli coinbase up` workstation path) on one host.
- Async tasks: discovery, wallet refresh, books, quote loop, watchdog, dashboard, optional TradingView listener.

## Crate map (high level)
- Venue adapters: `pt-coinbase`, `pt-polymarket`, optional `pt-kraken` / `pt-gemini`.
- Strategy research: `pt-strategy-lab` (library + `/lab/*` HTTP surface); operator promotion and batch backtests also use `tools/coinbase_strategy_lab.py`.
- Execution helpers: `pt-order-manager`, `pt-route`, `pt-quote`, `pt-risk`, `pt-replay`.

## State/Storage
- SQLite (WAL): snapshots, execution reports, risk events.
- Parquet roll files: snapshot archives.
- In-memory rolling buffers: orderbooks, executions, per-market history.
