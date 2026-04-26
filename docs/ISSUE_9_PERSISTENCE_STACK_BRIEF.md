# Issue #9 Persistence Stack Brief

## Phase
Phase 1: sandbox trading / paper ROI

## Current audit finding
A full queue audit of the work already in flight shows that the repo is still in Phase 1 and that the real next backend blocker remains issue `#9`: restart-safe approval-queue persistence.

Grounded queue state from the current open PR stack:
- PR `#12` hardens the repeatability gate and should stay separate from runtime storage work.
- PR `#13` already introduces the read-only `/api/v1/approval-queue` contract.
- PR `#18` is stacked on `#13` and consumes that queue surface in the frontend.
- PR `#19` refreshes canonical trackers on `main`, but does not remove the runtime blocker.
- planning-only PR churn exists around the queue, so the next step should be implementation-directed and narrowly scoped.

## Recommended next action
Implement issue `#9` as a small runtime PR stacked on top of PR `#13`, not directly on `main`, unless PR `#13` merges first.

Recommended sequencing:
1. keep PR `#13` as the API-contract base for the approval queue
2. stack the persistence PR on top of `codex/read-only-approval-queue-api`
3. rebase PR `#18` after the persistence slice lands or stabilizes

This keeps the work reviewable and avoids duplicating the queue endpoint on `main` while it is still draft-only.

## Recommended implementation scope
Persist only queue-relevant workstation orders through `storage.sqlite_path`.

Persist only these statuses:
- `draft`
- `cancel_requested`

Do not persist:
- `open`
- `filled`
- `canceled`
- `rejected`
- `auto_canceled`
- `scan_only`

Recommended SQLite table shape:

```sql
CREATE TABLE IF NOT EXISTS workstation_order_queue (
  order_id TEXT PRIMARY KEY,
  client_order_id TEXT,
  product_id TEXT NOT NULL,
  side TEXT,
  route TEXT,
  status TEXT NOT NULL CHECK (status IN ('draft','cancel_requested')),
  live INTEGER NOT NULL,
  post_only INTEGER NOT NULL,
  limit_price REAL,
  base_size REAL NOT NULL,
  quote_notional REAL NOT NULL,
  expected_net_bps REAL NOT NULL,
  reason TEXT,
  created_ts_ms INTEGER,
  updated_ts_ms INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_workstation_order_queue_updated
  ON workstation_order_queue(updated_ts_ms DESC);
```

Recommended persistence rule:
- write the current eligible queue as an atomic snapshot replace in one transaction
- load only eligible queue rows on startup
- keep `/api/v1/orders` and `/api/v1/approval-queue` read-only

## Runtime hook plan
Files most likely involved:
- `crates/pt-cli/src/coinbase.rs`
- `crates/pt-cli/Cargo.toml`
- `crates/pt-dashboard/tests/api_contract.rs`
- `docs/RUNBOOK.md`
- `docs/api/dashboard-openapi.yaml`

Runtime hook recommendation:
1. in `CoinbaseWorkstationRuntime::new(...)`, initialize the queue store and hydrate persisted queue rows into `state.coinbase.orders`
2. in the workstation order loop, persist a queue snapshot after order-state transitions and before sleep
3. on shutdown, attempt one final best-effort snapshot write

## Acceptance criteria
- only `draft` and `cancel_requested` workstation orders survive restart
- queue state hydrates deterministically on startup
- non-queue statuses do not reload into runtime state
- `/api/v1/approval-queue` remains informational and non-autonomous
- focused tests cover create, update, and reload behavior
- no live-mode enablement, credential changes, deployment changes, or risk-cap changes

## Validation commands
Local-first validation is still required before merge:

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
- restoring non-queue statuses after restart can create duplicate paper/live behavior
- persisting auto-generated drafts can unintentionally carry automation forward after restart
- cancel-requested rows may refer to orders that no longer exist remotely
- repeated docs churn in `docs/PROGRESS.md` will create avoidable rebase conflicts with PRs `#13`, `#18`, and `#19`

Guardrails for the next PR:
- keep the persistence slice backend-focused
- avoid broad tracker edits unless strictly needed
- do not expand execution authority
- do not enable live mode
- do not raise risk caps

## Codex-ready task prompt
Title:
Persist issue #9 approval queue state on top of PR #13

Repository:
Xellos1010/Polymarket-Trader

Goal:
Add restart-safe persistence for approval-queue workstation orders without changing execution authority, stacked on top of the read-only approval-queue API branch.

Context:
- issue #9 is the next backend blocker after the current Phase 1 queue audit
- PR #13 already adds `/api/v1/approval-queue`
- PR #18 depends on that queue surface in the frontend
- current runtime queue state lives in memory in `crates/pt-cli/src/coinbase.rs`
- existing SQLite storage patterns live in `crates/pt-engine/src/lib.rs`

Files likely involved:
- `crates/pt-cli/src/coinbase.rs`
- `crates/pt-cli/Cargo.toml`
- `crates/pt-dashboard/tests/api_contract.rs`
- `docs/RUNBOOK.md`
- `docs/api/dashboard-openapi.yaml`

Required implementation:
1. add a narrow SQLite-backed queue store using `storage.sqlite_path`
2. persist only `draft` and `cancel_requested` workstation orders
3. hydrate persisted queue rows during workstation startup
4. write queue snapshots after workstation order-state transitions
5. add focused create/update/reload tests
6. keep queue surfaces read-only and non-autonomous

Validation:
- `cargo fmt --all`
- `cargo check --workspace`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test -p pt-cli`
- `cargo test -p pt-dashboard`
- `cargo test --workspace`

Definition of done:
- queue state survives restart for review-relevant statuses only
- dashboard queue reads remain deterministic
- non-queue statuses are not rehydrated
- tests cover create, update, and reload behavior
- no live-mode, credential, deployment, or risk-cap changes

Safety:
- Do not commit secrets.
- Do not enable live mode.
- Do not raise risk caps.
- Do not deploy without approval.
- Keep changes small and reviewable.

## Operator decision needed
No deployment or live-trading approval is needed for this planning/stacking step.
The next implementation PR should still remain draft until the local-first validation ladder can be run.
