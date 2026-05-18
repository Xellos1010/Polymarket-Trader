use serde::{Deserialize, Serialize};

/// Advisory summary for a single position.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionSummary {
    pub market_id: String,
    pub position_usd: f64,
    pub pnl_usd: f64,
    pub age_secs: u64,
    pub is_anomalous: bool,
    pub anomaly_reason: Option<String>,
}

/// Advisory summary across all monitored positions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringSummary {
    pub positions: Vec<PositionSummary>,
    pub total_exposure_usd: f64,
    pub total_pnl_usd: f64,
    pub anomalous_count: usize,
    pub notes: Vec<String>,
}

/// Position input for monitoring analysis.
#[derive(Debug, Clone)]
pub struct PositionInput {
    pub market_id: String,
    pub position_usd: f64,
    pub pnl_usd: f64,
    pub age_secs: u64,
}

pub struct MonitoringConfig {
    /// PnL loss below this triggers anomaly flag (negative = loss).
    pub pnl_loss_threshold_usd: f64,
    /// Position age above this triggers stale-position anomaly.
    pub stale_position_age_secs: u64,
    /// Notional above this triggers oversize anomaly.
    pub max_notional_usd: f64,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            pnl_loss_threshold_usd: -25.0,
            stale_position_age_secs: 86400,
            max_notional_usd: 100.0,
        }
    }
}

/// Generate a monitoring summary from current position inputs.
pub fn summarize_positions(inputs: &[PositionInput], cfg: &MonitoringConfig) -> MonitoringSummary {
    let mut positions = Vec::with_capacity(inputs.len());
    let mut total_exposure = 0.0f64;
    let mut total_pnl = 0.0f64;
    let mut anomalous_count = 0usize;
    let mut notes = Vec::new();

    for p in inputs {
        let mut anomaly_reasons = Vec::new();
        if p.pnl_usd < cfg.pnl_loss_threshold_usd {
            anomaly_reasons.push(format!(
                "PnL {:.2} USD below loss threshold {:.2}",
                p.pnl_usd, cfg.pnl_loss_threshold_usd
            ));
        }
        if p.age_secs > cfg.stale_position_age_secs {
            anomaly_reasons.push(format!(
                "Position age {}s exceeds stale threshold {}s",
                p.age_secs, cfg.stale_position_age_secs
            ));
        }
        if p.position_usd.abs() > cfg.max_notional_usd {
            anomaly_reasons.push(format!(
                "Notional {:.2} USD exceeds max {:.2}",
                p.position_usd.abs(),
                cfg.max_notional_usd
            ));
        }

        let is_anomalous = !anomaly_reasons.is_empty();
        if is_anomalous {
            anomalous_count += 1;
        }

        total_exposure += p.position_usd.abs();
        total_pnl += p.pnl_usd;

        positions.push(PositionSummary {
            market_id: p.market_id.clone(),
            position_usd: p.position_usd,
            pnl_usd: p.pnl_usd,
            age_secs: p.age_secs,
            is_anomalous,
            anomaly_reason: if anomaly_reasons.is_empty() {
                None
            } else {
                Some(anomaly_reasons.join("; "))
            },
        });
    }

    if anomalous_count > 0 {
        notes.push(format!("{anomalous_count} position(s) require operator attention."));
    }
    if inputs.is_empty() {
        notes.push("No active positions.".to_string());
    }

    MonitoringSummary {
        positions,
        total_exposure_usd: total_exposure,
        total_pnl_usd: total_pnl,
        anomalous_count,
        notes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normal() -> PositionInput {
        PositionInput {
            market_id: "BTC-USD".into(),
            position_usd: 10.0,
            pnl_usd: 0.5,
            age_secs: 100,
        }
    }

    #[test]
    fn normal_position_not_anomalous() {
        let s = summarize_positions(&[normal()], &MonitoringConfig::default());
        assert_eq!(s.anomalous_count, 0);
        assert!(!s.positions[0].is_anomalous);
    }

    #[test]
    fn loss_threshold_triggers_anomaly() {
        let mut p = normal();
        p.pnl_usd = -50.0;
        let s = summarize_positions(&[p], &MonitoringConfig::default());
        assert_eq!(s.anomalous_count, 1);
        assert!(s.positions[0].anomaly_reason.as_deref().unwrap().contains("loss threshold"));
    }

    #[test]
    fn stale_position_triggers_anomaly() {
        let mut p = normal();
        p.age_secs = 90_000;
        let s = summarize_positions(&[p], &MonitoringConfig::default());
        assert_eq!(s.anomalous_count, 1);
    }

    #[test]
    fn oversize_triggers_anomaly() {
        let mut p = normal();
        p.position_usd = 200.0;
        let s = summarize_positions(&[p], &MonitoringConfig::default());
        assert_eq!(s.anomalous_count, 1);
    }

    #[test]
    fn empty_inputs_produce_no_active_positions_note() {
        let s = summarize_positions(&[], &MonitoringConfig::default());
        assert!(s.notes.iter().any(|n| n.contains("No active positions")));
    }
}
