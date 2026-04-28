# Progress

## Current phase
Phase 0: repo readiness

## Current audit finding
As of April 28, 2026, the repository is blocked in Phase 0 by compile-integrity failures and the work should stay consolidated into a single active integration PR.

Grounded current state:
- PR `#47`, PR `#49`, and PR `#50` are already merged into `main`.
- PR `#51` is now the only active integration PR.
- Issue `#48` is the active next code-bearing slice.
- Issue `#9` remains paused until compile integrity is restored.
- Remote branch search shows a larger `codex/` inventory than the earlier short tracker list, so the branch set has now been explicitly classified in the control files.
- `docs/WORK_STATUS.md` is the operator-readable stage tracker and `docs/WORK_STATUS.json` is the machine-readable mirror.

## Completed recently
- Earlier open-work consolidation was merged through PR `#47`.
- Queue-start and status-tracker cleanup landed through PR `#49` and PR `#50`.
- PR `#51` re-established one integration branch, one canonical work board, and one machine-readable work-stage mirror.
- The tracker set now classifies all visible `codex/` branches so future work does not accidentally reopen parallel review lanes.

## In progress
- Keep all current-cycle coordination in the single integration PR.
- Keep the next code-bearing step narrowly focused on issue `#48`.
- Record the fallback execution rule so work keeps moving if a later stage requires human input.
- Hold deferred branch payload as salvage or reference material only until Phase 0 is green again.

## Active execution rule
- Work the active next slice first.
- If a stage later requires human approval, move to the next eligible queued feature instead of opening a second integration PR.
- When no more eligible features remain, stop expanding scope and run one refinement pass for consistency and best practices.

## Branch classes now tracked
- compile recovery references
- frontend or dashboard salvage references
- approval-queue persistence references
- planning or audit archive references

Canonical branch-classification source:
- `docs/INTEGRATION_BOARD.md`
- `docs/WORK_STATUS.md`
- `docs/WORK_STATUS.json`

## Next queue
1. Issue `#48`: repair `crates/pt-cli/src/main.rs` and `crates/pt-coinbase/src/lib.rs` only.
2. Recover `crates/pt-core/src/config.rs`.
3. Recover remaining parser-blocked dashboard and runtime files.
4. Run the Phase 0 validation ladder end to end.
5. Re-open deferred Phase 1 work as fresh small PRs on current `main`.
6. Finish with a consistency and best-practices refinement pass once no more defined eligible features remain.

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
