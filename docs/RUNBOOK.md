# Operational Runbook

## Startup
```bash
cargo run -p pt-cli -- run --config config/config.toml
```

Strategy lab dashboard (Coinbase local workflow):
```bash
python3 tools/coinbase_strategy_lab.py dashboard --config config/coinbase_strategy_lab.json --serve 9090
```

## Live Readiness Gate
```bash
cargo run -p pt-cli -- preflight-live --config config/config.toml --timeout-ms 3000
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
- Enter safe mode/flatten behavior: `POST /ops/flatten`

## Incident Triage
1. Check kill-switch and risk state (`/state/risk`).
2. Inspect recent executions (`/state/executions`).
3. Verify market feed freshness (`/state/books`, `/state/history`).
4. If exchange/hedge degraded, keep `halt` or `flatten` active.
5. Preserve context snapshot:
   ```bash
   ./scripts/save_context.sh "incident note" docs/SESSION_CONTEXT.md config/config.toml
   ```

## Rollback
- Runtime rollback: set manual halt, then restart previous binary/config.
- Code rollback (git): `git revert <commit>` and redeploy.

## Post-Incident
- Append notes to `docs/SESSION_CONTEXT.md`.
- Update `docs/SDLC_CHECKLIST.md` if process/tooling gaps were found.

## External AI Handoff
```bash
./scripts/export_prompt_bundle.sh
```
