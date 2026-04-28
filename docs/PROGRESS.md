# Progress

## Current phase
Phase 0: repo readiness

## Current audit finding
As of April 28, 2026, the repository is blocked in Phase 0 by compile-integrity failures and there is no open implementation PR.

Grounded current state:
- PR `#47`, PR `#49`, and PR `#50` are already merged into `main`.
- Issue `#48` is the active next code-bearing slice.
- Issue `#9` remains paused until compile integrity is restored.
- Several diverged remote branches still hold unmerged queue, frontend, and dashboard payload, but none of them should be treated as the active review target.
- `docs/WORK_STATUS.md` is the canonical stage tracker.
- `codex/single-integration-board-2026-04-28` is the canonical integration branch.

## Completed recently
- Earlier open-work consolidation was merged through PR `#47`.
- Queue-start and status-tracker cleanup landed through PR `#49` and PR `#50`.
- The repo now has one explicit ordered recovery queue in `docs/WORK_STATUS.md`.

## In progress
- Re-establish one active integration branch and one accurate status board for the remaining work.
- Audit stale diverged branches so future work is replayed from current `main` instead of stacked on outdated branches.
- Keep the next code-bearing step narrowly focused on issue `#48`.

## Deferred remote work inventory
These branches still contain unmerged payload and should be treated as reference material only until Phase 0 is green again:
- `codex/approval-queue-frontend-panel`
- `codex/read-only-approval-queue-api`
- `codex/approval-queue-storage-foundation`
- `codex/approval-queue-snapshot-reconcile`
- `codex/approval-queue-runtime-store-bridge`
- `codex/approval-queue-runtime-hydration-helpers`
- `codex/dashboard-shell-current-api`

## Next queue
1. Issue `#48`: repair `crates/pt-cli/src/main.rs` and `crates/pt-coinbase/src/lib.rs` only.
2. Recover `crates/pt-core/src/config.rs`.
3. Recover remaining parser-blocked dashboard and runtime files.
4. Run the Phase 0 validation ladder end to end.
5. Re-open deferred Phase 1 work as fresh small PRs on current `main`.

## Validation ladder
```bash
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo build --workspace
cargo audit
./scripts/generate_sbom.sh artifacts
```

## Risks and guardrails
- Do not enable live mode.
- Do not add or modify credentials.
- Do not raise risk caps.
- Do not merge stale diverged branches directly.
- Do not resume Phase 1 queue-runtime or dashboard expansion work before Phase 0 is green.
- Do not treat a status doc update as proof that compile recovery is complete.