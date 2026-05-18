import importlib.util
import json
import sys
import tempfile
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).resolve().parents[1] / "tools" / "phase1_metrics.py"
SPEC = importlib.util.spec_from_file_location("phase1_metrics", MODULE_PATH)
phase1_metrics = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = phase1_metrics
assert SPEC.loader is not None
SPEC.loader.exec_module(phase1_metrics)


class Phase1MetricsTests(unittest.TestCase):
    def test_build_metrics_derives_required_fields(self):
        report = {
            "meta": {
                "strategy": {
                    "starting_equity": 1000.0,
                    "fee_bps": 8.0,
                    "slippage_bps": 2.0,
                }
            },
            "markets": {
                "SOL-USD": {
                    "default_variant": "sma_baseline",
                    "variants": {
                        "sma_baseline": {
                            "metrics": {"total_return": 0.01, "bars": 300},
                            "attribution": {"pnl_abs": 10.0, "gross_turnover": 1500.0},
                            "trades": [],
                        }
                    },
                }
            },
        }
        promotion = {"source_report": "report.json", "market": "SOL-USD", "variant": "sma_baseline"}
        replay_acceptance = {"status": "pass"}
        paper_soak = {"pass": True, "halt_count": 0, "failed_probes": 0, "max_abs_unhedged_delta": 1.0}

        payload = phase1_metrics.build_metrics(
            report=report,
            promotion=promotion,
            replay_acceptance=replay_acceptance,
            paper_soak=paper_soak,
            market="SOL-USD",
            variant="sma_baseline",
            max_unhedged_delta=10.0,
        )

        self.assertEqual(payload["net_pnl_after_costs"], 10.0)
        self.assertAlmostEqual(payload["fees"], 1.2)
        self.assertAlmostEqual(payload["slippage"], 0.3)
        self.assertFalse(payload["unexpected_auto_halt"])
        self.assertFalse(payload["max_unhedged_delta_breached"])

    def test_choose_market_variant_falls_back_to_promotion(self):
        report = {
            "markets": {
                "SOL-USD": {
                    "variants": {"sma_baseline": {}},
                    "default_variant": "sma_baseline",
                }
            }
        }
        market, variant = phase1_metrics.choose_market_variant(
            report,
            {"market": "SOL-USD", "variant": "sma_baseline"},
            None,
            None,
        )
        self.assertEqual((market, variant), ("SOL-USD", "sma_baseline"))

    def test_cli_writes_metrics_json(self):
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            report_path = tmp_path / "report.json"
            promotion_path = tmp_path / "promotion.json"
            replay_path = tmp_path / "replay_acceptance.json"
            paper_path = tmp_path / "paper_soak.json"
            out_path = tmp_path / "metrics.json"

            report_path.write_text(
                json.dumps(
                    {
                        "meta": {"strategy": {"starting_equity": 1000.0, "fee_bps": 8.0, "slippage_bps": 2.0}},
                        "markets": {
                            "SOL-USD": {
                                "default_variant": "sma_baseline",
                                "variants": {
                                    "sma_baseline": {
                                        "metrics": {"total_return": 0.01, "bars": 300},
                                        "attribution": {"pnl_abs": 10.0, "gross_turnover": 1500.0},
                                        "trades": [],
                                    }
                                },
                            }
                        },
                    }
                ),
                encoding="utf-8",
            )
            promotion_path.write_text(
                json.dumps({"source_report": str(report_path), "market": "SOL-USD", "variant": "sma_baseline"}),
                encoding="utf-8",
            )
            replay_path.write_text(json.dumps({"status": "pass"}), encoding="utf-8")
            paper_path.write_text(
                json.dumps(
                    {"pass": True, "halt_count": 0, "failed_probes": 0, "max_abs_unhedged_delta": 1.0}
                ),
                encoding="utf-8",
            )

            saved_argv = sys.argv
            try:
                sys.argv = [
                    "phase1_metrics.py",
                    "--report",
                    str(report_path),
                    "--promotion",
                    str(promotion_path),
                    "--replay-acceptance",
                    str(replay_path),
                    "--paper-soak",
                    str(paper_path),
                    "--out",
                    str(out_path),
                ]
                exit_code = phase1_metrics.main()
            finally:
                sys.argv = saved_argv

            self.assertEqual(exit_code, 0)
            payload = json.loads(out_path.read_text(encoding="utf-8"))
            self.assertEqual(payload["market"], "SOL-USD")


if __name__ == "__main__":
    unittest.main()
