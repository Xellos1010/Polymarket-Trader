## Phase

Phase 0: repo readiness

## Current audit finding

A full re-audit of the work queue on April 27, 2026 shows the repository is blocked in Phase 0 before any more Phase 1 approval-queue work should continue.

Grounded from repo state and the open PR queue:
- PR `#40` identifies a real manifest parse blocker in `crates/pt-cli/Cargo.toml`.
- PR `#41` correctly pivots the repo back to compile-integrity recovery first.
- Open PR `#37` remains the right queued Phase 1 helper-stack branch, but it should stay parked until Phase 0 is green.
- PRs `#32` through `#39` are not the correct base for the next recovery move.

This handoff consolidates the parallel sub-agent analysis into one canonical next-round execution plan.

## Parallel audit summary

Three parallel analysis tracks were run:

1. File recovery track
- confirmed syntax or structural corruption in seven Rust files plus one manifest parse failure
- identified which files can be repaired surgically versus which need coherent reconstruction
- confirmed that most broken files should not be wholesale reverted to the older baseline because that would discard newer repo functionality

2. PR and branch strategy track
- confirmed the next compile-integrity PR should supersede PR `#40` and PR `#41`
- confirmed the next branch should start from `main`, not from PR `#37` or any docs-only branch
- confirmed Phase 1 queue-runtime work should remain out of scope for the recovery PR

3. Scope and acceptance track
- confirmed the hotfix gate is compile integrity only
- confirmed local-first validation must drive the next step
- confirmed `cargo audit` follow-up should stay separate if it still fails after syntax recovery

## Broken files and recovery guidance

### 1. `crates/pt-cli/Cargo.toml`
- current problem: duplicate `chrono.workspace = true` in `[dependencies]`
- recovery: remove the duplicate line only
- note: do not wholesale revert this file because `main` also carries newer dependency entries that should remain

### 2. `crates/pt-cli/src/main.rs`
- current problem: parse break around `Commands::StrategyProfileLoad`, plus duplicated/conflicting imports
- recovery: repair the current file structure and restore a valid enum/match layout
- note: do not wholesale revert because that would drop newer CLI commands and runtime-control wiring

### 3. `crates/pt-coinbase/src/lib.rs`
- current problem: merge-spliced import/header region with stray duplicated tokens near the top of the file
- recovery: surgical merge repair
- note: do not wholesale revert because newer auth/profile/websocket behavior appears to be expected elsewhere

### 4. `crates/pt-core/src/config.rs`
- current problem: duplicated and interleaved type definitions, including conflicting `CoinbaseAuthConfig` and `Execution*` blocks
- recovery: reconstruct one coherent current version rather than patching isolated lines
- note: do not wholesale revert because current code likely expects newer config fields

### 5. `crates/pt-dashboard/src/lib.rs`
- current problem: severe merge corruption, including duplicated imports, overlapping structs, and embedded frontend JavaScript leaking into Rust source
- recovery: rebuild from one coherent Rust source snapshot, then preserve the intended newer API surface carefully
- note: do not wholesale revert because that may regress current workstation/dashboard surfaces

### 6. `crates/pt-dashboard/tests/api_contract.rs`
- current problem: duplicated imports and broken fixture construction / request table assembly
- recovery: likely fastest to replace with a clean coherent test file and then re-add any newer endpoint assertions needed
- note: test-only file, so replacement risk is lower than in runtime modules

### 7. `crates/pt-quote/src/lib.rs`
- current problem: broken top-of-file import/header merge
- recovery: surgical header repair while preserving newer public helpers already present on `main`
- note: do not wholesale revert because newer quote helpers may now be used by other crates

### 8. `crates/pt-risk/src/lib.rs`
- current problem: malformed test helper block around `risk_cfg` with brace mismatch and a stray `pilot` splice
- recovery: surgical repair in the test section
- note: a full revert is probably possible for compile recovery, but a small targeted repair is safer

## Recommended next action

Open one new code-bearing hotfix PR from `main` with this exact scope:

1. absorb the manifest fix from PR `#40`
2. repair syntax and structural corruption in the seven Rust files listed above
3. keep the branch separate from PR `#37` and any Phase 1 queue-runtime work
4. prefer the last clean merged baseline commit `7da0bd8ba608f0f57e2edc83b7bf1f73cff955b1` as a reference source, but do not blindly revert files that now contain newer valid repo behavior

## Branch strategy

Use this exact branch shape for the next code round:
- branch from `main`
- branch name: `codex/phase0-compile-integrity-hotfix`
- supersede PR `#40` and PR `#41` in one new code-bearing PR
- keep PR `#37` untouched and queued behind Phase 0 recovery

## Acceptance criteria

The hotfix is complete only when all of the following are true:
- `cargo metadata --format-version 1` succeeds
- `cargo fmt --all` completes with no parser or delimiter errors
- `cargo check --workspace` passes
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes
- `cargo test --workspace` passes
- `cargo build --workspace` passes
- the PR contains only compile-integrity recovery changes

## Validation commands

```bash
cargo metadata --format-version 1
cargo fmt --all
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo build --workspace
cargo audit
```

## Expected follow-on risks after syntax recovery

These are likely next-wave issues once parsing is restored:
- semantic compile errors caused by type drift around `pt-core::config`
- clippy-deny warnings that are currently hidden behind parser failures
- test or OpenAPI contract drift in dashboard-related modules
- remaining `cargo audit` failures, which should stay in a separate PR unless they directly block compile integrity

## Risks and guardrails

- do not enable live mode
- do not add or modify credentials
- do not raise risk caps
- do not widen `/api/v1/orders` or `/api/v1/approval-queue` authority
- do not bundle Phase 1 approval-queue runtime wiring into this recovery step
- do not use PR `#37` as the branch base for compile recovery
- do not open more planning-only queue PRs on issue `#9` until Phase 0 is green again

## Codex-ready task prompt

Title:
Restore compile integrity on `main` and unblock the local-first validation ladder

Repository:
Xellos1010/Polymarket-Trader

Goal:
Repair the manifest parse failure and the seven syntax-corrupted Rust files on `main` without bundling any Phase 1 queue-runtime work.

Context:
- `docs/PHASE0_COMPILE_INTEGRITY_RECOVERY_2026-04-27.md`
- `docs/PHASE0_COMPILE_HOTFIX_EXECUTION_2026-04-27.md`
- PR `#40`
- PR `#41`
- PR `#37`
- issue `#9`
- clean reference commit `7da0bd8ba608f0f57e2edc83b7bf1f73cff955b1`

Files likely involved:
- `crates/pt-cli/Cargo.toml`
- `crates/pt-cli/src/main.rs`
- `crates/pt-coinbase/src/lib.rs`
- `crates/pt-core/src/config.rs`
- `crates/pt-dashboard/src/lib.rs`
- `crates/pt-dashboard/tests/api_contract.rs`
- `crates/pt-quote/src/lib.rs`
- `crates/pt-risk/src/lib.rs`

Required implementation:
1. Remove the duplicate `chrono.workspace = true` entry from `crates/pt-cli/Cargo.toml`.
2. Repair the parse break in `crates/pt-cli/src/main.rs` around `Commands::StrategyProfileLoad` and clean duplicated imports.
3. Repair the merge-spliced header/import corruption in `crates/pt-coinbase/src/lib.rs`.
4. Reconstruct one coherent `crates/pt-core/src/config.rs` from the current intended config surface.
5. Rebuild `crates/pt-dashboard/src/lib.rs` from one coherent Rust source snapshot, preserving the intended current workstation/dashboard APIs.
6. Replace or reconstruct `crates/pt-dashboard/tests/api_contract.rs` so it is syntactically valid and aligned with the intended dashboard API surface.
7. Repair the broken import/header merge in `crates/pt-quote/src/lib.rs` while keeping newer helper functions present on `main`.
8. Repair the malformed test helper block in `crates/pt-risk/src/lib.rs`.
9. Keep scope to compile-integrity recovery only.

Validation:
- `cargo metadata --format-version 1`
- `cargo fmt --all`
- `cargo check --workspace`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`
- `cargo build --workspace`
- `cargo audit`

Definition of done:
- the workspace parses again
- the local-first validation ladder is green through `cargo build --workspace`
- Phase 1 queue-runtime work is unblocked but not included in the PR

Safety:
- Do not commit secrets.
- Do not enable live mode.
- Do not raise risk caps.
- Do not deploy without approval.
- Keep changes small and reviewable.

## Operator decision needed

No approval is needed for this handoff PR.
Explicit approval is still required before merge, deployment, live mode, live credentials, or a tiny live pilot.
