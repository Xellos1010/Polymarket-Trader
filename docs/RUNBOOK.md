# Operational Runbook

## Pre-merge validation

Before merge, deployment, or any operator-readiness claim, run the canonical local-first ladder:

```bash
./scripts/local_validation_ladder.sh
```

Reference guide:
- `docs/LOCAL_VALIDATION.md`

## Startup
```bash
cargo run -p pt-cli -- coinbase up --config config/config.toml --mode paper
```

Strategy lab dashboard (Coinbase local workflow):
```bash
python3 tools/coinbase_strategy_lab.py dashboard --config config/coinbase_strategy_lab.json --serve 9090
```

Promote strategy-lab result to replay artifact:
```bash
./scripts/promote_strategy_lab.sh data/strategy_lab/<report>.json BTC-USD sma_baseline
```

## Live Readiness Gate
```bash
cargo run -p pt-cli -- coinbase preflight --config config/config.toml --mode live --timeout-ms 3000
```

Tiny live pilot guard:
```bash
./scripts/tiny_live_pilot.sh config/config.toml 3000
```

## Health Checks
- `GET /health`
- `GET /healthz`
- `GET /ready`
- `GET /metrics`

## Emergency Controls
- Halt quoting: `POST /ops/halt`
- Resume: `POST /ops/resume`
- Enter safe mode or flatten behavior: `POST /ops/flatten`

## Incident Triage
1. Check kill-switch and risk state (`/state/risk`).
2. Inspect recent executions (`/state/executions`).
3. Verify market feed freshness (`/state/books`, `/state/history`).
4. If exchange or hedge is degraded, keep `halt` or `flatten` active.
5. Preserve context snapshot:
   ```bash
   ./scripts/save_context.sh "incident note" docs/SESSION_CONTEXT.md config/config.toml
   ```

## Rollback
- Runtime rollback: set manual halt, then restart previous binary or config.
- Code rollback (git): `git revert <commit>` and redeploy.

## Post-Incident
- Append notes to `docs/SESSION_CONTEXT.md`.
- Update `docs/SDLC_CHECKLIST.md` if process or tooling gaps were found.

## External AI Handoff
```bash
./scripts/export_prompt_bundle.sh
```
