# Phase 1 Runtime Wiring Execution Plan

## Phase

Phase 1: sandbox trading / paper ROI.

## Current audit finding

Grounded repo state after parallel review:

- `main` already contains the approval-queue stack through PRs `#27`, `#29`, `#30`, and `#31`.
- The read-only operator surfaces are already in place from PRs `#13` and `#18`.
- Open PRs `#32`, `#33`, `#34`, and `#35` are coordination-heavy drafts and should not block the next engineering step.
- The remaining blocker for issue `#9` is still the final runtime integration inside `crates/pt-cli/src/coinbase.rs`.
- This environment still does not provide a safe local checkout plus `gh` publish path, so the safest action here is to pin one exact next implementation slice in the repo itself rather than guess at an unvalidated blind runtime edit.

## Recommended next action

Open one narrow backend PR on top of `main` that wires the existing approval-queue persistence helpers into `CoinbaseWorkstationRuntime`.

Scope that PR to only these responsibilities:

1. Open the queue store from `storage.sqlite_path` during runtime construction.
2. Hydrate queue-relevant rows (`draft`, `cancel_requested`) into `state.coinbase.orders` on startup.
3. Reconcile persisted queue state after runtime order lifecycle changes and after live-order sync.
4. Keep `/api/v1/orders` and `/api/v1/approval-queue` read-only.
5. Add focused `pt-cli` tests for startup restore and post-mutation reconciliation.

## Acceptance criteria

- Restarting the Coinbase workstation restores persisted `draft` and `cancel_requested` rows.
- Orders that move to non-queue statuses are pruned from persisted queue state.
- Identity changes from local draft ids to remote exchange ids do not leave stale queue rows behind.
- Runtime wiring does not add approval, execution, or live-autonomy behavior.
- Queue persistence remains limited to `draft` and `cancel_requested` only.

## Validation commands

```bash
cargo check -p pt-cli
cargo test -p pt-cli
cargo check --workspace
cargo test --workspace
```

Phase 1 local-first ladder before merge when available:

```bash
cargo fmt --all
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo build --workspace
cargo audit
./scripts/generate_sbom.sh artifacts
```

## Risks and guardrails

- Do not enable live mode.
- Do not add or modify credentials.
- Do not raise risk caps.
- Do not widen persistence beyond `draft` and `cancel_requested`.
- Do not add approval or execution mutation endpoints as part of this slice.
- Do not treat helper-layer merges alone as proof that Phase 1 ROI gates passed.

## Parallel workstreams used for this checkpoint

- PR queue audit: confirmed the real active implementation base is the merged `#27 -> #29 -> #30 -> #31` chain.
- Issue `#9` audit: confirmed the remaining gap is runtime wiring, not storage design or dashboard surface design.
- Environment audit: confirmed the current runtime lacks a safe local checkout plus `gh` publish path, so repo-native queuing is the reliable action from this session.

## Codex-ready task prompt

Title:
Wire approval queue persistence into Coinbase workstation runtime

Repository:
Xellos1010/Polymarket-Trader

Goal:
Finish issue `#9` by wiring the existing queue persistence helpers into `crates/pt-cli/src/coinbase.rs` without changing execution authority.

Context:
- `docs/APPROVAL_QUEUE_PERSISTENCE_PLAN.md`
- `docs/PROGRESS.md`
- issue `#9`
- merged PRs `#27`, `#29`, `#30`, `#31`
- read-only queue surfaces already merged in PRs `#13` and `#18`

Files likely involved:
- `crates/pt-cli/src/coinbase.rs`
- `crates/pt-cli/src/queue_store.rs`
- `crates/pt-cli/src/queue_runtime.rs`
- `crates/pt-cli/src/queue_runtime_store.rs`
- `crates/pt-cli/src/lib.rs`
- `docs/PROGRESS.md`

Required implementation:
1. Open `ApprovalQueueStore` from `cfg.storage.sqlite_path` in `CoinbaseWorkstationRuntime::new(...)`.
2. Hydrate queue-relevant rows into `state.coinbase.orders` during startup.
3. Reconcile queue persistence after local status transitions, submissions, cancels, fills, and live-order sync.
4. Add focused tests for startup restore, state reconciliation, and stale-row pruning after identity change.
5. Keep all approval surfaces read-only and keep persistence scope limited to `draft` and `cancel_requested`.

Validation:
- `cargo check -p pt-cli`
- `cargo test -p pt-cli`
- `cargo check --workspace`
- `cargo test --workspace`

Definition of done:
- restart-safe queue recovery works
- stale persisted rows are pruned deterministically
- no live-mode or risk-cap changes
- no new approval/execution mutation authority

Safety:
- Do not commit secrets.
- Do not enable live mode.
- Do not raise risk caps.
- Do not deploy without approval.
- Keep changes small and reviewable.

## Operator decision needed

No approval is needed for the next runtime-wiring implementation PR.

Explicit approval is still required before any deployment, live credentials, live mode, merge, or tiny live pilot action.
