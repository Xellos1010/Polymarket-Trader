use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Candle {
    pub asset_id: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub ts_open_ms: i64,
    pub ts_close_ms: i64,
}

struct InProgress {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    ts_open_ms: i64,
}

pub struct CandleAggregator {
    granularity_ms: i64,
    buckets: HashMap<String, InProgress>,
}

impl CandleAggregator {
    pub fn new(granularity_secs: u32) -> Self {
        Self {
            granularity_ms: granularity_secs as i64 * 1000,
            buckets: HashMap::new(),
        }
    }

    /// Ingest one price tick. Returns a completed Candle when the current
    /// tick crosses a bucket boundary (i.e., a new granularity window begins).
    pub fn ingest(
        &mut self,
        asset_id: &str,
        price: f64,
        volume: f64,
        ts_ms: i64,
    ) -> Option<Candle> {
        let bucket_ts = (ts_ms / self.granularity_ms) * self.granularity_ms;

        if let Some(ip) = self.buckets.get(asset_id) {
            let existing_bucket = (ip.ts_open_ms / self.granularity_ms) * self.granularity_ms;
            if existing_bucket != bucket_ts {
                let completed = Candle {
                    asset_id: asset_id.to_string(),
                    open: ip.open,
                    high: ip.high,
                    low: ip.low,
                    close: ip.close,
                    volume: ip.volume,
                    ts_open_ms: ip.ts_open_ms,
                    ts_close_ms: existing_bucket + self.granularity_ms - 1,
                };
                self.buckets.insert(asset_id.to_string(), InProgress {
                    open: price, high: price, low: price, close: price,
                    volume,
                    ts_open_ms: ts_ms,
                });
                return Some(completed);
            }
        }

        let entry = self.buckets.entry(asset_id.to_string()).or_insert(InProgress {
            open: price, high: price, low: price, close: price,
            volume: 0.0,
            ts_open_ms: ts_ms,
        });
        if price > entry.high { entry.high = price; }
        if price < entry.low  { entry.low  = price; }
        entry.close = price;
        entry.volume += volume;
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_bucket_no_flush() {
        let mut agg = CandleAggregator::new(60);
        assert!(agg.ingest("X", 1.0, 10.0, 0).is_none());
        assert!(agg.ingest("X", 1.1, 10.0, 30_000).is_none());
        assert!(agg.ingest("X", 0.9, 10.0, 59_000).is_none());
    }

    #[test]
    fn boundary_crossing_emits_candle() {
        let mut agg = CandleAggregator::new(60);
        agg.ingest("X", 1.0, 10.0, 0);
        agg.ingest("X", 1.2, 10.0, 30_000);
        let candle = agg.ingest("X", 1.3, 10.0, 60_000);
        assert!(candle.is_some());
        let c = candle.unwrap();
        assert_eq!(c.asset_id, "X");
        assert!((c.open - 1.0).abs() < 1e-9);
        assert!((c.high - 1.2).abs() < 1e-9);
        assert!((c.low  - 1.0).abs() < 1e-9);
        assert!((c.close - 1.2).abs() < 1e-9);
        assert!((c.volume - 20.0).abs() < 1e-9);
    }

    #[test]
    fn ohlcv_correct_across_ticks() {
        let mut agg = CandleAggregator::new(60);
        agg.ingest("Y", 5.0, 1.0, 1000);
        agg.ingest("Y", 7.0, 2.0, 20_000);
        agg.ingest("Y", 3.0, 3.0, 45_000);
        agg.ingest("Y", 6.0, 1.0, 59_000);
        let candle = agg.ingest("Y", 6.5, 1.0, 60_000);
        let c = candle.unwrap();
        assert!((c.open - 5.0).abs() < 1e-9);
        assert!((c.high - 7.0).abs() < 1e-9);
        assert!((c.low  - 3.0).abs() < 1e-9);
        assert!((c.close - 6.0).abs() < 1e-9);
        assert!((c.volume - 7.0).abs() < 1e-9);
    }

    #[test]
    fn multiple_assets_tracked_independently() {
        let mut agg = CandleAggregator::new(60);
        agg.ingest("A", 1.0, 1.0, 0);
        agg.ingest("B", 2.0, 1.0, 0);
        let ca = agg.ingest("A", 1.5, 1.0, 60_000);
        let cb = agg.ingest("B", 2.5, 1.0, 60_000);
        assert!(ca.is_some());
        assert!(cb.is_some());
        assert_eq!(ca.unwrap().asset_id, "A");
        assert_eq!(cb.unwrap().asset_id, "B");
    }
}
