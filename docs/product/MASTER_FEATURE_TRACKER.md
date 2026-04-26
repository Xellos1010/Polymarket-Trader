# Master Feature Tracker

## Purpose

This is the master backlog for evolving Polymarket Trader into a Coinbase-first, multi-venue, multi-chain, AI-enabled trading operating system.

The platform should not be built as an unconstrained AI bot. It should be a deterministic execution, risk, wallet, portfolio, and audit platform with an AI supervision layer on top. Deterministic systems own ingestion, simulation, policy checks, signing, routing, fills, storage, replay, and kill switches. AI ranks opportunities, explains decisions, detects anomalies, proposes parameter changes, and escalates high-risk situations.

## Product north star

Build the most capable personal crypto trading command center:

- Coinbase-first launch.
- Pre-listing and listing intelligence as the first alpha wedge.
- Maker-first order placement with state-dependent taker authorization.
- Multi-chain pre-Coinbase execution readiness.
- Event-driven and replayable strategy engine.
- Bounded autonomous AI agent running 24/7.
- Institutional-grade UI, risk cockpit, and audit trail.

## Priority ladder

| Tier | Name | Meaning | Default gate |
|---|---|---|---|
| P0 | Safety and determinism | Required before any capital is at risk | unit + replay + paper evidence |
| P1 | Coinbase-first trading | Required for initial bot operation | paper, tiny-live, kill switch |
| P2 | Listing radar alpha | Required for new supported-pair strategy | public listing ingest + onchain scoring |
| P3 | Portfolio and AI agent | Required for 24/7 operation | bounded autonomy and approvals |
| P4 | Multi-chain execution | Required before pre-Coinbase DEX trading | route simulation + policy wallet |
| P5 | Performance mode | Required for latency-sensitive operation | benchmark and deployment profile |

## Capability groups

### 1. Market data and venue ingestion

| ID | Feature | Priority | Status | Notes |
|---|---:|---:|---|---|
| MD-001 | Coinbase product/ticker/orderbook/trade/candle ingest | P1 | partial | Existing Coinbase strategy lab and adapters should become unified stream sources. |
| MD-002 | Coinbase listing-roadmap and announcement watcher | P2 | planned | Watch Coinbase roadmap, @CoinbaseMarkets announcements, transfer-only, auction, limit-only, full trading states. |
| MD-003 | Multi-venue normalized orderbook schema | P1 | planned | One schema for Coinbase, Polymarket, CEXs, DEX quoted routes, and replay. |
| MD-004 | Market data quality scoring | P1 | planned | Latency, staleness, dropped updates, crossed book, sequence gap. |
| MD-005 | Event-sourced raw feed store | P1 | planned | Persist raw events before normalized derived state. |
| MD-006 | Replayable market-data backfill | P1 | partial | Existing replay can be extended into deterministic market-data playback. |
| MD-007 | Exchange health monitor | P1 | planned | Websocket liveness, REST fallbacks, rate limits, auth status. |
| MD-008 | Chain event subscriptions | P3 | planned | Coinbase/CDP, Dune, Alchemy/QuickNode, Moralis, Helius, or native RPC adapters. |
| MD-009 | DEX price and route quotes | P4 | planned | 0x for EVM, Jupiter for Solana, Uniswap route/liquidity data. |
| MD-010 | News and social event ingest | P3 | planned | Official accounts only for execution-impacting events; broader social as sentiment signal. |

### 2. Strategy families

| ID | Feature | Priority | Status | Notes |
|---|---:|---:|---|---|
| ST-001 | SMA baseline and plugin variants | P1 | partial | Strategy lab has SMA/plugin foundation. |
| ST-002 | Coinbase listing event strategy | P2 | planned | Roadmap/announcement/transfer-only/auction/full-trading lifecycle. |
| ST-003 | Maker-first orderbook strategy | P1 | planned | Passive quote, queue-risk, adverse-selection, inventory-aware widths. |
| ST-004 | Trend-triggered taker entry | P1 | planned | Only allowed when signal confidence, volatility, liquidity, and urgency clear policy gates. |
| ST-005 | Taker flatten/exit strategy | P1 | planned | Inventory breach, alpha decay, risk-off, stale hedge, crash/rip exit. |
| ST-006 | Cross-venue arbitrage | P3 | planned | Coinbase vs DEX/CEX price route with fees, settlement, and balance constraints. |
| ST-007 | Basis/carry strategy | P4 | planned | Spot/futures/funding relative value once derivative venues exist. |
| ST-008 | Statistical arbitrage | P4 | planned | Cointegration, sector baskets, z-score, capacity controls. |
| ST-009 | Volatility breakout strategy | P2 | planned | Useful around listing windows and macro events. |
| ST-010 | Mean reversion strategy | P3 | planned | Needs robust regime classifier to avoid trend traps. |
| ST-011 | Onchain whale/smart-money follow strategy | P3 | planned | Treat as signal, never direct copy-trade without policy gate. |
| ST-012 | Unlock/vesting event strategy | P2 | planned | Float expansion, cliffs, investor unlocks, market-maker flows. |
| ST-013 | Liquidity migration strategy | P4 | planned | DEX pool depth, bridge route, CEX listing rotations. |
| ST-014 | Pine Script import and alert strategy | P2 | partial | Existing Pine parameter tooling should feed production-safe signals. |
| ST-015 | Strategy ensemble ranker | P3 | planned | AI/rules rank candidate strategies by regime and risk budget. |

### 3. Execution and order management

| ID | Feature | Priority | Status | Notes |
|---|---:|---:|---|---|
| EX-001 | Unified order intent model | P1 | partial | Quote intent exists; expand to venue-agnostic order intent. |
| EX-002 | Maker/taker policy engine | P1 | planned | Per-strategy, per-symbol, per-regime execution mode. |
| EX-003 | Post-only maker order router | P1 | partial | Polymarket has post-only semantics; Coinbase path must be explicit. |
| EX-004 | Taker order router | P1 | planned | Disabled by default until audited and policy-covered. |
| EX-005 | Dynamic order slicing | P2 | planned | Notional, time, depth, volatility, and urgency based. |
| EX-006 | Cancel/replace engine | P1 | planned | Rate-limit aware, queue-risk aware, anti-churn. |
| EX-007 | Fill and partial-fill tracking | P1 | planned | Required for PnL, inventory, and replay. |
| EX-008 | Slippage and TCA engine | P2 | planned | Expected vs realized spread, fees, market impact, adverse selection. |
| EX-009 | Venue fee model | P1 | planned | Maker/taker, tier, gas/priority fee, RFQ spread, bridge cost. |
| EX-010 | Smart order router | P4 | planned | Route across Coinbase, DEX aggregators, direct pools, future CEXs. |
| EX-011 | RFQ route support | P4 | planned | 0x RFQ and Jupiter RFQ if access granted. |
| EX-012 | Bridge-aware execution planner | P4 | planned | Do not hide trust assumptions, delays, or failure modes. |
| EX-013 | Execution simulator | P1 | planned | Same order intent should run in replay, paper, and live. |
| EX-014 | Execution policy audit log | P0 | planned | Store every allow/mutate/reject reason. |

### 4. Coinbase listing radar and pre-listing intelligence

| ID | Feature | Priority | Status | Notes |
|---|---:|---:|---|---|
| LR-001 | Listing lifecycle state machine | P2 | planned | Rumor, roadmap, announced, transfer-only, auction, limit-only, full trading, post-launch. |
| LR-002 | Official announcement ingestion | P2 | planned | Coinbase blog/support/X channels only for execution-critical status. |
| LR-003 | Asset metadata profile | P2 | planned | Chain, contract, supply, float, TVL, holders, wallets, venues. |
| LR-004 | Pre-Coinbase venue map | P2 | planned | DEX pools, CEX venues, bridge routes, liquidity fragmentation. |
| LR-005 | Tokenomics and unlock model | P2 | planned | Cliff/linear unlocks, circulating supply impact, investor/team allocation. |
| LR-006 | Holder and wallet growth monitor | P2 | planned | Track Coinbase-relevant listing factors. |
| LR-007 | TVL and protocol activity monitor | P2 | planned | TVL, revenue, fees, active users, chain activity. |
| LR-008 | Sentiment and narrative ranker | P3 | planned | Social/news only as non-deterministic signal. |
| LR-009 | Listing event backtester | P2 | planned | Recreate historical Coinbase listing windows. |
| LR-010 | Entry/exit policy templates | P2 | planned | Maker accumulation, taker breakout, taker flatten, no-trade. |
| LR-011 | High-yield short-horizon score | P2 | planned | Composite score with liquidity, float, demand, route risk, volatility. |
| LR-012 | Pre-listing trade approval workflow | P2 | planned | Higher approval threshold before multi-chain capital deployment. |

### 5. Onchain intelligence

| ID | Feature | Priority | Status | Notes |
|---|---:|---:|---|---|
| OC-001 | Wallet/entity labeling provider adapters | P3 | planned | Nansen, Arkham, internal heuristics, exchange/treasury tags. |
| OC-002 | Dune query integration | P3 | planned | Saved queries, materialized views, dashboards, API-driven metrics. |
| OC-003 | DeFiLlama fundamentals adapter | P2 | planned | TVL, volume, fees, revenue, stablecoin, unlocks/yields where available. |
| OC-004 | Token Terminal fundamentals adapter | P3 | planned | Fees, revenue, users, valuations for protocol scoring. |
| OC-005 | Smart money flow monitor | P3 | planned | Accumulation/distribution by labeled cohorts. |
| OC-006 | Exchange inflow/outflow monitor | P3 | planned | Centralized venue risk and sell pressure. |
| OC-007 | Liquidity pool monitor | P3 | planned | Depth, tick concentration, LP adds/removes, fee tiers. |
| OC-008 | Contract risk annotations | P3 | planned | Ownership, proxy, pausability, taxes, mint/burn, blacklist. |
| OC-009 | Bridge route risk map | P4 | planned | Trust model, cost, delay, liquidity, exploit history. |
| OC-010 | Chain/RPC health monitor | P4 | planned | Latency, missed blocks, congestion, priority fee. |

### 6. Portfolio, treasury, and risk

| ID | Feature | Priority | Status | Notes |
|---|---:|---:|---|---|
| PR-001 | Multi-wallet portfolio view | P3 | planned | CEX, DEX, chain, strategy, and stablecoin sleeves. |
| PR-002 | Strategy capital budgets | P1 | planned | Hard caps by strategy and venue. |
| PR-003 | Dynamic cash buffer | P2 | planned | Volatility-targeted reserve. |
| PR-004 | Volatility targeting | P2 | planned | Scale exposure to realized/projected volatility. |
| PR-005 | Correlation/regime model | P3 | planned | Avoid false diversification in crypto stress. |
| PR-006 | Drawdown budget engine | P1 | partial | Risk crate has kill-switch state; expand into budgets. |
| PR-007 | Concentration caps | P1 | planned | Asset, sector, chain, venue, route. |
| PR-008 | Inventory aging and decay | P1 | planned | Age-based risk for failed maker fills and stale positions. |
| PR-009 | Treasury operations view | P3 | planned | Stablecoin inventory, collateral, gas, chain working capital. |
| PR-010 | PnL attribution | P2 | planned | Strategy, venue, route, signal, fees, slippage, inventory. |

### 7. AI agent and governance

| ID | Feature | Priority | Status | Notes |
|---|---:|---:|---|---|
| AI-001 | Autonomy tiers | P0 | planned | Observe, recommend, approve-required, auto-execute bounded, emergency halt. |
| AI-002 | Agent decision log | P0 | planned | Prompt/model/version/input/evidence/output/action link. |
| AI-003 | Regime classifier | P3 | planned | Volatility, trend, liquidity, listing, macro, chain congestion. |
| AI-004 | Strategy ranker | P3 | planned | Rank opportunities by expected value and risk capacity. |
| AI-005 | Parameter recommender | P3 | planned | Propose bounded changes, never silently widen risk. |
| AI-006 | Anomaly detector | P2 | planned | Feed gaps, order rejects, abnormal slippage, wallet drift. |
| AI-007 | Natural-language trade review | P3 | planned | Explain why a trade was allowed, rejected, or escalated. |
| AI-008 | Agent memory and evidence store | P3 | planned | Durable, queryable, policy-filtered. |
| AI-009 | Human approval queue | P1 | planned | Approve parameter changes, taker mode, new routes, live enablement. |
| AI-010 | Self-improvement loop | P4 | planned | Offline evaluation, no unreviewed live model changes. |

### 8. Frontend and UX

| ID | Feature | Priority | Status | Notes |
|---|---:|---:|---|---|
| UX-001 | Trading command center | P1 | partial | Existing dashboard frontend is foundation. |
| UX-002 | Listing Radar workspace | P2 | planned | Roadmap, announcement, token, chain, venues, unlocks. |
| UX-003 | Execution Workspace | P1 | planned | Orderbook, maker/taker toggle, intent, route preview, policy reason. |
| UX-004 | Strategy Lab workspace | P2 | partial | Python strategy lab should be unified with frontend UX. |
| UX-005 | Risk Cockpit | P1 | planned | Budgets, drawdown, concentration, bridge exposure, kill switches. |
| UX-006 | AI Agent Console | P3 | planned | Agent tasks, recommendations, approvals, model lineage. |
| UX-007 | Incident/Ops Center | P2 | planned | Websocket/RPC/venue/webhook/job health. |
| UX-008 | Portfolio and Treasury | P3 | planned | Wallet, venue, strategy, chain, stablecoin, gas. |
| UX-009 | Onchain Intelligence workspace | P3 | planned | Wallet labels, flows, TVL, unlocks, route health. |
| UX-010 | Backtest/replay viewer | P1 | planned | Replay timeline, signal overlays, fills, policy decisions. |
| UX-011 | Mobile alert console | P4 | planned | Read-only initially; approval flows later. |
| UX-012 | Prestigious visual system | P2 | planned | High-density institutional terminal with calm, legible risk states. |

### 9. Cloud, CI, and agent development workflow

| ID | Feature | Priority | Status | Notes |
|---|---:|---:|---|---|
| DEV-001 | Codespaces parity | P1 | planned | Rust, Node, Python, frontend, data tools. |
| DEV-002 | Cloud-agent TDD cycle | P1 | planned | Quick/frontend/backend/full validation modes. |
| DEV-003 | Frontend package detection | P1 | planned | Current frontend is `crates/pt-dashboard/frontend`; future app workspaces auto-detected. |
| DEV-004 | GitHub Actions matrix | P1 | planned | Rust, Python, frontend, security, SBOM, docs. |
| DEV-005 | Fixture generator | P1 | planned | Deterministic samples for replay, listings, orderbook, onchain events. |
| DEV-006 | Contract tests | P1 | planned | OpenAPI, JSON schemas, frontend API clients. |
| DEV-007 | Paper soak workflow | P1 | partial | Existing script should become CI/manual workflow. |
| DEV-008 | Release bundle | P2 | planned | Binary, frontend assets, schemas, docs, SBOM, runbook. |
| DEV-009 | Observability baseline | P2 | planned | Metrics, tracing, structured logs, alerts. |
| DEV-010 | Docs-as-code gates | P2 | planned | Feature tracker and work orders must be updated with major PRs. |

## MVP sequence

1. Stabilize Codespaces + CI around Rust, Python, and the new dashboard frontend.
2. Create frontend command-center shell using the existing Vite package.
3. Add listing-radar data models and sample fixtures.
4. Add maker/taker execution-policy API and dashboard surface.
5. Add Coinbase listing lifecycle ingest and replay fixtures.
6. Add strategy-lab to dashboard integration.
7. Add agent autonomy tiers and approval queue.
8. Add onchain provider adapter interfaces.
9. Add DEX route quote adapters in simulation mode only.
10. Add tiny-live Coinbase pilot after all paper gates pass.
