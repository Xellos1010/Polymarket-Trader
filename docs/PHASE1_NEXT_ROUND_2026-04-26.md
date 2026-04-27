# Phase 1 Next Round Coordination (2026-04-26)

## Phase

Phase 1: sandbox trading / paper ROI.

## Current audit finding

Grounded review of `main`, the open PR queue, and the current issue backlog shows:

- `main` remains centered on replay, paper, and safety-gate work.
- PR #12 is the active reproducibility track for the 3-run Phase 1 evidence gate.
- PR #13 is the active operator-facing contract track for a read-only approval queue API.
- PR #11 is the active dashboard frontend safety-net track.
- PR #14 is a planning-only queue PR and should not block code work.
- issue #9 remains the next clean implementation blocker after the current evidence/API slices: workstation order state is still held in memory, so approval-queue state is not restart-safe.

## Recommended next action

Open the next implementation PR for issue #9 as a focused storage/runtime slice:

- persist workstation order lifecycle state to SQLite using the existing `storage.sqlite_path`
- hydrate persisted orders into the workstation on startup
- keep execution authority unchanged
- keep `/api/v1/orders` and `/api/v1/approval-queue` read-only
- add deterministic tests for create, update, and restart reload behavior

## Merge and queue order

1. Merge PR #12 once the local-first validation ladder passes.
2. Merge PR #13 next so the operator-facing queue contract is settled before persistence internals land.
3. Merge PR #11 as the frontend regression safety net.
4. Close or supersede PR #14 after the implementation slice is underway, because it is planning-only and overlaps with the active queue.
5. Then merge the issue #9 implementation PR only after local storage/runtime validation passes.

## Acceptance criteria

- approval-queue state survives restart and reload
- persisted workstation orders hydrate deterministically into runtime memory
- dashboard reads remain stable through existing read models
- tests cover create, update, cancel-requested, and reload paths
- no live-mode enablement, credential changes, deployment changes, or risk-cap changes

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

- Do not treat in-memory order state as durable until SQLite hydration is proven.
- Do not introduce mutating approval APIs or any path that implies autonomous execution.
- Do not allow state drift between runtime memory and SQLite writes.
- Do not let persistence work bypass Phase 1 replay, paper, and evidence gates.
- Do not enable live mode, add credentials, or raise risk caps.

## Likely files for issue #9

- `crates/pt-cli/src/coinbase.rs`
- `crates/pt-cli/src/order_store.rs` (new small persistence helper)
- `crates/pt-core/src/config.rs`
- `crates/pt-dashboard/src/lib.rs` (only if read-model shaping is required)
- `crates/pt-dashboard/tests/api_contract.rs`
- `docs/RUNBOOK.md`
- `docs/PROGRESS.md`

## Codex-ready task prompt

Title:
Persist workstation order state for restart-safe approval queue recovery

Repository:
Xellos1010/Polymarket-Trader

Goal:
Persist workstation order lifecycle state to SQLite and hydrate it on startup so approval-queue state survives restart without changing execution authority.

Context:
- issue #9 is the next Phase 1 blocker after the current evidence and read-only API PRs
- `main` holds workstation orders in memory today
- the safest path is to persist existing order state rather than invent new approval mutations

Files likely involved:
- `crates/pt-cli/src/coinbase.rs`
- `crates/pt-cli/src/order_store.rs`
- `crates/pt-core/src/config.rs`
- `crates/pt-dashboard/src/lib.rs`
- `crates/pt-dashboard/tests/api_contract.rs`
- `docs/RUNBOOK.md`
- `docs/PROGRESS.md`

Required implementation:
1. Add a small SQLite-backed store for workstation orders using existing `storage.sqlite_path`.
2. Hydrate persisted orders into runtime state during workstation startup.
3. Upsert order lifecycle transitions for create, cancel-requested, open, filled, canceled, and rejected states.
4. Keep `/api/v1/orders` and any approval-queue surface read-only.
5. Add tests for create, update, and restart reload behavior.
6. Update operator docs only as needed to explain restart-safe queue expectations.

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
- approval-queue state survives restart
- hydration is deterministic and idempotent
- tests cover create, update, and reload paths
- no live-mode, credential, deployment, or risk-cap changes

Safety:
- Do not commit secrets.
- Do not enable live mode.
- Do not raise risk caps.
- Do not deploy without approval.
- Keep changes small and reviewable.
