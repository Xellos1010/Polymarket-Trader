use chrono::{DateTime, Utc};
use pt_core::{PtError, PtResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KrakenTopOfBook {
    pub product_id: String,
    pub bid: f64,
    pub ask: f64,
    pub ts: DateTime<Utc>,
}

#[derive(Clone)]
pub struct KrakenClient {
    http: reqwest::Client,
    api_base: String,
}

impl KrakenClient {
    pub fn new(api_base: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            api_base: api_base.into(),
        }
    }

    pub async fn ping(&self) -> PtResult<()> {
        let url = format!(
            "{}/0/public/SystemStatus",
            self.api_base.trim_end_matches('/')
        );
        let resp = self
            .http
            .get(url)
            .send()
            .await
            .map_err(|e| PtError::Http(format!("kraken ping failed: {e}")))?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(PtError::Http(format!(
                "kraken ping status {}",
                resp.status()
            )))
        }
    }
}
