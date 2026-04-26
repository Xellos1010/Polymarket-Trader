# Product Expansion Workspace

This folder is the source of truth for expanding Polymarket Trader into a Coinbase-first, AI-enabled crypto trading operating system.

## Files

- `MASTER_FEATURE_TRACKER.md` — exhaustive feature backlog, capability map, priority ladder, and release sequencing.
- `SERVICE_PROVIDER_MATRIX.md` — benchmark and integration map for the best-in-market providers to emulate or integrate.
- `FRONTEND_UX_MASTER_PLAN.md` — dashboard/frontend expansion plan grounded in the current Vite/React dashboard.
- `IMPLEMENTATION_WORK_ORDERS.md` — TDD-ready work orders for cloud agents and Codespaces execution.
- `FEATURE_TRACKER.json` — machine-readable tracker for agents and CI workflows.

## Product thesis

The platform should become a deterministic, replayable execution and risk engine with an AI supervision layer. The deterministic layer owns data ingestion, routing, wallet signing, policy enforcement, order management, portfolio limits, replay, and audit logs. The AI layer classifies regimes, ranks opportunities, proposes parameter changes, explains decisions, and escalates anomalies inside strict policy boundaries.

## Current frontend baseline

The latest `main` branch includes a dashboard frontend at `crates/pt-dashboard/frontend` using Vite, React 18, TypeScript, and Vitest. All future UX expansion should build from that package, keeping it modular enough to graduate into a larger app workspace if/when Nx or a separate `apps/trading-terminal` frontend becomes necessary.

## Operating principle

No live trading expansion is complete until it has:

1. A deterministic replay path.
2. Unit and contract tests.
3. Paper-mode validation.
4. Risk-policy coverage.
5. Dashboard visibility.
6. Audit log evidence.
7. Operator kill-switch support.
