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
- Dashboard now exposes a read-only approval queue view for draft and cancel-requested workstation orders.
- Approval queue storage now has snapshot reconciliation, a tested runtime hydration helper layer, and a runtime-store bridge in `pt-cli`.

## In Progress

- Validate promoted replay artifacts against `pt-cli` replay mode in a Rust-enabled environment.
- Wire hosted branch protection/manual approval settings in GitHub.
- Finish issue `#9` by calling the approval queue runtime-store bridge from Coinbase workstation startup and runtime lifecycle hooks.

## Next Queue

1. Run the full Rust validation ladder in an environment with `cargo` and network access.
2. Wire `CoinbaseWorkstationRuntime` to call the new runtime-store bridge against `storage.sqlite_path` during startup hydration and continuous reconciliation.
3. Add external incident destinations (PagerDuty/Slack/Sentry/OTel) after deployment target is selected.
4. Add mutation tests for risk and quote-critical logic.
5. Configure hosted branch protections and required status checks.
