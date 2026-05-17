use crate::indicators;
use crate::ir::{CompareNode, InputNode, IrAction, IrDecision, MaKind, RuleNode, StrategyIrDef};
use crate::types::{Candle, MaType};

fn resolve(node: &InputNode, candles: &[Candle]) -> Vec<Option<f64>> {
    match node {
        InputNode::Close => candles.iter().map(|c| Some(c.close)).collect(),
        InputNode::High => candles.iter().map(|c| Some(c.high)).collect(),
        InputNode::Low => candles.iter().map(|c| Some(c.low)).collect(),
        InputNode::Volume => candles.iter().map(|c| Some(c.volume)).collect(),
        InputNode::Ma {
            source,
            ma_type,
            period,
        } => {
            let src = resolve(source, candles);
            let vals: Vec<f64> = src.iter().map(|v| v.unwrap_or(0.0)).collect();
            let vols: Vec<f64> = candles.iter().map(|c| c.volume).collect();
            let rust_ma_type = match ma_type {
                MaKind::Ema => MaType::Ema,
                MaKind::Sma => MaType::Sma,
                MaKind::Wma => MaType::Wma,
                MaKind::Hma => MaType::Hma,
                MaKind::Dema => MaType::Dema,
                MaKind::Tema => MaType::Tema,
                MaKind::Vwma => MaType::Vwma,
                MaKind::Rma => MaType::Rma,
                MaKind::Zlema => MaType::Zlema,
            };
            indicators::ma(&vals, &vols, *period, &rust_ma_type)
        }
        InputNode::Rsi { source, period } => {
            let src = resolve(source, candles);
            let vals: Vec<f64> = src.iter().map(|v| v.unwrap_or(0.0)).collect();
            indicators::rsi(&vals, *period)
        }
        InputNode::Atr { period } => indicators::atr(candles, *period),
    }
}

fn eval_compare(node: &CompareNode, candles: &[Candle]) -> Vec<bool> {
    let n = candles.len();
    match node {
        CompareNode::Gt { left, right } => {
            let l = resolve(left, candles);
            let r = resolve(right, candles);
            (0..n)
                .map(|i| matches!((l[i], r[i]), (Some(lv), Some(rv)) if lv > rv))
                .collect()
        }
        CompareNode::Lt { left, right } => {
            let l = resolve(left, candles);
            let r = resolve(right, candles);
            (0..n)
                .map(|i| matches!((l[i], r[i]), (Some(lv), Some(rv)) if lv < rv))
                .collect()
        }
        CompareNode::CrossOver { fast, slow } => {
            let f = resolve(fast, candles);
            let s = resolve(slow, candles);
            let mut out = vec![false; n];
            for i in 1..n {
                if let (Some(f_cur), Some(s_cur)) = (f[i], s[i]) {
                    // Treat unknown prior state as "no cross possible" (suppress warmup signal).
                    let prev_above = match (f[i - 1], s[i - 1]) {
                        (Some(fp), Some(sp)) => fp > sp,
                        _ => true,
                    };
                    out[i] = f_cur > s_cur && !prev_above;
                }
            }
            out
        }
        CompareNode::CrossUnder { fast, slow } => {
            let f = resolve(fast, candles);
            let s = resolve(slow, candles);
            let mut out = vec![false; n];
            for i in 1..n {
                if let (Some(f_cur), Some(s_cur)) = (f[i], s[i]) {
                    // Treat unknown prior state as "no cross possible" (suppress warmup signal).
                    let prev_below = match (f[i - 1], s[i - 1]) {
                        (Some(fp), Some(sp)) => fp < sp,
                        _ => true,
                    };
                    out[i] = f_cur < s_cur && !prev_below;
                }
            }
            out
        }
        CompareNode::AboveThreshold { source, threshold } => {
            let s = resolve(source, candles);
            s.iter()
                .map(|v| matches!(v, Some(x) if *x > *threshold))
                .collect()
        }
        CompareNode::BelowThreshold { source, threshold } => {
            let s = resolve(source, candles);
            s.iter()
                .map(|v| matches!(v, Some(x) if *x < *threshold))
                .collect()
        }
    }
}

fn eval_rule(rule: &RuleNode, candles: &[Candle]) -> Vec<bool> {
    let n = candles.len();
    match rule {
        RuleNode::All { conditions } => {
            if conditions.is_empty() {
                return vec![false; n];
            }
            let mut acc = vec![true; n];
            for cond in conditions {
                let result = eval_compare(cond, candles);
                for i in 0..n {
                    acc[i] = acc[i] && result[i];
                }
            }
            acc
        }
        RuleNode::Any { conditions } => {
            if conditions.is_empty() {
                return vec![false; n];
            }
            let mut acc = vec![false; n];
            for cond in conditions {
                let result = eval_compare(cond, candles);
                for i in 0..n {
                    acc[i] = acc[i] || result[i];
                }
            }
            acc
        }
        RuleNode::Never => vec![false; n],
    }
}

/// Evaluate a `StrategyIrDef` against a candle slice.
///
/// Returns one `IrDecision` per candle. A position is modelled as a simple
/// long-only state machine: `Buy` opens, `Sell` closes, `Hold` does nothing.
/// Exit takes priority if already in position.
pub fn eval_ir(ir: &StrategyIrDef, candles: &[Candle]) -> Vec<IrDecision> {
    let entry = eval_rule(&ir.entry_rule, candles);
    let exit = eval_rule(&ir.exit_rule, candles);
    let mut in_position = false;
    candles
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let action = if in_position && exit[i] {
                in_position = false;
                IrAction::Sell
            } else if !in_position && entry[i] {
                in_position = true;
                IrAction::Buy
            } else {
                IrAction::Hold
            };
            IrDecision {
                ts_ms: c.ts_ms,
                action,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{CompareNode, InputNode, MaKind, RuleNode, StrategyIrDef, IR_VERSION};
    use crate::types::Candle;
    use std::collections::HashMap;

    fn candles_rising(n: usize, start: f64, step: f64) -> Vec<Candle> {
        (0..n)
            .map(|i| {
                let p = start + i as f64 * step;
                Candle {
                    ts_ms: i as i64 * 300_000,
                    open: p,
                    high: p + 1.0,
                    low: p - 1.0,
                    close: p,
                    volume: 100.0,
                }
            })
            .collect()
    }

    fn sma_cross_ir(fast: usize, slow: usize) -> StrategyIrDef {
        StrategyIrDef {
            ir_version: IR_VERSION,
            name: "test".into(),
            product_id: "BTC-USD".into(),
            granularity_sec: 300,
            entry_rule: RuleNode::All {
                conditions: vec![CompareNode::CrossOver {
                    fast: InputNode::Ma {
                        source: Box::new(InputNode::Close),
                        ma_type: MaKind::Sma,
                        period: fast,
                    },
                    slow: InputNode::Ma {
                        source: Box::new(InputNode::Close),
                        ma_type: MaKind::Sma,
                        period: slow,
                    },
                }],
            },
            exit_rule: RuleNode::All {
                conditions: vec![CompareNode::CrossUnder {
                    fast: InputNode::Ma {
                        source: Box::new(InputNode::Close),
                        ma_type: MaKind::Sma,
                        period: fast,
                    },
                    slow: InputNode::Ma {
                        source: Box::new(InputNode::Close),
                        ma_type: MaKind::Sma,
                        period: slow,
                    },
                }],
            },
            sizing: None,
            provenance: HashMap::new(),
        }
    }

    #[test]
    fn eval_produces_one_decision_per_candle() {
        let candles = candles_rising(50, 100.0, 1.0);
        let ir = sma_cross_ir(3, 5);
        let decisions = eval_ir(&ir, &candles);
        assert_eq!(decisions.len(), candles.len());
    }

    #[test]
    fn eval_timestamps_match_candles() {
        let candles = candles_rising(30, 50.0, 2.0);
        let ir = sma_cross_ir(3, 5);
        let decisions = eval_ir(&ir, &candles);
        for (d, c) in decisions.iter().zip(candles.iter()) {
            assert_eq!(d.ts_ms, c.ts_ms);
        }
    }

    #[test]
    fn eval_never_rule_produces_only_hold() {
        let candles = candles_rising(20, 100.0, 1.0);
        let ir = StrategyIrDef {
            ir_version: IR_VERSION,
            name: "never".into(),
            product_id: "BTC-USD".into(),
            granularity_sec: 300,
            entry_rule: RuleNode::Never,
            exit_rule: RuleNode::Never,
            sizing: None,
            provenance: HashMap::new(),
        };
        let decisions = eval_ir(&ir, &candles);
        assert!(decisions.iter().all(|d| d.action == IrAction::Hold));
    }

    #[test]
    fn eval_rising_series_produces_at_least_one_buy() {
        // Flat warm-up so both SMAs settle, then a sharp price jump makes fast SMA
        // cross above slow SMA — producing a genuine (non-warmup) Buy signal.
        let flat: Vec<Candle> = (0..15)
            .map(|i| Candle {
                ts_ms: i as i64 * 300_000,
                open: 100.0,
                high: 101.0,
                low: 99.0,
                close: 100.0,
                volume: 100.0,
            })
            .collect();
        let rise: Vec<Candle> = (0..35)
            .map(|i| {
                let p = 100.0 + (i + 1) as f64 * 10.0;
                Candle {
                    ts_ms: (i + 15) as i64 * 300_000,
                    open: p,
                    high: p + 1.0,
                    low: p - 1.0,
                    close: p,
                    volume: 100.0,
                }
            })
            .collect();
        let candles: Vec<Candle> = flat.into_iter().chain(rise).collect();
        let ir = sma_cross_ir(3, 10);
        let decisions = eval_ir(&ir, &candles);
        assert!(decisions.iter().any(|d| d.action == IrAction::Buy));
    }

    #[test]
    fn eval_no_double_buys_without_intervening_sell() {
        let candles = candles_rising(50, 100.0, 1.0);
        let ir = sma_cross_ir(3, 5);
        let decisions = eval_ir(&ir, &candles);
        let mut in_pos = false;
        for d in &decisions {
            match d.action {
                IrAction::Buy => {
                    assert!(!in_pos, "double buy without sell");
                    in_pos = true;
                }
                IrAction::Sell => {
                    in_pos = false;
                }
                IrAction::Hold => {}
            }
        }
    }

    #[test]
    fn eval_rsi_below_threshold_entry() {
        let candles: Vec<Candle> = (0..50)
            .map(|i| {
                let p = 200.0 - i as f64 * 3.0;
                Candle {
                    ts_ms: i as i64 * 300_000,
                    open: p,
                    high: p + 0.5,
                    low: (p - 0.5).max(0.01),
                    close: p,
                    volume: 100.0,
                }
            })
            .collect();
        let ir = StrategyIrDef {
            ir_version: IR_VERSION,
            name: "rsi".into(),
            product_id: "BTC-USD".into(),
            granularity_sec: 300,
            entry_rule: RuleNode::All {
                conditions: vec![CompareNode::BelowThreshold {
                    source: InputNode::Rsi {
                        source: Box::new(InputNode::Close),
                        period: 14,
                    },
                    threshold: 40.0,
                }],
            },
            exit_rule: RuleNode::Never,
            sizing: None,
            provenance: HashMap::new(),
        };
        let decisions = eval_ir(&ir, &candles);
        assert!(decisions.iter().any(|d| d.action == IrAction::Buy));
    }

    #[test]
    fn eval_cross_under_produces_sell() {
        // Flat warm-up → sharp rise (crossover Buy) → sharp fall (crossunder Sell).
        let flat: Vec<Candle> = (0..15)
            .map(|i| Candle {
                ts_ms: i as i64 * 300_000,
                open: 100.0,
                high: 101.0,
                low: 99.0,
                close: 100.0,
                volume: 100.0,
            })
            .collect();
        let rise: Vec<Candle> = (0..20)
            .map(|i| {
                let p = 100.0 + (i + 1) as f64 * 10.0;
                Candle {
                    ts_ms: (i + 15) as i64 * 300_000,
                    open: p,
                    high: p + 1.0,
                    low: p - 1.0,
                    close: p,
                    volume: 100.0,
                }
            })
            .collect();
        let fall: Vec<Candle> = (0..25)
            .map(|i| {
                let p = 300.0 - (i + 1) as f64 * 10.0;
                Candle {
                    ts_ms: (i + 35) as i64 * 300_000,
                    open: p,
                    high: p + 1.0,
                    low: p - 1.0,
                    close: p,
                    volume: 100.0,
                }
            })
            .collect();
        let candles: Vec<Candle> = flat.into_iter().chain(rise).chain(fall).collect();
        let ir = sma_cross_ir(3, 10);
        let decisions = eval_ir(&ir, &candles);
        assert!(
            decisions.iter().any(|d| d.action == IrAction::Buy),
            "expected at least one Buy"
        );
        assert!(
            decisions.iter().any(|d| d.action == IrAction::Sell),
            "expected at least one Sell"
        );
    }

    #[test]
    fn eval_rule_any_fires_on_either_condition() {
        // RuleNode::Any: entry fires when EITHER close > 150 OR close > 200.
        // With prices rising from 100 to 200, close > 150 fires first.
        let candles = candles_rising(50, 100.0, 2.0);
        let ir = StrategyIrDef {
            ir_version: IR_VERSION,
            name: "any-test".into(),
            product_id: "BTC-USD".into(),
            granularity_sec: 300,
            entry_rule: RuleNode::Any {
                conditions: vec![
                    CompareNode::AboveThreshold {
                        source: InputNode::Close,
                        threshold: 150.0,
                    },
                    CompareNode::AboveThreshold {
                        source: InputNode::Close,
                        threshold: 200.0,
                    },
                ],
            },
            exit_rule: RuleNode::Never,
            sizing: None,
            provenance: HashMap::new(),
        };
        let decisions = eval_ir(&ir, &candles);
        // At price 152 (bar 26), close > 150 → Buy should fire
        assert!(decisions.iter().any(|d| d.action == IrAction::Buy));
    }
}
