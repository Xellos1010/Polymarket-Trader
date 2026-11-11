---
name: nx-verification
description: >
  Runs Nx-orchestrated verification for Polymarket-Trader (Rust workspace + optional
  dashboard frontend). Use for builder/verifier gates and CI parity checks.
---

# Nx verification (Polymarket-Trader)

This workspace uses **Nx** with `nx:run-commands` targets; there is no `@nx/js` typecheck target on the root Rust project.

## Projects

| Project | Root | Role |
|---------|------|------|
| `polymarket-trader` | `.` | Cargo workspace, scripts, SBOM |
| `pt-dashboard-frontend` | `crates/pt-dashboard/frontend` | Vite + Vitest |

## Standard Rust pipeline (cached where configured)

```bash
pnpm exec nx run-many -t fmt check clippy test build --projects=polymarket-trader
```

Or individually:

```bash
pnpm exec nx run polymarket-trader:fmt
pnpm exec nx run polymarket-trader:check
pnpm exec nx run polymarket-trader:clippy
pnpm exec nx run polymarket-trader:test
pnpm exec nx run polymarket-trader:build
pnpm exec nx run polymarket-trader:audit
pnpm exec nx run polymarket-trader:sbom
```

## Full local ladder (matches `scripts/local_validation_ladder.sh`)

```bash
pnpm exec nx run polymarket-trader:local-validation
```

With dashboard frontend checks:

```bash
pnpm exec nx run polymarket-trader:local-validation-frontend
```

## Dashboard frontend

From repo root (requires `npm install` inside `crates/pt-dashboard/frontend` first, or use the ladder with `RUN_FRONTEND=1`):

```bash
pnpm exec nx run pt-dashboard-frontend:typecheck
pnpm exec nx run pt-dashboard-frontend:test
pnpm exec nx run pt-dashboard-frontend:build
```

## Affected (when using Nx graph)

```bash
pnpm exec nx affected -t fmt check clippy test build --base=main --head=HEAD
```

If affected is not configured for your branch, prefer explicit `nx run` / `run-many` above.

## Result format

Produce a short report:

- **Overall**: PASS / FAIL
- **Commands run**: list
- **Failures**: first failing command output summary

## Error handling

- If `node_modules` is missing, run `pnpm install` at the repo root first.
- If a target is intentionally skipped (no frontend install), mark SKIPPED, not FAIL.
- Do not retry flaky network fetches automatically; report and stop.
