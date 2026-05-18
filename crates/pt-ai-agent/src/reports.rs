use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Structured morning brief for operator review.
/// Generated from local runtime inputs — no external calls required.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorningBrief {
    pub generated_at: DateTime<Utc>,
    pub session_date: String,
    pub positions_active: usize,
    pub pending_proposals: usize,
    pub regime_summary: String,
    pub top_signals: Vec<String>,
    pub alerts: Vec<String>,
}

impl MorningBrief {
    pub fn generate(
        positions_active: usize,
        pending_proposals: usize,
        regime_summary: impl Into<String>,
        top_signals: Vec<String>,
        alerts: Vec<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            generated_at: now,
            session_date: now.format("%Y-%m-%d").to_string(),
            positions_active,
            pending_proposals,
            regime_summary: regime_summary.into(),
            top_signals,
            alerts,
        }
    }

    pub fn render_text(&self) -> String {
        let alerts = if self.alerts.is_empty() {
            "None".to_string()
        } else {
            self.alerts.join("; ")
        };
        let signals = if self.top_signals.is_empty() {
            "None".to_string()
        } else {
            self.top_signals.join(", ")
        };
        format!(
            "=== Morning Brief ({}) ===\nPositions: {}  Pending proposals: {}\nRegime: {}\nTop signals: {}\nAlerts: {}",
            self.session_date,
            self.positions_active,
            self.pending_proposals,
            self.regime_summary,
            signals,
            alerts,
        )
    }
}

/// Structured end-of-day report for operator review.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndOfDayReport {
    pub generated_at: DateTime<Utc>,
    pub session_date: String,
    pub trades_executed: usize,
    pub net_pnl_usd: f64,
    pub proposals_reviewed: usize,
    pub proposals_accepted: usize,
    pub signal_quality_summary: String,
    pub notes: Vec<String>,
}

impl EndOfDayReport {
    pub fn generate(
        trades_executed: usize,
        net_pnl_usd: f64,
        proposals_reviewed: usize,
        proposals_accepted: usize,
        signal_quality_summary: impl Into<String>,
        notes: Vec<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            generated_at: now,
            session_date: now.format("%Y-%m-%d").to_string(),
            trades_executed,
            net_pnl_usd,
            proposals_reviewed,
            proposals_accepted,
            signal_quality_summary: signal_quality_summary.into(),
            notes,
        }
    }

    pub fn render_text(&self) -> String {
        let notes = if self.notes.is_empty() {
            "None".to_string()
        } else {
            self.notes.join("; ")
        };
        format!(
            "=== EOD Report ({}) ===\nTrades: {}  Net PnL: {:.4} USD\nProposals reviewed: {} | accepted: {}\nSignal quality: {}\nNotes: {}",
            self.session_date,
            self.trades_executed,
            self.net_pnl_usd,
            self.proposals_reviewed,
            self.proposals_accepted,
            self.signal_quality_summary,
            notes,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn morning_brief_render_includes_key_fields() {
        let brief = MorningBrief::generate(
            3,
            1,
            "BTC bullish",
            vec!["BTC-USD +0.8".into()],
            vec!["Regime shift detected".into()],
        );
        let text = brief.render_text();
        assert!(text.contains("Positions: 3"));
        assert!(text.contains("BTC bullish"));
        assert!(text.contains("Regime shift detected"));
    }

    #[test]
    fn eod_report_render_includes_pnl() {
        let report = EndOfDayReport::generate(5, 12.34, 3, 2, "Good signal alignment", vec![]);
        let text = report.render_text();
        assert!(text.contains("Trades: 5"));
        assert!(text.contains("12.3400 USD"));
    }

    #[test]
    fn morning_brief_no_alerts() {
        let brief = MorningBrief::generate(0, 0, "Neutral", vec![], vec![]);
        assert!(brief.render_text().contains("Alerts: None"));
    }
}
