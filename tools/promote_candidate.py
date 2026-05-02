#!/usr/bin/env python3
"""Promote the best tuning candidate into an operational artifact.

Usage:
  python3 tools/promote_candidate.py \
    --report data/tuning/pine_tuning_results.json \
    --out data/tuning/promoted_candidate.json \
    --asset BTC --horizon 15m
"""

from __future__ import annotations

import argparse
import json
import pathlib
import time
from typing import Any, Dict, Optional


def pick_candidate(report: Dict[str, Any]) -> Dict[str, Any]:
    best = report.get("best")
    if isinstance(best, dict) and isinstance(best.get("params"), dict):
        return best

    top = report.get("top_candidates") or []
    if isinstance(top, list):
        for c in top:
            if isinstance(c, dict) and isinstance(c.get("params"), dict):
                return c

    raise ValueError("no candidate found in tuning report")


def main() -> int:
    p = argparse.ArgumentParser(description="Promote Pine tuning candidate")
    p.add_argument("--report", required=True, help="path to tuning report JSON")
    p.add_argument("--out", default="data/tuning/promoted_candidate.json", help="output JSON")
    p.add_argument("--asset", default="BTC", help="asset label")
    p.add_argument("--horizon", default="15m", help="horizon label")
    p.add_argument("--paper-hours", type=int, default=1, help="paper verification hours")
    args = p.parse_args()

    report_path = pathlib.Path(args.report)
    report = json.loads(report_path.read_text(encoding="utf-8"))

    candidate = pick_candidate(report)
    output = {
        "promoted_at_epoch": int(time.time()),
        "source_report": str(report_path),
        "script_path": report.get("script_path"),
        "asset": args.asset,
        "horizon": args.horizon,
        "iteration": candidate.get("iteration"),
        "score": candidate.get("score"),
        "params": candidate.get("params"),
        "verification": {
            "required_mode": "paper",
            "paper_hours": args.paper_hours,
            "command": f"./scripts/paper_soak.sh {args.paper_hours * 3600} 30 config/config.toml",
            "must_pass": True,
        },
    }

    out_path = pathlib.Path(args.out)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(output, indent=2, sort_keys=True), encoding="utf-8")
    print(str(out_path))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
