# Approval Queue Persistence Plan

## Phase
Phase 1: sandbox trading / paper ROI

## Current audit finding
Workstation order and approval-queue state is still in-memory in the Coinbase workstation path.

Grounded repo context:
- `crates/pt-cli/src/coinbase.rs` owns the Coinbase workstation runtime and mutates `state.coinbase.orders`.
- `crates/pt-dashboard/src/lib.rs` exposes `/api/v1/orders` and, in open PR #13, a read-only `/api/v1/approval-queue` read model.
- `crates/pt-engine/src/lib.rs` already uses `storage.sqlite_path` for SQLite-backed runtime telemetry.
- `config/config.example.toml` already provides `storage.sqlite_path`, so a restart-safe queue can reuse the existing storage path.

## Recommended next action
Implement a small PR that adds durable workstation-order persistence and startup hydration in the Coinbase workstation runtime only.

Recommended scope:
1. Add a SQLite table for workstation orders using `storage.sqlite_path`.
2. Hydrate `state.coinbase.orders` from SQLite during Coinbase workstation startup.
3. Persist all queue-relevant order mutations as row upserts.
4. Keep `/api/v1/orders` and `/api/v1/approval-queue` read-only.
5. Add tests for create, update, and restart recovery.

## Acceptance criteria
- Queue-relevant states survive restart: `draft`, `cancel_requested`, `open`, `filled`, `canceled`, `rejected`.
- Dashboard reads are deterministic after restart.
- No new write endpoints are added.
- No live-mode enablement changes are introduced.
- No credentials, deployment settings, or risk caps are changed.

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
- Do not let persistence imply autonomous approval or execution.
- Persist order state only; do not replay side effects on startup.
- Preserve operator auditability for status transitions.
- Treat missing local validation as an unresolved gate, not a pass.

## Codex-ready task prompt
Title:
Persist Coinbase workstation order state for restart-safe approval queue

Repository:
Xellos1010/Polymarket-Trader

Goal:
Add durable SQLite persistence and startup hydration for workstation orders so approval-queue state survives restart without changing execution authority.

Context:
- `crates/pt-cli/src/coinbase.rs` currently mutates `state.coinbase.orders` in memory.
- `crates/pt-dashboard/src/lib.rs` reads workstation orders for dashboard views.
- `crates/pt-engine/src/lib.rs` already uses `storage.sqlite_path` for SQLite-backed runtime persistence.
- `config/config.example.toml` already defines `[storage].sqlite_path`.
- Open PR #13 adds a read-only approval queue view but not durability.

Files likely involved:
- `crates/pt-cli/src/coinbase.rs`
- optional small new helper under `crates/pt-cli/src/`
- `docs/data/SCHEMA.md`
- `docs/RUNBOOK.md`
- `docs/PROGRESS.md`
- focused tests under `crates/pt-cli` and `crates/pt-dashboard`

Required implementation:
1. Create a SQLite table for workstation orders.
2. Hydrate workstation orders from SQLite before serving dashboard traffic.
3. Upsert order state on create, cancel, submission, rejection, fill, and live-sync updates.
4. Keep all queue visibility read-only.
5. Add restart persistence tests and queue-state read coverage.

Definition of done:
- Restart-safe queue state is deterministic and test-backed.
- No live-mode enablement changes.
- No risk-cap changes.
- Docs are updated where schema or operator behavior changed.

Safety:
- Do not commit secrets.
- Do not enable live mode.
- Do not raise risk caps.
- Do not deploy without approval.
- Keep changes small and reviewable.
