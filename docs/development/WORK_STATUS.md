# Work Status

**Strategic direction** is defined in the repository root [`ROADMAP.md`](../../ROADMAP.md). This file tracks **current execution stage**, queue, and validation evidence—keep it aligned when phases in the roadmap advance.

## Phase
Phase 1: sandbox trading / paper ROI preparation

## Current audit finding
The adversarial roadmap queue has advanced through the remaining webhook resilience issue `#86` and epics `#69`, `#70`, and `#71`. AI foundation, AI management, and bounded AI improvement issues `#87`-`#101` are implemented with fresh local validation on May 17, 2026; issue `#102` is the next active queue head under epic `#72`.

## Audit stamp
- last audited on: May 17, 2026
- audit source: GitHub issue state, direct workspace validation, and continued agent-driven development after interrupted issue work
- validation evidence state: fresh May 17, 2026 validation for issues `#86` and `#93`-`#101`: `./scripts/webhook_resilience_drill.sh`, `cargo fmt --all -- --check`, `cargo test -p pt-ai-agent`, `cargo test -p pt-dashboard --test api_contract`, `cargo test --workspace`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings` all passed
- truth standard: status files may record operator validation evidence, and may record agent reruns when explicitly stated in this section

## Automation mirror
- [`docs/development/WORK_STATUS.json`](WORK_STATUS.json) is the machine-readable mirror of this file.
- Keep the Markdown and JSON trackers aligned whenever the active stage, blocker, queue state, validation evidence, or next work order changes.

## Current stage
Stage: `phase5_ai_discovery_queue`

Stage contract:
- keep work sandbox and paper only
- keep strategy artifacts, review surfaces, benchmark evidence, and backlog mapping versioned and local-first
- treat the chart workstation as a presentation surface, not replay or paper evidence
- keep AI recommendations behind artifact promotion, replay validation, paper evidence, and risk controls

## Workflow invariants
- one active implementation lane at a time
- one active PR per lane
- issues `#59`, `#60`, `#61`, and `#10` are complete and recorded in local evidence
- issue `#102` is the active AI discovery lane and epic `#72` is the parent tracker for the next queue
- strategy definitions, chart presentation, optimizer candidates, and paper runtime evidence remain separate versioned objects
- frontend chart state must not become the source of truth for strategy math
- no live-mode expansion without explicit approval

## Current active next step
Continue with issue `#102` under epic `#72`, then work downward through the filed roadmap backlog in [`ROADMAP_ISSUE_BACKLOG_2026-05-17.md`](ROADMAP_ISSUE_BACKLOG_2026-05-17.md).

Current queue head:
1. `#102` compositional indicator synthesis framework
2. `#103` supervised pattern-discovery framework
3. `#104` sentiment integration framework

## Queue summary
- completed Track A/Phase 0 unblock: issue `#48`
- completed Track B: issues `#54`, `#55`, `#56`, PR `#57`
- completed strategy artifact handoff: issue `#53`
- completed Rust-native IR lane: issue `#58`
- active now: issue `#102`
- queued after current stage: issues `#72`-`#111` per the filed roadmap backlog
- blocked on human decision right now: 0 items

## Consolidated feature queue
| Order | Feature or slice | Source | Queue state | Human decision required | Next action |
|---|---|---|---|---|---|
| 1 | Compile recovery slice 1 (`pt-cli` + `pt-coinbase`) | issue `#48` | completed | no | closed after compile recovery evidence |
| 2 | Coinbase visual workstation contract | issue `#54`, PR `#57` | completed | no | closed by merged PR `#57` |
| 3 | Chart-first Coinbase product detail surface | issue `#55`, PR `#57` | completed | no | closed with evidence comment |
| 4 | Strategy review rail with import lineage | issue `#56`, PR `#57` | completed | no | closed with evidence comment |
| 5 | Strategy-lab handoff into Coinbase paper runtime | issue `#53` | completed | no | closed after artifact-lineage smoke and paper-runtime traceability evidence |
| 6 | Rust-native strategy IR and adapter layer | issue `#58` | completed | no | closed after IR implementation, adapters, evaluator, and 17 new tests landed on `main` |
| 7 | Bounded AI optimizer objective and candidate sweep lane | issue `#59` | completed | no | objective schema, bounded sweep metadata, and optimizer-cycle decision reasons landed |
| 8 | Strategy AI review surfaces | issue `#60` | completed | no | `/api/v1/strategy-candidates` plus dashboard candidate review UI/tests landed |
| 9 | Visual and strategy benchmark harnesses | issue `#61` | completed | no | baseline benchmark artifacts captured under `artifacts/benchmarks/2026-05-17/` |
| 10 | Repeatable replay and paper evidence gate report | issue `#10` | completed | no | `data/evidence/phase1/2026-05-17/report.json` and `report.md` both show status `pass` |
| 11 | Critical correctness, Pine parity, webhook, and AI foundation/management/improvement | epics `#66`-`#71` | completed | no | closed with local validation evidence |
| 12 | AI discovery layer | epic `#72`, issue `#102` | active_now | no | implement bounded compositional indicator synthesis framework |
| 13 | Portfolio management backlog | epic `#73`, issues `#107`-`#111` | queued | no | start after AI discovery issues close |

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
No human decision is needed to begin issue `#65` or the filed backlog beneath epic `#66`.

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
This file is the operator-readable work-stage tracker. Update it with [`docs/development/WORK_STATUS.json`](WORK_STATUS.json) whenever active stage, queue state, validation evidence, issue closure, or PR state changes.
