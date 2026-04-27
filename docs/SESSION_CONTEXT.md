# Session Context

Generated at UNIX epoch seconds: `1777288058`

## Phase
Phase 1: sandbox trading / paper ROI.

## Current audit finding
I re-audited the repo state, issue `#9`, the merged queue, and the open draft PR stack on 2026-04-27.

What is grounded now:
- `main` remains in Phase 1.
- The materially relevant merged work is:
  - PR `#11`: fixture-backed dashboard frontend tests
  - PR `#12`: deterministic 3-run Phase 1 evidence gate
  - PR `#13`: read-only approval-queue backend contract
  - PR `#18`: read-only approval-queue frontend panel
  - PR `#24`: tracker refresh on `main`
- Open draft PRs now span `#32` through `#38`.
- PR `#37` is the only open code-bearing approval-queue branch that should drive the next runtime sequence.
- PR `#32` overlaps the same helper-stack area and is the highest merge/conflict risk.
- PRs `#33`, `#34`, `#35`, `#36`, and `#38` are docs/control-tower handoff drafts only and should not steer implementation sequencing.
- The remaining runtime blocker for issue `#9` is still the final `crates/pt-cli/src/coinbase.rs` wiring for restart-safe approval-queue behavior.

## Recommended next implementation slice
Land PR `#37` first, then open one narrow follow-up PR that only wires the approval-queue helpers into the Coinbase workstation runtime.

Keep that follow-up PR limited to:
- open `ApprovalQueueStore` from `storage.sqlite_path`
- hydrate queue-relevant rows (`draft`, `cancel_requested`) on workstation startup
- reconcile queue state after local lifecycle mutations
- reconcile queue state after live-order sync and identity changes
- keep `/api/v1/orders` and `/api/v1/approval-queue` read-only
- avoid live-mode, credential, deployment, or risk-cap changes

## Acceptance criteria
- PR `#37` remains the canonical helper-stack base.
- The next runtime-wiring PR touches only the `coinbase.rs` integration slice plus focused tests/docs as needed.
- Queue persistence stays limited to `draft` and `cancel_requested`.
- Restart/reload behavior becomes deterministic without widening execution authority.
- No live-mode enablement, credential changes, or risk-cap increases are introduced.

## Validation ladder
1. `cargo fmt --all`
2. `cargo check --workspace`
3. `cargo clippy --workspace --all-targets --all-features -- -D warnings`
4. `cargo test -p pt-cli`
5. `cargo test -p pt-dashboard`
6. `cargo test --workspace`
7. `cargo build --workspace`
8. `cargo audit`
9. `./scripts/generate_sbom.sh artifacts`
10. `python3 tools/coinbase_strategy_lab.py backtest --config config/coinbase_strategy_lab.json`
11. `python3 tools/coinbase_strategy_lab.py overlap --config config/coinbase_strategy_lab.json --auto-discovery`
12. `python3 tools/coinbase_strategy_lab.py optimize --config config/coinbase_strategy_lab.json`
13. `cargo run -p pt-cli -- run --config config/config.toml`
14. `./scripts/paper_soak.sh 86400 30 config/config.toml`

## Risks and guardrails
- Do not enable live mode.
- Do not add or modify credentials.
- Do not raise risk caps.
- Do not widen queue persistence beyond `draft` and `cancel_requested` in this blocker path.
- Do not add approval or execution mutation endpoints as part of the persistence slice.
- Do not let docs-only draft PR churn obscure the code-bearing sequence.

## Operator decision needed
No approval is needed to prepare the next narrow issue `#9` runtime slice.
Explicit approval is still required before merge, deployment, live mode, live credentials, or a tiny live pilot.