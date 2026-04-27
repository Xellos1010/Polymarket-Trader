# Issue #9 Runtime Wiring Map (2026-04-27)

## Phase
Phase 1: sandbox trading / paper ROI.

## Current grounded finding
The remaining blocker for issue `#9` is the final runtime wiring in `crates/pt-cli/src/coinbase.rs`.

What is grounded now:
- `main` already carries the read-only approval-queue API and operator surface from PR `#13` and PR `#18`.
- The merged helper stack from PR `#27`, PR `#29`, PR `#30`, and PR `#31` established the storage, hydration, and reconciliation substrate.
- PR `#37` is the current code-bearing helper-stack consolidation branch and should remain the next engineering base.
- `crates/pt-cli/src/coinbase.rs` is still the missing runtime integration point.

## Recommended next action
After PR `#37` lands, open one narrow follow-up PR from `main` that only wires the approval-queue helpers into the Coinbase workstation runtime.

## Runtime hook points
1. `CoinbaseWorkstationRuntime::new`
   Open `ApprovalQueueStore` from `cfg.storage.sqlite_path` and hydrate `state.coinbase.orders` with persisted queue-relevant rows.
2. Local lifecycle mutation path
   Reconcile queue state after lifecycle transitions in `process_cancel_requests`, `process_draft_orders`, `advance_paper_orders`, and `maybe_submit_auto_orders`.
3. Shared mutation helpers
   Keep the narrowest integration in `update_order_status` and `complete_order_submission` so the queue store stays aligned when order status or identity changes.
4. Live sync path
   Reconcile again after `merge_live_orders(...)` so local draft ids that become remote exchange ids prune stale queue rows deterministically.
