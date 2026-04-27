# Phase 1 Control-Tower Checkpoint (2026-04-27)

## Phase
Phase 1: sandbox trading / paper ROI.

## Current audit finding
A full re-audit of the repo and recent PR queue shows that the approval-queue persistence track is partially landed but not yet fully promoted on `main`.

Grounded repo state:
- `main` already includes the read-only approval-queue surfaces from PR `#13` and PR `#18`.
- PR `#31` merged the approval-queue runtime-store bridge into the stacked branch path.
- PR `#32` is the broad stacked tracker refresh and is still draft-only.
- PR `#33` is the narrow runtime-wiring brief and is still draft-only.
- No unresolved review threads are blocking PR `#32` or PR `#33` right now.

What the parallel sub-agent audit agreed on:
- the remaining engineering blocker for issue `#9` is still runtime wiring inside `crates/pt-cli/src/coinbase.rs`
- the helper layer already exists on the stacked branch path:
  - `crates/pt-cli/src/queue_store.rs`
  - `crates/pt-cli/src/queue_runtime.rs`
  - `crates/pt-cli/src/queue_runtime_store.rs`
- those helpers assume snapshot-based reconciliation and queue identity keyed by `client_order_id` first, then `order_id`
- the safe hook points are:
  - `CoinbaseWorkstationRuntime::new(...)`
  - `spawn_order_loop(...)`
  - `spawn_live_order_sync_loop(...)`
  - `merge_live_orders(...)`

## Recommended next action
Promote the approval-queue helper stack to a usable base, then land one narrow runtime PR that wires the existing helpers into the Coinbase workstation runtime.

That runtime PR should only:
1. Open `ApprovalQueueStore` from `storage.sqlite_path` during `CoinbaseWorkstationRuntime::new(...)`.
2. Hydrate persisted queue-relevant rows into `state.coinbase.orders` before runtime loops start.
3. Reconcile the current runtime order snapshot back into SQLite after local lifecycle changes in `spawn_order_loop(...)`.
4. Reconcile again after live-order sync updates so identity/status transitions prune stale queue rows.
5. Keep `/api/v1/orders` and `/api/v1/approval-queue` read-only.

## Acceptance criteria
- restart restores persisted `draft` and `cancel_requested` rows deterministically
- non-queue runtime orders are preserved during hydration
- stale queue rows are pruned when orders become `open`, `filled`, `canceled`, or `rejected`
- local-id to remote-id transitions do not leave duplicate queue rows behind when `client_order_id` is stable
- repeated reconcile calls with the same runtime snapshot are idempotent
- no live-mode, credential, deployment, approval-action, or risk-cap changes are introduced

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
```

## Risks and guardrails
- do not branch the runtime PR from the open docs drafts unless that is the explicit merge plan
- do not widen persistence beyond `draft` and `cancel_requested` in this slice
- avoid holding the runtime order write lock while doing SQLite work
- do not treat docs or draft PRs as evidence that Phase 1 ROI gates have passed
- do not enable live mode or inject credentials as part of queue persistence work

## Codex-ready task prompt
Title:
Wire approval queue persistence into Coinbase workstation runtime

Repository:
Xellos1010/Polymarket-Trader

Goal:
Finish issue `#9` by hydrating and reconciling queue-relevant workstation orders through the existing SQLite-backed queue-store bridge.

Context:
- `docs/SESSION_CONTEXT.md`
- `docs/PROGRESS.md`
- PR `#31`
- PR `#32`
- PR `#33`
- `crates/pt-cli/src/coinbase.rs`
- `crates/pt-cli/src/queue_store.rs`
- `crates/pt-cli/src/queue_runtime.rs`
- `crates/pt-cli/src/queue_runtime_store.rs`

Required implementation:
1. Open `ApprovalQueueStore` from `storage.sqlite_path` during `CoinbaseWorkstationRuntime::new(...)`.
2. Hydrate persisted queue rows before runtime loops begin.
3. Reconcile queue persistence after local lifecycle mutations in `spawn_order_loop(...)`.
4. Reconcile queue persistence after live-order sync updates in `spawn_live_order_sync_loop(...)` or `merge_live_orders(...)`.
5. Keep queue persistence limited to `draft` and `cancel_requested`.
6. Keep `/api/v1/orders` and `/api/v1/approval-queue` read-only.
7. Add focused tests for restart hydration, dedupe, pruning, runtime precedence, and idempotent reconcile behavior.

Definition of done:
- queue state survives restart for queue-relevant statuses only
- stale rows are pruned when orders leave the approval queue
- identity changes do not leave duplicate queue rows behind
- runtime state wins when the same queue identity exists in both persisted and live snapshots
- no live-mode, credential, deployment, or risk-cap changes

Safety:
- Do not commit secrets.
- Do not enable live mode.
- Do not raise risk caps.
- Do not deploy without approval.
- Keep changes small and reviewable.

## Operator decision needed
No approval is needed to continue with the narrow issue `#9` runtime-wiring slice.
Explicit approval is still required before merge, deployment, live mode, live credentials, or any tiny live pilot.
