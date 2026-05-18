use parking_lot::RwLock;
use pt_core::TradingViewBias;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

/// A normalized signal emitted by any external data source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedExternalSignal {
    /// Human-readable source identifier, e.g. "fear_greed", "tradingview_webhook".
    pub source: String,
    /// Unix epoch milliseconds of the observation.
    pub ts_ms: i64,
    /// Directional bias in [-1.0, 1.0]: positive = bullish, negative = bearish.
    pub bias: f64,
    /// Confidence in [0.0, 1.0].
    pub confidence: f64,
    /// Optional semantic tags (e.g. ["sentiment", "crypto"]).
    pub tags: Vec<String>,
    /// Raw payload preserved for debugging / downstream enrichment.
    pub raw: Value,
}

/// Trait implemented by all external data source adaptors.
/// Implementors poll a data source (REST, file, cache) and return a batch of signals.
/// This is a synchronous interface; wrap with `tokio::task::spawn_blocking` when needed.
pub trait ExternalSignalAdapter: Send + Sync {
    fn source_id(&self) -> &str;
    fn poll(&self) -> Vec<NormalizedExternalSignal>;
}

/// Adaptor that wraps the in-memory TradingView bias state shared by the engine.
/// Each call to `poll()` snapshots the current `Option<TradingViewBias>` and returns
/// it as a `NormalizedExternalSignal` — or an empty vec if no bias has been received yet.
pub struct WebhookSignalAdapter {
    tv_bias: Arc<RwLock<Option<TradingViewBias>>>,
}

impl WebhookSignalAdapter {
    pub fn new(tv_bias: Arc<RwLock<Option<TradingViewBias>>>) -> Self {
        Self { tv_bias }
    }
}

impl ExternalSignalAdapter for WebhookSignalAdapter {
    fn source_id(&self) -> &str {
        "tradingview_webhook"
    }

    fn poll(&self) -> Vec<NormalizedExternalSignal> {
        let guard = self.tv_bias.read();
        let Some(ref bias) = *guard else {
            return vec![];
        };
        vec![NormalizedExternalSignal {
            source: "tradingview_webhook".to_string(),
            ts_ms: bias.ts.timestamp_millis(),
            bias: bias.bias,
            confidence: bias.confidence,
            tags: vec!["tradingview".to_string(), "webhook".to_string()],
            raw: serde_json::json!({
                "source": bias.source,
                "bias": bias.bias,
                "confidence": bias.confidence,
                "ts": bias.ts.to_rfc3339(),
            }),
        }]
    }
}

/// Alternative.me Crypto Fear & Greed Index adaptor.
/// Polls https://api.alternative.me/fng/?limit=1 (free, no auth).
/// Maps value ∈ [0, 100] → bias = (value - 50) / 50.
pub struct FearGreedAdapter {
    client: reqwest::blocking::Client,
}

impl FearGreedAdapter {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .user_agent("pt-signal/0.1")
                .build()
                .unwrap_or_default(),
        }
    }
}

impl Default for FearGreedAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ExternalSignalAdapter for FearGreedAdapter {
    fn source_id(&self) -> &str {
        "fear_greed"
    }

    fn poll(&self) -> Vec<NormalizedExternalSignal> {
        let url = "https://api.alternative.me/fng/?limit=1";
        let resp = match self.client.get(url).send() {
            Ok(r) => r,
            Err(_) => return vec![],
        };
        let body: Value = match resp.json() {
            Ok(v) => v,
            Err(_) => return vec![],
        };

        let entry = body
            .get("data")
            .and_then(|d| d.get(0))
            .cloned()
            .unwrap_or(Value::Null);

        let value_str = entry
            .get("value")
            .and_then(Value::as_str)
            .unwrap_or("50");
        let ts_str = entry
            .get("timestamp")
            .and_then(Value::as_str)
            .unwrap_or("0");

        let value: f64 = value_str.parse().unwrap_or(50.0);
        let ts_secs: i64 = ts_str.parse().unwrap_or(0);

        // Extreme readings (< 20 or > 80) get higher confidence.
        let distance_from_neutral = (value - 50.0).abs() / 50.0;
        let confidence = distance_from_neutral.min(1.0);
        let bias = (value - 50.0) / 50.0;

        vec![NormalizedExternalSignal {
            source: "fear_greed".to_string(),
            ts_ms: ts_secs * 1000,
            bias,
            confidence,
            tags: vec!["sentiment".to_string(), "crypto".to_string()],
            raw: entry,
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use parking_lot::RwLock;
    use std::sync::Arc;

    #[test]
    fn webhook_adapter_returns_empty_when_no_bias() {
        let state: Arc<RwLock<Option<TradingViewBias>>> = Arc::new(RwLock::new(None));
        let adapter = WebhookSignalAdapter::new(state);
        assert!(adapter.poll().is_empty());
    }

    #[test]
    fn webhook_adapter_maps_bias_to_signal() {
        let bias = TradingViewBias {
            bias: 0.7,
            confidence: 0.9,
            source: "tv-test".to_string(),
            ts: Utc::now(),
        };
        let state = Arc::new(RwLock::new(Some(bias)));
        let adapter = WebhookSignalAdapter::new(state);
        let signals = adapter.poll();
        assert_eq!(signals.len(), 1);
        assert!((signals[0].bias - 0.7).abs() < 1e-9);
        assert!((signals[0].confidence - 0.9).abs() < 1e-9);
        assert_eq!(signals[0].source, "tradingview_webhook");
    }

    fn make_signal(bias: f64, confidence: f64) -> NormalizedExternalSignal {
        NormalizedExternalSignal {
            source: "test".to_string(),
            ts_ms: 1_700_000_000_000,
            bias,
            confidence,
            tags: vec![],
            raw: Value::Null,
        }
    }

    #[test]
    fn normalized_signal_serializes_roundtrip() {
        let s = make_signal(0.5, 0.8);
        let json = serde_json::to_string(&s).unwrap();
        let back: NormalizedExternalSignal = serde_json::from_str(&json).unwrap();
        assert!((back.bias - 0.5).abs() < 1e-9);
        assert!((back.confidence - 0.8).abs() < 1e-9);
    }

    #[test]
    fn fear_greed_bias_formula() {
        // value=75 → bias=(75-50)/50=0.5, confidence=(75-50).abs()/50=0.5
        let value = 75.0_f64;
        let bias = (value - 50.0) / 50.0;
        let confidence = (value - 50.0).abs() / 50.0;
        assert!((bias - 0.5).abs() < 1e-9);
        assert!((confidence - 0.5).abs() < 1e-9);
    }

    #[test]
    fn fear_greed_extreme_confidence_capped() {
        // value=0 → confidence min(50/50, 1.0) = 1.0
        let value = 0.0_f64;
        let confidence = ((value - 50.0).abs() / 50.0).min(1.0);
        assert!((confidence - 1.0).abs() < 1e-9);
    }

    #[test]
    fn fear_greed_neutral_has_zero_bias_zero_confidence() {
        let value = 50.0_f64;
        let bias = (value - 50.0) / 50.0;
        let confidence = (value - 50.0).abs() / 50.0;
        assert!(bias.abs() < 1e-9);
        assert!(confidence.abs() < 1e-9);
    }
}
