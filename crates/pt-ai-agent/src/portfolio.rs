use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyAllocationInput {
    pub strategy_id: String,
    pub artifact_id: String,
    pub expected_return: f64,
    pub max_drawdown: f64,
    pub current_allocation_usd: f64,
    pub max_allocation_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapitalAllocationPolicy {
    pub total_capital_usd: f64,
    pub min_allocation_usd: f64,
    pub max_strategy_fraction: f64,
    pub drawdown_penalty_weight: f64,
    pub requires_human_approval: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyAllocation {
    pub strategy_id: String,
    pub artifact_id: String,
    pub target_allocation_usd: f64,
    pub target_fraction: f64,
    pub score: f64,
    pub rejection_reasons: Vec<String>,
}

pub fn allocate_capital(
    policy: &CapitalAllocationPolicy,
    inputs: &[StrategyAllocationInput],
) -> Vec<StrategyAllocation> {
    if policy.total_capital_usd <= 0.0 || inputs.is_empty() || !policy.requires_human_approval {
        return inputs
            .iter()
            .map(|input| StrategyAllocation {
                strategy_id: input.strategy_id.clone(),
                artifact_id: input.artifact_id.clone(),
                target_allocation_usd: 0.0,
                target_fraction: 0.0,
                score: 0.0,
                rejection_reasons: if policy.requires_human_approval {
                    vec!["invalid_policy_or_empty_inputs".to_string()]
                } else {
                    vec!["missing_human_approval_gate".to_string()]
                },
            })
            .collect();
    }

    let scored = inputs
        .iter()
        .map(|input| {
            let score = (input.expected_return
                - input.max_drawdown.abs() * policy.drawdown_penalty_weight)
                .max(0.0);
            (input, score)
        })
        .collect::<Vec<_>>();
    let score_sum = scored.iter().map(|(_, score)| *score).sum::<f64>();

    scored
        .into_iter()
        .map(|(input, score)| {
            let mut rejection_reasons = Vec::new();
            if input.artifact_id.trim().is_empty() {
                rejection_reasons.push("missing_artifact".to_string());
            }
            if input.max_allocation_usd <= 0.0 {
                rejection_reasons.push("invalid_max_allocation".to_string());
            }
            if score <= 0.0 {
                rejection_reasons.push("non_positive_risk_adjusted_score".to_string());
            }

            let strategy_cap = policy
                .total_capital_usd
                .mul_add(policy.max_strategy_fraction, 0.0)
                .min(input.max_allocation_usd);
            let raw_target = if score_sum > 0.0 {
                policy.total_capital_usd * (score / score_sum)
            } else {
                0.0
            };
            let target = if rejection_reasons.is_empty() {
                raw_target.clamp(policy.min_allocation_usd.min(strategy_cap), strategy_cap)
            } else {
                0.0
            };

            StrategyAllocation {
                strategy_id: input.strategy_id.clone(),
                artifact_id: input.artifact_id.clone(),
                target_allocation_usd: target,
                target_fraction: if policy.total_capital_usd > 0.0 {
                    target / policy.total_capital_usd
                } else {
                    0.0
                },
                score,
                rejection_reasons,
            }
        })
        .collect()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyReturnSeries {
    pub strategy_id: String,
    pub artifact_id: String,
    pub returns: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyCorrelation {
    pub left_strategy_id: String,
    pub right_strategy_id: String,
    pub samples: usize,
    pub correlation: f64,
    pub artifact_ids: Vec<String>,
}

pub fn compute_strategy_correlations(series: &[StrategyReturnSeries]) -> Vec<StrategyCorrelation> {
    let mut rows = Vec::new();
    for i in 0..series.len() {
        for j in (i + 1)..series.len() {
            let left = &series[i];
            let right = &series[j];
            let samples = left.returns.len().min(right.returns.len());
            rows.push(StrategyCorrelation {
                left_strategy_id: left.strategy_id.clone(),
                right_strategy_id: right.strategy_id.clone(),
                samples,
                correlation: pearson(&left.returns[..samples], &right.returns[..samples]),
                artifact_ids: vec![left.artifact_id.clone(), right.artifact_id.clone()],
            });
        }
    }
    rows
}

fn pearson(left: &[f64], right: &[f64]) -> f64 {
    if left.len() < 2 || right.len() < 2 || left.len() != right.len() {
        return 0.0;
    }
    let n = left.len() as f64;
    let left_mean = left.iter().sum::<f64>() / n;
    let right_mean = right.iter().sum::<f64>() / n;
    let mut numerator = 0.0;
    let mut left_var = 0.0;
    let mut right_var = 0.0;
    for (l, r) in left.iter().zip(right.iter()) {
        let dl = l - left_mean;
        let dr = r - right_mean;
        numerator += dl * dr;
        left_var += dl * dl;
        right_var += dr * dr;
    }
    if left_var <= f64::EPSILON || right_var <= f64::EPSILON {
        0.0
    } else {
        (numerator / (left_var.sqrt() * right_var.sqrt())).clamp(-1.0, 1.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebalancePolicy {
    pub min_drift_usd: f64,
    pub min_drift_fraction: f64,
    pub paper_only: bool,
    pub requires_human_approval: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RebalanceActionKind {
    Increase,
    Decrease,
    Hold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebalanceAction {
    pub strategy_id: String,
    pub action: RebalanceActionKind,
    pub delta_usd: f64,
    pub requires_human_approval: bool,
    pub blocked_reasons: Vec<String>,
}

pub fn plan_rebalance(
    policy: &RebalancePolicy,
    allocations: &[StrategyAllocation],
    current_allocations: &[(String, f64)],
) -> Vec<RebalanceAction> {
    allocations
        .iter()
        .map(|allocation| {
            let current = current_allocations
                .iter()
                .find(|(strategy_id, _)| strategy_id == &allocation.strategy_id)
                .map(|(_, value)| *value)
                .unwrap_or(0.0);
            let delta = allocation.target_allocation_usd - current;
            let drift_fraction = if allocation.target_allocation_usd.abs() > f64::EPSILON {
                (delta / allocation.target_allocation_usd).abs()
            } else {
                0.0
            };
            let mut blocked_reasons = Vec::new();
            if !policy.paper_only {
                blocked_reasons.push("policy_not_paper_only".to_string());
            }
            if !policy.requires_human_approval {
                blocked_reasons.push("missing_human_approval_gate".to_string());
            }
            if delta.abs() < policy.min_drift_usd || drift_fraction < policy.min_drift_fraction {
                blocked_reasons.push("drift_below_threshold".to_string());
            }

            let action = if blocked_reasons
                .iter()
                .any(|reason| reason == "drift_below_threshold")
            {
                RebalanceActionKind::Hold
            } else if delta > 0.0 {
                RebalanceActionKind::Increase
            } else if delta < 0.0 {
                RebalanceActionKind::Decrease
            } else {
                RebalanceActionKind::Hold
            };

            RebalanceAction {
                strategy_id: allocation.strategy_id.clone(),
                action,
                delta_usd: delta,
                requires_human_approval: true,
                blocked_reasons,
            }
        })
        .collect()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StrategyIntentSide {
    Long,
    Short,
    Flat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyExecutionIntent {
    pub strategy_id: String,
    pub product_id: String,
    pub side: StrategyIntentSide,
    pub notional_usd: f64,
    pub priority: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyCollision {
    pub product_id: String,
    pub strategy_ids: Vec<String>,
    pub reason: String,
    pub blocked: bool,
}

pub fn detect_strategy_collisions(intents: &[StrategyExecutionIntent]) -> Vec<StrategyCollision> {
    let mut collisions = Vec::new();
    for i in 0..intents.len() {
        for j in (i + 1)..intents.len() {
            let left = &intents[i];
            let right = &intents[j];
            if left.product_id != right.product_id {
                continue;
            }
            if left.side == StrategyIntentSide::Flat || right.side == StrategyIntentSide::Flat {
                continue;
            }
            let opposing = left.side != right.side;
            let same_priority = left.priority == right.priority;
            if opposing || same_priority {
                collisions.push(StrategyCollision {
                    product_id: left.product_id.clone(),
                    strategy_ids: vec![left.strategy_id.clone(), right.strategy_id.clone()],
                    reason: if opposing {
                        "opposing_direction_same_product".to_string()
                    } else {
                        "same_priority_parallel_intent".to_string()
                    },
                    blocked: true,
                });
            }
        }
    }
    collisions
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioReview {
    pub allocations: Vec<StrategyAllocation>,
    pub correlations: Vec<StrategyCorrelation>,
    pub rebalance_actions: Vec<RebalanceAction>,
    pub collisions: Vec<StrategyCollision>,
    pub advisory_only: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn policy() -> CapitalAllocationPolicy {
        CapitalAllocationPolicy {
            total_capital_usd: 100.0,
            min_allocation_usd: 5.0,
            max_strategy_fraction: 0.6,
            drawdown_penalty_weight: 0.5,
            requires_human_approval: true,
        }
    }

    #[test]
    fn allocation_is_bounded_by_policy_and_artifacts() {
        let rows = allocate_capital(
            &policy(),
            &[
                StrategyAllocationInput {
                    strategy_id: "s1".into(),
                    artifact_id: "a1".into(),
                    expected_return: 0.2,
                    max_drawdown: 0.1,
                    current_allocation_usd: 0.0,
                    max_allocation_usd: 80.0,
                },
                StrategyAllocationInput {
                    strategy_id: "s2".into(),
                    artifact_id: "a2".into(),
                    expected_return: 0.1,
                    max_drawdown: 0.05,
                    current_allocation_usd: 0.0,
                    max_allocation_usd: 80.0,
                },
            ],
        );
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|row| row.target_allocation_usd <= 60.0));
        assert!(rows.iter().all(|row| row.rejection_reasons.is_empty()));
    }

    #[test]
    fn allocation_rejects_missing_approval_gate() {
        let mut p = policy();
        p.requires_human_approval = false;
        let rows = allocate_capital(
            &p,
            &[StrategyAllocationInput {
                strategy_id: "s1".into(),
                artifact_id: "a1".into(),
                expected_return: 0.2,
                max_drawdown: 0.1,
                current_allocation_usd: 0.0,
                max_allocation_usd: 80.0,
            }],
        );
        assert_eq!(rows[0].target_allocation_usd, 0.0);
        assert!(rows[0]
            .rejection_reasons
            .contains(&"missing_human_approval_gate".to_string()));
    }

    #[test]
    fn correlation_artifact_is_reproducible() {
        let rows = compute_strategy_correlations(&[
            StrategyReturnSeries {
                strategy_id: "s1".into(),
                artifact_id: "a1".into(),
                returns: vec![0.01, 0.02, 0.03],
            },
            StrategyReturnSeries {
                strategy_id: "s2".into(),
                artifact_id: "a2".into(),
                returns: vec![0.02, 0.04, 0.06],
            },
        ]);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].samples, 3);
        assert!((rows[0].correlation - 1.0).abs() < 1e-9);
    }

    #[test]
    fn rebalance_policy_requires_paper_and_approval() {
        let allocation = StrategyAllocation {
            strategy_id: "s1".into(),
            artifact_id: "a1".into(),
            target_allocation_usd: 40.0,
            target_fraction: 0.4,
            score: 1.0,
            rejection_reasons: vec![],
        };
        let actions = plan_rebalance(
            &RebalancePolicy {
                min_drift_usd: 5.0,
                min_drift_fraction: 0.05,
                paper_only: false,
                requires_human_approval: false,
            },
            &[allocation],
            &[("s1".into(), 10.0)],
        );
        assert!(actions[0]
            .blocked_reasons
            .contains(&"policy_not_paper_only".to_string()));
        assert!(actions[0]
            .blocked_reasons
            .contains(&"missing_human_approval_gate".to_string()));
    }

    #[test]
    fn collision_detection_blocks_opposing_same_product_intents() {
        let collisions = detect_strategy_collisions(&[
            StrategyExecutionIntent {
                strategy_id: "trend".into(),
                product_id: "BTC-USD".into(),
                side: StrategyIntentSide::Long,
                notional_usd: 25.0,
                priority: 1,
            },
            StrategyExecutionIntent {
                strategy_id: "mean-revert".into(),
                product_id: "BTC-USD".into(),
                side: StrategyIntentSide::Short,
                notional_usd: 25.0,
                priority: 2,
            },
        ]);
        assert_eq!(collisions.len(), 1);
        assert!(collisions[0].blocked);
        assert_eq!(collisions[0].reason, "opposing_direction_same_product");
    }
}
