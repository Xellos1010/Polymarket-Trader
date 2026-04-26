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
- Product expansion workspace now exists under `docs/product/` with a master feature tracker, provider matrix, UX plan, work orders, and machine-readable tracker.
- Dashboard frontend shell now supports multi-workspace operator views for command, listing, risk, strategy, and agent supervision.

## In Progress

- Validate promoted replay artifacts against `pt-cli` replay mode in a Rust-enabled environment.
- Wire hosted branch protection/manual approval settings in GitHub.
- Convert the new dashboard shell into backend-backed workspaces with fixture-driven frontend tests.

## Next Queue

1. Run the full Rust validation ladder in an environment with `cargo` and network access.
2. Add listing-radar backend models and fixture data for the new frontend workspace.
3. Add execution-policy and approval-queue API surfaces to back the risk and agent workspaces.
4. Add external incident destinations (PagerDuty/Slack/Sentry/OTel) after deployment target is selected.
5. Add mutation tests for risk and quote-critical logic.
6. Configure hosted branch protections and required status checks.
