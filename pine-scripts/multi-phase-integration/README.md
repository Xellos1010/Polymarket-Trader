# Multi-phase Pine integration (CTAP)

This folder holds **phased** variants of [`../multi-indicator-daterange.pine`](../multi-indicator-daterange.pine) so the main script stays stable while we migrate behavior.

## Phase 1 (`phase1-multi-indicator-weighted.pine`)

- Removes **Entry setup requirement** and **Entry confirmation** dropdowns.
- Adds per-indicator **Visual** vs **Backtesting** toggles (only applies when that indicator’s master enable is on).
- Adds per-indicator **required** + **entry/exit weight** (0–100%) for backtesting.
- **Evaluation order** (fixed in code for phase 1: RSI → Trend MA / Regime → Fibonacci BB → Ichimoku). A placeholder **Evaluation order** input documents intent for a later phase where order becomes user-configurable and drives UI ordering.
- **100% entry weight**: the **first** indicator in evaluation order that is on for backtest and has entry weight ≥ 99.99 is treated as the **sole** source of truth for the entry signal (other indicators do not vote on that side). Evaluation order therefore matters when multiple indicators could be set to 100%.
- Default activation: **RSI on**; **Fibonacci BB, Trend / Regime, Ichimoku off** (same as prior defaults for non-RSI).
- RSI defaults: source **close**, length **9**, oversold **31**, overbought **67.5**; trade direction default **Long only**.

## Pine limitation (inactive indicator settings)

TradingView **does not** allow hiding or disabling input fields based on another input. Inactive indicator sections still appear in the settings dialog. The script applies logic only when the indicator is **enabled**; labels and tooltips call out inactive groups.

## Syncing with Rust / strategy IR

When porting to the engine, mirror the same concepts: `visual`, `backtest`, `entry_required`, `exit_required`, `entry_weight`, `exit_weight`, and an explicit **evaluation order** list.
