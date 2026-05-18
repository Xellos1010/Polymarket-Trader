use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Staleness threshold: signals older than this are flagged as stale.
pub const DEFAULT_STALENESS_SECS: i64 = 300; // 5 minutes

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalValidation {
    pub signal_id: String,
    pub is_stale: bool,
    pub is_regime_aligned: bool,
    /// Seconds since the signal was produced. Negative means future-dated (clock skew).
    pub staleness_secs: i64,
    pub signal_bias: f64,
    pub regime_bias: f64,
    pub advisory: String,
}

impl SignalValidation {
    pub fn is_usable(&self) -> bool {
        !self.is_stale && self.is_regime_aligned
    }
}

/// Validate a single signal against staleness and regime-alignment thresholds.
///
/// `signal_ts` — when the signal was produced.
/// `signal_bias` — the signal direction (positive = bullish, negative = bearish).
/// `regime_bias` — current market regime direction from the fused bias engine.
/// `staleness_threshold_secs` — signals older than this are stale (default: 300).
/// `alignment_threshold` — minimum abs(regime_bias) to require alignment (default: 0.1).
pub fn validate_signal(
    signal_id: impl Into<String>,
    signal_ts: DateTime<Utc>,
    signal_bias: f64,
    regime_bias: f64,
    staleness_threshold_secs: i64,
    alignment_threshold: f64,
) -> SignalValidation {
    let staleness_secs = (Utc::now() - signal_ts).num_seconds();
    let is_stale = staleness_secs > staleness_threshold_secs;

    let is_regime_aligned = if regime_bias.abs() < alignment_threshold {
        true // regime is neutral — do not penalize
    } else {
        signal_bias.signum() == regime_bias.signum()
    };

    let advisory = match (is_stale, is_regime_aligned) {
        (true, true) => format!("Signal is stale ({staleness_secs}s). Regime aligned."),
        (true, false) => format!("Signal is stale ({staleness_secs}s) and regime-misaligned. Do not act."),
        (false, false) => "Signal is fresh but regime-misaligned. Consider regime context.".to_string(),
        (false, true) => "Signal is fresh and regime-aligned. Usable.".to_string(),
    };

    SignalValidation {
        signal_id: signal_id.into(),
        is_stale,
        is_regime_aligned,
        staleness_secs,
        signal_bias,
        regime_bias,
        advisory,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn fresh_ts() -> DateTime<Utc> {
        Utc::now() - Duration::seconds(60)
    }

    fn stale_ts() -> DateTime<Utc> {
        Utc::now() - Duration::seconds(600)
    }

    #[test]
    fn fresh_aligned_signal_is_usable() {
        let v = validate_signal("s1", fresh_ts(), 0.5, 0.4, 300, 0.1);
        assert!(!v.is_stale);
        assert!(v.is_regime_aligned);
        assert!(v.is_usable());
    }

    #[test]
    fn stale_signal_flagged() {
        let v = validate_signal("s2", stale_ts(), 0.5, 0.4, 300, 0.1);
        assert!(v.is_stale);
        assert!(!v.is_usable());
    }

    #[test]
    fn misaligned_signal_flagged() {
        let v = validate_signal("s3", fresh_ts(), 0.5, -0.4, 300, 0.1);
        assert!(!v.is_stale);
        assert!(!v.is_regime_aligned);
        assert!(!v.is_usable());
    }

    #[test]
    fn neutral_regime_does_not_penalize_alignment() {
        let v = validate_signal("s4", fresh_ts(), 0.5, 0.05, 300, 0.1);
        assert!(v.is_regime_aligned); // regime is neutral (< threshold)
    }
}
