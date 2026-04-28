# Work Status

## Phase
Phase 0: repo readiness

## Current audit finding
The repository is still blocked in Phase 0 and the only active PR should remain the integration-tracking PR until the queue is fully consolidated.

Grounded state as of April 28, 2026:
- `main` already contains the earlier consolidation work from PR `#47`, PR `#49`, and PR `#50`.
- PR `#51` is now the only active integration PR.
- Issue `#48` is still the active next code-bearing slice.
- Issue `#9` remains paused until compile integrity is recovered.
- Diverged remote branches still carry unmerged payload, but they are behind current `main` and must not be treated as parallel active review tracks.

## Current stage
Stage: `phase0_slice1_waiting_on_compile_recovery`

Meaning:
- one canonical PR owns the active work queue
- the next safe code change is still issue `#48`
- no Phase 1 queue-runtime or frontend expansion should outrun compile recovery

## Single active integration branch
- branch: `codex/single-integration-board-2026-04-28`
- PR: `#51`
- purpose: keep one truthful PR and one truthful status board while the repo works through the remaining recovery and salvage queue
- status: active

## Stage execution rule
- If the current stage does not require a human decision, continue the active next step.
- If the current stage becomes `waiting_on_human_decision`, record the blocker here and advance the next eligible feature that does not depend on that decision and does not violate the current phase guardrails.
- Never open a second integration PR to keep work moving.
- When no more defined eligible features remain, stop adding scope and do one refinement pass for clarity, consistency, and software-engineering hygiene.

## Active next step
Open one narrow code-bearing follow-up for issue `#48` with this exact scope:
1. repair `crates/pt-cli/src/main.rs`
2. repair `crates/pt-coinbase/src/lib.rs`
3. keep changes syntax and structure only
4. avoid queue-runtime behavior in this slice

## Consolidated feature queue
| Order | Feature or slice | Source | Status | Human decision required | Next action |
|---|---|---|---|---|---|
| 1 | Compile recovery slice 1 (`pt-cli` + `pt-coinbase`) | issue `#48` | active next step | no | open one narrow code-bearing follow-up |
| 2 | Compile recovery slice 2 (`pt-core/src/config.rs`) | Phase 0 recovery queue | queued | no | start after slice 1 parser recovery is confirmed |
| 3 | Compile recovery slice 3 (remaining parser-blocked dashboard/runtime files) | Phase 0 recovery queue | queued | no | start after slice 2 |
| 4 | Phase 0 validation ladder | repo readiness gate | queued | no | run fmt, check, clippy, test, build, audit, SBOM |
| 5 | Deterministic risk and quote failure-path tests | issue `#23` | deferred until Phase 0 green | no | reopen as fresh small PR after validation ladder passes |
| 6 | Dashboard safety-net and read-only queue test consolidation | issue `#22` | deferred until Phase 0 green | no | replay surviving frontend/test payload on current `main` |
| 7 | Repeatable replay and paper evidence bundle | issue `#10` | deferred until Phase 0 green | no | refresh artifacts and gate report path after repo readiness recovers |
| 8 | Durable approval-queue persistence | issue `#9` | blocked by Phase 0 | no | resume only after compile integrity and validation ladder are green |
| 9 | Dashboard shell/UI salvage | `codex/dashboard-shell-current-api` | deferred until current API re-audit | possible later | salvage only the current-API-backed pieces after Phase 0 recovery |

## Outstanding remote work to consolidate later
- `codex/approval-queue-frontend-panel`
- `codex/read-only-approval-queue-api`
- `codex/approval-queue-storage-foundation`
- `codex/approval-queue-snapshot-reconcile`
- `codex/approval-queue-runtime-store-bridge`
- `codex/approval-queue-runtime-hydration-helpers`
- `codex/dashboard-shell-current-api`

Disposition for all of the above:
- reference only while Phase 0 is red
- do not merge them directly
- replay any still-useful payload only through fresh small PRs on current `main` after repo readiness is restored

## Acceptance criteria for the current stage
- one draft PR exists and remains the only active integration PR
- `docs/WORK_STATUS.md`, `docs/SESSION_CONTEXT.md`, `docs/PROGRESS.md`, and `docs/INTEGRATION_BOARD.md` all agree on the active stage and next slice
- the repo no longer implies multiple active PR tracks
- issue `#48` remains the next code-bearing slice
- the queue includes a defined rule for what happens when a stage needs human input

## Integration completion rule
Integration is complete for this cycle when:
- every defined non-blocked feature has either been merged, explicitly deferred, or marked as needing human approval
- no additional stale branch payload remains untriaged
- the remaining open work is represented truthfully in this file
- the final pass has refined status and planning files for consistency and engineering clarity

## Validation commands
```bash
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo build --workspace
cargo audit
./scripts/generate_sbom.sh artifacts
```

## Human decision gates
No human decision is needed to continue the current integration-tracking PR or to start issue `#48`.

Human approval is still required before:
- merge
- deployment
- live mode
- live credentials
- risk-cap increases
- a tiny live pilot

## Risks and guardrails
- Do not enable live mode.
- Do not add or modify credentials.
- Do not raise risk caps.
- Do not mix queue-runtime wiring into the Phase 0 recovery slice.
- Do not treat stale diverged branches as merge-ready evidence.
- Do not resume Phase 1 work before compile integrity is restored.
- Do not expand scope when the next safe action is still blocked on repo readiness.

## Status ownership
This file is the canonical work-stage tracker.
Update it whenever the active stage, blocker, integration branch, eligible next feature, or decision-gate state changes.
