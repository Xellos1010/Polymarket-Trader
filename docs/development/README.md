# Documentation portal

Use this page to navigate the tree before opening deep links from the root `README.md`.

## Start here (public / new contributors)

| Doc | Purpose |
|-----|---------|
| [../README.md](../README.md) | Workspace overview, crate list, `pt-cli` surface, quick start |
| [../CONTRIBUTING.md](../CONTRIBUTING.md) | Branching, commits, quality gates, security rules |
| [LOCAL_VALIDATION.md](LOCAL_VALIDATION.md) | Full local validation ladder (canonical pre-merge gate) |
| [architecture/system-overview.md](architecture/system-overview.md) | Runtime topology and crate map |
| [api/dashboard-openapi.yaml](api/dashboard-openapi.yaml) | **Canonical** operator HTTP contract for `pt-dashboard` |
| [data/SCHEMA.md](data/SCHEMA.md) | Runtime persistence and data shapes |

## Runbooks (operational)

| Doc | Purpose |
|-----|---------|
| [runbooks/README.md](runbooks/README.md) | Runbook index |
| [runbooks/AGGREGATED_OPERATIONS.md](runbooks/AGGREGATED_OPERATIONS.md) | Single aggregated operations guide (deploy, health, incidents, pilot) |

Source material (still authoritative for detail): [../DEPLOYMENT.md](../DEPLOYMENT.md), [RUNBOOK.md](RUNBOOK.md), [TINY_LIVE_PILOT.md](TINY_LIVE_PILOT.md), [CONTEXT_PERSISTENCE.md](CONTEXT_PERSISTENCE.md), [INSTRUCTIONS.md](INSTRUCTIONS.md).

## Decisions and governance

| Doc | Purpose |
|-----|---------|
| [DECISIONS.md](DECISIONS.md) | Consolidated engineering decisions (with ADR pointers) |
| [adr/README.md](adr/README.md) | ADR index |
| [SDLC_CHECKLIST.md](SDLC_CHECKLIST.md) | SDLC / release checklist |

## Product and strategy (reference)

| Path | Purpose |
|------|---------|
| [STRATEGY_LAB.md](STRATEGY_LAB.md) | Strategy lab workflows (Python driver + Rust crate role) |
| [PINE_TUNING.md](PINE_TUNING.md) | Pine tuning loop |
| [PROMPT_BUNDLE.md](PROMPT_BUNDLE.md) | External AI / prompt bundle export |
| [product/](product/) | Roadmaps, trackers, UX plans (may lag code; not a substitute for OpenAPI) |

## Release preparation

| Doc | Purpose |
|-----|---------|
| [AUDIT_PUBLIC_RELEASE.md](AUDIT_PUBLIC_RELEASE.md) | Documentation audit, classification, and follow-ups for a public release |
| [../SECURITY.md](../SECURITY.md) | Vulnerability reporting |
| [../CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md) | Community standards |

## Roadmap

| Doc | Purpose |
|-----|---------|
| [../ROADMAP.md](../ROADMAP.md) | Consolidated near/mid/long-term roadmap (links product trackers) |

## Historical / program notes (optional reading)

Phase-0/1 execution notes, queue audits, and integration-board snapshots live under **[archive/program-history-2026/](archive/program-history-2026/)** (see its [README](archive/program-history-2026/README.md)). Treat them as **program history**, not guaranteed current procedure. Prefer **OpenAPI**, **LOCAL_VALIDATION.md**, and **runbooks/AGGREGATED_OPERATIONS.md** for what the repo does today.
