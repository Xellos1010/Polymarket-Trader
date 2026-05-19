# Comprehensive Engine Enhancement Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement Groups A–F from the 2026-05-18 design spec: live Polymarket/Coinbase WS data pipeline, infrastructure hardening, operator workflow, smart order placement advisor, tick-aggregated live chart candles, and operator strategy promotion workspace.

**Architecture:** Six independent groups executed in parallel batches. All new Rust features are advisory-only or additive — no live-mode paths, no credential changes. Frontend additions hook into the existing Axum/SSE dashboard pattern.

**Tech Stack:** Rust (Axum, tokio-tungstenite, DashMap, DuckDB), TypeScript/Vite frontend, shell (bash/systemd), Polymarket CLOB WS, Coinbase Advanced Trade WS.

**Spec:** `docs/superpowers/specs/2026-05-18-comprehensive-engine-enhancement-design.md`

---

## Parallel Execution Batches

- **Batch 1 (all independent, start together):** Tasks 1–3 (workspace deps, dashmap, rate-limit)
- **Batch 2 (after Batch 1):** Tasks 4–9 (tsdb retention, Polymarket WS, CB adapter, session vwap, pi scripts)
- **Batch 3 (independent of Batch 2):** Tasks 10–12 (operator workflow scripts/runbook), Task 13 (order advisor crate)
- **Batch 4 (after Task 5):** Tasks 14–16 (candle aggregator, SSE, frontend chart)
- **Batch 5 (after Tasks 10+14):** Tasks 17–18 (workspace API + frontend)

---

## File Map

### New files
- `crates/pt-polymarket/src/ws.rs` — PolymarketWsClient + BookUpdate
- `crates/pt-signal/src/cb_price.rs` — CoinbasePriceAdapter
- `crates/pt-engine/src/candle_agg.rs` — CandleAggregator
- `crates/pt-order-advisor/` — new crate (Cargo.toml, src/lib.rs, src/advisor.rs, src/monitor.rs)
- `infra/systemd/tailscale-reauth.service.template` — systemd unit
- `infra/systemd/tailscale-reauth.timer` — systemd timer
- `scripts/pi_dev_tailscale_setup.sh` — Tailscale auto-reauth installer
- `scripts/pi_dev_cloudflare.sh` — Cloudflare tunnel installer
- `docs/runbooks/LAMBDA_MIGRATION.md` — migration runbook
- `scripts/webhook_dual_write_test.sh` — dual-write smoke test

### Modified files
- `Cargo.toml` — add dashmap workspace dep, add pt-order-advisor to workspace members
- `crates/pt-engine/Cargo.toml` — add dashmap, pt-tsdb, pt-order-advisor deps
- `crates/pt-engine/src/lib.rs` — dashmap migration, rate-limit backoff, candle aggregator wiring, candle broadcast in DashboardHandles, retention cron call
- `crates/pt-dashboard/Cargo.toml` — add dashmap dep
- `crates/pt-dashboard/src/lib.rs` — dashmap in DashboardHandles/State, candle broadcast field, SSE candle endpoint, comparison endpoint, workspace endpoints
- `crates/pt-tsdb/src/lib.rs` — add prune_older_than_days
- `crates/pt-polymarket/src/lib.rs` — pub mod ws
- `crates/pt-signal/src/lib.rs` — pub mod cb_price, re-export CoinbasePriceAdapter
- `crates/pt-strategy-lab/src/signals.rs` — wire session_vwap signal
- `scripts/pi_lib.sh` — fix /22 subnet sweep
- `project.json` — add pi-dev-tailscale-setup, pi-dev-cloudflare-setup targets

---

## Task 1: Add dashmap to workspace

**Files:**
- Modify: `Cargo.toml`
- Modify: `crates/pt-engine/Cargo.toml`
- Modify: `crates/pt-dashboard/Cargo.toml`

- [ ] **Step 1: Add dashmap to workspace Cargo.toml**

In the `[workspace.dependencies]` section of `Cargo.toml`, add after the `futures` line:

```toml
dashmap = "6"
```

- [ ] **Step 2: Add dashmap to pt-engine and pt-dashboard**

In `crates/pt-engine/Cargo.toml` `[dependencies]`, add:
```toml
dashmap.workspace = true
```

In `crates/pt-dashboard/Cargo.toml` `[dependencies]`, add:
```toml
dashmap.workspace = true
```

- [ ] **Step 3: Verify compilation**

```bash
cargo check -p pt-engine -p pt-dashboard 2>&1 | tail -5
```
Expected: no errors (dashmap just added, nothing uses it yet).

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml Cargo.lock crates/pt-engine/Cargo.toml crates/pt-dashboard/Cargo.toml
git commit -m "chore: add dashmap 6 to workspace dependencies"
```

---

## Task 2: Migrate latest_books to DashMap (B2)

**Files:**
- Modify: `crates/pt-engine/src/lib.rs`
- Modify: `crates/pt-dashboard/src/lib.rs`

- [ ] **Step 1: Update SharedState in pt-engine**

In `crates/pt-engine/src/lib.rs`, add `use dashmap::DashMap;` near the top imports.

Change `SharedState`:
```rust
// Before:
latest_books: Arc<RwLock<HashMap<String, MarketSnapshot>>>,

// After:
latest_books: Arc<DashMap<String, MarketSnapshot>>,
```

Change `SharedState::new()`:
```rust
// Before:
latest_books: Arc::new(RwLock::new(HashMap::new())),

// After:
latest_books: Arc::new(DashMap::new()),
```

- [ ] **Step 2: Update all read/write accesses in pt-engine**

In `spawn_orderbook_loop` (around line 720), change the insert:
```rust
// Before:
latest.write().insert(m.market_id.clone(), snap.clone());

// After:
latest.insert(m.market_id.clone(), snap.clone());
```

Find all other `latest_books` usages with `write()` or `read()`:

```bash
grep -n "latest_books\|\.read()\|\.write()" crates/pt-engine/src/lib.rs | grep -v "^Binary" | head -30
```

For each `latest.read().get(key)` pattern → change to `latest.get(key)`.
For each `latest.read().clone()` pattern → change to `latest.iter().map(|e| e.value().clone()).collect::<Vec<_>>()` (or equivalent).

- [ ] **Step 3: Update DashboardHandles and DashboardState in pt-dashboard**

In `crates/pt-dashboard/src/lib.rs`, add `use dashmap::DashMap;`.

Change `DashboardHandles`:
```rust
// Before:
pub latest_books: Arc<RwLock<HashMap<String, MarketSnapshot>>>,

// After:
pub latest_books: Arc<DashMap<String, MarketSnapshot>>,
```

Change `DashboardHandles::default()`:
```rust
// Before:
latest_books: Arc::new(RwLock::new(HashMap::new())),

// After:
latest_books: Arc::new(DashMap::new()),
```

Change `DashboardState`:
```rust
// Before:
pub latest_books: Arc<RwLock<HashMap<String, MarketSnapshot>>>,

// After:
pub latest_books: Arc<DashMap<String, MarketSnapshot>>,
```

- [ ] **Step 4: Update dashboard read accesses**

```bash
grep -n "latest_books" crates/pt-dashboard/src/lib.rs
```

For each `state.latest_books.read().iter()` or similar:
```rust
// Before:
state.latest_books.read().values().cloned().collect::<Vec<_>>()

// After:
state.latest_books.iter().map(|e| e.value().clone()).collect::<Vec<_>>()
```

For any single-key lookup:
```rust
// Before:
state.latest_books.read().get(key).cloned()

// After:
state.latest_books.get(key).map(|e| e.value().clone())
```

- [ ] **Step 5: Build and verify tests**

```bash
cargo test -p pt-engine -p pt-dashboard 2>&1 | tail -20
```
Expected: all tests pass.

- [ ] **Step 6: Commit**

```bash
git add crates/pt-engine/src/lib.rs crates/pt-dashboard/src/lib.rs
git commit -m "perf(engine): replace RwLock<HashMap> with DashMap for latest_books"
```

---

## Task 3: Rate-limit backoff in orderbook loop (B1)

**Files:**
- Modify: `crates/pt-engine/src/lib.rs`

- [ ] **Step 1: Add backoff tracking to the loop**

In `spawn_orderbook_loop`, the inner loop iterates over `active` markets and calls `poly.get_best_book(...)`. Wrap this to detect 429-style errors.

`PtError::Http` wraps the reqwest error string. A 429 response produces a string containing "429". Add a backoff counter above the `for m in active` loop:

```rust
fn spawn_orderbook_loop(&self) -> JoinHandle<()> {
    // ... existing setup unchanged ...
    tokio::spawn(async move {
        let mut backoff_secs: u64 = 0;
        loop {
            if backoff_secs > 0 {
                tokio::time::sleep(Duration::from_secs(backoff_secs)).await;
            }
            let markets = selected.read().clone();
            // ... existing market filtering ...

            let mut got_429 = false;
            for m in active {
                match poly.get_best_book(&m.token_id_yes).await {
                    Ok(best) => {
                        backoff_secs = 0; // reset on success
                        // ... existing snap / insert / metrics code unchanged ...
                    }
                    Err(ref e) if e.to_string().contains("429") => {
                        let new_backoff = (backoff_secs * 2).max(1).min(60);
                        warn!(backoff_secs = new_backoff, "polymarket rate-limited, backing off");
                        backoff_secs = new_backoff;
                        got_429 = true;
                        metrics.inc_counter("book_poll_rate_limited", 1.0);
                        break;
                    }
                    Err(e) => {
                        warn!(market_id = %m.market_id, %e, "book poll failed");
                        metrics.inc_counter("book_poll_error", 1.0);
                    }
                }
            }

            if !got_429 {
                // ... existing storage.roll_snapshots_if_due() and sleep unchanged ...
                tokio::time::sleep(Duration::from_millis(loop_ms)).await;
            }
        }
    })
}
```

The key change: `got_429` flag breaks inner loop early and sets exponential backoff. On the next outer loop iteration, the backoff sleep fires before any polls.

- [ ] **Step 2: Build**

```bash
cargo build -p pt-engine 2>&1 | grep "^error" | head -10
```
Expected: no errors.

- [ ] **Step 3: Commit**

```bash
git add crates/pt-engine/src/lib.rs
git commit -m "feat(engine): exponential backoff on polymarket 429 rate-limit"
```

---

## Task 4: DuckDB retention — prune_older_than_days (B3)

**Files:**
- Modify: `crates/pt-tsdb/src/lib.rs`
- Modify: `crates/pt-engine/Cargo.toml`
- Modify: `crates/pt-engine/src/lib.rs`

- [ ] **Step 1: Write the failing test in pt-tsdb**

Add to the `#[cfg(test)]` section at the bottom of `crates/pt-tsdb/src/lib.rs`:

```rust
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
```

- [ ] **Step 2: Run to confirm it fails**

```bash
cargo test -p pt-tsdb prune_older_than_days 2>&1 | tail -5
```
Expected: FAIL — `prune_older_than_days` not found.

- [ ] **Step 3: Implement prune_older_than_days**

Add this method to `impl TsDb` in `crates/pt-tsdb/src/lib.rs` after `signal_count`:

```rust
pub fn prune_older_than_days(&self, table: &str, days: u32) -> PtResult<()> {
    // Only allow known table names to prevent SQL injection.
    if table != "candles" && table != "signals" {
        return Err(PtError::InvalidInput(format!("unknown table: {table}")));
    }
    let threshold_ms: i64 = chrono::Utc::now().timestamp_millis()
        - (days as i64) * 86_400_000;
    let sql = format!("DELETE FROM {table} WHERE ts_ms < {threshold_ms}");
    self.conn
        .lock()
        .execute_batch(&sql)
        .map_err(|e| PtError::Io(e.to_string()))
}
```

- [ ] **Step 4: Run to confirm it passes**

```bash
cargo test -p pt-tsdb prune_older_than_days 2>&1 | tail -5
```
Expected: test result: ok. 1 passed.

- [ ] **Step 5: Wire retention call into pt-engine**

Add `pt-tsdb = { path = "../pt-tsdb" }` to `crates/pt-engine/Cargo.toml` `[dependencies]`.

In `crates/pt-engine/src/lib.rs`, add `use pt_tsdb::TsDb;` to imports.

Add `tsdb: Option<Arc<TsDb>>` to `SharedState` and initialize it from the engine config (mirroring how `pt-dashboard` stores it). Then in the spawn loop that runs every N minutes (or in an existing periodic task), call:

```rust
// Inside a periodic task or at the top of the main engine loop (once per hour):
if let Some(ref db) = self.tsdb {
    let db = db.clone();
    tokio::spawn(async move {
        if let Err(e) = db.prune_older_than_days("candles", 90) {
            tracing::warn!("tsdb candle prune failed: {e}");
        }
        if let Err(e) = db.prune_older_than_days("signals", 90) {
            tracing::warn!("tsdb signal prune failed: {e}");
        }
    });
}
```

Place this in a new `spawn_retention_task` method that uses `tokio::time::interval(Duration::from_secs(3600))`.

- [ ] **Step 6: Build**

```bash
cargo build -p pt-engine 2>&1 | grep "^error" | head -10
```

- [ ] **Step 7: Commit**

```bash
git add crates/pt-tsdb/src/lib.rs crates/pt-engine/Cargo.toml crates/pt-engine/src/lib.rs
git commit -m "feat(tsdb): add prune_older_than_days + engine retention task (90d default)"
```

---

## Task 5: PolymarketWsClient (A1)

**Files:**
- Create: `crates/pt-polymarket/src/ws.rs`
- Modify: `crates/pt-polymarket/src/lib.rs`

- [ ] **Step 1: Write the failing tests (in a new file)**

Create `crates/pt-polymarket/src/ws.rs` with only the test module first:

```rust
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use futures::{SinkExt, StreamExt};
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
    fn default() -> Self { Self::new() }
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
    size: String,
}

impl WsMarketMsg {
    fn into_book_update(self) -> Option<BookUpdate> {
        let event_type = self.event_type.as_deref().unwrap_or("");
        if event_type != "book" && event_type != "price_change" {
            return None;
        }
        let asset_id = self.asset_id?;
        let best_bid = self.bids.iter()
            .filter_map(|l| l.price.parse::<f64>().ok())
            .reduce(f64::max);
        let best_ask = self.asks.iter()
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
        assert!(msgs.into_iter().next().unwrap().into_book_update().is_none());
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
```

- [ ] **Step 2: Run tests to verify they pass**

```bash
cargo test -p pt-polymarket 2>&1 | tail -10
```
Expected: 4 passed.

- [ ] **Step 3: Expose the module in lib.rs**

Add to `crates/pt-polymarket/src/lib.rs` near the top (after existing use statements):

```rust
pub mod ws;
pub use ws::{BookUpdate, PolymarketWsClient};
```

- [ ] **Step 4: Build**

```bash
cargo build -p pt-polymarket 2>&1 | grep "^error"
```

- [ ] **Step 5: Commit**

```bash
git add crates/pt-polymarket/src/ws.rs crates/pt-polymarket/src/lib.rs
git commit -m "feat(polymarket): PolymarketWsClient with BookUpdate broadcast channel"
```

---

## Task 6: CoinbasePriceAdapter (A2)

**Files:**
- Create: `crates/pt-signal/src/cb_price.rs`
- Modify: `crates/pt-signal/src/lib.rs`
- Modify: `crates/pt-signal/Cargo.toml`

- [ ] **Step 1: Add pt-coinbase dep to pt-signal**

In `crates/pt-signal/Cargo.toml` `[dependencies]`, add:
```toml
tokio.workspace = true
pt-coinbase = { path = "../pt-coinbase" }
```

- [ ] **Step 2: Create cb_price.rs with tests first**

Create `crates/pt-signal/src/cb_price.rs`:

```rust
use chrono::Utc;
use parking_lot::Mutex;
use pt_coinbase::WsTicker;
use serde_json::json;
use tokio::sync::broadcast;

use crate::{ExternalSignalAdapter, NormalizedExternalSignal};

/// Bridges CoinbaseWsClient into the ExternalSignalAdapter pipeline.
/// Emits a momentum bias: +1 if price rose vs last seen, -1 if fell, 0 if flat.
/// Confidence is fixed at 0.8 (price data is reliable but raw momentum is noisy).
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
    use pt_coinbase::{CoinbaseWsClient, WsTicker};

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
```

Note: `CoinbaseWsClient` has a public `sender` field — used in tests only, matching the pattern in `pt-coinbase/src/ws.rs` tests.

- [ ] **Step 3: Run tests**

```bash
cargo test -p pt-signal cb_price 2>&1 | tail -10
```
Expected: 4 passed.

- [ ] **Step 4: Expose in pt-signal lib.rs**

Add to `crates/pt-signal/src/lib.rs` at the top (after `pub mod external`):

```rust
pub mod cb_price;
pub use cb_price::CoinbasePriceAdapter;
```

- [ ] **Step 5: Build workspace**

```bash
cargo build -p pt-signal 2>&1 | grep "^error"
```

- [ ] **Step 6: Commit**

```bash
git add crates/pt-signal/src/cb_price.rs crates/pt-signal/src/lib.rs crates/pt-signal/Cargo.toml
git commit -m "feat(signal): CoinbasePriceAdapter bridges WS tickers into ExternalSignalAdapter"
```

---

## Task 7: Wire SessionVwapSignal into signals.rs (A3)

**Files:**
- Modify: `crates/pt-strategy-lab/src/signals.rs`

- [ ] **Step 1: Read the existing signals.rs to find SignalConfig and evaluation logic**

```bash
grep -n "SignalConfig\|pub struct\|session_vwap\|fn evaluate\|fn score" crates/pt-strategy-lab/src/signals.rs | head -30
```

- [ ] **Step 2: Add session_vwap fields to SignalConfig**

Locate the `SignalConfig` struct and add two fields with defaults:

```rust
#[serde(default = "default_session_vwap_threshold")]
pub session_vwap_threshold: f64,    // default 0.005
#[serde(default = "default_session_start_hour")]
pub session_start_hour_utc: u32,    // default 0
```

Add the default functions near other defaults in the file:

```rust
fn default_session_vwap_threshold() -> f64 { 0.005 }
fn default_session_start_hour() -> u32 { 0 }
```

- [ ] **Step 3: Write failing test**

Add to the `#[cfg(test)]` block in `signals.rs`:

```rust
#[test]
fn session_vwap_signal_above_threshold_emits() {
    use crate::indicators::{Candle, session_vwap};

    // Build 3 candles all in the same session bucket.
    // VWAP will be ~1.0; close is 1.01 → 1% above → above 0.5% threshold.
    let candles = vec![
        Candle { ts_ms: 0, open: 1.0, high: 1.0, low: 1.0, close: 1.0, volume: 100.0 },
        Candle { ts_ms: 86_400_000, open: 1.0, high: 1.0, low: 1.0, close: 1.0, volume: 100.0 },
        Candle { ts_ms: 2 * 86_400_000, open: 1.0, high: 1.02, low: 1.0, close: 1.01, volume: 100.0 },
    ];
    let vwaps = session_vwap(&candles, 0);
    assert!(vwaps[2].is_some(), "last candle should have a vwap");

    let cfg = SignalConfig {
        session_vwap_threshold: 0.005,
        session_start_hour_utc: 0,
        ..Default::default()
    };
    // Deviation = (1.01 - vwap) / vwap; if vwap ≈ 1.0 this is ~0.01 > threshold 0.005.
    // Confirm the signal function produces a non-zero result.
    let signal = session_vwap_signal(&candles, &cfg);
    assert!(signal.is_some(), "should emit signal when deviation exceeds threshold");
    let s = signal.unwrap();
    assert!(s.bias < 0.0, "above vwap should produce bearish bias");
}
```

- [ ] **Step 4: Run to confirm it fails**

```bash
cargo test -p pt-strategy-lab session_vwap_signal 2>&1 | tail -5
```
Expected: FAIL — `session_vwap_signal` not found.

- [ ] **Step 5: Implement session_vwap_signal**

Add this function in `signals.rs` near where other signal evaluation functions live:

```rust
/// Returns a bearish bias when price is above session VWAP by > threshold,
/// bullish when below, None when deviation is within threshold.
pub fn session_vwap_signal(
    candles: &[crate::indicators::Candle],
    cfg: &SignalConfig,
) -> Option<crate::signals::SignalOutput> {
    if candles.is_empty() {
        return None;
    }
    let vwaps = crate::indicators::session_vwap(candles, cfg.session_start_hour_utc);
    let last_vwap = vwaps.last()?.as_ref()?;
    let last_close = candles.last()?.close;
    let deviation = (last_close - last_vwap) / last_vwap;
    if deviation.abs() < cfg.session_vwap_threshold {
        return None;
    }
    // Above VWAP = price stretched up = bearish mean-reversion signal.
    let bias = if deviation > 0.0 { -1.0_f64 } else { 1.0_f64 };
    let confidence = (deviation.abs() / cfg.session_vwap_threshold).min(1.0);
    Some(crate::signals::SignalOutput { bias, confidence })
}
```

Adjust `SignalOutput` struct reference to match whatever type `signals.rs` actually uses (run `grep -n "SignalOutput\|pub struct.*Signal" crates/pt-strategy-lab/src/signals.rs` to confirm).

- [ ] **Step 6: Run tests**

```bash
cargo test -p pt-strategy-lab session_vwap 2>&1 | tail -10
```
Expected: all pass.

- [ ] **Step 7: Commit**

```bash
git add crates/pt-strategy-lab/src/signals.rs
git commit -m "feat(signals): wire session_vwap indicator into SessionVwapSignal evaluation"
```

---

## Task 8: Fix pi_lib.sh /22 subnet sweep (B4)

**Files:**
- Modify: `scripts/pi_lib.sh`

- [ ] **Step 1: Locate the _pi_local_subnet24 function**

```bash
grep -n "_pi_local_subnet24\|subnet\|/24\|local_ip\|default.*route" scripts/pi_lib.sh | head -20
```

- [ ] **Step 2: Read the full function**

```bash
grep -n "" scripts/pi_lib.sh | grep -A 20 "_pi_local_subnet24"
```

- [ ] **Step 3: Replace /24-hardcoded logic with CIDR-aware sweep**

The current function extracts just the first 3 octets (e.g. 192.168.4) and sweeps .1–.254. Replace with:

```bash
_pi_local_subnet24() {
  # Derive actual subnet from the default-route interface.
  local iface cidr prefix
  iface=$(ip route show default 2>/dev/null | awk '/default/ {print $5; exit}')
  if [[ -n "$iface" ]]; then
    cidr=$(ip addr show dev "$iface" 2>/dev/null \
      | awk '/inet / {print $2; exit}')  # e.g. 192.168.4.20/22
  fi

  if [[ -z "$cidr" ]]; then
    # Fallback: derive from hostname -I
    local local_ip
    local_ip=$(hostname -I 2>/dev/null | awk '{print $1}')
    [[ -z "$local_ip" ]] && return
    # Default to /24
    echo "${local_ip%.*}"
    return
  fi

  local ip="${cidr%/*}"
  local prefix="${cidr#*/}"

  # Cap sweep at 1024 hosts (/22 = 1022 usable).
  if (( prefix < 22 )); then
    echo "WARNING: subnet /$prefix is too large to sweep (>${prefix} bits); capping at /22" >&2
    prefix=22
  fi

  # Compute network address using bash arithmetic.
  IFS='.' read -r o1 o2 o3 o4 <<< "$ip"
  local ip_int=$(( (o1 << 24) | (o2 << 16) | (o3 << 8) | o4 ))
  local mask=$(( (0xFFFFFFFF << (32 - prefix)) & 0xFFFFFFFF ))
  local net_int=$(( ip_int & mask ))
  local host_count=$(( (1 << (32 - prefix)) - 2 ))

  local -a hosts=()
  for (( i = 1; i <= host_count; i++ )); do
    local h=$(( net_int + i ))
    hosts+=("$(( (h >> 24) & 0xFF )).$(( (h >> 16) & 0xFF )).$(( (h >> 8) & 0xFF )).$(( h & 0xFF ))")
  done
  printf '%s\n' "${hosts[@]}"
}
```

Ensure the sweep caller passes the full host list (not just a prefix). If `_pi_local_subnet24` was used as a prefix string (e.g. `"${prefix}.${i}"`), update callers to iterate over the returned host list directly.

- [ ] **Step 4: shellcheck**

```bash
shellcheck scripts/pi_lib.sh 2>&1 | grep "^scripts" | head -10
```
Fix any SC errors (common: quote variables, `(( ))` vs `[ ]`).

- [ ] **Step 5: Commit**

```bash
git add scripts/pi_lib.sh
git commit -m "fix(pi): derive actual CIDR from interface instead of hardcoding /24"
```

---

## Task 9: Tailscale auto-reauth + Cloudflare tunnel scripts (B5, C2)

**Files:**
- Create: `scripts/pi_dev_tailscale_setup.sh`
- Create: `scripts/pi_dev_cloudflare.sh`
- Create: `infra/systemd/tailscale-reauth.service.template`
- Create: `infra/systemd/tailscale-reauth.timer`
- Modify: `project.json`

- [ ] **Step 1: Create systemd templates**

Create `infra/systemd/tailscale-reauth.service.template`:

```ini
[Unit]
Description=Tailscale re-authentication
After=network-online.target tailscaled.service
Wants=network-online.target

[Service]
Type=oneshot
ExecStart=/usr/bin/tailscale up --auth-key=${TS_AUTH_KEY} --accept-routes
RemainAfterExit=no
```

Create `infra/systemd/tailscale-reauth.timer`:

```ini
[Unit]
Description=Tailscale re-auth timer (boot + 12h)

[Timer]
OnBootSec=30s
OnUnitInactiveSec=12h
Persistent=true

[Install]
WantedBy=timers.target
```

- [ ] **Step 2: Create pi_dev_tailscale_setup.sh**

Create `scripts/pi_dev_tailscale_setup.sh`:

```bash
#!/usr/bin/env bash
# Install Tailscale auto-reauth systemd timer on the Pi.
# Requires: PI_TAILSCALE_AUTH_KEY env var (ephemeral key from Tailscale admin panel).
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=pi_lib.sh
source "$SCRIPT_DIR/pi_lib.sh"

: "${PI_TAILSCALE_AUTH_KEY:?PI_TAILSCALE_AUTH_KEY is required}"

PI_HOST=$(resolve_pi_host)
PI_USER=$(resolve_pi_user)
PI_KEY=$(resolve_pi_key)
PI_SSH="ssh -i $PI_KEY -o StrictHostKeyChecking=no $PI_USER@$PI_HOST"

echo "==> Installing Tailscale auto-reauth on $PI_HOST"

# Render service file with key substituted (stored only on Pi, never in repo).
SERVICE_CONTENT=$(sed "s|\${TS_AUTH_KEY}|$PI_TAILSCALE_AUTH_KEY|g" \
  "$SCRIPT_DIR/../infra/systemd/tailscale-reauth.service.template")

$PI_SSH "sudo tee /etc/systemd/system/tailscale-reauth.service > /dev/null" <<< "$SERVICE_CONTENT"
scp -i "$PI_KEY" -o StrictHostKeyChecking=no \
  "$SCRIPT_DIR/../infra/systemd/tailscale-reauth.timer" \
  "$PI_USER@$PI_HOST:/tmp/tailscale-reauth.timer"
$PI_SSH "sudo mv /tmp/tailscale-reauth.timer /etc/systemd/system/ && \
  sudo chmod 600 /etc/systemd/system/tailscale-reauth.service && \
  sudo systemctl daemon-reload && \
  sudo systemctl enable --now tailscale-reauth.timer && \
  tailscale status | head -3"

echo "==> Tailscale auto-reauth installed."
```

```bash
chmod +x scripts/pi_dev_tailscale_setup.sh
```

- [ ] **Step 3: Create pi_dev_cloudflare.sh**

Create `scripts/pi_dev_cloudflare.sh`:

```bash
#!/usr/bin/env bash
# Install Cloudflare Tunnel on the Pi for stable remote dashboard access.
# Requires: CF_TUNNEL_TOKEN env var (from Cloudflare Zero Trust dashboard).
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=pi_lib.sh
source "$SCRIPT_DIR/pi_lib.sh"

: "${CF_TUNNEL_TOKEN:?CF_TUNNEL_TOKEN is required}"

PI_HOST=$(resolve_pi_host)
PI_USER=$(resolve_pi_user)
PI_KEY=$(resolve_pi_key)
PI_SSH="ssh -i $PI_KEY -o StrictHostKeyChecking=no $PI_USER@$PI_HOST"

echo "==> Installing cloudflared on $PI_HOST"

$PI_SSH "
  curl -fsSL https://pkg.cloudflare.com/cloudflare-main.gpg \
    | sudo tee /usr/share/keyrings/cloudflare-main.gpg > /dev/null
  echo 'deb [signed-by=/usr/share/keyrings/cloudflare-main.gpg] https://pkg.cloudflare.com/cloudflared any main' \
    | sudo tee /etc/apt/sources.list.d/cloudflared.list
  sudo apt-get update -qq
  sudo apt-get install -y cloudflared
"

echo "==> Registering tunnel"
$PI_SSH "sudo cloudflared service install '$CF_TUNNEL_TOKEN'"

echo "==> Verifying tunnel"
$PI_SSH "cloudflared tunnel info 2>/dev/null || echo '(info unavailable — check Cloudflare dashboard)'"
echo "==> Done."
```

```bash
chmod +x scripts/pi_dev_cloudflare.sh
```

- [ ] **Step 4: Add Nx targets to project.json**

In `project.json`, locate the `pi-dev-deploy-tailscale` target (or similar) and add alongside it:

```json
"pi-dev-tailscale-setup": {
  "executor": "nx:run-commands",
  "options": {
    "command": "./scripts/pi_dev_tailscale_setup.sh",
    "cwd": "{workspaceRoot}"
  }
},
"pi-dev-cloudflare-setup": {
  "executor": "nx:run-commands",
  "options": {
    "command": "./scripts/pi_dev_cloudflare.sh",
    "cwd": "{workspaceRoot}"
  }
}
```

- [ ] **Step 5: shellcheck both scripts**

```bash
shellcheck scripts/pi_dev_tailscale_setup.sh scripts/pi_dev_cloudflare.sh 2>&1 | grep "^scripts" | head -10
```

- [ ] **Step 6: Commit**

```bash
git add scripts/pi_dev_tailscale_setup.sh scripts/pi_dev_cloudflare.sh \
  infra/systemd/tailscale-reauth.service.template \
  infra/systemd/tailscale-reauth.timer project.json
git commit -m "feat(pi): Tailscale auto-reauth timer + Cloudflare tunnel installer scripts"
```

---

## Task 10: Lambda migration runbook (C3)

**Files:**
- Create: `docs/runbooks/LAMBDA_MIGRATION.md`
- Create: `scripts/webhook_dual_write_test.sh`

- [ ] **Step 1: Create the runbook**

Create `docs/runbooks/LAMBDA_MIGRATION.md`:

```markdown
# Lambda Migration Runbook

Migrate the TradingView webhook + signal ingestion layer from Raspberry Pi
to AWS Lambda + API Gateway, with zero-downtime cutover via dual-write.

## Prerequisites

- AWS account with Lambda + API Gateway permissions
- Pi currently serving webhooks at `PI_WEBHOOK_URL` (read from `.env.pi`)
- Lambda function deployed at `LAMBDA_WEBHOOK_URL` (see below)
- `jq`, `curl` available locally

## Architecture (dual-write phase)

```
TradingView alert
      │
      ├──► Pi endpoint (current)         → pt-engine signal pipeline
      └──► Lambda endpoint (new)         → same payload, same response contract
```

Both endpoints receive every alert during the dual-write window. Compare
responses to validate Lambda parity before cutting over DNS.

## Lambda Deployment

1. Package the webhook handler:
   ```bash
   cd lambda/webhook-handler && zip -r ../webhook.zip .
   aws lambda create-function \
     --function-name pt-webhook \
     --runtime python3.12 \
     --handler handler.lambda_handler \
     --zip-file fileb://../webhook.zip \
     --role arn:aws:iam::ACCOUNT_ID:role/lambda-basic-exec
   ```

2. Create API Gateway HTTP API and integrate with `pt-webhook`.

3. Set `LAMBDA_WEBHOOK_URL` in your local `.env` to the Gateway invoke URL.

## Dual-Write Test

Run before cutover to validate parity:

```bash
PI_WEBHOOK_URL=https://pi.example.com/webhook \
LAMBDA_WEBHOOK_URL=https://api.example.com/webhook \
./scripts/webhook_dual_write_test.sh
```

Exits 0 if both endpoints return HTTP 200 and identical JSON response bodies.
Exits 1 and prints diff if responses diverge.

## Cutover Steps

1. Run `webhook_dual_write_test.sh` — must exit 0.
2. Update TradingView alert webhook URL to Lambda endpoint.
3. Monitor Lambda CloudWatch logs for 30 minutes.
4. If stable: stop `pt-engine` on Pi, disable Pi webhook service.
5. Run `pi-dev-down` to stop the Pi service.

## Rollback

Revert TradingView alert URL to Pi endpoint. Pi service should still be running
(was not stopped until step 4 above).

## Success Criteria

- Dual-write test exits 0 for 3 consecutive runs.
- Lambda p99 latency ≤ 500ms (CloudWatch metric: `pt-webhook` invocation duration).
- No signal processing errors in `pt-engine` log for 30 minutes post-cutover.
```

- [ ] **Step 2: Create dual-write test script**

Create `scripts/webhook_dual_write_test.sh`:

```bash
#!/usr/bin/env bash
set -euo pipefail

: "${PI_WEBHOOK_URL:?PI_WEBHOOK_URL required}"
: "${LAMBDA_WEBHOOK_URL:?LAMBDA_WEBHOOK_URL required}"

PAYLOAD='{"strategy_name":"test","action":"buy","bias":0.8,"confidence":0.9}'
HEADERS='-H "Content-Type: application/json"'

echo "==> Sending test payload to Pi..."
PI_RESP=$(curl -sfS -X POST "$PI_WEBHOOK_URL" \
  -H "Content-Type: application/json" \
  -d "$PAYLOAD" -w "\n%{http_code}" 2>&1)
PI_CODE=$(echo "$PI_RESP" | tail -1)
PI_BODY=$(echo "$PI_RESP" | head -n -1)

echo "==> Sending test payload to Lambda..."
LAMBDA_RESP=$(curl -sfS -X POST "$LAMBDA_WEBHOOK_URL" \
  -H "Content-Type: application/json" \
  -d "$PAYLOAD" -w "\n%{http_code}" 2>&1)
LAMBDA_CODE=$(echo "$LAMBDA_RESP" | tail -1)
LAMBDA_BODY=$(echo "$LAMBDA_RESP" | head -n -1)

echo "Pi HTTP $PI_CODE: $PI_BODY"
echo "Lambda HTTP $LAMBDA_CODE: $LAMBDA_BODY"

if [[ "$PI_CODE" != "200" ]]; then
  echo "FAIL: Pi returned non-200 ($PI_CODE)"
  exit 1
fi
if [[ "$LAMBDA_CODE" != "200" ]]; then
  echo "FAIL: Lambda returned non-200 ($LAMBDA_CODE)"
  exit 1
fi

DIFF=$(diff \
  <(echo "$PI_BODY" | jq -S . 2>/dev/null || echo "$PI_BODY") \
  <(echo "$LAMBDA_BODY" | jq -S . 2>/dev/null || echo "$LAMBDA_BODY") \
  || true)

if [[ -n "$DIFF" ]]; then
  echo "FAIL: response bodies differ:"
  echo "$DIFF"
  exit 1
fi

echo "PASS: both endpoints returned HTTP 200 with identical bodies."
```

```bash
chmod +x scripts/webhook_dual_write_test.sh
```

- [ ] **Step 3: shellcheck**

```bash
shellcheck scripts/webhook_dual_write_test.sh 2>&1 | head -5
```

- [ ] **Step 4: Commit**

```bash
git add docs/runbooks/LAMBDA_MIGRATION.md scripts/webhook_dual_write_test.sh
git commit -m "docs(ops): Lambda migration runbook + dual-write validation script"
```

---

## Task 11: pt-order-advisor crate (Group D)

**Files:**
- Create: `crates/pt-order-advisor/Cargo.toml`
- Create: `crates/pt-order-advisor/src/lib.rs`
- Create: `crates/pt-order-advisor/src/advisor.rs`
- Create: `crates/pt-order-advisor/src/monitor.rs`
- Modify: `Cargo.toml` (workspace members)
- Modify: `crates/pt-engine/Cargo.toml`

- [ ] **Step 1: Create crate scaffold**

```bash
mkdir -p crates/pt-order-advisor/src
```

Create `crates/pt-order-advisor/Cargo.toml`:

```toml
[package]
name = "pt-order-advisor"
version.workspace = true
edition.workspace = true
license.workspace = true
authors.workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true
tracing.workspace = true
```

Add to workspace `Cargo.toml` members array:
```toml
"crates/pt-order-advisor",
```

- [ ] **Step 2: Write all tests first in lib.rs**

Create `crates/pt-order-advisor/src/lib.rs`:

```rust
pub mod advisor;
pub mod monitor;

pub use advisor::{
    Config, ContextualAdvisor, MarketContext, MakerTakerAdvisor, PlacementMode,
};
pub use monitor::ProfitProtectionMonitor;
```

- [ ] **Step 3: Write tests for advisor.rs**

Create `crates/pt-order-advisor/src/advisor.rs` with tests first:

```rust
/// Configures spread/velocity thresholds for placement decisions.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    /// Below this spread (bps), escalate to Taker when velocity is also high.
    pub taker_threshold_bps: f64,
    /// Taker escalation also requires tick velocity ≥ this (adverse ticks/sec).
    pub velocity_threshold: f64,
    /// Above this spread (bps), skip — market is too illiquid.
    pub skip_threshold_bps: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            taker_threshold_bps: 5.0,
            velocity_threshold: 0.5,
            skip_threshold_bps: 50.0,
        }
    }
}

/// Snapshot of current market conditions used for placement decisions.
#[derive(Debug, Clone, Default)]
pub struct MarketContext {
    pub spread_bps: f64,
    pub tick_velocity: f64,
    pub signal_direction: i8,  // +1 bullish, -1 bearish, 0 neutral
    pub position_pnl: f64,     // unrealized P&L, used for context only
}

/// The placement mode recommended by an advisor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlacementMode {
    Maker,
    Taker,
    Skip,
}

/// Stateless advisor: maps MarketContext to PlacementMode.
/// Default = Maker (0% fee). Taker only when spread is tight AND velocity is high.
pub struct MakerTakerAdvisor {
    pub cfg: Config,
}

impl MakerTakerAdvisor {
    pub fn new(cfg: Config) -> Self { Self { cfg } }

    pub fn advise(&self, ctx: &MarketContext) -> PlacementMode {
        if ctx.spread_bps >= self.cfg.skip_threshold_bps {
            return PlacementMode::Skip;
        }
        if ctx.spread_bps <= self.cfg.taker_threshold_bps
            && ctx.tick_velocity >= self.cfg.velocity_threshold
        {
            return PlacementMode::Taker;
        }
        PlacementMode::Maker
    }
}

/// Wraps MakerTakerAdvisor; defaults to Maker when there is no open position.
pub struct ContextualAdvisor {
    inner: MakerTakerAdvisor,
}

impl ContextualAdvisor {
    pub fn new(cfg: Config) -> Self {
        Self { inner: MakerTakerAdvisor::new(cfg) }
    }

    /// `has_position`: true if the engine currently holds an open position.
    pub fn advise(&self, ctx: &MarketContext, has_position: bool) -> PlacementMode {
        if !has_position {
            // Entry: always try Maker first regardless of velocity.
            if ctx.spread_bps >= self.inner.cfg.skip_threshold_bps {
                return PlacementMode::Skip;
            }
            return PlacementMode::Maker;
        }
        self.inner.advise(ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(spread_bps: f64, velocity: f64) -> MarketContext {
        MarketContext { spread_bps, tick_velocity: velocity, ..Default::default() }
    }

    #[test]
    fn default_is_maker() {
        let advisor = MakerTakerAdvisor::new(Config::default());
        assert_eq!(advisor.advise(&ctx(10.0, 0.0)), PlacementMode::Maker);
    }

    #[test]
    fn wide_spread_skips() {
        let advisor = MakerTakerAdvisor::new(Config::default());
        assert_eq!(advisor.advise(&ctx(60.0, 1.0)), PlacementMode::Skip);
    }

    #[test]
    fn tight_spread_high_velocity_is_taker() {
        let advisor = MakerTakerAdvisor::new(Config::default());
        assert_eq!(advisor.advise(&ctx(3.0, 0.8)), PlacementMode::Taker);
    }

    #[test]
    fn tight_spread_low_velocity_is_maker() {
        let advisor = MakerTakerAdvisor::new(Config::default());
        assert_eq!(advisor.advise(&ctx(3.0, 0.2)), PlacementMode::Maker);
    }

    #[test]
    fn contextual_no_position_always_maker() {
        let advisor = ContextualAdvisor::new(Config::default());
        // Even with high velocity, no-position entry uses Maker.
        assert_eq!(advisor.advise(&ctx(3.0, 2.0), false), PlacementMode::Maker);
    }

    #[test]
    fn contextual_no_position_wide_spread_skips() {
        let advisor = ContextualAdvisor::new(Config::default());
        assert_eq!(advisor.advise(&ctx(60.0, 0.0), false), PlacementMode::Skip);
    }

    #[test]
    fn contextual_with_position_delegates_to_inner() {
        let advisor = ContextualAdvisor::new(Config::default());
        assert_eq!(advisor.advise(&ctx(3.0, 0.8), true), PlacementMode::Taker);
    }

    #[test]
    fn skip_threshold_exact_boundary() {
        let advisor = MakerTakerAdvisor::new(Config::default());
        // At exactly 50.0 bps (skip_threshold_bps) → Skip.
        assert_eq!(advisor.advise(&ctx(50.0, 0.0)), PlacementMode::Skip);
    }
}
```

- [ ] **Step 4: Write tests for monitor.rs**

Create `crates/pt-order-advisor/src/monitor.rs`:

```rust
use crate::advisor::PlacementMode;

/// Configuration for the two-gate profit protection monitor.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MonitorConfig {
    /// Number of consecutive adverse ticks needed to arm Gate 1.
    pub adverse_ticks_to_arm: usize,
    /// Rolling window in seconds for tick velocity measurement.
    pub window_secs: u64,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self { adverse_ticks_to_arm: 3, window_secs: 60 }
    }
}

/// Two-gate profit protection monitor.
///
/// Gate 1 (velocity arm): N consecutive adverse ticks within rolling window.
/// Gate 2 (signal confirm): composite signal flips against position direction.
///
/// Both gates must be open to recommend Taker escalation for exit.
/// The monitor is purely advisory — the engine decides whether to act.
#[derive(Debug, Default)]
pub struct ProfitProtectionMonitor {
    cfg: MonitorConfig,
    /// Direction of current open position: +1 long, -1 short, 0 none.
    position_direction: i8,
    consecutive_adverse: usize,
    gate1_armed: bool,
}

impl ProfitProtectionMonitor {
    pub fn new(cfg: MonitorConfig) -> Self {
        Self { cfg, ..Default::default() }
    }

    /// Call when a position is opened. Resets monitor state.
    pub fn on_position_opened(&mut self, direction: i8) {
        self.position_direction = direction;
        self.consecutive_adverse = 0;
        self.gate1_armed = false;
    }

    /// Call when position is closed. Resets state.
    pub fn on_position_closed(&mut self) {
        self.position_direction = 0;
        self.consecutive_adverse = 0;
        self.gate1_armed = false;
    }

    /// Feed a price tick. `price_direction`: +1 if price rose, -1 if fell.
    /// A tick is adverse if it moves against the position direction.
    pub fn on_tick(&mut self, price_direction: i8) {
        if self.position_direction == 0 { return; }
        let adverse = price_direction != 0 && price_direction != self.position_direction;
        if adverse {
            self.consecutive_adverse += 1;
        } else {
            self.consecutive_adverse = 0;
        }
        if self.consecutive_adverse >= self.cfg.adverse_ticks_to_arm {
            self.gate1_armed = true;
        }
    }

    /// Feed the current composite signal direction.
    /// Returns PlacementMode::Taker if both gates are open, otherwise Maker.
    pub fn check(&self, signal_direction: i8) -> PlacementMode {
        if self.position_direction == 0 { return PlacementMode::Maker; }
        if !self.gate1_armed { return PlacementMode::Maker; }
        // Gate 2: signal flipped against position.
        let signal_flipped = signal_direction != 0
            && signal_direction != self.position_direction;
        if signal_flipped {
            PlacementMode::Taker
        } else {
            PlacementMode::Maker
        }
    }

    pub fn is_armed(&self) -> bool { self.gate1_armed }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_position_always_maker() {
        let mut m = ProfitProtectionMonitor::new(MonitorConfig::default());
        m.on_tick(-1);
        m.on_tick(-1);
        m.on_tick(-1);
        assert_eq!(m.check(-1), PlacementMode::Maker);
    }

    #[test]
    fn gate1_not_armed_until_n_consecutive() {
        let mut m = ProfitProtectionMonitor::new(MonitorConfig { adverse_ticks_to_arm: 3, window_secs: 60 });
        m.on_position_opened(1); // long
        m.on_tick(-1); // adverse
        m.on_tick(-1); // adverse (2)
        assert!(!m.is_armed());
        m.on_tick(-1); // adverse (3) → arms Gate 1
        assert!(m.is_armed());
    }

    #[test]
    fn favorable_tick_resets_consecutive() {
        let mut m = ProfitProtectionMonitor::new(MonitorConfig::default());
        m.on_position_opened(1);
        m.on_tick(-1);
        m.on_tick(-1);
        m.on_tick(1); // favorable — resets
        assert_eq!(m.consecutive_adverse, 0);
        assert!(!m.is_armed());
    }

    #[test]
    fn both_gates_open_recommends_taker() {
        let mut m = ProfitProtectionMonitor::new(MonitorConfig { adverse_ticks_to_arm: 2, window_secs: 60 });
        m.on_position_opened(1); // long
        m.on_tick(-1);
        m.on_tick(-1); // Gate 1 armed
        // Gate 2: signal flipped bearish (against long position)
        assert_eq!(m.check(-1), PlacementMode::Taker);
    }

    #[test]
    fn gate1_armed_but_signal_agrees_stays_maker() {
        let mut m = ProfitProtectionMonitor::new(MonitorConfig { adverse_ticks_to_arm: 2, window_secs: 60 });
        m.on_position_opened(1);
        m.on_tick(-1);
        m.on_tick(-1); // Gate 1 armed
        // Signal still bullish (agrees with position) → Gate 2 not open
        assert_eq!(m.check(1), PlacementMode::Maker);
    }

    #[test]
    fn position_close_resets_monitor() {
        let mut m = ProfitProtectionMonitor::new(MonitorConfig { adverse_ticks_to_arm: 2, window_secs: 60 });
        m.on_position_opened(1);
        m.on_tick(-1);
        m.on_tick(-1);
        assert!(m.is_armed());
        m.on_position_closed();
        assert!(!m.is_armed());
        assert_eq!(m.check(-1), PlacementMode::Maker);
    }

    #[test]
    fn short_position_adverse_tick_is_price_rise() {
        let mut m = ProfitProtectionMonitor::new(MonitorConfig { adverse_ticks_to_arm: 2, window_secs: 60 });
        m.on_position_opened(-1); // short
        m.on_tick(1);  // price rising = adverse for short
        m.on_tick(1);
        assert!(m.is_armed());
    }
}
```

- [ ] **Step 5: Run all advisor tests**

```bash
cargo test -p pt-order-advisor 2>&1 | tail -10
```
Expected: all 15 tests pass.

- [ ] **Step 6: Add pt-order-advisor dep to pt-engine**

In `crates/pt-engine/Cargo.toml`:
```toml
pt-order-advisor = { path = "../pt-order-advisor" }
```

- [ ] **Step 7: Build**

```bash
cargo build -p pt-order-advisor -p pt-engine 2>&1 | grep "^error"
```

- [ ] **Step 8: Commit**

```bash
git add crates/pt-order-advisor/ Cargo.toml Cargo.lock crates/pt-engine/Cargo.toml
git commit -m "feat(order-advisor): new crate with MakerTakerAdvisor + ProfitProtectionMonitor"
```

---

## Task 12: CandleAggregator + SSE candle broadcast (Group E)

**Files:**
- Create: `crates/pt-engine/src/candle_agg.rs`
- Modify: `crates/pt-engine/src/lib.rs`
- Modify: `crates/pt-dashboard/src/lib.rs`

- [ ] **Step 1: Write failing tests in candle_agg.rs**

Create `crates/pt-engine/src/candle_agg.rs`:

```rust
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
                // Crossing a boundary: emit the completed candle.
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
        // All ticks within the same 60s bucket — no candle emitted yet.
        assert!(agg.ingest("X", 1.0, 10.0, 0).is_none());
        assert!(agg.ingest("X", 1.1, 10.0, 30_000).is_none());
        assert!(agg.ingest("X", 0.9, 10.0, 59_000).is_none());
    }

    #[test]
    fn boundary_crossing_emits_candle() {
        let mut agg = CandleAggregator::new(60);
        agg.ingest("X", 1.0, 10.0, 0);
        agg.ingest("X", 1.2, 10.0, 30_000);
        // Tick at 60_000ms crosses the bucket boundary.
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
        agg.ingest("Y", 7.0, 2.0, 20_000);  // new high
        agg.ingest("Y", 3.0, 3.0, 45_000);  // new low
        agg.ingest("Y", 6.0, 1.0, 59_000);  // close
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
```

- [ ] **Step 2: Run to confirm tests pass**

```bash
cargo test -p pt-engine candle_agg 2>&1 | tail -10
```
Expected: 4 passed (module needs to be declared first).

- [ ] **Step 3: Declare module in lib.rs**

In `crates/pt-engine/src/lib.rs`, near the top:
```rust
pub mod candle_agg;
pub use candle_agg::{Candle as EngineCandle, CandleAggregator};
```

Rerun tests to confirm.

- [ ] **Step 4: Add candle broadcast to DashboardHandles**

In `crates/pt-dashboard/src/lib.rs`, add to `DashboardHandles`:
```rust
pub candle_tx: tokio::sync::broadcast::Sender<crate::candle_agg::Candle>,
```

Wait — `pt-dashboard` doesn't depend on `pt-engine`. The `Candle` type should live in `pt-core` or be re-exported from `pt-engine` and added as a dep. Simpler: define a `LiveCandle` struct directly in `pt-dashboard` (since dashboard already depends on pt-engine indirectly via types). 

Actually: add `pt-engine` as a dep of `pt-dashboard`? That creates a circular dep (`pt-engine` already depends on `pt-dashboard`). 

Correct approach: move `Candle` (or define a `LiveCandle` alias) to `pt-core`, or duplicate a slim `LiveCandle` struct in `pt-dashboard`. Use duplication (YAGNI — don't restructure crate graph for one struct):

In `pt-dashboard/src/lib.rs`, define:
```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LiveCandle {
    pub asset_id: String,
    pub open: f64, pub high: f64, pub low: f64, pub close: f64, pub volume: f64,
    pub ts_open_ms: i64, pub ts_close_ms: i64,
}
```

Add to `DashboardHandles`:
```rust
pub candle_tx: Arc<tokio::sync::broadcast::Sender<LiveCandle>>,
```

Add to `DashboardHandles::default()`:
```rust
candle_tx: {
    let (tx, _) = tokio::sync::broadcast::channel(512);
    Arc::new(tx)
},
```

Add to `DashboardState`:
```rust
pub candle_tx: Arc<tokio::sync::broadcast::Sender<LiveCandle>>,
```

Copy it through in `DashboardState::new(handles)`:
```rust
candle_tx: handles.candle_tx,
```

- [ ] **Step 5: Add SSE live candle endpoint**

In `crates/pt-dashboard/src/lib.rs`, add a new route handler:

```rust
async fn get_live_candles(
    Query(q): Query<StreamCandleQuery>,
    State(state): State<DashboardState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let mut rx = state.candle_tx.subscribe();
    let asset_filter = q.product_id.clone();

    let stream = async_stream::stream! {
        loop {
            match rx.recv().await {
                Ok(candle) if candle.asset_id == asset_filter => {
                    let data = serde_json::to_string(&candle).unwrap_or_default();
                    yield Ok::<_, Infallible>(
                        Event::default().event("candle").data(data)
                    );
                }
                Ok(_) => continue,
                Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                    tracing::warn!("live candle SSE lagged {n}");
                    continue;
                }
                Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
            }
        }
    };

    Sse::new(stream).keep_alive(KeepAlive::default())
}
```

Add `async-stream = "0.3"` to `crates/pt-dashboard/Cargo.toml` `[dependencies]`.

Register the route in `router()`:
```rust
.route("/api/v1/charts/candles/live", get(get_live_candles))
```

- [ ] **Step 6: Wire CandleAggregator into pt-engine**

In `crates/pt-engine/src/lib.rs`, after the existing book poll logic inside `spawn_orderbook_loop`, when a successful `MarketSnapshot` is received, ingest the mid-price into a `CandleAggregator`:

Add `candle_agg: Arc<Mutex<CandleAggregator>>` and `candle_tx: Arc<broadcast::Sender<EngineCandle>>` to `SharedState`.

In the success branch of `spawn_orderbook_loop`:
```rust
let mid = (best.best_bid + best.best_ask) / 2.0;
let ts_ms = best.ts_ms;
if let Some(candle) = candle_agg.lock().ingest(&m.market_id, mid, 0.0, ts_ms) {
    // Convert EngineCandle to LiveCandle and broadcast.
    let _ = candle_tx.send(candle);
}
```

Wire `candle_tx` into `DashboardHandles` before calling `DashboardState::new`.

- [ ] **Step 7: Build**

```bash
cargo build -p pt-engine -p pt-dashboard 2>&1 | grep "^error" | head -10
```

- [ ] **Step 8: Commit**

```bash
git add crates/pt-engine/src/candle_agg.rs crates/pt-engine/src/lib.rs \
  crates/pt-dashboard/src/lib.rs crates/pt-dashboard/Cargo.toml
git commit -m "feat(engine,dashboard): CandleAggregator + live SSE candle endpoint /api/v1/charts/candles/live"
```

---

## Task 13: Frontend live chart EventSource (Group E continued)

**Files:**
- Modify: `crates/pt-dashboard/src/` (frontend TypeScript)

- [ ] **Step 1: Find the existing chart component**

```bash
find crates/pt-dashboard/src -name "*.ts" -o -name "*.tsx" -o -name "*.js" | xargs grep -l "candle\|chart\|stream" 2>/dev/null | head -5
```

- [ ] **Step 2: Locate the chart render code and add EventSource**

Once found, add an `EventSource` subscription that appends incoming candles:

```typescript
// In the relevant chart component/module:
function connectLiveCandleStream(assetId: string, onCandle: (c: LiveCandle) => void) {
  const es = new EventSource(`/api/v1/charts/candles/live?product_id=${encodeURIComponent(assetId)}&granularity=60`);
  es.addEventListener('candle', (e: MessageEvent) => {
    try {
      const candle: LiveCandle = JSON.parse(e.data);
      onCandle(candle);
    } catch { /* ignore parse errors */ }
  });
  es.onerror = () => {
    console.warn('Live candle SSE disconnected, retrying...');
    // Browser auto-retries SSE on error after ~3s. No manual reconnect needed.
  };
  return es;
}

interface LiveCandle {
  asset_id: string;
  open: number; high: number; low: number; close: number; volume: number;
  ts_open_ms: number; ts_close_ms: number;
}
```

In the chart initialization code, after loading historical candles via REST, call `connectLiveCandleStream` and on each incoming candle:
1. Check if the last chart bar has the same `ts_open_ms` → update it in-place.
2. Otherwise append a new bar.
3. Call `chart.update()` or the equivalent for the chart library in use.

- [ ] **Step 3: Build frontend**

```bash
cd crates/pt-dashboard && pnpm run build 2>&1 | tail -10
```
Expected: build succeeds.

- [ ] **Step 4: Commit**

```bash
git add crates/pt-dashboard/src/
git commit -m "feat(dashboard): live candle EventSource integration for chart"
```

---

## Task 14: Artifact comparison endpoint (C1)

**Files:**
- Modify: `crates/pt-dashboard/src/lib.rs`
- Modify: `crates/pt-dashboard/src/` (frontend)

- [ ] **Step 1: Read how backtest artifacts are stored**

```bash
grep -n "StrategyRunReport\|run_id\|manifest\|last_backtest\|get_backtest" crates/pt-dashboard/src/lib.rs | head -20
grep -n "run_id\|pub struct StrategyRunReport" crates/pt-strategy-lab/src/lib.rs | head -10
```

- [ ] **Step 2: Add comparison query params struct**

In `crates/pt-dashboard/src/lib.rs`, add:

```rust
#[derive(serde::Deserialize)]
struct CompareQuery {
    a: String,
    b: String,
}
```

- [ ] **Step 3: Implement comparison handler**

Backtest manifests are written to `data/backtest/{run_id}.json` by `save_run_manifest`. Load them from the filesystem and diff key metrics. `StrategyRunReport` fields: `run_id`, `max_drawdown_pct: f64`, `trades: usize`, `win_rate: f64`, `total_return_pct: f64`, `pnl: f64`.

Add handler:
```rust
async fn get_artifacts_compare(
    Query(q): Query<CompareQuery>,
) -> impl IntoResponse {
    fn load_report(run_id: &str) -> Option<pt_strategy_lab::StrategyRunReport> {
        let path = format!("data/backtest/{run_id}.json");
        let raw = std::fs::read_to_string(&path).ok()?;
        serde_json::from_str(&raw).ok()
    }

    let (Some(a), Some(b)) = (load_report(&q.a), load_report(&q.b)) else {
        return (StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "one or both run_ids not found"}))).into_response();
    };

    let delta_return = a.total_return_pct - b.total_return_pct;
    let delta_drawdown = a.max_drawdown_pct - b.max_drawdown_pct;
    let delta_trades = a.trades as i64 - b.trades as i64;
    let delta_win_rate = a.win_rate - b.win_rate;

    (StatusCode::OK, Json(serde_json::json!({
        "a": {
            "run_id": a.run_id,
            "total_return_pct": a.total_return_pct,
            "max_drawdown_pct": a.max_drawdown_pct,
            "trades": a.trades,
            "win_rate": a.win_rate,
            "pnl": a.pnl
        },
        "b": {
            "run_id": b.run_id,
            "total_return_pct": b.total_return_pct,
            "max_drawdown_pct": b.max_drawdown_pct,
            "trades": b.trades,
            "win_rate": b.win_rate,
            "pnl": b.pnl
        },
        "delta": {
            "total_return_pct": delta_return,
            "max_drawdown_pct": delta_drawdown,
            "trades": delta_trades,
            "win_rate": delta_win_rate
        }
    }))).into_response()
}
```

Add `pt-strategy-lab = { path = "../pt-strategy-lab" }` to `crates/pt-dashboard/Cargo.toml` if not already present, and add `use pt_strategy_lab::StrategyRunReport;` to the imports.

- [ ] **Step 4: Register route**

```rust
.route("/api/v1/artifacts/compare", get(get_artifacts_compare))
```

- [ ] **Step 5: Add comparison UI**

In the frontend backtest/artifacts section, add a "Compare" button that opens a side-by-side panel. When two run IDs are selected, fetch `/api/v1/artifacts/compare?a=RUN_A&b=RUN_B` and display the response metrics in a two-column layout with delta indicators (green if improvement, red if regression).

The UI must have no "Promote" or execution button in this panel — it is read-only.

- [ ] **Step 6: Build and commit**

```bash
cargo build -p pt-dashboard 2>&1 | grep "^error"
cd crates/pt-dashboard && pnpm run build 2>&1 | tail -5
git add crates/pt-dashboard/src/lib.rs crates/pt-dashboard/src/
git commit -m "feat(dashboard): artifact comparison endpoint + frontend panel"
```

---

## Task 15: Operator Strategy Promotion Workspace (Group F)

**Files:**
- Modify: `crates/pt-dashboard/src/lib.rs`
- Modify: `crates/pt-dashboard/src/` (frontend)

- [ ] **Step 1: Add workspace state to DashboardHandles**

In `crates/pt-dashboard/src/lib.rs`, add types:

```rust
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize)]
pub struct GateStatus {
    pub min_trades_ok: bool,
    pub sharpe_ok: bool,
    pub drawdown_ok: bool,
    pub no_conflict_ok: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct WorkspaceCandidate {
    pub run_id: String,
    pub sharpe: f64,
    pub max_drawdown: f64,
    pub trade_count: u32,
    pub gates: GateStatus,
}
```

Add to `DashboardHandles`:
```rust
pub workspace_tokens: Arc<parking_lot::Mutex<HashMap<String, std::time::Instant>>>,
pub workspace_promoted: Arc<parking_lot::RwLock<Vec<String>>>,
```

Initialize in `DashboardHandles::default()`:
```rust
workspace_tokens: Arc::new(parking_lot::Mutex::new(HashMap::new())),
workspace_promoted: Arc::new(parking_lot::RwLock::new(Vec::new())),
```

Copy through in `DashboardState::new`.

- [ ] **Step 2: Note on gate check**

`StrategyRunReport` actual fields: `trades: usize`, `max_drawdown_pct: f64` (already in percentage, e.g. `15.0` = 15%), `win_rate: f64`, `total_return_pct: f64`. No sharpe field — use `objective_breakdown.final_score` from `StrategyCandidateReviewView` as the score proxy. The gate check helper is inlined into the endpoint handlers (Step 3) since each handler needs slightly different data sources.

- [ ] **Step 3: Implement workspace endpoints**

`StrategyCandidateReviewView` actual fields: `rank`, `score: f64`, `objective_breakdown.drawdown_penalty: f64`, `promotion_gate.status: String`, `source_report_path: Option<String>`. There is no direct `run_id` field; use `source_report_path` as the stable identifier, or derive a rank-based ID. Load `StrategyRunReport` from `data/backtest/{source_run_id}.json` to get `trades` count.

```rust
// GET /api/v1/workspace/candidates
async fn get_workspace_candidates(
    State(state): State<DashboardState>,
) -> Json<Vec<WorkspaceCandidate>> {
    let candidates = state.coinbase.strategy_candidates.read();
    let result: Vec<WorkspaceCandidate> = candidates.iter().map(|c| {
        // Try to load the underlying StrategyRunReport for trade count.
        let trades = c.source_report_path.as_deref()
            .and_then(|p| std::fs::read_to_string(p).ok())
            .and_then(|raw| serde_json::from_str::<pt_strategy_lab::StrategyRunReport>(&raw).ok())
            .map(|r| r.trades as u32)
            .unwrap_or(0);
        let drawdown_abs = c.objective_breakdown.drawdown_penalty.abs();
        let score = c.objective_breakdown.final_score;
        let run_id = c.source_report_path.clone()
            .unwrap_or_else(|| format!("rank-{}", c.rank));

        let min_trades_ok = trades >= 30;
        let score_ok = score >= 1.0;
        let drawdown_ok = drawdown_abs <= 0.25;
        let reason = if !min_trades_ok {
            Some(format!("needs ≥30 trades, got {trades}"))
        } else if !score_ok {
            Some(format!("score {score:.2} < 1.0"))
        } else if !drawdown_ok {
            Some(format!("drawdown {:.1}% > 25%", drawdown_abs * 100.0))
        } else {
            None
        };

        WorkspaceCandidate {
            run_id,
            sharpe: score,
            max_drawdown: drawdown_abs,
            trade_count: trades,
            gates: GateStatus { min_trades_ok, sharpe_ok: score_ok, drawdown_ok, no_conflict_ok: true, reason },
        }
    }).collect();
    Json(result)
}

// POST /api/v1/workspace/request-approval  → returns {token: "uuid", expires_in_secs: 600}
async fn post_request_approval(
    State(state): State<DashboardState>,
) -> Json<serde_json::Value> {
    let token = Uuid::new_v4().to_string();
    state.workspace_tokens.lock().insert(token.clone(), std::time::Instant::now());
    Json(serde_json::json!({"token": token, "expires_in_secs": 600}))
}

// POST /api/v1/workspace/promote  body: {run_id, token}
async fn post_promote(
    State(state): State<DashboardState>,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    let run_id = body.get("run_id").and_then(|v| v.as_str()).unwrap_or("");
    let token  = body.get("token").and_then(|v| v.as_str()).unwrap_or("");

    // Validate token (single-use, 10-minute TTL).
    let valid = {
        let mut tokens = state.workspace_tokens.lock();
        if let Some(issued_at) = tokens.remove(token) {
            issued_at.elapsed().as_secs() < 600
        } else { false }
    };
    if !valid {
        return (StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({"error": "invalid or expired approval token"}))).into_response();
    }

    // Load source report and verify gates.
    let report_path = {
        let candidates = state.coinbase.strategy_candidates.read();
        candidates.iter()
            .find(|c| c.source_report_path.as_deref() == Some(run_id)
                   || format!("rank-{}", c.rank) == run_id)
            .and_then(|c| c.source_report_path.clone())
    };
    let Some(path) = report_path else {
        return (StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "run_id not found"}))).into_response();
    };
    let Ok(raw) = std::fs::read_to_string(&path) else {
        return (StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "report file not found"}))).into_response();
    };
    let Ok(report) = serde_json::from_str::<pt_strategy_lab::StrategyRunReport>(&raw) else {
        return (StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "could not parse report"}))).into_response();
    };

    if report.trades < 30 {
        return (StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({"error": format!("gate fail: needs ≥30 trades, got {}", report.trades)}))).into_response();
    }
    // Use max_drawdown_pct which is already a percentage (e.g. 15.0 = 15%).
    if report.max_drawdown_pct.abs() > 25.0 {
        return (StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({"error": format!("gate fail: drawdown {:.1}% > 25%", report.max_drawdown_pct.abs())}))).into_response();
    }

    state.workspace_promoted.write().push(run_id.to_string());
    tracing::info!(run_id, "strategy promoted to paper by operator");
    (StatusCode::OK, Json(serde_json::json!({"status": "promoted", "run_id": run_id, "mode": "paper"}))).into_response()
}
```

Add `uuid.workspace = true` to `crates/pt-dashboard/Cargo.toml` (workspace already has `uuid`).
Add `pt-strategy-lab = { path = "../pt-strategy-lab" }` to `crates/pt-dashboard/Cargo.toml` if not already present.

Register routes in `router()`:
```rust
.route("/api/v1/workspace/candidates", get(get_workspace_candidates))
.route("/api/v1/workspace/request-approval", post(post_request_approval))
.route("/api/v1/workspace/promote", post(post_promote))
```

- [ ] **Step 4: Add Workspace tab to frontend**

In the frontend, add a "Workspace" tab with four panels:

1. **Candidates** — fetch `GET /api/v1/workspace/candidates`, render table with columns: run_id, sharpe, max_drawdown, trade_count, gate status (✓/✗ per gate). Sortable.

2. **Gate Detail** — click a row to expand inline: show per-gate reason text when failed.

3. **Comparison** — "Compare" button on any two selected rows calls the comparison endpoint (Task 14) and renders a two-column metric diff.

4. **Promotion** — below the table:
   - "Request Approval" button → POST `/api/v1/workspace/request-approval` → stores token in component state, enables "Promote to Paper" for 10 min.
   - "Promote to Paper" button (disabled until token active) → POST `/api/v1/workspace/promote` with `{run_id, token}`.
   - On success: show green banner "Promoted to paper mode". No "Go Live" button anywhere.

- [ ] **Step 5: Build**

```bash
cargo build -p pt-dashboard 2>&1 | grep "^error"
cd crates/pt-dashboard && pnpm run build 2>&1 | tail -5
```

- [ ] **Step 6: Run full test suite**

```bash
cargo test --workspace 2>&1 | tail -20
```
Expected: all tests pass.

- [ ] **Step 7: Commit**

```bash
git add crates/pt-dashboard/src/lib.rs crates/pt-dashboard/src/
git commit -m "feat(workspace): operator strategy promotion API + Workspace frontend tab"
```

---

## Final validation

- [ ] **Full workspace build**

```bash
cargo build --workspace 2>&1 | grep "^error"
```

- [ ] **Full test suite**

```bash
cargo test --workspace 2>&1 | grep -E "^(test result|FAILED|error)" | head -20
```

- [ ] **Clippy**

```bash
cargo clippy --workspace -- -D warnings 2>&1 | grep "^error" | head -10
```

- [ ] **Frontend build**

```bash
cd crates/pt-dashboard && pnpm run build 2>&1 | tail -5
```

- [ ] **shellcheck all scripts**

```bash
shellcheck scripts/pi_lib.sh scripts/pi_dev_tailscale_setup.sh \
  scripts/pi_dev_cloudflare.sh scripts/webhook_dual_write_test.sh 2>&1 | head -20
```
