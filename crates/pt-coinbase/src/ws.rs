use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tokio_tungstenite::{connect_async, tungstenite::Message};

const CB_WS_URL: &str = "wss://advanced-trade-ws.coinbase.com";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsTicker {
    pub product_id: String,
    pub price: f64,
    pub volume_24h: f64,
    pub ts: String,
}

#[derive(Debug, Clone)]
pub struct CoinbaseWsClient {
    sender: broadcast::Sender<WsTicker>,
}

impl CoinbaseWsClient {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(256);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<WsTicker> {
        self.sender.subscribe()
    }

    /// Connect to Coinbase WS and forward ticker events. Reconnects on disconnect.
    /// Call this in a background task; the returned handle keeps the task alive.
    pub async fn run(&self, product_ids: Vec<String>) {
        let sender = self.sender.clone();
        tokio::spawn(async move {
            loop {
                match Self::connect_once(&product_ids, &sender).await {
                    Ok(()) => tracing::info!("coinbase ws: clean close, reconnecting"),
                    Err(e) => tracing::warn!("coinbase ws: {e}, reconnecting in 5s"),
                }
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        });
    }

    async fn connect_once(
        product_ids: &[String],
        sender: &broadcast::Sender<WsTicker>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let (mut ws, _) = connect_async(CB_WS_URL).await?;

        let subscribe_msg = serde_json::json!({
            "type": "subscribe",
            "product_ids": product_ids,
            "channel": "ticker"
        });
        ws.send(Message::Text(subscribe_msg.to_string()))
            .await?;

        while let Some(msg) = ws.next().await {
            let msg = msg?;
            if let Message::Text(text) = msg {
                if let Ok(event) = serde_json::from_str::<WsEvent>(&text) {
                    for ticker in event.into_tickers() {
                        // ignore send error (no active receivers)
                        let _ = sender.send(ticker);
                    }
                }
            }
        }
        Ok(())
    }
}

impl Default for CoinbaseWsClient {
    fn default() -> Self {
        Self::new()
    }
}

// Internal deserialization types for the Coinbase Advanced Trade WS protocol.

#[derive(Debug, Deserialize)]
struct WsEvent {
    #[serde(rename = "type")]
    event_type: String,
    events: Option<Vec<WsEventPayload>>,
}

#[derive(Debug, Deserialize)]
struct WsEventPayload {
    tickers: Option<Vec<WsTickerRaw>>,
}

#[derive(Debug, Deserialize)]
struct WsTickerRaw {
    product_id: String,
    price: String,
    volume_24_h: Option<String>,
    time: Option<String>,
}

impl WsEvent {
    fn into_tickers(self) -> Vec<WsTicker> {
        if self.event_type != "update" && self.event_type != "snapshot" {
            return vec![];
        }
        self.events
            .unwrap_or_default()
            .into_iter()
            .flat_map(|e| e.tickers.unwrap_or_default())
            .filter_map(|t| {
                let price = t.price.parse::<f64>().ok().filter(|p| p.is_finite())?;
                Some(WsTicker {
                    product_id: t.product_id,
                    price,
                    volume_24h: t
                        .volume_24_h
                        .as_deref()
                        .and_then(|v| v.parse::<f64>().ok())
                        .unwrap_or(0.0),
                    ts: t.time.unwrap_or_default(),
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ws_event_parses_ticker_update() {
        let json = r#"{
            "type": "update",
            "events": [{
                "tickers": [{
                    "product_id": "BTC-USD",
                    "price": "67000.50",
                    "volume_24_h": "1234.5",
                    "time": "2024-01-01T00:00:00Z"
                }]
            }]
        }"#;
        let event: WsEvent = serde_json::from_str(json).unwrap();
        let tickers = event.into_tickers();
        assert_eq!(tickers.len(), 1);
        assert_eq!(tickers[0].product_id, "BTC-USD");
        assert!((tickers[0].price - 67000.50).abs() < 1e-6);
        assert!((tickers[0].volume_24h - 1234.5).abs() < 1e-6);
    }

    #[test]
    fn ws_event_ignores_non_update_types() {
        let json = r#"{"type": "subscriptions", "events": []}"#;
        let event: WsEvent = serde_json::from_str(json).unwrap();
        assert!(event.into_tickers().is_empty());
    }

    #[test]
    fn ws_event_skips_bad_price() {
        let json = r#"{
            "type": "update",
            "events": [{
                "tickers": [{"product_id": "X", "price": "nan", "volume_24_h": null, "time": null}]
            }]
        }"#;
        let event: WsEvent = serde_json::from_str(json).unwrap();
        assert!(event.into_tickers().is_empty());
    }

    #[test]
    fn coinbase_ws_client_subscribe_receives_broadcast() {
        let client = CoinbaseWsClient::new();
        let mut rx = client.subscribe();
        let ticker = WsTicker {
            product_id: "BTC-USD".to_string(),
            price: 50_000.0,
            volume_24h: 100.0,
            ts: "2024-01-01T00:00:00Z".to_string(),
        };
        client.sender.send(ticker.clone()).unwrap();
        let received = rx.try_recv().unwrap();
        assert_eq!(received.product_id, ticker.product_id);
        assert!((received.price - ticker.price).abs() < 1e-6);
    }
}
