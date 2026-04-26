#!/usr/bin/env python3
"""Validate promoted replay artifacts and optional runtime evidence.

This tool is intentionally local-first and safe: it reads replay NDJSON,
optional promotion metadata, optional SQLite runtime output, and optional local
operator dashboard endpoints. It does not place orders or require credentials.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import pathlib
import sqlite3
import urllib.request
from typing import Any, Dict, List, Optional, Tuple

UTC = dt.timezone.utc


def parse_iso_ts(value: str) -> dt.datetime:
    text = value.strip()
    if text.endswith("Z"):
        text = text[:-1] + "+00:00"
    parsed = dt.datetime.fromisoformat(text)
    if parsed.tzinfo is None:
        parsed = parsed.replace(tzinfo=UTC)
    return parsed.astimezone(UTC)


def load_ndjson(path: pathlib.Path) -> List[dict]:
    rows: List[dict] = []
    with path.open("r", encoding="utf-8") as f:
        for line_no, line in enumerate(f, start=1):
            line = line.strip()
            if not line:
                continue
            try:
                row = json.loads(line)
            except json.JSONDecodeError as exc:
                raise ValueError(f"{path}:{line_no}: invalid JSON: {exc}") from exc
            if not isinstance(row, dict):
                raise ValueError(f"{path}:{line_no}: expected JSON object")
            rows.append(row)
    return rows


def validate_replay_rows(rows: List[dict], min_frames: int) -> Tuple[dict, List[str]]:
    failures: List[str] = []
    warnings: List[str] = []
    markets = set()
    tokens = set()
    last_ts: Optional[dt.datetime] = None
    min_spread = None
    max_spread = None
    min_liquidity = None
    max_abs_bias = 0.0

    if len(rows) < min_frames:
        failures.append(f"frame_count {len(rows)} < min_frames {min_frames}")

    required_snapshot = {"market_id", "token_id", "bid", "ask", "spread", "liquidity", "ts"}
    for idx, row in enumerate(rows):
        snap = row.get("snapshot")
        if not isinstance(snap, dict):
            failures.append(f"frame {idx}: missing snapshot object")
            continue

        missing = required_snapshot - set(snap)
        if missing:
            failures.append(f"frame {idx}: missing snapshot fields {sorted(missing)}")
            continue

        market_id = str(snap.get("market_id", ""))
        token_id = str(snap.get("token_id", ""))
        markets.add(market_id)
        tokens.add(token_id)

        try:
            bid = float(snap["bid"])
            ask = float(snap["ask"])
            spread = float(snap["spread"])
            liquidity = float(snap["liquidity"])
            ts = parse_iso_ts(str(snap["ts"]))
            bias = float(row.get("bias", 0.0))
        except Exception as exc:
            failures.append(f"frame {idx}: invalid numeric/timestamp field: {exc}")
            continue

        if not market_id:
            failures.append(f"frame {idx}: empty market_id")
        if not token_id:
            failures.append(f"frame {idx}: empty token_id")
        if bid <= 0 or ask <= 0:
            failures.append(f"frame {idx}: bid/ask must be positive")
        if ask < bid:
            failures.append(f"frame {idx}: ask < bid")
        if abs((ask - bid) - spread) > max(1e-8, abs(spread) * 1e-6):
            warnings.append(f"frame {idx}: spread field differs from ask-bid")
        if spread < 0:
            failures.append(f"frame {idx}: negative spread")
        if liquidity < 0:
            failures.append(f"frame {idx}: negative liquidity")
        if not -1.0 <= bias <= 1.0:
            failures.append(f"frame {idx}: bias {bias} outside [-1,1]")
        if last_ts is not None and ts < last_ts:
            failures.append(f"frame {idx}: timestamp moved backwards")
        last_ts = ts

        min_spread = spread if min_spread is None else min(min_spread, spread)
        max_spread = spread if max_spread is None else max(max_spread, spread)
        min_liquidity = liquidity if min_liquidity is None else min(min_liquidity, liquidity)
        max_abs_bias = max(max_abs_bias, abs(bias))

    summary = {
        "frames": len(rows),
        "markets": sorted(markets),
        "tokens": sorted(tokens),
        "min_spread": min_spread,
        "max_spread": max_spread,
        "min_liquidity": min_liquidity,
        "max_abs_bias": max_abs_bias,
    }
    return summary, failures + [f"warning: {w}" for w in warnings]


def read_promotion(path: pathlib.Path) -> dict:
    if not path.exists():
        raise FileNotFoundError(f"promotion file not found: {path}")
    payload = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(payload, dict):
        raise ValueError("promotion file must contain JSON object")
    return payload


def validate_promotion(promotion: dict, replay_path: pathlib.Path, replay_summary: dict) -> List[str]:
    failures: List[str] = []
    replay = promotion.get("replay", {}) if isinstance(promotion.get("replay"), dict) else {}
    declared_path = replay.get("path")
    if declared_path and pathlib.Path(str(declared_path)).name != replay_path.name:
        failures.append(f"promotion replay.path {declared_path!r} does not match {replay_path}")
    declared_frames = replay.get("frames")
    if declared_frames is not None and int(declared_frames) != int(replay_summary.get("frames", 0)):
        failures.append(
            f"promotion replay.frames {declared_frames} does not match replay rows {replay_summary.get('frames')}"
        )
    return failures


def sqlite_table_count(conn: sqlite3.Connection, table: str) -> int:
    try:
        row = conn.execute(f"SELECT COUNT(*) FROM {table}").fetchone()
    except sqlite3.Error:
        return 0
    return int(row[0] if row else 0)


def latest_risk_payload(conn: sqlite3.Connection) -> Optional[dict]:
    try:
        row = conn.execute("SELECT payload FROM risk_events ORDER BY ts_ms DESC LIMIT 1").fetchone()
    except sqlite3.Error:
        return None
    if not row:
        return None
    try:
        payload = json.loads(row[0])
    except Exception:
        return None
    return payload if isinstance(payload, dict) else None


def validate_sqlite(
    path: pathlib.Path,
    min_snapshots: int,
    min_risk_events: int,
    max_stale_books: Optional[int],
) -> Tuple[dict, List[str]]:
    failures: List[str] = []
    conn = sqlite3.connect(str(path))
    try:
        snapshots = sqlite_table_count(conn, "market_snapshots")
        executions = sqlite_table_count(conn, "execution_reports")
        risk_events = sqlite_table_count(conn, "risk_events")
        risk = latest_risk_payload(conn)
    finally:
        conn.close()

    if snapshots < min_snapshots:
        failures.append(f"sqlite market_snapshots {snapshots} < {min_snapshots}")
    if risk_events < min_risk_events:
        failures.append(f"sqlite risk_events {risk_events} < {min_risk_events}")
    if max_stale_books is not None and risk is not None:
        stale = int(risk.get("stale_books", 0))
        if stale > max_stale_books:
            failures.append(f"latest risk stale_books {stale} > {max_stale_books}")

    return {
        "market_snapshots": snapshots,
        "execution_reports": executions,
        "risk_events": risk_events,
        "latest_risk": risk,
    }, failures


def http_json(url: str) -> Any:
    with urllib.request.urlopen(url, timeout=5) as resp:  # nosec B310 - local/operator supplied URL
        text = resp.read().decode("utf-8")
    return json.loads(text)


def http_text(url: str) -> str:
    with urllib.request.urlopen(url, timeout=5) as resp:  # nosec B310 - local/operator supplied URL
        return resp.read().decode("utf-8")


def validate_dashboard(base_url: str) -> Tuple[dict, List[str]]:
    failures: List[str] = []
    base = base_url.rstrip("/")
    result: Dict[str, Any] = {}
    try:
        result["health"] = http_json(base + "/health")
    except Exception as exc:
        failures.append(f"dashboard /health failed: {exc}")
    try:
        result["risk"] = http_json(base + "/state/risk")
    except Exception as exc:
        failures.append(f"dashboard /state/risk failed: {exc}")
    try:
        metrics = http_text(base + "/metrics")
        result["metrics_bytes"] = len(metrics.encode("utf-8"))
        result["metrics_lines"] = len([line for line in metrics.splitlines() if line.strip()])
    except Exception as exc:
        failures.append(f"dashboard /metrics failed: {exc}")
    return result, failures


def main() -> int:
    p = argparse.ArgumentParser(description="Validate promoted replay artifact and optional runtime evidence")
    p.add_argument("--replay", required=True, help="Replay NDJSON path")
    p.add_argument("--promotion", default=None, help="Optional promotion JSON artifact")
    p.add_argument("--sqlite", default=None, help="Optional engine SQLite output path")
    p.add_argument("--dashboard-url", default=None, help="Optional dashboard URL, e.g. http://127.0.0.1:8080")
    p.add_argument("--min-frames", type=int, default=3)
    p.add_argument("--min-snapshots", type=int, default=0)
    p.add_argument("--min-risk-events", type=int, default=0)
    p.add_argument("--max-stale-books", type=int, default=None)
    p.add_argument("--out", default=None, help="Optional JSON summary path")
    args = p.parse_args()

    replay_path = pathlib.Path(args.replay)
    rows = load_ndjson(replay_path)
    replay_summary, failures = validate_replay_rows(rows, args.min_frames)

    summary: Dict[str, Any] = {
        "status": "pass",
        "replay": replay_summary,
        "checks": [],
        "failures": [],
    }

    if args.promotion:
        promotion = read_promotion(pathlib.Path(args.promotion))
        promotion_failures = validate_promotion(promotion, replay_path, replay_summary)
        summary["promotion"] = {
            "market": promotion.get("market"),
            "variant": promotion.get("variant"),
            "source_report": promotion.get("source_report"),
        }
        failures.extend(promotion_failures)

    if args.sqlite:
        sqlite_summary, sqlite_failures = validate_sqlite(
            pathlib.Path(args.sqlite),
            min_snapshots=int(args.min_snapshots),
            min_risk_events=int(args.min_risk_events),
            max_stale_books=args.max_stale_books,
        )
        summary["sqlite"] = sqlite_summary
        failures.extend(sqlite_failures)

    if args.dashboard_url:
        dashboard_summary, dashboard_failures = validate_dashboard(args.dashboard_url)
        summary["dashboard"] = dashboard_summary
        failures.extend(dashboard_failures)

    hard_failures = [f for f in failures if not str(f).startswith("warning:")]
    summary["checks"] = failures
    summary["failures"] = hard_failures
    summary["status"] = "fail" if hard_failures else "pass"

    if args.out:
        out = pathlib.Path(args.out)
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(json.dumps(summary, indent=2, sort_keys=True), encoding="utf-8")

    print(json.dumps(summary, indent=2, sort_keys=True))
    return 1 if hard_failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
