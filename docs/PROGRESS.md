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
- Listing overlap now supports auto-discovery of likely recent Coinbase listings.
- Listing overlap now ranks post-anchor cohorts by impulse and volatility buckets.
- Strategy variants now support a plugin interface:
  - `external_bias_file` (Pine/AI bias series input)
  - `tradingview_webhook_file` (direct TradingView webhook snapshot replay input)
  - `momentum_bias`
  - `rsi_bias`
- Persistent SQLite trade journal now records runs/trades and exposes per-market attribution summaries.
- Strategy-lab promotion tooling now converts selected market/variant into replay NDJSON for Rust replay mode.
- Replay acceptance tooling now validates promoted replay NDJSON plus optional SQLite risk/execution evidence.

## In Progress

- PR `#12`: harden the Phase 1 evidence bundle into a deterministic 3-run repeatability gate.
- PR `#13`: add a read-only `/api/v1/approval-queue` operator API as the backend base for issue `#9`.
- PR `#18`: add the read-only approval-queue frontend panel on top of PR `#13`.
- PR `#11`: keep fixture-backed dashboard frontend tests available as the current-API safety net.

## Current audit finding

- The repository is still in Phase 1: sandbox trading / paper ROI.
- The next concrete runtime blocker remains issue `#9`: queue-relevant workstation orders are memory-only in the Coinbase workstation and do not survive restart.
- Coordination churn is now overlapping the active implementation queue:
  - main-based tracker PRs `#14` through `#17` and `#19`
  - stacked planning PR `#20` on top of `#13`
- Older PRs `#4` and `#8` remain stale or superseded and should not be treated as current delivery tracks.

## Next Queue

1. Implement issue `#9` as a small backend PR stacked on PR `#13`:
   - persist only `draft` and `cancel_requested` workstation orders via `storage.sqlite_path`
   - hydrate those queue-relevant rows on startup
   - prune persisted rows once orders move out of queue-relevant statuses
   - keep approval surfaces read-only and non-autonomous
   - add focused create/update/reload tests
2. Run the local-first validation ladder on the active approval-queue stack.
3. Rebase PR `#18` after the backend queue stack settles.
4. Reduce review noise by keeping one canonical coordination track and closing or superseding overlapping planning PRs.

## Validation ladder

1. `cargo fmt --all`
2. `cargo check --workspace`
3. `cargo clippy --workspace --all-targets --all-features -- -D warnings`
4. `cargo test --workspace`
5. `cargo build --workspace`
6. `cargo audit`
7. `./scripts/generate_sbom.sh artifacts`
8. `python3 tools/coinbase_strategy_lab.py backtest --config config/coinbase_strategy_lab.json`
9. `python3 tools/coinbase_strategy_lab.py overlap --config config/coinbase_strategy_lab.json --auto-discovery`
10. `python3 tools/coinbase_strategy_lab.py optimize --config config/coinbase_strategy_lab.json`
11. `cargo run -p pt-cli -- run --config config/config.toml`
12. `./scripts/paper_soak.sh 86400 30 config/config.toml`
