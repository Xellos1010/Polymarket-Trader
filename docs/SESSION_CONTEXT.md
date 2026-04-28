# Session Context

Generated at UNIX epoch seconds: `1777339800`

## Note
Control-tower checkpoint after re-auditing all previously completed work and delegating the next-round Phase 0 review on April 28, 2026.

## Current phase
Phase 0: repo readiness

## Grounded repo state
- PR `#47` is already merged and is the effective landing point for the meaningful earlier queue.
- PR `#49` is the active queue-start draft for Recovery Slice 1, but it is still docs-only.
- Issue `#48` is the correct next code slice.
- Issue `#9` remains paused until compile integrity is recovered.
- Older PRs `#37`, `#40`, `#41`, `#44`, `#45`, and `#46` should be treated as historical context, not active delivery tracks.

## Parallel delegated findings
- PR queue audit: the safest single next PR-sized item is still a code-bearing recovery branch for issue `#48`.
- Compile-recovery audit: the current blockers remain localized to:
  - `crates/pt-cli/src/main.rs`
  - `crates/pt-coinbase/src/lib.rs`
- GitHub-only operations audit: this environment can create branches, update files, and manage PRs, but local cargo validation is still not available here.

## Recommended next implementation slice
Recover compile integrity for issue `#48` with one narrow code-bearing PR limited to:
- `crates/pt-cli/src/main.rs`
- `crates/pt-coinbase/src/lib.rs`

Required scope:
- repair import/header duplication and command-boundary corruption only
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
- Do not treat queue-start docs as evidence that Phase 0 gates have passed.

## Operator decision needed
No approval is needed to keep the Phase 0 queue current.
Explicit approval is still required for merge, deployment, live mode, live credentials, or a tiny live pilot.
