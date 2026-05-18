#!/usr/bin/env python3
"""Measure local strategy-lab throughput and emit a benchmark report."""

from __future__ import annotations

import argparse
import datetime as dt
import importlib.util
import json
import pathlib
import resource
import subprocess
import sys
import time
from typing import Any, Dict, Optional

UTC = dt.timezone.utc
REPO_ROOT = pathlib.Path(__file__).resolve().parents[1]
STRATEGY_LAB_PATH = REPO_ROOT / "tools" / "coinbase_strategy_lab.py"

SPEC = importlib.util.spec_from_file_location("coinbase_strategy_lab", STRATEGY_LAB_PATH)
coinbase_strategy_lab = importlib.util.module_from_spec(SPEC)
assert SPEC is not None and SPEC.loader is not None
sys.modules[SPEC.name] = coinbase_strategy_lab
SPEC.loader.exec_module(coinbase_strategy_lab)


def max_rss_mb() -> float:
    usage = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    if sys.platform == "darwin":
        return usage / (1024 * 1024)
    return usage / 1024


def read_json(path: pathlib.Path) -> Dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def run_replay_acceptance(replay: pathlib.Path, promotion: pathlib.Path) -> Dict[str, Any]:
    started = time.perf_counter()
    proc = subprocess.run(
        [
            "python3",
            "tools/replay_acceptance.py",
            "--replay",
            str(replay),
            "--promotion",
            str(promotion),
            "--out",
            str(REPO_ROOT / "artifacts" / "benchmarks" / "latest-replay-acceptance.json"),
            "--min-frames",
            "3",
        ],
        cwd=str(REPO_ROOT),
        capture_output=True,
        text=True,
        check=False,
    )
    elapsed = time.perf_counter() - started
    return {
        "status": "available" if proc.returncode == 0 else "failed",
        "elapsed_sec": elapsed,
        "returncode": proc.returncode,
        "stdout": proc.stdout,
        "stderr": proc.stderr,
    }


def markdown_report(report: Dict[str, Any]) -> str:
    lines = [
        "# Strategy Benchmark",
        "",
        f"- Generated at: {report['generated_at']}",
        f"- Config: `{report['config_path']}`",
        f"- Provider: `{report['provider']}`",
        f"- Granularity: `{report['granularity_sec']} sec`",
        "",
        "## Metrics",
        "",
        f"- Candidate count: {report['metrics']['candidate_count']}",
        f"- Candidates/minute: {report['metrics']['candidates_per_minute']:.2f}",
        f"- Bars/second: {report['metrics']['bars_per_second']:.2f}",
        f"- Optimize elapsed: {report['metrics']['optimize_elapsed_sec']:.3f} sec",
        f"- Artifact generation time: {report['metrics']['artifact_generation_sec']:.3f} sec",
        f"- Peak memory: {report['metrics']['peak_memory_mb']:.2f} MB",
        f"- Replay throughput status: {report['metrics']['replay_throughput']['status']}",
        "",
        "This is a local fixture/sandbox benchmark. It is not replay or paper evidence.",
    ]
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description="Local strategy benchmark harness")
    parser.add_argument("--config", default="config/coinbase_strategy_lab.json")
    parser.add_argument("--out-dir", default=None)
    parser.add_argument("--replay", default=None)
    parser.add_argument("--promotion", default=None)
    parser.add_argument("--provider", default=None)
    parser.add_argument("--markets", default=None)
    parser.add_argument("--granularity-sec", type=int, default=None)
    parser.add_argument("--limit", type=int, default=None)
    args = parser.parse_args()

    config_path = REPO_ROOT / args.config
    config = coinbase_strategy_lab.apply_cli_overrides(
        coinbase_strategy_lab.read_config(str(config_path)),
        argparse.Namespace(
            provider=args.provider,
            markets=args.markets,
            granularity_sec=args.granularity_sec,
            limit=args.limit,
            short_window=None,
            long_window=None,
            fee_bps=None,
            slippage_bps=None,
            starting_equity=None,
            auto_discovery=False,
            disable_auto_discovery=False,
            disable_journal=False,
            journal_path=None,
        ),
    )

    started = time.perf_counter()
    optimize_payload = coinbase_strategy_lab.run_optimize_data(config)
    optimize_elapsed = time.perf_counter() - started

    out_dir = pathlib.Path(
        args.out_dir
        or REPO_ROOT / "artifacts" / "benchmarks" / dt.datetime.now(UTC).strftime("%Y-%m-%d")
    )
    out_dir.mkdir(parents=True, exist_ok=True)

    artifact_started = time.perf_counter()
    optimize_json = out_dir / "strategy-benchmark-optimize.json"
    optimize_json.write_text(json.dumps(optimize_payload, indent=2), encoding="utf-8")
    artifact_generation = time.perf_counter() - artifact_started

    candidate_count = int(optimize_payload.get("meta", {}).get("candidate_count", 0))
    bars = int(config.get("backtest", {}).get("limit", 0)) * len(
        optimize_payload.get("meta", {}).get("markets", [])
    )
    replay_info: Dict[str, Any]
    if args.replay and args.promotion:
        replay_info = run_replay_acceptance(REPO_ROOT / args.replay, REPO_ROOT / args.promotion)
    else:
        replay_info = {
            "status": "unavailable",
            "elapsed_sec": None,
            "reason": "provide --replay and --promotion to benchmark replay acceptance throughput",
        }

    report = {
        "generated_at": dt.datetime.now(UTC).isoformat(),
        "config_path": str(config_path),
        "provider": config.get("provider", "coinbase"),
        "granularity_sec": int(config.get("granularity_sec", 300)),
        "metrics": {
            "candidate_count": candidate_count,
            "candidates_per_minute": (candidate_count / optimize_elapsed * 60.0) if optimize_elapsed > 0 else 0.0,
            "bars_per_second": (bars / optimize_elapsed) if optimize_elapsed > 0 else 0.0,
            "optimize_elapsed_sec": optimize_elapsed,
            "artifact_generation_sec": artifact_generation,
            "peak_memory_mb": max_rss_mb(),
            "replay_throughput": replay_info,
        },
        "notes": {
            "evidence_boundary": "Local benchmark only. Not replay or paper evidence.",
        },
    }

    json_path = out_dir / "strategy-benchmark.json"
    md_path = out_dir / "strategy-benchmark.md"
    json_path.write_text(json.dumps(report, indent=2), encoding="utf-8")
    md_path.write_text(markdown_report(report), encoding="utf-8")
    print(json_path)
    print(md_path)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
