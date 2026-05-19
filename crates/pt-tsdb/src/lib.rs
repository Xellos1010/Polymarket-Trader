use duckdb::{params, Connection};
use parking_lot::Mutex;
use pt_core::{PtError, PtResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// One OHLCV candle bar stored in the time-series database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TsCandle {
    pub ts_ms: i64,
    pub product_id: String,
    pub granularity_sec: u32,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

/// One external/internal signal observation stored in the time-series database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TsSignal {
    pub ts_ms: i64,
    pub source: String,
    pub bias: f64,
    pub confidence: f64,
    pub tags: String,
}

/// High-performance local time-series store backed by DuckDB.
///
/// Candles and signals are stored in columnar tables that support fast
/// range queries and OLAP aggregations without re-scanning Parquet files.
pub struct TsDb {
    conn: Arc<Mutex<Connection>>,
}

impl TsDb {
    pub fn open(path: &str) -> PtResult<Self> {
        let conn = if path == ":memory:" {
            Connection::open_in_memory()
        } else {
            if let Some(parent) = std::path::Path::new(path).parent() {
                if !parent.as_os_str().is_empty() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| PtError::Io(e.to_string()))?;
                }
            }
            Connection::open(path)
        }
        .map_err(|e| PtError::Io(e.to_string()))?;

        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS candles (
                ts_ms           BIGINT NOT NULL,
                product_id      VARCHAR NOT NULL,
                granularity_sec INTEGER NOT NULL,
                open            DOUBLE NOT NULL,
                high            DOUBLE NOT NULL,
                low             DOUBLE NOT NULL,
                close           DOUBLE NOT NULL,
                volume          DOUBLE NOT NULL,
                PRIMARY KEY (ts_ms, product_id, granularity_sec)
            );
            CREATE TABLE IF NOT EXISTS signals (
                ts_ms       BIGINT NOT NULL,
                source      VARCHAR NOT NULL,
                bias        DOUBLE NOT NULL,
                confidence  DOUBLE NOT NULL,
                tags        VARCHAR NOT NULL DEFAULT ''
            );
            CREATE INDEX IF NOT EXISTS idx_candles_product_ts
                ON candles (product_id, granularity_sec, ts_ms);
            CREATE INDEX IF NOT EXISTS idx_signals_source_ts
                ON signals (source, ts_ms);
            ",
        )
        .map_err(|e| PtError::Io(e.to_string()))?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Insert a batch of candles. Duplicate (ts_ms, product_id, granularity_sec) rows are ignored.
    pub fn insert_candle_batch(&self, candles: &[TsCandle]) -> PtResult<()> {
        if candles.is_empty() {
            return Ok(());
        }
        let conn = self.conn.lock();
        let mut stmt = conn
            .prepare(
                "INSERT OR IGNORE INTO candles
                 (ts_ms, product_id, granularity_sec, open, high, low, close, volume)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .map_err(|e| PtError::Io(e.to_string()))?;

        for c in candles {
            stmt.execute(params![
                c.ts_ms,
                c.product_id,
                c.granularity_sec as i32,
                c.open,
                c.high,
                c.low,
                c.close,
                c.volume,
            ])
            .map_err(|e| PtError::Io(e.to_string()))?;
        }
        Ok(())
    }

    /// Query candles for a product/granularity within an inclusive time range.
    pub fn query_candles(
        &self,
        product_id: &str,
        granularity_sec: u32,
        start_ms: i64,
        end_ms: i64,
    ) -> PtResult<Vec<TsCandle>> {
        let conn = self.conn.lock();
        let mut stmt = conn
            .prepare(
                "SELECT ts_ms, product_id, granularity_sec, open, high, low, close, volume
                 FROM candles
                 WHERE product_id = ?
                   AND granularity_sec = ?
                   AND ts_ms >= ?
                   AND ts_ms <= ?
                 ORDER BY ts_ms ASC",
            )
            .map_err(|e| PtError::Io(e.to_string()))?;

        let rows = stmt
            .query_map(
                params![product_id, granularity_sec as i32, start_ms, end_ms],
                |row| {
                    Ok(TsCandle {
                        ts_ms: row.get(0)?,
                        product_id: row.get(1)?,
                        granularity_sec: row.get::<_, i32>(2)? as u32,
                        open: row.get(3)?,
                        high: row.get(4)?,
                        low: row.get(5)?,
                        close: row.get(6)?,
                        volume: row.get(7)?,
                    })
                },
            )
            .map_err(|e| PtError::Io(e.to_string()))?;

        let mut out = Vec::new();
        for row in rows {
            out.push(row.map_err(|e| PtError::Io(e.to_string()))?);
        }
        Ok(out)
    }

    /// Insert a batch of external signals.
    pub fn insert_signal_batch(&self, signals: &[TsSignal]) -> PtResult<()> {
        if signals.is_empty() {
            return Ok(());
        }
        let conn = self.conn.lock();
        let mut stmt = conn
            .prepare(
                "INSERT INTO signals (ts_ms, source, bias, confidence, tags)
                 VALUES (?, ?, ?, ?, ?)",
            )
            .map_err(|e| PtError::Io(e.to_string()))?;

        for s in signals {
            stmt.execute(params![s.ts_ms, s.source, s.bias, s.confidence, s.tags])
                .map_err(|e| PtError::Io(e.to_string()))?;
        }
        Ok(())
    }

    /// Query signals from a given source within an inclusive time range.
    pub fn query_signals(
        &self,
        source: &str,
        start_ms: i64,
        end_ms: i64,
    ) -> PtResult<Vec<TsSignal>> {
        let conn = self.conn.lock();
        let mut stmt = conn
            .prepare(
                "SELECT ts_ms, source, bias, confidence, tags
                 FROM signals
                 WHERE source = ?
                   AND ts_ms >= ?
                   AND ts_ms <= ?
                 ORDER BY ts_ms ASC",
            )
            .map_err(|e| PtError::Io(e.to_string()))?;

        let rows = stmt
            .query_map(params![source, start_ms, end_ms], |row| {
                Ok(TsSignal {
                    ts_ms: row.get(0)?,
                    source: row.get(1)?,
                    bias: row.get(2)?,
                    confidence: row.get(3)?,
                    tags: row.get(4)?,
                })
            })
            .map_err(|e| PtError::Io(e.to_string()))?;

        let mut out = Vec::new();
        for row in rows {
            out.push(row.map_err(|e| PtError::Io(e.to_string()))?);
        }
        Ok(out)
    }

    /// Count total candle rows (useful for health checks and tests).
    pub fn candle_count(&self) -> PtResult<i64> {
        let conn = self.conn.lock();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM candles", [], |row| row.get(0))
            .map_err(|e| PtError::Io(e.to_string()))?;
        Ok(count)
    }

    /// Count total signal rows.
    pub fn signal_count(&self) -> PtResult<i64> {
        let conn = self.conn.lock();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM signals", [], |row| row.get(0))
            .map_err(|e| PtError::Io(e.to_string()))?;
        Ok(count)
    }

    /// Delete rows older than `days` days from the given table.
    ///
    /// Only `"candles"` and `"signals"` are accepted; any other value returns
    /// an `Err` to prevent accidental SQL injection via the table name.
    pub fn prune_older_than_days(&self, table: &str, days: u32) -> PtResult<()> {
        if table != "candles" && table != "signals" {
            return Err(PtError::Io(format!("unknown table: {table}")));
        }
        let threshold_ms: i64 = chrono::Utc::now().timestamp_millis()
            - (days as i64) * 86_400_000;
        let sql = format!("DELETE FROM {table} WHERE ts_ms < {threshold_ms}");
        self.conn
            .lock()
            .execute_batch(&sql)
            .map_err(|e| PtError::Io(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn in_memory() -> TsDb {
        TsDb::open(":memory:").unwrap()
    }

    fn sample_candles(n: usize) -> Vec<TsCandle> {
        (0..n)
            .map(|i| TsCandle {
                ts_ms: 1_700_000_000_000 + (i as i64) * 300_000,
                product_id: "BTC-USD".to_string(),
                granularity_sec: 300,
                open: 30_000.0 + i as f64,
                high: 30_100.0,
                low: 29_900.0,
                close: 30_050.0,
                volume: 1.0,
            })
            .collect()
    }

    #[test]
    fn insert_and_count_candles() {
        let db = in_memory();
        let candles = sample_candles(5);
        db.insert_candle_batch(&candles).unwrap();
        assert_eq!(db.candle_count().unwrap(), 5);
    }

    #[test]
    fn duplicate_candles_are_ignored() {
        let db = in_memory();
        let candles = sample_candles(3);
        db.insert_candle_batch(&candles).unwrap();
        db.insert_candle_batch(&candles).unwrap();
        assert_eq!(db.candle_count().unwrap(), 3);
    }

    #[test]
    fn query_candles_returns_range() {
        let db = in_memory();
        let candles = sample_candles(10);
        db.insert_candle_batch(&candles).unwrap();
        let start = candles[2].ts_ms;
        let end = candles[5].ts_ms;
        let result = db
            .query_candles("BTC-USD", 300, start, end)
            .unwrap();
        assert_eq!(result.len(), 4); // indices 2,3,4,5
        assert_eq!(result[0].ts_ms, start);
        assert_eq!(result[3].ts_ms, end);
    }

    #[test]
    fn query_candles_filters_by_product() {
        let db = in_memory();
        let mut eth_candle = sample_candles(1);
        eth_candle[0].product_id = "ETH-USD".to_string();
        db.insert_candle_batch(&sample_candles(5)).unwrap();
        db.insert_candle_batch(&eth_candle).unwrap();
        let btc = db
            .query_candles("BTC-USD", 300, 0, i64::MAX)
            .unwrap();
        assert_eq!(btc.len(), 5);
        let eth = db
            .query_candles("ETH-USD", 300, 0, i64::MAX)
            .unwrap();
        assert_eq!(eth.len(), 1);
    }

    #[test]
    fn insert_and_query_signals() {
        let db = in_memory();
        let signals = vec![
            TsSignal {
                ts_ms: 1_000_000,
                source: "fear_greed".to_string(),
                bias: -0.3,
                confidence: 0.6,
                tags: "sentiment,crypto".to_string(),
            },
            TsSignal {
                ts_ms: 2_000_000,
                source: "fear_greed".to_string(),
                bias: 0.4,
                confidence: 0.8,
                tags: "sentiment,crypto".to_string(),
            },
        ];
        db.insert_signal_batch(&signals).unwrap();
        assert_eq!(db.signal_count().unwrap(), 2);
        let result = db.query_signals("fear_greed", 0, i64::MAX).unwrap();
        assert_eq!(result.len(), 2);
        assert!((result[0].bias - (-0.3)).abs() < 1e-9);
    }

    #[test]
    fn empty_batch_is_noop() {
        let db = in_memory();
        db.insert_candle_batch(&[]).unwrap();
        db.insert_signal_batch(&[]).unwrap();
        assert_eq!(db.candle_count().unwrap(), 0);
        assert_eq!(db.signal_count().unwrap(), 0);
    }

    #[test]
    fn prune_older_than_days_removes_old_rows() {
        let db = TsDb::open(":memory:").unwrap();
        let now_ms = chrono::Utc::now().timestamp_millis();
        let old_ms = now_ms - 100 * 24 * 3_600_000_i64; // 100 days ago
        let candles = vec![
            TsCandle { ts_ms: old_ms, product_id: "X".into(), granularity_sec: 60,
                open: 1.0, high: 1.0, low: 1.0, close: 1.0, volume: 1.0 },
            TsCandle { ts_ms: now_ms, product_id: "X".into(), granularity_sec: 60,
                open: 2.0, high: 2.0, low: 2.0, close: 2.0, volume: 2.0 },
        ];
        db.insert_candle_batch(&candles).unwrap();
        assert_eq!(db.candle_count().unwrap(), 2);
        db.prune_older_than_days("candles", 90).unwrap();
        assert_eq!(db.candle_count().unwrap(), 1);
    }
}
