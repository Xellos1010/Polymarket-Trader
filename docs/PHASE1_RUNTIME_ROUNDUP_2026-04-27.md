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

## Parallel analysis summary
Sub-agent review agreed on three points:
- the runtime blocker still lives in `crates/pt-cli/src/coinbase.rs`
- the helper layer already exists and should be called, not re-designed
- the open coordination PRs overlap and should not distract from the runtime-wiring PR

## Codex-ready task prompt
Title:
Wire approval queue runtime persistence into Coinbase workstation startup and lifecycle hooks

Repository:
Xellos1010/Polymarket-Trader

Goal:
Finish issue `#9` by wiring the existing approval-queue store helpers into `crates/pt-cli/src/coinbase.rs` without changing execution authority.

Context:
- issue `#9`
- merged read-only approval queue surfaces from PR `#13` and PR `#18`
- helper stack summarized in `docs/PHASE1_RUNTIME_ROUNDUP_2026-04-27.md`
- runtime-store bridge already exists and should be reused

Files likely involved:
- `crates/pt-cli/src/coinbase.rs`
- `crates/pt-cli/src/queue_runtime_store.rs`
- `crates/pt-cli/src/queue_runtime.rs`
- `crates/pt-cli/src/queue_store.rs`
- optional focused tests in `pt-cli`

Required implementation:
1. Open `ApprovalQueueStore` from `storage.sqlite_path` during runtime construction.
2. Hydrate queue-relevant rows into the in-memory order list before runtime loops start.
3. Reconcile persistence after local order lifecycle mutations.
4. Reconcile persistence after live-order sync merges remote order state.
5. Keep persistence scoped to `draft` and `cancel_requested` only.
6. Keep all approval surfaces read-only.

Definition of done:
- restart-safe queue hydration works for queue-relevant statuses only
- stale queue rows are pruned when order state leaves the review queue
- dedupe behavior remains stable across `client_order_id` / `order_id` transitions
- no live-mode, credential, deployment, or risk-cap changes

Safety:
- Do not commit secrets.
- Do not enable live mode.
- Do not raise risk caps.
- Do not deploy without approval.
- Keep changes small and reviewable.

## Operator decision needed
No approval is needed to prepare the next narrow runtime PR.
Explicit approval is still required before merge, deployment, live credentials, live mode, or any tiny live pilot action.
