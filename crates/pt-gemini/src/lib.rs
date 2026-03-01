use chrono::{DateTime, Utc};
use pt_core::{PtError, PtResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiTopOfBook {
    pub product_id: String,
    pub bid: f64,
    pub ask: f64,
    pub ts: DateTime<Utc>,
}

#[derive(Clone)]
pub struct GeminiClient {
    http: reqwest::Client,
    api_base: String,
}

impl GeminiClient {
    pub fn new(api_base: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            api_base: api_base.into(),
        }
    }

    pub async fn ping(&self) -> PtResult<()> {
        let url = format!("{}/v1/symbols", self.api_base.trim_end_matches('/'));
        let resp = self
            .http
            .get(url)
            .send()
            .await
            .map_err(|e| PtError::Http(format!("gemini ping failed: {e}")))?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(PtError::Http(format!(
                "gemini ping status {}",
                resp.status()
            )))
        }
    }
}
