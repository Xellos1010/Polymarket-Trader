use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureCategory {
    RiskGate,
    ReplayGate,
    Stability,
    Drawdown,
    Turnover,
    DataQuality,
    Budget,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureObservation {
    pub artifact_id: String,
    pub candidate_id: String,
    pub reason_code: String,
    pub reason_detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureCluster {
    pub category: FailureCategory,
    pub count: usize,
    pub candidate_ids: Vec<String>,
    pub reason_codes: Vec<String>,
    pub examples: Vec<String>,
}

pub fn classify_failure(reason_code: &str, reason_detail: &str) -> FailureCategory {
    let text = format!("{reason_code} {reason_detail}").to_ascii_lowercase();
    if text.contains("risk") || text.contains("exposure") || text.contains("loss_limit") {
        FailureCategory::RiskGate
    } else if text.contains("replay") || text.contains("acceptance") {
        FailureCategory::ReplayGate
    } else if text.contains("stability") || text.contains("walk_forward") {
        FailureCategory::Stability
    } else if text.contains("drawdown") {
        FailureCategory::Drawdown
    } else if text.contains("turnover") || text.contains("churn") {
        FailureCategory::Turnover
    } else if text.contains("data") || text.contains("missing") || text.contains("nan") {
        FailureCategory::DataQuality
    } else if text.contains("budget") || text.contains("cap") || text.contains("limit") {
        FailureCategory::Budget
    } else {
        FailureCategory::Unknown
    }
}

pub fn cluster_failures(observations: &[FailureObservation]) -> Vec<FailureCluster> {
    let mut grouped: BTreeMap<FailureCategory, FailureCluster> = BTreeMap::new();
    for obs in observations {
        let category = classify_failure(&obs.reason_code, &obs.reason_detail);
        let cluster = grouped.entry(category).or_insert_with(|| FailureCluster {
            category,
            count: 0,
            candidate_ids: Vec::new(),
            reason_codes: Vec::new(),
            examples: Vec::new(),
        });
        cluster.count += 1;
        if !cluster.candidate_ids.contains(&obs.candidate_id) {
            cluster.candidate_ids.push(obs.candidate_id.clone());
        }
        if !cluster.reason_codes.contains(&obs.reason_code) {
            cluster.reason_codes.push(obs.reason_code.clone());
        }
        if cluster.examples.len() < 3 {
            cluster.examples.push(format!(
                "{}:{}:{}",
                obs.artifact_id, obs.reason_code, obs.reason_detail
            ));
        }
    }
    grouped.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_common_optimizer_failures() {
        assert_eq!(
            classify_failure("risk_gate_failed", "daily loss limit"),
            FailureCategory::RiskGate
        );
        assert_eq!(
            classify_failure("replay_acceptance_failed", "bad fill path"),
            FailureCategory::ReplayGate
        );
        assert_eq!(
            classify_failure("candidate_rejected", "drawdown too high"),
            FailureCategory::Drawdown
        );
    }

    #[test]
    fn clusters_failures_with_bounded_examples() {
        let observations = vec![
            FailureObservation {
                artifact_id: "a1".into(),
                candidate_id: "c1".into(),
                reason_code: "risk_gate_failed".into(),
                reason_detail: "exposure cap".into(),
            },
            FailureObservation {
                artifact_id: "a2".into(),
                candidate_id: "c2".into(),
                reason_code: "risk_gate_failed".into(),
                reason_detail: "loss limit".into(),
            },
        ];
        let clusters = cluster_failures(&observations);
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].category, FailureCategory::RiskGate);
        assert_eq!(clusters[0].count, 2);
        assert_eq!(clusters[0].candidate_ids.len(), 2);
    }
}
