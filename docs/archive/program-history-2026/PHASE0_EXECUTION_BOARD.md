# Phase 0 Execution Board

## Phase
Phase 0: repo readiness

## Current blocker
The repository is still blocked before more Phase 1 approval-queue work should continue.

Grounded findings as of April 28, 2026:
- PR `#47` already consolidated the meaningful earlier work.
- PR `#49` already refreshed the queue-start handoff.
- there is currently no open implementation PR
- issue `#48` is the next code-bearing slice
- issue `#9` must remain paused until compile integrity is restored and the validation ladder is green again

## Canonical tracker
Use `docs/WORK_STATUS.md` as the single stage tracker.
This file keeps the ordered Phase 0 recovery ladder only.

## Recovery sequence

### S1: Compile recovery slice 1
- Owner: open
- Branch: new branch from `main`
- Issue: `#48`
- Status: ready to start
- Scope:
  - recover `crates/pt-cli/src/main.rs`
  - recover `crates/pt-coinbase/src/lib.rs`
  - keep scope to syntactic and compile-integrity recovery only
- Exit criteria:
  - files are syntactically coherent
  - `cargo fmt --all -- --check` no longer fails on these files
  - no Phase 1 feature expansion is mixed into the slice

### S2: Compile recovery slice 2
- Owner: open
- Branch: new branch from `main`
- Status: queued after S1
- Scope:
  - recover `crates/pt-core/src/config.rs`
- Exit criteria:
  - config types, defaults, and validation blocks are syntactically coherent
  - workspace validation can progress through config compilation

### S3: Compile recovery slice 3
- Owner: open
- Branch: new branch from `main`
- Status: queued after S2
- Scope:
  - recover remaining dashboard, contract, and runtime files still failing parse or delimiter checks
  - expected follow-up targets include:
    - `crates/pt-dashboard/src/lib.rs`
    - `crates/pt-dashboard/tests/api_contract.rs`
    - `crates/pt-quote/src/lib.rs`
    - `crates/pt-risk/src/lib.rs`
- Exit criteria:
  - workspace parser and delimiter failures are cleared
  - the repo can run the local-first ladder through build

### S4: Validation ladder
- Owner: open
- Status: blocked on S1 through S3
- Scope:
  - run the local-first validation ladder end to end
- Exit criteria:
  - `cargo fmt --all`
  - `cargo check --workspace`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo test --workspace`
  - `cargo build --workspace`
  - `cargo audit`
  - `./scripts/generate_sbom.sh artifacts`

## Resume rule
Do not resume issue `#9` queue-runtime wiring until S4 is green enough to trust local and CI validation again.

## Guardrails
- Do not enable live mode.
- Do not add or modify credentials.
- Do not raise risk caps.
- Do not widen approval or execution authority while Phase 0 is red.
- Keep recovery PRs reviewable and slice-sized.

## Historical context
Treat these as already-merged or superseded context rather than active execution boards:
- `#37`
- `#40`
- `#41`
- `#44`
- `#45`
- `#46`
- `#47`
- `#49`

## Recommended next action
1. Open the S1 code-bearing PR for issue `#48`.
2. Keep the scope limited to `pt-cli` and `pt-coinbase` compile recovery.
3. Continue the remaining slices in order until the validation ladder is green again.