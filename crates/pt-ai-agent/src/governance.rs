use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationBudgetPolicy {
    pub max_candidates: usize,
    pub max_wall_time_secs: u64,
    pub max_model_spend_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationBudgetUsage {
    pub candidates_evaluated: usize,
    pub wall_time_secs: u64,
    pub model_spend_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetDecision {
    pub allowed: bool,
    pub remaining_candidates: usize,
    pub remaining_wall_time_secs: u64,
    pub remaining_model_spend_usd: f64,
    pub rejection_reasons: Vec<String>,
}

impl EvaluationBudgetPolicy {
    pub fn evaluate(&self, usage: &EvaluationBudgetUsage) -> BudgetDecision {
        let mut rejection_reasons = Vec::new();
        if usage.candidates_evaluated > self.max_candidates {
            rejection_reasons.push(format!(
                "candidate budget exceeded: {} > {}",
                usage.candidates_evaluated, self.max_candidates
            ));
        }
        if usage.wall_time_secs > self.max_wall_time_secs {
            rejection_reasons.push(format!(
                "wall time budget exceeded: {}s > {}s",
                usage.wall_time_secs, self.max_wall_time_secs
            ));
        }
        if usage.model_spend_usd > self.max_model_spend_usd {
            rejection_reasons.push(format!(
                "model spend budget exceeded: {:.4} > {:.4}",
                usage.model_spend_usd, self.max_model_spend_usd
            ));
        }

        BudgetDecision {
            allowed: rejection_reasons.is_empty(),
            remaining_candidates: self
                .max_candidates
                .saturating_sub(usage.candidates_evaluated),
            remaining_wall_time_secs: self.max_wall_time_secs.saturating_sub(usage.wall_time_secs),
            remaining_model_spend_usd: (self.max_model_spend_usd - usage.model_spend_usd).max(0.0),
            rejection_reasons,
        }
    }
}

impl Default for EvaluationBudgetPolicy {
    fn default() -> Self {
        Self {
            max_candidates: 100,
            max_wall_time_secs: 900,
            max_model_spend_usd: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn budget_allows_usage_inside_caps() {
        let decision = EvaluationBudgetPolicy::default().evaluate(&EvaluationBudgetUsage {
            candidates_evaluated: 10,
            wall_time_secs: 60,
            model_spend_usd: 0.05,
        });
        assert!(decision.allowed);
        assert_eq!(decision.remaining_candidates, 90);
    }

    #[test]
    fn budget_rejects_hidden_widening() {
        let decision = EvaluationBudgetPolicy {
            max_candidates: 2,
            max_wall_time_secs: 30,
            max_model_spend_usd: 0.01,
        }
        .evaluate(&EvaluationBudgetUsage {
            candidates_evaluated: 3,
            wall_time_secs: 31,
            model_spend_usd: 0.02,
        });
        assert!(!decision.allowed);
        assert_eq!(decision.rejection_reasons.len(), 3);
    }
}
