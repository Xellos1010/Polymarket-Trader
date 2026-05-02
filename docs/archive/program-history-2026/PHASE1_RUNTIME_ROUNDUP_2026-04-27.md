# Phase 1 Runtime Roundup - 2026-04-27

## Phase
Phase 1: sandbox trading / paper ROI.

## Current audit finding
This control-tower round re-audited the repo history, issue `#9`, open PRs, and the prior queue-persistence stack using parallel sub-agent review.

Grounded state as of 2026-04-27:
- PR `#11` is merged.
- PR `#12` is merged.
- PR `#13` is merged and keeps the approval queue read-only.
- PR `#18` is merged and adds the read-only approval-queue frontend panel.
- PR `#24` is merged.
- PR `#31` merged the runtime-store bridge into the stacked helper path.
- PR `#32` is still draft, not mergeable, and overlaps tracker plus helper work.
- PR `#33` is a draft docs-only runtime brief.
- PR `#34` is a draft docs-only control-tower checkpoint.

Most important conclusion:
- the remaining engineering blocker is not another planning PR
- the next real implementation slice is the final `crates/pt-cli/src/coinbase.rs` runtime wiring for issue `#9`

## Recommended next action
Start one narrow backend PR that only wires the existing queue-store helpers into the Coinbase workstation runtime.

Do only this:
1. Open `ApprovalQueueStore` from `storage.sqlite_path` during `CoinbaseWorkstationRuntime::new(...)`.
2. Hydrate queue-relevant rows into `state.coinbase.orders` on startup via `queue_runtime_store::hydrate_runtime_orders(...)`.
3. Reconcile queue persistence after local lifecycle mutations in the order loop.
4. Reconcile queue persistence after live-order sync merges remote order state.
5. Keep `/api/v1/orders` and `/api/v1/approval-queue` read-only.
6. Keep persistence limited to `draft` and `cancel_requested` only.

## Acceptance criteria
- persisted `draft` and `cancel_requested` orders are restored deterministically after workstation restart
- rows are pruned once orders move to non-queue states such as `open`, `filled`, `canceled`, or `rejected`
- hydration does not duplicate orders when `client_order_id` stays stable but `order_id` changes after submit
- no approval or execution mutation endpoints are added
- no live-mode enablement, credential changes, deployment changes, or risk-cap changes are introduced

## Validation commands
```bash
cargo fmt --all
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test -p pt-cli
cargo test -p pt-dashboard
cargo test --workspace
cargo build --workspace
cargo audit
./scripts/generate_sbom.sh artifacts
python3 tools/coinbase_strategy_lab.py backtest --config config/coinbase_strategy_lab.json
python3 tools/coinbase_strategy_lab.py overlap --config config/coinbase_strategy_lab.json --auto-discovery
python3 tools/coinbase_strategy_lab.py optimize --config config/coinbase_strategy_lab.json
cargo run -p pt-cli -- run --config config/config.toml
./scripts/paper_soak.sh 86400 30 config/config.toml
```

## Risks and guardrails
- Do not widen persistence beyond queue-relevant statuses in this slice.
- Do not imply autonomous approval or execution on startup hydration.
- Do not enable live mode.
- Do not add credentials.
- Do not raise risk caps.
- Do not treat missing local validation as a pass.
