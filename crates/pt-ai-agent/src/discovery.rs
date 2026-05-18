use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryDecision {
    AcceptForReview,
    Reject,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IndicatorOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    RollingMean,
    RollingStdDev,
    ZScore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesizedIndicatorSpec {
    pub indicator_id: String,
    pub source_artifact_id: String,
    pub inputs: Vec<String>,
    pub operations: Vec<IndicatorOperation>,
    pub max_depth: usize,
    pub rejection_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryReview {
    pub decision: DiscoveryDecision,
    pub artifact_id: String,
    pub reason_codes: Vec<String>,
}

impl SynthesizedIndicatorSpec {
    pub fn review(&self) -> DiscoveryReview {
        let mut reason_codes = Vec::new();
        if self.source_artifact_id.trim().is_empty() {
            reason_codes.push("missing_source_artifact".to_string());
        }
        if self.inputs.is_empty() {
            reason_codes.push("missing_inputs".to_string());
        }
        if self.operations.is_empty() {
            reason_codes.push("missing_operations".to_string());
        }
        if self.max_depth == 0 || self.operations.len() > self.max_depth {
            reason_codes.push("depth_limit_exceeded".to_string());
        }
        if self.rejection_criteria.is_empty() {
            reason_codes.push("missing_rejection_criteria".to_string());
        }

        DiscoveryReview {
            decision: if reason_codes.is_empty() {
                DiscoveryDecision::AcceptForReview
            } else {
                DiscoveryDecision::Reject
            },
            artifact_id: self.source_artifact_id.clone(),
            reason_codes,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternDiscoverySpec {
    pub dataset_artifact_id: String,
    pub label: String,
    pub feature_names: Vec<String>,
    pub min_samples: usize,
    pub max_false_positive_rate: f64,
    pub min_precision: f64,
    pub rejection_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternDiscoveryResult {
    pub spec: PatternDiscoverySpec,
    pub samples: usize,
    pub precision: f64,
    pub false_positive_rate: f64,
    pub decision: DiscoveryDecision,
    pub rejection_reasons: Vec<String>,
}

pub fn evaluate_pattern_discovery(
    spec: PatternDiscoverySpec,
    samples: usize,
    precision: f64,
    false_positive_rate: f64,
) -> PatternDiscoveryResult {
    let mut rejection_reasons = Vec::new();
    if spec.dataset_artifact_id.trim().is_empty() {
        rejection_reasons.push("missing_dataset_artifact".to_string());
    }
    if spec.feature_names.is_empty() {
        rejection_reasons.push("missing_features".to_string());
    }
    if spec.rejection_criteria.is_empty() {
        rejection_reasons.push("missing_rejection_criteria".to_string());
    }
    if samples < spec.min_samples {
        rejection_reasons.push("insufficient_samples".to_string());
    }
    if precision < spec.min_precision {
        rejection_reasons.push("precision_below_threshold".to_string());
    }
    if false_positive_rate > spec.max_false_positive_rate {
        rejection_reasons.push("false_positive_rate_above_threshold".to_string());
    }

    PatternDiscoveryResult {
        spec,
        samples,
        precision,
        false_positive_rate,
        decision: if rejection_reasons.is_empty() {
            DiscoveryDecision::AcceptForReview
        } else {
            DiscoveryDecision::Reject
        },
        rejection_reasons,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SentimentSourceKind {
    News,
    Social,
    Funding,
    OnChain,
    OperatorNote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentSourceSpec {
    pub source_id: String,
    pub kind: SentimentSourceKind,
    pub artifact_id: String,
    pub max_age_secs: u64,
    pub min_coverage: f64,
    pub rejection_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentObservation {
    pub source_id: String,
    pub age_secs: u64,
    pub coverage: f64,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentReview {
    pub source_id: String,
    pub decision: DiscoveryDecision,
    pub normalized_score: f64,
    pub rejection_reasons: Vec<String>,
}

pub fn review_sentiment_source(
    spec: &SentimentSourceSpec,
    observation: &SentimentObservation,
) -> SentimentReview {
    let mut rejection_reasons = Vec::new();
    if spec.artifact_id.trim().is_empty() {
        rejection_reasons.push("missing_artifact".to_string());
    }
    if spec.rejection_criteria.is_empty() {
        rejection_reasons.push("missing_rejection_criteria".to_string());
    }
    if observation.source_id != spec.source_id {
        rejection_reasons.push("source_mismatch".to_string());
    }
    if observation.age_secs > spec.max_age_secs {
        rejection_reasons.push("observation_stale".to_string());
    }
    if observation.coverage < spec.min_coverage {
        rejection_reasons.push("coverage_below_threshold".to_string());
    }

    SentimentReview {
        source_id: spec.source_id.clone(),
        decision: if rejection_reasons.is_empty() {
            DiscoveryDecision::AcceptForReview
        } else {
            DiscoveryDecision::Reject
        },
        normalized_score: observation.score.clamp(-1.0, 1.0),
        rejection_reasons,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegimeClassifierSpec {
    pub classifier_id: String,
    pub dataset_artifact_id: String,
    pub labels: Vec<String>,
    pub min_holdout_accuracy: f64,
    pub min_window_stability: f64,
    pub rejection_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegimeClassifierMetrics {
    pub holdout_accuracy: f64,
    pub window_stability: f64,
    pub confusion_matrix_artifact_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegimeClassifierReview {
    pub classifier_id: String,
    pub decision: DiscoveryDecision,
    pub rejection_reasons: Vec<String>,
}

pub fn review_regime_classifier(
    spec: &RegimeClassifierSpec,
    metrics: &RegimeClassifierMetrics,
) -> RegimeClassifierReview {
    let mut rejection_reasons = Vec::new();
    if spec.dataset_artifact_id.trim().is_empty() {
        rejection_reasons.push("missing_dataset_artifact".to_string());
    }
    if spec.labels.len() < 2 {
        rejection_reasons.push("insufficient_labels".to_string());
    }
    if spec.rejection_criteria.is_empty() {
        rejection_reasons.push("missing_rejection_criteria".to_string());
    }
    if metrics.confusion_matrix_artifact_id.trim().is_empty() {
        rejection_reasons.push("missing_confusion_matrix_artifact".to_string());
    }
    if metrics.holdout_accuracy < spec.min_holdout_accuracy {
        rejection_reasons.push("accuracy_below_threshold".to_string());
    }
    if metrics.window_stability < spec.min_window_stability {
        rejection_reasons.push("stability_below_threshold".to_string());
    }

    RegimeClassifierReview {
        classifier_id: spec.classifier_id.clone(),
        decision: if rejection_reasons.is_empty() {
            DiscoveryDecision::AcceptForReview
        } else {
            DiscoveryDecision::Reject
        },
        rejection_reasons,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GeneratedCodeTarget {
    RustStrategyIr,
    PineResearch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedCodeCandidate {
    pub candidate_id: String,
    pub target: GeneratedCodeTarget,
    pub source_artifact_id: String,
    pub compile_passed: bool,
    pub backtest_artifact_id: Option<String>,
    pub replay_gate_passed: bool,
    pub requires_human_approval: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedCodeGate {
    pub candidate_id: String,
    pub decision: DiscoveryDecision,
    pub rejection_reasons: Vec<String>,
}

pub fn gate_generated_code(candidate: &GeneratedCodeCandidate) -> GeneratedCodeGate {
    let mut rejection_reasons = Vec::new();
    if candidate.source_artifact_id.trim().is_empty() {
        rejection_reasons.push("missing_source_artifact".to_string());
    }
    if !candidate.compile_passed {
        rejection_reasons.push("compile_failed".to_string());
    }
    if candidate
        .backtest_artifact_id
        .as_deref()
        .unwrap_or("")
        .trim()
        .is_empty()
    {
        rejection_reasons.push("missing_backtest_artifact".to_string());
    }
    if !candidate.replay_gate_passed {
        rejection_reasons.push("replay_gate_failed".to_string());
    }
    if !candidate.requires_human_approval {
        rejection_reasons.push("missing_human_approval_gate".to_string());
    }

    GeneratedCodeGate {
        candidate_id: candidate.candidate_id.clone(),
        decision: if rejection_reasons.is_empty() {
            DiscoveryDecision::AcceptForReview
        } else {
            DiscoveryDecision::Reject
        },
        rejection_reasons,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn indicator_spec() -> SynthesizedIndicatorSpec {
        SynthesizedIndicatorSpec {
            indicator_id: "synth-zscore-spread".into(),
            source_artifact_id: "artifact-1".into(),
            inputs: vec!["rsi".into(), "macd_hist".into()],
            operations: vec![IndicatorOperation::Subtract, IndicatorOperation::ZScore],
            max_depth: 3,
            rejection_criteria: vec!["reject if unstable across holdout windows".into()],
        }
    }

    #[test]
    fn synthesized_indicator_requires_artifact_and_rejection_criteria() {
        let mut spec = indicator_spec();
        assert_eq!(spec.review().decision, DiscoveryDecision::AcceptForReview);
        spec.rejection_criteria.clear();
        let review = spec.review();
        assert_eq!(review.decision, DiscoveryDecision::Reject);
        assert!(review
            .reason_codes
            .contains(&"missing_rejection_criteria".to_string()));
    }

    #[test]
    fn pattern_discovery_rejects_weak_or_under_sampled_results() {
        let spec = PatternDiscoverySpec {
            dataset_artifact_id: "dataset-1".into(),
            label: "next_window_positive".into(),
            feature_names: vec!["body_ratio".into(), "volume_z".into()],
            min_samples: 100,
            max_false_positive_rate: 0.2,
            min_precision: 0.65,
            rejection_criteria: vec!["reject if holdout precision is weak".into()],
        };
        let result = evaluate_pattern_discovery(spec, 30, 0.5, 0.4);
        assert_eq!(result.decision, DiscoveryDecision::Reject);
        assert!(result
            .rejection_reasons
            .contains(&"insufficient_samples".to_string()));
        assert!(result
            .rejection_reasons
            .contains(&"precision_below_threshold".to_string()));
    }

    #[test]
    fn sentiment_review_clamps_score_and_rejects_stale_inputs() {
        let spec = SentimentSourceSpec {
            source_id: "operator-news".into(),
            kind: SentimentSourceKind::OperatorNote,
            artifact_id: "sentiment-artifact-1".into(),
            max_age_secs: 300,
            min_coverage: 0.7,
            rejection_criteria: vec!["reject stale sentiment".into()],
        };
        let observation = SentimentObservation {
            source_id: "operator-news".into(),
            age_secs: 600,
            coverage: 0.8,
            score: 2.5,
        };
        let review = review_sentiment_source(&spec, &observation);
        assert_eq!(review.normalized_score, 1.0);
        assert_eq!(review.decision, DiscoveryDecision::Reject);
        assert!(review
            .rejection_reasons
            .contains(&"observation_stale".to_string()));
    }

    #[test]
    fn regime_classifier_requires_stability_and_confusion_artifact() {
        let spec = RegimeClassifierSpec {
            classifier_id: "regime-v1".into(),
            dataset_artifact_id: "regime-dataset-1".into(),
            labels: vec!["trend".into(), "range".into()],
            min_holdout_accuracy: 0.7,
            min_window_stability: 0.6,
            rejection_criteria: vec!["reject unstable regimes".into()],
        };
        let review = review_regime_classifier(
            &spec,
            &RegimeClassifierMetrics {
                holdout_accuracy: 0.75,
                window_stability: 0.3,
                confusion_matrix_artifact_id: String::new(),
            },
        );
        assert_eq!(review.decision, DiscoveryDecision::Reject);
        assert!(review
            .rejection_reasons
            .contains(&"missing_confusion_matrix_artifact".to_string()));
        assert!(review
            .rejection_reasons
            .contains(&"stability_below_threshold".to_string()));
    }

    #[test]
    fn generated_code_cannot_bypass_validation_gates() {
        let candidate = GeneratedCodeCandidate {
            candidate_id: "codegen-1".into(),
            target: GeneratedCodeTarget::RustStrategyIr,
            source_artifact_id: "prompt-artifact-1".into(),
            compile_passed: true,
            backtest_artifact_id: Some("backtest-1".into()),
            replay_gate_passed: false,
            requires_human_approval: false,
        };
        let gate = gate_generated_code(&candidate);
        assert_eq!(gate.decision, DiscoveryDecision::Reject);
        assert!(gate
            .rejection_reasons
            .contains(&"replay_gate_failed".to_string()));
        assert!(gate
            .rejection_reasons
            .contains(&"missing_human_approval_gate".to_string()));
    }

    #[test]
    fn generated_code_with_all_gates_passes_to_review_only() {
        let candidate = GeneratedCodeCandidate {
            candidate_id: "codegen-2".into(),
            target: GeneratedCodeTarget::PineResearch,
            source_artifact_id: "prompt-artifact-2".into(),
            compile_passed: true,
            backtest_artifact_id: Some("backtest-2".into()),
            replay_gate_passed: true,
            requires_human_approval: true,
        };
        assert_eq!(
            gate_generated_code(&candidate).decision,
            DiscoveryDecision::AcceptForReview
        );
    }
}
