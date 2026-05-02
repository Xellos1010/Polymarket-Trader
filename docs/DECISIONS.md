# Consolidated engineering decisions

This page summarizes decisions that shape how the repository is built and operated. Authoritative detail remains in ADRs, schemas, and OpenAPI where noted.

## Runtime and platform

| Decision | Rationale | Where recorded |
|----------|-----------|----------------|
| **Rust-first single-process engine** on one host | Predictable latency, strong typing, modular crates | [ADR-001](adr/001-rust-first-polymarket-engine.md), [architecture/system-overview.md](architecture/system-overview.md) |
| **Tokio async** orchestration | Single binary, concurrent market IO, dashboard, optional listeners | ADR-001, `crates/pt-engine` |
| **Paper / replay / live** driven by config | Safe progression before capital at risk | `schemas/config.schema.json`, `config/config.example.toml` |

## Operator and API surface

| Decision | Rationale | Where recorded |
|----------|-----------|----------------|
| **`pt-dashboard` HTTP contract is canonical** | Avoid drift between prose and code | [api/dashboard-openapi.yaml](api/dashboard-openapi.yaml) |
| **Emergency controls** via `POST /ops/halt`, `/ops/resume`, `/ops/flatten` | Fast operator intervention | OpenAPI, `crates/pt-dashboard` |

## Strategy lab and research loop

| Decision | Rationale | Where recorded |
|----------|-----------|----------------|
| **Batch strategy lab** (`backtest`, `overlap`, `optimize`, `dashboard`) via **`tools/coinbase_strategy_lab.py`** | Matches CI/local ladder, stable CLI surface | [LOCAL_VALIDATION.md](LOCAL_VALIDATION.md), [STRATEGY_LAB.md](STRATEGY_LAB.md) |
| **`pt-strategy-lab` as a library** with `/lab/*` Axum routes | Shared engine for tests and embedding; not a separate `pt-cli` server today | [STRATEGY_LAB.md](STRATEGY_LAB.md), `crates/pt-strategy-lab` |
| **Replay acceptance** via `scripts/replay_acceptance.sh` / `tools/replay_acceptance.py` | Scripted gate for promotion + replay NDJSON (+ optional SQLite) | [PINE_TUNING.md](PINE_TUNING.md), scripts |

## Quality and release discipline

| Decision | Rationale | Where recorded |
|----------|-----------|----------------|
| **Local-first validation ladder** before merge/deploy claims | CI is a subset; replay/paper/soak are local | [LOCAL_VALIDATION.md](LOCAL_VALIDATION.md), [CONTRIBUTING.md](../CONTRIBUTING.md) |
| **Secrets out of git**; config from `*.example` templates | Reduce credential leakage | [CONTRIBUTING.md](../CONTRIBUTING.md), root `README.md`, `.gitignore` |

## Explicit non-goals (today)

- **No guarantee** that date-stamped phase docs (`PHASE*`, `ISSUE_*`, etc.) reflect current CLI or routes; they are retained as history.
- **Not all** workstation UX described in long-form product docs is exposed as public HTTP; verify against OpenAPI before integration.
