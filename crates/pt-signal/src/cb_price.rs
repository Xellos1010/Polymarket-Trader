use chrono::Utc;
use parking_lot::Mutex;
use pt_coinbase::WsTicker;
use serde_json::json;
use tokio::sync::broadcast;

use crate::{ExternalSignalAdapter, NormalizedExternalSignal};

/// Bridges CoinbaseWsClient into the ExternalSignalAdapter pipeline.
/// Emits a momentum bias: +1 if price rose vs last seen, -1 if fell, 0 if flat.
/// Confidence is fixed at 0.8.
pub struct CoinbasePriceAdapter {
    rx: Mutex<broadcast::Receiver<WsTicker>>,
    last_prices: Mutex<std::collections::HashMap<String, f64>>,
}

impl CoinbasePriceAdapter {
    pub fn new(rx: broadcast::Receiver<WsTicker>) -> Self {
        Self {
            rx: Mutex::new(rx),
            last_prices: Mutex::new(std::collections::HashMap::new()),
        }
    }
}

impl ExternalSignalAdapter for CoinbasePriceAdapter {
    fn source_id(&self) -> &str {
        "coinbase_price"
    }

    fn poll(&self) -> Vec<NormalizedExternalSignal> {
        let mut signals = Vec::new();
        let now_ms = Utc::now().timestamp_millis();
        let mut rx = self.rx.lock();
        let mut prices = self.last_prices.lock();

        loop {
            match rx.try_recv() {
                Ok(ticker) => {
                    let bias = if let Some(&last) = prices.get(&ticker.product_id) {
                        if ticker.price > last { 1.0 }
                        else if ticker.price < last { -1.0 }
                        else { 0.0 }
                    } else {
                        0.0
                    };
                    prices.insert(ticker.product_id.clone(), ticker.price);
                    signals.push(NormalizedExternalSignal {
                        source: "coinbase_price".to_string(),
                        ts_ms: now_ms,
                        bias,
                        confidence: 0.8,
                        tags: vec!["price".to_string(), "coinbase".to_string()],
                        raw: json!({
                            "product_id": ticker.product_id,
                            "price": ticker.price,
                            "volume_24h": ticker.volume_24h,
                            "ts": ticker.ts,
                        }),
                    });
                }
                Err(broadcast::error::TryRecvError::Empty) => break,
                Err(broadcast::error::TryRecvError::Lagged(n)) => {
                    tracing::warn!("CoinbasePriceAdapter lagged by {n} messages");
                    break;
                }
                Err(broadcast::error::TryRecvError::Closed) => break,
            }
        }
        signals
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pt_coinbase::CoinbaseWsClient;

    fn make_ticker(product_id: &str, price: f64) -> WsTicker {
        WsTicker {
            product_id: product_id.to_string(),
            price,
            volume_24h: 100.0,
            ts: "2024-01-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn poll_empty_returns_no_signals() {
        let client = CoinbaseWsClient::new();
        let rx = client.subscribe();
        let adapter = CoinbasePriceAdapter::new(rx);
        assert!(adapter.poll().is_empty());
    }

    #[test]
    fn poll_returns_signal_per_tick() {
        let client = CoinbaseWsClient::new();
        let rx = client.subscribe();
        let adapter = CoinbasePriceAdapter::new(rx);
        client.sender.send(make_ticker("BTC-USD", 50_000.0)).unwrap();
        client.sender.send(make_ticker("ETH-USD", 3_000.0)).unwrap();
        let signals = adapter.poll();
        assert_eq!(signals.len(), 2);
        assert!(signals.iter().all(|s| s.source == "coinbase_price"));
    }

    #[test]
    fn bias_positive_on_price_rise() {
        let client = CoinbaseWsClient::new();
        let rx = client.subscribe();
        let adapter = CoinbasePriceAdapter::new(rx);
        client.sender.send(make_ticker("BTC-USD", 50_000.0)).unwrap();
        adapter.poll(); // consume first tick — bias=0, sets last_price
        client.sender.send(make_ticker("BTC-USD", 51_000.0)).unwrap();
        let signals = adapter.poll();
        assert_eq!(signals.len(), 1);
        assert!((signals[0].bias - 1.0).abs() < 1e-9);
    }

    #[test]
    fn bias_negative_on_price_fall() {
        let client = CoinbaseWsClient::new();
        let rx = client.subscribe();
        let adapter = CoinbasePriceAdapter::new(rx);
        client.sender.send(make_ticker("BTC-USD", 50_000.0)).unwrap();
        adapter.poll();
        client.sender.send(make_ticker("BTC-USD", 49_000.0)).unwrap();
        let signals = adapter.poll();
        assert!((signals[0].bias - (-1.0)).abs() < 1e-9);
    }
}
