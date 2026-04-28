# Session Context

Generated at UNIX epoch seconds: `1777348800`

## Note
Control-tower checkpoint after re-auditing the merged consolidation path and current blocker queue on April 28, 2026.

## Current phase
Phase 0: repo readiness

## Grounded repo state
- PR `#47` is already merged and remains the effective landing point for the meaningful earlier queue.
- PR `#49` is already merged and should no longer be treated as an active draft.
- There is currently no open implementation PR.
- Issue `#48` is the correct next code-bearing slice.
- Issue `#9` remains paused until compile integrity is recovered.
- Older PRs `#37`, `#40`, `#41`, `#44`, `#45`, and `#46` are historical context, not active delivery tracks.

## Canonical status file
- `docs/WORK_STATUS.md`

## Recommended next implementation slice
Recover compile integrity for issue `#48` with one narrow code-bearing PR limited to:
- `crates/pt-cli/src/main.rs`
- `crates/pt-coinbase/src/lib.rs`

Required scope:
- repair import, header, and command-boundary corruption only
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
- Do not treat tracker docs as evidence that Phase 0 gates have passed.

## Operator decision needed
No approval is needed to start issue `#48`.
Explicit approval is still required for merge, deployment, live mode, live credentials, or a tiny live pilot.