# Open PR Consolidation Review (2026-04-27)

## Purpose
This ledger records the branch-by-branch review used to consolidate the open PR queue into PR `#47` before branch pruning.

## Review outcome
The open queue separated into two groups:
- code-bearing payload that needed to stay live in the consolidated branch
- docs and tracker payload that needed preservation so no branch-only context would be lost during pruning

## Code-bearing payload preserved in PR #47
- PR `#37`: `crates/pt-cli/src/lib.rs`, `crates/pt-cli/src/queue_store.rs`, `crates/pt-cli/src/queue_runtime.rs`, `crates/pt-cli/src/queue_runtime_store.rs`
- PR `#46`: `crates/pt-cli/Cargo.toml`, `crates/pt-risk/src/lib.rs`
- PR `#45`: `./PHASE0_EXECUTION_BOARD.md` (this archive)

## Unique docs preserved in PR #47
- PR `#33`: `./ISSUE_9_RUNTIME_WIRING_BRIEF.md`
- PR `#34`: `./PHASE1_CONTROL_TOWER_2026-04-27.md`
- PR `#35`: `./PHASE1_RUNTIME_ROUNDUP_2026-04-27.md`
- PR `#36`: `./PHASE1_RUNTIME_WIRING_EXECUTION_PLAN_2026-04-27.md`
- PR `#38`: `./PHASE1_QUEUE_CONTROL_2026-04-27.md`
- PR `#39`: `./ISSUE_9_BRANCH_SEQUENCE_2026-04-27.md`, `./ISSUE_9_RUNTIME_WIRING_MAP_2026-04-27.md`
- PR `#41`: `./PHASE0_COMPILE_INTEGRITY_RECOVERY_2026-04-27.md`
- PR `#42`: `./PHASE0_COMPILE_HOTFIX_EXECUTION_2026-04-27.md`
- PR `#43`: `./PHASE1_NEXT_ROUND_2026-04-27.md`
- PR `#44`: `./PHASE0_RECOVERY_QUEUE_2026-04-27.md`

## Reviewed as overlap only
These PRs were reviewed and found to be overlap or superseded context rather than unique code that needed to survive as separate branches:
- PR `#32`
- PR `#40`

## Closure rule
After this review, PR `#47` is the single active submission branch. All other open PRs can be closed and later pruned at the branch layer without losing reviewed payload recorded above.

## Guardrails
- Local validation is still required before merge claims.
- Closing superseded PRs does not mean the Phase 0 gate is green.
- Runtime wiring in `crates/pt-cli/src/coinbase.rs` is still future work, not merged work.
