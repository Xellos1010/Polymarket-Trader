use crate::types::Candle;
use pt_core::{PtError, PtResult};
use serde_json::Value;

pub async fn fetch_coinbase_candles(
    product_id: &str,
    granularity_sec: u32,
    limit: usize,
) -> PtResult<Vec<Candle>> {
    let url = format!(
        "https://api.exchange.coinbase.com/products/{}/candles?granularity={}",
        product_id, granularity_sec
    );
    let client = reqwest::Client::builder()
        .user_agent("Polymarket-Trader-StrategyLab/1.0")
        .build()
        .map_err(|e| PtError::Http(e.to_string()))?;

    let resp = client
        .get(url)
        .header("accept", "application/json")
        .send()
        .await
        .map_err(|e| PtError::Http(e.to_string()))?;
    let status = resp.status();
    let body = resp
        .text()
        .await
        .map_err(|e| PtError::Http(e.to_string()))?;

    if !status.is_success() {
        return Err(PtError::Http(format!(
            "coinbase candles fetch failed status={} body={}",
            status, body
        )));
    }

    let parsed: Value = serde_json::from_str(&body).map_err(|e| PtError::Serde(e.to_string()))?;
    let rows = parsed
        .as_array()
        .ok_or_else(|| PtError::Serde("coinbase candles response is not an array".to_string()))?;

    let mut candles = Vec::with_capacity(rows.len());
    for row in rows {
        let Some(arr) = row.as_array() else {
            continue;
        };
        if arr.len() < 6 {
            continue;
        }

        let ts_s = arr[0].as_f64().unwrap_or(0.0);
        let low = arr[1].as_f64().unwrap_or(0.0);
        let high = arr[2].as_f64().unwrap_or(0.0);
        let open = arr[3].as_f64().unwrap_or(0.0);
        let close = arr[4].as_f64().unwrap_or(0.0);
        let volume = arr[5].as_f64().unwrap_or(0.0);

        if ts_s <= 0.0 || open <= 0.0 || high <= 0.0 || low <= 0.0 || close <= 0.0 {
            continue;
        }

        candles.push(Candle {
            ts_ms: (ts_s * 1000.0) as i64,
            open,
            high,
            low,
            close,
            volume,
        });
    }

    candles.sort_by_key(|c| c.ts_ms);
    if limit > 0 && candles.len() > limit {
        let start = candles.len().saturating_sub(limit);
        candles = candles[start..].to_vec();
    }

    Ok(candles)
}
