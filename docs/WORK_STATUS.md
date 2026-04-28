# Work Status

## Phase
Phase 0: repo readiness

## Current audit finding
The repository is still blocked in Phase 0 by compile-integrity failures, and PR `#51` should remain the only active integration PR until the queue either clears or reaches a real human-decision gate.

Grounded state as of April 28, 2026:
- `main` already contains the earlier consolidation work from PR `#47`, PR `#49`, and PR `#50`.
- PR `#51` is the only open integration PR.
- PR `#51` still carries tracker and control-file work only; the first code-bearing recovery slice has not landed yet.
- Issue `#48` is still the active next code-bearing slice.
- Issue `#9` remains paused until compile integrity is recovered and the local validation ladder is green.
- Visible `codex/` branches remain reference-only unless they are explicitly replayed onto PR `#51`.

## Audit stamp
- last audited on: April 28, 2026
- audit source: GitHub repository state, open issue queue, open PR queue, active integration branch documents, visible `codex/` branch inventory, and current branch versions of the Slice 1 Rust files
- validation evidence state: not rerun from a full private-repo checkout in this environment
- truth standard: tracker files may describe queue order, blockers, and execution policy, but they must not imply that the Phase 0 validation ladder is green

## Automation mirror
- `docs/WORK_STATUS.json` is the machine-readable mirror of this file.
- Keep the Markdown and JSON trackers aligned whenever the active stage, blocker, eligibility state, next slice, decision-gate state, validation evidence state, or branch-classification inventory changes.

## Current stage
Stage: `phase0_slice1_waiting_on_compile_recovery`

Stage contract:
- one canonical PR owns the active work queue
- exactly one queue item is marked `active_now`
- the next safe code change is still issue `#48`
- no Phase 1 queue-runtime or frontend expansion should outrun compile recovery

## Stage execution state
- stage owner: PR `#51`
- stage source: issue `#48`
- stage execution status: exact Slice 1 repairs identified and waiting on a proper code-bearing pass
- code-bearing progress on the active PR: not started yet
- next required environment: authenticated checkout of branch `codex/single-integration-board-2026-04-28`
- environment note: this audit environment does not currently provide a usable authenticated checkout of the private repository, so large-file Rust recovery must still be completed from a proper checkout before any readiness claim changes

## Exact blocker signature for the active slice
### `crates/pt-cli/src/main.rs`
- duplicate `pt_core` import blocks are present
- duplicate `pt_engine::TradingEngine` imports are present
- the `Commands` enum is broken at the `StrategyProfileLoad` to `Coinbase` boundary
- the immediate safe repair is:
  1. merge the `pt_core` imports so the file keeps one source of `AppConfig`, `EngineMode`, `ReplayAcceptanceReport`, `RuntimeRole`, and `MarketSnapshot`
  2. keep only one `pt_engine::TradingEngine` import
  3. close `StrategyProfileLoad` cleanly and keep `Coinbase` as its own subcommand variant
  4. preserve the newer command surface already present in the file

### `crates/pt-coinbase/src/lib.rs`
- the top-level import and header block is merge-corrupted
- duplicated `pt_core` fragments are present
- duplicated `reqwest::header` fragments are present
- the immediate safe repair is:
  1. keep the newer auth-manager, websocket, and runtime imports intact
  2. merge the older advanced-trade imports into that same top block without duplicates
  3. remove the duplicated header fragments without changing runtime behavior below the import section

## Workflow invariants
- one active integration branch
- one active integration PR
- one ordered queue
- one operator-readable status file plus one machine-readable mirror
- every visible `codex/` branch classified before salvage work begins
- tracker truth kept separate from validation evidence
- exactly one `active_now` feature at a time
- refinement only begins after the queue has no more defined eligible implementation work

## Single active integration branch
- branch: `codex/single-integration-board-2026-04-28`
- PR: `#51`
- purpose: keep one truthful PR and one truthful status board while the repo works through recovery, validation, and later salvage work
- status: active

## Queue state machine
Allowed queue states for this cycle:
- `active_now`: the one feature currently being worked
- `queued`: defined and ready after the current item
- `blocked_by_phase0`: defined but not eligible until Phase 0 is green
- `blocked_on_human_decision`: defined but waiting on explicit operator approval
- `deferred_until_phase0_green`: useful later, but intentionally paused behind repo readiness
- `deferred_until_current_api_reaudit`: useful later, but requires a fresh backend or API fit check first
- `completed`: finished on the active integration PR or already merged
- `explicitly_deferred`: intentionally left out of the current cycle

## Decision-gate tracker
- current stage requires human decision: no
- current blocker type: compile-integrity recovery, not operator approval
- current `active_now` item: issue `#48` compile recovery slice 1
- next automatically eligible item if the current stage later becomes `blocked_on_human_decision`: compile recovery slice 2 in `crates/pt-core/src/config.rs`
- canonical fallback policy: `docs/INTEGRATION_BOARD.md`

## Execution rule
- If the current stage does not require a human decision, continue the `active_now` item on PR `#51`.
- If the current stage becomes `blocked_on_human_decision`, record the blocker here and promote the next queue item that is both defined and independent of that decision.
- Never open a second integration PR to keep work moving.
- When no more defined eligible implementation items remain, stop adding scope and run one refinement pass for clarity, consistency, and software-engineering hygiene.

## Active next step
Continue PR `#51` with the issue `#48` code-bearing slice, limited to:
1. repair `crates/pt-cli/src/main.rs`
2. repair `crates/pt-coinbase/src/lib.rs`
3. keep changes syntax and structure only
4. preserve coherent newer behavior where it is already intact
5. avoid queue-runtime behavior in this slice
6. land the slice on the existing PR instead of starting a new branch or PR

If those repairs land cleanly, the next immediate action is:
1. `cargo fmt --all -- --check`
2. `cargo check --workspace`
3. if parser failures move to `crates/pt-core/src/config.rs`, promote Slice 2 to `active_now` on the same PR

## Queue summary
- active now: 1 item
- queued after current stage: 3 Phase 0 items
- deferred until Phase 0 green: 3 items
- blocked by Phase 0: 1 item
- deferred until current API re-audit: 1 item
- blocked on human decision right now: 0 items

## Consolidated feature queue
| Order | Feature or slice | Source | Queue state | Human decision required | Eligible after current item | Next action |
|---|---|---|---|---|---|---|
| 1 | Compile recovery slice 1 (`pt-cli` + `pt-coinbase`) | issue `#48` | active_now | no | current item | continue on PR `#51` with a narrow code-bearing commit set from a full checkout |
| 2 | Compile recovery slice 2 (`pt-core/src/config.rs`) | Phase 0 recovery queue | queued | no | yes | promote to `active_now` after slice 1 parser recovery is confirmed |
| 3 | Compile recovery slice 3 (remaining parser-blocked dashboard/runtime files) | Phase 0 recovery queue | queued | no | yes | continue on PR `#51` after slice 2 |
| 4 | Phase 0 validation ladder | repo readiness gate | queued | no | yes | run fmt, check, clippy, test, build, audit, and SBOM on the consolidated branch |
| 5 | Deterministic risk and quote failure-path tests | issue `#23` | deferred_until_phase0_green | no | no | continue on PR `#51` after the validation ladder passes |
| 6 | Dashboard safety-net and read-only queue test consolidation | issue `#22` | deferred_until_phase0_green | no | no | replay surviving frontend or test payload onto PR `#51` after repo readiness recovers |
| 7 | Repeatable replay and paper evidence bundle refresh | issue `#10` | deferred_until_phase0_green | no | no | refresh artifacts and gate report path on PR `#51` after repo readiness recovers |
| 8 | Durable approval-queue persistence | issue `#9` | blocked_by_phase0 | no | no | resume on PR `#51` only after compile integrity and validation ladder are green |
| 9 | Dashboard shell or UI salvage | `codex/dashboard-shell-current-api` | deferred_until_current_api_reaudit | possible later | no | salvage only the current-API-backed pieces onto PR `#51` after Phase 0 recovery |

## Branch-classification coverage
The tracker treats every visible `codex/` branch as one of these states:
- active integration branch
- compile-recovery reference
- frontend or dashboard salvage reference
- approval-queue persistence reference
- planning or audit archive context

Current rule:
- only the active integration branch may carry new implementation work for this cycle
- every other branch is reference-only until the current phase gates allow targeted salvage

## Acceptance criteria for the current stage
- one draft PR exists and remains the only active integration PR
- the active PR still tells the truth about status and blockers while waiting for the first code-bearing slice
- `docs/WORK_STATUS.md`, `docs/WORK_STATUS.json`, `docs/SESSION_CONTEXT.md`, `docs/PROGRESS.md`, and `docs/INTEGRATION_BOARD.md` all agree on the active stage and next slice
- the repo no longer implies multiple active PR tracks
- every visible `codex/` branch is classified as active, salvage, blocked, or archive context
- issue `#48` remains the next code-bearing slice
- the queue includes a defined rule for what happens when a stage needs human input
- the status board clearly distinguishes tracker truth from validation evidence
- exactly one queue item is marked `active_now`

## Integration completion rule
Integration is complete for this cycle when:
- every defined feature has one truthful queue state
- every non-blocked and non-human-gated item has either been completed or explicitly deferred
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
- Do not mix queue-runtime wiring into the Phase 0 recovery slice.
- Do not raise risk caps.
- Do not treat stale diverged branches as merge-ready evidence.
- Do not resume Phase 1 work before compile integrity is restored.
- Do not expand scope when the next safe action is still blocked on repo readiness.

## Status ownership
This file is the operator-readable work-stage tracker.
Update it together with `docs/WORK_STATUS.json` whenever the active stage, blocker, integration branch, eligible next feature, queue state, decision-gate state, validation evidence state, or branch-classification inventory changes.
