use crate::config::LocalModelConfig;
use crate::error::AgentError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Boundary trait for local inference. Implementations may target Ollama,
/// llama.cpp HTTP server, or any compatible local endpoint.
///
/// Missing runtime: if the local model server is not running, `complete`
/// returns `AgentError::LocalModelUnavailable`. The caller must decide
/// whether to fall back to an external provider or return a non-blocking
/// advisory error.
#[async_trait]
pub trait LocalModelClient: Send + Sync {
    async fn complete(&self, prompt: &str) -> Result<String, AgentError>;
    fn is_available(&self) -> bool;
    fn model_id(&self) -> &str;
}

#[derive(Debug, Serialize)]
struct OllamaRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

/// HTTP client for Ollama-compatible local inference servers.
pub struct HttpLocalModelClient {
    config: LocalModelConfig,
    client: reqwest::Client,
    available: std::sync::atomic::AtomicBool,
}

impl HttpLocalModelClient {
    pub fn new(config: LocalModelConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .build()
            .expect("reqwest client build");
        Self {
            config,
            client,
            available: std::sync::atomic::AtomicBool::new(true),
        }
    }
}

#[async_trait]
impl LocalModelClient for HttpLocalModelClient {
    async fn complete(&self, prompt: &str) -> Result<String, AgentError> {
        let url = format!("{}/api/generate", self.config.endpoint.trim_end_matches('/'));
        let body = OllamaRequest {
            model: &self.config.model,
            prompt,
            stream: false,
        };

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                self.available
                    .store(false, std::sync::atomic::Ordering::Relaxed);
                AgentError::LocalModelUnavailable(e.to_string())
            })?;

        if !resp.status().is_success() {
            self.available
                .store(false, std::sync::atomic::Ordering::Relaxed);
            return Err(AgentError::LocalModelUnavailable(format!(
                "inference server returned {}",
                resp.status()
            )));
        }

        let parsed: OllamaResponse = resp
            .json()
            .await
            .map_err(|e| AgentError::LocalModelUnavailable(e.to_string()))?;

        self.available
            .store(true, std::sync::atomic::Ordering::Relaxed);
        Ok(parsed.response)
    }

    fn is_available(&self) -> bool {
        self.available.load(std::sync::atomic::Ordering::Relaxed)
    }

    fn model_id(&self) -> &str {
        &self.config.model
    }
}
