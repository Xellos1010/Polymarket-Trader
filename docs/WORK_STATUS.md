# Work Status

## Phase
Phase 0: repo readiness

## Current audit finding
The repository is still blocked in Phase 0 and there is no active open PR right now.

Grounded state as of April 28, 2026:
- PR `#47` merged and already consolidated the meaningful earlier queue.
- PR `#49` merged and advanced the queue-start handoff.
- Issue `#48` is the current next code-bearing slice.
- Issue `#9` remains paused until compile integrity is recovered.
- Older PRs `#37`, `#40`, `#41`, `#44`, `#45`, and `#46` are historical context, not active delivery tracks.

## Current stage
Stage: `phase0_slice1_ready`

Meaning:
- the repo has one truthful next engineering slice
- no approval is needed to start that slice
- no other feature should outrun compile recovery first

## Active next step
Open one narrow PR from `main` for issue `#48` with this exact scope:
1. repair `crates/pt-cli/src/main.rs`
2. repair `crates/pt-coinbase/src/lib.rs`
3. keep changes syntax and structure only
4. avoid queue-runtime behavior in this slice

## Ordered queue after the current stage
1. Issue `#48`: compile recovery slice 1 (`pt-cli` + `pt-coinbase`)
2. Phase 0 slice 2: recover `crates/pt-core/src/config.rs`
3. Phase 0 slice 3: recover remaining parser-blocked dashboard and runtime files
4. Phase 0 validation ladder: fmt, check, clippy, test, build, audit, SBOM
5. Only then resume Phase 1 issue `#9`
6. After Phase 0 is green again, continue the Phase 1 queue in this order:
   - issue `#23` deterministic risk and quote failure-path tests
   - issue `#22` dashboard safety-net test consolidation
   - issue `#10` repeatable replay and paper evidence bundle

## Acceptance criteria for the current stage
- one small PR exists for issue `#48`
- `cargo fmt --all -- --check` no longer reports parser errors for:
  - `crates/pt-cli/src/main.rs`
  - `crates/pt-coinbase/src/lib.rs`
- issue `#9` remains paused until later recovery slices and the validation ladder pass

## Validation commands
```bash
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo build --workspace
cargo audit
./scripts/generate_sbom.sh artifacts
```

## Human decision gates
No human decision is needed to start issue `#48`.

Human approval is still required before:
- merge
- deployment
- live mode
- live credentials
- risk-cap increases
- a tiny live pilot

## Risks and guardrails
- Do not enable live mode.
- Do not add or modify credentials.
- Do not raise risk caps.
- Do not mix queue-runtime wiring into the Phase 0 recovery slice.
- Do not treat stale tracker text as evidence of current readiness.

## Status ownership
This file is the canonical work-stage tracker.
Update it whenever the active stage, blocker, or next slice changes.