use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{AgentProposal, ProposalKind};

/// Per-parameter attribution from a backtest or optimizer run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterAttribution {
    pub parameter: String,
    /// Normalized sensitivity score: how much PnL changes per unit change in this parameter.
    /// Positive = increasing parameter helps, negative = hurts.
    pub sensitivity: f64,
    /// The current parameter value in the evaluated candidate.
    pub current_value: f64,
    /// Suggested adjusted value, bounded by `adjustment_bounds`.
    pub suggested_value: f64,
    pub adjustment_rationale: String,
}

/// Bounds applied to any parameter adjustment to prevent unbounded optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdjustmentBounds {
    pub min: f64,
    pub max: f64,
    /// Maximum fractional change from current_value allowed per proposal (e.g., 0.2 = ±20%).
    pub max_step_fraction: f64,
}

impl AdjustmentBounds {
    pub fn clamp_step(&self, current: f64, candidate: f64) -> f64 {
        let max_delta = current.abs() * self.max_step_fraction;
        let delta = (candidate - current).clamp(-max_delta, max_delta);
        (current + delta).clamp(self.min, self.max)
    }
}

/// A bounded parameter adjustment proposal derived from attribution analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterAdjustmentProposal {
    pub strategy_id: String,
    pub candidate_id: String,
    pub adjustments: Vec<ParameterAttribution>,
    /// Evidence tying this proposal to optimizer/replay artifacts.
    pub artifact_context: String,
}

impl ParameterAdjustmentProposal {
    pub fn is_empty(&self) -> bool {
        self.adjustments.is_empty()
    }

    pub fn to_agent_proposal(&self, model_source: impl Into<String>) -> AgentProposal {
        AgentProposal::new(
            ProposalKind::StrategyAdjustment {
                parameter: "bounded_parameter_adjustments".to_string(),
                value: json!({
                    "strategy_id": self.strategy_id,
                    "candidate_id": self.candidate_id,
                    "adjustments": self.adjustments,
                    "artifact_context": self.artifact_context,
                }),
            },
            format!(
                "{} bounded parameter adjustment(s) derived from attribution evidence",
                self.adjustments.len()
            ),
            self.context_value(),
            model_source,
        )
    }

    fn context_value(&self) -> Value {
        json!({
            "strategy_id": self.strategy_id,
            "candidate_id": self.candidate_id,
            "artifact_context": self.artifact_context,
        })
    }
}

/// Generate bounded parameter adjustment proposals from attribution data.
///
/// Only parameters with `|sensitivity| >= sensitivity_threshold` and a non-trivial
/// suggested step are included. The step is further clamped by `AdjustmentBounds`.
pub fn generate_adjustments(
    strategy_id: impl Into<String>,
    candidate_id: impl Into<String>,
    attributions: &[(String, f64, f64)], // (param, sensitivity, current_value)
    bounds_map: &std::collections::HashMap<String, AdjustmentBounds>,
    sensitivity_threshold: f64,
    artifact_context: impl Into<String>,
) -> ParameterAdjustmentProposal {
    let mut adjustments = Vec::new();

    for (param, sensitivity, current) in attributions {
        if sensitivity.abs() < sensitivity_threshold {
            continue;
        }
        let naive_suggestion = current + sensitivity * current.abs().max(1.0) * 0.1;
        let suggested = if let Some(bounds) = bounds_map.get(param.as_str()) {
            bounds.clamp_step(*current, naive_suggestion)
        } else {
            naive_suggestion
        };
        if (suggested - current).abs() < 1e-9 {
            continue;
        }
        adjustments.push(ParameterAttribution {
            parameter: param.clone(),
            sensitivity: *sensitivity,
            current_value: *current,
            suggested_value: suggested,
            adjustment_rationale: format!(
                "sensitivity={:.4}, step={:.4}",
                sensitivity,
                suggested - current
            ),
        });
    }

    ParameterAdjustmentProposal {
        strategy_id: strategy_id.into(),
        candidate_id: candidate_id.into(),
        adjustments,
        artifact_context: artifact_context.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn low_sensitivity_filtered() {
        let proposal = generate_adjustments(
            "s1",
            "c1",
            &[("rsi_len".into(), 0.001, 14.0)],
            &HashMap::new(),
            0.05,
            "backtest-artifact",
        );
        assert!(proposal.is_empty());
    }

    #[test]
    fn high_sensitivity_included() {
        let proposal = generate_adjustments(
            "s1",
            "c1",
            &[("atr_mult".into(), 0.8, 2.0)],
            &HashMap::new(),
            0.05,
            "backtest-artifact",
        );
        assert!(!proposal.is_empty());
        assert_eq!(proposal.adjustments[0].parameter, "atr_mult");
    }

    #[test]
    fn adjustment_bounds_clamp_step() {
        let bounds = AdjustmentBounds {
            min: 1.0,
            max: 5.0,
            max_step_fraction: 0.1,
        };
        // Current 2.0, want to move to 3.0 (50% step). Clamp to 10% = 0.2 → 2.2.
        let clamped = bounds.clamp_step(2.0, 3.0);
        assert!((clamped - 2.2).abs() < 1e-6);
    }

    #[test]
    fn bounds_clamp_min_max() {
        let bounds = AdjustmentBounds {
            min: 1.0,
            max: 5.0,
            max_step_fraction: 10.0, // very large fraction
        };
        let clamped = bounds.clamp_step(2.0, 0.0); // try to go below min
        assert!(clamped >= 1.0);
    }

    #[test]
    fn adjustment_proposal_routes_to_existing_review_kind() {
        let proposal = generate_adjustments(
            "s1",
            "c1",
            &[("atr_mult".into(), 0.8, 2.0)],
            &HashMap::new(),
            0.05,
            "optimizer-run-1",
        );
        let agent_proposal = proposal.to_agent_proposal("local:test");
        match agent_proposal.kind {
            ProposalKind::StrategyAdjustment { parameter, value } => {
                assert_eq!(parameter, "bounded_parameter_adjustments");
                assert_eq!(value["strategy_id"], "s1");
                assert_eq!(value["candidate_id"], "c1");
            }
            other => panic!("unexpected proposal kind: {other:?}"),
        }
    }
}
