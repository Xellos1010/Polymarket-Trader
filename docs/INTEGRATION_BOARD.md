# Integration Board

## Purpose
This file is the branch-consolidation board for the current cycle.

Use it to keep one active integration PR, one truthful current stage, and one ordered salvage queue for remote branches that still contain unmerged work.

## Active integration target
- branch: `codex/single-integration-board-2026-04-28`
- PR: `#51`
- phase: Phase 0: repo readiness
- current blocker: issue `#48` compile recovery slice 1 for `crates/pt-cli/src/main.rs` and `crates/pt-coinbase/src/lib.rs`

## Integration rule
- Do not open multiple parallel integration PRs.
- Do not merge stale diverged branches directly.
- Replay surviving payload from stale branches onto current `main` only after the Phase 0 validation ladder is green again.
- If the active stage hits a human-decision gate, keep the same PR open and move to the next eligible queued feature that does not depend on that decision.
- When no more eligible queued features remain, stop adding scope and run one best-practices refinement pass across the touched planning and status files.

## Workflow invariants
- Exactly one active integration branch exists for the current cycle.
- Exactly one open integration PR exists for the current cycle.
- `docs/WORK_STATUS.md`, `docs/WORK_STATUS.json`, `docs/SESSION_CONTEXT.md`, `docs/PROGRESS.md`, and this file agree on phase, stage, next step, and branch policy.
- Every visible `codex/` branch is classified before any salvage decision is made.
- No queue item behind a blocked dependency may be marked active.
- Tracker updates must never be presented as validation evidence.

## Current execution order
1. issue `#48` compile recovery slice 1
2. Phase 0 compile recovery slice 2 (`pt-core/src/config.rs`)
3. Phase 0 compile recovery slice 3 (remaining parser-blocked dashboard/runtime files)
4. Phase 0 validation ladder
5. issue `#23` deterministic risk and quote tests
6. issue `#22` dashboard safety-net and read-only queue test consolidation
7. issue `#10` repeatable replay and paper evidence bundle refresh
8. issue `#9` durable approval-queue persistence
9. dashboard shell or UI salvage from `codex/dashboard-shell-current-api`

## Queue progression rule
- Advance the first queue item that is both well-defined and unblocked.
- If that item requires human approval, record the decision gate in the status files and advance the next eligible item on the same PR.
- If no eligible items remain, stop queue expansion and run the final refinement pass.

## Full codex branch inventory status
Remote branch search for `codex/` currently returns 44 visible branches, all of which are classified below.

### Active branch
- `codex/single-integration-board-2026-04-28`: only active integration branch and PR source for `#51`

### Compile-recovery references
These branches may contain useful structural hints for Phase 0 recovery, but they are reference-only while PR `#51` is active:
- `codex/fix-pt-cli-duplicate-chrono`
- `codex/phase0-compile-hotfix-execution-2026-04-27`
- `codex/phase0-compile-recovery-2026-04-27`
- `codex/phase0-execution-board-2026-04-27`
- `codex/phase0-manifest-and-risk-scar-2026-04-27`
- `codex/phase0-recovery-queue-2026-04-27`
- `codex/phase0-slice1-compile-recovery`
- `codex/phase0-slice1-start-2026-04-27`

Disposition:
- use as structural reference only for issue `#48` and later Phase 0 slices
- do not open fresh PRs from them
- do not merge them directly

### Frontend and dashboard references
These branches may hold salvageable UI or test payload after Phase 0 is green again:
- `codex/approval-queue-frontend-panel`
- `codex/dashboard-shell-current-api`
- `codex/frontend-fixture-tests-current-api`
- `codex/full-scale-product-expansion`
- `codex/set-up-portfolio-and-orders-management`

Disposition:
- salvage only current-API-backed surfaces later
- keep all frontend or dashboard scope behind repo-readiness recovery
- do not let UI work outrun validation evidence

### Approval-queue persistence references
These branches remain relevant to issue `#9`, but stay blocked by Phase 0:
- `codex/approval-queue-runtime-hydration-helpers`
- `codex/approval-queue-runtime-store-bridge`
- `codex/approval-queue-snapshot-reconcile`
- `codex/approval-queue-storage-foundation`
- `codex/issue-9-persistence-stack-brief`
- `codex/issue-9-runtime-wiring-brief`
- `codex/phase1-approval-queue-persistence-plan`
- `codex/queue-helper-stack-on-main`
- `codex/read-only-approval-queue-api`

Disposition:
- keep issue `#9` paused until Phase 0 validation is green
- replay only the still-useful parts onto PR `#51` after repo readiness recovers
- keep the queue read-only and non-autonomous

### Planning and audit references
These branches are historical context or tracker material and should not be treated as active delivery lanes:
- `codex/codespaces-cloud-agent-tdd`
- `codex/consolidated-open-work-2026-04-27`
- `codex/issue-5-phase1-product-bootstrap`
- `codex/issue-21-local-validation-ladder`
- `codex/issue-23-risk-quote-tests`
- `codex/local-validation-bounded-smoke`
- `codex/phase1-canonical-next-round-2026-04-26`
- `codex/phase1-control-tower-2026-04-26`
- `codex/phase1-control-tower-2026-04-27`
- `codex/phase1-evidence-bundle-starter`
- `codex/phase1-next-round-after-pr31`
- `codex/phase1-next-round-after-pr37-audit`
- `codex/phase1-next-round-canonical-issue9`
- `codex/phase1-next-round-coordination-2026-04-26`
- `codex/phase1-next-round-start-2026-04-26`
- `codex/phase1-queue-audit-2026-04-26`
- `codex/phase1-queue-control-after-pr37`
- `codex/phase1-runtime-roundup-2026-04-27`
- `codex/phase1-runtime-wiring-execution-plan-2026-04-27`
- `codex/project-completion-pass`
- `codex/single-status-board-2026-04-28`

Disposition:
- archive as context only
- do not branch future implementation from them
- keep one truthful tracker set on the active integration branch

## Decision-gate policy
- Default rule: keep advancing the queue while the next item is well-defined and does not require a blocked human decision.
- Pause only when the next remaining items all require human approval or depend on unresolved validation evidence.
- Record the blocker and the fallback next feature in `docs/WORK_STATUS.md` and `docs/WORK_STATUS.json` whenever this happens.

## Refinement trigger
Run the final consistency and best-practices refinement pass when all remaining queue items are either:
- completed
- explicitly deferred
- blocked on human approval
- blocked on missing validation evidence

That pass should tighten status, planning, and operator-facing instructions without widening runtime or live-trading scope.

## Validation gate before any salvage work
```bash
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo build --workspace
cargo audit
./scripts/generate_sbom.sh artifacts
```

## Guardrails
- Do not enable live mode.
- Do not add or modify credentials.
- Do not raise risk caps.
- Do not bypass issue `#48` while Phase 0 remains red.
- Do not promote Phase 1 or Phase 2 work ahead of repo readiness.
- Do not treat tracker updates as a substitute for validation evidence.
- Do not let the Markdown and JSON status mirrors drift.
