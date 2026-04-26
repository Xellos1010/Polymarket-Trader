#!/usr/bin/env python3
"""Run the sandbox optimization cycle on a fixed interval.

This is a scheduler wrapper for local paper/replay workflows only.
It repeatedly invokes `tools/sandbox_optimizer_cycle.py` and writes a
small heartbeat/status file for operator visibility.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import pathlib
import subprocess
import time
from typing import Any, Dict

UTC = dt.timezone.utc


def now_utc() -> dt.datetime:
    return dt.datetime.now(UTC)


def write_json(path: pathlib.Path, payload: Dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True), encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description="Run sandbox optimizer on a fixed interval")
    parser.add_argument("--config", default="config/sandbox_optimizer_cycle.json")
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--interval-secs", type=int, default=3600)
    parser.add_argument("--max-cycles", type=int, default=0, help="0 means run forever")
    parser.add_argument(
        "--status-out",
        default="data/strategy_lab/hourly_optimizer_daemon_status.json",
        help="status JSON path",
    )
    args = parser.parse_args()

    repo_root = pathlib.Path(args.repo_root).resolve()
    status_path = repo_root / args.status_out
    cycle_cmd = [
        "python3",
        "tools/sandbox_optimizer_cycle.py",
        "--config",
        args.config,
        "--repo-root",
        str(repo_root),
    ]

    cycles_run = 0
    while True:
        cycle_started = now_utc()
        proc = subprocess.run(
            cycle_cmd,
            cwd=str(repo_root),
            capture_output=True,
            text=True,
            check=False,
        )
        cycle_finished = now_utc()
        cycles_run += 1

        status = {
            "sandbox_only": True,
            "cycle_cmd": cycle_cmd,
            "cycles_run": cycles_run,
            "last_started_at": cycle_started.isoformat().replace("+00:00", "Z"),
            "last_finished_at": cycle_finished.isoformat().replace("+00:00", "Z"),
            "last_returncode": proc.returncode,
            "last_stdout": proc.stdout,
            "last_stderr": proc.stderr,
            "next_run_at": (
                cycle_finished + dt.timedelta(seconds=max(args.interval_secs, 1))
            )
            .isoformat()
            .replace("+00:00", "Z"),
        }
        write_json(status_path, status)

        if args.max_cycles > 0 and cycles_run >= args.max_cycles:
            break

        time.sleep(max(args.interval_secs, 1))

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
