# Service Provider Matrix

## Goal

Benchmark the most credible providers and product patterns to emulate or integrate while keeping the platform deterministic, replayable, and safe.

## Integration principles

- Prefer official or primary data for execution-critical state.
- Treat third-party enrichment as advisory unless it is replayable and policy-covered.
- Avoid provider lock-in at the domain layer.
- Keep every provider behind an adapter interface and record provenance in the audit trail.

## Provider matrix

| Domain | Preferred providers | Why they matter | Integration shape | Safety notes |
|---|---|---|---|---|
| Coinbase trading and wallet ops | Coinbase Advanced Trade, CDP, AgentKit | Official product status, execution venue, wallet and agent primitives | Native adapters in `pt-coinbase`; signed requests; typed product state models | Execution-critical. Prefer direct official sources only. |
| Polymarket | Native Polymarket REST + websocket | Existing prediction-market support and replay path | Keep current adapters; extend normalized event model | Keep isolated from Coinbase workstation logic where semantics differ. |
| DEX route quotes | 0x for EVM, Jupiter for Solana, Uniswap routing data | Best route simulation before multi-chain trading goes live | Quote adapters feeding simulation first, live routing later | No live execution until bridge, gas, and settlement policy are modeled. |
| Charting and operator overlays | TradingView | Best operator-facing chart UX and Pine ecosystem | Import alert snapshots and parameters, never let Pine bypass policy gates | Pine is a signal source, not an execution authority. |
| Bot/operator UX benchmark | Hummingbot, 3Commas | Strong patterns for strategy control, inventory visibility, and automation settings | Borrow UX patterns, not opaque execution logic | Keep all execution logic local and replayable. |
| Onchain analytics | Dune, DeFiLlama, Token Terminal | High-signal fundamentals, TVL, fees, protocol activity | Read-only analytics adapters with timestamped snapshots | Treat as research context unless data is refreshed and versioned. |
| Wallet/entity intelligence | Nansen, Arkham | Entity labels and money-flow context | Evidence adapters feeding wallet intel and confidence scores | Labels can be wrong. Keep confidence and provenance explicit. |
| RPC and chain data | Alchemy, QuickNode, Helius, native RPC | Reliable chain events and route-state context | Adapter-per-chain with health scoring and replay storage | Never make routing decisions off a single degraded RPC. |
| Alerts and incidenting | Slack, PagerDuty, Sentry, OpenTelemetry | Operator awareness and postmortems | Alert fan-out from dashboard/metrics pipeline | Alerting is additive; it must not replace hard stops. |

## Best-in-market product patterns to borrow

### Coinbase

- Explicit trading lifecycle states.
- Strong wallet and credential boundaries.
- Clean product metadata and per-asset status surfaces.

### TradingView

- Dense but readable chart workspace.
- Script parameter workflows.
- Clear alerting and watchlist ergonomics.

### Hummingbot

- Strategy-specific configuration with hard guardrails.
- Inventory-aware execution language.
- Operator-first automation controls.

### 3Commas

- Approval-friendly bot management flows.
- Portfolio level visibility across strategies.
- Clear distinction between signal generation and execution configuration.

### Dune / Nansen / Arkham / DeFiLlama

- Research workspaces that blend overview metrics with drill-down evidence.
- Saved views and repeatable intelligence panels.
- Clear provenance for external analytics.

## Initial implementation order

1. Coinbase official status and execution surfaces.
2. TradingView and strategy-lab research import paths.
3. Dune and DeFiLlama listing-radar research adapters.
4. 0x and Jupiter in quote-simulation mode only.
5. Wallet/entity adapters with explicit confidence scoring.
6. Incident destinations and traces once deployment target is chosen.
