#!/usr/bin/env python3
"""Fetch OHLCV candles and export CSV for evaluator usage.

Default provider is Coinbase Exchange because Binance can return 451 in some regions.
"""

from __future__ import annotations

import argparse
import csv
import json
import pathlib
import urllib.parse
import urllib.request
from typing import List, Tuple


def fetch_json(url: str):
    req = urllib.request.Request(
        url,
        headers={
            "User-Agent": "Polymarket-Trader/0.1 (+https://localhost)",
            "Accept": "application/json",
        },
    )
    with urllib.request.urlopen(req, timeout=20) as resp:  # nosec B310 - controlled URL
        raw = resp.read().decode("utf-8")
    return json.loads(raw)


def fetch_coinbase(symbol: str, interval_sec: int, limit: int) -> List[Tuple[float, ...]]:
    product = symbol.replace("USDT", "-USD").replace("USD", "-USD")
    if "-" not in product:
        product = f"{symbol}-USD"
    # Coinbase endpoint returns up to 300 rows per request.
    granularity = interval_sec
    req_limit = min(max(limit, 1), 300)
    url = f"https://api.exchange.coinbase.com/products/{product}/candles?{urllib.parse.urlencode({'granularity': granularity})}"

    rows = fetch_json(url)
    rows = sorted(rows, key=lambda r: r[0])[:req_limit]

    out = []
    for r in rows:
        ts, low, high, open_, close, volume = r
        out.append((ts * 1000, open_, high, low, close, volume))
    return out


def fetch_binance(symbol: str, interval: str, limit: int) -> List[Tuple[float, ...]]:
    query = urllib.parse.urlencode(
        {
            "symbol": symbol,
            "interval": interval,
            "limit": limit,
        }
    )
    url = f"https://api.binance.com/api/v3/klines?{query}"

    rows = fetch_json(url)
    out = []
    for r in rows:
        out.append((r[0], r[1], r[2], r[3], r[4], r[5]))
    return out


def fetch_kraken(symbol: str, interval: str, limit: int) -> List[Tuple[float, ...]]:
    pair_map = {
        "BTCUSD": "XBTUSD",
        "ETHUSD": "ETHUSD",
        "SOLUSD": "SOLUSD",
        "XRPUSD": "XRPUSD",
    }
    kraken_pair = pair_map.get(symbol.upper(), symbol.upper())
    interval_map = {"1m": 1, "5m": 5, "15m": 15, "1h": 60}
    if interval not in interval_map:
        raise ValueError(f"unsupported interval '{interval}' for kraken mode")
    url = (
        "https://api.kraken.com/0/public/OHLC?"
        + urllib.parse.urlencode({"pair": kraken_pair, "interval": interval_map[interval]})
    )
    payload = fetch_json(url)
    if payload.get("error"):
        raise RuntimeError(f"kraken error: {payload['error']}")
    result = payload.get("result", {})
    rows_raw = []
    for key, value in result.items():
        if key != "last":
            rows_raw = value
            break

    out = []
    for r in rows_raw[-limit:]:
        ts = int(float(r[0]) * 1000)
        open_, high, low, close, _vwap, volume, _count = r[1:8]
        out.append((ts, open_, high, low, close, volume))
    return out


def make_synthetic(limit: int) -> List[Tuple[float, ...]]:
    out = []
    base_ts = 1_700_000_000_000
    price = 100.0
    for i in range(limit):
        drift = 0.02
        wobble = ((i % 11) - 5) * 0.01
        open_ = price
        close = max(1.0, price + drift + wobble)
        high = max(open_, close) + 0.05
        low = min(open_, close) - 0.05
        volume = 1.0 + (i % 7) * 0.1
        out.append((base_ts + i * 60_000, open_, high, low, close, volume))
        price = close
    return out


def parse_interval_to_seconds(interval: str) -> int:
    mapping = {
        "1m": 60,
        "5m": 300,
        "15m": 900,
        "1h": 3600,
    }
    if interval not in mapping:
        raise ValueError(f"unsupported interval '{interval}' for coinbase mode")
    return mapping[interval]


def main() -> int:
    p = argparse.ArgumentParser(description="Fetch OHLCV data")
    p.add_argument(
        "--provider", default="coinbase", choices=["coinbase", "binance", "kraken"]
    )
    p.add_argument("--symbol", default="BTCUSD")
    p.add_argument("--interval", default="1m")
    p.add_argument("--limit", type=int, default=300)
    p.add_argument("--out", default="data/ohlcv/btcusd_1m.csv")
    p.add_argument(
        "--allow-synthetic-fallback",
        action="store_true",
        help="if provider is blocked, write deterministic synthetic candles",
    )
    args = p.parse_args()

    try:
        if args.provider == "coinbase":
            rows = fetch_coinbase(
                symbol=args.symbol,
                interval_sec=parse_interval_to_seconds(args.interval),
                limit=args.limit,
            )
        elif args.provider == "kraken":
            rows = fetch_kraken(symbol=args.symbol, interval=args.interval, limit=args.limit)
        else:
            rows = fetch_binance(symbol=args.symbol, interval=args.interval, limit=args.limit)
    except Exception as exc:
        if not args.allow_synthetic_fallback:
            raise
        print(f"[warn] provider fetch failed ({exc}); using synthetic fallback")
        rows = make_synthetic(args.limit)

    out = pathlib.Path(args.out)
    out.parent.mkdir(parents=True, exist_ok=True)

    with out.open("w", newline="", encoding="utf-8") as f:
        w = csv.writer(f)
        w.writerow(["ts", "open", "high", "low", "close", "volume"])
        for row in rows:
            w.writerow(row)

    print(str(out))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
