# Phase 1 Queue Audit - 2026-04-26

## Phase
Phase 1: sandbox trading / paper ROI.

## Current audit finding
Grounded review of `main`, open PRs, and issue #9 shows the current queue is converging on the right priorities but still has overlap and one clear runtime blocker.

What is already in flight:
- PR #12 hardens the 3-run evidence gate and makes repeatability a strict pass/fail requirement.
- PR #13 adds a read-only approval queue API surface for operator visibility.
- PR #11 adds fixture-backed frontend tests around the current workstation API.
- PR #14 and PR #15 are both planning-only coordination PRs for the same next slice.

What remains blocked:
- approval queue state is still runtime-memory only
- restart recovery for draft and cancel-requested workstation orders is not yet durable
- Phase 1 evidence is still incomplete until the local-first validation ladder runs in a Rust-capable environment

## Recommended merge and queue order
1. Merge PR #12 after local validation, because repeatability evidence is the strictest Phase 1 gate.
2. Merge PR #13 after local validation, because it gives operators a read-only queue view without expanding authority.
3. Merge PR #11 after local validation, because it strengthens the frontend safety net with minimal overlap.
4. Close or supersede PR #14 after the next implementation PR is opened, because PR #15 already overlaps it heavily.
5. Keep PR #15 as a planning checkpoint only until the issue #9 implementation PR is open.
6. Start issue #9 as the next code PR: durable approval-queue persistence and restart recovery using `storage.sqlite_path`.

## Recommended next action
Open one focused PR for issue #9 that persists only review-relevant workstation order state.

Smallest safe slice:
- add a tiny SQLite-backed approval queue store in `pt-cli`
- persist only `draft` and `cancel_requested` orders
- hydrate those rows on startup before loops begin
- snapshot-sync queue rows from memory so stale rows are deleted when status leaves review states
- keep `/api/v1/orders` and `/api/v1/approval-queue` read-only with respect to approval authority

## Acceptance criteria
- `draft` workstation orders survive restart
- `cancel_requested` workstation orders survive restart
- queue rows disappear once orders leave review states such as `canceled`, `filled`, or `rejected`
- no live-mode enablement changes
- no credential changes
- no risk-cap increases
- no new approval or auto-execution authority

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
- Do not treat open draft PRs as proof that Phase 1 gates are passing.
- Do not merge issue #9 without the local-first ladder.
- Do not broaden persistence into live approval authority.
- Do not raise risk caps above default guardrails.
- Do not enable live mode or add credentials in repo files.

## Codex-ready task prompt
Title:
Persist approval queue state across restart using SQLite

Repository:
Xellos1010/Polymarket-Trader

Goal:
Add durable persistence and restart recovery for approval-queue order state (`Draft`, `CancelRequested`) without changing execution authority.

Context:
- `crates/pt-cli/src/coinbase.rs` currently keeps workstation orders in memory.
- `config/config.example.toml` already defines `storage.sqlite_path`.
- `crates/pt-engine/src/lib.rs` shows the existing SQLite pattern used elsewhere in the repo.
- issue #9 and PR #13 define the approval-queue intent and read-only posture.

Files likely involved:
- `crates/pt-cli/src/coinbase.rs`
- `crates/pt-cli/Cargo.toml`
- `docs/data/SCHEMA.md`
- `docs/RUNBOOK.md`
- `docs/PROGRESS.md`

Required implementation:
1. Implement an `ApprovalQueueStore` in `pt-cli` backed by `storage.sqlite_path`.
2. Create a table for persisted queue rows, keyed by `order_id` and storing serialized workstation order payload plus timestamps.
3. Hydrate persisted queue rows into `state.coinbase.orders` at runtime startup.
4. Snapshot-sync queue rows from memory each order-loop iteration by upserting current queue rows and deleting stale ones.
5. Add focused tests for create, update, and restart-reload semantics.

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
- approval queue survives restart deterministically
- no new execution authority is added
- tests cover create, update, and reload behavior
- local-first validation ladder passes before merge

Safety:
- Do not commit secrets.
- Do not enable live mode.
- Do not raise risk caps.
- Do not deploy without approval.
- Keep changes small and reviewable.

## Operator decision needed
No approval is needed to prepare the issue #9 implementation PR.

Explicit approval is still required before any merge, deployment, live credentials, live mode, or tiny live pilot action.
