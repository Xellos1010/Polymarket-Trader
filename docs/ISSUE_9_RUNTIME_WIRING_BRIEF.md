# Issue #9 Runtime Wiring Brief

## Phase
Phase 1: sandbox trading / paper ROI

## Current audit finding
The approval-queue persistence stack has moved forward materially, but issue `#9` is still not complete.

Grounded repo state as of 2026-04-27:
- PR `#27` added the SQLite approval-queue storage foundation.
- PR `#29` added snapshot reconciliation and stale-row pruning.
- PR `#30` added queue-runtime hydration and dedupe helpers.
- PR `#31` added the runtime-store bridge helpers.
- PR `#32` is the current tracker refresh draft on top of that history.

The remaining narrow runtime blocker is still `crates/pt-cli/src/coinbase.rs`.

## Recommended next action
Implement one small backend PR that wires the existing queue-store bridge into the Coinbase workstation runtime.

That PR should only:
1. Open `ApprovalQueueStore` from `storage.sqlite_path` during `CoinbaseWorkstationRuntime::new(...)`.
2. Hydrate persisted queue-relevant rows (`draft`, `cancel_requested`) into runtime orders before loops start.
3. Reconcile the current runtime order snapshot back into SQLite after runtime lifecycle mutations and after live-order sync updates.
4. Preserve the existing read-only operator surface for `/api/v1/orders` and `/api/v1/approval-queue`.

## Acceptance criteria
- Restart restores queue-relevant orders deterministically.
- Non-queue statuses are pruned from `approval_queue_orders`.
- Identity transitions do not duplicate rows when `client_order_id` stays stable and `order_id` changes.
- Runtime state wins over stale persisted payload for the same queue identity.
- No approval, execute, or autonomous-live mutation behavior is added.

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
- Avoid holding large runtime locks while doing SQLite work.
- Treat persistence failures as operationally visible problems, not as silent state loss.

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
3. Reconcile queue persistence after order lifecycle mutations and after live-order sync updates.
4. Keep queue persistence limited to `draft` and `cancel_requested`.
5. Keep `/api/v1/orders` and `/api/v1/approval-queue` read-only.
6. Update progress/session/runbook docs to reflect that runtime wiring has landed.

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
- read-only operator surfaces stay read-only
- no live-mode, credential, deployment, or risk-cap changes

Safety:
- Do not commit secrets.
- Do not enable live mode.
- Do not raise risk caps.
- Do not deploy without approval.
- Keep changes small and reviewable.
