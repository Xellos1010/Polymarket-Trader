# Phase 0 Slice 1 Start - 2026-04-27

## Phase
Phase 0: repo readiness

## Current audit finding
I re-audited the previously completed work, the merged consolidation branch, the stale closed handoff PRs, issue `#9`, and the current known parser blockers.

Grounded state after PR `#47`:
- PR `#47` is the effective landing point for the prior queue and is already merged.
- The older handoff PRs (`#37`, `#40`, `#41`, `#44`, `#45`, `#46`) are now historical context, not active delivery tracks.
- The repository is still blocked before more Phase 1 approval-queue runtime work should continue.
- The next clean engineering move is Recovery Slice 1 for:
  - `crates/pt-cli/src/main.rs`
  - `crates/pt-coinbase/src/lib.rs`
- I opened issue `#48` to make that slice explicit and reviewable.

## Recommended next action
Open one narrow code-bearing PR from current `main` for issue `#48` with this exact scope:
1. repair `enum Commands` / command-boundary corruption in `crates/pt-cli/src/main.rs`
2. repair duplicated header/import corruption in `crates/pt-coinbase/src/lib.rs`
3. keep the slice syntax/structure-only
4. avoid edits outside those two files

## Acceptance criteria
- `cargo fmt --all -- --check` no longer reports parser errors for:
  - `crates/pt-cli/src/main.rs`
  - `crates/pt-coinbase/src/lib.rs`
- the PR remains limited to those two files
- issue `#9` runtime wiring stays paused until later slices clear the remaining parser blockers

## Validation commands
```bash
cargo fmt --all -- --check
cargo check --workspace
```

## Risks and guardrails
- do not blindly full-file rollback if that removes intended newer auth/runtime behavior
- use baseline commit `7da0bd8ba608f0f57e2edc83b7bf1f73cff955b1` only as a structural reference
- do not enable live mode
- do not add or modify credentials
- do not raise risk caps
- do not mix queue-runtime work into this slice

## Why this handoff exists
This note is intentionally small: PR `#47` already absorbed the earlier queue/handoff stack, so the next helpful artifact is one precise start marker for the first remaining compile-recovery slice, not another broad planning branch.
