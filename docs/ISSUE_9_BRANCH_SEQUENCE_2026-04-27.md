# Issue #9 Branch Sequence (2026-04-27)

## Phase
Phase 1: sandbox trading / paper ROI.

## Current grounded finding
The repo has one code-bearing approval-queue branch worth advancing and several overlapping tracker branches.

Grounded queue state:
- PR `#37` is the only open code-bearing branch for the approval-queue helper stack.
- PR `#32` overlaps the same helper-stack surface and is the main merge/conflict risk.
- PRs `#33`, `#34`, `#35`, `#36`, `#38`, and this tracker PR `#39` are control-tower or docs branches.
- The remaining engineering blocker after `#37` is still the final runtime wiring inside `crates/pt-cli/src/coinbase.rs`.

## Execution decision
Use this exact sequence unless repo state changes materially:

1. Keep PR `#37` as the canonical code-bearing base for issue `#9`.
2. Treat PR `#32` as overlap risk, not as an additional base.
3. Use PR `#39` only as the canonical tracker handoff for the runtime slice.
4. After PR `#37` lands, open one narrow follow-up PR from `main` that only wires the queue helpers into `crates/pt-cli/src/coinbase.rs`.
5. Freeze additional docs-only PRs on issue `#9` until that code-bearing runtime wiring PR exists.

## Required scope for the next code PR
- open `ApprovalQueueStore` from `storage.sqlite_path`
- hydrate persisted `draft` and `cancel_requested` rows on startup
- reconcile queue state after local lifecycle mutations
- reconcile queue state after live-order sync and order-identity changes
- keep `/api/v1/orders` and `/api/v1/approval-queue` read-only
