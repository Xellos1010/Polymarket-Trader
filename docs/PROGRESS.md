# Progress

## Completed (Latest)

- Rust workspace baseline for Polymarket + Coinbase hedge engine is in place.
- Local-first safeguards and runbooks are in place (no CI/CD required before local validation).
- Coinbase strategy lab now supports:
  - backtest
  - overlap (candle-aligned listing analysis)
  - optimize
  - unified dashboard
  - comparative CSV/Markdown report export
- Listing overlap now supports **auto-discovery** of likely recent Coinbase listings.
- Listing overlap now ranks **post-anchor cohorts** by impulse and volatility buckets.
- Strategy variants now support a **plugin interface**:
  - `external_bias_file` (Pine/AI bias series input)
  - `tradingview_webhook_file` (direct TradingView webhook snapshot replay input)
  - `momentum_bias`
  - `rsi_bias`
- Persistent SQLite trade journal now records runs/trades and exposes per-market attribution summaries.
- Strategy-lab promotion tooling now converts selected market/variant into replay NDJSON for Rust replay mode.
- Replay acceptance tooling now validates promoted replay NDJSON plus optional SQLite risk/execution evidence.
- Phase 1 evidence bundle scaffolding now exists for dated run bundles plus a strict gate report (`scripts/phase1_evidence_bundle.sh`, `tools/phase1_gate_report.py`, `docs/PHASE1_EVIDENCE.md`).
- The Phase 1 gate report now enforces a deterministic three-run repeatability standard with aggregate net-after-costs checks, manifest schema validation, and fixture-backed Python tests.

## In Progress

- Validate promoted replay artifacts against `pt-cli` replay mode in a Rust-enabled environment.
- Capture at least three independent replay/paper evidence bundles and generate a Phase 1 gate report.
- Wire hosted branch protection/manual approval settings in GitHub.

## Next Queue

1. Run the full Rust validation ladder in an environment with `cargo` and network access.
2. Populate `data/evidence/phase1/<bundle>/` with three independent runs and generate `report.json` plus `report.md`.
3. Persist dashboard approval queue state across restart/reload using `storage.sqlite_path`, keeping execution authority unchanged.
4. Add external incident destinations (PagerDuty/Slack/Sentry/OTel) after deployment target is selected.
5. Add mutation tests for risk and quote-critical logic.
6. Configure hosted branch protections and required status checks.
