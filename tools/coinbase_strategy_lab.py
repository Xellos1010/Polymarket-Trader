#!/usr/bin/env python3
"""Coinbase strategy lab: backtest, overlap, optimize, and dashboard rendering.

Local-first tooling for rapid strategy iteration on Coinbase markets.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import math
import pathlib
import statistics
import time
import urllib.parse
import urllib.request
from dataclasses import dataclass
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from typing import Dict, List, Optional, Sequence, Tuple


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
        parsed = parsed.replace(tzinfo=dt.timezone.utc)
    return parsed.astimezone(dt.timezone.utc)


def http_json(url: str) -> object:
    req = urllib.request.Request(
        url,
        headers={
            "User-Agent": "Polymarket-Trader-StrategyLab/0.2",
            "Accept": "application/json",
        },
    )
    with urllib.request.urlopen(req, timeout=20) as resp:  # nosec B310
        return json.loads(resp.read().decode("utf-8"))


def fetch_coinbase_candles(
    product_id: str,
    granularity_sec: int,
    limit: int,
    start: Optional[dt.datetime] = None,
    end: Optional[dt.datetime] = None,
) -> List[Candle]:
    if start and end:
        query = {
            "granularity": str(granularity_sec),
            "start": start.isoformat().replace("+00:00", "Z"),
            "end": end.isoformat().replace("+00:00", "Z"),
        }
    else:
        query = {"granularity": str(granularity_sec)}

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
    interval_map = {60: 1, 300: 5, 900: 15, 3600: 60}
    if granularity_sec not in interval_map:
        raise ValueError(
            f"granularity {granularity_sec} not supported for kraken (try 60/300/900/3600)"
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
                time.sleep(0.3 * (idx + 1))
    raise RuntimeError(f"failed to fetch candles for {product_id}: {last_error}")


def sma(values: List[float], window: int) -> List[Optional[float]]:
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


def max_drawdown(equity: Sequence[float]) -> float:
    peak = -float("inf")
    max_dd = 0.0
    for x in equity:
        peak = max(peak, x)
        if peak > 0:
            dd = (peak - x) / peak
            max_dd = max(max_dd, dd)
    return max_dd


def backtest_sma_crossover(
    candles: List[Candle],
    short_window: int,
    long_window: int,
    fee_bps: float,
    slippage_bps: float,
    starting_equity: float,
) -> dict:
    closes = [c.close for c in candles]
    short = sma(closes, short_window)
    long = sma(closes, long_window)

    position = 0.0
    equity = starting_equity
    eq_curve = [equity]
    returns = []
    trades = []

    friction = (fee_bps + slippage_bps) / 10_000.0

    for i in range(1, len(candles)):
        s_val = short[i]
        l_val = long[i]
        target = 1.0 if (s_val is not None and l_val is not None and s_val > l_val) else 0.0
        delta = abs(target - position)
        position = target

        ret = closes[i] / closes[i - 1] - 1.0
        pnl = position * ret - friction * delta
        equity *= 1.0 + pnl
        returns.append(pnl)
        eq_curve.append(equity)

        if delta > 0:
            trades.append(
                {
                    "idx": i,
                    "ts_ms": candles[i].ts_ms,
                    "side": "BUY" if target > 0 else "SELL",
                    "price": closes[i],
                }
            )

    total_return = (equity / starting_equity - 1.0) if starting_equity > 0 else 0.0
    avg_ret = statistics.fmean(returns) if returns else 0.0
    std_ret = statistics.pstdev(returns) if len(returns) > 1 else 0.0
    sharpe_like = (avg_ret / std_ret * math.sqrt(len(returns))) if std_ret > 1e-12 else 0.0

    return {
        "equity_curve": eq_curve,
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


def run_backtest_data(config: dict) -> dict:
    provider = config.get("provider", "coinbase")
    granularity = int(config.get("granularity_sec", 300))

    bt = config.get("backtest", {})
    markets = bt.get("markets", [])
    limit = int(bt.get("limit", 300))

    st = strategy_settings(config)

    payload = {
        "meta": {
            "provider": provider,
            "granularity_sec": granularity,
            "limit": limit,
            "strategy": st,
        },
        "markets": {},
        "errors": [],
    }

    for market in markets:
        try:
            candles = fetch_candles_retry(provider, market, granularity, limit)
        except Exception as exc:
            payload["errors"].append({"market": market, "error": str(exc)})
            continue
        if len(candles) < max(st["short_window"], st["long_window"]) + 2:
            payload["errors"].append({"market": market, "error": "insufficient candle history"})
            continue

        bt_res = backtest_sma_crossover(
            candles,
            short_window=st["short_window"],
            long_window=st["long_window"],
            fee_bps=st["fee_bps"],
            slippage_bps=st["slippage_bps"],
            starting_equity=st["starting_equity"],
        )

        first_close = candles[0].close
        last_close = candles[-1].close
        period_return = (last_close / first_close - 1.0) if first_close > 0 else 0.0

        payload["markets"][market] = {
            "close": [c.close for c in candles],
            "short_sma": bt_res["short_sma"],
            "long_sma": bt_res["long_sma"],
            "equity_curve": bt_res["equity_curve"],
            "trades": bt_res["trades"],
            "metrics": bt_res["metrics"],
            "market_stats": {
                "current_price": last_close,
                "range_return": period_return,
                "bars": len(candles),
                "volume_last": candles[-1].volume,
            },
        }

    return payload


def run_overlap_data(config: dict) -> dict:
    provider = config.get("provider", "coinbase")
    granularity = int(config.get("granularity_sec", 300))
    ov = config.get("overlap", {})
    before = int(ov.get("candles_before", 0))
    after = int(ov.get("candles_after", 120))
    limit = before + after + 1

    series: Dict[str, List[float]] = {}
    summary_rows = []
    errors = []

    for row in ov.get("markets", []):
        product = row.get("product_id")
        if not product:
            continue
        label = row.get("label") or product
        anchor_time = row.get("anchor_time")

        try:
            if anchor_time:
                anchor = parse_iso_utc(anchor_time)
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
                "anchor_price": anchor_close,
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
        },
        "series": series,
        "summary_rows": summary_rows,
        "errors": errors,
    }


def run_optimize_data(config: dict) -> dict:
    provider = config.get("provider", "coinbase")
    granularity = int(config.get("granularity_sec", 300))
    backtest = config.get("backtest", {})
    markets = backtest.get("markets", [])
    limit = int(backtest.get("limit", 300))

    strategy = strategy_settings(config)
    optimize = config.get("optimize", {})

    short_windows = [int(x) for x in optimize.get("short_windows", [5, 7, 9, 12, 15])]
    long_windows = [int(x) for x in optimize.get("long_windows", [21, 34, 55, 89])]
    min_gap = int(optimize.get("min_gap", 2))
    top_n = int(optimize.get("top_n", 15))
    drawdown_penalty = float(optimize.get("drawdown_penalty", 0.8))
    turnover_penalty = float(optimize.get("turnover_penalty", 0.2))

    market_candles: Dict[str, List[Candle]] = {}
    errors = []
    for market in markets:
        try:
            candles = fetch_candles_retry(provider, market, granularity, limit)
        except Exception as exc:
            errors.append({"market": market, "error": str(exc)})
            continue
        market_candles[market] = candles

    rankings = []
    for short_window in short_windows:
        for long_window in long_windows:
            if long_window <= short_window + min_gap:
                continue

            per_market = []
            for market, candles in market_candles.items():
                if len(candles) < long_window + 2:
                    continue
                bt_res = backtest_sma_crossover(
                    candles,
                    short_window=short_window,
                    long_window=long_window,
                    fee_bps=strategy["fee_bps"],
                    slippage_bps=strategy["slippage_bps"],
                    starting_equity=strategy["starting_equity"],
                )
                metrics = bt_res["metrics"]
                trade_rate = metrics["trades"] / max(1, metrics["bars"])
                score = (
                    metrics["total_return"]
                    - drawdown_penalty * metrics["max_drawdown"]
                    - turnover_penalty * trade_rate
                )
                per_market.append(
                    {
                        "market": market,
                        "score": score,
                        "metrics": metrics,
                        "trade_rate": trade_rate,
                    }
                )

            if not per_market:
                continue

            avg_score = statistics.fmean(x["score"] for x in per_market)
            avg_return = statistics.fmean(x["metrics"]["total_return"] for x in per_market)
            avg_drawdown = statistics.fmean(x["metrics"]["max_drawdown"] for x in per_market)
            avg_trades = statistics.fmean(x["metrics"]["trades"] for x in per_market)

            rankings.append(
                {
                    "params": {"short_window": short_window, "long_window": long_window},
                    "score": avg_score,
                    "avg_return": avg_return,
                    "avg_drawdown": avg_drawdown,
                    "avg_trades": avg_trades,
                    "market_count": len(per_market),
                    "per_market": per_market,
                }
            )

    rankings.sort(key=lambda x: x["score"], reverse=True)
    top = rankings[:top_n]

    return {
        "meta": {
            "provider": provider,
            "granularity_sec": granularity,
            "markets": list(market_candles.keys()),
            "top_n": top_n,
            "drawdown_penalty": drawdown_penalty,
            "turnover_penalty": turnover_penalty,
        },
        "top": top,
        "errors": errors,
    }


def render_backtest_html(payload: dict, out_path: pathlib.Path) -> None:
    html = f"""<!doctype html>
<html>
<head>
<meta charset=\"utf-8\" />
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />
<title>Strategy Lab Backtest</title>
<style>
:root {{ --bg:#0b1220; --card:#111827; --ink:#e5e7eb; --muted:#94a3b8; --grid:#334155; }}
body {{ font-family: ui-monospace, Menlo, Consolas, monospace; background:var(--bg); color:var(--ink); margin:0; }}
.wrap {{ max-width:1240px; margin:0 auto; padding:16px; }}
.card {{ background:var(--card); border:1px solid #1f2937; border-radius:10px; padding:12px; margin-bottom:12px; }}
.grid {{ display:grid; grid-template-columns:repeat(auto-fit,minmax(220px,1fr)); gap:10px; }}
label, select {{ color:var(--ink); }}
select {{ background:var(--bg); border:1px solid var(--grid); border-radius:6px; padding:6px; }}
canvas {{ width:100%; height:260px; background:var(--bg); border:1px solid #1f2937; border-radius:8px; }}
.table {{ width:100%; border-collapse:collapse; }}
.table td, .table th {{ border-bottom:1px solid #1f2937; padding:6px 8px; text-align:left; font-size:12px; }}
.error {{ color:#f87171; font-size:12px; }}
</style>
</head>
<body>
<div class=\"wrap\">
  <h2>Coinbase Strategy Lab: Backtest</h2>
  <div class=\"card\">
    <label>Market: </label><select id=\"market\"></select>
  </div>
  <div class=\"card\"><div>Current Market Snapshot</div><div id=\"snapshot\" class=\"grid\"></div></div>
  <div class=\"card\"><div>Price + SMA</div><canvas id=\"price\" width=\"1120\" height=\"280\"></canvas></div>
  <div class=\"card\"><div>Equity Curve</div><canvas id=\"equity\" width=\"1120\" height=\"220\"></canvas></div>
  <div class=\"card\"><div>Metrics</div><table class=\"table\" id=\"metrics\"></table></div>
  <div class=\"card\"><div>Trades</div><table class=\"table\" id=\"trades\"></table></div>
  <div class=\"card\"><div>Fetch Errors</div><div id=\"errors\" class=\"error\"></div></div>
</div>
<script>
const DATA = {json.dumps(payload)};
const marketEl = document.getElementById('market');

function fmt(n, digits=6) {{
  const v = Number(n);
  return Number.isFinite(v) ? v.toFixed(digits) : '-';
}}

function pct(n) {{
  return `${{fmt(n * 100, 2)}}%`;
}}

function drawSeries(canvasId, seriesMap) {{
  const c = document.getElementById(canvasId);
  const ctx = c.getContext('2d');
  ctx.clearRect(0, 0, c.width, c.height);

  const names = Object.keys(seriesMap);
  const all = [];
  names.forEach(k => (seriesMap[k] || []).forEach(v => {{ if (v !== null) all.push(v); }}));
  if (all.length < 2) return;

  const min = Math.min(...all), max = Math.max(...all), span = (max - min) || 1;
  const colors = ['#22c55e','#60a5fa','#f59e0b','#ef4444','#a78bfa'];

  ctx.strokeStyle = '#334155';
  for (let i = 0; i < 5; i++) {{
    const y = 20 + i * ((c.height - 40) / 4);
    ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(c.width, y); ctx.stroke();
  }}

  names.forEach((name, idx) => {{
    const vals = seriesMap[name] || [];
    if (vals.length < 2) return;
    ctx.strokeStyle = colors[idx % colors.length];
    ctx.lineWidth = 2;
    ctx.beginPath();
    let started = false;
    vals.forEach((v, i) => {{
      if (v === null) return;
      const x = (i / (vals.length - 1)) * (c.width - 20) + 10;
      const y = c.height - 20 - ((v - min) / span) * (c.height - 40);
      if (!started) {{ ctx.moveTo(x, y); started = true; }} else {{ ctx.lineTo(x, y); }}
    }});
    ctx.stroke();
  }});
}}

function renderErrors() {{
  const rows = (DATA.errors || []).map(e => `${{e.market}}: ${{e.error}}`);
  document.getElementById('errors').innerHTML = rows.length ? rows.join('<br/>') : 'none';
}}

function render(market) {{
  const m = DATA.markets[market];
  if (!m) return;

  drawSeries('price', {{ close: m.close, sma_short: m.short_sma, sma_long: m.long_sma }});
  drawSeries('equity', {{ equity: m.equity_curve }});

  const metricRows = Object.entries(m.metrics)
    .map(([k,v]) => `<tr><td>${{k}}</td><td>${{fmt(v)}}</td></tr>`)
    .join('');
  document.getElementById('metrics').innerHTML = metricRows;

  const trades = m.trades || [];
  const tradeHead = '<tr><th>#</th><th>side</th><th>price</th><th>bar</th></tr>';
  const tradeRows = trades.slice(-40).map((t, idx) =>
    `<tr><td>${{idx + 1}}</td><td>${{t.side}}</td><td>${{fmt(t.price, 4)}}</td><td>${{t.idx}}</td></tr>`
  ).join('');
  document.getElementById('trades').innerHTML = tradeHead + tradeRows;

  const s = m.market_stats || {{}};
  document.getElementById('snapshot').innerHTML = `
    <div><b>Current Price</b><br/>${{fmt(s.current_price, 6)}}</div>
    <div><b>Return (window)</b><br/>${{pct(s.range_return || 0)}}</div>
    <div><b>Candles</b><br/>${{fmt(s.bars || 0, 0)}}</div>
    <div><b>Last Volume</b><br/>${{fmt(s.volume_last || 0, 4)}}</div>
  `;
}}

Object.keys(DATA.markets || {{}}).forEach(m => {{
  const o = document.createElement('option');
  o.value = m;
  o.textContent = m;
  marketEl.appendChild(o);
}});
marketEl.addEventListener('change', () => render(marketEl.value));
if (marketEl.options.length > 0) {{
  marketEl.value = marketEl.options[0].value;
  render(marketEl.value);
}}
renderErrors();
</script>
</body>
</html>
"""
    out_path.write_text(html, encoding="utf-8")


def render_overlap_html(payload: dict, out_path: pathlib.Path) -> None:
    html = f"""<!doctype html>
<html>
<head>
<meta charset=\"utf-8\" />
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />
<title>Strategy Lab Overlap</title>
<style>
body {{ font-family: ui-monospace, Menlo, Consolas, monospace; background:#0b1220; color:#e5e7eb; margin:0; }}
.wrap {{ max-width:1200px; margin:0 auto; padding:16px; }}
.card {{ background:#111827; border:1px solid #1f2937; border-radius:10px; padding:12px; margin-bottom:12px; }}
canvas {{ width:100%; height:320px; background:#0b1220; border:1px solid #1f2937; border-radius:8px; }}
.error {{ color:#f87171; font-size:12px; }}
</style>
</head>
<body>
<div class=\"wrap\">
  <h2>Coinbase Listing Overlap (Candle-Aligned)</h2>
  <div class=\"card\">X-axis is candle offset from anchor (0). This is intentionally not timestamp-aligned.</div>
  <div class=\"card\"><canvas id=\"overlap\" width=\"1100\" height=\"340\"></canvas></div>
  <div class=\"card\" id=\"summary\"></div>
  <div class=\"card error\" id=\"errors\"></div>
</div>
<script>
const DATA = {json.dumps(payload)};
const c = document.getElementById('overlap');
const ctx = c.getContext('2d');

function draw() {{
  const series = DATA.series || {{}};
  const names = Object.keys(series);
  if (!names.length) return;

  const all = [];
  names.forEach(n => series[n].forEach(v => all.push(v)));
  if (all.length < 2) return;

  const min = Math.min(...all), max = Math.max(...all), span = (max - min) || 1;
  ctx.clearRect(0,0,c.width,c.height);

  ctx.strokeStyle = '#334155';
  for (let i=0;i<5;i++) {{
    const y = 20 + i*((c.height-40)/4);
    ctx.beginPath(); ctx.moveTo(0,y); ctx.lineTo(c.width,y); ctx.stroke();
  }}

  const colors = ['#22c55e','#60a5fa','#f59e0b','#ef4444','#eab308','#a78bfa','#14b8a6'];
  names.forEach((name, idx) => {{
    const vals = series[name];
    const n = vals.length;
    ctx.strokeStyle = colors[idx % colors.length];
    ctx.lineWidth = 2;
    ctx.beginPath();
    vals.forEach((v,i) => {{
      const x = n > 1 ? (i/(n-1))*(c.width-20)+10 : c.width/2;
      const y = c.height-20-((v-min)/span)*(c.height-40);
      if (i===0) ctx.moveTo(x,y); else ctx.lineTo(x,y);
    }});
    ctx.stroke();
  }});
}}

draw();

document.getElementById('summary').innerHTML = (DATA.summary_rows || [])
  .map(r => `${{r.label}}: anchor=${{r.anchor_price.toFixed(6)}}, +1c=${{(r.ret_1*100).toFixed(2)}}%, +3c=${{(r.ret_3*100).toFixed(2)}}%, +10c=${{(r.ret_10*100).toFixed(2)}}%`)
  .join('<br/>');

document.getElementById('errors').innerHTML = (DATA.errors || [])
  .map(e => `${{e.market}}: ${{e.error}}`)
  .join('<br/>') || 'none';
</script>
</body>
</html>
"""
    out_path.write_text(html, encoding="utf-8")


def render_optimize_html(payload: dict, out_path: pathlib.Path) -> None:
    html = f"""<!doctype html>
<html>
<head>
<meta charset=\"utf-8\" />
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />
<title>Strategy Lab Optimization</title>
<style>
body {{ font-family: ui-monospace, Menlo, Consolas, monospace; background:#0b1220; color:#e5e7eb; margin:0; }}
.wrap {{ max-width:1200px; margin:0 auto; padding:16px; }}
.card {{ background:#111827; border:1px solid #1f2937; border-radius:10px; padding:12px; margin-bottom:12px; }}
.table {{ width:100%; border-collapse:collapse; }}
.table td, .table th {{ border-bottom:1px solid #1f2937; padding:6px 8px; text-align:left; font-size:12px; }}
</style>
</head>
<body>
<div class=\"wrap\">
  <h2>Coinbase Strategy Lab: Parameter Optimization</h2>
  <div class=\"card\">Objective = avg_return - drawdown_penalty*avg_drawdown - turnover_penalty*trade_rate</div>
  <div class=\"card\"><table class=\"table\" id=\"rankings\"></table></div>
  <div class=\"card\" id=\"errors\"></div>
</div>
<script>
const DATA = {json.dumps(payload)};

function fmt(n, d=6) {{
  const v = Number(n);
  return Number.isFinite(v) ? v.toFixed(d) : '-';
}}

const head = '<tr><th>rank</th><th>short</th><th>long</th><th>score</th><th>avg_return</th><th>avg_drawdown</th><th>avg_trades</th><th>markets</th></tr>';
const rows = (DATA.top || []).map((x, i) => `<tr><td>${{i+1}}</td><td>${{x.params.short_window}}</td><td>${{x.params.long_window}}</td><td>${{fmt(x.score, 6)}}</td><td>${{fmt(x.avg_return, 6)}}</td><td>${{fmt(x.avg_drawdown, 6)}}</td><td>${{fmt(x.avg_trades, 2)}}</td><td>${{x.market_count}}</td></tr>`).join('');
document.getElementById('rankings').innerHTML = head + rows;

document.getElementById('errors').innerHTML = (DATA.errors || []).map(e => `${{e.market}}: ${{e.error}}`).join('<br/>') || 'none';
</script>
</body>
</html>
"""
    out_path.write_text(html, encoding="utf-8")


def render_dashboard_html(payload: dict, out_path: pathlib.Path) -> None:
    html = f"""<!doctype html>
<html>
<head>
<meta charset=\"utf-8\" />
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />
<title>Coinbase Strategy Dashboard</title>
<style>
:root {{ --bg:#07111f; --card:#0f172a; --ink:#e5e7eb; --muted:#94a3b8; --grid:#334155; }}
body {{ margin:0; background:var(--bg); color:var(--ink); font-family: ui-monospace, Menlo, Consolas, monospace; }}
.wrap {{ max-width:1300px; margin:0 auto; padding:14px; }}
.card {{ background:var(--card); border:1px solid #1f2937; border-radius:10px; padding:12px; margin-bottom:12px; }}
.grid {{ display:grid; grid-template-columns:repeat(auto-fit,minmax(240px,1fr)); gap:10px; }}
.table {{ width:100%; border-collapse:collapse; }}
.table td, .table th {{ border-bottom:1px solid #1f2937; padding:6px 8px; text-align:left; font-size:12px; }}
canvas {{ width:100%; height:280px; background:#060d18; border:1px solid #1f2937; border-radius:8px; }}
select {{ background:#060d18; color:var(--ink); border:1px solid #334155; border-radius:6px; padding:6px; }}
.hint {{ color:var(--muted); font-size:12px; }}
</style>
</head>
<body>
<div class=\"wrap\">
  <h2>Coinbase Strategy Dashboard</h2>
  <div class=\"card hint\">Generated: {dt.datetime.now(dt.timezone.utc).isoformat()}</div>

  <div class=\"card\">
    <div><b>Current Market Prices (from latest candle)</b></div>
    <table class=\"table\" id=\"prices\"></table>
  </div>

  <div class=\"card\">
    <label>Backtest Market: </label>
    <select id=\"market\"></select>
  </div>
  <div class=\"card\"><div>Price + SMA</div><canvas id=\"price\" width=\"1180\" height=\"300\"></canvas></div>
  <div class=\"card\"><div>Equity Curve</div><canvas id=\"equity\" width=\"1180\" height=\"220\"></canvas></div>
  <div class=\"card\"><div>Backtest Metrics</div><table class=\"table\" id=\"metrics\"></table></div>

  <div class=\"card\">
    <div><b>Listing Theory Check (Candle-Aligned Overlap)</b></div>
    <div class=\"hint\">Series are normalized at anchor candle = 1.0.</div>
    <canvas id=\"overlap\" width=\"1180\" height=\"320\"></canvas>
    <div id=\"overlapSummary\" class=\"hint\"></div>
  </div>

  <div class=\"card\">
    <div><b>Top Parameter Sets</b></div>
    <table class=\"table\" id=\"opt\"></table>
  </div>

  <div class=\"card\">
    <div><b>Errors</b></div>
    <div id=\"errors\" class=\"hint\"></div>
  </div>
</div>
<script>
const DATA = {json.dumps(payload)};
const marketEl = document.getElementById('market');

function fmt(n, d=6) {{
  const v = Number(n);
  return Number.isFinite(v) ? v.toFixed(d) : '-';
}}

function pct(n) {{
  return `${{fmt(n * 100, 2)}}%`;
}}

function drawSeries(canvasId, seriesMap) {{
  const c = document.getElementById(canvasId);
  const ctx = c.getContext('2d');
  ctx.clearRect(0,0,c.width,c.height);

  const names = Object.keys(seriesMap);
  const all = [];
  names.forEach(k => (seriesMap[k] || []).forEach(v => {{ if (v !== null) all.push(v); }}));
  if (all.length < 2) return;

  const min = Math.min(...all), max = Math.max(...all), span = (max-min) || 1;
  const colors = ['#22c55e','#60a5fa','#f59e0b','#ef4444','#a78bfa','#14b8a6'];

  ctx.strokeStyle = '#334155';
  for (let i=0;i<5;i++) {{
    const y = 20 + i * ((c.height-40)/4);
    ctx.beginPath(); ctx.moveTo(0,y); ctx.lineTo(c.width,y); ctx.stroke();
  }}

  names.forEach((name, idx) => {{
    const vals = seriesMap[name] || [];
    if (vals.length < 2) return;
    ctx.strokeStyle = colors[idx % colors.length];
    ctx.lineWidth = 2;
    ctx.beginPath();
    let started = false;
    vals.forEach((v, i) => {{
      if (v === null) return;
      const x = (i/(vals.length-1))*(c.width-20)+10;
      const y = c.height-20-((v-min)/span)*(c.height-40);
      if (!started) {{ ctx.moveTo(x,y); started = true; }} else {{ ctx.lineTo(x,y); }}
    }});
    ctx.stroke();
  }});
}}

function renderPriceTable() {{
  const markets = DATA.backtest.markets || {{}};
  const head = '<tr><th>market</th><th>current_price</th><th>window_return</th><th>bars</th><th>last_volume</th></tr>';
  const rows = Object.entries(markets).map(([name, m]) => {{
    const s = m.market_stats || {{}};
    return `<tr><td>${{name}}</td><td>${{fmt(s.current_price, 6)}}</td><td>${{pct(s.range_return || 0)}}</td><td>${{fmt(s.bars || 0, 0)}}</td><td>${{fmt(s.volume_last || 0, 4)}}</td></tr>`;
  }}).join('');
  document.getElementById('prices').innerHTML = head + rows;
}}

function renderBacktest(market) {{
  const m = DATA.backtest.markets[market];
  if (!m) return;
  drawSeries('price', {{ close: m.close, sma_short: m.short_sma, sma_long: m.long_sma }});
  drawSeries('equity', {{ equity: m.equity_curve }});
  const rows = Object.entries(m.metrics || {{}}).map(([k,v]) => `<tr><td>${{k}}</td><td>${{fmt(v)}}</td></tr>`).join('');
  document.getElementById('metrics').innerHTML = rows;
}}

function renderOverlap() {{
  drawSeries('overlap', DATA.overlap.series || {{}});
  const rows = (DATA.overlap.summary_rows || []).map(
    r => `${{r.label}}: anchor=${{fmt(r.anchor_price, 6)}}, +1=${{pct(r.ret_1)}}, +3=${{pct(r.ret_3)}}, +10=${{pct(r.ret_10)}}`
  );
  document.getElementById('overlapSummary').innerHTML = rows.join('<br/>') || 'none';
}}

function renderOptimize() {{
  const head = '<tr><th>rank</th><th>short</th><th>long</th><th>score</th><th>avg_return</th><th>avg_drawdown</th><th>avg_trades</th><th>markets</th></tr>';
  const rows = (DATA.optimize.top || []).map((x, i) =>
    `<tr><td>${{i + 1}}</td><td>${{x.params.short_window}}</td><td>${{x.params.long_window}}</td><td>${{fmt(x.score, 6)}}</td><td>${{fmt(x.avg_return, 6)}}</td><td>${{fmt(x.avg_drawdown, 6)}}</td><td>${{fmt(x.avg_trades, 2)}}</td><td>${{x.market_count}}</td></tr>`
  ).join('');
  document.getElementById('opt').innerHTML = head + rows;
}}

function renderErrors() {{
  const rows = [];
  (DATA.backtest.errors || []).forEach(e => rows.push(`backtest ${{e.market}}: ${{e.error}}`));
  (DATA.overlap.errors || []).forEach(e => rows.push(`overlap ${{e.market}}: ${{e.error}}`));
  (DATA.optimize.errors || []).forEach(e => rows.push(`optimize ${{e.market}}: ${{e.error}}`));
  document.getElementById('errors').innerHTML = rows.join('<br/>') || 'none';
}}

Object.keys(DATA.backtest.markets || {{}}).forEach(m => {{
  const o = document.createElement('option');
  o.value = m;
  o.textContent = m;
  marketEl.appendChild(o);
}});
marketEl.addEventListener('change', () => renderBacktest(marketEl.value));
if (marketEl.options.length > 0) {{
  marketEl.value = marketEl.options[0].value;
  renderBacktest(marketEl.value);
}}

renderPriceTable();
renderOverlap();
renderOptimize();
renderErrors();
</script>
</body>
</html>
"""
    out_path.write_text(html, encoding="utf-8")


def write_json_and_html(
    out_dir: pathlib.Path,
    prefix: str,
    payload: dict,
    render_fn,
) -> Tuple[pathlib.Path, pathlib.Path]:
    ts = dt.datetime.now(dt.timezone.utc).strftime("%Y%m%d-%H%M%S")
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
                "generated_at": dt.datetime.now(dt.timezone.utc).isoformat(),
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
    p.add_argument("--markets", default=None, help="comma-separated market list (BTC-USD,ETH-USD,...)" )
    p.add_argument("--granularity-sec", type=int, default=None)
    p.add_argument("--limit", type=int, default=None)
    p.add_argument("--short-window", type=int, default=None)
    p.add_argument("--long-window", type=int, default=None)
    p.add_argument("--fee-bps", type=float, default=None)
    p.add_argument("--slippage-bps", type=float, default=None)
    p.add_argument("--starting-equity", type=float, default=None)
    p.add_argument("--serve", type=int, default=None, help="optional local static server port")

    args = p.parse_args()

    cfg_path = pathlib.Path(args.config)
    if not cfg_path.exists():
        example = pathlib.Path("config/coinbase_strategy_lab.example.json")
        raise SystemExit(
            f"config file not found: {cfg_path}. Copy from {example} and edit markets/strategy."
        )

    config = apply_cli_overrides(read_config(str(cfg_path)), args)
    out_dir = pathlib.Path(args.out)

    outputs = run_mode(args.mode, config, out_dir)
    for path in outputs:
        print(path)

    if args.serve is not None:
        serve_directory(out_dir, args.serve)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
