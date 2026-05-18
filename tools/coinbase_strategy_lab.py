#!/usr/bin/env python3
"""Coinbase strategy lab: backtest, overlap, optimize, and dashboard rendering.

Local-first tooling for rapid strategy iteration on Coinbase markets.
"""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import json
import math
import pathlib
import sqlite3
import statistics
import time
import urllib.parse
import urllib.request
from dataclasses import dataclass
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from typing import Any, Dict, List, Optional, Sequence, Tuple

# Optional Bayesian optimisation back-end (pip install optuna).
# Falls back to grid search silently when not installed.
try:
    import optuna
    optuna.logging.set_verbosity(optuna.logging.WARNING)
    OPTUNA_AVAILABLE = True
except ImportError:
    OPTUNA_AVAILABLE = False

UTC = dt.timezone.utc


@dataclass
class Candle:
    ts_ms: int
    open: float
    high: float
    low: float
    close: float
    volume: float


def read_config(path: str) -> dict:
    return json.loads(pathlib.Path(path).read_text(encoding="utf-8"))


def parse_iso_utc(text: str) -> dt.datetime:
    if text.endswith("Z"):
        text = text[:-1] + "+00:00"
    parsed = dt.datetime.fromisoformat(text)
    if parsed.tzinfo is None:
        parsed = parsed.replace(tzinfo=UTC)
    return parsed.astimezone(UTC)


def iso_utc_from_ms(ts_ms: int) -> str:
    return dt.datetime.fromtimestamp(ts_ms / 1000.0, tz=UTC).isoformat().replace("+00:00", "Z")


def clamp(value: float, lo: float, hi: float) -> float:
    return max(lo, min(hi, value))


def http_json(url: str) -> object:
    req = urllib.request.Request(
        url,
        headers={
            "User-Agent": "Polymarket-Trader-StrategyLab/0.3",
            "Accept": "application/json",
        },
    )
    with urllib.request.urlopen(req, timeout=30) as resp:  # nosec B310
        return json.loads(resp.read().decode("utf-8"))


def fetch_coinbase_products() -> List[dict]:
    payload = http_json("https://api.exchange.coinbase.com/products")
    if not isinstance(payload, list):
        raise RuntimeError("unexpected products payload")
    return payload


def fetch_coinbase_candles(
    product_id: str,
    granularity_sec: int,
    limit: int,
    start: Optional[dt.datetime] = None,
    end: Optional[dt.datetime] = None,
) -> List[Candle]:
    query: Dict[str, str] = {"granularity": str(granularity_sec)}
    if start and end:
        query["start"] = start.isoformat().replace("+00:00", "Z")
        query["end"] = end.isoformat().replace("+00:00", "Z")

    url = (
        f"https://api.exchange.coinbase.com/products/{product_id}/candles?"
        + urllib.parse.urlencode(query)
    )

    raw = http_json(url)
    rows = []
    for item in raw:
        # [time, low, high, open, close, volume]
        ts_s, low, high, open_, close, volume = item
        rows.append(
            Candle(
                ts_ms=int(float(ts_s) * 1000),
                open=float(open_),
                high=float(high),
                low=float(low),
                close=float(close),
                volume=float(volume),
            )
        )

    rows.sort(key=lambda c: c.ts_ms)
    if limit > 0 and len(rows) > limit:
        rows = rows[-limit:]
    return rows


def fetch_kraken_candles(product_id: str, granularity_sec: int, limit: int) -> List[Candle]:
    symbol = product_id.replace("-", "").upper().replace("BTC", "XBT")
    interval_map = {60: 1, 300: 5, 900: 15, 3600: 60, 21600: 240, 86400: 1440}
    if granularity_sec not in interval_map:
        raise ValueError(
            f"granularity {granularity_sec} not supported for kraken (try 60/300/900/3600/21600/86400)"
        )

    url = "https://api.kraken.com/0/public/OHLC?" + urllib.parse.urlencode(
        {"pair": symbol, "interval": interval_map[granularity_sec]}
    )
    payload = http_json(url)
    if payload.get("error"):
        raise RuntimeError(f"kraken error: {payload['error']}")

    result = payload.get("result", {})
    series = []
    for key, value in result.items():
        if key != "last":
            series = value
            break

    candles: List[Candle] = []
    for item in series[-limit:]:
        ts_s = float(item[0])
        open_ = float(item[1])
        high = float(item[2])
        low = float(item[3])
        close = float(item[4])
        volume = float(item[6])
        candles.append(
            Candle(
                ts_ms=int(ts_s * 1000),
                open=open_,
                high=high,
                low=low,
                close=close,
                volume=volume,
            )
        )

    candles.sort(key=lambda c: c.ts_ms)
    return candles


def fetch_candles(
    provider: str,
    product_id: str,
    granularity_sec: int,
    limit: int,
    start: Optional[dt.datetime] = None,
    end: Optional[dt.datetime] = None,
) -> List[Candle]:
    if provider == "coinbase":
        return fetch_coinbase_candles(product_id, granularity_sec, limit, start, end)
    if provider == "kraken":
        return fetch_kraken_candles(product_id, granularity_sec, limit)
    raise ValueError(f"unsupported provider: {provider}")


def fetch_candles_retry(
    provider: str,
    product_id: str,
    granularity_sec: int,
    limit: int,
    start: Optional[dt.datetime] = None,
    end: Optional[dt.datetime] = None,
    attempts: int = 3,
) -> List[Candle]:
    last_error: Optional[Exception] = None
    for idx in range(attempts):
        try:
            return fetch_candles(provider, product_id, granularity_sec, limit, start, end)
        except Exception as exc:  # broad catch to keep batch jobs moving
            last_error = exc
            if idx + 1 < attempts:
                time.sleep(0.35 * (idx + 1))
    raise RuntimeError(f"failed to fetch candles for {product_id}: {last_error}")


def sma(values: Sequence[float], window: int) -> List[Optional[float]]:
    if window <= 0:
        raise ValueError("window must be > 0")
    out: List[Optional[float]] = [None] * len(values)
    if len(values) < window:
        return out
    s = sum(values[:window])
    out[window - 1] = s / window
    for i in range(window, len(values)):
        s += values[i] - values[i - window]
        out[i] = s / window
    return out


def rsi(values: Sequence[float], window: int) -> List[Optional[float]]:
    out: List[Optional[float]] = [None] * len(values)
    if window <= 1 or len(values) < window + 1:
        return out

    gains = 0.0
    losses = 0.0
    for i in range(1, window + 1):
        delta = values[i] - values[i - 1]
        gains += max(delta, 0.0)
        losses += max(-delta, 0.0)

    avg_gain = gains / window
    avg_loss = losses / window

    def current_rsi(g: float, l: float) -> float:
        if l <= 1e-12:
            return 100.0
        rs = g / l
        return 100.0 - (100.0 / (1.0 + rs))

    out[window] = current_rsi(avg_gain, avg_loss)

    for i in range(window + 1, len(values)):
        delta = values[i] - values[i - 1]
        gain = max(delta, 0.0)
        loss = max(-delta, 0.0)
        avg_gain = ((avg_gain * (window - 1)) + gain) / window
        avg_loss = ((avg_loss * (window - 1)) + loss) / window
        out[i] = current_rsi(avg_gain, avg_loss)

    return out


def max_drawdown(equity: Sequence[float]) -> float:
    peak = -float("inf")
    max_dd = 0.0
    for x in equity:
        peak = max(peak, x)
        if peak > 0:
            dd = (peak - x) / peak
            max_dd = max(max_dd, dd)
    return max_dd


def strategy_settings(config: dict) -> dict:
    backtest = config.setdefault("backtest", {})
    strategy = backtest.setdefault("strategy", {})
    return {
        "short_window": int(strategy.get("short_window", 9)),
        "long_window": int(strategy.get("long_window", 21)),
        "fee_bps": float(strategy.get("trade_fee_bps", 8.0)),
        "slippage_bps": float(strategy.get("slippage_bps", 2.0)),
        "starting_equity": float(strategy.get("starting_equity", 1000.0)),
    }


def resolve_variants(config: dict) -> List[dict]:
    backtest = config.get("backtest", {})
    variants = backtest.get("variants")
    if isinstance(variants, list) and variants:
        out = []
        for idx, row in enumerate(variants):
            if not isinstance(row, dict):
                continue
            name = str(row.get("name") or f"variant_{idx + 1}")
            out.append(
                {
                    "name": name,
                    "bias_gain": float(row.get("bias_gain", 0.0)),
                    "plugins": row.get("plugins", []) if isinstance(row.get("plugins"), list) else [],
                }
            )
        if out:
            return out

    # Backwards compatible default.
    return [{"name": "sma_baseline", "bias_gain": 0.0, "plugins": []}]


def load_external_bias_series(path: str, candles: Sequence[Candle]) -> Tuple[List[float], dict]:
    n = len(candles)
    series = [0.0] * n
    p = pathlib.Path(path)
    if not p.exists():
        raise FileNotFoundError(f"external bias file not found: {path}")

    text = p.read_text(encoding="utf-8")
    loaded_points = 0

    if p.suffix.lower() in {".csv", ".tsv"}:
        dialect = "excel-tab" if p.suffix.lower() == ".tsv" else "excel"
        reader = csv.DictReader(text.splitlines(), dialect=dialect)
        ts_to_idx = {c.ts_ms: idx for idx, c in enumerate(candles)}
        for row in reader:
            if not row:
                continue
            bias = float(row.get("bias", 0.0))
            if "idx" in row and row["idx"]:
                idx = int(row["idx"])
                if 0 <= idx < n:
                    series[idx] = clamp(bias, -1.0, 1.0)
                    loaded_points += 1
                continue
            if "ts_ms" in row and row["ts_ms"]:
                ts = int(float(row["ts_ms"]))
                idx = ts_to_idx.get(ts)
                if idx is not None:
                    series[idx] = clamp(bias, -1.0, 1.0)
                    loaded_points += 1
        return series, {"points_loaded": loaded_points, "source": path}

    payload = json.loads(text)
    rows: List[Any]
    if isinstance(payload, list):
        rows = payload
    elif isinstance(payload, dict):
        if isinstance(payload.get("bias_by_index"), list):
            rows = [{"idx": idx, "bias": val} for idx, val in enumerate(payload["bias_by_index"])]
        elif isinstance(payload.get("series"), list):
            rows = payload["series"]
        else:
            rows = []
    else:
        rows = []

    ts_to_idx = {c.ts_ms: idx for idx, c in enumerate(candles)}
    for row in rows:
        if not isinstance(row, dict):
            continue
        bias = float(row.get("bias", 0.0))
        if row.get("idx") is not None:
            idx = int(row.get("idx"))
            if 0 <= idx < n:
                series[idx] = clamp(bias, -1.0, 1.0)
                loaded_points += 1
            continue

        ts_ms: Optional[int] = None
        if row.get("ts_ms") is not None:
            ts_ms = int(float(row.get("ts_ms")))
        elif row.get("ts"):
            ts_ms = int(parse_iso_utc(str(row.get("ts"))).timestamp() * 1000)
        elif row.get("time"):
            ts_ms = int(parse_iso_utc(str(row.get("time"))).timestamp() * 1000)

        if ts_ms is None:
            continue

        idx = ts_to_idx.get(ts_ms)
        if idx is not None:
            series[idx] = clamp(bias, -1.0, 1.0)
            loaded_points += 1

    return series, {"points_loaded": loaded_points, "source": path}


def compute_plugin_series(candles: Sequence[Candle], plugin: dict) -> Tuple[List[float], dict]:
    name = str(plugin.get("name", "")).strip().lower()
    closes = [c.close for c in candles]
    n = len(candles)

    if name == "external_bias_file":
        path = str(plugin.get("path", "")).strip()
        if not path:
            raise ValueError("external_bias_file requires plugin.path")
        return load_external_bias_series(path, candles)

    if name == "momentum_bias":
        lookback = int(plugin.get("lookback", 6))
        scale = float(plugin.get("scale", 12.0))
        out = [0.0] * n
        for i in range(lookback, n):
            ret = closes[i] / closes[i - lookback] - 1.0
            out[i] = clamp(math.tanh(scale * ret), -1.0, 1.0)
        return out, {"lookback": lookback, "scale": scale}

    if name == "rsi_bias":
        window = int(plugin.get("window", 14))
        oversold = float(plugin.get("oversold", 30.0))
        overbought = float(plugin.get("overbought", 70.0))
        rsi_vals = rsi(closes, window)
        out = [0.0] * n
        for i, rv in enumerate(rsi_vals):
            if rv is None:
                continue
            if rv <= oversold:
                out[i] = clamp((oversold - rv) / max(1.0, oversold), 0.0, 1.0)
            elif rv >= overbought:
                out[i] = clamp(-((rv - overbought) / max(1.0, 100.0 - overbought)), -1.0, 0.0)
        return out, {"window": window, "oversold": oversold, "overbought": overbought}

    raise ValueError(f"unsupported plugin: {name}")


def build_combined_bias_series(candles: Sequence[Candle], plugins: Sequence[dict]) -> Tuple[List[float], dict, List[str]]:
    combined = [0.0] * len(candles)
    details: Dict[str, Any] = {"plugins": []}
    errors: List[str] = []

    for idx, plugin in enumerate(plugins):
        if not isinstance(plugin, dict):
            continue
        if plugin.get("enabled", True) is False:
            continue

        name = str(plugin.get("name") or f"plugin_{idx + 1}")
        weight = float(plugin.get("weight", 1.0))

        try:
            series, meta = compute_plugin_series(candles, plugin)
        except Exception as exc:  # keep backtest running if one plugin fails
            errors.append(f"{name}: {exc}")
            continue

        if len(series) != len(combined):
            errors.append(f"{name}: bias series length mismatch")
            continue

        for i in range(len(combined)):
            combined[i] += weight * series[i]

        details["plugins"].append(
            {
                "name": name,
                "weight": weight,
                "meta": meta,
            }
        )

    combined = [clamp(v, -1.0, 1.0) for v in combined]
    details["active_plugins"] = len(details["plugins"])
    return combined, details, errors


def backtest_sma_crossover(
    candles: Sequence[Candle],
    short_window: int,
    long_window: int,
    fee_bps: float,
    slippage_bps: float,
    starting_equity: float,
    bias_series: Optional[Sequence[float]] = None,
    bias_gain: float = 0.0,
) -> dict:
    closes = [c.close for c in candles]
    short = sma(closes, short_window)
    long = sma(closes, long_window)

    position = 0.0
    equity = starting_equity
    eq_curve = [equity]
    pos_curve = [position]
    returns = []
    trades = []

    friction = (fee_bps + slippage_bps) / 10_000.0

    for i in range(1, len(candles)):
        s_val = short[i]
        l_val = long[i]

        base_target = 1.0 if (s_val is not None and l_val is not None and s_val > l_val) else 0.0
        bias = 0.0
        if bias_series is not None and i < len(bias_series):
            bias = clamp(float(bias_series[i]), -1.0, 1.0)

        target = clamp(base_target + bias_gain * bias, 0.0, 1.0)
        delta = target - position
        turnover = abs(delta)
        position = target

        ret = closes[i] / closes[i - 1] - 1.0
        pnl = position * ret - friction * turnover
        equity *= 1.0 + pnl

        returns.append(pnl)
        eq_curve.append(equity)
        pos_curve.append(position)

        if turnover > 1e-12:
            trades.append(
                {
                    "idx": i,
                    "ts_ms": candles[i].ts_ms,
                    "side": "BUY" if delta > 0 else "SELL",
                    "delta": delta,
                    "target_position": target,
                    "bias": bias,
                    "price": closes[i],
                }
            )

    total_return = (equity / starting_equity - 1.0) if starting_equity > 0 else 0.0
    avg_ret = statistics.fmean(returns) if returns else 0.0
    std_ret = statistics.pstdev(returns) if len(returns) > 1 else 0.0
    sharpe_like = (avg_ret / std_ret * math.sqrt(len(returns))) if std_ret > 1e-12 else 0.0

    return {
        "equity_curve": eq_curve,
        "position_curve": pos_curve,
        "short_sma": short,
        "long_sma": long,
        "trades": trades,
        "metrics": {
            "total_return": total_return,
            "max_drawdown": max_drawdown(eq_curve),
            "sharpe_like": sharpe_like,
            "trades": len(trades),
            "bars": len(candles),
        },
    }


def build_attribution(metrics: dict, trades: Sequence[dict], starting_equity: float) -> dict:
    buy_count = sum(1 for t in trades if t.get("side") == "BUY")
    sell_count = sum(1 for t in trades if t.get("side") == "SELL")
    gross_turnover = sum(abs(float(t.get("delta", 0.0))) * float(t.get("price", 0.0)) for t in trades)

    total_return = float(metrics.get("total_return", 0.0))
    pnl_abs = starting_equity * total_return

    return {
        "buy_count": buy_count,
        "sell_count": sell_count,
        "gross_turnover": gross_turnover,
        "pnl_abs": pnl_abs,
        "final_equity": starting_equity * (1.0 + total_return),
    }


class TradeJournal:
    def __init__(self, sqlite_path: str) -> None:
        self.sqlite_path = sqlite_path
        path = pathlib.Path(sqlite_path)
        path.parent.mkdir(parents=True, exist_ok=True)
        self.conn = sqlite3.connect(sqlite_path)
        self.conn.execute("PRAGMA journal_mode=WAL")
        self.conn.execute(
            """
            CREATE TABLE IF NOT EXISTS lab_runs (
                run_id TEXT PRIMARY KEY,
                created_ts_ms INTEGER NOT NULL,
                mode TEXT NOT NULL,
                provider TEXT NOT NULL,
                granularity_sec INTEGER NOT NULL,
                config_json TEXT NOT NULL
            )
            """
        )
        self.conn.execute(
            """
            CREATE TABLE IF NOT EXISTS market_results (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                run_id TEXT NOT NULL,
                market TEXT NOT NULL,
                variant TEXT NOT NULL,
                total_return REAL NOT NULL,
                max_drawdown REAL NOT NULL,
                sharpe_like REAL NOT NULL,
                trades INTEGER NOT NULL,
                bars INTEGER NOT NULL,
                pnl_abs REAL NOT NULL,
                final_equity REAL NOT NULL,
                created_ts_ms INTEGER NOT NULL
            )
            """
        )
        self.conn.execute(
            """
            CREATE TABLE IF NOT EXISTS trade_fills (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                run_id TEXT NOT NULL,
                market TEXT NOT NULL,
                variant TEXT NOT NULL,
                bar_idx INTEGER NOT NULL,
                ts_ms INTEGER NOT NULL,
                side TEXT NOT NULL,
                price REAL NOT NULL,
                delta REAL NOT NULL,
                target_position REAL NOT NULL,
                bias REAL NOT NULL
            )
            """
        )
        self.conn.execute("CREATE INDEX IF NOT EXISTS idx_market_results_market_variant ON market_results(market, variant)")
        self.conn.execute("CREATE INDEX IF NOT EXISTS idx_market_results_run_id ON market_results(run_id)")
        self.conn.execute("CREATE INDEX IF NOT EXISTS idx_trade_fills_market_variant ON trade_fills(market, variant)")
        self.conn.commit()

    def start_run(self, mode: str, provider: str, granularity_sec: int, config: dict) -> str:
        run_id = f"{mode}-{int(time.time() * 1000)}"
        self.conn.execute(
            "INSERT INTO lab_runs (run_id, created_ts_ms, mode, provider, granularity_sec, config_json) VALUES (?, ?, ?, ?, ?, ?)",
            (
                run_id,
                int(time.time() * 1000),
                mode,
                provider,
                granularity_sec,
                json.dumps(config, separators=(",", ":")),
            ),
        )
        self.conn.commit()
        return run_id

    def record_result(self, run_id: str, market: str, variant: str, metrics: dict, attribution: dict) -> None:
        self.conn.execute(
            """
            INSERT INTO market_results (
                run_id, market, variant, total_return, max_drawdown, sharpe_like,
                trades, bars, pnl_abs, final_equity, created_ts_ms
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
            (
                run_id,
                market,
                variant,
                float(metrics.get("total_return", 0.0)),
                float(metrics.get("max_drawdown", 0.0)),
                float(metrics.get("sharpe_like", 0.0)),
                int(metrics.get("trades", 0)),
                int(metrics.get("bars", 0)),
                float(attribution.get("pnl_abs", 0.0)),
                float(attribution.get("final_equity", 0.0)),
                int(time.time() * 1000),
            ),
        )

    def record_trades(self, run_id: str, market: str, variant: str, trades: Sequence[dict]) -> None:
        self.conn.executemany(
            """
            INSERT INTO trade_fills (
                run_id, market, variant, bar_idx, ts_ms, side, price, delta, target_position, bias
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
            [
                (
                    run_id,
                    market,
                    variant,
                    int(t.get("idx", 0)),
                    int(t.get("ts_ms", 0)),
                    str(t.get("side", "")),
                    float(t.get("price", 0.0)),
                    float(t.get("delta", 0.0)),
                    float(t.get("target_position", 0.0)),
                    float(t.get("bias", 0.0)),
                )
                for t in trades
            ],
        )

    def flush(self) -> None:
        self.conn.commit()

    def summarize(self, max_rows: int = 50) -> List[dict]:
        cur = self.conn.execute(
            """
            SELECT
                market,
                variant,
                COUNT(*) as runs,
                AVG(total_return) as avg_total_return,
                AVG(max_drawdown) as avg_max_drawdown,
                AVG(pnl_abs) as avg_pnl_abs,
                MAX(created_ts_ms) as last_ts_ms
            FROM market_results
            GROUP BY market, variant
            ORDER BY last_ts_ms DESC
            LIMIT ?
            """,
            (max_rows,),
        )
        rows = []
        for market, variant, runs, avg_ret, avg_dd, avg_pnl, last_ts in cur.fetchall():
            rows.append(
                {
                    "market": market,
                    "variant": variant,
                    "runs": int(runs),
                    "avg_total_return": float(avg_ret or 0.0),
                    "avg_max_drawdown": float(avg_dd or 0.0),
                    "avg_pnl_abs": float(avg_pnl or 0.0),
                    "last_ts_ms": int(last_ts or 0),
                }
            )
        return rows



def discover_recent_coinbase_listings(overlap_cfg: dict) -> Tuple[List[dict], List[str]]:
    auto = overlap_cfg.get("auto_discovery", {}) if isinstance(overlap_cfg, dict) else {}
    if not auto.get("enabled", False):
        return [], []

    quote_currencies = {str(x).upper() for x in auto.get("quote_currencies", ["USD"])}
    lookback_days = int(auto.get("lookback_days", 120))
    discovery_granularity_sec = int(auto.get("discovery_granularity_sec", 86400))
    max_products_scan = int(auto.get("max_products_scan", 120))
    max_results = int(auto.get("max_results", 10))
    min_candles = int(auto.get("min_candles", 20))
    min_gap_candles = int(auto.get("min_gap_from_window_start_candles", 2))

    diagnostics: List[str] = []

    now = dt.datetime.now(UTC)
    start = now - dt.timedelta(days=lookback_days)

    products = fetch_coinbase_products()
    eligible: List[dict] = []
    for p in products:
        if not isinstance(p, dict):
            continue
        product_id = str(p.get("id", ""))
        if "-" not in product_id:
            continue

        quote = str(p.get("quote_currency", "")).upper()
        status = str(p.get("status", "")).lower()
        trading_disabled = bool(p.get("trading_disabled", False))

        if quote not in quote_currencies:
            continue
        if trading_disabled:
            continue
        if status not in {"online", "offline", "internal"}:
            continue

        eligible.append(p)

    eligible.sort(key=lambda p: str(p.get("id", "")))

    discovered = []
    scanned = 0
    for p in eligible:
        if scanned >= max_products_scan:
            break

        product_id = str(p.get("id"))
        scanned += 1
        try:
            candles = fetch_coinbase_candles(
                product_id,
                discovery_granularity_sec,
                limit=300,
                start=start,
                end=now,
            )
        except Exception as exc:
            diagnostics.append(f"{product_id}: discovery fetch failed: {exc}")
            continue

        if len(candles) < min_candles:
            continue

        first = candles[0]
        threshold_ms = int(start.timestamp() * 1000) + min_gap_candles * discovery_granularity_sec * 1000
        if first.ts_ms <= threshold_ms:
            continue

        discovered.append(
            {
                "product_id": product_id,
                "label": product_id.split("-")[0],
                "anchor_time": iso_utc_from_ms(first.ts_ms),
                "source": "auto_discovery",
                "first_window_volume": sum(c.volume for c in candles[: min(5, len(candles))]),
                "first_candle_ts_ms": first.ts_ms,
            }
        )

    discovered.sort(key=lambda x: (x["first_candle_ts_ms"], x["first_window_volume"]), reverse=True)
    trimmed = discovered[:max_results]
    diagnostics.append(
        f"auto-discovery scanned={scanned}, eligible={len(eligible)}, discovered={len(discovered)}, selected={len(trimmed)}"
    )

    for row in trimmed:
        row.pop("first_candle_ts_ms", None)
    return trimmed, diagnostics


def open_journal_if_enabled(config: dict) -> Optional[TradeJournal]:
    j = config.get("journal", {}) if isinstance(config.get("journal"), dict) else {}
    if j.get("enabled", True) is False:
        return None
    path = str(j.get("sqlite_path", "data/strategy_lab/trade_journal.sqlite"))
    return TradeJournal(path)


def run_backtest_data(config: dict) -> dict:
    provider = config.get("provider", "coinbase")
    granularity = int(config.get("granularity_sec", 300))

    bt = config.get("backtest", {})
    markets = bt.get("markets", [])
    limit = int(bt.get("limit", 300))
    st = strategy_settings(config)
    variants = resolve_variants(config)

    payload = {
        "meta": {
            "provider": provider,
            "granularity_sec": granularity,
            "limit": limit,
            "strategy": st,
            "variants": variants,
        },
        "markets": {},
        "errors": [],
        "journal_summary": [],
    }

    journal = open_journal_if_enabled(config)
    run_id: Optional[str] = None
    if journal is not None:
        run_id = journal.start_run("backtest", provider, granularity, config)
        payload["meta"]["journal_path"] = journal.sqlite_path
        payload["meta"]["journal_run_id"] = run_id

    for market in markets:
        try:
            candles = fetch_candles_retry(provider, market, granularity, limit)
        except Exception as exc:
            payload["errors"].append({"market": market, "error": str(exc)})
            continue

        min_bars_needed = max(st["short_window"], st["long_window"]) + 2
        if len(candles) < min_bars_needed:
            payload["errors"].append({"market": market, "error": "insufficient candle history"})
            continue

        first_close = candles[0].close
        last_close = candles[-1].close
        period_return = (last_close / first_close - 1.0) if first_close > 0 else 0.0

        market_payload: Dict[str, Any] = {
            "ts_ms": [c.ts_ms for c in candles],
            "close": [c.close for c in candles],
            "market_stats": {
                "current_price": last_close,
                "range_return": period_return,
                "bars": len(candles),
                "volume_last": candles[-1].volume,
            },
            "variants": {},
            "variant_errors": [],
        }

        for variant in variants:
            v_name = variant["name"]
            bias_gain = float(variant.get("bias_gain", 0.0))
            plugins = variant.get("plugins", [])

            combined_bias, plugin_meta, plugin_errors = build_combined_bias_series(candles, plugins)
            bias_series = combined_bias if (plugins and abs(bias_gain) > 1e-12) else None

            bt_res = backtest_sma_crossover(
                candles,
                short_window=st["short_window"],
                long_window=st["long_window"],
                fee_bps=st["fee_bps"],
                slippage_bps=st["slippage_bps"],
                starting_equity=st["starting_equity"],
                bias_series=bias_series,
                bias_gain=bias_gain,
            )
            attribution = build_attribution(
                metrics=bt_res["metrics"],
                trades=bt_res["trades"],
                starting_equity=st["starting_equity"],
            )

            variant_payload = {
                "bias_gain": bias_gain,
                "plugins": plugins,
                "plugin_meta": plugin_meta,
                "plugin_errors": plugin_errors,
                "bias_series": combined_bias,
                "short_sma": bt_res["short_sma"],
                "long_sma": bt_res["long_sma"],
                "equity_curve": bt_res["equity_curve"],
                "position_curve": bt_res["position_curve"],
                "trades": bt_res["trades"],
                "metrics": bt_res["metrics"],
                "attribution": attribution,
            }
            market_payload["variants"][v_name] = variant_payload

            if plugin_errors:
                market_payload["variant_errors"].append({"variant": v_name, "errors": plugin_errors})

            if journal is not None and run_id is not None:
                journal.record_result(run_id, market, v_name, bt_res["metrics"], attribution)
                journal.record_trades(run_id, market, v_name, bt_res["trades"])

        if not market_payload["variants"]:
            payload["errors"].append({"market": market, "error": "no variant results"})
            continue

        default_variant = variants[0]["name"]
        if default_variant not in market_payload["variants"]:
            default_variant = sorted(market_payload["variants"].keys())[0]
        market_payload["default_variant"] = default_variant

        # Backward-compatible top-level fields for the default variant.
        dv = market_payload["variants"][default_variant]
        market_payload["short_sma"] = dv["short_sma"]
        market_payload["long_sma"] = dv["long_sma"]
        market_payload["equity_curve"] = dv["equity_curve"]
        market_payload["trades"] = dv["trades"]
        market_payload["metrics"] = dv["metrics"]

        payload["markets"][market] = market_payload

    if journal is not None:
        journal.flush()
        payload["journal_summary"] = journal.summarize()

    return payload


def run_overlap_data(config: dict) -> dict:
    provider = config.get("provider", "coinbase")
    granularity = int(config.get("granularity_sec", 300))
    ov = config.get("overlap", {})
    before = int(ov.get("candles_before", 0))
    after = int(ov.get("candles_after", 120))
    limit = before + after + 1

    manual_rows = list(ov.get("markets", [])) if isinstance(ov.get("markets"), list) else []
    auto_rows: List[dict] = []
    auto_diagnostics: List[str] = []
    if provider == "coinbase":
        try:
            auto_rows, auto_diagnostics = discover_recent_coinbase_listings(ov)
        except Exception as exc:
            auto_diagnostics = [f"auto-discovery failed: {exc}"]

    dedup: Dict[str, dict] = {}
    for row in manual_rows + auto_rows:
        if not isinstance(row, dict):
            continue
        product = str(row.get("product_id", "")).strip()
        if not product:
            continue
        if product not in dedup:
            dedup[product] = dict(row)

    series: Dict[str, List[float]] = {}
    summary_rows = []
    errors = []

    for row in dedup.values():
        product = row.get("product_id")
        label = row.get("label") or product
        anchor_time = row.get("anchor_time")
        source = row.get("source") or "manual"

        try:
            if anchor_time:
                anchor = parse_iso_utc(str(anchor_time))
                start = anchor - dt.timedelta(seconds=before * granularity)
                end = anchor + dt.timedelta(seconds=after * granularity)
                candles = fetch_candles_retry(
                    provider,
                    product,
                    granularity,
                    limit,
                    start=start,
                    end=end,
                )
            else:
                candles = fetch_candles_retry(provider, product, granularity, limit)
        except Exception as exc:
            errors.append({"market": product, "error": str(exc)})
            continue

        if len(candles) < 5:
            errors.append({"market": product, "error": "not enough candles"})
            continue

        anchor_idx = before if len(candles) > before else 0
        anchor_close = candles[anchor_idx].close
        norm = [c.close / anchor_close for c in candles]
        series[label] = norm

        def rel(i: int) -> float:
            j = min(anchor_idx + i, len(candles) - 1)
            return candles[j].close / anchor_close - 1.0

        summary_rows.append(
            {
                "label": label,
                "product_id": product,
                "source": source,
                "anchor_price": anchor_close,
                "anchor_time": iso_utc_from_ms(candles[anchor_idx].ts_ms),
                "ret_1": rel(1),
                "ret_3": rel(3),
                "ret_10": rel(10),
            }
        )

    return {
        "meta": {
            "provider": provider,
            "granularity_sec": granularity,
            "before": before,
            "after": after,
            "manual_markets": len(manual_rows),
            "auto_markets": len(auto_rows),
        },
        "series": series,
        "summary_rows": summary_rows,
        "errors": errors,
        "auto_discovery": {
            "markets": auto_rows,
            "diagnostics": auto_diagnostics,
        },
    }


def optimize_window_slices(total_bars: int, splits: int) -> List[Tuple[int, int]]:
    if total_bars <= 0:
        return []
    splits = max(1, splits)
    if splits == 1:
        return [(0, total_bars)]

    window = max(2, total_bars // splits)
    slices: List[Tuple[int, int]] = []
    start = 0
    for idx in range(splits):
        end = total_bars if idx == splits - 1 else min(total_bars, start + window)
        if end - start >= 2:
            slices.append((start, end))
        start = end
        if start >= total_bars:
            break
    return slices


def evaluate_risk_gate(
    per_market: Sequence[dict],
    *,
    max_drawdown: float,
    max_trade_rate: float,
    min_total_return: float,
    min_sharpe_like: Optional[float],
) -> Tuple[dict, List[str]]:
    failures: List[dict] = []
    rejection_reasons: List[str] = []

    for row in per_market:
        market = str(row.get("market", "unknown"))
        metrics = row.get("metrics", {})
        drawdown = float(metrics.get("max_drawdown", 0.0))
        trades = float(metrics.get("trades", 0.0))
        bars = max(1.0, float(metrics.get("bars", 0.0)))
        trade_rate = float(row.get("trade_rate", trades / bars))
        total_return = float(metrics.get("total_return", 0.0))
        sharpe_like = float(metrics.get("sharpe_like", 0.0))

        market_failures: List[str] = []
        if drawdown > max_drawdown:
            market_failures.append("max_drawdown_exceeded")
        if trade_rate > max_trade_rate:
            market_failures.append("trade_rate_exceeded")
        if total_return < min_total_return:
            market_failures.append("return_below_minimum")
        if min_sharpe_like is not None and sharpe_like < min_sharpe_like:
            market_failures.append("sharpe_like_below_minimum")

        if market_failures:
            failures.append(
                {
                    "market": market,
                    "failures": market_failures,
                    "metrics": {
                        "max_drawdown": drawdown,
                        "trade_rate": trade_rate,
                        "total_return": total_return,
                        "sharpe_like": sharpe_like,
                    },
                }
            )
            for failure in market_failures:
                rejection_reasons.append(f"risk:{failure}@{market}")

    return (
        {
            "status": "pass" if not failures else "fail",
            "thresholds": {
                "max_drawdown": max_drawdown,
                "max_trade_rate": max_trade_rate,
                "min_total_return": min_total_return,
                "min_sharpe_like": min_sharpe_like,
            },
            "failures": failures,
        },
        rejection_reasons,
    )


def build_promotion_gate(
    risk_gate: dict,
    rejection_reasons: Sequence[str],
    requires_replay_acceptance: bool,
) -> dict:
    eligible = risk_gate.get("status") == "pass" and not rejection_reasons
    reason_codes = [reason.split("@", 1)[0].replace("risk:", "") for reason in rejection_reasons]
    return {
        "status": "eligible_for_manual_review" if eligible else "blocked",
        "requires_replay_acceptance": requires_replay_acceptance,
        "requires_manual_promotion": True,
        "reason_codes": reason_codes,
    }


def objective_meta(config: dict, variants_considered: Sequence[str], markets_considered: Sequence[str]) -> dict:
    optimize = config.get("optimize", {})
    return {
        "formula": "net_return_after_costs - drawdown_penalty - turnover_penalty - stability_penalty",
        "weights": {
            "drawdown_penalty": float(optimize.get("drawdown_penalty", 0.8)),
            "turnover_penalty": float(optimize.get("turnover_penalty", 0.2)),
            "stability_penalty": float(optimize.get("stability_penalty", 0.35)),
        },
        "risk_thresholds": {
            "max_drawdown": float(optimize.get("max_drawdown", 0.35)),
            "max_trade_rate": float(optimize.get("max_trade_rate", 0.2)),
            "min_total_return": float(optimize.get("min_total_return", -0.05)),
            "min_sharpe_like": (
                None
                if optimize.get("min_sharpe_like") is None
                else float(optimize.get("min_sharpe_like", 0.0))
            ),
        },
        "bounds": {
            "max_candidates": int(optimize.get("max_candidates", 200)),
            "max_markets": int(optimize.get("max_markets", max(1, len(markets_considered)))),
            "max_variants": int(optimize.get("max_variants", max(1, len(variants_considered)))),
            "max_runtime_sec": (
                None
                if optimize.get("max_runtime_sec") is None
                else float(optimize.get("max_runtime_sec"))
            ),
            "stability_splits": int(optimize.get("stability_splits", 3)),
        },
        "notes": [
            "total_return already includes modeled fees/slippage from the backtest engine",
            "risk failures are hard disqualifiers for promotion eligibility",
            "promotion still requires replay acceptance after optimize ranking",
        ],
    }


def _score_candidate(
    short_window: int,
    long_window: int,
    variant: dict,
    market_candles: Dict[str, List[Candle]],
    strategy: dict,
    drawdown_penalty: float,
    turnover_penalty: float,
    stability_penalty_weight: float,
    stability_splits: int,
    min_gap: int,
) -> Optional[Tuple[float, dict]]:
    """Evaluate a single (short_window, long_window) candidate across all markets for a variant.

    Returns ``(final_score, ranking_entry)`` or ``None`` if the candidate is
    invalid (e.g. gap constraint violated or no markets have enough bars).
    """
    if long_window <= short_window + min_gap:
        return None

    bias_gain = float(variant.get("bias_gain", 0.0))
    plugins = variant.get("plugins", [])

    per_market = []
    for market, candles in market_candles.items():
        if len(candles) < long_window + 2:
            continue

        combined_bias, _, _ = build_combined_bias_series(candles, plugins)
        bias_series = combined_bias if (plugins and abs(bias_gain) > 1e-12) else None

        bt_res = backtest_sma_crossover(
            candles,
            short_window=short_window,
            long_window=long_window,
            fee_bps=strategy["fee_bps"],
            slippage_bps=strategy["slippage_bps"],
            starting_equity=strategy["starting_equity"],
            bias_series=bias_series,
            bias_gain=bias_gain,
        )
        metrics = bt_res["metrics"]
        trade_rate = metrics["trades"] / max(1, metrics["bars"])

        window_scores = []
        window_returns = []
        for start, end in optimize_window_slices(len(candles), stability_splits):
            window_candles = candles[start:end]
            if len(window_candles) < long_window + 2:
                continue
            window_bias = bias_series[start:end] if bias_series is not None else None
            window_result = backtest_sma_crossover(
                window_candles,
                short_window=short_window,
                long_window=long_window,
                fee_bps=strategy["fee_bps"],
                slippage_bps=strategy["slippage_bps"],
                starting_equity=strategy["starting_equity"],
                bias_series=window_bias,
                bias_gain=bias_gain,
            )
            window_metrics = window_result["metrics"]
            window_trade_rate = window_metrics["trades"] / max(1, window_metrics["bars"])
            window_score = (
                window_metrics["total_return"]
                - drawdown_penalty * window_metrics["max_drawdown"]
                - turnover_penalty * window_trade_rate
            )
            window_scores.append(window_score)
            window_returns.append(float(window_metrics["total_return"]))

        score = (
            metrics["total_return"]
            - drawdown_penalty * metrics["max_drawdown"]
            - turnover_penalty * trade_rate
        )
        score_stddev = statistics.pstdev(window_scores) if len(window_scores) > 1 else 0.0
        return_stddev = statistics.pstdev(window_returns) if len(window_returns) > 1 else 0.0
        stability_penalty = stability_penalty_weight * score_stddev
        per_market.append(
            {
                "market": market,
                "score": score,
                "metrics": metrics,
                "trade_rate": trade_rate,
                "window_scores": window_scores,
                "window_returns": window_returns,
                "stability": {
                    "splits_requested": stability_splits,
                    "splits_evaluated": len(window_scores),
                    "score_stddev": score_stddev,
                    "return_stddev": return_stddev,
                    "positive_windows": sum(1 for value in window_returns if value > 0),
                    "penalty": stability_penalty,
                },
            }
        )

    if not per_market:
        return None

    avg_score = statistics.fmean(x["score"] for x in per_market)
    avg_return = statistics.fmean(x["metrics"]["total_return"] for x in per_market)
    avg_drawdown = statistics.fmean(x["metrics"]["max_drawdown"] for x in per_market)
    avg_trades = statistics.fmean(x["metrics"]["trades"] for x in per_market)
    avg_trade_rate = statistics.fmean(x["trade_rate"] for x in per_market)
    stability_score_stddev = statistics.fmean(x["stability"]["score_stddev"] for x in per_market)
    stability_return_stddev = statistics.fmean(x["stability"]["return_stddev"] for x in per_market)
    stability_penalty = stability_penalty_weight * stability_score_stddev
    final_score = avg_score - stability_penalty

    entry = {
        "variant": variant["name"],
        "params": {"short_window": short_window, "long_window": long_window},
        "score": final_score,
        "avg_return": avg_return,
        "avg_drawdown": avg_drawdown,
        "avg_trades": avg_trades,
        "avg_trade_rate": avg_trade_rate,
        "market_count": len(per_market),
        "objective_breakdown": {
            "net_return_after_costs": avg_return,
            "drawdown_penalty": drawdown_penalty * avg_drawdown,
            "turnover_penalty": turnover_penalty * avg_trade_rate,
            "stability_penalty": stability_penalty,
            "final_score": final_score,
        },
        "stability": {
            "splits_requested": stability_splits,
            "score_stddev": stability_score_stddev,
            "return_stddev": stability_return_stddev,
            "penalty": stability_penalty,
            "positive_windows": sum(x["stability"]["positive_windows"] for x in per_market),
        },
        "per_market": per_market,
        # risk/promotion fields are filled in by the caller
        "_stability_score_stddev": stability_score_stddev,
        "_stability_return_stddev": stability_return_stddev,
    }
    return final_score, entry


def run_optuna_optimization(
    config: dict,
    market_candles: Dict[str, List[Candle]],
    strategy: dict,
    variant: dict,
    short_windows: List[int],
    long_windows: List[int],
    min_gap: int,
    drawdown_penalty: float,
    turnover_penalty: float,
    stability_penalty_weight: float,
    stability_splits: int,
    n_trials: int,
    seed: int = 42,
) -> List[Tuple[float, dict]]:
    """Bayesian (TPE) parameter search via Optuna.

    Returns a list of ``(final_score, ranking_entry)`` tuples for every trial
    that produced a valid result, sorted descending by score.
    """
    import optuna  # already confirmed available at call site

    short_lo, short_hi = min(short_windows), max(short_windows)
    long_lo, long_hi = min(long_windows), max(long_windows)

    results: List[Tuple[float, dict]] = []

    def objective(trial: "optuna.Trial") -> float:  # type: ignore[name-defined]
        short_window = trial.suggest_int("short_window", short_lo, short_hi)
        long_window = trial.suggest_int("long_window", long_lo, long_hi)

        result = _score_candidate(
            short_window=short_window,
            long_window=long_window,
            variant=variant,
            market_candles=market_candles,
            strategy=strategy,
            drawdown_penalty=drawdown_penalty,
            turnover_penalty=turnover_penalty,
            stability_penalty_weight=stability_penalty_weight,
            stability_splits=stability_splits,
            min_gap=min_gap,
        )
        if result is None:
            return float("-inf")

        final_score, entry = result
        results.append((final_score, entry))
        return final_score

    study = optuna.create_study(
        direction="maximize",
        sampler=optuna.samplers.TPESampler(seed=seed),
    )
    study.optimize(objective, n_trials=n_trials, show_progress_bar=False)

    results.sort(key=lambda t: t[0], reverse=True)
    return results


def _apply_gates_to_entry(
    entry: dict,
    per_market: List[dict],
    stability_score_stddev: float,
    stability_return_stddev: float,
    max_drawdown: float,
    max_trade_rate: float,
    min_total_return: float,
    min_sharpe_like: Optional[float],
    requires_replay_acceptance: bool,
) -> dict:
    """Attach risk_gate, promotion_gate and rejection_reasons to a ranking entry in-place."""
    rejection_reasons: List[str] = []
    risk_gate, risk_rejections = evaluate_risk_gate(
        per_market,
        max_drawdown=max_drawdown,
        max_trade_rate=max_trade_rate,
        min_total_return=min_total_return,
        min_sharpe_like=min_sharpe_like,
    )
    rejection_reasons.extend(risk_rejections)
    if stability_score_stddev > 0.12:
        rejection_reasons.append("stability:score_stddev_above_threshold")
    if stability_return_stddev > 0.12:
        rejection_reasons.append("stability:return_stddev_above_threshold")
    promotion_gate = build_promotion_gate(
        risk_gate,
        rejection_reasons,
        requires_replay_acceptance=requires_replay_acceptance,
    )
    final_score = entry["score"]
    ranking_score = final_score if not rejection_reasons else final_score - 1000.0
    entry["risk_gate"] = risk_gate
    entry["promotion_gate"] = promotion_gate
    entry["rejection_reasons"] = rejection_reasons
    entry["ranking_score"] = ranking_score
    # Remove internal helpers that were stored by _score_candidate
    entry.pop("_stability_score_stddev", None)
    entry.pop("_stability_return_stddev", None)
    return entry


def run_optimize_data(config: dict) -> dict:
    provider = config.get("provider", "coinbase")
    granularity = int(config.get("granularity_sec", 300))
    backtest = config.get("backtest", {})
    markets = backtest.get("markets", [])
    limit = int(backtest.get("limit", 300))

    # Resolve search strategy: tpe (default) or random/grid.
    search_strategy = str(config.get("_search", "tpe")).lower()
    use_tpe = search_strategy == "tpe"
    if use_tpe and not OPTUNA_AVAILABLE:
        import warnings
        warnings.warn(
            "Optuna is not installed — falling back to grid search. "
            "Install with: pip install optuna",
            stacklevel=2,
        )
        use_tpe = False

    strategy = strategy_settings(config)
    variants = resolve_variants(config)

    optimize = config.get("optimize", {})
    short_windows = [int(x) for x in optimize.get("short_windows", [5, 7, 9, 12, 15])]
    long_windows = [int(x) for x in optimize.get("long_windows", [21, 34, 55, 89])]
    min_gap = int(optimize.get("min_gap", 2))
    top_n = int(optimize.get("top_n", 15))
    drawdown_penalty = float(optimize.get("drawdown_penalty", 0.8))
    turnover_penalty = float(optimize.get("turnover_penalty", 0.2))
    stability_penalty_weight = float(optimize.get("stability_penalty", 0.35))
    stability_splits = max(1, int(optimize.get("stability_splits", 3)))
    max_candidates = max(1, int(optimize.get("max_candidates", 200)))
    max_markets = max(1, int(optimize.get("max_markets", max(1, len(markets)))))
    max_variants = max(1, int(optimize.get("max_variants", max(1, len(variants)))))
    max_runtime_sec = (
        None if optimize.get("max_runtime_sec") is None else max(0.01, float(optimize.get("max_runtime_sec")))
    )
    max_drawdown = float(optimize.get("max_drawdown", 0.35))
    max_trade_rate = float(optimize.get("max_trade_rate", 0.2))
    min_total_return = float(optimize.get("min_total_return", -0.05))
    min_sharpe_like = (
        None if optimize.get("min_sharpe_like") is None else float(optimize.get("min_sharpe_like", 0.0))
    )
    requires_replay_acceptance = bool(optimize.get("requires_replay_acceptance", True))

    bounded_markets = [str(m).strip() for m in markets[:max_markets] if str(m).strip()]
    bounded_variants = variants[:max_variants]
    objective_descriptor = objective_meta(config, [v["name"] for v in bounded_variants], bounded_markets)
    started_at = time.monotonic()

    market_candles: Dict[str, List[Candle]] = {}
    errors = []
    for market in bounded_markets:
        try:
            candles = fetch_candles_retry(provider, market, granularity, limit)
        except Exception as exc:
            errors.append({"market": market, "error": str(exc)})
            continue
        market_candles[market] = candles

    rankings: List[dict] = []
    candidate_count = 0
    runtime_limited = False

    # ------------------------------------------------------------------ #
    # Helper: finalise a raw entry from _score_candidate into a full      #
    # ranking row with gates, then append it to rankings.                 #
    # ------------------------------------------------------------------ #
    def _finalise_and_append(final_score: float, entry: dict) -> None:
        nonlocal candidate_count
        per_market_rows = entry.get("per_market", [])
        stability_score_stddev = entry.pop("_stability_score_stddev", 0.0)
        stability_return_stddev = entry.pop("_stability_return_stddev", 0.0)
        _apply_gates_to_entry(
            entry=entry,
            per_market=per_market_rows,
            stability_score_stddev=stability_score_stddev,
            stability_return_stddev=stability_return_stddev,
            max_drawdown=max_drawdown,
            max_trade_rate=max_trade_rate,
            min_total_return=min_total_return,
            min_sharpe_like=min_sharpe_like,
            requires_replay_acceptance=requires_replay_acceptance,
        )
        rankings.append(entry)
        candidate_count += 1

    for variant in bounded_variants:
        if use_tpe:
            # ---------------------------------------------------------- #
            # Bayesian (TPE) path via Optuna                              #
            # ---------------------------------------------------------- #
            n_trials = max_candidates  # honour the same budget
            tpe_results = run_optuna_optimization(
                config=config,
                market_candles=market_candles,
                strategy=strategy,
                variant=variant,
                short_windows=short_windows,
                long_windows=long_windows,
                min_gap=min_gap,
                drawdown_penalty=drawdown_penalty,
                turnover_penalty=turnover_penalty,
                stability_penalty_weight=stability_penalty_weight,
                stability_splits=stability_splits,
                n_trials=n_trials,
                seed=42,
            )
            for final_score, entry in tpe_results:
                if max_runtime_sec is not None and (time.monotonic() - started_at) >= max_runtime_sec:
                    runtime_limited = True
                    break
                _finalise_and_append(final_score, entry)
            if runtime_limited:
                break
        else:
            # ---------------------------------------------------------- #
            # Grid-search path (original behaviour)                       #
            # ---------------------------------------------------------- #
            for short_window in short_windows:
                for long_window in long_windows:
                    if candidate_count >= max_candidates:
                        break
                    if max_runtime_sec is not None and (time.monotonic() - started_at) >= max_runtime_sec:
                        runtime_limited = True
                        break

                    result = _score_candidate(
                        short_window=short_window,
                        long_window=long_window,
                        variant=variant,
                        market_candles=market_candles,
                        strategy=strategy,
                        drawdown_penalty=drawdown_penalty,
                        turnover_penalty=turnover_penalty,
                        stability_penalty_weight=stability_penalty_weight,
                        stability_splits=stability_splits,
                        min_gap=min_gap,
                    )
                    if result is None:
                        continue
                    final_score, entry = result
                    _finalise_and_append(final_score, entry)

                if candidate_count >= max_candidates or runtime_limited:
                    break
            if candidate_count >= max_candidates or runtime_limited:
                break

    rankings.sort(key=lambda x: x["ranking_score"], reverse=True)
    top = rankings[:top_n]
    for idx, row in enumerate(top, start=1):
        row["rank"] = idx
        row.pop("ranking_score", None)

    return {
        "meta": {
            "provider": provider,
            "granularity_sec": granularity,
            "markets": list(market_candles.keys()),
            "variants": [v["name"] for v in bounded_variants],
            "top_n": top_n,
            "objective": objective_descriptor,
            "candidate_count": candidate_count,
            "search_strategy": "tpe" if use_tpe else "grid",
            "bounded_search": {
                "max_candidates": max_candidates,
                "max_markets": max_markets,
                "max_variants": max_variants,
                "max_runtime_sec": max_runtime_sec,
                "runtime_limited": runtime_limited,
                "elapsed_sec": round(time.monotonic() - started_at, 6),
            },
        },
        "top": top,
        "errors": errors,
    }


def html_shell(title: str, body: str, data: dict, script: str) -> str:
    payload = json.dumps(data)
    return (
        "<!doctype html>\n"
        "<html><head><meta charset=\"utf-8\" />"
        "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />"
        f"<title>{title}</title>"
        "<style>"
        ":root{--bg:#07111f;--card:#0f172a;--ink:#e5e7eb;--muted:#94a3b8;--grid:#334155;}"
        "body{margin:0;background:var(--bg);color:var(--ink);font-family:ui-monospace,Menlo,Consolas,monospace;}"
        ".wrap{max-width:1300px;margin:0 auto;padding:14px;}"
        ".card{background:var(--card);border:1px solid #1f2937;border-radius:10px;padding:12px;margin-bottom:12px;}"
        ".grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(220px,1fr));gap:10px;}"
        ".table{width:100%;border-collapse:collapse;}"
        ".table td,.table th{border-bottom:1px solid #1f2937;padding:6px 8px;text-align:left;font-size:12px;}"
        "canvas{width:100%;height:280px;background:#060d18;border:1px solid #1f2937;border-radius:8px;}"
        "select{background:#060d18;color:var(--ink);border:1px solid #334155;border-radius:6px;padding:6px;}"
        ".hint{color:var(--muted);font-size:12px;}"
        ".error{color:#f87171;font-size:12px;}"
        "</style></head><body>"
        f"<div class=\"wrap\">{body}</div>"
        f"<script>const DATA = {payload};{script}</script>"
        "</body></html>"
    )


def common_chart_js() -> str:
    return (
        "function fmt(n,d=6){const v=Number(n);return Number.isFinite(v)?v.toFixed(d):'-';}\n"
        "function pct(n){return `${fmt(n*100,2)}%`;}\n"
        "function drawSeries(id,seriesMap){const c=document.getElementById(id);if(!c)return;const ctx=c.getContext('2d');ctx.clearRect(0,0,c.width,c.height);"
        "const names=Object.keys(seriesMap||{});const all=[];names.forEach(k=>(seriesMap[k]||[]).forEach(v=>{if(v!==null&&v!==undefined)all.push(v);}));if(all.length<2)return;"
        "const min=Math.min(...all),max=Math.max(...all),span=(max-min)||1;const colors=['#22c55e','#60a5fa','#f59e0b','#ef4444','#a78bfa','#14b8a6','#eab308'];"
        "ctx.strokeStyle='#334155';for(let i=0;i<5;i++){const y=20+i*((c.height-40)/4);ctx.beginPath();ctx.moveTo(0,y);ctx.lineTo(c.width,y);ctx.stroke();}"
        "names.forEach((name,idx)=>{const vals=seriesMap[name]||[];if(vals.length<2)return;ctx.strokeStyle=colors[idx%colors.length];ctx.lineWidth=2;ctx.beginPath();let started=false;"
        "vals.forEach((v,i)=>{if(v===null||v===undefined)return;const x=(vals.length>1?(i/(vals.length-1))*(c.width-20)+10:c.width/2);const y=c.height-20-((v-min)/span)*(c.height-40);if(!started){ctx.moveTo(x,y);started=true;}else{ctx.lineTo(x,y);}});ctx.stroke();});}\n"
    )


def render_backtest_html(payload: dict, out_path: pathlib.Path) -> None:
    body = """
<h2>Coinbase Strategy Lab: Backtest</h2>
<div class="card hint">Baseline + plugin variants are selectable per market.</div>
<div class="card">
  <label>Market: </label><select id="market"></select>
  <label style="margin-left:12px;">Variant: </label><select id="variant"></select>
</div>
<div class="card"><div id="snapshot" class="grid"></div></div>
<div class="card"><div>Price + SMA</div><canvas id="price" width="1180" height="300"></canvas></div>
<div class="card"><div>Equity Curve</div><canvas id="equity" width="1180" height="220"></canvas></div>
<div class="card"><div>Metrics</div><table class="table" id="metrics"></table></div>
<div class="card"><div>Attribution</div><table class="table" id="attrib"></table></div>
<div class="card"><div>Trades (last 50)</div><table class="table" id="trades"></table></div>
<div class="card"><div>Journal Summary (persistent)</div><table class="table" id="journal"></table></div>
<div class="card error" id="errors"></div>
"""

    script = (
        common_chart_js()
        + "const marketEl=document.getElementById('market');const variantEl=document.getElementById('variant');"
        "function marketObj(){return DATA.markets?.[marketEl.value]||null;}"
        "function variantObj(){const m=marketObj();if(!m)return null;return m.variants?.[variantEl.value]||null;}"
        "function renderVariantOptions(){const m=marketObj();variantEl.innerHTML='';if(!m)return;const variants=Object.keys(m.variants||{});variants.forEach(v=>{const o=document.createElement('option');o.value=v;o.textContent=v;variantEl.appendChild(o);});"
        "const preferred=m.default_variant&&m.variants[m.default_variant]?m.default_variant:(variants[0]||'');if(preferred){variantEl.value=preferred;}render();}"
        "function render(){const m=marketObj();const v=variantObj();if(!m||!v)return;"
        "drawSeries('price',{close:m.close,sma_short:v.short_sma,sma_long:v.long_sma});"
        "drawSeries('equity',{equity:v.equity_curve});"
        "const s=m.market_stats||{};document.getElementById('snapshot').innerHTML=`"
        "<div><b>Current Price</b><br/>${fmt(s.current_price,6)}</div>"
        "<div><b>Window Return</b><br/>${pct(s.range_return||0)}</div>"
        "<div><b>Bars</b><br/>${fmt(s.bars||0,0)}</div>"
        "<div><b>Last Volume</b><br/>${fmt(s.volume_last||0,4)}</div>`;"
        "document.getElementById('metrics').innerHTML=Object.entries(v.metrics||{}).map(([k,val])=>`<tr><td>${k}</td><td>${fmt(val,6)}</td></tr>`).join('');"
        "document.getElementById('attrib').innerHTML=Object.entries(v.attribution||{}).map(([k,val])=>`<tr><td>${k}</td><td>${fmt(val,6)}</td></tr>`).join('');"
        "const th='<tr><th>#</th><th>side</th><th>price</th><th>delta</th><th>bias</th><th>bar</th></tr>';"
        "const rows=(v.trades||[]).slice(-50).map((t,i)=>`<tr><td>${i+1}</td><td>${t.side}</td><td>${fmt(t.price,6)}</td><td>${fmt(t.delta,4)}</td><td>${fmt(t.bias,4)}</td><td>${t.idx}</td></tr>`).join('');"
        "document.getElementById('trades').innerHTML=th+rows;"
        "const errRows=[...(DATA.errors||[])];(m.variant_errors||[]).forEach(e=>errRows.push({market:marketEl.value,error:`${e.variant}: ${(e.errors||[]).join('; ')}`}));"
        "document.getElementById('errors').innerHTML=errRows.map(e=>`${e.market}: ${e.error}`).join('<br/>')||'none';"
        "const jHead='<tr><th>market</th><th>variant</th><th>runs</th><th>avg_return</th><th>avg_drawdown</th><th>avg_pnl</th><th>last_ts</th></tr>';"
        "const jRows=(DATA.journal_summary||[]).map(r=>`<tr><td>${r.market}</td><td>${r.variant}</td><td>${r.runs}</td><td>${fmt(r.avg_total_return,6)}</td><td>${fmt(r.avg_max_drawdown,6)}</td><td>${fmt(r.avg_pnl_abs,4)}</td><td>${r.last_ts_ms}</td></tr>`).join('');"
        "document.getElementById('journal').innerHTML=jHead+jRows;"
        "}"
        "Object.keys(DATA.markets||{}).forEach(m=>{const o=document.createElement('option');o.value=m;o.textContent=m;marketEl.appendChild(o);});"
        "marketEl.addEventListener('change',renderVariantOptions);variantEl.addEventListener('change',render);"
        "if(marketEl.options.length>0){marketEl.value=marketEl.options[0].value;renderVariantOptions();}"
    )

    out_path.write_text(html_shell("Strategy Lab Backtest", body, payload, script), encoding="utf-8")


def render_overlap_html(payload: dict, out_path: pathlib.Path) -> None:
    body = """
<h2>Coinbase Listing Overlap (Candle-Aligned)</h2>
<div class="card hint">X-axis is candle offset from anchor (0), intentionally not timestamp-aligned.</div>
<div class="card"><canvas id="overlap" width="1180" height="340"></canvas></div>
<div class="card" id="summary"></div>
<div class="card"><div>Auto-Discovery</div><div id="auto" class="hint"></div></div>
<div class="card error" id="errors"></div>
"""

    script = (
        common_chart_js()
        + "drawSeries('overlap',DATA.series||{});"
        "document.getElementById('summary').innerHTML=(DATA.summary_rows||[]).map(r=>`${r.label} (${r.source}): anchor=${fmt(r.anchor_price,6)}, +1=${pct(r.ret_1)}, +3=${pct(r.ret_3)}, +10=${pct(r.ret_10)}`).join('<br/>')||'none';"
        "const ad=DATA.auto_discovery||{};"
        "const mk=(ad.markets||[]).map(m=>`${m.label} (${m.product_id}) @ ${m.anchor_time}`).join('<br/>');"
        "const dg=(ad.diagnostics||[]).join('<br/>');"
        "document.getElementById('auto').innerHTML=(mk||'no auto markets') + (dg?'<hr/>'+dg:'');"
        "document.getElementById('errors').innerHTML=(DATA.errors||[]).map(e=>`${e.market}: ${e.error}`).join('<br/>')||'none';"
    )

    out_path.write_text(html_shell("Strategy Lab Overlap", body, payload, script), encoding="utf-8")


def render_optimize_html(payload: dict, out_path: pathlib.Path) -> None:
    body = """
<h2>Coinbase Strategy Lab: Optimization</h2>
<div class="card hint">Objective = net return after modeled costs - drawdown penalty - turnover penalty - stability penalty. Risk failures block promotion eligibility.</div>
<div class="card"><table class="table" id="rankings"></table></div>
<div class="card error" id="errors"></div>
"""

    script = (
        common_chart_js()
        + "const head='<tr><th>rank</th><th>variant</th><th>short</th><th>long</th><th>score</th><th>stability</th><th>risk</th><th>promotion</th><th>rejections</th></tr>';"
        "const rows=(DATA.top||[]).map((x,i)=>`<tr><td>${x.rank??(i+1)}</td><td>${x.variant}</td><td>${x.params.short_window}</td><td>${x.params.long_window}</td><td>${fmt(x.score,6)}</td><td>${fmt(x.stability?.penalty,6)}</td><td>${x.risk_gate?.status||'-'}</td><td>${x.promotion_gate?.status||'-'}</td><td>${(x.rejection_reasons||[]).join(', ')||'-'}</td></tr>`).join('');"
        "document.getElementById('rankings').innerHTML=head+rows;"
        "document.getElementById('errors').innerHTML=(DATA.errors||[]).map(e=>`${e.market}: ${e.error}`).join('<br/>')||'none';"
    )

    out_path.write_text(html_shell("Strategy Lab Optimization", body, payload, script), encoding="utf-8")


def render_dashboard_html(payload: dict, out_path: pathlib.Path) -> None:
    body = """
<h2>Coinbase Strategy Dashboard</h2>
<div class="card hint">Generated local dashboard with prices, variants, overlap cohorts, and optimization rankings.</div>
<div class="card"><div><b>Current Market Prices</b></div><table class="table" id="prices"></table></div>
<div class="card">
  <label>Backtest Market: </label><select id="market"></select>
  <label style="margin-left:12px;">Variant: </label><select id="variant"></select>
</div>
<div class="card"><div>Price + SMA</div><canvas id="price" width="1180" height="300"></canvas></div>
<div class="card"><div>Equity Curve</div><canvas id="equity" width="1180" height="220"></canvas></div>
<div class="card"><div>Metrics</div><table class="table" id="metrics"></table></div>
<div class="card"><div>Listing Theory Check (Candle-Aligned)</div><canvas id="overlap" width="1180" height="340"></canvas><div id="overlapSummary" class="hint"></div></div>
<div class="card"><div>Top Parameter Sets</div><table class="table" id="opt"></table></div>
<div class="card"><div>Journal Summary</div><table class="table" id="journal"></table></div>
<div class="card error" id="errors"></div>
"""

    script = (
        common_chart_js()
        + "const backtest=DATA.backtest||{};const marketEl=document.getElementById('market');const variantEl=document.getElementById('variant');"
        "function marketObj(){return backtest.markets?.[marketEl.value]||null;}"
        "function variantObj(){const m=marketObj();if(!m)return null;return m.variants?.[variantEl.value]||null;}"
        "function renderPrices(){const head='<tr><th>market</th><th>current_price</th><th>window_return</th><th>bars</th><th>last_volume</th></tr>';"
        "const rows=Object.entries(backtest.markets||{}).map(([name,m])=>{const s=m.market_stats||{};return `<tr><td>${name}</td><td>${fmt(s.current_price,6)}</td><td>${pct(s.range_return||0)}</td><td>${fmt(s.bars||0,0)}</td><td>${fmt(s.volume_last||0,4)}</td></tr>`;}).join('');"
        "document.getElementById('prices').innerHTML=head+rows;}"
        "function renderVariantOptions(){const m=marketObj();variantEl.innerHTML='';if(!m)return;const variants=Object.keys(m.variants||{});variants.forEach(v=>{const o=document.createElement('option');o.value=v;o.textContent=v;variantEl.appendChild(o);});"
        "const preferred=m.default_variant&&m.variants[m.default_variant]?m.default_variant:(variants[0]||'');if(preferred)variantEl.value=preferred;renderBacktest();}"
        "function renderBacktest(){const m=marketObj();const v=variantObj();if(!m||!v)return;drawSeries('price',{close:m.close,sma_short:v.short_sma,sma_long:v.long_sma});drawSeries('equity',{equity:v.equity_curve});"
        "document.getElementById('metrics').innerHTML=Object.entries(v.metrics||{}).map(([k,val])=>`<tr><td>${k}</td><td>${fmt(val,6)}</td></tr>`).join('');}"
        "function renderOverlap(){drawSeries('overlap',(DATA.overlap||{}).series||{});document.getElementById('overlapSummary').innerHTML=((DATA.overlap||{}).summary_rows||[]).map(r=>`${r.label} (${r.source}): +1=${pct(r.ret_1)}, +3=${pct(r.ret_3)}, +10=${pct(r.ret_10)}`).join('<br/>')||'none';}"
        "function renderOptimize(){const head='<tr><th>rank</th><th>variant</th><th>short</th><th>long</th><th>score</th><th>avg_return</th><th>avg_drawdown</th><th>avg_trades</th><th>markets</th></tr>';"
        "const rows=((DATA.optimize||{}).top||[]).map((x,i)=>`<tr><td>${i+1}</td><td>${x.variant}</td><td>${x.params.short_window}</td><td>${x.params.long_window}</td><td>${fmt(x.score,6)}</td><td>${fmt(x.avg_return,6)}</td><td>${fmt(x.avg_drawdown,6)}</td><td>${fmt(x.avg_trades,2)}</td><td>${x.market_count}</td></tr>`).join('');"
        "document.getElementById('opt').innerHTML=head+rows;}"
        "function renderJournal(){const head='<tr><th>market</th><th>variant</th><th>runs</th><th>avg_return</th><th>avg_drawdown</th><th>avg_pnl</th><th>last_ts</th></tr>';"
        "const rows=(backtest.journal_summary||[]).map(r=>`<tr><td>${r.market}</td><td>${r.variant}</td><td>${r.runs}</td><td>${fmt(r.avg_total_return,6)}</td><td>${fmt(r.avg_max_drawdown,6)}</td><td>${fmt(r.avg_pnl_abs,4)}</td><td>${r.last_ts_ms}</td></tr>`).join('');"
        "document.getElementById('journal').innerHTML=head+rows;}"
        "function renderErrors(){const rows=[];((backtest.errors)||[]).forEach(e=>rows.push(`backtest ${e.market}: ${e.error}`));(((DATA.overlap||{}).errors)||[]).forEach(e=>rows.push(`overlap ${e.market}: ${e.error}`));(((DATA.optimize||{}).errors)||[]).forEach(e=>rows.push(`optimize ${e.market}: ${e.error}`));document.getElementById('errors').innerHTML=rows.join('<br/>')||'none';}"
        "Object.keys(backtest.markets||{}).forEach(m=>{const o=document.createElement('option');o.value=m;o.textContent=m;marketEl.appendChild(o);});"
        "marketEl.addEventListener('change',renderVariantOptions);variantEl.addEventListener('change',renderBacktest);"
        "if(marketEl.options.length>0){marketEl.value=marketEl.options[0].value;renderVariantOptions();}"
        "renderPrices();renderOverlap();renderOptimize();renderJournal();renderErrors();"
    )

    out_path.write_text(html_shell("Coinbase Strategy Dashboard", body, payload, script), encoding="utf-8")


def write_json_and_html(out_dir: pathlib.Path, prefix: str, payload: dict, render_fn) -> Tuple[pathlib.Path, pathlib.Path]:
    ts = dt.datetime.now(UTC).strftime("%Y%m%d-%H%M%S")
    out_dir.mkdir(parents=True, exist_ok=True)
    json_path = out_dir / f"{prefix}-{ts}.json"
    html_path = out_dir / f"{prefix}-{ts}.html"
    json_path.write_text(json.dumps(payload, indent=2), encoding="utf-8")
    render_fn(payload, html_path)
    return json_path, html_path


def serve_directory(out_dir: pathlib.Path, port: int) -> None:
    class StaticHandler(SimpleHTTPRequestHandler):
        def __init__(self, *args, **kwargs):
            super().__init__(*args, directory=str(out_dir), **kwargs)

    print(f"serving {out_dir} at http://127.0.0.1:{port}/")
    server = ThreadingHTTPServer(("127.0.0.1", port), StaticHandler)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()


def apply_cli_overrides(config: dict, args: argparse.Namespace) -> dict:
    cfg = dict(config)
    if args.provider:
        cfg["provider"] = args.provider
    if args.granularity_sec is not None:
        cfg["granularity_sec"] = int(args.granularity_sec)

    backtest = dict(cfg.get("backtest", {}))
    strategy = dict(backtest.get("strategy", {}))

    if args.markets:
        backtest["markets"] = [x.strip() for x in args.markets.split(",") if x.strip()]
    if args.limit is not None:
        backtest["limit"] = int(args.limit)

    if args.short_window is not None:
        strategy["short_window"] = int(args.short_window)
    if args.long_window is not None:
        strategy["long_window"] = int(args.long_window)
    if args.fee_bps is not None:
        strategy["trade_fee_bps"] = float(args.fee_bps)
    if args.slippage_bps is not None:
        strategy["slippage_bps"] = float(args.slippage_bps)
    if args.starting_equity is not None:
        strategy["starting_equity"] = float(args.starting_equity)

    backtest["strategy"] = strategy
    cfg["backtest"] = backtest

    overlap = dict(cfg.get("overlap", {}))
    auto = dict(overlap.get("auto_discovery", {}))
    if args.auto_discovery:
        auto["enabled"] = True
    if args.disable_auto_discovery:
        auto["enabled"] = False
    if auto:
        overlap["auto_discovery"] = auto
    cfg["overlap"] = overlap

    journal = dict(cfg.get("journal", {}))
    if args.disable_journal:
        journal["enabled"] = False
    if args.journal_path:
        journal["sqlite_path"] = args.journal_path
    if journal:
        cfg["journal"] = journal

    return cfg


def run_mode(mode: str, config: dict, out_dir: pathlib.Path) -> List[pathlib.Path]:
    outputs: List[pathlib.Path] = []

    if mode == "backtest":
        payload = run_backtest_data(config)
        json_path, html_path = write_json_and_html(out_dir, "backtest", payload, render_backtest_html)
        outputs.extend([json_path, html_path])
    elif mode == "overlap":
        payload = run_overlap_data(config)
        json_path, html_path = write_json_and_html(out_dir, "overlap", payload, render_overlap_html)
        outputs.extend([json_path, html_path])
    elif mode == "optimize":
        payload = run_optimize_data(config)
        json_path, html_path = write_json_and_html(out_dir, "optimize", payload, render_optimize_html)
        outputs.extend([json_path, html_path])
    elif mode == "dashboard":
        backtest_payload = run_backtest_data(config)
        overlap_payload = run_overlap_data(config)
        optimize_payload = run_optimize_data(config)
        payload = {
            "meta": {
                "provider": config.get("provider", "coinbase"),
                "granularity_sec": int(config.get("granularity_sec", 300)),
                "generated_at": dt.datetime.now(UTC).isoformat(),
            },
            "backtest": backtest_payload,
            "overlap": overlap_payload,
            "optimize": optimize_payload,
        }
        json_path, html_path = write_json_and_html(out_dir, "dashboard", payload, render_dashboard_html)
        outputs.extend([json_path, html_path])
    else:
        raise ValueError(f"unsupported mode: {mode}")

    return outputs


def main() -> int:
    p = argparse.ArgumentParser(description="Coinbase strategy lab")
    p.add_argument("mode", choices=["backtest", "overlap", "optimize", "dashboard"])
    p.add_argument("--config", default="config/coinbase_strategy_lab.json")
    p.add_argument("--out", default="data/strategy_lab")

    p.add_argument("--provider", choices=["coinbase", "kraken"], default=None)
    p.add_argument("--markets", default=None, help="comma-separated market list (BTC-USD,ETH-USD,...)")
    p.add_argument("--granularity-sec", type=int, default=None)
    p.add_argument("--limit", type=int, default=None)
    p.add_argument("--short-window", type=int, default=None)
    p.add_argument("--long-window", type=int, default=None)
    p.add_argument("--fee-bps", type=float, default=None)
    p.add_argument("--slippage-bps", type=float, default=None)
    p.add_argument("--starting-equity", type=float, default=None)

    p.add_argument("--auto-discovery", action="store_true", help="enable overlap auto discovery")
    p.add_argument("--disable-auto-discovery", action="store_true", help="disable overlap auto discovery")
    p.add_argument("--disable-journal", action="store_true", help="disable persistent journal writes")
    p.add_argument("--journal-path", default=None, help="override journal sqlite path")

    p.add_argument("--serve", type=int, default=None, help="optional local static server port")
    p.add_argument(
        "--search",
        choices=["random", "tpe"],
        default="tpe",
        help=(
            "Search strategy for the 'optimize' mode: "
            "'tpe' uses Optuna Bayesian optimization (default, requires: pip install optuna), "
            "'random' uses the original grid search over configured window lists."
        ),
    )

    args = p.parse_args()

    cfg_path = pathlib.Path(args.config)
    if not cfg_path.exists():
        example = pathlib.Path("config/coinbase_strategy_lab.example.json")
        raise SystemExit(
            f"config file not found: {cfg_path}. Copy from {example} and edit markets/strategy."
        )

    config = apply_cli_overrides(read_config(str(cfg_path)), args)
    # Inject search strategy into config so run_optimize_data can read it.
    # We normalise "random" -> "grid" internally; both mean grid/exhaustive search.
    config["_search"] = "tpe" if args.search == "tpe" else "grid"
    out_dir = pathlib.Path(args.out)

    outputs = run_mode(args.mode, config, out_dir)
    for path in outputs:
        print(path)

    if args.serve is not None:
        serve_directory(out_dir, args.serve)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
