# Phase 1 Runtime Wiring Execution Plan

## Phase

Phase 1: sandbox trading / paper ROI.

## Current audit finding

This round re-audited all materially relevant prior work using parallel sub-agent review plus live GitHub inspection.

Grounded repo state now:

- The approval-queue persistence helper stack is already the effective engineering base for the next slice:
  - PR `#27` queue storage foundation
  - PR `#29` queue snapshot reconciliation
  - PR `#30` runtime hydration merge helper
  - PR `#31` runtime-store bridge helper
- The remaining blocker for issue `#9` is now only the final runtime integration inside `crates/pt-cli/src/coinbase.rs`.
- Open PR `#32` is mixed overlap and largely repackages already-known queue work.
- Open PRs `#33` through `#36` are coordination-heavy checkpoint drafts and should not block the next code slice.
- The safest repo-control move now is to keep this PR as the single canonical execution-plan handoff and stop opening more planning-only PRs until the runtime-wiring PR lands.
- This environment still does not provide a safe local checkout plus authenticated publish path for the private repo, so it is not the right place to guess at a blind `coinbase.rs` edit.

## Recommended next action

Open one narrow implementation PR that only wires the existing approval-queue helpers into `CoinbaseWorkstationRuntime`.

Keep that PR limited to these responsibilities:

1. Open `ApprovalQueueStore` from `storage.sqlite_path` during runtime construction.
2. Hydrate queue-relevant rows (`draft`, `cancel_requested`) into `state.coinbase.orders` on startup.
3. Reconcile persisted queue state after local order-loop lifecycle mutations.
4. Reconcile persisted queue state after live-order sync so remote identity or status changes prune stale queue rows.
5. Keep `/api/v1/orders` and `/api/v1/approval-queue` read-only.
6. Avoid any further planning-only PRs on this blocker until that code slice lands.

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
cargo fmt --all
cargo check -p pt-cli
cargo test -p pt-cli
cargo test -p pt-dashboard
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo build --workspace
```

Phase 1 local-first ladder before merge when available:

```bash
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
