## Phase

Phase 0: repo readiness

## Current audit finding

A full re-audit of the work queue on April 27, 2026 shows the repository is blocked in Phase 0 before any more Phase 1 approval-queue work should continue.

Grounded from repo state and the open PR queue:
- PR `#40` identifies a real manifest parse blocker in `crates/pt-cli/Cargo.toml`.
- PR `#41` correctly pivots the repo back to compile-integrity recovery first.
- Open PR `#37` remains the right queued Phase 1 helper-stack branch, but it should stay parked until Phase 0 is green.
- PRs `#32` through `#39` are not the correct base for the next recovery move.

## Recommended next action
Open one new code-bearing hotfix PR from `main` with this exact scope:
1. absorb the manifest fix from PR `#40`
2. repair syntax and structural corruption in the seven Rust files listed above
3. keep the branch separate from PR `#37` and any Phase 1 queue-runtime work
4. prefer the last clean merged baseline commit `7da0bd8ba608f0f57e2edc83b7bf1f73cff955b1` as a reference source, but do not blindly revert files that now contain newer valid repo behavior
