use crate::ir::{
    CompareNode, InputNode, MaKind, RuleNode, SizingHint, StrategyIrDef, IR_VERSION,
};
use crate::types::{MaType, StrategyProfile};
use std::collections::HashMap;

const DEFAULT_FIXED_FRACTION: f64 = 0.10;

fn ma_kind_from(mt: &MaType) -> MaKind {
    match mt {
        MaType::Ema => MaKind::Ema,
        MaType::Sma => MaKind::Sma,
        MaType::Wma => MaKind::Wma,
        MaType::Hma => MaKind::Hma,
        MaType::Dema => MaKind::Dema,
        MaType::Tema => MaKind::Tema,
        MaType::Vwma => MaKind::Vwma,
        MaType::Rma => MaKind::Rma,
        MaType::Zlema => MaKind::Zlema,
    }
}

/// Convert a `StrategyProfile` into a `StrategyIrDef`.
pub fn from_profile(profile: &StrategyProfile) -> StrategyIrDef {
    let ma_kind = ma_kind_from(&profile.indicators.ma_type);

    let fast_ma = InputNode::Ma {
        source: Box::new(InputNode::Close),
        ma_type: ma_kind,
        period: profile.indicators.ma_fast,
    };
    let slow_ma = InputNode::Ma {
        source: Box::new(InputNode::Close),
        ma_type: ma_kind,
        period: profile.indicators.ma_slow,
    };
    let rsi_node = InputNode::Rsi {
        source: Box::new(InputNode::Close),
        period: profile.indicators.rsi_len,
    };

    let entry_rule = RuleNode::All {
        conditions: vec![
            CompareNode::CrossOver {
                fast: fast_ma.clone(),
                slow: slow_ma.clone(),
            },
            // Guard: do not enter if RSI is already overbought at time of crossover.
            CompareNode::BelowThreshold {
                source: rsi_node,
                threshold: profile.indicators.rsi_overbought,
            },
        ],
    };

    let exit_rule = RuleNode::All {
        conditions: vec![CompareNode::CrossUnder {
            fast: fast_ma,
            slow: slow_ma,
        }],
    };

    let mut provenance = HashMap::new();
    provenance.insert("profile_id".into(), profile.profile_id.clone());
    provenance.insert(
        "buy_threshold".into(),
        profile.fusion.buy_threshold.to_string(),
    );
    provenance.insert(
        "sell_threshold".into(),
        profile.fusion.sell_threshold.to_string(),
    );

    StrategyIrDef {
        ir_version: IR_VERSION,
        name: profile.name.clone(),
        product_id: profile.product_id.clone(),
        granularity_sec: profile.granularity_sec,
        entry_rule,
        exit_rule,
        sizing: Some(SizingHint::FixedFraction { fraction: DEFAULT_FIXED_FRACTION }),
        provenance,
    }
}

/// Convert a strategy-lab promotion artifact JSON value into a `StrategyIrDef`.
///
/// Uses SMA 9/21 crossover as the canonical sma_baseline configuration
/// (short_window=9, long_window=21 from coinbase_strategy_lab.example.json).
/// Indicator fields in the artifact JSON are not parsed — this produces the
/// standard baseline IR for replay and paper evaluation.
pub fn from_promotion_json(json: &serde_json::Value) -> Option<StrategyIrDef> {
    let market = json.get("market")?.as_str()?.to_string();
    let variant = json
        .get("variant")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    let source_report = json
        .get("source_report")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let mut provenance = HashMap::new();
    provenance.insert("market".into(), market.clone());
    provenance.insert("variant".into(), variant.clone());
    provenance.insert("source_report".into(), source_report);

    Some(StrategyIrDef {
        ir_version: IR_VERSION,
        name: format!("{}-{}", market, variant),
        product_id: market,
        granularity_sec: 300,
        entry_rule: RuleNode::All {
            conditions: vec![CompareNode::CrossOver {
                fast: InputNode::Ma {
                    source: Box::new(InputNode::Close),
                    ma_type: MaKind::Sma,
                    period: 9,
                },
                slow: InputNode::Ma {
                    source: Box::new(InputNode::Close),
                    ma_type: MaKind::Sma,
                    period: 21,
                },
            }],
        },
        exit_rule: RuleNode::All {
            conditions: vec![CompareNode::CrossUnder {
                fast: InputNode::Ma {
                    source: Box::new(InputNode::Close),
                    ma_type: MaKind::Sma,
                    period: 9,
                },
                slow: InputNode::Ma {
                    source: Box::new(InputNode::Close),
                    ma_type: MaKind::Sma,
                    period: 21,
                },
            }],
        },
        sizing: None,
        provenance,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{CompareNode, RuleNode};
    use crate::types::StrategyProfile;
    use serde_json::json;

    #[test]
    fn from_profile_produces_ma_crossover_entry_rule() {
        let profile = StrategyProfile::default();
        let ir = from_profile(&profile);
        assert_eq!(ir.ir_version, IR_VERSION);
        assert_eq!(ir.product_id, profile.product_id);
        assert_eq!(
            ir.provenance.get("profile_id").map(String::as_str),
            Some("default")
        );
        match &ir.entry_rule {
            RuleNode::All { conditions } => {
                assert!(!conditions.is_empty());
                assert!(matches!(conditions[0], CompareNode::CrossOver { .. }));
            }
            other => panic!("expected All, got {:?}", other),
        }
    }

    #[test]
    fn from_profile_exit_rule_is_cross_under() {
        let profile = StrategyProfile::default();
        let ir = from_profile(&profile);
        match &ir.exit_rule {
            RuleNode::All { conditions } => {
                assert!(!conditions.is_empty(), "exit conditions must not be empty");
                assert!(matches!(conditions[0], CompareNode::CrossUnder { .. }));
            }
            other => panic!("expected All with CrossUnder, got {:?}", other),
        }
    }

    #[test]
    fn from_profile_records_thresholds_in_provenance() {
        let profile = StrategyProfile::default();
        let ir = from_profile(&profile);
        assert!(ir.provenance.contains_key("buy_threshold"));
        assert!(ir.provenance.contains_key("sell_threshold"));
    }

    #[test]
    fn from_promotion_json_extracts_market_and_variant() {
        let json = json!({
            "market": "BTC-USD",
            "variant": "sma_baseline",
            "source_report": "data/strategy_lab/backtest.json"
        });
        let ir = from_promotion_json(&json).expect("should produce IR");
        assert_eq!(ir.product_id, "BTC-USD");
        assert_eq!(
            ir.provenance.get("variant").map(String::as_str),
            Some("sma_baseline")
        );
    }

    #[test]
    fn from_promotion_json_returns_none_on_missing_market() {
        let json = json!({ "variant": "sma_baseline" });
        assert!(from_promotion_json(&json).is_none());
    }

    #[test]
    fn adapted_ir_roundtrips_json() {
        let profile = StrategyProfile::default();
        let ir = from_profile(&profile);
        let json = serde_json::to_string(&ir).expect("serialize");
        let restored: StrategyIrDef = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(ir, restored);
    }
}
