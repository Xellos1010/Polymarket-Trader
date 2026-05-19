use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tokio_tungstenite::{connect_async, tungstenite::Message};

const POLY_WS_URL: &str = "wss://ws-subscriptions-clob.polymarket.com/ws/market";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookUpdate {
    pub asset_id: String,
    pub best_bid: Option<f64>,
    pub best_ask: Option<f64>,
    pub ts: String,
}

#[derive(Debug, Clone)]
pub struct PolymarketWsClient {
    sender: broadcast::Sender<BookUpdate>,
}

impl PolymarketWsClient {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(512);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<BookUpdate> {
        self.sender.subscribe()
    }

    pub async fn run(&self, asset_ids: Vec<String>) {
        let sender = self.sender.clone();
        tokio::spawn(async move {
            loop {
                match Self::connect_once(&asset_ids, &sender).await {
                    Ok(()) => tracing::info!("polymarket ws: clean close, reconnecting"),
                    Err(e) => tracing::warn!("polymarket ws: {e}, reconnecting in 5s"),
                }
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        });
    }

    async fn connect_once(
        asset_ids: &[String],
        sender: &broadcast::Sender<BookUpdate>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let (mut ws, _) = connect_async(POLY_WS_URL).await?;
        let sub = serde_json::json!({
            "assets_ids": asset_ids,
            "type": "subscribe"
        });
        ws.send(Message::Text(sub.to_string())).await?;
        while let Some(msg) = ws.next().await {
            let msg = msg?;
            if let Message::Text(text) = msg {
                if let Ok(events) = serde_json::from_str::<Vec<WsMarketMsg>>(&text) {
                    for event in events {
                        if let Some(update) = event.into_book_update() {
                            let _ = sender.send(update);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl Default for PolymarketWsClient {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
struct WsMarketMsg {
    event_type: Option<String>,
    asset_id: Option<String>,
    timestamp: Option<String>,
    #[serde(default)]
    bids: Vec<WsPriceLevel>,
    #[serde(default)]
    asks: Vec<WsPriceLevel>,
}

#[derive(Debug, Deserialize)]
struct WsPriceLevel {
    price: String,
    #[allow(dead_code)]
    size: String,
}

impl WsMarketMsg {
    fn into_book_update(self) -> Option<BookUpdate> {
        let event_type = self.event_type.as_deref().unwrap_or("");
        if event_type != "book" && event_type != "price_change" {
            return None;
        }
        let asset_id = self.asset_id?;
        let best_bid = self
            .bids
            .iter()
            .filter_map(|l| l.price.parse::<f64>().ok())
            .reduce(f64::max);
        let best_ask = self
            .asks
            .iter()
            .filter_map(|l| l.price.parse::<f64>().ok())
            .reduce(f64::min);
        Some(BookUpdate {
            asset_id,
            best_bid,
            best_ask,
            ts: self.timestamp.unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn book_msg_parses_best_bid_ask() {
        let json = r#"[{
            "event_type": "book",
            "asset_id": "0xabc",
            "timestamp": "2024-01-01T00:00:00Z",
            "bids": [{"price": "0.45", "size": "100"}, {"price": "0.44", "size": "50"}],
            "asks": [{"price": "0.47", "size": "80"}, {"price": "0.48", "size": "30"}]
        }]"#;
        let msgs: Vec<WsMarketMsg> = serde_json::from_str(json).unwrap();
        let update = msgs.into_iter().next().unwrap().into_book_update().unwrap();
        assert_eq!(update.asset_id, "0xabc");
        assert!((update.best_bid.unwrap() - 0.45).abs() < 1e-9);
        assert!((update.best_ask.unwrap() - 0.47).abs() < 1e-9);
    }

    #[test]
    fn non_book_event_returns_none() {
        let json = r#"[{"event_type": "subscriptions", "asset_id": "0xabc"}]"#;
        let msgs: Vec<WsMarketMsg> = serde_json::from_str(json).unwrap();
        assert!(msgs
            .into_iter()
            .next()
            .unwrap()
            .into_book_update()
            .is_none());
    }

    #[test]
    fn empty_sides_gives_none_bid_ask() {
        let json = r#"[{"event_type": "book", "asset_id": "0xdef", "bids": [], "asks": []}]"#;
        let msgs: Vec<WsMarketMsg> = serde_json::from_str(json).unwrap();
        let update = msgs.into_iter().next().unwrap().into_book_update().unwrap();
        assert!(update.best_bid.is_none());
        assert!(update.best_ask.is_none());
    }

    #[test]
    fn subscribe_broadcasts_book_update() {
        let client = PolymarketWsClient::new();
        let mut rx = client.subscribe();
        let update = BookUpdate {
            asset_id: "0xabc".to_string(),
            best_bid: Some(0.45),
            best_ask: Some(0.47),
            ts: "2024-01-01T00:00:00Z".to_string(),
        };
        client.sender.send(update.clone()).unwrap();
        let received = rx.try_recv().unwrap();
        assert_eq!(received.asset_id, "0xabc");
    }
}
