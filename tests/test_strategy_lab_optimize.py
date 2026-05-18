import importlib.util
import sys
import unittest
from pathlib import Path
from unittest.mock import patch


MODULE_PATH = Path(__file__).resolve().parents[1] / "tools" / "coinbase_strategy_lab.py"
SPEC = importlib.util.spec_from_file_location("coinbase_strategy_lab", MODULE_PATH)
coinbase_strategy_lab = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
sys.modules[SPEC.name] = coinbase_strategy_lab
SPEC.loader.exec_module(coinbase_strategy_lab)


def rising_candles(count: int = 48):
    candles = []
    base = 100.0
    for idx in range(count):
        price = base + idx * 0.8
        candles.append(
            coinbase_strategy_lab.Candle(
                ts_ms=idx * 300_000,
                open=price - 0.2,
                high=price + 0.4,
                low=price - 0.5,
                close=price,
                volume=10.0 + idx,
            )
        )
    return candles


def drawdown_candles():
    prices = [
        100.0,
        105.0,
        110.0,
        115.0,
        90.0,
        92.0,
        95.0,
        96.0,
        97.0,
        98.0,
        99.0,
        100.0,
        101.0,
        102.0,
        103.0,
    ]
    candles = []
    for idx, price in enumerate(prices):
        candles.append(
            coinbase_strategy_lab.Candle(
                ts_ms=idx * 300_000,
                open=price,
                high=price + 0.5,
                low=price - 0.5,
                close=price,
                volume=20.0,
            )
        )
    return candles


def base_config():
    return {
        "provider": "coinbase",
        "granularity_sec": 300,
        "backtest": {
            "markets": ["BTC-USD", "ETH-USD"],
            "limit": 64,
            "strategy": {
                "short_window": 3,
                "long_window": 8,
                "trade_fee_bps": 1.0,
                "slippage_bps": 1.0,
                "starting_equity": 1000.0,
            },
            "variants": [{"name": "sma_baseline", "bias_gain": 0.0, "plugins": []}],
        },
        "optimize": {
            "short_windows": [3],
            "long_windows": [8],
            "top_n": 5,
            "stability_splits": 3,
            "max_candidates": 10,
            "max_markets": 2,
            "max_variants": 1,
            "drawdown_penalty": 0.8,
            "turnover_penalty": 0.2,
            "stability_penalty": 0.35,
        },
    }


class StrategyLabOptimizeTests(unittest.TestCase):
    def test_optimize_exposes_objective_breakdown_and_gates(self):
        cfg = base_config()
        with patch.object(
            coinbase_strategy_lab,
            "fetch_candles_retry",
            side_effect=lambda provider, market, granularity, limit, start=None, end=None, attempts=3: rising_candles(),
        ):
            payload = coinbase_strategy_lab.run_optimize_data(cfg)

        self.assertIn("objective", payload["meta"])
        self.assertEqual(payload["meta"]["candidate_count"], 1)
        top = payload["top"][0]
        self.assertEqual(top["rank"], 1)
        self.assertIn("objective_breakdown", top)
        self.assertIn("stability", top)
        self.assertIn("risk_gate", top)
        self.assertIn("promotion_gate", top)
        self.assertIn("rejection_reasons", top)
        self.assertEqual(top["risk_gate"]["status"], "pass")
        self.assertEqual(top["promotion_gate"]["status"], "eligible_for_manual_review")
        self.assertEqual(top["rejection_reasons"], [])

    def test_optimize_respects_candidate_cap(self):
        cfg = base_config()
        cfg["optimize"]["short_windows"] = [3, 4]
        cfg["optimize"]["long_windows"] = [8, 9]
        cfg["optimize"]["max_candidates"] = 1
        cfg["backtest"]["variants"] = [
            {"name": "v1", "bias_gain": 0.0, "plugins": []},
            {"name": "v2", "bias_gain": 0.0, "plugins": []},
        ]
        cfg["optimize"]["max_variants"] = 2
        with patch.object(
            coinbase_strategy_lab,
            "fetch_candles_retry",
            side_effect=lambda provider, market, granularity, limit, start=None, end=None, attempts=3: rising_candles(),
        ):
            payload = coinbase_strategy_lab.run_optimize_data(cfg)

        self.assertEqual(payload["meta"]["candidate_count"], 1)
        self.assertEqual(len(payload["top"]), 1)

    def test_optimize_marks_runtime_limit(self):
        cfg = base_config()
        cfg["optimize"]["max_runtime_sec"] = 0.5
        monotonic_values = iter([0.0, 1.0, 1.0])
        with patch.object(
            coinbase_strategy_lab,
            "fetch_candles_retry",
            side_effect=lambda provider, market, granularity, limit, start=None, end=None, attempts=3: rising_candles(),
        ), patch.object(
            coinbase_strategy_lab.time,
            "monotonic",
            side_effect=lambda: next(monotonic_values),
        ):
            payload = coinbase_strategy_lab.run_optimize_data(cfg)

        self.assertTrue(payload["meta"]["bounded_search"]["runtime_limited"])
        self.assertEqual(payload["meta"]["candidate_count"], 0)

    def test_optimize_hard_rejects_risk_failures(self):
        cfg = base_config()
        cfg["backtest"]["markets"] = ["BTC-USD"]
        cfg["optimize"]["max_drawdown"] = 0.02
        cfg["optimize"]["min_total_return"] = 0.5
        cfg["optimize"]["long_windows"] = [6]
        with patch.object(
            coinbase_strategy_lab,
            "fetch_candles_retry",
            side_effect=lambda provider, market, granularity, limit, start=None, end=None, attempts=3: drawdown_candles(),
        ):
            payload = coinbase_strategy_lab.run_optimize_data(cfg)

        top = payload["top"][0]
        self.assertEqual(top["risk_gate"]["status"], "fail")
        self.assertEqual(top["promotion_gate"]["status"], "blocked")
        self.assertTrue(top["rejection_reasons"])
        self.assertTrue(any(reason.startswith("risk:") for reason in top["rejection_reasons"]))


if __name__ == "__main__":
    unittest.main()
