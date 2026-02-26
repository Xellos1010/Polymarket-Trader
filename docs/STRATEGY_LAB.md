# Coinbase Strategy Lab

Local-first visualization + backtest tooling for Coinbase markets.

## Files

- Tool: `tools/coinbase_strategy_lab.py`
- Config example: `config/coinbase_strategy_lab.example.json`
- Config schema: `schemas/coinbase_strategy_lab.schema.json`
- Output folder: `data/strategy_lab/`

## Setup

```bash
cp config/coinbase_strategy_lab.example.json config/coinbase_strategy_lab.json
```

## Modes

### 1) Backtest

```bash
python3 tools/coinbase_strategy_lab.py backtest --config config/coinbase_strategy_lab.json
```

### 2) Listing-theory overlap (candle aligned)

```bash
python3 tools/coinbase_strategy_lab.py overlap --config config/coinbase_strategy_lab.json
```

The overlap chart aligns each asset series at its anchor candle (`anchor_time`) and compares forward candle movement (`+1`, `+3`, `+10`) independent of wall-clock timestamps.

### 3) Parameter optimization

```bash
python3 tools/coinbase_strategy_lab.py optimize --config config/coinbase_strategy_lab.json
```

This runs SMA parameter grid search and ranks parameter pairs by:

`score = avg_return - drawdown_penalty*avg_drawdown - turnover_penalty*trade_rate`

### 4) Unified dashboard

```bash
python3 tools/coinbase_strategy_lab.py dashboard --config config/coinbase_strategy_lab.json
```

Outputs a single HTML with:
- current market prices (from latest candle)
- strategy price/SMA/equity view
- listing overlap chart
- optimization leaderboard
- fetch errors

Serve locally:

```bash
python3 tools/coinbase_strategy_lab.py dashboard --config config/coinbase_strategy_lab.json --serve 9090
```

## Fast Strategy Edits (CLI overrides)

```bash
python3 tools/coinbase_strategy_lab.py backtest \
  --config config/coinbase_strategy_lab.json \
  --markets BTC-USD,ETH-USD,SOL-USD,XRP-USD \
  --short-window 7 \
  --long-window 34 \
  --fee-bps 6 \
  --slippage-bps 2 \
  --limit 500
```

## Pine + AI fine-tuning integration

Use this lab for quick market-level validation and use the Pine pipeline for strategy-parameter iteration:

1. Extract Pine parameters: `pt-cli pine-params`
2. Generate/tune candidates: `pt-cli tune-pine`
3. Score with evaluator: `tools/evaluate_candidate.py`
4. Promote candidate: `scripts/promote_candidate.sh`

Reference: `docs/PINE_TUNING.md`
