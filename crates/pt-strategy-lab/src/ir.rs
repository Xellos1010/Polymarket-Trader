use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const IR_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum InputNode {
    Close,
    High,
    Low,
    Volume,
    Ma {
        source: Box<InputNode>,
        ma_type: MaKind,
        period: usize,
    },
    Rsi {
        source: Box<InputNode>,
        period: usize,
    },
    Atr {
        period: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaKind {
    Ema,
    Sma,
    Wma,
    Hma,
    Dema,
    Tema,
    Vwma,
    Rma,
    Zlema,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "op")]
pub enum CompareNode {
    Gt { left: InputNode, right: InputNode },
    Lt { left: InputNode, right: InputNode },
    CrossOver { fast: InputNode, slow: InputNode },
    CrossUnder { fast: InputNode, slow: InputNode },
    AboveThreshold { source: InputNode, threshold: f64 },
    BelowThreshold { source: InputNode, threshold: f64 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum RuleNode {
    All { conditions: Vec<CompareNode> },
    Any { conditions: Vec<CompareNode> },
    Never,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum SizingHint {
    FixedFraction { fraction: f64 },
    FixedNotional { usd: f64 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StrategyIrDef {
    pub ir_version: u32,
    pub name: String,
    pub product_id: String,
    pub granularity_sec: u32,
    pub entry_rule: RuleNode,
    pub exit_rule: RuleNode,
    pub sizing: Option<SizingHint>,
    pub provenance: HashMap<String, String>,
}

/// Action produced by IR evaluation at a single bar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IrAction {
    Buy,
    Sell,
    Hold,
}

/// A single bar's evaluation output from `eval_ir`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrDecision {
    pub ts_ms: i64,
    pub action: IrAction,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn sma_crossover_ir() -> StrategyIrDef {
        StrategyIrDef {
            ir_version: IR_VERSION,
            name: "SMA Crossover".into(),
            product_id: "BTC-USD".into(),
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
            sizing: Some(SizingHint::FixedFraction { fraction: 0.10 }),
            provenance: HashMap::new(),
        }
    }

    #[test]
    fn ir_roundtrips_json() {
        let original = sma_crossover_ir();
        let json = serde_json::to_string(&original).expect("serialize");
        let restored: StrategyIrDef = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(original, restored);
        assert_eq!(restored.ir_version, IR_VERSION);
    }

    #[test]
    fn ir_version_field_is_present_in_json() {
        let ir = sma_crossover_ir();
        let json = serde_json::to_string(&ir).expect("serialize");
        let v: serde_json::Value = serde_json::from_str(&json).expect("parse");
        assert_eq!(v["ir_version"], IR_VERSION);
    }

    #[test]
    fn rsi_ir_roundtrips_json() {
        let ir = StrategyIrDef {
            ir_version: IR_VERSION,
            name: "RSI".into(),
            product_id: "ETH-USD".into(),
            granularity_sec: 300,
            entry_rule: RuleNode::All {
                conditions: vec![CompareNode::BelowThreshold {
                    source: InputNode::Rsi {
                        source: Box::new(InputNode::Close),
                        period: 14,
                    },
                    threshold: 30.0,
                }],
            },
            exit_rule: RuleNode::All {
                conditions: vec![CompareNode::AboveThreshold {
                    source: InputNode::Rsi {
                        source: Box::new(InputNode::Close),
                        period: 14,
                    },
                    threshold: 70.0,
                }],
            },
            sizing: None,
            provenance: HashMap::new(),
        };
        let json = serde_json::to_string(&ir).expect("serialize");
        let restored: StrategyIrDef = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(ir, restored);
    }
}
