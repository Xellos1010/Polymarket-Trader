# Product Planning Bootstrap

Engineering doc portal: [../README.md](../README.md).

This folder holds the smallest planning surface needed to move the repository through Phase 1 sandbox trading safely.

## Purpose

Use these files to queue small, reviewable tasks that improve replay validation, paper-mode repeatability, risk evidence, and operator visibility without changing runtime behavior.

## Files

- `IMPLEMENTATION_WORK_ORDERS.md` - PR-sized work orders for the next execution rounds.
- `FEATURE_TRACKER.json` - machine-readable tracker for the current Phase 1 queue.

## Operating boundaries

1. Replay and paper evidence stay ahead of product expansion.
2. These docs do not enable live trading, deployment, or risk-cap changes.
3. New planning artifacts should stay focused on Phase 0 and Phase 1 until replay and paper gates pass repeatedly.

## Current focus

- Run the local validation ladder in a Rust-enabled environment.
- Verify promoted strategy-lab outputs in replay mode (`cargo run -p pt-cli -- coinbase up --config config/config.toml --mode replay` or `engine.mode = "replay"` with `pt-cli run`).
- Complete repeatable paper-soak evidence with risk counters and attribution.
- Add fixture-backed dashboard coverage only where it improves operator visibility for those gates.
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
