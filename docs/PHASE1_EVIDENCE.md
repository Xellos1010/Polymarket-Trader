# Phase 1 Evidence Bundle

This workflow turns replay and paper artifacts into one operator-facing evidence bundle.

It is intentionally strict:
- a single profitable run is not enough
- Phase 1 requires at least three independent runs
- missing modeled-cost or risk-gate evidence is treated as incomplete
- no live credentials or live mode are required

## Goal

Create a repeatable artifact set for Phase 1 sandbox ROI review so operators can judge replay and paper readiness from evidence instead of anecdotes.

## Bundle layout

```text
data/evidence/phase1/<bundle>/
  run-001/
    manifest.json
    replay_acceptance.json
    paper_soak.json
    metrics.json
  run-002/
    ...
  run-003/
    ...
  report.json
  report.md
```

`metrics.json` is optional for the bundle command, but the gate report will mark the run incomplete until modeled-cost and risk-breach fields are present.

## Per-run bundle command

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

The bundle command always writes `manifest.json` and `replay_acceptance.json`. It copies `paper_soak.json` and `metrics.json` when those files are provided.

The generated `manifest.json` includes `schema_version: 1`. The gate report treats a missing or mismatched manifest schema as incomplete evidence.

## Gate report command

```bash
python3 tools/phase1_gate_report.py \
  --bundle-dir data/evidence/phase1/20260426 \
  --min-runs 3 \
  --out-json data/evidence/phase1/20260426/report.json \
  --out-md data/evidence/phase1/20260426/report.md
```

The report returns:
- `pass` only when at least three independent runs exist, all run labels are unique, every run clears replay and paper checks, aggregate net PnL after costs is positive, and no hard risk breach is present
- `fail` when any run shows a hard gate break such as negative net PnL after costs, a risk breach, replay failure, paper-soak failure, or unhedged-delta breach
- `incomplete` when evidence is missing, malformed, uses the wrong manifest schema, or the run count is below the required threshold

## Expected metrics.json fields

```json
{
  "net_pnl_after_costs": 0.42,
  "fees": 0.05,
  "slippage": 0.04,
  "hedge_cost": 0.03,
  "gas_amortized": 0.0,
  "adverse_selection": 0.02,
  "daily_loss_limit_breached": false,
  "max_market_notional_breached": false,
  "max_total_open_notional_breached": false,
  "max_unhedged_delta_breached": false,
  "stale_book_breached": false,
  "unexpected_auto_halt": false
}
```

These fields keep modeled-cost attribution and risk-gate evidence explicit. If the current runtime does not emit them yet, the report will stay `incomplete` until that gap is closed.

## Recommended operator flow

1. Promote a strategy-lab result into replay NDJSON.
2. Run replay validation and optional SQLite evidence capture.
3. Run a paper soak and keep its JSON report.
4. Bundle each independent run under one dated bundle directory using unique run labels.
5. Generate `report.json` and `report.md` from the full bundle directory.
6. Treat any `incomplete` report as a blocker, not a soft pass.
7. Update `docs/PROGRESS.md` and `docs/SESSION_CONTEXT.md` with the resulting status.

## Deterministic test command

```bash
python3 -m unittest discover -s tests -p 'test_phase1_gate_report.py'
```

## Safety

- Do not enable live mode from this workflow.
- Do not inject credentials into repo files.
- Do not treat `incomplete` as a pass.
- Keep risk caps at or below the tiny-pilot defaults unless explicitly approved.
