# Roadmap

This roadmap is **outcome-oriented** and links to deeper trackers. It is updated as priorities shift; detailed backlog rows live in product docs.

## North star

Build a **deterministic** trading and risk stack (ingestion, replay, paper, live guardrails, audit) with a **Coinbase-first** workstation and optional Polymarket and other venues—see [docs/product/MASTER_FEATURE_TRACKER.md](docs/product/MASTER_FEATURE_TRACKER.md) for the full capability ladder (P0–P5).

## Near term — reliability and public OSS hygiene

| Theme | Outcomes | Tracking |
|-------|----------|----------|
| **Validation & contracts** | Local ladder stays green; dashboard HTTP matches OpenAPI; contract tests cover shipped routes | [docs/LOCAL_VALIDATION.md](docs/LOCAL_VALIDATION.md), [docs/api/dashboard-openapi.yaml](docs/api/dashboard-openapi.yaml), `crates/pt-dashboard/tests/api_contract.rs` |
| **Operator clarity** | Single aggregated runbook; no stale “phantom” endpoints in primary docs | [docs/runbooks/AGGREGATED_OPERATIONS.md](docs/runbooks/AGGREGATED_OPERATIONS.md), [docs/RUNBOOK.md](docs/RUNBOOK.md) |
| **Persistence** | Restart-safe state where configured (engine SQLite path, workstation order lifecycle as designed) | [docs/data/SCHEMA.md](docs/data/SCHEMA.md), archived planning under [docs/archive/program-history-2026/](docs/archive/program-history-2026/) |
| **Security & community** | Clear reporting path and conduct expectations | [SECURITY.md](SECURITY.md), [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) |

## Mid term — sandbox ROI and evidence loop

Goal: repeatable **sandbox** research and promotion with operator-visible evidence—not widened live risk without human gates.

| Theme | Outcomes | Tracking |
|-------|----------|----------|
| **Strategy lab loop** | `backtest` → `overlap` → `optimize` → promotion → replay acceptance reproducible on a clean checkout | [docs/product/SANDBOX_ROI_ROADMAP.md](docs/product/SANDBOX_ROI_ROADMAP.md), [docs/STRATEGY_LAB.md](docs/STRATEGY_LAB.md) |
| **Market data & replay** | Richer replayable market-data path; clearer data-quality signals | [docs/product/MASTER_FEATURE_TRACKER.md](docs/product/MASTER_FEATURE_TRACKER.md) (MD-* rows) |
| **Dashboard** | Evidence-first views (incumbent vs candidate, promotion reasons, policy timeline) without bypassing risk | [docs/product/FRONTEND_UX_MASTER_PLAN.md](docs/product/FRONTEND_UX_MASTER_PLAN.md) |

## Longer term — platform expansion

| Tier | Focus | Doc |
|------|--------|-----|
| P2+ | Listing radar, portfolio, bounded agent autonomy | [docs/product/MASTER_FEATURE_TRACKER.md](docs/product/MASTER_FEATURE_TRACKER.md) |
| P4+ | Multi-chain routes and execution readiness | Same |

## How we sequence work

1. **P0 gates** (safety, determinism, replay/paper evidence) before expanding live surface area.  
2. **Small PRs** with ladder + tests; see [CONTRIBUTING.md](CONTRIBUTING.md).  
3. **Product execution** uses [docs/product/IMPLEMENTATION_WORK_ORDERS.md](docs/product/IMPLEMENTATION_WORK_ORDERS.md) and [docs/product/FEATURE_TRACKER.json](docs/product/FEATURE_TRACKER.json) for PR-sized items.

## Changelog

- **2026-05-02** — Initial consolidated roadmap; historical phase docs moved to [docs/archive/program-history-2026/](docs/archive/program-history-2026/).
