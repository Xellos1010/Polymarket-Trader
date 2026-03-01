use crate::backtest::run_backtest;
use crate::types::{StrategyProfile, TuningCandidate, TuningReport};
use rand::{rngs::StdRng, Rng, SeedableRng};

fn score_report(ret: f64, dd: f64, trades: usize) -> f64 {
    ret - (dd * 0.8) - ((trades as f64).ln_1p() * 0.01)
}

fn mutate_profile(base: &StrategyProfile, rng: &mut StdRng) -> StrategyProfile {
    let mut p = base.clone();
    p.version = p.version.saturating_add(1);

    p.fusion.buy_threshold = rng.gen_range(0.45..0.80);
    p.fusion.sell_threshold = -rng.gen_range(0.45..0.80);
    p.fusion.min_confluence = rng.gen_range(2..=4);
    p.fusion.neutral_regime_multiplier = rng.gen_range(1.0..1.8);

    p.indicators.ma_fast = rng.gen_range(10..=80);
    p.indicators.ma_slow = rng.gen_range(120..=260);
    p.indicators.rsi_len = rng.gen_range(8..=21);
    p.indicators.bb_len = rng.gen_range(14..=40);
    p.indicators.ichimoku_conv = rng.gen_range(6..=12);
    p.indicators.ichimoku_base = rng.gen_range(18..=34);
    p.indicators.ichimoku_span_b = rng.gen_range(34..=70);

    for key in [
        "ma_regime",
        "rsi",
        "fib_bb",
        "ichimoku",
        "macd",
        "adx",
        "atr",
        "volume",
        "vwap_dev",
        "stoch_rsi",
    ] {
        p.weights.insert(key.to_string(), rng.gen_range(0.1..1.4));
    }

    p
}

pub fn optimize_random_walk_forward(
    base: &StrategyProfile,
    candles: &[crate::types::Candle],
    iterations: usize,
    walk_forward_splits: usize,
    seed: u64,
) -> TuningReport {
    let mut rng = StdRng::seed_from_u64(seed);
    let splits = walk_forward_splits.max(1);
    let n = candles.len();
    let chunk = (n / splits).max(2);

    let mut scored: Vec<(f64, StrategyProfile, crate::types::StrategyRunReport)> = Vec::new();

    for _ in 0..iterations.max(1) {
        let candidate = mutate_profile(base, &mut rng);
        let mut fold_score = 0.0;
        let mut last_report = None;
        let mut folds = 0usize;

        for s in 0..splits {
            let start = s * chunk;
            let end = ((s + 1) * chunk).min(n);
            if end.saturating_sub(start) < 50 {
                continue;
            }
            let report = run_backtest(&candidate, &candles[start..end]);
            fold_score += score_report(
                report.total_return_pct,
                report.max_drawdown_pct,
                report.trades,
            );
            last_report = Some(report);
            folds += 1;
        }

        if let Some(report) = last_report {
            let average_score = if folds > 0 {
                fold_score / folds as f64
            } else {
                f64::NEG_INFINITY
            };
            scored.push((average_score, candidate, report));
        }
    }

    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    let top: Vec<TuningCandidate> = scored
        .into_iter()
        .take(20)
        .enumerate()
        .map(|(i, (score, profile, report))| TuningCandidate {
            rank: i + 1,
            score,
            profile,
            report,
        })
        .collect();

    TuningReport {
        created_ts_ms: chrono::Utc::now().timestamp_millis(),
        iterations,
        walk_forward_splits: splits,
        top,
    }
}
