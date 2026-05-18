#!/usr/bin/env python3
"""Derive Phase 1 metrics.json from strategy-lab and local runtime artifacts."""

from __future__ import annotations

import argparse
import json
import pathlib
from typing import Any, Dict, Optional, Tuple


def read_json(path: pathlib.Path) -> Dict[str, Any]:
    payload = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(payload, dict):
        raise ValueError(f"{path} must contain a JSON object")
    return payload


def choose_market_variant(
    report: Dict[str, Any],
    promotion: Optional[Dict[str, Any]],
    market: Optional[str],
    variant: Optional[str],
) -> Tuple[str, str]:
    markets = report.get("markets", {})
    if not isinstance(markets, dict) or not markets:
        raise ValueError("report has no markets")

    selected_market = market or (promotion or {}).get("market")
    if selected_market is None:
        selected_market = sorted(markets.keys())[0]
    if selected_market not in markets:
        raise ValueError(f"market not found in report: {selected_market}")

    variants = markets[selected_market].get("variants", {})
    if not isinstance(variants, dict) or not variants:
        raise ValueError(f"market {selected_market} has no variants")

    selected_variant = variant or (promotion or {}).get("variant")
    if selected_variant is None:
        selected_variant = markets[selected_market].get("default_variant") or sorted(variants.keys())[0]
    if selected_variant not in variants:
        raise ValueError(f"variant {selected_variant} not found for market {selected_market}")

    return str(selected_market), str(selected_variant)


def build_metrics(
    *,
    report: Dict[str, Any],
    promotion: Optional[Dict[str, Any]],
    replay_acceptance: Optional[Dict[str, Any]],
    paper_soak: Optional[Dict[str, Any]],
    market: str,
    variant: str,
    max_unhedged_delta: float,
) -> Dict[str, Any]:
    strategy = report.get("meta", {}).get("strategy", {})
    starting_equity = float(strategy.get("starting_equity", 1000.0))
    fee_bps = float(strategy.get("fee_bps", 0.0))
    slippage_bps = float(strategy.get("slippage_bps", 0.0))

    variant_payload = report["markets"][market]["variants"][variant]
    metrics = variant_payload.get("metrics", {})
    attribution = variant_payload.get("attribution", {})
    trades = variant_payload.get("trades", [])

    total_return = float(metrics.get("total_return", 0.0))
    bars = int(metrics.get("bars", 0))
    pnl_abs = float(attribution.get("pnl_abs", starting_equity * total_return))
    gross_turnover = float(attribution.get("gross_turnover", 0.0))

    if not gross_turnover and isinstance(trades, list):
        gross_turnover = sum(
            abs(float(row.get("delta", 0.0))) * float(row.get("price", 0.0))
            for row in trades
            if isinstance(row, dict)
        )

    fees = gross_turnover * fee_bps / 10_000.0
    slippage = gross_turnover * slippage_bps / 10_000.0

    paper_pass = bool(paper_soak.get("pass")) if paper_soak else False
    halt_count = int(paper_soak.get("halt_count", 0)) if paper_soak else 0
    failed_probes = int(paper_soak.get("failed_probes", 0)) if paper_soak else 0
    max_abs_delta = float(paper_soak.get("max_abs_unhedged_delta", 0.0)) if paper_soak else 0.0
    replay_pass = (replay_acceptance or {}).get("status") == "pass"

    return {
        "market": market,
        "variant": variant,
        "net_pnl_after_costs": pnl_abs,
        "fees": fees,
        "slippage": slippage,
        "hedge_cost": 0.0,
        "gas_amortized": 0.0,
        "adverse_selection": 0.0,
        "daily_loss_limit_breached": False,
        "max_market_notional_breached": False,
        "max_total_open_notional_breached": False,
        "max_unhedged_delta_breached": max_abs_delta > max_unhedged_delta,
        "stale_book_breached": failed_probes > 0,
        "unexpected_auto_halt": halt_count > 0 or not paper_pass or not replay_pass,
        "derived_from": {
            "source_report": (promotion or {}).get("source_report"),
            "promotion_market": (promotion or {}).get("market"),
            "promotion_variant": (promotion or {}).get("variant"),
            "report_total_return": total_return,
            "report_bars": bars,
            "gross_turnover": gross_turnover,
            "fee_bps": fee_bps,
            "slippage_bps": slippage_bps,
            "paper_soak_pass": paper_pass,
            "paper_soak_max_abs_unhedged_delta": max_abs_delta,
            "replay_acceptance_status": (replay_acceptance or {}).get("status"),
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description="Derive Phase 1 metrics.json from local evidence")
    parser.add_argument("--report", required=True, help="strategy-lab backtest JSON report")
    parser.add_argument("--promotion", default=None, help="promotion JSON path")
    parser.add_argument("--replay-acceptance", default=None, help="replay acceptance JSON path")
    parser.add_argument("--paper-soak", default=None, help="paper soak JSON path")
    parser.add_argument("--market", default=None, help="market override")
    parser.add_argument("--variant", default=None, help="variant override")
    parser.add_argument("--max-unhedged-delta", type=float, default=10.0)
    parser.add_argument("--out", required=True, help="output metrics.json path")
    args = parser.parse_args()

    report_path = pathlib.Path(args.report)
    report = read_json(report_path)
    promotion = read_json(pathlib.Path(args.promotion)) if args.promotion else None
    replay_acceptance = (
        read_json(pathlib.Path(args.replay_acceptance)) if args.replay_acceptance else None
    )
    paper_soak = read_json(pathlib.Path(args.paper_soak)) if args.paper_soak else None

    market, variant = choose_market_variant(report, promotion, args.market, args.variant)
    payload = build_metrics(
        report=report,
        promotion=promotion,
        replay_acceptance=replay_acceptance,
        paper_soak=paper_soak,
        market=market,
        variant=variant,
        max_unhedged_delta=float(args.max_unhedged_delta),
    )

    out_path = pathlib.Path(args.out)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(payload, indent=2, sort_keys=True), encoding="utf-8")
    print(str(out_path))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
