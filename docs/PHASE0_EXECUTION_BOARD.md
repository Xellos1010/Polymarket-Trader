# Phase 0 Execution Board

## Phase
Phase 0: repo readiness

## Current blocker
The repository is blocked before more Phase 1 approval-queue work should continue.

Grounded findings from the April 27, 2026 re-audit:
- `main` still contains a duplicate `chrono.workspace = true` entry in `crates/pt-cli/Cargo.toml`
- multiple Rust files remain merge-corrupted enough to break parser / format / compile steps
- PR `#40` is still necessary, but it is not sufficient on its own
- issue `#9` runtime wiring should stay paused until compile integrity is restored

## Recovery sequence

### S0: Manifest unblocker
- Owner: open
- Branch: `codex/fix-pt-cli-duplicate-chrono`
- PR: `#40`
- Status: open / draft
- Scope:
  - remove the duplicate `chrono.workspace = true` entry from `crates/pt-cli/Cargo.toml`
- Exit criteria:
  - `cargo metadata --format-version 1` loads successfully again
  - CI can proceed past manifest parsing
- Evidence link:
  - PR `#40`

### S1: Compile recovery slice 1
- Owner: open
- Branch: new branch from `main`
- PR: to be opened after `#40`
- Status: queued
- Scope:
  - recover `crates/pt-cli/src/main.rs`
  - recover `crates/pt-coinbase/src/lib.rs`
  - keep scope to syntactic / compile-integrity recovery only
- Exit criteria:
  - files are syntactically coherent
  - `cargo fmt --all` no longer fails on these files
  - no Phase 1 feature expansion is mixed into the slice
- Evidence link:
  - use the future PR plus CI run links

### S2: Compile recovery slice 2
- Owner: open
- Branch: new branch from `main`
- PR: to be opened after S1
- Status: queued
- Scope:
  - recover `crates/pt-core/src/config.rs`
- Exit criteria:
  - config types / defaults / validation blocks are syntactically coherent
  - `cargo check --workspace` can progress through config compilation
- Evidence link:
  - use the future PR plus CI run links

### S3: Compile recovery slice 3
- Owner: open
- Branch: new branch from `main`
- PR: to be opened after S2
- Status: queued
- Scope:
  - recover remaining dashboard / contract / runtime files still failing parse or delimiter checks
  - expected follow-up targets include:
    - `crates/pt-dashboard/src/lib.rs`
    - `crates/pt-dashboard/tests/api_contract.rs`
    - `crates/pt-quote/src/lib.rs`
    - `crates/pt-risk/src/lib.rs`
- Exit criteria:
  - workspace parser / delimiter failures are cleared
  - the repo can run the local-first ladder through build
- Evidence link:
  - use the future PR plus CI run links

### S4: Validation ladder
- Owner: open
- Branch: current recovered `main`
- PR: none; this is the gate after S0-S3 land
- Status: blocked on S0-S3
- Scope:
  - run the local-first validation ladder end to end
- Exit criteria:
  - `cargo fmt --all`
  - `cargo check --workspace`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo test --workspace`
  - `cargo build --workspace`
  - `cargo audit`
- Evidence link:
  - attach command output or CI links once run

## Resume rule
Do not resume issue `#9` queue-runtime wiring until S4 is green enough to trust local and CI validation again.

## Guardrails
- Do not enable live mode.
- Do not add or modify credentials.
- Do not raise risk caps.
- Do not widen approval or execution authority while Phase 0 is red.
- Keep recovery PRs reviewable and slice-sized.

## Superseded tracker PRs
Treat these as historical handoffs, not the active execution board:
- `#39`
- `#41`
- `#42`
- `#44`

## Recommended next action
1. Land PR `#40`.
2. Open the S1 code-bearing compile-recovery PR from `main`.
3. Keep issue `#9` paused until S4 passes.
