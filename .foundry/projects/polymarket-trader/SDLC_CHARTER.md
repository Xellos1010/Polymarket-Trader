# Polymarket-Trader — SDLC charter (lightweight)

This repo is **Rust-first** with optional Nx orchestration for verification and continuity.

## Phases (aligned with `docs/` and operator instructions)

1. **Phase 0 — Repo readiness**: fmt, check, clippy, test, build, audit, SBOM, config validation.
2. **Phase 1 — Sandbox / paper ROI**: replay, paper soak, risk gates; no autonomous live trading.
3. **Phase 2 — Tiny live pilot**: only after explicit human approval and gate checklist in `docs/`.

## Quality gates

Prefer `./scripts/local_validation_ladder.sh` or Nx: `pnpm exec nx run polymarket-trader:local-validation`.

## Continuity

Machine-readable state: `current-task.json` in this directory. Update with the continuity-writer skill after phase or branch changes.
