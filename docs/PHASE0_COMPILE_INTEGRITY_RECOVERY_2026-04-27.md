## Phase

Phase 0: repo readiness

## Current audit finding

Re-auditing all previously completed work, the open PR queue, issue `#9`, and the latest CI state shows that the repository is currently blocked in Phase 0 before the next Phase 1 queue-runtime slice can safely continue.

Grounded findings as of 2026-04-27:
- PR `#40` is valid and still necessary, but it only removes the duplicate `chrono.workspace = true` entry in `crates/pt-cli/Cargo.toml`.
- Recent CI runs tied to the active queue work still fail at parser/format stages after that manifest fix path, including runs `24986437008`, `24989037786`, and `25000254220`.
- The next safe engineering move is not more queue-runtime wiring or more planning-only PRs.

## Broken files currently blocking compile integrity

The following files were confirmed as parser- or delimiter-broken through CI failure context plus direct file inspection on `main`:
- `crates/pt-cli/src/main.rs`
- `crates/pt-coinbase/src/lib.rs`
- `crates/pt-core/src/config.rs`
- `crates/pt-dashboard/src/lib.rs`
- `crates/pt-dashboard/tests/api_contract.rs`
- `crates/pt-quote/src/lib.rs`
- `crates/pt-risk/src/lib.rs`

Secondary but not first-gate work still pending after compile integrity:
- `cargo audit` currently reports 14 vulnerabilities in the same failing CI window

## Last known clean merged recovery source

The last grounded merged state that re-opened clean source for the broken runtime/dashboard/risk surfaces was merge commit:
- `7da0bd8ba608f0f57e2edc83b7bf1f73cff955b1`
- PR `#31` `[codex] Add approval queue runtime store bridge`

That commit is the safest recovery baseline for the broken files above because it is:
- merged on `main`
- newer than the earlier Phase 1 queue storage work
- earlier than the currently observed syntax corruption on `main`

## Recommended next action

Open one code-bearing compile-integrity hotfix PR against `main` with this exact scope:
1. Fold in the manifest fix from PR `#40`.
2. Restore syntactic correctness in the seven broken Rust files listed above.
3. Prefer recovery from the last clean merged baseline (`7da0bd8ba608f0f57e2edc83b7bf1f73cff955b1`) over ad hoc line editing when the current file is clearly merge-corrupted.
4. Do not widen runtime behavior, risk policy, live authority, or approval surfaces as part of the hotfix.

## Acceptance criteria

The hotfix is only complete when all of the following are true:
- `cargo fmt --all` runs without parser errors
- `cargo check --workspace` passes
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes
- `cargo test --workspace` passes
- `cargo build --workspace` passes
- `cargo audit` is rerun and any remaining failures are reported separately from compile-integrity recovery
- queue-runtime work for issue `#9` is explicitly resumed only after the Phase 0 ladder is green

## Validation commands

```bash
cargo fmt --all
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo build --workspace
cargo audit
```

## Risks and guardrails

- Do not enable live mode.
- Do not add or modify credentials.
- Do not raise risk caps.
- Do not widen `/api/v1/orders` or `/api/v1/approval-queue` authority while Phase 0 is red.
- Do not treat docs-only queue activity as progress toward sandbox ROI while compile integrity is broken.
- Keep PR `#37` and issue `#9` queued behind this recovery step.

## Sequencing after this recovers

1. Land the compile-integrity hotfix.
2. Re-run the full local-first ladder.
3. Address `cargo audit` findings in a separate focused PR if still needed.
4. Resume PR `#37` as the Phase 1 helper-stack base.
5. Then open the narrow `crates/pt-cli/src/coinbase.rs` runtime-wiring PR for hydration and reconciliation only.

## Codex-ready task prompt

Title:
Restore compile integrity from the last clean merged baseline and unblock Phase 0

Repository:
Xellos1010/Polymarket-Trader

Goal:
Repair syntax-corrupted Rust files on `main`, fold in the manifest fix from PR `#40`, and restore the local-first validation ladder through `cargo build --workspace`.

Context:
- `docs/PHASE0_COMPILE_INTEGRITY_RECOVERY_2026-04-27.md`
- PR `#40`
- PR `#37`
- issue `#9`
- last known clean merged recovery source: `7da0bd8ba608f0f57e2edc83b7bf1f73cff955b1`

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
2. Restore syntactic correctness in the seven broken Rust files.
3. Prefer recovery from the clean merged baseline commit above where current files are clearly merge-corrupted.
4. Keep scope to compile-integrity recovery only.
5. Run the validation ladder through `cargo build --workspace`.
6. Report `cargo audit` separately if it still fails after compile recovery.

Validation:
- `cargo fmt --all`
- `cargo check --workspace`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`
- `cargo build --workspace`
- `cargo audit`

Definition of done:
- parser and delimiter failures are gone
- the workspace builds again
- Phase 1 queue-runtime work is unblocked but not bundled into this PR

Safety:
- Do not commit secrets.
- Do not enable live mode.
- Do not raise risk caps.
- Do not deploy without approval.
- Keep changes small and reviewable.
