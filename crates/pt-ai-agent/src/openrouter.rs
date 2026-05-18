use crate::config::{OpenRouterConfig, RoutingPolicy};
use crate::error::AgentError;
use async_trait::async_trait;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

const OPENROUTER_API_URL: &str = "https://openrouter.ai/api/v1/chat/completions";

/// Boundary trait for OpenRouter. Spend cap is enforced locally before any
/// network request. API key is read from the environment at construction time
/// and never stored in any repo-tracked file.
///
/// Spend tracking is in-process and resets on restart. For production use,
/// persist the counter to the operator's SQLite store.
#[async_trait]
pub trait OpenRouterClient: Send + Sync {
    async fn complete(&self, prompt: &str) -> Result<String, AgentError>;
    fn spend_today_usd(&self) -> f64;
    fn cap_usd(&self) -> f64;
    fn model_id(&self) -> &str;
    fn routing_policy(&self) -> &RoutingPolicy;
}

#[derive(Debug, Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
}

#[derive(Debug, Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
    #[serde(default)]
    usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Debug, Deserialize)]
struct MessageContent {
    content: String,
}

#[derive(Debug, Deserialize)]
struct Usage {
    #[serde(default)]
    total_cost: f64,
}

pub struct OpenRouterHttpClient {
    config: OpenRouterConfig,
    client: reqwest::Client,
    api_key: String,
    daily_spend: Arc<Mutex<f64>>,
}

impl OpenRouterHttpClient {
    /// Constructs a client. `api_key` should be injected from the environment
    /// (e.g., `std::env::var("OPENROUTER_API_KEY")`), not from any config file.
    pub fn new(config: OpenRouterConfig, api_key: impl Into<String>) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .expect("reqwest client build");
        Self {
            config,
            client,
            api_key: api_key.into(),
            daily_spend: Arc::new(Mutex::new(0.0)),
        }
    }
}

#[async_trait]
impl OpenRouterClient for OpenRouterHttpClient {
    async fn complete(&self, prompt: &str) -> Result<String, AgentError> {
        // Spend cap enforced before any network request.
        let current_spend = *self.daily_spend.lock();
        if current_spend >= self.config.daily_spend_cap_usd {
            return Err(AgentError::SpendCapExceeded {
                cap: self.config.daily_spend_cap_usd,
                spent: current_spend,
            });
        }

        let body = ChatRequest {
            model: &self.config.model,
            messages: vec![Message {
                role: "user",
                content: prompt,
            }],
        };

        let resp = self
            .client
            .post(OPENROUTER_API_URL)
            .bearer_auth(&self.api_key)
            .header(
                "HTTP-Referer",
                "https://github.com/evanmccall/polymarket-trader",
            )
            .json(&body)
            .send()
            .await
            .map_err(|e| AgentError::ProviderError(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(AgentError::ProviderError(format!(
                "OpenRouter returned {}",
                resp.status()
            )));
        }

        let parsed: ChatResponse = resp
            .json()
            .await
            .map_err(|e| AgentError::ProviderError(e.to_string()))?;

        // Track cost if reported.
        if let Some(usage) = parsed.usage {
            let mut spend = self.daily_spend.lock();
            *spend += usage.total_cost;
        }

        parsed
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| AgentError::ProviderError("empty choices from OpenRouter".into()))
    }

    fn spend_today_usd(&self) -> f64 {
        *self.daily_spend.lock()
    }

    fn cap_usd(&self) -> f64 {
        self.config.daily_spend_cap_usd
    }

    fn model_id(&self) -> &str {
        &self.config.model
    }

    fn routing_policy(&self) -> &RoutingPolicy {
        &self.config.routing_policy
    }
}
