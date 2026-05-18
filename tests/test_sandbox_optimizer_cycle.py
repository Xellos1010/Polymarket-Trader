import importlib.util
import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch


MODULE_PATH = Path(__file__).resolve().parents[1] / "tools" / "sandbox_optimizer_cycle.py"
SPEC = importlib.util.spec_from_file_location("sandbox_optimizer_cycle", MODULE_PATH)
sandbox_optimizer_cycle = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
sys.modules[SPEC.name] = sandbox_optimizer_cycle
SPEC.loader.exec_module(sandbox_optimizer_cycle)


def write_json(path: Path, payload) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True), encoding="utf-8")


class SandboxOptimizerCycleTests(unittest.TestCase):
    def make_config(self, repo: Path) -> Path:
        cfg = {
            "strategy_lab_config": "config/coinbase_strategy_lab.json",
            "strategy_lab_out_dir": "data/strategy_lab",
            "cycle_state_path": "data/strategy_lab/hourly_optimizer_state.json",
            "cycle_history_dir": "data/strategy_lab/hourly_optimizer_runs",
            "commands": {"python": "python3"},
            "promotion": {
                "replay_path": "data/replay/strategy_lab_promoted.ndjson",
                "promotion_path": "data/tuning/strategy_lab_promoted.json",
            },
            "replay_acceptance": {"min_frames": 3},
            "promotion_gate": {
                "require_acceptance_pass": True,
                "min_score_delta": 0.05,
            },
        }
        path = repo / "config" / "sandbox_optimizer_cycle.json"
        write_json(path, cfg)
        write_json(repo / "config" / "coinbase_strategy_lab.json", {"provider": "coinbase"})
        return path

    def optimize_candidate(self, score: float) -> dict:
        return {
            "variant": "sma_baseline",
            "params": {"short_window": 5, "long_window": 21},
            "score": score,
            "avg_return": 0.12,
            "avg_drawdown": 0.03,
            "avg_trades": 4.0,
            "market_count": 1,
            "objective_breakdown": {
                "net_return_after_costs": 0.12,
                "drawdown_penalty": 0.024,
                "turnover_penalty": 0.01,
                "stability_penalty": 0.005,
                "final_score": score,
            },
            "stability": {"penalty": 0.005},
            "risk_gate": {"status": "pass"},
            "promotion_gate": {"status": "eligible_for_manual_review"},
            "rejection_reasons": [],
            "per_market": [{"market": "BTC-USD", "score": score}],
        }

    def test_cycle_records_no_promotion_reason_code(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo = Path(tmp)
            self.make_config(repo)
            write_json(
                repo / "data/strategy_lab/hourly_optimizer_state.json",
                {"score": 0.5},
            )
            optimize_report = repo / "data/strategy_lab/optimize-fixture.json"
            write_json(optimize_report, {"top": [self.optimize_candidate(0.51)]})

            def fake_run_command(argv, cwd):
                return sandbox_optimizer_cycle.CommandResult(
                    argv=list(argv),
                    returncode=0,
                    stdout=str(optimize_report.relative_to(repo)) + "\n",
                    stderr="",
                )

            with patch.object(sandbox_optimizer_cycle, "run_command", side_effect=fake_run_command):
                with patch.object(
                    sys,
                    "argv",
                    ["sandbox_optimizer_cycle.py", "--config", "config/sandbox_optimizer_cycle.json", "--repo-root", str(repo)],
                ):
                    code = sandbox_optimizer_cycle.main()

            self.assertEqual(code, 0)
            history = [
                path
                for path in (repo / "data/strategy_lab/hourly_optimizer_runs").glob("cycle-*.json")
                if ".acceptance." not in path.name
            ]
            self.assertEqual(len(history), 1)
            payload = json.loads(history[0].read_text(encoding="utf-8"))
            self.assertEqual(payload["status"], "no_promotion")
            self.assertEqual(payload["decision"]["reason_code"], "incumbent_score_gate")
            self.assertIn("objective_breakdown", payload["candidate"])

    def test_cycle_records_promoted_reason_code(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo = Path(tmp)
            self.make_config(repo)
            optimize_report = repo / "data/strategy_lab/optimize-fixture.json"
            backtest_report = repo / "data/strategy_lab/backtest-fixture.json"
            replay_path = repo / "data/replay/strategy_lab_promoted.ndjson"
            promotion_path = repo / "data/tuning/strategy_lab_promoted.json"
            acceptance_path = repo / "data/strategy_lab/hourly_optimizer_runs/cycle-fixture.acceptance.json"
            write_json(optimize_report, {"top": [self.optimize_candidate(0.72)]})
            write_json(backtest_report, {"ok": True})
            replay_path.parent.mkdir(parents=True, exist_ok=True)
            replay_path.write_text("{}\n", encoding="utf-8")
            write_json(promotion_path, {"promotion_status": "promoted"})

            def fake_run_command(argv, cwd):
                argv = list(argv)
                if "optimize" in argv:
                    return sandbox_optimizer_cycle.CommandResult(argv, 0, str(optimize_report.relative_to(repo)) + "\n", "")
                if "backtest" in argv:
                    return sandbox_optimizer_cycle.CommandResult(argv, 0, str(backtest_report.relative_to(repo)) + "\n", "")
                if any("promote_strategy_lab.py" in part for part in argv):
                    stdout = (
                        str(replay_path.relative_to(repo))
                        + "\n"
                        + str(promotion_path.relative_to(repo))
                        + "\n"
                    )
                    return sandbox_optimizer_cycle.CommandResult(argv, 0, stdout, "")
                if any("replay_acceptance.py" in part for part in argv):
                    out_idx = argv.index("--out") + 1
                    target = Path(argv[out_idx])
                    write_json(target, {"status": "pass"})
                    return sandbox_optimizer_cycle.CommandResult(argv, 0, str(target) + "\n", "")
                raise AssertionError(f"unexpected argv: {argv}")

            with patch.object(sandbox_optimizer_cycle, "run_command", side_effect=fake_run_command):
                with patch.object(
                    sys,
                    "argv",
                    ["sandbox_optimizer_cycle.py", "--config", "config/sandbox_optimizer_cycle.json", "--repo-root", str(repo)],
                ):
                    code = sandbox_optimizer_cycle.main()

            self.assertEqual(code, 0)
            state = json.loads((repo / "data/strategy_lab/hourly_optimizer_state.json").read_text(encoding="utf-8"))
            self.assertEqual(state["score"], 0.72)
            history = [
                path
                for path in (repo / "data/strategy_lab/hourly_optimizer_runs").glob("cycle-*.json")
                if ".acceptance." not in path.name
            ]
            self.assertEqual(len(history), 1)
            payload = json.loads(history[0].read_text(encoding="utf-8"))
            self.assertEqual(payload["status"], "promoted")
            self.assertEqual(payload["decision"]["reason_code"], "promoted_after_replay_gate")


if __name__ == "__main__":
    unittest.main()
