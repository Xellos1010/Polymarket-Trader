#!/usr/bin/env python3
"""Build a Phase 1 evidence gate summary from replay/paper artifacts.

This report is intentionally conservative. It only returns a Phase 1 pass when
there are enough independent runs and each run includes explicit modeled-cost
and risk-gate evidence. Missing evidence is reported as incomplete rather than
silently treated as success.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import pathlib
from typing import Any, Dict, List, Optional

UTC = dt.timezone.utc
REQUIRED_METRIC_FIELDS = {
    "net_pnl_after_costs",
    "fees",
    "slippage",
    "hedge_cost",
    "gas_amortized",
    "adverse_selection",
    "daily_loss_limit_breached",
    "max_market_notional_breached",
    "max_total_open_notional_breached",
    "max_unhedged_delta_breached",
    "stale_book_breached",
    "unexpected_auto_halt",
}
RISK_BOOL_FIELDS = [
    "daily_loss_limit_breached",
    "max_market_notional_breached",
    "max_total_open_notional_breached",
    "max_unhedged_delta_breached",
    "stale_book_breached",
    "unexpected_auto_halt",
]


def load_json(path: pathlib.Path) -> Dict[str, Any]:
    payload = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(payload, dict):
        raise ValueError(f"{path}: expected JSON object")
    return payload


def discover_run_dirs(bundle_dir: pathlib.Path) -> List[pathlib.Path]:
    run_dirs = []
    for child in sorted(bundle_dir.iterdir()):
        if child.is_dir() and (child / "manifest.json").exists():
            run_dirs.append(child)
    return run_dirs


def coerce_bool(value: Any) -> Optional[bool]:
    if isinstance(value, bool):
        return value
    return None


def coerce_float(value: Any) -> Optional[float]:
    if isinstance(value, (int, float)):
        return float(value)
    return None


def run_status(run: Dict[str, Any], max_unhedged_delta: float) -> str:
    checks = run["checks"]
    if checks["replay_acceptance"] is False:
        return "fail"
    if checks["paper_soak"] is False:
        return "fail"
    if checks["max_abs_unhedged_delta_ok"] is False:
        return "fail"
    if checks["metrics_complete"] is False:
        return "incomplete"
    if checks["modeled_cost_roi_positive"] is False:
        return "fail"
    if checks["risk_breaches_clear"] is False:
        return "fail"
    if checks["replay_acceptance"] is None:
        return "incomplete"
    if checks["paper_soak"] is None:
        return "incomplete"
    if checks["max_abs_unhedged_delta_ok"] is None:
        return "incomplete"
    if checks["metrics_complete"] is None:
        return "incomplete"
    if checks["modeled_cost_roi_positive"] is None:
        return "incomplete"
    if checks["risk_breaches_clear"] is None:
        return "incomplete"
    return "pass"


def evaluate_run(run_dir: pathlib.Path, max_unhedged_delta: float) -> Dict[str, Any]:
    manifest = load_json(run_dir / "manifest.json")
    replay = load_json(run_dir / "replay_acceptance.json") if (run_dir / "replay_acceptance.json").exists() else None
    paper = load_json(run_dir / "paper_soak.json") if (run_dir / "paper_soak.json").exists() else None
    metrics = load_json(run_dir / "metrics.json") if (run_dir / "metrics.json").exists() else None

    notes: List[str] = []
    checks: Dict[str, Optional[bool]] = {
        "replay_acceptance": None,
        "paper_soak": None,
        "max_abs_unhedged_delta_ok": None,
        "metrics_complete": None,
        "modeled_cost_roi_positive": None,
        "risk_breaches_clear": None,
    }

    if replay is not None:
        checks["replay_acceptance"] = replay.get("status") == "pass"
        if checks["replay_acceptance"] is False:
            notes.extend(str(item) for item in replay.get("failures", []))
    else:
        notes.append("missing replay_acceptance.json")

    if paper is not None:
        checks["paper_soak"] = coerce_bool(paper.get("pass"))
        max_delta = coerce_float(paper.get("max_abs_unhedged_delta"))
        if max_delta is None:
            notes.append("paper_soak.json missing max_abs_unhedged_delta")
        else:
            checks["max_abs_unhedged_delta_ok"] = max_delta <= max_unhedged_delta
            if max_delta > max_unhedged_delta:
                notes.append(
                    f"max_abs_unhedged_delta {max_delta} > threshold {max_unhedged_delta}"
                )
        if checks["paper_soak"] is False:
            notes.append(f"paper_soak failed: {paper.get('reason', 'unknown')}")
    else:
        notes.append("missing paper_soak.json")

    if metrics is not None:
        missing_fields = sorted(REQUIRED_METRIC_FIELDS - set(metrics))
        checks["metrics_complete"] = not missing_fields
        if missing_fields:
            notes.append("metrics.json missing fields: " + ", ".join(missing_fields))
        net_pnl = coerce_float(metrics.get("net_pnl_after_costs"))
        checks["modeled_cost_roi_positive"] = None if net_pnl is None else net_pnl > 0
        if net_pnl is not None and net_pnl <= 0:
            notes.append(f"net_pnl_after_costs {net_pnl} <= 0")
        risk_flags = [coerce_bool(metrics.get(field)) for field in RISK_BOOL_FIELDS]
        if any(flag is None for flag in risk_flags):
            checks["risk_breaches_clear"] = None
        else:
            checks["risk_breaches_clear"] = not any(risk_flags)
            if any(risk_flags):
                breached = [field for field in RISK_BOOL_FIELDS if metrics.get(field)]
                notes.append("risk breaches flagged: " + ", ".join(breached))
    else:
        notes.append("missing metrics.json")

    result = {
        "run_label": manifest.get("run_label", run_dir.name),
        "generated_at": manifest.get("generated_at"),
        "manifest": manifest,
        "checks": checks,
        "notes": notes,
    }
    result["status"] = run_status(result, max_unhedged_delta=max_unhedged_delta)
    return result


def summarize_runs(runs: List[Dict[str, Any]], min_runs: int) -> Dict[str, Any]:
    status_counts = {"pass": 0, "fail": 0, "incomplete": 0}
    for run in runs:
        status_counts[run["status"]] = status_counts.get(run["status"], 0) + 1

    if len(runs) < min_runs:
        gate = "incomplete"
    elif status_counts.get("fail", 0) > 0:
        gate = "fail"
    elif status_counts.get("incomplete", 0) > 0:
        gate = "incomplete"
    else:
        gate = "pass"

    return {
        "phase": "Phase 1",
        "generated_at": dt.datetime.now(tz=UTC).isoformat(),
        "required_independent_runs": min_runs,
        "run_count": len(runs),
        "status": gate,
        "status_counts": status_counts,
    }


def render_markdown(summary: Dict[str, Any], runs: List[Dict[str, Any]]) -> str:
    lines = [
        "# Phase 1 Evidence Gate Report",
        "",
        f"- Status: `{summary['status']}`",
        f"- Required independent runs: `{summary['required_independent_runs']}`",
        f"- Run count: `{summary['run_count']}`",
        f"- Generated at: `{summary['generated_at']}`",
        "",
        "## Run Summary",
        "",
    ]
    for run in runs:
        lines.append(f"### {run['run_label']}")
        lines.append(f"- Status: `{run['status']}`")
        lines.append(f"- Replay acceptance: `{run['checks']['replay_acceptance']}`")
        lines.append(f"- Paper soak: `{run['checks']['paper_soak']}`")
        lines.append(
            f"- Max abs unhedged delta ok: `{run['checks']['max_abs_unhedged_delta_ok']}`"
        )
        lines.append(f"- Metrics complete: `{run['checks']['metrics_complete']}`")
        lines.append(
            f"- Net modeled-cost PnL positive: `{run['checks']['modeled_cost_roi_positive']}`"
        )
        lines.append(f"- Risk breaches clear: `{run['checks']['risk_breaches_clear']}`")
        if run["notes"]:
            lines.append("- Notes:")
            for note in run["notes"]:
                lines.append(f"  - {note}")
        lines.append("")
    return "\n".join(lines).rstrip() + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description="Build a Phase 1 evidence gate report")
    parser.add_argument("--bundle-dir", required=True, help="Directory containing run-*/manifest.json")
    parser.add_argument("--min-runs", type=int, default=3)
    parser.add_argument("--max-unhedged-delta", type=float, default=10.0)
    parser.add_argument("--out-json", default=None)
    parser.add_argument("--out-md", default=None)
    args = parser.parse_args()

    bundle_dir = pathlib.Path(args.bundle_dir)
    run_dirs = discover_run_dirs(bundle_dir)
    runs = [evaluate_run(run_dir, max_unhedged_delta=args.max_unhedged_delta) for run_dir in run_dirs]
    summary = summarize_runs(runs, min_runs=args.min_runs)
    payload = {"summary": summary, "runs": runs}

    if args.out_json:
        out_json = pathlib.Path(args.out_json)
        out_json.parent.mkdir(parents=True, exist_ok=True)
        out_json.write_text(json.dumps(payload, indent=2, sort_keys=True), encoding="utf-8")

    markdown = render_markdown(summary, runs)
    if args.out_md:
        out_md = pathlib.Path(args.out_md)
        out_md.parent.mkdir(parents=True, exist_ok=True)
        out_md.write_text(markdown, encoding="utf-8")

    print(json.dumps(payload, indent=2, sort_keys=True))
    return 0 if summary["status"] == "pass" else 1


if __name__ == "__main__":
    raise SystemExit(main())
