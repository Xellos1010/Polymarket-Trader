# Integration Board

## Purpose
This file is the branch-consolidation board for the current cycle.

Use it to keep one active integration PR, one truthful current stage, and one ordered salvage queue for remote branches that still contain unmerged work.

## Active integration target
- branch: `codex/single-integration-board-2026-04-28`
- PR: `#51`
- phase: Phase 0: repo readiness
- current blocker: issue `#48` compile recovery slice 1 for `crates/pt-cli/src/main.rs` and `crates/pt-coinbase/src/lib.rs`

## Integration rule
- Do not open multiple parallel integration PRs.
- Do not merge stale diverged branches directly.
- Replay surviving payload from stale branches onto current `main` only after the Phase 0 validation ladder is green again.
- If the active stage hits a human-decision gate, keep the same PR open and move to the next eligible queued feature that does not depend on that decision.
- When no more eligible queued features remain, stop adding scope and run one best-practices refinement pass across the touched planning and status files.

## Current execution order
1. issue `#48` compile recovery slice 1
2. Phase 0 compile recovery slice 2 (`pt-core/src/config.rs`)
3. Phase 0 compile recovery slice 3 (remaining parser-blocked dashboard/runtime files)
4. Phase 0 validation ladder
5. issue `#23` deterministic risk and quote tests
6. issue `#22` dashboard safety-net and read-only queue test consolidation
7. issue `#10` repeatable replay and paper evidence bundle refresh
8. issue `#9` durable approval-queue persistence
9. dashboard shell/UI salvage from `codex/dashboard-shell-current-api`

## Deferred branch inventory
| Branch | Payload seen in compare | Current disposition |
|---|---|---|
| `codex/approval-queue-frontend-panel` | approval-queue frontend panel, `App.tsx`, `App.test.tsx`, progress note | defer until Phase 0 is green and current API-backed frontend work is re-audited |
| `codex/read-only-approval-queue-api` | queue API-related files, frontend updates, persistence brief | salvage later as fresh small PRs after compile recovery |
| `codex/approval-queue-storage-foundation` | queue storage foundation files | reference only until issue `#48` and later recovery slices pass |
| `codex/approval-queue-snapshot-reconcile` | queue snapshot/reconcile helper files | reference only until Phase 0 is green |
| `codex/approval-queue-runtime-store-bridge` | queue runtime store bridge files | reference only until Phase 0 is green |
| `codex/approval-queue-runtime-hydration-helpers` | queue hydration/runtime helper files | reference only until Phase 0 is green |
| `codex/dashboard-shell-current-api` | dashboard shell expansion, tests, product plan doc | defer until current API-backed frontend work is re-audited on restored build |

## Decision-gate policy
- Default rule: keep advancing the queue while the next item is well-defined and does not require a blocked human decision.
- Pause only when the next remaining items all require human approval or depend on unresolved validation evidence.
- Record the blocker and the fallback next feature in `docs/WORK_STATUS.md` and `docs/WORK_STATUS.json` whenever this happens.

## Validation gate before any salvage work
```bash
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo build --workspace
cargo audit
./scripts/generate_sbom.sh artifacts
```

## Guardrails
- Do not enable live mode.
- Do not add or modify credentials.
- Do not raise risk caps.
- Do not bypass issue `#48` while Phase 0 remains red.
- Do not promote Phase 1 or Phase 2 work ahead of repo readiness.
- Do not treat tracker updates as a substitute for validation evidence.
- Do not let the Markdown and JSON status mirrors drift.
