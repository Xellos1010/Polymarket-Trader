# Phase 1 Runtime Wiring Execution Plan

## Phase

Phase 1: sandbox trading / paper ROI.

## Current audit finding

This round re-audited all materially relevant prior work using parallel sub-agent review plus live GitHub inspection.

Grounded repo state now:

- Merged work already in place on `main`:
  - PR `#11` dashboard fixture tests
  - PR `#12` Phase 1 evidence-gate hardening
  - PR `#13` read-only approval queue API
  - PR `#18` read-only approval queue frontend panel
  - PR `#27` queue storage foundation
  - PR `#29` queue snapshot reconciliation
  - PR `#30` runtime hydration merge helper
  - PR `#31` runtime-store bridge helper
- The remaining engineering blocker for issue `#9` is still the final runtime integration inside `crates/pt-cli/src/coinbase.rs`.
- Open PRs `#32` through `#35` are overlapping coordination drafts and should not block the next code slice.
- This environment still does not provide a safe local checkout plus `gh` publish path, so the safest repo-native action from this run is to keep one canonical execution-plan PR updated rather than open another planning PR on top of the same blocker.

## Recommended next action

Open one narrow backend PR on top of `main` that wires the existing approval-queue persistence helpers into `CoinbaseWorkstationRuntime`.

Keep that PR limited to these responsibilities:

1. Open the queue store from `storage.sqlite_path` during runtime construction.
2. Hydrate queue-relevant rows (`draft`, `cancel_requested`) into `state.coinbase.orders` on startup.
3. Reconcile persisted queue state from the current runtime order snapshot after order-loop lifecycle changes.
4. Reconcile persisted queue state after live-order sync so remote identity/status changes prune stale queue rows.
5. Keep `/api/v1/orders` and `/api/v1/approval-queue` read-only.
6. Avoid any new planning-only PRs until that runtime slice lands.

## Acceptance criteria

- Restarting the Coinbase workstation restores persisted `draft` and `cancel_requested` rows.
- Orders that move to non-queue statuses are pruned from persisted queue state.
- Identity changes from local draft ids to remote exchange ids do not leave stale queue rows behind.
- Runtime wiring does not add approval, execution, or live-autonomy behavior.
- Queue persistence remains limited to `draft` and `cancel_requested` only.
- The next implementation PR is code-focused and reviewable, not another coordination-only branch.

## Validation commands

Targeted runtime slice validation:

```bash
cargo check -p pt-cli
cargo test -p pt-cli
cargo test -p pt-dashboard
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
python3 tools/coinbase_strategy_lab.py backtest --config config/coinbase_strategy_lab.json
python3 tools/coinbase_strategy_lab.py overlap --config config/coinbase_strategy_lab.json --auto-discovery
python3 tools/coinbase_strategy_lab.py optimize --config config/coinbase_strategy_lab.json
cargo run -p pt-cli -- run --config config/config.toml
./scripts/paper_soak.sh 86400 30 config/config.toml
```

## Risks and guardrails

- Do not enable live mode.
- Do not add or modify credentials.
- Do not raise risk caps.
- Do not widen persistence beyond `draft` and `cancel_requested`.
- Do not add approval or execution mutation endpoints as part of this slice.
- Do not treat helper-layer merges alone as proof that Phase 1 ROI gates passed.
- Do not create more docs-only checkpoint PRs for the same blocker unless repo state materially changes.

## Parallel workstreams used for this checkpoint

- Prior-work audit: confirmed the merged base is `#11`, `#12`, `#13`, `#18`, `#27`, `#29`, `#30`, `#31`.
- PR queue audit: confirmed `#32` through `#35` are coordination-heavy and should not block the next code slice.
- Runtime audit: confirmed the missing work is startup hydration plus runtime reconciliation in `crates/pt-cli/src/coinbase.rs`.
- Environment audit: confirmed this session can keep the queue grounded in GitHub, but not safely run a full local Rust checkout and publish workflow.

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
- `docs/SESSION_CONTEXT.md`
- issue `#9`
- merged PRs `#13`, `#18`, `#27`, `#29`, `#30`, `#31`

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
- `cargo test -p pt-dashboard`
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
