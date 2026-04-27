# Issue #9 Runtime Wiring Brief

## Phase
Phase 1: sandbox trading / paper ROI

## Current audit finding
The approval-queue persistence stack is materially in place, but issue `#9` is still open because the Coinbase workstation runtime has not yet called the merged helper layer.

Grounded repo state as of 2026-04-27:
- PR `#27` added the SQLite approval-queue storage foundation.
- PR `#29` added snapshot reconciliation and stale-row pruning.
- PR `#30` added queue-runtime hydration and dedupe helpers.
- PR `#31` added the runtime-store bridge helpers.
- PR `#32` and PR `#33` are still open draft coordination PRs, so the cleanest code base for the next runtime slice is `main`.

The remaining narrow runtime blocker is still `crates/pt-cli/src/coinbase.rs`.

## Recommended next action
Implement one small backend PR from `main` that wires the existing queue-store bridge into the Coinbase workstation runtime.

That PR should only:
1. Open `ApprovalQueueStore` from `storage.sqlite_path` during `CoinbaseWorkstationRuntime::new(...)`.
2. Hydrate persisted queue-relevant rows (`draft`, `cancel_requested`) into runtime orders before loops start by calling `hydrate_runtime_orders(...)`.
3. Reconcile the current runtime order snapshot back into SQLite after local order lifecycle mutations by calling `reconcile_runtime_orders(...)` once per `spawn_order_loop` iteration, after:
   - `process_cancel_requests()`
   - `process_draft_orders()`
   - `advance_paper_orders()`
   - `maybe_submit_auto_orders()`
4. Reconcile the runtime snapshot again after live-order sync updates so `merge_live_orders(...)` identity/status transitions prune stale queue rows.
5. Preserve the existing read-only operator surface for `/api/v1/orders` and `/api/v1/approval-queue`.

## Exact hook points
The smallest complete runtime-wiring slice is:
- `CoinbaseWorkstationRuntime::new(...)`
- `CoinbaseWorkstationRuntime::spawn_order_loop(...)`
- `CoinbaseWorkstationRuntime::spawn_live_order_sync_loop(...)`
- `CoinbaseWorkstationRuntime::merge_live_orders(...)`

Use the already-merged helpers from:
- `ApprovalQueueStore::open(...)`
- `hydrate_runtime_orders(...)`
- `reconcile_runtime_orders(...)`

## Acceptance criteria
- Restart restores queue-relevant orders deterministically.
- Existing non-queue runtime orders are preserved during startup hydration.
- Non-queue statuses are pruned from `approval_queue_orders`.
- Identity transitions do not duplicate rows when `client_order_id` stays stable and `order_id` changes.
- Runtime state wins over stale persisted payload for the same queue identity.
- Repeated reconcile calls with an unchanged snapshot are idempotent.
- No approval, execute, or autonomous-live mutation behavior is added.

## Minimum test coverage
1. Startup hydration integration for `CoinbaseWorkstationRuntime::new(...)`.
2. Restart recovery for persisted `draft` and `cancel_requested` rows.
3. Dedupe on identity change when a local draft becomes a remote exchange order with the same `client_order_id`.
4. Runtime-precedence conflict when persisted and in-memory rows share queue identity.
5. Pruning after status transitions such as `Draft -> CancelRequested -> Open`.
6. Live-sync reconciliation after `merge_live_orders(...)` updates status or `order_id`.
7. Idempotent reconcile behavior when the runtime snapshot is unchanged.

## Edge cases to keep in scope
- Empty `client_order_id` should fall back to `order_id` identity.
- Hydration must not drop unrelated non-queue runtime orders.
- Mixed timestamp ordering should remain deterministic.
- Persistence failures should degrade safely with visible logging, not silent queue-state loss.

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
- Do not enable live mode.
- Do not add or modify credentials.
- Do not raise risk caps.
- Do not widen the queue scope beyond `draft` and `cancel_requested`.
- Do not add approval or execution mutation endpoints.
- Avoid holding large runtime locks while doing SQLite work.
- Treat this brief as a runtime-integration handoff, not as evidence that Phase 1 ROI gates have passed.

## Files likely involved
- `crates/pt-cli/src/coinbase.rs`
- `crates/pt-cli/src/queue_runtime_store.rs`
- `crates/pt-cli/src/queue_runtime.rs`
- `crates/pt-cli/src/queue_store.rs`
- `docs/PROGRESS.md`
- `docs/SESSION_CONTEXT.md`
- `docs/RUNBOOK.md`

## Codex-ready task prompt
Title:
Wire approval queue persistence into Coinbase workstation runtime

Repository:
Xellos1010/Polymarket-Trader

Goal:
Finish issue `#9` by hydrating and reconciling queue-relevant workstation orders through the existing SQLite-backed queue-store bridge.

Context:
- PR `#27`: SQLite approval-queue store foundation
- PR `#29`: snapshot reconciliation and stale-row pruning
- PR `#30`: queue-runtime hydration helpers
- PR `#31`: runtime-store bridge helpers
- This brief records the next narrow runtime slice after those merged helpers
- Branch from `main`, not from open draft docs PRs

Files likely involved:
- `crates/pt-cli/src/coinbase.rs`
- `crates/pt-cli/src/queue_runtime_store.rs`
- `crates/pt-cli/src/queue_runtime.rs`
- `crates/pt-cli/src/queue_store.rs`
- `docs/PROGRESS.md`
- `docs/SESSION_CONTEXT.md`
- `docs/RUNBOOK.md`

Required implementation:
1. Open `ApprovalQueueStore` from `storage.sqlite_path` during `CoinbaseWorkstationRuntime::new(...)`.
2. Hydrate persisted queue rows into runtime order state before runtime loops start.
3. Reconcile queue persistence after local order lifecycle mutations in `spawn_order_loop(...)`.
4. Reconcile queue persistence after live-order sync updates in `spawn_live_order_sync_loop(...)` or `merge_live_orders(...)`.
5. Keep queue persistence limited to `draft` and `cancel_requested`.
6. Keep `/api/v1/orders` and `/api/v1/approval-queue` read-only.
7. Add focused tests for restart hydration, dedupe, pruning, runtime precedence, and idempotent reconcile behavior.
8. Update progress/session/runbook docs to reflect that runtime wiring has landed.

Validation:
- `cargo fmt --all`
- `cargo check --workspace`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test -p pt-cli`
- `cargo test -p pt-dashboard`
- `cargo test --workspace`
- `cargo build --workspace`
- `cargo audit`
- `./scripts/generate_sbom.sh artifacts`

Definition of done:
- queue state survives restart for queue-relevant statuses only
- stale rows are pruned when orders leave the approval queue
- identity changes do not leave duplicate queue rows behind
- runtime state wins when the same queue identity exists in both persisted and live snapshots
- read-only operator surfaces stay read-only
- no live-mode, credential, deployment, or risk-cap changes

Safety:
- Do not commit secrets.
- Do not enable live mode.
- Do not raise risk caps.
- Do not deploy without approval.
- Keep changes small and reviewable.
