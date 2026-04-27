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

## Files likely involved
- `crates/pt-cli/src/coinbase.rs`
- `crates/pt-cli/src/queue_runtime.rs`
- `crates/pt-cli/src/queue_runtime_store.rs`
- `crates/pt-cli/src/queue_store.rs`
- `crates/pt-cli/src/lib.rs`

## Acceptance criteria
- Startup hydration merges persisted `draft` and `cancel_requested` rows into `state.coinbase.orders`.
- Local status transitions out of queue-relevant states prune persisted queue rows.
- Live sync identity/status updates remove stale local queue rows deterministically.
- `/api/v1/orders` and `/api/v1/approval-queue` remain read-only.
- No live-mode enablement, credential handling changes, deployment changes, or risk-cap increases are introduced.

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
- Do not widen queue persistence beyond `draft` and `cancel_requested`.
- Do not add approval or execution mutation endpoints as part of this slice.
- Do not let more docs-only PR churn delay the code-bearing runtime wiring PR.

## Codex-ready task prompt
Title:
Wire approval queue persistence into the Coinbase workstation runtime

Repository:
Xellos1010/Polymarket-Trader

Goal:
Finish issue `#9` by wiring the existing approval-queue helpers into `crates/pt-cli/src/coinbase.rs` so queue-relevant workstation orders survive restart/reload without changing execution authority.

Context:
- `docs/ISSUE_9_RUNTIME_WIRING_MAP_2026-04-27.md`
- `docs/SESSION_CONTEXT.md`
- `docs/PROGRESS.md`
- issue `#9`
- PR `#37`

Files likely involved:
- `crates/pt-cli/src/coinbase.rs`
- `crates/pt-cli/src/queue_runtime.rs`
- `crates/pt-cli/src/queue_runtime_store.rs`
- `crates/pt-cli/src/queue_store.rs`
- focused tests in `crates/pt-cli`

Required implementation:
1. Open `ApprovalQueueStore` from `storage.sqlite_path` during workstation runtime construction.
2. Hydrate queue-relevant rows into `state.coinbase.orders` on startup.
3. Reconcile queue state after local lifecycle mutations.
4. Reconcile queue state after live-order sync and order-identity changes.
5. Add focused tests for startup hydration, local prune behavior, and identity-change reconciliation.
6. Keep `/api/v1/orders` and `/api/v1/approval-queue` read-only.

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
- queue state survives restart/reload for `draft` and `cancel_requested`
- stale queue rows are pruned after lifecycle and live-sync transitions
- test coverage exists for create/update/reload behavior
- no live-mode, credential, deployment, or risk-cap changes are introduced

Safety:
- Do not commit secrets.
- Do not enable live mode.
- Do not raise risk caps.
- Do not deploy without approval.
- Keep changes small and reviewable.
