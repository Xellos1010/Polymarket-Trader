# Session Context

Generated at UNIX epoch seconds: `1777348800`

## Note
Control-tower checkpoint after auditing current `main`, merged consolidation history, open issues, and diverged remote work branches on April 28, 2026.

## Current phase
Phase 0: repo readiness

## Grounded repo state
- PR `#47`, PR `#49`, and PR `#50` are already merged into `main`.
- There is no open implementation PR on the repository right now.
- Issue `#48` is the correct next code-bearing slice.
- Issue `#9` remains paused until compile integrity is recovered.
- Several remote branches still carry unmerged work, but they are behind `main` and must be treated as salvage/reference inputs rather than active review targets.

## Canonical status file
- `docs/WORK_STATUS.md`

## Canonical integration branch
- `codex/single-integration-board-2026-04-28`

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
- salvage any still-useful payload only after Phase 0 is green again and only through fresh small PRs on current `main`

## Recommended next implementation slice
Recover compile integrity for issue `#48` with one narrow code-bearing PR limited to:
- `crates/pt-cli/src/main.rs`
- `crates/pt-coinbase/src/lib.rs`

Required scope:
- repair import, header, and command-boundary corruption only
- preserve coherent newer behavior where it is already intact
- do not add queue-runtime behavior in this slice
- do not change live-mode behavior, credentials, deployment posture, or risk caps

## Acceptance criteria
- `cargo fmt --all -- --check` no longer reports parser errors for the two Slice 1 files
- the recovery PR remains limited to those two files
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
- Do not mix issue `#9` queue-runtime wiring into the Slice 1 compile-recovery PR.
- Do not treat diverged remote branches as current readiness evidence.
- Do not open more than one active integration PR for this queue.

## Operator decision needed
No approval is needed to open this integration-tracking PR.
Explicit approval is still required for merge, deployment, live mode, live credentials, or a tiny live pilot.