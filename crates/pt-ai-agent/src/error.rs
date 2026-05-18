use thiserror::Error;

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("local model unavailable: {0}")]
    LocalModelUnavailable(String),
    #[error("provider error: {0}")]
    ProviderError(String),
    #[error("spend cap exceeded (cap={cap:.4} USD, spent={spent:.4} USD)")]
    SpendCapExceeded { cap: f64, spent: f64 },
    #[error("proposal not found: {0}")]
    ProposalNotFound(String),
    #[error("max pending proposals reached ({0})")]
    MaxPendingReached(usize),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
