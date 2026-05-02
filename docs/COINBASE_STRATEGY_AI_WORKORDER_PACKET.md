# Coinbase Strategy AI Workorder Packet

## Purpose
This packet closes the planning gap after the Coinbase visual workstation foundation and prepares the next implementation sequence for agent review.

The guiding roadmap is the Coinbase visual workstation plus strategy artifact, strategy IR, optimizer, review surface, and benchmark sequence. The roadmap’s key success criteria are that the visual workstation renders chart, scanner, and strategy context; strategy artifacts become first-class; optimizer candidates are ranked and reproducible; replay and paper evidence point back to exact artifact ids; and risk controls remain authoritative.

## Current closed work

### Track A: Phase 0 unblock
- Issue: `#48`
- State: completed
- Evidence: operator reported compile integrity recovery landed in commit `7dfb650`; `cargo fmt --all -- --check` and `cargo check --workspace` passed.

### Track B: Coinbase visual workstation foundation
- Contract issue: `#54`
- Chart issue: `#55`
- Strategy rail issue: `#56`
- PR: `#57`
- State: completed
- Merge commit: `0a4af223cf7a342f17d7dbdd9982d406d629657a`
- Follow-up Vitest fix commit: `95d913d6fc10a8e9e9dcb541b218499b95dc5a68`
- Operator-reported validation:
  - `cargo check --workspace` passed
  - `cargo test --workspace` passed
  - `cd crates/pt-dashboard/frontend && npm run build` passed
  - `cd crates/pt-dashboard/frontend && npm test` passed after the Vitest fix

## Active implementation lane

### Issue `#53`: Strengthen strategy-lab handoff into Coinbase paper runtime

Goal: move from imported strategy visibility and shallow bias influence to a traceable strategy artifact contract that can be followed through replay and Coinbase paper runtime.

Required outputs:
1. Strategy artifact schema or hardened Rust/Python model
2. Strategy-lab output import path that preserves artifact id, source run id, product mapping, variant id, parameters, scoring, sizing hints, timeframe, provenance, promotion status, and replay acceptance status
3. Active artifact exposure through dashboard/API surfaces
4. Paper runtime evidence that can point back to an exact artifact id
5. Replay evidence that can point back to the same exact artifact id

Likely starting files:
- `crates/pt-cli/src/coinbase.rs`
- `crates/pt-dashboard/src/lib.rs`
- `tools/coinbase_strategy_lab.py`
- `tools/promote_strategy_lab.py`
- `docs/COINBASE_SANDBOX_ROI_FLOW.md`

Acceptance criteria:
- imported or promoted strategy artifacts can be traced to paper runtime behavior
- operator surfaces show which strategy artifact is active
- paper ROI evidence can point back to the exact strategy-lab source artifact
- replay evidence can point back to the same artifact id
- sandbox and paper only; no live-mode changes

Validation expectations:
```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
python3 tools/coinbase_strategy_lab.py backtest --config config/coinbase_strategy_lab.json
python3 tools/coinbase_strategy_lab.py optimize --config config/coinbase_strategy_lab.json
```

If runtime or replay wiring changes, also run the applicable smoke/acceptance command from the Coinbase sandbox ROI flow.

## Queued implementation lanes

### Issue `#58`: Define Rust-native strategy IR and adapter layer for strategy AI
Source work orders: Track D1 and D2.

Deliver a versioned, serializable strategy IR and adapter layer that represents existing strategy-lab strategies without cloning Pine Script syntax.

Key requirements:
- price/volume inputs
- rolling windows
- indicator nodes
- comparisons and composition
- entry and exit rules
- stop, take-profit, trailing, and sizing hints
- alert and webhook outputs
- projection into chart overlays and runtime decision outputs

### Issue `#59`: Implement bounded AI optimizer objective and candidate sweep lane
Source work orders: Track E1 and E2.

Deliver a first optimizer that can rank reproducible strategy candidates offline.

Core objective components:
- net PnL after modeled fees and slippage
- drawdown penalty
- turnover penalty
- stability across windows
- replay acceptance compatibility
- risk-limit violations as hard fails

Candidate families:
- parameter sweeps
- regime filters
- feature toggles
- sizing-policy variants

### Issue `#60`: Add strategy AI review surfaces for candidate ranking and promotion evidence
Source work order: Track E3.

Deliver read-first operator surfaces for candidate ranking, objective breakdown, rejection reasons, promotion state, and artifact lineage.

### Issue `#61`: Add visual and strategy benchmark harnesses for Coinbase workstation
Source work orders: Track F1 and F2.

Deliver repeatable benchmark harnesses for visual workstation performance and strategy search throughput.

Frontend benchmark dimensions:
- first render time
- candle update latency
- overlay redraw latency
- memory footprint
- pane sync responsiveness
- 1-product vs 8-product mode

Strategy benchmark dimensions:
- candidates per minute
- bars processed per second
- memory use
- replay throughput
- artifact generation time

## Execution rules for the next agent
1. Start with `#53` before optimizer widening.
2. Keep all work sandbox and paper only.
3. Do not enable live mode.
4. Do not add or modify credentials.
5. Do not raise risk caps.
6. Do not treat chart-derived fixture bars as replay, backtest, or paper evidence.
7. Do not let AI directly mutate live or paper positions outside bounded policy controls.
8. Do not build a Pine clone before the artifact and IR lanes are stable.

## PR handoff checklist
Before opening or marking the next implementation PR ready:
- link the PR to the issue it intends to close
- state which validation commands were run and where
- distinguish local command evidence from operator-reported evidence
- call out any fixture-backed data separately from current API-backed data
- explicitly state that no live-mode authority changed

## Recommended next PR title
`Track C: strengthen Coinbase strategy artifact handoff`
