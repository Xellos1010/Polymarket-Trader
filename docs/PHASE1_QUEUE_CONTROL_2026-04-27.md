# Phase 1 Queue Control - 2026-04-27

## Phase
Phase 1: sandbox trading / paper ROI.

## Current audit finding
Grounded from repo history, open PR inspection, issue `#9`, and parallel sub-agent review:

- `main` remains in Phase 1.
- Issue `#9` is still the next concrete runtime blocker.
- PR `#37` is now the single code-bearing approval-queue blocker worth advancing.
- PRs `#32` through `#36` are planning or checkpoint overlap and should not drive implementation sequencing.
- The missing work after PR `#37` is a narrow runtime wiring slice in `crates/pt-cli/src/coinbase.rs`.

## Recommended next action
After PR `#37` lands, open one small follow-up PR from `main` named `codex/issue-9-coinbase-runtime-wiring` and keep it limited to runtime integration of the existing helper stack.

Required scope:
1. Open `ApprovalQueueStore` from `storage.sqlite_path` during Coinbase workstation startup.
2. Hydrate queue-relevant orders (`draft`, `cancel_requested`) into runtime state on startup.
3. Reconcile persisted queue rows after local lifecycle mutations.
4. Reconcile persisted queue rows after live-order sync and identity changes.
5. Keep `/api/v1/orders` and `/api/v1/approval-queue` read-only.

## Acceptance criteria
- Restart restores queue-relevant workstation orders deterministically.
- Persisted rows are limited to `draft` and `cancel_requested`.
- Persisted rows are pruned once orders leave queue-relevant status.
- No live-mode enablement, credential changes, or risk-cap changes are introduced.
- No approval or execution mutation endpoints are added.

## Validation commands
```bash
cargo fmt --all
cargo check -p pt-cli
cargo test -p pt-cli
cargo test -p pt-dashboard
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
- Do not treat tracker work as evidence that the Phase 1 ROI gate has passed.

## Codex-ready task prompt
Title:
Wire approval-queue persistence into the Coinbase workstation runtime

Repository:
Xellos1010/Polymarket-Trader

Goal:
Hydrate and reconcile restart-safe approval-queue state in `crates/pt-cli/src/coinbase.rs` using the helper stack from PR `#37`.

Context:
- `docs/PHASE1_QUEUE_CONTROL_2026-04-27.md`
- issue `#9`
- PR `#37`
- `crates/pt-cli/src/coinbase.rs`
- `crates/pt-cli/src/queue_store.rs`
- `crates/pt-cli/src/queue_runtime.rs`
- `crates/pt-cli/src/queue_runtime_store.rs`

Files likely involved:
- `crates/pt-cli/src/coinbase.rs`
- optional small test additions if hook coverage is needed

Required implementation:
1. Open the queue store from `cfg.storage.sqlite_path`.
2. Hydrate persisted queue rows on startup.
3. Reconcile queue rows after local lifecycle changes.
4. Reconcile queue rows after live-order sync.
5. Keep approval surfaces read-only.

Definition of done:
- restart-safe queue state is wired into the runtime
- queue scope stays limited to operator-review states
- validation commands above pass in a real checkout
- no live authority is expanded

Safety:
- Do not commit secrets.
- Do not enable live mode.
- Do not raise risk caps.
- Do not deploy without approval.
- Keep changes small and reviewable.

## Operator decision needed
No approval is needed to implement the narrow runtime wiring slice.
Explicit approval is still required for merge, deployment, live mode, live credentials, or a tiny live pilot.
