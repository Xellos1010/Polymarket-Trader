# Session Context

Generated at UNIX epoch seconds: `1777348800`

## Note
Control-tower checkpoint after auditing current `main`, merged consolidation history, open issues, diverged remote work branches, and the active integration PR on April 28, 2026.

## Current phase
Phase 0: repo readiness

## Grounded repo state
- PR `#47`, PR `#49`, and PR `#50` are already merged into `main`.
- PR `#51` is the only active integration PR.
- Issue `#48` is the correct next code-bearing slice.
- Issue `#9` remains paused until compile integrity is recovered.
- Several remote branches still carry unmerged work, but they are behind `main` and must be treated as salvage or reference inputs rather than active review targets.

## Canonical status files
- `docs/WORK_STATUS.md`
- `docs/WORK_STATUS.json`

## Canonical integration branch
- `codex/single-integration-board-2026-04-28`

## Execution policy for this cycle
- keep one active integration PR
- take the next code-bearing slice from the ordered queue and land it on PR `#51`
- if a later stage requires human approval, keep the same PR and move to the next eligible queued feature
- when no more eligible queued features remain, finish with one refinement pass for consistency and best practices

## Deferred remote work inventory
- `codex/approval-queue-frontend-panel`: approval-queue UI and frontend tests
- `codex/read-only-approval-queue-api`: queue API plus frontend updates and persistence brief
- `codex/approval-queue-storage-foundation`
- `codex/approval-queue-snapshot-reconcile`
- `codex/approval-queue-runtime-store-bridge`
- `codex/approval-queue-runtime-hydration-helpers`
- `codex/dashboard-shell-current-api`

Current disposition for all of the above:
- do not merge them directly
- do not stack new PRs on those stale branches
- salvage any still-useful payload only after Phase 0 is green again and only onto PR `#51`

## Recommended next implementation slice
Recover compile integrity for issue `#48` on PR `#51`, limited to:
- `crates/pt-cli/src/main.rs`
- `crates/pt-coinbase/src/lib.rs`

Required scope:
- repair import, header, and command-boundary corruption only
- preserve coherent newer behavior where it is already intact
- do not add queue-runtime behavior in this slice
- do not change live-mode behavior, credentials, deployment posture, or risk caps

## Acceptance criteria
- `cargo fmt --all -- --check` no longer reports parser errors for the two Slice 1 files
- the recovery work stays limited to those two files
- queue-runtime work for issue `#9` stays paused until later slices clear the remaining parser blockers

## Validation ladder
1. `cargo fmt --all -- --check`
2. `cargo check --workspace`
3. `cargo clippy --workspace --all-targets --all-features -- -D warnings`
4. `cargo test --workspace`
5. `cargo build --workspace`
6. `cargo audit`
7. `./scripts/generate_sbom.sh artifacts`

## Guardrails
- Do not enable live mode.
- Do not add or modify credentials.
- Do not raise risk caps.
- Do not mix issue `#9` queue-runtime wiring into the Slice 1 compile-recovery work.
- Do not treat diverged remote branches as current readiness evidence.
- Do not open more than one active integration PR for this queue.

## Operator decision needed
No approval is needed to continue PR `#51` or to start issue `#48`.
Explicit approval is still required for merge, deployment, live mode, live credentials, or a tiny live pilot.
