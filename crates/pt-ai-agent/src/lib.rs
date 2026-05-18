pub mod attribution;
pub mod calibration;
pub mod config;
pub mod discovery;
pub mod error;
pub mod failure;
pub mod governance;
pub mod local_client;
pub mod monitoring;
pub mod openrouter;
pub mod reports;
pub mod types;
pub mod validation;
pub mod walk_forward;

pub use attribution::{
    generate_adjustments, AdjustmentBounds, ParameterAdjustmentProposal, ParameterAttribution,
};
pub use calibration::{compute_calibration, CalibrationArtifact, CalibrationSample, RocPoint};
pub use config::{AgentConfig, LocalModelConfig, OpenRouterConfig, RoutingPolicy};
pub use discovery::{
    evaluate_pattern_discovery, gate_generated_code, review_regime_classifier,
    review_sentiment_source, DiscoveryDecision, DiscoveryReview, GeneratedCodeCandidate,
    GeneratedCodeGate, GeneratedCodeTarget, IndicatorOperation, PatternDiscoveryResult,
    PatternDiscoverySpec, RegimeClassifierMetrics, RegimeClassifierReview, RegimeClassifierSpec,
    SentimentObservation, SentimentReview, SentimentSourceKind, SentimentSourceSpec,
    SynthesizedIndicatorSpec,
};
pub use error::AgentError;
pub use failure::{
    classify_failure, cluster_failures, FailureCategory, FailureCluster, FailureObservation,
};
pub use governance::{BudgetDecision, EvaluationBudgetPolicy, EvaluationBudgetUsage};
pub use local_client::{HttpLocalModelClient, LocalModelClient};
pub use monitoring::{
    summarize_positions, MonitoringConfig, MonitoringSummary, PositionInput, PositionSummary,
};
pub use openrouter::{OpenRouterClient, OpenRouterHttpClient};
pub use reports::{EndOfDayReport, MorningBrief};
pub use types::{AgentProposal, ProposalKind, ProposalResult, ProposalStatus};
pub use validation::{validate_signal, SignalValidation, DEFAULT_STALENESS_SECS};
pub use walk_forward::{build_walk_forward_plan, WalkForwardPlan, WalkForwardWindow};

use parking_lot::RwLock;
use std::sync::Arc;
use tracing::warn;

/// In-memory proposal store. For persistence, see the `pt-dashboard`
/// `AgentProposalStore` which mirrors these records into SQLite.
#[derive(Default, Clone)]
pub struct ProposalQueue {
    inner: Arc<RwLock<Vec<AgentProposal>>>,
}

impl ProposalQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&self, proposal: AgentProposal, max_pending: usize) -> Result<(), AgentError> {
        let mut queue = self.inner.write();
        let pending = queue
            .iter()
            .filter(|p| p.status == ProposalStatus::Pending)
            .count();
        if pending >= max_pending {
            return Err(AgentError::MaxPendingReached(max_pending));
        }
        queue.push(proposal);
        Ok(())
    }

    pub fn list(&self) -> Vec<AgentProposal> {
        self.inner.read().clone()
    }

    pub fn resolve(&self, id: &str, accepted: bool) -> Result<AgentProposal, AgentError> {
        let mut queue = self.inner.write();
        let proposal = queue
            .iter_mut()
            .find(|p| p.id == id)
            .ok_or_else(|| AgentError::ProposalNotFound(id.to_owned()))?;
        if proposal.status != ProposalStatus::Pending {
            warn!(id, ?proposal.status, "resolve called on non-pending proposal");
        }
        proposal.resolve(accepted);
        Ok(proposal.clone())
    }

    pub fn expire_stale(&self, ttl_secs: u64) {
        let cutoff = chrono::Utc::now() - chrono::Duration::seconds(ttl_secs as i64);
        let mut queue = self.inner.write();
        for p in queue.iter_mut() {
            if p.status == ProposalStatus::Pending && p.created_at < cutoff {
                p.status = ProposalStatus::Expired;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample_proposal(tag: &str) -> AgentProposal {
        AgentProposal::new(
            ProposalKind::Alert {
                message: format!("test alert {tag}"),
            },
            "test reasoning",
            json!({"tag": tag}),
            "local:mistral",
        )
    }

    #[test]
    fn proposal_starts_pending() {
        let p = sample_proposal("a");
        assert_eq!(p.status, ProposalStatus::Pending);
        assert!(p.resolved_at.is_none());
    }

    #[test]
    fn proposal_resolve_approve() {
        let mut p = sample_proposal("b");
        p.resolve(true);
        assert_eq!(p.status, ProposalStatus::Approved);
        assert!(p.resolved_at.is_some());
    }

    #[test]
    fn proposal_resolve_reject() {
        let mut p = sample_proposal("c");
        p.resolve(false);
        assert_eq!(p.status, ProposalStatus::Rejected);
    }

    #[test]
    fn queue_push_and_list() {
        let queue = ProposalQueue::new();
        queue.push(sample_proposal("d"), 10).unwrap();
        queue.push(sample_proposal("e"), 10).unwrap();
        assert_eq!(queue.list().len(), 2);
    }

    #[test]
    fn queue_max_pending_enforced() {
        let queue = ProposalQueue::new();
        queue.push(sample_proposal("f"), 1).unwrap();
        let err = queue.push(sample_proposal("g"), 1).unwrap_err();
        assert!(matches!(err, AgentError::MaxPendingReached(1)));
    }

    #[test]
    fn queue_resolve_found() {
        let queue = ProposalQueue::new();
        let p = sample_proposal("h");
        let id = p.id.clone();
        queue.push(p, 10).unwrap();
        let resolved = queue.resolve(&id, true).unwrap();
        assert_eq!(resolved.status, ProposalStatus::Approved);
    }

    #[test]
    fn queue_resolve_not_found() {
        let queue = ProposalQueue::new();
        let err = queue.resolve("nonexistent-id", true).unwrap_err();
        assert!(matches!(err, AgentError::ProposalNotFound(_)));
    }

    #[test]
    fn queue_expire_stale() {
        let queue = ProposalQueue::new();
        // Push a proposal and manually set its created_at to the past.
        let mut p = sample_proposal("i");
        p.created_at = chrono::Utc::now() - chrono::Duration::seconds(400);
        queue.inner.write().push(p);
        queue.expire_stale(300);
        let list = queue.list();
        assert_eq!(list[0].status, ProposalStatus::Expired);
    }

    #[test]
    fn config_defaults_are_safe() {
        let cfg = AgentConfig::default();
        assert!(!cfg.enabled);
        assert_eq!(cfg.max_pending_proposals, 50);
        assert_eq!(cfg.proposal_ttl_secs, 3600);
        assert!(cfg.local_model.is_none());
        assert!(cfg.openrouter.is_none());
    }

    #[test]
    fn mode_transition_proposal_routes_through_queue() {
        let queue = ProposalQueue::new();
        let proposal = AgentProposal::new(
            ProposalKind::ModeTransition {
                from_mode: "paper".into(),
                to_mode: "sandbox".into(),
                evidence: vec![
                    "3 consecutive paper sessions with positive PnL".into(),
                    "All replay acceptance tests pass".into(),
                ],
                gate_conditions_met: true,
            },
            "Paper results sufficient to advance to sandbox evaluation",
            json!({"session_count": 3}),
            "local:mistral",
        );
        let id = proposal.id.clone();
        queue.push(proposal, 50).unwrap();
        // Must remain Pending until human resolves.
        let pending = queue.list();
        assert_eq!(pending[0].status, ProposalStatus::Pending);
        // Human approves.
        let resolved = queue.resolve(&id, true).unwrap();
        assert_eq!(resolved.status, ProposalStatus::Approved);
    }

    #[test]
    fn openrouter_config_routing_policy_default() {
        let cfg = OpenRouterConfig {
            model: "anthropic/claude-haiku-4-5".into(),
            daily_spend_cap_usd: 1.0,
            routing_policy: RoutingPolicy::default(),
        };
        assert_eq!(cfg.routing_policy, RoutingPolicy::LocalFirst);
    }
}
