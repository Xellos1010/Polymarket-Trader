# Sandbox ROI Roadmap

## Goal

The first goal is a Coinbase-first sandbox trading environment where an agent can:

- optimize strategy parameters every hour
- backtest and replay-validate candidates
- promote winners into paper mode only after passing gates
- use operator-visible evidence to make bounded decisions

This roadmap is for sandbox ROI and operator readiness, not live trading.

## Already done

- Rust workspace and crate structure exist.
- Coinbase workstation startup exists for `paper` and `replay`.
- Strategy lab supports backtest, overlap, optimize, dashboard, and export.
- Replay promotion tooling exists.
- Replay acceptance tooling exists.
- SQLite strategy-lab trade journal exists.
- Dashboard workspaces exist for command, listing, risk, strategy, and agent views.
- Risk controls, kill switch, and paper soak tooling exist.
- Risk and Agent summary endpoints exist.
- Sandbox optimization cycle tooling now exists to run optimize -> backtest -> promote -> replay acceptance -> incumbent update.

## Partially done

- Coinbase ingest and replayable market-data support are still partial.
- Strategy system is usable for research but not yet a full promote/retire registry.
- Maker/taker execution policy is incomplete.
- Full route-level PnL attribution is incomplete.
- Dashboard operator views are still summary-heavy.
- Full Rust/frontend validation is still not confirmed in a real checkout.

## Needed next

### 1. Validation baseline

- pass `cargo fmt`, `check`, `clippy`, `test`, and `build`
- confirm strategy-lab, replay promotion, replay acceptance, and paper soak work end to end

### 2. Operator evidence loop

- show current incumbent strategy
- show latest candidate and replay result
- show promotion and rejection reasons
- expose policy-event timeline and approval queue clearly in the dashboard

### 3. Hourly sandbox loop

- schedule the sandbox optimization cycle every hour
- keep all promotions sandbox-only
- never widen risk caps automatically

### 4. Agent decision layer

- autonomy tiers
- decision log with evidence links
- approval-required actions for risky changes
- parameter recommendations bounded by policy

### 5. Sandbox ROI gate

Only treat the system as successful when:

- net paper or replay PnL is positive after modeled costs
- no risk controls are breached
- results repeat across multiple independent runs
- attribution is clear enough to explain why it worked

## Current recommended build order

1. Finish local validation.
2. Put the sandbox optimization cycle on a schedule.
3. Expose incumbent/candidate/promotion evidence in operator surfaces.
4. Add bounded agent autonomy on top of that evidence.
5. Run repeatable paper trials before discussing live readiness.
