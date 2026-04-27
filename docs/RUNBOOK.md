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

Replay acceptance check:
```bash
./scripts/replay_acceptance.sh data/replay/strategy_lab_promoted.ndjson data/tuning/strategy_lab_promoted.json data/output/pt.sqlite
```

Phase 1 evidence bundle:
```bash
./scripts/phase1_evidence_bundle.sh \
  --bundle-dir data/evidence/phase1/20260426 \
  --run-label run-001 \
  --replay data/replay/strategy_lab_promoted.ndjson \
  --promotion data/tuning/strategy_lab_promoted.json \
  --sqlite data/output/pt.sqlite \
  --paper-soak data/soak/paper-soak-20260426-010203.json \
  --metrics data/evidence/run-001-metrics.json
```

Phase 1 gate report:
```bash
python3 tools/phase1_gate_report.py \
  --bundle-dir data/evidence/phase1/20260426 \
  --min-runs 3 \
  --out-json data/evidence/phase1/20260426/report.json \
  --out-md data/evidence/phase1/20260426/report.md
```

Phase 1 evidence self-check:
```bash
python3 -m unittest discover -s tests -p 'test_phase1_gate_report.py'
```

Interpretation:
- `pass` means at least three independent runs were present, aggregate net PnL after costs stayed positive, and no hard risk breach was detected.
- `fail` means a hard gate was violated.
- `incomplete` means evidence is missing, malformed, or not yet repeatable across three runs.

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

## Operator Review Queue
- `GET /api/v1/approval-queue`
- This endpoint is informational and read-only.
- It surfaces draft and cancel-requested workstation orders that still need human review.
- It does not approve, place, or authorize live orders.
- Durable queue persistence remains a separate follow-up; current queue visibility is derived from runtime state.

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
4. Inspect operator review state (`/api/v1/approval-queue`) for queued or cancel-requested orders.
5. If exchange/hedge degraded, keep `halt` or `flatten` active.
6. Preserve context snapshot:
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
