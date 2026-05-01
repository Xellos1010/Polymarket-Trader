# Session Context

Generated at UNIX epoch seconds: `1777454150`

## Note
Control-tower checkpoint after a connector-backed audit of the active integration PR, current branch status files, open issue queue, and the current Slice 1 Rust files on April 29, 2026.

## Current phase
Phase 0: repo readiness

## Grounded repo state
- PR `#47`, PR `#49`, and PR `#50` are already merged into `main`.
- PR `#51` is still the only active integration PR, so the single-PR consolidation rule is currently satisfied.
- Issue `#48` is still the correct next code-bearing slice.
- Issue `#9` remains paused until compile integrity is recovered.
- The visible `codex/` branch inventory currently contains 44 branches and the control files now classify the full set.
- Diverged remote branches must be treated as salvage or archive context rather than active review targets.

## Validation evidence state
- The Phase 0 validation ladder has not been rerun from a full private-repo checkout in this environment.
- The current tracker set is a truthful queue and blocker description, not evidence that compile recovery is already complete.
- Any readiness upgrade still requires fresh local validation on the active integration branch.

## Canonical status files
- `docs/WORK_STATUS.md`
- `docs/WORK_STATUS.json`
- `docs/INTEGRATION_BOARD.md`

## Canonical integration branch
- `codex/single-integration-board-2026-04-28`

## Execution policy for this cycle
- keep one active integration PR
- take the next code-bearing slice from the ordered queue and land it on PR `#51`
- if a later stage requires human approval, keep the same PR and move to the next eligible queued feature
- if a run is blocked only by missing authenticated checkout or another audit-environment limitation, keep the same active slice and resume it from a proper checkout
- when no more eligible queued features remain, finish with one refinement pass for consistency and best practices

## Branch-classification policy
Treat every visible `codex/` branch as exactly one of:
- active integration branch
- compile-recovery reference
- frontend or dashboard salvage reference
- approval-queue persistence reference
- planning or audit archive context

Current rule:
- only `codex/single-integration-board-2026-04-28` is active
- all other visible `codex/` branches are reference-only until the current phase gates allow targeted salvage
- the canonical classification source is `docs/INTEGRATION_BOARD.md`

## Recommended next implementation slice
Recover compile integrity for issue `#48` on PR `#51`, limited to:
- `crates/pt-cli/src/main.rs`
- `crates/pt-coinbase/src/lib.rs`

Required scope:
- repair import, header, and command-boundary corruption only
- preserve coherent newer behavior where it is already intact
- do not add queue-runtime behavior in this slice
- do not change live-mode behavior, credentials, deployment posture, or risk caps

## Exact Slice 1 blocker signatures
### `crates/pt-cli/src/main.rs`
- duplicate `pt_core` imports
- duplicate `pt_engine::TradingEngine` imports
- a broken `Commands` enum boundary where `StrategyProfileLoad` runs into `Coinbase`

Immediate safe repair:
1. merge the `pt_core` imports so one block carries `AppConfig`, `EngineMode`, `ReplayAcceptanceReport`, `RuntimeRole`, and `MarketSnapshot`
2. keep only one `pt_engine::TradingEngine` import
3. close `StrategyProfileLoad` cleanly and keep `Coinbase` as its own subcommand variant
4. preserve the newer command surface already present

### `crates/pt-coinbase/src/lib.rs`
- merge-corrupted top-level import and header block
- duplicated `pt_core` fragments
- duplicated `reqwest::header` fragments

Immediate safe repair:
1. keep the newer auth-manager, websocket, and runtime imports intact
2. merge the older advanced-trade imports into that same top block without duplicates
3. remove the duplicated header fragments without changing behavior below the import section

## Environment blocker for the code-bearing slice
- This audit run has GitHub connector access for repository and PR analysis.
- This audit run still does not provide a usable authenticated local checkout of the private repository.
- Direct git clone from GitHub was not available in this environment.
- This is an execution-environment blocker, not a human-decision gate, so issue `#48` must remain the `active_now` slice.
- The next code-bearing pass should therefore happen from a proper checkout of branch `codex/single-integration-board-2026-04-28`, and it should stay limited to issue `#48`.

## Acceptance criteria
- `cargo fmt --all -- --check` no longer reports parser errors for the two Slice 1 files
- the recovery work stays limited to those two files
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
- Do not mix issue `#9` queue-runtime wiring into the Slice 1 compile-recovery work.
- Do not treat diverged remote branches as current readiness evidence.
- Do not open more than one active integration PR for this queue.

## Operator decision needed
No approval is needed to continue PR `#51` or to start issue `#48`.
Explicit approval is still required for merge, deployment, live mode, live credentials, or a tiny live pilot.
