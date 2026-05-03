# Documentation audit — public release readiness

**Audit date:** 2026-05-02  
**Scope:** `docs/` tree, root operator docs (`README.md`, `DEPLOYMENT.md`, `CONTRIBUTING.md`), and alignment with `docs/api/dashboard-openapi.yaml`.

## Executive summary

- **Strengths:** Clear local validation story ([LOCAL_VALIDATION.md](LOCAL_VALIDATION.md)), OpenAPI as HTTP source of truth, CONTRIBUTING security baseline, deployment doc for single-host EC2.
- **Gaps addressed in this pass:** Documentation portal ([README.md](README.md)), consolidated decisions ([DECISIONS.md](DECISIONS.md)), ADR index ([adr/README.md](adr/README.md)), aggregated runbook ([runbooks/AGGREGATED_OPERATIONS.md](runbooks/AGGREGATED_OPERATIONS.md)), root **MIT [LICENSE](../LICENSE)** matching `Cargo.toml`.
- **Residual risk:** Many **date-stamped phase / issue / board** markdown files remain useful internally but can confuse external readers; they are classified below as **historical**. `SESSION_CONTEXT.md` may contain operator notes — treat as **non-authoritative** and scrub before release if needed.

## Classification of `docs/` materials

### Tier A — Safe defaults for external readers

| Path | Role |
|------|------|
| [README.md](README.md) (this folder) | Doc portal |
| [LOCAL_VALIDATION.md](LOCAL_VALIDATION.md) | Canonical local gate |
| [architecture/system-overview.md](architecture/system-overview.md) | Architecture |
| [api/dashboard-openapi.yaml](api/dashboard-openapi.yaml) | HTTP contract |
| [data/SCHEMA.md](data/SCHEMA.md) | Data model |
| [STRATEGY_LAB.md](STRATEGY_LAB.md), [PINE_TUNING.md](PINE_TUNING.md) | Research workflows |
| [PROMPT_BUNDLE.md](PROMPT_BUNDLE.md) | AI handoff |
| [CONTEXT_PERSISTENCE.md](CONTEXT_PERSISTENCE.md) | Resume workflow |
| [INSTRUCTIONS.md](INSTRUCTIONS.md) | Operator copy-paste (verify against OpenAPI when in doubt) |
| [RUNBOOK.md](RUNBOOK.md) | Ops (cross-check OpenAPI) |
| [TINY_LIVE_PILOT.md](TINY_LIVE_PILOT.md) | Constrained live pilot |
| [SDLC_CHECKLIST.md](SDLC_CHECKLIST.md) | Process |
| [DECISIONS.md](DECISIONS.md), [adr/README.md](adr/README.md), [adr/001-rust-first-polymarket-engine.md](adr/001-rust-first-polymarket-engine.md) | Decisions |
| [runbooks/](runbooks/) | Aggregated operations |

### Tier B — Product / planning (accurate at time of writing; may lag code)

| Path | Role |
|------|------|
| [product/](product/) | Roadmaps, trackers, UX plans |
| [PROGRESS.md](PROGRESS.md) | Progress narrative |

### Tier C — Historical / program artifacts (internal or time-bound)

Moved to **`docs/archive/program-history-2026/`** (see [archive README](archive/program-history-2026/README.md)) to reduce noise for newcomers:

| Pattern | Examples (now archived) |
|---------|-------------------------|
| Phase notes | `PHASE*.md` |
| Issue / branch wiring | `ISSUE_9*.md` |
| Board / audit snapshots | `INTEGRATION_BOARD.md`, `OPEN_PR_*`, `COMPLETION_AUDIT.md`, `PHASE1_EVIDENCE.md`, `APPROVAL_QUEUE_PERSISTENCE_PLAN.md`, `COINBASE_STRATEGY_AI_WORKORDER_PACKET.md` |

Still in `docs/` (review before public tag):

| Item | Note |
|------|------|
| `SESSION_CONTEXT.md` | May contain dated operator narrative |
| `WORK_STATUS.md` / `WORK_STATUS.json` | Maintainer queue; OK if no secrets — **review** |

### Tier D — API bundle

| Path | Role |
|------|------|
| [api/dashboard-openapi.yaml](api/dashboard-openapi.yaml) | Versioned contract — bump `info.version` when breaking |

## Drift checks performed

- **RUNBOOK** previously referenced `GET /api/v1/approval-queue`; that path is **not** in current OpenAPI — corrected in [RUNBOOK.md](RUNBOOK.md) to use documented surfaces (`/api/v1/orders`, OpenAPI).
- Operator curls must always be reconciled with **OpenAPI** after router changes.

## Secrets scan (docs only)

- Grep for obvious credential patterns in `docs/**/*.md`: no PEM blocks found; only **field name** references (e.g. `api_key`) in configuration discussion — acceptable.
- **Re-run before tag:** `git grep -iE 'BEGIN (RSA|EC|OPENSSH) PRIVATE'` and review `SESSION_CONTEXT.md`, `WORK_STATUS.md`, `product/*.json`.

## Suggested follow-ups before tagging `v*` public

1. ~~**Security policy:**~~ **`SECURITY.md`** added at repo root.
2. ~~**Code of conduct:**~~ **`CODE_OF_CONDUCT.md`** added at repo root.
3. ~~**Archive:**~~ Tier C docs moved to **`docs/archive/program-history-2026/`** with index README.
4. ~~**Roadmap:**~~ Root **`ROADMAP.md`** consolidates priorities with links to product trackers.
5. **CI:** Confirm public workflows do not print secrets; redact logs.

## Sign-off checklist (manual)

- [ ] `SESSION_CONTEXT.md` reviewed or truncated for public tag
- [ ] `WORK_STATUS*.md` acceptable for public audience
- [ ] `./scripts/local_validation_ladder.sh` green on clean checkout
- [ ] OpenAPI version reflects any HTTP changes since last release
- [x] Root README links to `docs/README.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`, and `ROADMAP.md`
