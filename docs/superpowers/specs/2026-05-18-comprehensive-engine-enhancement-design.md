# Comprehensive Engine Enhancement Design

**Date:** 2026-05-18  
**Status:** Approved

---

## Security Constraints (Non-negotiable, verbatim)

- Do not modify or inject real credentials.
- No repo-tracked secrets are introduced.
- Keep sandbox/paper-only guardrails.
- Do not enable uncontrolled provider usage.
- Approval flow remains human-gated.
- Do not let the UI imply automatic execution.
- Do not expose secrets in telemetry.
- Secrets stay out of the repo: rsync exclude list drops `.env*`, `config/config.toml`, credential JSON files.
- Do not raise `engine.mode` to live from this skill.
- Keep AI actions advisory and bounded from the start.
- Do not enable live mode.
- Do not add or modify credentials.
- Do not raise risk caps.
- Do not let AI bypass risk controls.
- Do not treat chart-derived fixture bars as replay or paper evidence.

---

## Group A — Live Data Pipeline

### A1: PolymarketWsClient (`pt-polymarket/src/ws.rs`)

**What it does:** Subscribes to the Polymarket CLOB WebSocket book channel and broadcasts `BookUpdate` events. Mirrors the pattern in `pt-coinbase/src/ws.rs`.

**Interface:**
```rust
pub struct BookUpdate {
    pub asset_id: String,
    pub best_bid: Option<f64>,
    pub best_ask: Option<f64>,
    pub ts: String,
}

pub struct PolymarketWsClient {
    sender: broadcast::Sender<BookUpdate>,
}

impl PolymarketWsClient {
    pub fn new() -> Self;
    pub fn subscribe(&self) -> broadcast::Receiver<BookUpdate>;
    pub async fn run(&self, asset_ids: Vec<String>);
}
```

**Protocol:** Polymarket CLOB WS (`wss://ws-subscriptions-clob.polymarket.com/ws/market`) — `subscribe` message with `assets_ids` list, receives `book` channel updates. Reconnects with 5s delay on error.

**Internal types:** `WsMarketMsg` → deserialize JSON → filter `event_type == "book"` → map to `BookUpdate`.

**Tests:** Parse a fixture `book` message, ignore non-book types, skip malformed price.

### A2: CoinbasePriceAdapter (`pt-signal/src/cb_price.rs`)

**What it does:** Implements `ExternalSignalAdapter` for Coinbase WS tickers — bridges `CoinbaseWsClient` (already complete) into the signal pipeline.

**Interface:**
```rust
pub struct CoinbasePriceAdapter {
    rx: broadcast::Receiver<WsTicker>,
    buffer: VecDeque<NormalizedExternalSignal>,
}

impl ExternalSignalAdapter for CoinbasePriceAdapter {
    fn source_id(&self) -> &str { "coinbase_price" }
    fn poll(&mut self) -> Vec<NormalizedExternalSignal>;
}
```

`poll()` drains all pending ticks from the broadcast receiver, converts each `WsTicker` to a `NormalizedExternalSignal { source_id, asset_id, value: price, confidence: 0.8, ts }`.

**Tests:** `poll()` returns empty when no messages, returns one signal per tick, maps product_id to asset_id.

### A3: SessionVwapSignal (`pt-strategy-lab/src/signals.rs`)

**What it does:** Wires the existing `session_vwap` indicator (already in `indicators.rs`) into the signal evaluation path. When current price deviates from session VWAP by ≥ threshold (configurable, default 0.5%), emit a directional signal.

**Config additions to `SignalConfig`:**
```rust
pub session_vwap_threshold: f64,     // default 0.005
pub session_start_hour_utc: u32,     // default 0
```

**Signal logic:** `(price - vwap) / vwap` → positive = above vwap (bearish mean-reversion signal), negative = below (bullish). `confidence = deviation.abs().min(1.0)` scaled to threshold multiple.

**Tests:** Above-VWAP produces negative-value signal, below-VWAP produces positive-value, zero-deviation produces no signal.

---

## Group B — Infrastructure Hardening

### B1: Rate-Limit Backoff in `spawn_orderbook_loop` (`pt-engine/src/lib.rs`)

**What it does:** Detects HTTP 429 from Polymarket REST and backs off exponentially before the next poll cycle. Prevents hammering a rate-limited endpoint.

**Design:** Wrap existing REST call in a retry loop. On 429, double the sleep interval (base 1s, max 60s), reset on success. Log each backoff at `warn` level. No config change required initially; add `max_backoff_secs: u64` to engine config later if needed.

**Tests:** Mock HTTP 429 response triggers backoff; successful 200 resets interval.

### B2: dashmap for `latest_books` (`pt-engine/src/lib.rs`)

**What it does:** Replace `Arc<RwLock<HashMap<String, MarketSnapshot>>>` with `Arc<DashMap<String, MarketSnapshot>>` to eliminate writer-starvation on the orderbook update path under concurrent readers.

**Dependency:** Add `dashmap = "6"` to `pt-engine/Cargo.toml`.

**Migration:** All `write().insert(...)` → `insert(...)`, all `read().get(...)` → `get(...)`. No functional change.

**Tests:** Concurrent readers + single writer compile and run without deadlock (trivially verified by existing tests continuing to pass).

### B3: DuckDB Retention (`pt-tsdb/src/lib.rs`)

**What it does:** Adds `prune_older_than_days(table: &str, days: u32)` method to `TsDb`. Called from a periodic task in `pt-engine` to cap unbounded time-series growth.

**SQL:** `DELETE FROM {table} WHERE ts < now() - INTERVAL '{days} days'`

**Config addition:**
```toml
[tsdb]
retention_days = 90
```

**Tests:** Insert rows with past timestamps, call `prune_older_than_days`, verify old rows deleted and recent rows intact.

### B4: `pi_lib.sh` /22 Fix

**What it does:** `_pi_local_subnet24` currently hardcodes a /24 sweep. Fix to derive the actual subnet prefix from the default-route interface's netmask, supporting /22 and other CIDR sizes.

**Approach:** Use `ip route` to find default-route interface → `ip addr show dev <iface>` to get CIDR → compute host range. Fall back to /24 behavior if parsing fails.

**Constraint:** Bash-only, no Python dependency. Cap sweep at 1024 hosts (warn if subnet is larger).

### B5: Tailscale Auto-Reauth (`scripts/pi_dev_tailscale_setup.sh` + Nx target `pi-dev-tailscale-setup`)

**What it does:** Installs a systemd timer on the Pi that runs `tailscale up --auth-key=<key>` on boot and every 12 hours, eliminating manual re-authentication when home IP changes.

**Inputs:** `PI_TAILSCALE_AUTH_KEY` env var (ephemeral key from Tailscale admin panel — not stored in repo).

**Script actions:**
1. SSH to Pi, render `infra/systemd/tailscale-reauth.service.template` and `.timer` with key substituted.
2. `systemctl daemon-reload && enable --now tailscale-reauth.timer`.
3. Verify `tailscale status` shows connected.

**Security:** Auth key passed via env var, never written to a repo-tracked file.

---

## Group C — Operator Workflow

### C1: Artifact Comparison (`/api/v1/artifacts/compare` + frontend panel)

**What it does:** Allows side-by-side comparison of two backtest artifacts (by run ID). Backend returns diff of key metrics; frontend renders a two-column comparison panel.

**Endpoint:** `GET /api/v1/artifacts/compare?a=<run_id>&b=<run_id>`

**Response:**
```json
{
  "a": { "run_id": "...", "sharpe": 1.2, "max_drawdown": 0.15, ... },
  "b": { "run_id": "...", "sharpe": 0.9, "max_drawdown": 0.22, ... },
  "delta": { "sharpe": 0.3, "max_drawdown": -0.07, ... }
}
```

**Frontend:** "Compare" button on artifact list → opens comparison panel. Green/red delta indicators. No auto-promote button — comparison is read-only.

### C2: Cloudflare Tunnel Nx Target (`scripts/pi_dev_cloudflare.sh` + `pi-dev-cloudflare-setup`)

**What it does:** Configures a Cloudflare Tunnel on the Pi for stable remote dashboard access without exposing a port or requiring a static IP.

**Inputs:** `CF_TUNNEL_TOKEN` env var (from Cloudflare Zero Trust dashboard — never repo-tracked).

**Script actions:**
1. SSH to Pi, install `cloudflared` via apt.
2. Run `cloudflared service install <token>` to register the tunnel.
3. Verify tunnel appears as healthy in Cloudflare dashboard (check `cloudflared tunnel info` over SSH).

**Security:** Token passed via env var, never written to a repo-tracked file.

### C3: Lambda Migration Runbook (`docs/runbooks/LAMBDA_MIGRATION.md` + `scripts/webhook_dual_write_test.sh`)

**What it does:** Documents the path from Pi-based deployment to AWS Lambda for the webhook/signal ingestion layer. Includes dual-write test script to validate both endpoints receive identical payloads before cutover.

**Runbook sections:** Prerequisites, dual-write architecture diagram (ASCII), migration steps, rollback procedure, success criteria.

**Test script:** Sends a fixture payload to both Pi endpoint and Lambda endpoint, diffs responses, exits non-zero on divergence.

---

## Group D — Smart Order Placement (new crate `pt-order-advisor`)

### Overview

A new crate with zero side effects — it advises placement mode (`Maker`, `Taker`, `Skip`) and is called by `pt-engine` before `post_quote()`. It never posts orders itself.

### Components

**`MarketContext`** (computed from last 60s WS window):
```rust
pub struct MarketContext {
    pub spread_bps: f64,
    pub tick_velocity: f64,    // adverse ticks per second
    pub signal_direction: i8,  // +1 / -1 / 0
    pub position_pnl: f64,     // unrealized P&L on current position
}
```

**`PlacementMode`:**
```rust
pub enum PlacementMode { Maker, Taker, Skip }
```

**`MakerTakerAdvisor`** (stateless):
- Default: `Maker` (0% fee).
- Escalate to `Taker` if: spread ≤ `taker_threshold_bps` AND `tick_velocity` ≥ `velocity_threshold`.
- `Skip` if spread ≥ `skip_threshold_bps`.

**`ContextualAdvisor`** — wraps `MakerTakerAdvisor`, adding:
- No-position default: `Maker`.
- In-position: delegate to `MakerTakerAdvisor`.

**`ProfitProtectionMonitor`** (stateful, two-gate):
- Gate 1 (WS velocity armed): N consecutive adverse ticks within rolling window arms the monitor. `N` and window configurable.
- Gate 2 (signal flip confirms): When armed, if composite signal flips against position, recommend escalation to `Taker` for exit.
- Both gates must be open to fire. Reset after exit or after window expires.

**Config (`[order_advisor]` in `config.toml`):**
```toml
taker_threshold_bps = 5.0
velocity_threshold = 0.5
skip_threshold_bps = 50.0
protection_adverse_ticks = 3
protection_window_secs = 60
```

**Adversarial notes:**
- `ProfitProtectionMonitor` is advisory only — `pt-engine` may ignore the recommendation; risk controls in `pt-risk` remain authoritative.
- `MarketContext.position_pnl` is unrealized; no realized P&L is used for gating.
- No automatic stop-loss is introduced. Human approval flow unchanged.

**Tests:** 15+ unit tests covering each gate independently, combined two-gate logic, config edge cases, and `PlacementMode` defaulting.

---

## Group E — Live WS Chart Candle Creation

### CandleAggregator (`pt-engine/src/candle_agg.rs`)

**What it does:** Consumes a stream of `BookUpdate` / `WsTicker` ticks and produces OHLCV candles at a configurable granularity (default 60s).

```rust
pub struct Candle {
    pub asset_id: String,
    pub open: f64, pub high: f64, pub low: f64, pub close: f64,
    pub volume: f64,
    pub ts_open_ms: i64,
    pub ts_close_ms: i64,
}

pub struct CandleAggregator {
    granularity_ms: i64,
    buckets: HashMap<String, InProgressCandle>,
}

impl CandleAggregator {
    pub fn ingest(&mut self, price: f64, volume: f64, asset_id: &str, ts_ms: i64) -> Option<Candle>;
}
```

`ingest()` returns `Some(Candle)` when the current tick crosses a bucket boundary (i.e., the previous candle is complete).

**Tests:** Single-bucket flush, multi-tick OHLCV calculation, boundary crossing emits exactly one candle.

### SSE Endpoint (`/api/v1/charts/candles/live`)

**What it does:** Streams completed candles as Server-Sent Events. One SSE stream per connected client; uses `tokio::sync::broadcast` to fan out from `CandleAggregator`.

**Event format:**
```
event: candle
data: {"asset_id":"...", "open":..., "high":..., "low":..., "close":..., "volume":..., "ts_open_ms":..., "ts_close_ms":...}
```

**Query params:** `?asset_id=<id>&granularity=60` (granularity in seconds).

### Frontend Integration

**What it does:** Replaces the static historical chart fixture with a live EventSource connection. When SSE is available, append incoming candles to the chart dataset. Fall back to REST historical data on connection error.

**Implementation:** `EventSource('/api/v1/charts/candles/live?asset_id=...')` in the existing chart component. On `message`, push candle to chart data array and call `chart.update()`. On `error`, log and fall back to polling REST.

**Security note:** Chart displays data only — no trade action is triggered by chart updates.

---

## Group F — Operator Strategy Promotion Workspace

### Backend (`/api/v1/workspace/*`)

**Endpoints:**
- `GET /api/v1/workspace/candidates` — list runs eligible for promotion (passed all gates).
- `POST /api/v1/workspace/promote` — promote a run to paper; returns 422 with reason if gates not met.
- `GET /api/v1/workspace/gates/:run_id` — return gate status for a specific run.
- `POST /api/v1/workspace/paper-activate` — activate promoted strategy in paper mode; requires prior `promote` success.

**Gate criteria (server-enforced, 422 if any fail):**
1. Minimum 30 backtest trades.
2. Sharpe ratio ≥ 1.0.
3. Max drawdown ≤ 25%.
4. No open positions in conflicting strategies.
5. Human approval token present in request (UUID issued by `/api/v1/workspace/request-approval`).

**Approval flow:**
1. Operator calls `POST /api/v1/workspace/request-approval` → server emits a one-time token (stored server-side, TTL 10 minutes).
2. Operator reviews gate status panel.
3. Operator submits `POST /api/v1/workspace/promote` with token.
4. Server validates token + all gates, promotes or returns 422.

**Security:** Approval token is single-use, TTL-bound. No auto-promotion path exists. `paper-activate` always sets `engine.mode = paper`, never `live`.

### Frontend (Workspace Tab)

**Four panels:**

1. **Candidates Panel** — table of runs with gate status indicators (✓/✗ per gate). Sortable by Sharpe, drawdown, trade count.
2. **Gate Detail Panel** — click any run → expand gate breakdown with pass/fail reason per criterion.
3. **Comparison Panel** — reuses C1 artifact comparison widget for side-by-side metric diff.
4. **Promotion Panel** — "Request Approval" button → token issued → "Promote to Paper" button enabled for 10 minutes → submit.

**UI constraints:**
- "Promote to Paper" button disabled until approval token is active.
- No "Go Live" button anywhere in the UI.
- All promotion actions log operator identity (from session) to audit log.

---

## Data Flow Summary

```
Polymarket CLOB WS ──► PolymarketWsClient (A1) ──► CandleAggregator (E) ──► SSE ──► Frontend Chart
                                                 └─► latest_books (dashmap, B2) ──► Engine
Coinbase WS ──────────► CoinbasePriceAdapter (A2) ──► ExternalSignalAdapter pipeline
                                                    └─► SessionVwapSignal (A3)

Engine poll loop ──► RateLimit backoff (B1)
MarketContext ◄──── last 60s WS window
MarketContext ──► MakerTakerAdvisor ──► PlacementMode ──► Engine (advisory only)
               └──► ProfitProtectionMonitor (two-gate) ──► PlacementMode (advisory)

Operator ──► /api/v1/workspace/* ──► Gate checks ──► Approval token ──► Promote to Paper
```

---

## Implementation Groups and Parallelism

Groups can be executed in parallel with the following dependency notes:

- **A, B, C** are fully independent.
- **D** has no code dependencies on A–C but benefits from B2 (dashmap) being done first for clean `MarketContext` extraction.
- **E** depends on A1 (`PolymarketWsClient`) being available as a tick source; can stub with `CoinbaseWsClient` initially.
- **F** is fully independent of A–E at the API level; frontend comparison panel reuses C1 work.

Suggested parallel batches:
1. `A1 + A2 + A3 + B1 + B2 + B3 + B4 + B5` (all infrastructure, no cross-deps)
2. `C1 + C2 + C3 + D` (operator workflow + order advisor)
3. `E + F` (chart pipeline + workspace tab)

---

## Testing Philosophy

- All new Rust code: unit tests within the module, integration tests in `tests/` where network I/O is mocked.
- `pt-order-advisor`: pure logic, no I/O — 100% unit-testable, target ≥ 15 tests.
- SSE endpoint: integration test with a real `tokio` runtime, mock tick source.
- Workspace API: integration tests covering the happy path and each 422 failure mode.
- Pine scripts: manual TradingView validation only (no automated test runner).
- Shell scripts: `shellcheck` clean; manual smoke tests on Pi.
