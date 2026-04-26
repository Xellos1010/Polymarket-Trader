import importlib.util
import json
import tempfile
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).resolve().parents[1] / "tools" / "phase1_gate_report.py"
SPEC = importlib.util.spec_from_file_location("phase1_gate_report", MODULE_PATH)
phase1_gate_report = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(phase1_gate_report)


BASE_METRICS = {
    "net_pnl_after_costs": 0.42,
    "fees": 0.05,
    "slippage": 0.04,
    "hedge_cost": 0.03,
    "gas_amortized": 0.0,
    "adverse_selection": 0.02,
    "daily_loss_limit_breached": False,
    "max_market_notional_breached": False,
    "max_total_open_notional_breached": False,
    "max_unhedged_delta_breached": False,
    "stale_book_breached": False,
    "unexpected_auto_halt": False,
}


class Phase1GateReportTests(unittest.TestCase):
    def write_json(self, path: Path, payload) -> None:
        path.write_text(json.dumps(payload, indent=2, sort_keys=True), encoding="utf-8")

    def create_run(
        self,
        bundle_dir: Path,
        run_label: str,
        *,
        replay_status: str = "pass",
        paper_pass: bool = True,
        max_abs_unhedged_delta: float = 5.0,
        metrics_override=None,
        manifest_override=None,
        malformed_metrics: bool = False,
    ) -> Path:
        run_dir = bundle_dir / run_label
        run_dir.mkdir(parents=True, exist_ok=True)
        manifest = {
            "schema_version": 1,
            "phase": "Phase 1",
            "run_label": run_label,
            "generated_at": "2026-04-26T12:00:00+00:00",
            "artifacts": {
                "replay_source": f"data/replay/{run_label}.ndjson",
                "replay_acceptance": str(run_dir / "replay_acceptance.json"),
                "promotion_source": None,
                "sqlite_source": None,
                "paper_soak": str(run_dir / "paper_soak.json"),
                "metrics": str(run_dir / "metrics.json"),
            },
        }
        if manifest_override:
            manifest.update(manifest_override)
        self.write_json(run_dir / "manifest.json", manifest)
        self.write_json(
            run_dir / "replay_acceptance.json",
            {"status": replay_status, "failures": []},
        )
        self.write_json(
            run_dir / "paper_soak.json",
            {
                "pass": paper_pass,
                "reason": "fixture",
                "max_abs_unhedged_delta": max_abs_unhedged_delta,
            },
        )
        if malformed_metrics:
            (run_dir / "metrics.json").write_text("{not-json", encoding="utf-8")
        else:
            metrics = dict(BASE_METRICS)
            if metrics_override:
                metrics.update(metrics_override)
            self.write_json(run_dir / "metrics.json", metrics)
        return run_dir

    def test_three_clean_runs_pass(self):
        with tempfile.TemporaryDirectory() as tmp:
            bundle_dir = Path(tmp)
            self.create_run(bundle_dir, "run-001", metrics_override={"net_pnl_after_costs": 0.25})
            self.create_run(bundle_dir, "run-002", metrics_override={"net_pnl_after_costs": 0.30})
            self.create_run(bundle_dir, "run-003", metrics_override={"net_pnl_after_costs": 0.35})

            runs = [
                phase1_gate_report.evaluate_run(run_dir, max_unhedged_delta=10.0)
                for run_dir in phase1_gate_report.discover_run_dirs(bundle_dir)
            ]
            summary = phase1_gate_report.summarize_runs(runs, min_runs=3)

            self.assertEqual(summary["status"], "pass")
            self.assertTrue(summary["independence_ok"])
            self.assertGreater(summary["aggregate_net_pnl_after_costs"], 0)

    def test_less_than_three_runs_is_incomplete(self):
        with tempfile.TemporaryDirectory() as tmp:
            bundle_dir = Path(tmp)
            self.create_run(bundle_dir, "run-001")
            self.create_run(bundle_dir, "run-002")

            runs = [
                phase1_gate_report.evaluate_run(run_dir, max_unhedged_delta=10.0)
                for run_dir in phase1_gate_report.discover_run_dirs(bundle_dir)
            ]
            summary = phase1_gate_report.summarize_runs(runs, min_runs=3)

            self.assertEqual(summary["status"], "incomplete")
            self.assertIn("run_count 2 < required_independent_runs 3", summary["notes"])

    def test_missing_metrics_field_is_incomplete(self):
        with tempfile.TemporaryDirectory() as tmp:
            bundle_dir = Path(tmp)
            self.create_run(bundle_dir, "run-001")
            self.create_run(bundle_dir, "run-002")
            run_dir = self.create_run(bundle_dir, "run-003")
            metrics = dict(BASE_METRICS)
            metrics.pop("adverse_selection")
            self.write_json(run_dir / "metrics.json", metrics)

            runs = [
                phase1_gate_report.evaluate_run(path, max_unhedged_delta=10.0)
                for path in phase1_gate_report.discover_run_dirs(bundle_dir)
            ]
            summary = phase1_gate_report.summarize_runs(runs, min_runs=3)

            self.assertEqual(summary["status"], "incomplete")
            target = next(run for run in runs if run["run_label"] == "run-003")
            self.assertEqual(target["status"], "incomplete")
            self.assertIn("metrics.json missing fields: adverse_selection", target["notes"])

    def test_negative_net_or_risk_breach_fails(self):
        with tempfile.TemporaryDirectory() as tmp:
            bundle_dir = Path(tmp)
            self.create_run(bundle_dir, "run-001")
            self.create_run(bundle_dir, "run-002", metrics_override={"net_pnl_after_costs": -0.01})
            self.create_run(bundle_dir, "run-003")

            runs = [
                phase1_gate_report.evaluate_run(run_dir, max_unhedged_delta=10.0)
                for run_dir in phase1_gate_report.discover_run_dirs(bundle_dir)
            ]
            summary = phase1_gate_report.summarize_runs(runs, min_runs=3)

            self.assertEqual(summary["status"], "fail")
            failing_run = next(run for run in runs if run["run_label"] == "run-002")
            self.assertEqual(failing_run["status"], "fail")

    def test_malformed_bundle_is_incomplete(self):
        with tempfile.TemporaryDirectory() as tmp:
            bundle_dir = Path(tmp)
            self.create_run(bundle_dir, "run-001")
            self.create_run(bundle_dir, "run-002")
            self.create_run(bundle_dir, "run-003", malformed_metrics=True)

            runs = [
                phase1_gate_report.evaluate_run(run_dir, max_unhedged_delta=10.0)
                for run_dir in phase1_gate_report.discover_run_dirs(bundle_dir)
            ]
            summary = phase1_gate_report.summarize_runs(runs, min_runs=3)

            self.assertEqual(summary["status"], "incomplete")
            target = next(run for run in runs if run["run_label"] == "run-003")
            self.assertEqual(target["status"], "incomplete")
            self.assertTrue(
                any("metrics.json is invalid JSON" in note for note in target["notes"])
            )


if __name__ == "__main__":
    unittest.main()
