# Session Context

Generated at UNIX epoch seconds: `1777242900`

## Phase
Phase 1: sandbox trading / paper ROI.

## Current audit finding
Re-audited the repo after the approval-queue persistence stack landed.

What is grounded on `main` now:
- PR `#13` merged the read-only `/api/v1/approval-queue` backend contract.
- PR `#18` merged the read-only approval-queue frontend panel.
- PR `#27` merged the SQLite storage foundation for queue-relevant workstation orders.
- PR `#29` merged snapshot reconciliation so stale queue rows are deleted when identity or status changes.
- PR `#30` merged runtime hydration helpers for queue-relevant orders.
- PR `#31` merged the runtime-store bridge helpers and documented the SQLite queue table in `docs/data/SCHEMA.md`.
- The next remaining blocker for issue `#9` is the final runtime wiring inside `crates/pt-cli/src/coinbase.rs`.

## Recommended next implementation slice
Start one narrow backend PR that only wires the existing helpers into the Coinbase workstation runtime.

Exact scope:
- open `ApprovalQueueStore` from `storage.sqlite_path` during `CoinbaseWorkstationRuntime::new(...)`
- hydrate queue-relevant rows into `state.coinbase.orders` on startup via `queue_runtime_store::hydrate_runtime_orders(...)`
- reconcile queue persistence from the runtime order snapshot via `queue_runtime_store::reconcile_runtime_orders(...)`
- call reconciliation from the order loop after local lifecycle mutations
- call reconciliation from the live-order sync loop after `merge_live_orders(...)`
- keep `/api/v1/orders` and `/api/v1/approval-queue` read-only
- keep persistence scoped to `draft` and `cancel_requested` only

## Acceptance criteria
- persisted `draft` and `cancel_requested` orders are visible after workstation restart
- queue rows are pruned once orders move to non-queue statuses such as `open`, `filled`, `canceled`, or `rejected`
- queue hydration does not duplicate orders when `client_order_id` is stable but `order_id` changes after submit
- no new approval or execution mutation endpoints are added
- no live-mode, credential, deployment, or risk-cap changes are introduced

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

## Risks and guardrails
- Do not widen persistence beyond queue-relevant statuses in this slice.
- Do not imply autonomous approval or execution on startup hydration.
- Do not enable live mode or add credentials.
- Do not raise risk caps.
- Treat missing local validation as an unresolved gate, not a pass.

## Operational note
This control-tower round used GitHub-backed inspection plus parallel sub-agent analysis to confirm the next slice. A full local checkout was not available in this environment, so no cargo validation was rerun here.

## Operator decision needed
No approval is needed to prepare the final `coinbase.rs` runtime-wiring PR.
Explicit approval is still required before merge, deployment, live mode, live credentials, or any tiny-live pilot action.
