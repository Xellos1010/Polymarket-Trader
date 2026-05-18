use crate::types::Candle;
use pt_core::{PtError, PtResult};
use serde_json::Value;

/// Maximum candles the Coinbase Exchange REST API returns in a single call.
const CB_CANDLE_PAGE_SIZE: usize = 300;

pub async fn fetch_coinbase_candles(
    product_id: &str,
    granularity_sec: u32,
    limit: usize,
) -> PtResult<Vec<Candle>> {
    fetch_coinbase_candles_range(product_id, granularity_sec, None, None, limit).await
}

/// Fetch candles for an optional [start_secs, end_secs) window, paginating if
/// the window spans more than CB_CANDLE_PAGE_SIZE bars.  Pass `max_bars = 0`
/// for no server-side cap (still capped by `max_bars_hard` below).
pub async fn fetch_coinbase_candles_range(
    product_id: &str,
    granularity_sec: u32,
    start_secs: Option<i64>,
    end_secs: Option<i64>,
    max_bars: usize,
) -> PtResult<Vec<Candle>> {
    const MAX_BARS_HARD: usize = 2_000;
    let cap = if max_bars == 0 {
        MAX_BARS_HARD
    } else {
        max_bars.min(MAX_BARS_HARD)
    };

    let client = reqwest::Client::builder()
        .user_agent("Polymarket-Trader-StrategyLab/1.0")
        .build()
        .map_err(|e| PtError::Http(e.to_string()))?;

    let now = chrono::Utc::now().timestamp();
    let gran = granularity_sec as i64;
    let window_end = end_secs.unwrap_or(now);
    let window_start = start_secs.unwrap_or_else(|| {
        window_end - gran * (CB_CANDLE_PAGE_SIZE as i64)
    });

    let mut all: Vec<Candle> = Vec::new();
    let mut page_end = window_end;

    loop {
        let page_start = (page_end - gran * (CB_CANDLE_PAGE_SIZE as i64)).max(window_start);
        let url = format!(
            "https://api.exchange.coinbase.com/products/{}/candles?granularity={}&start={}&end={}",
            product_id, granularity_sec, page_start, page_end
        );

        let resp = client
            .get(&url)
            .header("accept", "application/json")
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        let status = resp.status();
        let body = resp.text().await.map_err(|e| PtError::Http(e.to_string()))?;

        if !status.is_success() {
            return Err(PtError::Http(format!(
                "coinbase candles fetch failed status={} body={}",
                status, body
            )));
        }

        let parsed: Value =
            serde_json::from_str(&body).map_err(|e| PtError::Serde(e.to_string()))?;
        let rows = parsed.as_array().ok_or_else(|| {
            PtError::Serde("coinbase candles response is not an array".to_string())
        })?;

        let mut page_candles: Vec<Candle> = Vec::with_capacity(rows.len());
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
            page_candles.push(Candle {
                ts_ms: (ts_s * 1000.0) as i64,
                open,
                high,
                low,
                close,
                volume,
            });
        }

        all.extend(page_candles);

        // Stop paginating if we've covered the requested start or hit the cap.
        if page_start <= window_start || all.len() >= cap {
            break;
        }
        page_end = page_start;
    }

    all.sort_by_key(|c| c.ts_ms);
    all.dedup_by_key(|c| c.ts_ms);
    if cap > 0 && all.len() > cap {
        let trim = all.len() - cap;
        all.drain(..trim);
    }

    Ok(all)
}
