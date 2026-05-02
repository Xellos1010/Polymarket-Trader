# Work Status

## Phase
Phase 1: sandbox trading / paper ROI preparation

## Current audit finding
Phase 0 compile recovery is no longer the active blocker. Issue `#48` was closed after compile integrity was recovered, and PR `#57` merged the Track B Coinbase visual workstation foundation.

Grounded state as of May 2, 2026:
- issue `#48` is closed as completed.
- PR `#57` is merged with merge commit `0a4af223cf7a342f17d7dbdd9982d406d629657a`.
- Track B work orders are closed:
  - `#54` Coinbase workstation visual contract
  - `#55` chart-first Coinbase product detail surface
  - `#56` strategy review rail with import lineage and paper-only guardrails
- Operator-reported validation for the merged workstation branch:
  - `cargo check --workspace` passed
  - `cargo test --workspace` passed
  - `cd crates/pt-dashboard/frontend && npm run build` passed
  - `cd crates/pt-dashboard/frontend && npm test` passed after commit `95d913d`
- The next active implementation lane is issue `#53`: strengthen strategy-lab handoff into Coinbase paper runtime.
- Tracks D-F have been split into reviewable follow-up issues:
  - `#58` Rust-native strategy IR and adapter layer
  - `#59` bounded AI optimizer objective and candidate sweep lane
  - `#60` strategy AI review surfaces
  - `#61` visual and strategy benchmark harnesses

## Audit stamp
- last audited on: May 2, 2026
- audit source: GitHub issue state, merged PR `#57`, operator-reported validation evidence, and the Coinbase strategy AI roadmap packet supplied in the current planning context
- validation evidence state: operator-reported local validation, not rerun from this environment
- truth standard: status files may record operator validation evidence, but must not imply this agent reran local commands unless explicitly stated

## Automation mirror
- `docs/WORK_STATUS.json` is the machine-readable mirror of this file.
- Keep the Markdown and JSON trackers aligned whenever the active stage, blocker, queue state, validation evidence, or next work order changes.

## Current stage
Stage: `phase1_strategy_artifact_handoff`

Stage contract:
- keep work sandbox and paper only
- strengthen strategy artifacts before widening AI optimization
- treat the chart workstation as a presentation surface, not replay or paper evidence
- keep AI recommendations behind artifact promotion, replay validation, paper evidence, and risk controls

## Workflow invariants
- one active implementation lane at a time
- one active PR per lane
- issue `#53` is the active downstream lane after Track B
- strategy definitions, chart presentation, optimizer candidates, and paper runtime evidence remain separate versioned objects
- frontend chart state must not become the source of truth for strategy math
- no live-mode expansion without explicit approval

## Current active next step
Continue with issue `#53`: strengthen strategy-lab handoff into Coinbase paper runtime.

Required work-order scope:
1. define or harden a strategy artifact schema carrying artifact id, source run id, product mapping, variant id, parameters, expected edge inputs, score/confidence, optional sizing hints, timeframe, provenance, promotion status, and replay acceptance status
2. wire imported/promoted artifacts into Coinbase paper runtime traceability
3. expose active artifact context through dashboard/API surfaces
4. attach replay and paper evidence to exact artifact ids
5. keep scope sandbox and paper only

## Queue summary
- completed Track A/Phase 0 unblock: issue `#48`
- completed Track B: issues `#54`, `#55`, `#56`, PR `#57`
- active now: issue `#53`
- queued after current stage: issues `#58`, `#59`, `#60`, `#61`
- blocked on human decision right now: 0 items

## Consolidated feature queue
| Order | Feature or slice | Source | Queue state | Human decision required | Next action |
|---|---|---|---|---|---|
| 1 | Compile recovery slice 1 (`pt-cli` + `pt-coinbase`) | issue `#48` | completed | no | closed after compile recovery evidence |
| 2 | Coinbase visual workstation contract | issue `#54`, PR `#57` | completed | no | closed by merged PR `#57` |
| 3 | Chart-first Coinbase product detail surface | issue `#55`, PR `#57` | completed | no | closed with evidence comment |
| 4 | Strategy review rail with import lineage | issue `#56`, PR `#57` | completed | no | closed with evidence comment |
| 5 | Strategy-lab handoff into Coinbase paper runtime | issue `#53` | active_now | no | start next PR from main |
| 6 | Rust-native strategy IR and adapter layer | issue `#58` | queued | no | begin after or alongside artifact contract once #53 schema boundaries are stable |
| 7 | Bounded AI optimizer objective and candidate sweep lane | issue `#59` | queued | no | begin after artifact schema supports optimizer outputs |
| 8 | Strategy AI review surfaces | issue `#60` | queued | no | begin after #59 produces candidate artifacts |
| 9 | Visual and strategy benchmark harnesses | issue `#61` | queued | no | begin once chart and optimizer candidate lanes are measurable |

## Validation commands
Repo readiness and implementation safety:
```bash
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo build --workspace
```

Frontend workstation:
```bash
cd crates/pt-dashboard/frontend
npm test
npm run build
```

Strategy and sandbox ROI flow:
```bash
python3 tools/coinbase_strategy_lab.py backtest --config config/coinbase_strategy_lab.json
python3 tools/coinbase_strategy_lab.py overlap --config config/coinbase_strategy_lab.json --auto-discovery
python3 tools/coinbase_strategy_lab.py optimize --config config/coinbase_strategy_lab.json
cargo run -p pt-cli -- run --config config/config.toml
./scripts/paper_soak.sh 86400 30 config/config.toml
```

## Human decision gates
No human decision is needed to start issue `#53` in sandbox/paper scope.

Human approval is still required before:
- merge
- deployment
- live mode
- live credentials
- risk-cap increases
- any live pilot

## Risks and guardrails
- Do not enable live mode.
- Do not add or modify credentials.
- Do not raise risk caps.
- Do not let strategy artifacts bypass replay or paper validation.
- Do not let AI directly mutate live or paper positions outside bounded policy controls.
- Do not treat chart-derived fixture bars as replay, backtest, or paper evidence.
- Do not build a Pine clone before the typed strategy artifact and IR lanes are stable.

## Status ownership
This file is the operator-readable work-stage tracker. Update it with `docs/WORK_STATUS.json` whenever active stage, queue state, validation evidence, issue closure, or PR state changes.
