# Progress

## Completed (Latest)

- Rust workspace baseline for Polymarket + Coinbase hedge engine is in place.
- Local-first safeguards and runbooks are in place (no CI/CD required before local validation).
- Coinbase strategy lab now supports:
  - backtest
  - overlap (candle-aligned listing analysis)
  - optimize
  - unified dashboard
- Listing overlap now supports **auto-discovery** of likely recent Coinbase listings.
- Strategy variants now support a **plugin interface**:
  - `external_bias_file` (Pine/AI bias series input)
  - `momentum_bias`
  - `rsi_bias`
- Persistent SQLite trade journal now records runs/trades and exposes per-market attribution summaries.
- Strategy-lab promotion tooling now converts selected market/variant into replay NDJSON for Rust replay mode.

## In Progress

- Validate promotion flow end-to-end against `pt-cli` replay mode with a selected promoted artifact.
- Expand docs/examples for external bias file formats and tuning loop ergonomics.

## Next Queue

1. Add optional ranked listing cohorts by post-anchor impulse and volatility buckets.
2. Add plugin presets for direct TradingView webhook replay snapshots.
3. Add a replay acceptance script that checks risk/latency counters after promoted runs.
4. Add comparative report export (CSV/Markdown) across variants and markets.
