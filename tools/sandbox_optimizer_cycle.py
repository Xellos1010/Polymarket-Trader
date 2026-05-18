#!/usr/bin/env python3
"""Run one sandbox optimization cycle for the Coinbase workstation.

This tool keeps the loop local-first and sandbox-only:
1. run strategy-lab optimization
2. choose the top candidate
3. run a focused backtest for that candidate
4. promote the result into replay artifacts
5. run replay acceptance
6. store cycle history and optionally update the incumbent state

It does not place orders, enable live mode, or modify risk caps.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import pathlib
import subprocess
from dataclasses import dataclass
from typing import Any, Dict, List, Optional, Sequence

UTC = dt.timezone.utc


@dataclass
class CommandResult:
    argv: List[str]
    returncode: int
    stdout: str
    stderr: str


class CycleError(RuntimeError):
    pass


def now_utc() -> dt.datetime:
    return dt.datetime.now(UTC)


def read_json(path: pathlib.Path) -> Dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: pathlib.Path, payload: Dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True), encoding="utf-8")


def run_command(argv: Sequence[str], cwd: pathlib.Path) -> CommandResult:
    proc = subprocess.run(
        list(argv),
        cwd=str(cwd),
        capture_output=True,
        text=True,
        check=False,
    )
    return CommandResult(
        argv=list(argv),
        returncode=proc.returncode,
        stdout=proc.stdout,
        stderr=proc.stderr,
    )


def parse_output_paths(stdout: str) -> List[pathlib.Path]:
    paths: List[pathlib.Path] = []
    for raw in stdout.splitlines():
        line = raw.strip()
        if not line:
            continue
        candidate = pathlib.Path(line)
        suffix = candidate.suffix.lower()
        if suffix in {".json", ".html", ".ndjson"}:
            paths.append(candidate)
    return paths


def latest_with_prefix(paths: Sequence[pathlib.Path], prefix: str, suffix: str) -> pathlib.Path:
    matches = [path for path in paths if path.name.startswith(prefix) and path.suffix == suffix]
    if not matches:
        raise CycleError(f"no output path matched {prefix}*{suffix}")
    return matches[-1]


def load_cycle_config(path: pathlib.Path) -> Dict[str, Any]:
    cfg = read_json(path)
    if not isinstance(cfg, dict):
        raise CycleError("cycle config must be a JSON object")
    return cfg


def choose_best_market(candidate: Dict[str, Any], market_override: Optional[str]) -> str:
    per_market = candidate.get("per_market", [])
    if market_override:
        return market_override
    if not isinstance(per_market, list) or not per_market:
        raise CycleError("candidate did not include per_market ranking data")
    best = max(per_market, key=lambda row: float(row.get("score", float("-inf"))))
    market = str(best.get("market", "")).strip()
    if not market:
        raise CycleError("unable to determine best market for candidate")
    return market


def candidate_summary(candidate: Dict[str, Any], market: str) -> Dict[str, Any]:
    return {
        "variant": candidate.get("variant"),
        "params": candidate.get("params", {}),
        "score": float(candidate.get("score", 0.0)),
        "avg_return": float(candidate.get("avg_return", 0.0)),
        "avg_drawdown": float(candidate.get("avg_drawdown", 0.0)),
        "avg_trades": float(candidate.get("avg_trades", 0.0)),
        "market_count": int(candidate.get("market_count", 0)),
        "selected_market": market,
        "objective_breakdown": candidate.get("objective_breakdown", {}),
        "stability": candidate.get("stability", {}),
        "risk_gate": candidate.get("risk_gate", {}),
        "promotion_gate": candidate.get("promotion_gate", {}),
        "rejection_reasons": candidate.get("rejection_reasons", []),
    }


def load_incumbent(path: pathlib.Path) -> Optional[Dict[str, Any]]:
    if not path.exists():
        return None
    payload = read_json(path)
    return payload if isinstance(payload, dict) else None


def should_attempt_promotion(
    candidate: Dict[str, Any],
    incumbent: Optional[Dict[str, Any]],
    min_score_delta: float,
) -> bool:
    if incumbent is None:
        return True
    candidate_score = float(candidate.get("score", 0.0))
    incumbent_score = float(incumbent.get("score", 0.0))
    return candidate_score >= incumbent_score + min_score_delta


def build_strategy_lab_optimize_cmd(repo_root: pathlib.Path, cfg: Dict[str, Any]) -> List[str]:
    commands = cfg.get("commands", {}) if isinstance(cfg.get("commands"), dict) else {}
    python_bin = str(commands.get("python", "python3"))
    optimize = cfg.get("optimize", {}) if isinstance(cfg.get("optimize"), dict) else {}
    argv = [
        python_bin,
        "tools/coinbase_strategy_lab.py",
        "optimize",
        "--config",
        str(cfg.get("strategy_lab_config", "config/coinbase_strategy_lab.json")),
        "--out",
        str(cfg.get("strategy_lab_out_dir", "data/strategy_lab")),
    ]
    if optimize.get("provider"):
        argv.extend(["--provider", str(optimize["provider"])])
    if optimize.get("markets"):
        argv.extend(["--markets", str(optimize["markets"])])
    if optimize.get("granularity_sec") is not None:
        argv.extend(["--granularity-sec", str(optimize["granularity_sec"])])
    if optimize.get("limit") is not None:
        argv.extend(["--limit", str(optimize["limit"])])
    if optimize.get("disable_journal"):
        argv.append("--disable-journal")
    return argv


def build_strategy_lab_backtest_cmd(cfg: Dict[str, Any], candidate: Dict[str, Any]) -> List[str]:
    commands = cfg.get("commands", {}) if isinstance(cfg.get("commands"), dict) else {}
    python_bin = str(commands.get("python", "python3"))
    backtest = cfg.get("backtest", {}) if isinstance(cfg.get("backtest"), dict) else {}
    params = candidate.get("params", {}) if isinstance(candidate.get("params"), dict) else {}
    argv = [
        python_bin,
        "tools/coinbase_strategy_lab.py",
        "backtest",
        "--config",
        str(cfg.get("strategy_lab_config", "config/coinbase_strategy_lab.json")),
        "--out",
        str(cfg.get("strategy_lab_out_dir", "data/strategy_lab")),
        "--short-window",
        str(params.get("short_window")),
        "--long-window",
        str(params.get("long_window")),
    ]
    if backtest.get("provider"):
        argv.extend(["--provider", str(backtest["provider"])])
    if backtest.get("markets"):
        argv.extend(["--markets", str(backtest["markets"])])
    if backtest.get("granularity_sec") is not None:
        argv.extend(["--granularity-sec", str(backtest["granularity_sec"])])
    if backtest.get("limit") is not None:
        argv.extend(["--limit", str(backtest["limit"])])
    if backtest.get("disable_journal"):
        argv.append("--disable-journal")
    return argv


def build_promote_cmd(cfg: Dict[str, Any], backtest_json: pathlib.Path, market: str, variant: str) -> List[str]:
    commands = cfg.get("commands", {}) if isinstance(cfg.get("commands"), dict) else {}
    python_bin = str(commands.get("python", "python3"))
    promotion = cfg.get("promotion", {}) if isinstance(cfg.get("promotion"), dict) else {}
    argv = [
        python_bin,
        "tools/promote_strategy_lab.py",
        "--report",
        str(backtest_json),
        "--market",
        market,
        "--variant",
        variant,
        "--out-replay",
        str(promotion.get("replay_path", "data/replay/strategy_lab_promoted.ndjson")),
        "--out-promotion",
        str(promotion.get("promotion_path", "data/tuning/strategy_lab_promoted.json")),
    ]
    if promotion.get("spread_bps") is not None:
        argv.extend(["--spread-bps", str(promotion["spread_bps"])])
    if promotion.get("liquidity") is not None:
        argv.extend(["--liquidity", str(promotion["liquidity"])])
    if promotion.get("market_id_prefix"):
        argv.extend(["--market-id-prefix", str(promotion["market_id_prefix"])])
    if promotion.get("token_id_prefix"):
        argv.extend(["--token-id-prefix", str(promotion["token_id_prefix"])])
    return argv


def build_acceptance_cmd(cfg: Dict[str, Any], replay_path: pathlib.Path, promotion_path: pathlib.Path, summary_path: pathlib.Path) -> List[str]:
    commands = cfg.get("commands", {}) if isinstance(cfg.get("commands"), dict) else {}
    python_bin = str(commands.get("python", "python3"))
    acceptance = cfg.get("replay_acceptance", {}) if isinstance(cfg.get("replay_acceptance"), dict) else {}
    argv = [
        python_bin,
        "tools/replay_acceptance.py",
        "--replay",
        str(replay_path),
        "--promotion",
        str(promotion_path),
        "--out",
        str(summary_path),
        "--min-frames",
        str(acceptance.get("min_frames", 3)),
    ]
    if acceptance.get("sqlite"):
        argv.extend(["--sqlite", str(acceptance["sqlite"])])
    if acceptance.get("dashboard_url"):
        argv.extend(["--dashboard-url", str(acceptance["dashboard_url"])])
    if acceptance.get("min_snapshots") is not None:
        argv.extend(["--min-snapshots", str(acceptance["min_snapshots"])])
    if acceptance.get("min_risk_events") is not None:
        argv.extend(["--min-risk-events", str(acceptance["min_risk_events"])])
    if acceptance.get("max_stale_books") is not None:
        argv.extend(["--max-stale-books", str(acceptance["max_stale_books"])])
    return argv


def main() -> int:
    parser = argparse.ArgumentParser(description="Run one sandbox optimization cycle")
    parser.add_argument("--config", default="config/sandbox_optimizer_cycle.json")
    parser.add_argument("--repo-root", default=".")
    args = parser.parse_args()

    repo_root = pathlib.Path(args.repo_root).resolve()
    cfg = load_cycle_config((repo_root / args.config).resolve())

    state_path = repo_root / str(cfg.get("cycle_state_path", "data/strategy_lab/hourly_optimizer_state.json"))
    history_dir = repo_root / str(cfg.get("cycle_history_dir", "data/strategy_lab/hourly_optimizer_runs"))
    history_dir.mkdir(parents=True, exist_ok=True)
    cycle_ts = now_utc()
    cycle_id = cycle_ts.strftime("%Y%m%d-%H%M%S")
    cycle_summary_path = history_dir / f"cycle-{cycle_id}.json"
    acceptance_summary_path = history_dir / f"cycle-{cycle_id}.acceptance.json"

    result: Dict[str, Any] = {
        "cycle_id": cycle_id,
        "started_at": cycle_ts.isoformat().replace("+00:00", "Z"),
        "status": "running",
        "sandbox_only": True,
        "steps": {},
    }

    incumbent = load_incumbent(state_path)
    if incumbent is not None:
        result["incumbent_before"] = incumbent

    optimize_cmd = build_strategy_lab_optimize_cmd(repo_root, cfg)
    optimize_run = run_command(optimize_cmd, repo_root)
    result["steps"]["optimize"] = {
        "argv": optimize_run.argv,
        "returncode": optimize_run.returncode,
        "stdout": optimize_run.stdout,
        "stderr": optimize_run.stderr,
    }
    if optimize_run.returncode != 0:
        result["status"] = "failed_optimize"
        write_json(cycle_summary_path, result)
        return 1

    optimize_paths = parse_output_paths(optimize_run.stdout)
    optimize_json = latest_with_prefix(optimize_paths, "optimize-", ".json")
    optimize_payload = read_json(repo_root / optimize_json if not optimize_json.is_absolute() else optimize_json)
    top = optimize_payload.get("top", [])
    if not isinstance(top, list) or not top:
        raise CycleError("optimization output did not contain any ranked candidates")

    selected_candidate = top[0]
    selected_market = choose_best_market(
        selected_candidate,
        (cfg.get("promotion", {}) or {}).get("market_override") if isinstance(cfg.get("promotion"), dict) else None,
    )
    result["candidate"] = candidate_summary(selected_candidate, selected_market)
    result["steps"]["optimize"]["report_json"] = str(optimize_json)

    gate = cfg.get("promotion_gate", {}) if isinstance(cfg.get("promotion_gate"), dict) else {}
    min_score_delta = float(gate.get("min_score_delta", 0.0))
    if not should_attempt_promotion(selected_candidate, incumbent, min_score_delta):
        result["status"] = "no_promotion"
        result["decision"] = {
            "reason": "candidate did not beat incumbent score gate",
            "reason_code": "incumbent_score_gate",
            "reason_detail": {
                "candidate_score": float(selected_candidate.get("score", 0.0)),
                "incumbent_score": float((incumbent or {}).get("score", 0.0)),
            },
            "min_score_delta": min_score_delta,
        }
        write_json(cycle_summary_path, result)
        return 0

    backtest_cmd = build_strategy_lab_backtest_cmd(cfg, selected_candidate)
    backtest_run = run_command(backtest_cmd, repo_root)
    result["steps"]["backtest"] = {
        "argv": backtest_run.argv,
        "returncode": backtest_run.returncode,
        "stdout": backtest_run.stdout,
        "stderr": backtest_run.stderr,
    }
    if backtest_run.returncode != 0:
        result["status"] = "failed_backtest"
        write_json(cycle_summary_path, result)
        return 1

    backtest_paths = parse_output_paths(backtest_run.stdout)
    backtest_json = latest_with_prefix(backtest_paths, "backtest-", ".json")
    result["steps"]["backtest"]["report_json"] = str(backtest_json)

    variant = str(selected_candidate.get("variant", "")).strip()
    if not variant:
        raise CycleError("selected candidate was missing variant name")

    promote_cmd = build_promote_cmd(
        cfg,
        repo_root / backtest_json if not backtest_json.is_absolute() else backtest_json,
        selected_market,
        variant,
    )
    promote_run = run_command(promote_cmd, repo_root)
    result["steps"]["promote"] = {
        "argv": promote_run.argv,
        "returncode": promote_run.returncode,
        "stdout": promote_run.stdout,
        "stderr": promote_run.stderr,
    }
    if promote_run.returncode != 0:
        result["status"] = "failed_promote"
        write_json(cycle_summary_path, result)
        return 1

    promote_paths = parse_output_paths(promote_run.stdout)
    replay_path = latest_with_prefix(promote_paths, "", ".ndjson")
    promotion_json = latest_with_prefix(promote_paths, "", ".json")
    result["steps"]["promote"]["replay_path"] = str(replay_path)
    result["steps"]["promote"]["promotion_json"] = str(promotion_json)

    acceptance_cmd = build_acceptance_cmd(
        cfg,
        repo_root / replay_path if not replay_path.is_absolute() else replay_path,
        repo_root / promotion_json if not promotion_json.is_absolute() else promotion_json,
        acceptance_summary_path,
    )
    acceptance_run = run_command(acceptance_cmd, repo_root)
    result["steps"]["replay_acceptance"] = {
        "argv": acceptance_run.argv,
        "returncode": acceptance_run.returncode,
        "stdout": acceptance_run.stdout,
        "stderr": acceptance_run.stderr,
        "summary_path": str(acceptance_summary_path),
    }

    acceptance_summary = read_json(acceptance_summary_path)
    result["acceptance"] = acceptance_summary

    require_acceptance_pass = bool(gate.get("require_acceptance_pass", True))
    if require_acceptance_pass and acceptance_run.returncode != 0:
        result["status"] = "rejected_after_replay"
        result["decision"] = {
            "reason": "replay acceptance failed",
            "reason_code": "replay_acceptance_failed",
            "reason_detail": acceptance_summary,
            "require_acceptance_pass": True,
        }
        write_json(cycle_summary_path, result)
        return 0

    next_state = {
        "updated_at": now_utc().isoformat().replace("+00:00", "Z"),
        "cycle_id": cycle_id,
        "optimize_report": str(optimize_json),
        "backtest_report": str(backtest_json),
        "promotion_json": str(promotion_json),
        "replay_path": str(replay_path),
        **result["candidate"],
    }
    write_json(state_path, next_state)

    result["status"] = "promoted"
    result["decision"] = {
        "reason": "candidate cleared incumbent and replay gates",
        "reason_code": "promoted_after_replay_gate",
        "reason_detail": {
            "acceptance_status": acceptance_summary.get("status"),
            "promotion_gate": result["candidate"].get("promotion_gate"),
        },
        "state_path": str(state_path),
    }
    write_json(cycle_summary_path, result)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
