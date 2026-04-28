# Work Status

## Phase
Phase 0: repo readiness

## Current audit finding
The repository is still blocked in Phase 0 and there was no open implementation PR at the start of this review cycle.

Grounded state as of April 28, 2026:
- `main` already contains the earlier consolidation work from PR `#47`, PR `#49`, and PR `#50`.
- Issue `#48` is still the active next code-bearing slice.
- Issue `#9` remains paused until compile integrity is recovered.
- There are still multiple diverged remote branches carrying unmerged payload, but they are behind current `main` and must not be treated as parallel active PR tracks.
- This branch, `codex/single-integration-board-2026-04-28`, is now the single integration target for current audit and queue alignment.

## Current stage
Stage: `phase0_slice1_waiting_on_compile_recovery`

Meaning:
- the repo has one canonical integration branch
- the next safe code change is still issue `#48`
- no Phase 1 queue-runtime or frontend expansion should outrun compile recovery

## Single active integration branch
- branch: `codex/single-integration-board-2026-04-28`
- purpose: keep one truthful PR and one truthful status board while the repo works through the Phase 0 blocker queue
- status: ready to open as the only active review target

## Active next step
Open one narrow code-bearing follow-up on top of current `main` or this integration branch for issue `#48` with this exact scope:
1. repair `crates/pt-cli/src/main.rs`
2. repair `crates/pt-coinbase/src/lib.rs`
3. keep changes syntax and structure only
4. avoid queue-runtime behavior in this slice

## Outstanding remote work to consolidate after Phase 0 recovery
1. `codex/approval-queue-frontend-panel`
   - payload: approval-queue panel plus frontend tests
   - disposition: defer until Phase 0 is green and current API coverage is re-audited
2. `codex/read-only-approval-queue-api`
   - payload: read-only queue API, frontend updates, and persistence brief
   - disposition: treat as salvage source for refreshed Phase 1 work after issue `#48`
3. `codex/approval-queue-storage-foundation`
4. `codex/approval-queue-snapshot-reconcile`
5. `codex/approval-queue-runtime-store-bridge`
6. `codex/approval-queue-runtime-hydration-helpers`
   - payload: queue helper and persistence scaffolding
   - disposition: historical references only until compile recovery is complete and the surviving pieces are replayed onto current `main`
7. `codex/dashboard-shell-current-api`
   - payload: larger dashboard shell and test expansion
   - disposition: defer until current API-backed frontend work is re-audited against the restored build

## Ordered queue after the current stage
1. Issue `#48`: compile recovery slice 1 (`pt-cli` + `pt-coinbase`)
2. Phase 0 slice 2: recover `crates/pt-core/src/config.rs`
3. Phase 0 slice 3: recover remaining parser-blocked dashboard and runtime files
4. Phase 0 validation ladder: fmt, check, clippy, test, build, audit, SBOM
5. Only then resume Phase 1 issue `#9`
6. After Phase 0 is green again, re-open the deferred Phase 1 work as fresh small PRs in this order:
   - issue `#23` deterministic risk and quote failure-path tests
   - issue `#22` dashboard safety-net test consolidation
   - issue `#10` repeatable replay and paper evidence bundle
   - issue `#9` durable approval-queue persistence using the deferred branch payload only as reference

## Acceptance criteria for the current stage
- one draft PR exists for this integration branch
- `docs/WORK_STATUS.md`, `docs/SESSION_CONTEXT.md`, and `docs/PROGRESS.md` all agree on the active stage and next slice
- the repo no longer implies multiple active PR tracks
- issue `#48` remains the next code-bearing slice

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
No human decision is needed to open this integration-tracking PR.

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
- Do not treat stale diverged branches as merge-ready evidence.
- Do not resume Phase 1 work before compile integrity is restored.

## Status ownership
This file is the canonical work-stage tracker.
Update it whenever the active stage, blocker, integration branch, or next slice changes.