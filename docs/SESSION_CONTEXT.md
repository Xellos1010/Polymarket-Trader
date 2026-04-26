# Session Context

Generated at UNIX epoch seconds: `1777230241`

## Note
Control-tower checkpoint after auditing the full Phase 1 queue on 2026-04-26.

What is grounded right now:
- `main` remains in Phase 1: sandbox trading / paper ROI.
- The strongest active implementation stack is:
  - PR `#12` for deterministic 3-run evidence gating
  - PR `#13` for the read-only approval-queue backend contract
  - PR `#18` for the read-only approval-queue frontend panel stacked on `#13`
  - PR `#11` for current-API frontend fixture coverage
- The next runtime blocker is still issue `#9`: queue-relevant workstation orders in the Coinbase workstation runtime are memory-only and do not survive restart.
- Coordination work is now duplicated across PRs `#14` through `#17`, `#19`, and `#20`.
- PRs `#4` and `#8` should be treated as stale or superseded unless they are explicitly rebuilt and revalidated.

## Recommended next implementation slice
Implement issue `#9` as one narrow backend PR stacked on PR `#13`:
- persist only `draft` and `cancel_requested` workstation orders via `storage.sqlite_path`
- hydrate those rows on startup into `DashboardState.coinbase.orders`
- prune persisted rows once orders leave queue-relevant statuses
- keep `/api/v1/orders` and `/api/v1/approval-queue` read-only from an operator-action standpoint
- add focused tests for create, update, and restart reload behavior

## Runtime surface for issue #9
- queue state source today:
  - `crates/pt-dashboard/src/lib.rs`
  - `DashboardState.coinbase.orders`
- runtime initialization and lifecycle:
  - `crates/pt-cli/src/coinbase.rs`
  - `CoinbaseWorkstationRuntime::new(...)`
  - `spawn_order_loop`
  - `process_draft_orders`
  - `process_cancel_requests`
  - `maybe_submit_auto_orders`
  - `merge_live_orders`
- existing SQLite/storage pattern to reuse:
  - `crates/pt-engine/src/lib.rs`
  - `Storage` with `rusqlite`, WAL mode, and `CREATE TABLE IF NOT EXISTS`

## Validation ladder
1. `cargo fmt --all`
2. `cargo check --workspace`
3. `cargo clippy --workspace --all-targets --all-features -- -D warnings`
4. `cargo test -p pt-cli`
5. `cargo test -p pt-dashboard`
6. `cargo test --workspace`
7. `cargo build --workspace`
8. `cargo audit`
9. `./scripts/generate_sbom.sh artifacts`
10. `python3 tools/coinbase_strategy_lab.py backtest --config config/coinbase_strategy_lab.json`
11. `python3 tools/coinbase_strategy_lab.py overlap --config config/coinbase_strategy_lab.json --auto-discovery`
12. `python3 tools/coinbase_strategy_lab.py optimize --config config/coinbase_strategy_lab.json`
13. `cargo run -p pt-cli -- run --config config/config.toml`
14. `./scripts/paper_soak.sh 86400 30 config/config.toml`

## Guardrails
- Do not enable live mode.
- Do not add or modify credentials.
- Do not raise risk caps.
- Do not add approval or execution mutation endpoints as part of issue `#9`.
- Do not treat tracker refreshes as evidence that Phase 1 gates have passed.

## Operator decision needed
No approval is needed to implement the narrow issue `#9` persistence slice.
Explicit approval is still required for merge, deployment, live mode, live credentials, or a tiny live pilot.
