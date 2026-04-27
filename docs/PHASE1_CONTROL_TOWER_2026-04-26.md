# Phase 1 Control Tower Checkpoint

Date: 2026-04-26

## Phase

Phase 1: sandbox trading / paper ROI.

## Current audit finding

Grounded from `main`, open PRs `#11` through `#16`, issue `#9`, and current runtime code paths.

What is already completed on `main`:
- Rust workspace baseline and local-first operating docs are in place.
- Coinbase strategy-lab backtest, overlap, optimize, dashboard, and promotion tooling are in place.
- Replay acceptance tooling exists.
- Persistent engine-side SQLite and Parquet storage patterns already exist.

What is currently in flight:
- `#12` hardens the 3-run Phase 1 evidence gate and should land first because reproducibility is the highest unresolved safety priority.
- `#13` adds the read-only approval-queue API and should stay read-only.
- `#11` is an isolated frontend safety-net PR and is largely orthogonal.
- `#14`, `#15`, and `#16` all cover the same planning territory and now overlap heavily.

What is still blocked in code:
- workstation order lifecycle state still lives in memory in `crates/pt-cli/src/coinbase.rs` and `crates/pt-dashboard/src/lib.rs`
- restart-safe approval-queue state does not yet exist
- `storage.sqlite_path` already exists in config and `crates/pt-engine/src/lib.rs` already shows the repo’s SQLite persistence pattern

## Recommended next action

Open the next code PR for issue `#9`: persist and hydrate workstation approval-queue state using `storage.sqlite_path`, while keeping `/api/v1/orders` and `/api/v1/approval-queue` read-only operator surfaces.

Recommended queue order:
1. Merge `#12` after local validation.
2. Rebase and merge `#13`.
3. Merge `#11`.
4. Close `#14` and `#15` as superseded.
5. Keep only one coordination PR on the board at a time after the issue `#9` implementation PR is opened.

## Parallel tracks started

Track A: queue governance
- reduce planning PR overlap
- keep one control-tower checkpoint
- preserve review focus on reproducibility and restart safety

Track B: issue `#9` implementation design
- use SQLite-backed workstation-order persistence
- hydrate queue-relevant state on startup
- define explicit restart behavior for restored `draft` orders

## Acceptance criteria

- approval-queue-relevant order state survives restart
- startup hydration restores queue state deterministically
- `/api/v1/orders` and `/api/v1/approval-queue` remain read-only surfaces
- tests cover create, update, reload, and restart behavior
- no live-mode enablement
- no credential changes
- no risk-cap changes

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

- restart hazard: restored `draft` orders could auto-submit unless startup policy is explicit
- identity drift: local `order_id` values are later replaced by remote IDs, so persistence keys must be stable
- concurrency risk: dashboard handlers and runtime loops both mutate the shared workstation-order list
- planning churn risk: overlapping coordination PRs can obscure the real blocker and slow review

Guardrails:
- do not enable live mode
- do not add or modify credentials
- do not raise risk caps
- do not add autonomous approval or execution behavior
- default restored `draft` orders to hold-for-review unless an operator explicitly approves a different policy

## Codex-ready task prompt

Title:
Persist and hydrate workstation approval-queue state safely

Repository:
Xellos1010/Polymarket-Trader

Goal:
Add restart-safe persistence for workstation order queue state used by dashboard approval views, without expanding execution authority.

Context:
- `crates/pt-cli/src/coinbase.rs` owns the order lifecycle and startup/runtime loops.
- `crates/pt-dashboard/src/lib.rs` exposes order and queue APIs.
- `storage.sqlite_path` already exists in config.
- `crates/pt-engine/src/lib.rs` provides the existing SQLite initialization pattern.
- PR `#13` introduces the read-only approval-queue projection.

Files likely involved:
- `crates/pt-cli/src/coinbase.rs`
- `crates/pt-dashboard/src/lib.rs`
- `crates/pt-dashboard/tests/api_contract.rs`
- `crates/pt-core/src/types.rs` only if contract changes are truly required
- docs only where operator behavior needs clarification

Required implementation:
1. Add SQLite-backed workstation-order persistence in the Coinbase runtime.
2. Hydrate queue-relevant state at startup.
3. Persist state transitions deterministically on manual, automatic, cancel, fill, and reject paths.
4. Ensure `/api/v1/approval-queue` reads restored state without adding new mutation authority.
5. Add tests for create, update, reload, and restart safety behavior.

Validation:
- run the validation ladder listed above

Definition of done:
- queue state survives restart
- approval-queue endpoint remains read-only
- no live-mode, credential, or risk-cap changes
- tests cover create, update, reload, and restart semantics

Safety:
- Do not commit secrets.
- Do not enable live mode.
- Do not raise risk caps.
- Do not deploy without approval.
- Keep changes small and reviewable.

## Operator decision needed

Yes.

Before the issue `#9` implementation PR is merged, decide the startup policy for restored `draft` orders:
- recommended: restore as hold-for-review and require explicit operator action
- higher risk: restore and auto-submit

Without this decision, restart-safe persistence can accidentally widen execution behavior.
