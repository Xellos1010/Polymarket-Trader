# Phase 1 Next Round Checkpoint - 2026-04-27

## Phase
Phase 1: sandbox trading / paper ROI.

## Current audit finding
Grounded review of the current `main` branch, merged PR history through `#26`, issue `#9`, `docs/PROGRESS.md`, `docs/SESSION_CONTEXT.md`, `docs/APPROVAL_QUEUE_PERSISTENCE_PLAN.md`, and the current dashboard/runtime code shows:

- the repository is still in Phase 1 and should stay sandbox-only
- recent merged work materially improved test safety and operator visibility:
  - `#11` fixture-backed dashboard frontend tests
  - `#12` hardened three-run evidence gating
  - `#13` read-only approval queue API
  - `#18` read-only approval queue frontend panel
  - `#26` deterministic risk and quote failure-path tests
- the next runtime blocker is unchanged: issue `#9` because workstation approval-queue state is still memory-only in the Coinbase workstation runtime
- the read-only queue surface is already correct enough for operators; the missing step is durability and restart hydration, not more UI churn
- this workspace did not provide a normal authenticated local checkout for the private repository, so no fresh local Rust validation was run in this cycle

## Delegated parallel findings
Two parallel codebase reviews agreed on the same smallest safe slice.

Shared conclusion:
- implement persistence in `crates/pt-cli/src/coinbase.rs`
- reuse the existing SQLite/WAL style from `crates/pt-engine/src/lib.rs`
- keep `crates/pt-dashboard/src/lib.rs` read-only
- add focused create/update/reload tests instead of broadening scope

Recommended implementation shape:
1. Add a small workstation queue store in `pt-cli`, preferably a helper such as `crates/pt-cli/src/approval_queue_store.rs`.
2. Reuse `storage.sqlite_path` and create a dedicated workstation queue table.
3. Hydrate queue-relevant rows into `DashboardState.coinbase.orders` before the workstation loops start.
4. Snapshot-sync queue-relevant rows during the order loop:
   - upsert `draft`
   - upsert `cancel_requested`
   - delete rows that move out of queue-relevant statuses
5. Keep `/api/v1/orders` and `/api/v1/approval-queue` informational and read-only.

## Recommended next action
Open one small code PR for issue `#9` with this exact scope:

- persist only queue-relevant workstation order state needed for operator review
- hydrate persisted rows on startup
- prune stale persisted rows when orders leave the queue
- update `docs/data/SCHEMA.md`
- add focused tests for create, update, and restart reload behavior

## Acceptance criteria
- `draft` workstation orders survive restart
- `cancel_requested` workstation orders survive restart
- persisted queue rows are removed once orders move to `open`, `filled`, `canceled`, `rejected`, or `auto_canceled`
- dashboard queue reads remain deterministic after restart
- no new mutation endpoints are added
- no live mode, credential, deployment, or risk-cap changes are introduced

## Validation commands
Run the local-first ladder before merge or deployment claims:

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
- do not treat a docs/control-tower refresh as proof that Phase 1 gates passed
- do not expand persistence into approval or execution authority
- do not enable live mode automatically
- do not add credentials to the repository
- do not raise risk caps above the current guarded defaults

## Codex-ready task prompt
Title:
Persist Coinbase workstation approval queue state

Repository:
Xellos1010/Polymarket-Trader

Goal:
Add durable SQLite persistence and startup hydration for queue-relevant workstation orders so restart-safe operator review is deterministic without changing execution authority.

Context:
- `crates/pt-cli/src/coinbase.rs` owns the workstation runtime and mutates `state.coinbase.orders`
- `crates/pt-dashboard/src/lib.rs` already exposes a read-only `/api/v1/approval-queue`
- `crates/pt-engine/src/lib.rs` already contains the repository's preferred SQLite/WAL storage pattern
- issue `#9` is the active runtime blocker
- `docs/APPROVAL_QUEUE_PERSISTENCE_PLAN.md` and this checkpoint define the intended narrow scope

Files likely involved:
- `crates/pt-cli/src/coinbase.rs`
- `crates/pt-cli/src/approval_queue_store.rs`
- `crates/pt-cli/Cargo.toml`
- `docs/data/SCHEMA.md`
- focused tests under `crates/pt-cli` and `crates/pt-dashboard`

Required implementation:
1. Add a small SQLite-backed approval queue store using `storage.sqlite_path`.
2. Create the workstation queue table if it does not exist.
3. Hydrate queue-relevant rows into runtime state before background loops begin.
4. Snapshot-sync queue-relevant rows from memory each order-loop cycle.
5. Keep queue surfaces read-only and non-autonomous.
6. Add focused tests for create, update, prune, and restart reload behavior.

Definition of done:
- restart-safe queue state is deterministic and test-backed
- read-only queue API behavior remains unchanged
- no live-mode, credential, deployment, or risk-cap changes
- local-first validation is still required before merge

Safety:
- Do not commit secrets.
- Do not enable live mode.
- Do not raise risk caps.
- Do not deploy without approval.
- Keep changes small and reviewable.

## Operator decision needed
No approval is needed to prepare the narrow issue `#9` implementation PR.
Explicit approval is still required for merge, deployment, live credentials, live mode, or a tiny live pilot.
