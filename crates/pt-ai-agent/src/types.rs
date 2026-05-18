use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// What kind of change an agent is proposing. Advisory only — no self-execution authority.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ProposalKind {
    StrategyAdjustment { parameter: String, value: Value },
    MarketSelection { market_id: String, action: String },
    RiskParameterChange { parameter: String, value: Value },
    Alert { message: String },
    /// Human-gated mode transition proposal (#96). Execution authority is never granted automatically.
    /// The operator must explicitly approve via the approval queue before any mode change takes effect.
    ModeTransition {
        from_mode: String,
        to_mode: String,
        /// Evidence items that support this proposal (e.g., "3 consecutive profitable replay runs").
        evidence: Vec<String>,
        /// Whether all gate conditions were met at proposal generation time.
        gate_conditions_met: bool,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected,
    Expired,
}

/// An AI agent's advisory proposal awaiting operator review.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProposal {
    pub id: String,
    pub kind: ProposalKind,
    pub reasoning: String,
    pub context: Value,
    pub status: ProposalStatus,
    pub model_source: String,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

impl AgentProposal {
    pub fn new(
        kind: ProposalKind,
        reasoning: impl Into<String>,
        context: Value,
        model_source: impl Into<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            kind,
            reasoning: reasoning.into(),
            context,
            status: ProposalStatus::Pending,
            model_source: model_source.into(),
            created_at: Utc::now(),
            resolved_at: None,
        }
    }

    pub fn resolve(&mut self, accepted: bool) {
        self.status = if accepted {
            ProposalStatus::Approved
        } else {
            ProposalStatus::Rejected
        };
        self.resolved_at = Some(Utc::now());
    }
}

/// Operator decision on a proposal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalResult {
    pub proposal_id: String,
    pub accepted: bool,
    pub operator_note: Option<String>,
    pub resolved_at: DateTime<Utc>,
}
