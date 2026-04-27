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
