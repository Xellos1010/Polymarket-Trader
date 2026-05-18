use serde::{Deserialize, Serialize};

/// Agent-wide configuration. Stored in operator config file; secrets are never repo-tracked.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default = "default_max_pending")]
    pub max_pending_proposals: usize,
    /// How long a pending proposal survives before being expired.
    #[serde(default = "default_proposal_ttl_secs")]
    pub proposal_ttl_secs: u64,
    pub local_model: Option<LocalModelConfig>,
    pub openrouter: Option<OpenRouterConfig>,
}

fn default_enabled() -> bool { false }
fn default_max_pending() -> usize { 50 }
fn default_proposal_ttl_secs() -> u64 { 3600 }

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_pending_proposals: 50,
            proposal_ttl_secs: 3600,
            local_model: None,
            openrouter: None,
        }
    }
}

/// Configuration for a local inference server (e.g., Ollama).
/// The endpoint is a URL; model is the model name accepted by that server.
/// No secrets are required for a local server by default.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalModelConfig {
    /// HTTP endpoint for the inference server (e.g., "http://localhost:11434").
    pub endpoint: String,
    /// Model identifier (e.g., "mistral", "llama3").
    pub model: String,
    #[serde(default = "default_timeout_secs")]
    pub timeout_secs: u64,
}

fn default_timeout_secs() -> u64 { 30 }

/// Configuration for the OpenRouter API. The API key must be injected at runtime
/// via environment variable (OPENROUTER_API_KEY) or operator secret store — never
/// stored in repo-tracked files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRouterConfig {
    pub model: String,
    /// Daily spend cap in USD. Requests that would exceed the cap are rejected locally.
    #[serde(default = "default_daily_spend_cap")]
    pub daily_spend_cap_usd: f64,
    #[serde(default)]
    pub routing_policy: RoutingPolicy,
}

fn default_daily_spend_cap() -> f64 { 1.0 }

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RoutingPolicy {
    /// Try local model first; fall back to OpenRouter on failure or unavailability.
    #[default]
    LocalFirst,
    OpenRouterOnly,
    LocalOnly,
}
