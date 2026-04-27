#!/usr/bin/env python3
"""Build a Phase 1 evidence gate summary from replay/paper artifacts.

This report is intentionally conservative. It only returns a Phase 1 pass when
there are enough independent runs and each run includes explicit modeled-cost
and risk-gate evidence. Missing or malformed evidence is reported as
incomplete rather than silently treated as success.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import pathlib
from typing import Any, Dict, List, Optional, Tuple

UTC = dt.timezone.utc
MANIFEST_SCHEMA_VERSION = 1
REQUIRED_MANIFEST_FIELDS = {"run_label", "generated_at", "schema_version", "artifacts"}
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


def load_json_file(path: pathlib.Path) -> Tuple[Optional[Dict[str, Any]], Optional[str]]:
    if not path.exists():
        return None, f"missing {path.name}"
    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        return None, f"{path.name} is invalid JSON: {exc}"
    if not isinstance(payload, dict):
        return None, f"{path.name} must contain a JSON object"
    return payload, None


def discover_run_dirs(bundle_dir: pathlib.Path) -> List[pathlib.Path]:
    if not bundle_dir.exists():
        return []
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


def run_status(run: Dict[str, Any]) -> str:
    checks = run["checks"]
    if checks["manifest_valid"] is False:
        return "incomplete"
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
    if any(value is None for value in checks.values()):
        return "incomplete"
    return "pass"


def validate_manifest(manifest: Optional[Dict[str, Any]]) -> Tuple[Optional[bool], List[str]]:
    notes: List[str] = []
    if manifest is None:
        return None, notes

    missing_fields = sorted(REQUIRED_MANIFEST_FIELDS - set(manifest))
    if missing_fields:
        notes.append("manifest.json missing fields: " + ", ".join(missing_fields))
        return False, notes

    if manifest.get("schema_version") != MANIFEST_SCHEMA_VERSION:
        notes.append(
            "manifest.json schema_version must equal "
            f"{MANIFEST_SCHEMA_VERSION}, got {manifest.get('schema_version')!r}"
        )
        return False, notes

    artifacts = manifest.get("artifacts")
    if not isinstance(artifacts, dict):
        notes.append("manifest.json artifacts must be an object")
        return False, notes

    return True, notes


def evaluate_run(run_dir: pathlib.Path, max_unhedged_delta: float) -> Dict[str, Any]:
    manifest, manifest_error = load_json_file(run_dir / "manifest.json")
    replay, replay_error = load_json_file(run_dir / "replay_acceptance.json")
    paper, paper_error = load_json_file(run_dir / "paper_soak.json")
    metrics, metrics_error = load_json_file(run_dir / "metrics.json")

    notes: List[str] = []
    if manifest_error:
        notes.append(manifest_error)
    if replay_error:
        notes.append(replay_error)
    if paper_error:
        notes.append(paper_error)
    if metrics_error:
        notes.append(metrics_error)

    manifest_valid, manifest_notes = validate_manifest(manifest)
    notes.extend(manifest_notes)

    checks: Dict[str, Optional[bool]] = {
        "manifest_valid": manifest_valid,
        "replay_acceptance": None,
        "paper_soak": None,
        "max_abs_unhedged_delta_ok": None,
        "metrics_complete": None,
        "modeled_cost_roi_positive": None,
        "risk_breaches_clear": None,
    }
    risk_breaches: List[str] = []
    net_pnl_after_costs: Optional[float] = None

    if replay is not None:
        checks["replay_acceptance"] = replay.get("status") == "pass"
        if checks["replay_acceptance"] is False:
            notes.extend(str(item) for item in replay.get("failures", []))

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

    if metrics is not None:
        missing_fields = sorted(REQUIRED_METRIC_FIELDS - set(metrics))
        checks["metrics_complete"] = not missing_fields
        if missing_fields:
            notes.append("metrics.json missing fields: " + ", ".join(missing_fields))

        net_pnl_after_costs = coerce_float(metrics.get("net_pnl_after_costs"))
        checks["modeled_cost_roi_positive"] = (
            None if net_pnl_after_costs is None else net_pnl_after_costs > 0
        )
        if net_pnl_after_costs is not None and net_pnl_after_costs <= 0:
            notes.append(f"net_pnl_after_costs {net_pnl_after_costs} <= 0")

        risk_flags = {field: coerce_bool(metrics.get(field)) for field in RISK_BOOL_FIELDS}
        if any(flag is None for flag in risk_flags.values()):
            checks["risk_breaches_clear"] = None
        else:
            risk_breaches = [field for field, value in risk_flags.items() if value]
            checks["risk_breaches_clear"] = not risk_breaches
            if risk_breaches:
                notes.append("risk breaches flagged: " + ", ".join(risk_breaches))

    run_label = run_dir.name
    generated_at = None
    if manifest is not None:
        run_label = str(manifest.get("run_label") or run_dir.name)
        generated_at = manifest.get("generated_at")

    result = {
        "run_label": run_label,
        "generated_at": generated_at,
        "manifest": manifest,
        "checks": checks,
        "net_pnl_after_costs": net_pnl_after_costs,
        "risk_breaches": risk_breaches,
        "notes": notes,
    }
    result["status"] = run_status(result)
    return result


def summarize_runs(runs: List[Dict[str, Any]], min_runs: int) -> Dict[str, Any]:
    status_counts = {"pass": 0, "fail": 0, "incomplete": 0}
    for run in runs:
        status_counts[run["status"]] = status_counts.get(run["status"], 0) + 1

    run_labels = [run["run_label"] for run in runs]
    unique_run_labels = len(set(run_labels)) == len(run_labels)
    aggregate_net_pnl_after_costs = sum(
        run["net_pnl_after_costs"] or 0.0 for run in runs if run["net_pnl_after_costs"] is not None
    )

    summary_notes: List[str] = []
    if len(runs) < min_runs:
        summary_notes.append(f"run_count {len(runs)} < required_independent_runs {min_runs}")
    if not unique_run_labels:
        summary_notes.append("run labels must be unique to count as independent runs")
    if aggregate_net_pnl_after_costs <= 0:
        summary_notes.append(
            f"aggregate_net_pnl_after_costs {aggregate_net_pnl_after_costs} <= 0"
        )

    if status_counts.get("fail", 0) > 0:
        gate = "fail"
    elif len(runs) < min_runs or not unique_run_labels or status_counts.get("incomplete", 0) > 0:
        gate = "incomplete"
    elif aggregate_net_pnl_after_costs <= 0:
        gate = "fail"
    else:
        gate = "pass"

    return {
        "phase": "Phase 1",
        "generated_at": dt.datetime.now(tz=UTC).isoformat(),
        "required_independent_runs": min_runs,
        "run_count": len(runs),
        "independence_ok": len(runs) >= min_runs and unique_run_labels,
        "aggregate_net_pnl_after_costs": aggregate_net_pnl_after_costs,
        "status": gate,
        "status_counts": status_counts,
        "notes": summary_notes,
    }


def render_markdown(summary: Dict[str, Any], runs: List[Dict[str, Any]]) -> str:
    lines = [
        "# Phase 1 Evidence Gate Report",
        "",
        f"- Status: `{summary['status']}`",
        f"- Required independent runs: `{summary['required_independent_runs']}`",
        f"- Run count: `{summary['run_count']}`",
        f"- Independence ok: `{summary['independence_ok']}`",
        f"- Aggregate net PnL after costs: `{summary['aggregate_net_pnl_after_costs']}`",
        f"- Generated at: `{summary['generated_at']}`",
        "",
        "## Summary Notes",
        "",
    ]
    if summary["notes"]:
        for note in summary["notes"]:
            lines.append(f"- {note}")
    else:
        lines.append("- none")
    lines.extend(["", "## Run Summary", ""])

    for run in runs:
        lines.append(f"### {run['run_label']}")
        lines.append(f"- Status: `{run['status']}`")
        lines.append(f"- Replay acceptance: `{run['checks']['replay_acceptance']}`")
        lines.append(f"- Paper soak: `{run['checks']['paper_soak']}`")
        lines.append(
            f"- Max abs unhedged delta ok: `{run['checks']['max_abs_unhedged_delta_ok']}`"
        )
        lines.append(f"- Manifest valid: `{run['checks']['manifest_valid']}`")
        lines.append(f"- Metrics complete: `{run['checks']['metrics_complete']}`")
        lines.append(
            f"- Net modeled-cost PnL positive: `{run['checks']['modeled_cost_roi_positive']}`"
        )
        lines.append(f"- Risk breaches clear: `{run['checks']['risk_breaches_clear']}`")
        lines.append(f"- Net PnL after costs: `{run['net_pnl_after_costs']}`")
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
