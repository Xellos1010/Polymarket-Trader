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
- Listing Radar is now backed by dashboard API endpoints with typed shared models and OpenAPI coverage.
- Risk Cockpit and Agent Console now have backend summary endpoints instead of frontend-only placeholder content.
- Sandbox optimization cycle tooling now exists in `tools/sandbox_optimizer_cycle.py` to run optimize -> backtest -> promote -> replay acceptance -> incumbent update.
- Sandbox ROI roadmap now exists in `docs/product/SANDBOX_ROI_ROADMAP.md`.

## In Progress

- Validate promoted replay artifacts against `pt-cli` replay mode in a Rust-enabled environment.
- Wire hosted branch protection/manual approval settings in GitHub.
- Run end-stage Rust/frontend validation for the expanded dashboard surface.
- Turn the sandbox optimization cycle into a scheduled hourly workflow.

## Next Queue

1. Run the full Rust validation ladder in an environment with `cargo` and network access.
2. Put the sandbox optimization cycle on an hourly schedule in sandbox mode only.
3. Add operator-visible incumbent/candidate/promotion evidence to the dashboard.
4. Add fixture-driven frontend tests for workspace navigation and new listing/risk/agent API rendering.
5. Add execution-policy and approval-queue persistence so the new workspaces move from derived summaries to durable audit history.
6. Add external incident destinations (PagerDuty/Slack/Sentry/OTel) after deployment target is selected.
7. Add mutation tests for risk and quote-critical logic.
8. Configure hosted branch protections and required status checks.
