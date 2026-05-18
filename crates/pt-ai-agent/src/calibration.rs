use serde::{Deserialize, Serialize};

/// A single point on a ROC-style (TPR vs FPR) curve.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RocPoint {
    pub threshold: f64,
    pub true_positive_rate: f64,
    pub false_positive_rate: f64,
    pub precision: f64,
}

/// A calibration artifact: ROC curve + optimal threshold selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationArtifact {
    pub strategy_id: String,
    pub parameter: String,
    pub curve: Vec<RocPoint>,
    /// Threshold that maximizes (TPR - FPR) — Youden's J statistic.
    pub optimal_threshold: f64,
    pub auc: f64,
    pub artifact_source: String,
}

/// A labeled sample for calibration: `(score, is_positive)`.
#[derive(Debug, Clone, Copy)]
pub struct CalibrationSample {
    pub score: f64,
    pub is_positive: bool,
}

/// Compute a ROC-style curve and calibration artifact from labeled samples.
///
/// `thresholds` is the set of score values to evaluate. If empty, evenly-spaced
/// values from score min to max are generated automatically (max 100 points).
pub fn compute_calibration(
    strategy_id: impl Into<String>,
    parameter: impl Into<String>,
    samples: &[CalibrationSample],
    thresholds: &[f64],
    artifact_source: impl Into<String>,
) -> CalibrationArtifact {
    let n_pos = samples.iter().filter(|s| s.is_positive).count() as f64;
    let n_neg = samples.iter().filter(|s| !s.is_positive).count() as f64;

    let effective_thresholds: Vec<f64> = if thresholds.is_empty() {
        let min = samples
            .iter()
            .map(|s| s.score)
            .fold(f64::INFINITY, f64::min);
        let max = samples
            .iter()
            .map(|s| s.score)
            .fold(f64::NEG_INFINITY, f64::max);
        if min >= max || samples.is_empty() {
            return CalibrationArtifact {
                strategy_id: strategy_id.into(),
                parameter: parameter.into(),
                curve: vec![],
                optimal_threshold: 0.0,
                auc: 0.0,
                artifact_source: artifact_source.into(),
            };
        }
        (0..=100)
            .map(|i| min + (max - min) * i as f64 / 100.0)
            .collect()
    } else {
        thresholds.to_vec()
    };

    let mut curve: Vec<RocPoint> = effective_thresholds
        .iter()
        .map(|&t| {
            let tp = samples
                .iter()
                .filter(|s| s.is_positive && s.score >= t)
                .count() as f64;
            let fp = samples
                .iter()
                .filter(|s| !s.is_positive && s.score >= t)
                .count() as f64;
            let tn = samples
                .iter()
                .filter(|s| !s.is_positive && s.score < t)
                .count() as f64;
            let tpr = if n_pos > 0.0 { tp / n_pos } else { 0.0 };
            let fpr = if n_neg > 0.0 { fp / n_neg } else { 0.0 };
            let precision = if tp + fp > 0.0 { tp / (tp + fp) } else { 1.0 };
            let _true_negative_count = tn;
            RocPoint {
                threshold: t,
                true_positive_rate: tpr,
                false_positive_rate: fpr,
                precision,
            }
        })
        .collect();

    // Sort by FPR ascending for AUC trapezoidal integration.
    curve.sort_by(|a, b| {
        a.false_positive_rate
            .partial_cmp(&b.false_positive_rate)
            .unwrap()
    });

    let auc = auc_from_rank_probability(samples);

    // Optimal threshold: max Youden's J = TPR - FPR.
    let optimal = curve
        .iter()
        .map(|p| (p.threshold, p.true_positive_rate - p.false_positive_rate))
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|(t, _)| t)
        .unwrap_or(0.0);

    CalibrationArtifact {
        strategy_id: strategy_id.into(),
        parameter: parameter.into(),
        curve,
        optimal_threshold: optimal,
        auc: auc.clamp(0.0, 1.0),
        artifact_source: artifact_source.into(),
    }
}

fn auc_from_rank_probability(samples: &[CalibrationSample]) -> f64 {
    let positives: Vec<f64> = samples
        .iter()
        .filter(|s| s.is_positive)
        .map(|s| s.score)
        .collect();
    let negatives: Vec<f64> = samples
        .iter()
        .filter(|s| !s.is_positive)
        .map(|s| s.score)
        .collect();
    if positives.is_empty() || negatives.is_empty() {
        return 0.0;
    }

    let mut wins = 0.0;
    for positive in &positives {
        for negative in &negatives {
            wins += if positive > negative {
                1.0
            } else if (*positive - *negative).abs() < f64::EPSILON {
                0.5
            } else {
                0.0
            };
        }
    }
    wins / (positives.len() * negatives.len()) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_samples(n_pos: usize, n_neg: usize, sep: f64) -> Vec<CalibrationSample> {
        let mut s = Vec::new();
        for i in 0..n_pos {
            s.push(CalibrationSample {
                score: sep + i as f64 * 0.01,
                is_positive: true,
            });
        }
        for i in 0..n_neg {
            s.push(CalibrationSample {
                score: i as f64 * 0.01,
                is_positive: false,
            });
        }
        s
    }

    #[test]
    fn perfect_separation_auc_near_one() {
        let samples = make_samples(20, 20, 0.5);
        let artifact = compute_calibration(
            "s1",
            "rsi_threshold",
            &samples,
            &[0.1, 0.3, 0.5, 0.7, 0.9],
            "backtest-1",
        );
        assert!(artifact.auc > 0.8, "AUC={}", artifact.auc);
    }

    #[test]
    fn empty_samples_produce_empty_curve() {
        let artifact = compute_calibration("s1", "p", &[], &[], "src");
        assert!(artifact.curve.is_empty());
    }

    #[test]
    fn optimal_threshold_is_within_range() {
        let samples = make_samples(10, 10, 0.3);
        let artifact = compute_calibration("s1", "p", &samples, &[0.1, 0.3, 0.5, 0.7], "src");
        assert!(
            artifact.optimal_threshold >= 0.1 && artifact.optimal_threshold <= 0.7,
            "threshold={}",
            artifact.optimal_threshold
        );
    }

    #[test]
    fn artifact_records_source_for_review() {
        let samples = make_samples(5, 5, 0.2);
        let artifact = compute_calibration("s1", "entry_threshold", &samples, &[], "artifact-9");
        assert_eq!(artifact.strategy_id, "s1");
        assert_eq!(artifact.parameter, "entry_threshold");
        assert_eq!(artifact.artifact_source, "artifact-9");
    }
}
