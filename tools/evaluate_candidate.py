#!/usr/bin/env python3
"""Evaluate a Pine parameter candidate and print a numeric score.

Expected environment variables:
- PT_PINE_CANDIDATE_JSON: JSON object of candidate parameter overrides.
- PT_PINE_SCRIPT: Optional script path (only used for metadata/logging).

Optional data source:
- PT_EVAL_OHLCV: CSV path with at least a close price column.

CLI flags can override environment defaults:
  --ohlcv PATH
  --price-col close
  --timestamp-col ts
  --fee-bps 2.0

Output contract:
- Print one numeric score to stdout on the last non-empty line.
"""

from __future__ import annotations

import argparse
import csv
import json
import math
import os
import random
import sys
from dataclasses import dataclass
from typing import Any, Dict, Iterable, List, Optional, Sequence, Tuple


def clamp(value: float, lo: float, hi: float) -> float:
    return max(lo, min(hi, value))


def safe_float(v: Any) -> Optional[float]:
    if isinstance(v, bool):
        return 1.0 if v else 0.0
    if isinstance(v, (int, float)):
        return float(v)
    if isinstance(v, str):
        txt = v.strip().replace("_", "")
        if not txt:
            return None
        try:
            return float(txt)
        except ValueError:
            return None
    return None


def load_candidate() -> Dict[str, Any]:
    raw = os.getenv("PT_PINE_CANDIDATE_JSON", "").strip()
    if not raw:
        raise ValueError("PT_PINE_CANDIDATE_JSON is required")
    obj = json.loads(raw)
    if not isinstance(obj, dict):
        raise ValueError("PT_PINE_CANDIDATE_JSON must decode to a JSON object")
    return obj


def find_numeric(
    params: Dict[str, Any],
    patterns: Sequence[Sequence[str]],
    default: float,
    lo: float,
    hi: float,
) -> float:
    for key, value in params.items():
        lk = key.lower()
        for tokens in patterns:
            if all(token in lk for token in tokens):
                parsed = safe_float(value)
                if parsed is not None:
                    return clamp(parsed, lo, hi)
    return clamp(default, lo, hi)


def find_bool(params: Dict[str, Any], tokens: Sequence[str], default: bool) -> bool:
    for key, value in params.items():
        lk = key.lower()
        if all(token in lk for token in tokens):
            if isinstance(value, bool):
                return value
            if isinstance(value, str):
                txt = value.strip().lower()
                if txt == "true":
                    return True
                if txt == "false":
                    return False
    return default


@dataclass(frozen=True)
class EvalConfig:
    rsi_len: int
    fast_len: int
    slow_len: int
    bb_len: int
    bb_mult: float
    threshold: float
    fee_bps: float
    slippage_bps: float
    fixed_trade_cost: float
    enable_rsi: bool
    enable_ma: bool
    enable_bb: bool


@dataclass(frozen=True)
class EvalStats:
    score: float
    total_return: float
    max_drawdown: float
    sharpe_like: float
    turnover: float
    bars: int


def config_from_params(
    params: Dict[str, Any], fee_bps: float, slippage_bps: float, fixed_trade_cost: float
) -> EvalConfig:
    rsi_len = int(
        round(
            find_numeric(
                params,
                patterns=[("rsi", "len"), ("rsi", "period"), ("rsi", "length")],
                default=14.0,
                lo=2.0,
                hi=200.0,
            )
        )
    )

    fast_len = int(
        round(
            find_numeric(
                params,
                patterns=[("fast", "len"), ("short", "len"), ("ma", "fast")],
                default=9.0,
                lo=2.0,
                hi=250.0,
            )
        )
    )

    slow_len = int(
        round(
            find_numeric(
                params,
                patterns=[("slow", "len"), ("long", "len"), ("ma", "slow")],
                default=21.0,
                lo=3.0,
                hi=400.0,
            )
        )
    )

    if slow_len <= fast_len:
        slow_len = min(400, fast_len + 2)

    bb_len = int(
        round(
            find_numeric(
                params,
                patterns=[("bb", "len"), ("boll", "len"), ("bb", "period")],
                default=20.0,
                lo=5.0,
                hi=300.0,
            )
        )
    )

    bb_mult = find_numeric(
        params,
        patterns=[("bb", "mult"), ("boll", "std"), ("bb", "std")],
        default=2.0,
        lo=0.5,
        hi=6.0,
    )

    threshold = find_numeric(
        params,
        patterns=[("threshold",), ("entry", "th"), ("signal", "th")],
        default=0.15,
        lo=0.01,
        hi=1.0,
    )
    if threshold > 1.0:
        threshold = threshold / 100.0

    enable_rsi = find_bool(params, ("enable", "rsi"), True)
    enable_ma = find_bool(params, ("enable", "ma"), True)
    enable_bb = find_bool(params, ("enable", "bb"), True)

    return EvalConfig(
        rsi_len=rsi_len,
        fast_len=fast_len,
        slow_len=slow_len,
        bb_len=bb_len,
        bb_mult=bb_mult,
        threshold=threshold,
        fee_bps=max(0.0, fee_bps),
        slippage_bps=max(0.0, slippage_bps),
        fixed_trade_cost=max(0.0, fixed_trade_cost),
        enable_rsi=enable_rsi,
        enable_ma=enable_ma,
        enable_bb=enable_bb,
    )


def load_prices_from_csv(
    path: str, price_col: str, timestamp_col: Optional[str]
) -> List[float]:
    with open(path, "r", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        if reader.fieldnames is None:
            raise ValueError(f"CSV has no headers: {path}")

        fields = {field.lower(): field for field in reader.fieldnames}

        possible_price_fields = [
            price_col,
            price_col.lower(),
            "close",
            "c",
            "price",
            "last",
        ]

        chosen_price_field = None
        for p in possible_price_fields:
            if p in reader.fieldnames:
                chosen_price_field = p
                break
            if p.lower() in fields:
                chosen_price_field = fields[p.lower()]
                break

        if chosen_price_field is None:
            raise ValueError(
                f"CSV missing price column; tried {possible_price_fields} in {reader.fieldnames}"
            )

        ts_field = None
        if timestamp_col:
            if timestamp_col in reader.fieldnames:
                ts_field = timestamp_col
            elif timestamp_col.lower() in fields:
                ts_field = fields[timestamp_col.lower()]

        rows: List[Tuple[float, float]] = []
        for idx, row in enumerate(reader):
            p = safe_float(row.get(chosen_price_field))
            if p is None or p <= 0:
                continue
            ts = safe_float(row.get(ts_field)) if ts_field else None
            rows.append((float(idx if ts is None else ts), p))

    if not rows:
        raise ValueError(f"CSV had no valid price rows: {path}")

    rows.sort(key=lambda x: x[0])
    return [p for _, p in rows]


def make_synthetic_prices(n: int = 4000) -> List[float]:
    rng = random.Random(1337)
    prices: List[float] = []
    px = 100.0
    for i in range(n):
        trend = 0.00005 * i
        cyc = 0.8 * math.sin(i / 23.0) + 0.4 * math.sin(i / 67.0)
        noise = rng.uniform(-0.25, 0.25)
        px = max(1.0, px + trend + cyc * 0.02 + noise * 0.03)
        prices.append(px)
    return prices


def sma(values: Sequence[float], period: int) -> List[Optional[float]]:
    out: List[Optional[float]] = [None] * len(values)
    if period <= 0 or len(values) < period:
        return out

    window_sum = sum(values[:period])
    out[period - 1] = window_sum / period
    for i in range(period, len(values)):
        window_sum += values[i] - values[i - period]
        out[i] = window_sum / period
    return out


def rolling_std(values: Sequence[float], period: int) -> List[Optional[float]]:
    out: List[Optional[float]] = [None] * len(values)
    if period <= 1 or len(values) < period:
        return out

    for i in range(period - 1, len(values)):
        window = values[i - period + 1 : i + 1]
        m = sum(window) / period
        var = sum((x - m) ** 2 for x in window) / (period - 1)
        out[i] = math.sqrt(max(0.0, var))
    return out


def rsi(values: Sequence[float], period: int) -> List[Optional[float]]:
    out: List[Optional[float]] = [None] * len(values)
    if period <= 0 or len(values) <= period:
        return out

    gains = 0.0
    losses = 0.0
    for i in range(1, period + 1):
        delta = values[i] - values[i - 1]
        gains += max(delta, 0.0)
        losses += max(-delta, 0.0)

    avg_gain = gains / period
    avg_loss = losses / period

    rs = avg_gain / avg_loss if avg_loss > 1e-12 else float("inf")
    out[period] = 100.0 - (100.0 / (1.0 + rs))

    for i in range(period + 1, len(values)):
        delta = values[i] - values[i - 1]
        gain = max(delta, 0.0)
        loss = max(-delta, 0.0)
        avg_gain = (avg_gain * (period - 1) + gain) / period
        avg_loss = (avg_loss * (period - 1) + loss) / period
        rs = avg_gain / avg_loss if avg_loss > 1e-12 else float("inf")
        out[i] = 100.0 - (100.0 / (1.0 + rs))

    return out


def strategy_score(prices: Sequence[float], cfg: EvalConfig, params: Dict[str, Any]) -> EvalStats:
    if len(prices) < 100:
        raise ValueError("need at least 100 bars for evaluation")

    rsi_values = rsi(prices, cfg.rsi_len) if cfg.enable_rsi else [None] * len(prices)
    fast = sma(prices, cfg.fast_len) if cfg.enable_ma else [None] * len(prices)
    slow = sma(prices, cfg.slow_len) if cfg.enable_ma else [None] * len(prices)
    bb_mid = sma(prices, cfg.bb_len) if cfg.enable_bb else [None] * len(prices)
    bb_std = rolling_std(prices, cfg.bb_len) if cfg.enable_bb else [None] * len(prices)

    fee = cfg.fee_bps / 10_000.0
    slippage = cfg.slippage_bps / 10_000.0
    position = 0.0
    equity = 1.0
    peak_equity = equity
    max_drawdown = 0.0
    pnl_series: List[float] = []
    turnover = 0.0

    for i in range(1, len(prices)):
        components: List[float] = []

        r = rsi_values[i]
        if cfg.enable_rsi and r is not None:
            components.append(clamp((50.0 - r) / 50.0, -1.0, 1.0))

        f = fast[i]
        s = slow[i]
        if cfg.enable_ma and f is not None and s is not None and prices[i] > 0:
            ma_diff = (f - s) / prices[i]
            components.append(clamp(ma_diff * 25.0, -1.0, 1.0))

        m = bb_mid[i]
        sd = bb_std[i]
        if cfg.enable_bb and m is not None and sd is not None and sd > 1e-12:
            z = (prices[i] - m) / (cfg.bb_mult * sd)
            components.append(clamp(-z, -1.0, 1.0))

        signal = sum(components) / len(components) if components else 0.0

        target = 0.0
        if signal > cfg.threshold:
            target = 1.0
        elif signal < -cfg.threshold:
            target = -1.0

        trade_size = abs(target - position)
        turnover += trade_size
        position = target

        ret = prices[i] / prices[i - 1] - 1.0
        traded = trade_size > 1e-12
        trade_cost = (fee + slippage) * trade_size
        if traded:
            trade_cost += cfg.fixed_trade_cost
        pnl = position * ret - trade_cost
        pnl_series.append(pnl)

        equity *= 1.0 + pnl
        equity = max(equity, 1e-9)
        if equity > peak_equity:
            peak_equity = equity
        dd = (peak_equity - equity) / peak_equity if peak_equity > 0 else 0.0
        max_drawdown = max(max_drawdown, dd)

    mean = sum(pnl_series) / len(pnl_series)
    variance = sum((x - mean) ** 2 for x in pnl_series) / max(1, len(pnl_series) - 1)
    vol = math.sqrt(max(variance, 1e-12))
    if vol < 1e-6:
        sharpe_like = 0.0
    else:
        sharpe_like = (mean / vol) * math.sqrt(len(pnl_series))
    sharpe_capped = clamp(sharpe_like, -8.0, 8.0)

    total_return = equity - 1.0
    avg_turnover = turnover / max(1, len(pnl_series))

    complexity_penalty = 0.0
    for value in params.values():
        n = safe_float(value)
        if n is None:
            continue
        complexity_penalty += min(abs(n) / 5000.0, 0.02)

    score = (
        (total_return * 100.0)
        + (sharpe_capped * 8.0)
        - (max_drawdown * 90.0)
        - (avg_turnover * 6.0)
        - complexity_penalty
    )

    return EvalStats(
        score=score,
        total_return=total_return,
        max_drawdown=max_drawdown,
        sharpe_like=sharpe_like,
        turnover=avg_turnover,
        bars=len(prices),
    )


def parse_args(argv: Sequence[str]) -> argparse.Namespace:
    p = argparse.ArgumentParser(description="Evaluate a Pine candidate")
    p.add_argument("--ohlcv", default=os.getenv("PT_EVAL_OHLCV", ""), help="CSV with OHLCV data")
    p.add_argument("--price-col", default="close", help="price column name")
    p.add_argument("--timestamp-col", default="", help="optional timestamp column for sorting")
    p.add_argument(
        "--fee-bps",
        type=float,
        default=2.0,
        help="round-trip fee in bps per position change",
    )
    p.add_argument(
        "--slippage-bps",
        type=float,
        default=1.0,
        help="slippage estimate in bps per position change",
    )
    p.add_argument(
        "--fixed-trade-cost",
        type=float,
        default=0.0,
        help="fixed cost per trade in return units (e.g. 0.00005)",
    )
    p.add_argument("--verbose", action="store_true", help="emit evaluator internals to stderr")
    return p.parse_args(argv)


def main(argv: Sequence[str]) -> int:
    args = parse_args(argv)
    params = load_candidate()

    cfg = config_from_params(
        params,
        fee_bps=args.fee_bps,
        slippage_bps=args.slippage_bps,
        fixed_trade_cost=args.fixed_trade_cost,
    )

    if args.ohlcv:
        prices = load_prices_from_csv(
            args.ohlcv,
            price_col=args.price_col,
            timestamp_col=args.timestamp_col or None,
        )
        data_label = args.ohlcv
    else:
        prices = make_synthetic_prices()
        data_label = "synthetic"

    stats = strategy_score(prices, cfg, params)

    if args.verbose:
        script_path = os.getenv("PT_PINE_SCRIPT", "")
        print(
            json.dumps(
                {
                    "script": script_path,
                    "data": data_label,
                    "bars": stats.bars,
                    "total_return": stats.total_return,
                    "max_drawdown": stats.max_drawdown,
                    "sharpe_like": stats.sharpe_like,
                    "turnover": stats.turnover,
                    "score": stats.score,
                    "config": cfg.__dict__,
                },
                indent=2,
                sort_keys=True,
            ),
            file=sys.stderr,
        )

    # Last non-empty line must be numeric for pt-cli tune-pine evaluator contract.
    print(f"{stats.score:.10f}")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main(sys.argv[1:]))
    except Exception as exc:
        print(f"evaluator error: {exc}", file=sys.stderr)
        raise SystemExit(1)
