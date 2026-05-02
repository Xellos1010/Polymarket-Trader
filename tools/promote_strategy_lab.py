#!/usr/bin/env python3
"""Promote a strategy-lab result into replay artifacts for the Rust engine.

Takes backtest or dashboard JSON output from `coinbase_strategy_lab.py` and writes:
1) replay NDJSON frames compatible with `pt-replay::ReplayFrame`
2) a promotion JSON artifact with selected market/variant and replay instructions.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import pathlib
import time
from typing import Any, Dict, List, Optional, Tuple

UTC = dt.timezone.utc


def read_json(path: pathlib.Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def clamp(v: float, lo: float, hi: float) -> float:
    return max(lo, min(hi, v))


def extract_backtest(payload: dict) -> dict:
    if "backtest" in payload and isinstance(payload["backtest"], dict):
        return payload["backtest"]
    return payload


def choose_market_variant(
    backtest: dict,
    market: Optional[str],
    variant: Optional[str],
) -> Tuple[str, str]:
    markets = backtest.get("markets", {})
    if not isinstance(markets, dict) or not markets:
        raise ValueError("no markets in report")

    if market and market not in markets:
        raise ValueError(f"market not found in report: {market}")

    selected_market = market
    selected_variant = variant

    if selected_market is None:
        best_score = None
        for m_name, m_payload in markets.items():
            variants = m_payload.get("variants", {})
            if not isinstance(variants, dict) or not variants:
                continue
            v_name = selected_variant or m_payload.get("default_variant") or sorted(variants.keys())[0]
            v = variants.get(v_name)
            if not isinstance(v, dict):
                continue
            score = float(v.get("metrics", {}).get("total_return", -1e18))
            if best_score is None or score > best_score:
                best_score = score
                selected_market = m_name
                if selected_variant is None:
                    selected_variant = v_name

    if selected_market is None:
        raise ValueError("unable to select market")

    m_payload = markets[selected_market]
    variants = m_payload.get("variants", {})
    if not isinstance(variants, dict) or not variants:
        raise ValueError(f"market {selected_market} has no variants")

    if selected_variant is None:
        selected_variant = m_payload.get("default_variant") or sorted(variants.keys())[0]

    if selected_variant not in variants:
        raise ValueError(f"variant {selected_variant} not found for market {selected_market}")

    return selected_market, selected_variant


def build_frames(
    market: str,
    market_payload: dict,
    variant_payload: dict,
    granularity_sec: int,
    spread_bps: float,
    liquidity: float,
    market_id_prefix: str,
    token_id_prefix: str,
) -> List[dict]:
    closes = market_payload.get("close", [])
    if not isinstance(closes, list) or len(closes) < 3:
        raise ValueError(f"market {market} has insufficient close series")

    ts_ms = market_payload.get("ts_ms")
    if not isinstance(ts_ms, list) or len(ts_ms) != len(closes):
        # fallback synthetic timeline
        start_ms = int(dt.datetime.now(UTC).timestamp() * 1000)
        ts_ms = [start_ms + i * granularity_sec * 1000 for i in range(len(closes))]

    bias_series = variant_payload.get("bias_series", [])
    if not isinstance(bias_series, list):
        bias_series = []

    slug = market.replace("-", "_").lower()
    market_id = f"{market_id_prefix}-{slug}"
    token_id = f"{token_id_prefix}-{slug}"

    frames = []
    for i, px in enumerate(closes):
        close_px = float(px)
        spread_abs = max(close_px * spread_bps / 10_000.0, 1e-6)
        bid = max(1e-9, close_px - spread_abs / 2.0)
        ask = close_px + spread_abs / 2.0
        bias = 0.0
        if i < len(bias_series):
            bias = clamp(float(bias_series[i]), -1.0, 1.0)

        frames.append(
            {
                "snapshot": {
                    "market_id": market_id,
                    "token_id": token_id,
                    "bid": bid,
                    "ask": ask,
                    "spread": ask - bid,
                    "liquidity": liquidity,
                    "ts": dt.datetime.fromtimestamp(int(ts_ms[i]) / 1000.0, tz=UTC)
                    .isoformat()
                    .replace("+00:00", "Z"),
                },
                "bias": bias,
            }
        )

    return frames


def write_ndjson(path: pathlib.Path, rows: List[dict]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as f:
        for row in rows:
            f.write(json.dumps(row, separators=(",", ":")))
            f.write("\n")


def main() -> int:
    p = argparse.ArgumentParser(description="Promote strategy-lab result to replay artifacts")
    p.add_argument("--report", required=True, help="strategy-lab backtest or dashboard JSON")
    p.add_argument("--market", default=None, help="specific market (e.g., BTC-USD)")
    p.add_argument("--variant", default=None, help="specific variant (e.g., sma_baseline)")
    p.add_argument("--spread-bps", type=float, default=2.0, help="synthetic bid/ask spread in bps")
    p.add_argument("--liquidity", type=float, default=10000.0, help="synthetic liquidity per frame")
    p.add_argument("--market-id-prefix", default="coinbase-sim", help="market_id prefix")
    p.add_argument("--token-id-prefix", default="sim", help="token_id prefix")
    p.add_argument("--out-replay", default="data/replay/strategy_lab_promoted.ndjson", help="replay output path")
    p.add_argument("--out-promotion", default="data/tuning/strategy_lab_promoted.json", help="promotion artifact path")
    args = p.parse_args()

    report_path = pathlib.Path(args.report)
    payload = read_json(report_path)
    backtest = extract_backtest(payload)

    granularity_sec = int(backtest.get("meta", {}).get("granularity_sec", 300))

    selected_market, selected_variant = choose_market_variant(backtest, args.market, args.variant)
    market_payload = backtest["markets"][selected_market]
    variant_payload = market_payload["variants"][selected_variant]

    frames = build_frames(
        market=selected_market,
        market_payload=market_payload,
        variant_payload=variant_payload,
        granularity_sec=granularity_sec,
        spread_bps=float(args.spread_bps),
        liquidity=float(args.liquidity),
        market_id_prefix=str(args.market_id_prefix),
        token_id_prefix=str(args.token_id_prefix),
    )

    replay_path = pathlib.Path(args.out_replay)
    write_ndjson(replay_path, frames)

    metrics = variant_payload.get("metrics", {})
    promotion = {
        "promoted_at_epoch": int(time.time()),
        "source_report": str(report_path),
        "market": selected_market,
        "variant": selected_variant,
        "metrics": metrics,
        "replay": {
            "path": str(replay_path),
            "frames": len(frames),
            "market_id": frames[0]["snapshot"]["market_id"],
            "token_id": frames[0]["snapshot"]["token_id"],
        },
        "verification": {
            "required_mode": "replay",
            "config_patch": {
                "engine": {
                    "mode": "replay",
                    "replay_path": str(replay_path),
                }
            },
            "commands": [
                "cargo run -p pt-cli -- run --config config/config.toml",
                "cargo run -p pt-cli -- status --url http://127.0.0.1:8080/health",
            ],
        },
    }

    out_promotion = pathlib.Path(args.out_promotion)
    out_promotion.parent.mkdir(parents=True, exist_ok=True)
    out_promotion.write_text(json.dumps(promotion, indent=2, sort_keys=True), encoding="utf-8")

    print(str(replay_path))
    print(str(out_promotion))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
