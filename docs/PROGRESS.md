# Progress

## Completed (Merged on `main`)

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
- Merged PRs to date include:
  - `#1` maker-focused market scanner CLI + EC2 workflow notes
  - `#2` clippy fix + audit policy cleanup
  - `#3` replay acceptance + readiness docs

## Current Audit (2026-04-26)

- Repo remains in **Phase 1: sandbox trading / paper ROI**.
- The active implementation queue is concentrated in draft PRs `#11`, `#12`, `#13`, and `#18`.
- Planning-only queue churn exists across draft PRs `#14` through `#17`.
- Older open PRs `#4` and `#8` should be treated as stale/superseded until explicitly rebased and revalidated.
- The next runtime blocker is issue `#9`: workstation orders that drive the approval queue are still memory-only in the Coinbase workstation runtime, so restart-safe operator review is not yet achieved.
- Visible PR checks are not yet green across the active stack; latest failures repeatedly show format-check and dependency-vulnerability-scan failures.

## In Progress

- Harden the Phase 1 repeatability/evidence workflow in PR `#12` and gather fresh local validation evidence.
- Keep approval-queue visibility read-only in PR `#13` and PR `#18`; do not add approval/execution mutation controls in that surface.
- Start the next runtime slice for issue `#9` with a deliberately narrow scope:
  - persist only operator-review queue states (`draft`, `cancel_requested`) via `storage.sqlite_path`
  - hydrate those rows on startup
  - cover create/update/reload behavior with focused tests
- Refresh CI/local validation for the active stack before any merge/deploy decision.

## Next Queue

1. Implement issue `#9` persistence and restart-reload tests in one small PR.
2. Run the local-first validation ladder on the active Phase 1 stack in a Rust-enabled environment:
   - `cargo fmt --all`
   - `cargo check --workspace`
   - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
   - `cargo test --workspace`
   - `cargo build --workspace`
   - `cargo audit`
   - `./scripts/generate_sbom.sh artifacts`
3. Clean up duplicate planning PRs `#14`-`#17` and stale/superseded PRs `#4` and `#8` before merge sequencing.
4. Add hosted branch protections and required approvals after local checks are green.
5. Continue toward replay/paper repeatability proof with at least three independent positive-cost-modeled runs before any tiny-live recommendation.
