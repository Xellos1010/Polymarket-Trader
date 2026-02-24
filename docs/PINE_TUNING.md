# Pine Strategy Tuning Workflow

The engine can consume TradingView bias signals today.  
This workflow adds parameter extraction and candidate generation so you can iterate with an external evaluator.

## 1) Extract Parameters From Pine

```bash
cargo run -p pt-cli -- pine-params \
  --path pine-scripts/V1-RSI-Fibonacci-BB-Ichimoku-Pyramid-Date-Range-Enabled-v6 \
  --out data/tuning/pine_params.json
```

Optional: fetch real OHLCV for scoring:

```bash
python3 tools/fetch_ohlcv.py --provider coinbase --symbol BTCUSD --interval 1m --limit 300 --out data/ohlcv/btcusd_1m.csv
```

If your region blocks a provider, try:

```bash
python3 tools/fetch_ohlcv.py --provider kraken --symbol BTCUSD --interval 1m --limit 300 --out data/ohlcv/btcusd_1m.csv
```

## 2) Generate Candidate Parameter Sets

Without scoring:

```bash
cargo run -p pt-cli -- tune-pine \
  --path pine-scripts/V1-RSI-Fibonacci-BB-Ichimoku-Pyramid-Date-Range-Enabled-v6 \
  --iterations 200 \
  --top-k 20 \
  --out data/tuning/pine_tuning_results.json
```

## 3) Score Candidates With External Evaluator

Pass an evaluator command that reads:
- `PT_PINE_SCRIPT` (script path)
- `PT_PINE_CANDIDATE_JSON` (JSON map of candidate params)

Evaluator must print one numeric score on stdout (last non-empty line).

Example:

```bash
cargo run -p pt-cli -- tune-pine \
  --path pine-scripts/V1-RSI-Fibonacci-BB-Ichimoku-Pyramid-Date-Range-Enabled-v6 \
  --iterations 300 \
  --top-k 25 \
  --evaluate-cmd "python3 tools/evaluate_candidate.py" \
  --out data/tuning/pine_tuning_results.json
```

Bundled evaluator options:

```bash
PT_EVAL_OHLCV=data/ohlcv/btc_1m.csv \
cargo run -p pt-cli -- tune-pine \
  --path pine-scripts/V1-RSI-Fibonacci-BB-Ichimoku-Pyramid-Date-Range-Enabled-v6 \
  --iterations 200 \
  --top-k 20 \
  --evaluate-cmd "python3 tools/evaluate_candidate.py --fee-bps 2.0 --slippage-bps 1.0 --fixed-trade-cost 0.00005 --price-col close --timestamp-col ts"
```

If no OHLCV path is provided, `tools/evaluate_candidate.py` runs a deterministic synthetic-data evaluation so tuning still works end-to-end.
For realistic ranking, provide OHLCV and cost terms (`fee-bps`, `slippage-bps`, `fixed-trade-cost`) so candidates are penalized for over-trading and friction.

## 4) AI Fine-Tuning Loop

Use an LLM agent or script to:
1. Propose evaluator objective (PnL, drawdown, Sharpe, turnover penalty).
2. Run `tune-pine` with evaluator.
3. Analyze top candidates.
4. Promote best candidate into TradingView alerts and live paper mode.

Recommended objective shape:
- `score = net_pnl - 0.5*max_drawdown - cost_penalty - overtrade_penalty`

## 5) Promote Candidate to Verification Queue

```bash
./scripts/promote_candidate.sh \
  data/tuning/pine_tuning_results.json \
  data/tuning/promoted_candidate.json \
  BTC 15m
```

Promotion output contains:
- selected candidate params and score
- verification command (`paper_soak.sh`) that must pass before live consideration

## 6) Wire Signal Indications Into Engine

TradingView webhook listener:
- `POST /tradingview` (configured in `signals.tradingview.bind_addr`)
- Payload is parsed by `pt-signal`
- Combined with wallet bias:
  - `combined_bias = clamp(k_wallet*wallet_bias + k_tv*tv_bias, -1, 1)`

Run in `paper` mode first, then tiny `live`.
