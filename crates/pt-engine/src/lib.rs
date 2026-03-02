use arrow_array::{ArrayRef, Float64Array, RecordBatch, StringArray, TimestampMillisecondArray};
use arrow_schema::{DataType, Field, Schema, TimeUnit};
use axum::{extract::State, http::HeaderMap, response::IntoResponse, routing::post, Router};
use chrono::{DateTime, Utc};
use parking_lot::{Mutex, RwLock};
use parquet::arrow::ArrowWriter;
use pt_coinbase::{
    CoinbaseAuthManager, CoinbaseOrderSummary, CoinbaseSpotHedger, CoinbaseTopOfBook,
    CoinbaseTransactionSummary, CoinbaseWalletClient, CoinbaseWsEvent, CoinbaseWsRunConfig,
    HedgeExecutor, HedgeIntent, PaperCoinbaseHedger,
};
use pt_core::{
    AllocationDrift, AppConfig, ApprovalToken, Asset, CoinbaseL2Update, CoinbaseOrderBookState,
    EngineMode, ExecutionCostAttribution, ExecutionEvent, ExecutionMode, ExecutionPolicy,
    ExecutionReport, KillSwitchState, MarketHistoryPoint, MarketSelection, MarketSnapshot,
    MetricsRegistry, OrderLifecycleState, PtError, PtResult, RebalanceIntent, RebalancePlan,
    RebalancePlanStatus, RiskState, RouteExecutionPlan, RouteOpportunity, Side, TradingViewBias,
    UserOrderEvent, Venue, VenueCapability, WalletBalance, WalletModeConfig, WalletSignal,
};
use pt_dashboard::{router as dashboard_router, CoinbaseAuthController, DashboardState};
use pt_market_discovery::MarketDiscoveryClient;
use pt_order_manager::{
    OrderManager, OrderManagerConfig as RepriceManagerConfig, RestingOrder, TopOfBook,
};
use pt_polymarket::{
    LivePolymarketConfig, LivePolymarketExecutor, PaperPolymarketExecutor, PolymarketClient,
    PolymarketExecution,
};
use pt_quote::{
    build_quote_intent, default_fee_bps_for_venue, estimate_execution_cost, vector_gate,
    CostInputs, QuoteConfig,
};
use pt_replay::{load_replay_frames, PaperSimulator};
use pt_risk::RiskEngine;
use pt_route::{find_route_opportunities, RouteBook};
use pt_signal::{parse_tradingview_bias, SignalFusionEngine};
use pt_wallet_intel::WalletIntelClient;
use rusqlite::{params, Connection};
use std::{
    collections::HashMap,
    fs,
    net::SocketAddr,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::task::JoinHandle;
use tracing::{error, info, warn};

#[derive(Clone)]
struct SharedState {
    selected_markets: Arc<RwLock<Vec<MarketSelection>>>,
    latest_books: Arc<RwLock<HashMap<String, MarketSnapshot>>>,
    market_history: Arc<RwLock<HashMap<String, Vec<MarketHistoryPoint>>>>,
    recent_executions: Arc<RwLock<Vec<ExecutionReport>>>,
    execution_events: Arc<RwLock<Vec<ExecutionEvent>>>,
    execution_costs: Arc<RwLock<Vec<ExecutionCostAttribution>>>,
    execution_policy: Arc<RwLock<ExecutionPolicy>>,
    fused_bias: Arc<RwLock<HashMap<Asset, f64>>>,
    tv_bias: Arc<RwLock<Option<TradingViewBias>>>,
    risk_state: Arc<RwLock<RiskState>>,
    kill_switch: Arc<RwLock<KillSwitchState>>,
    inventory_usd: Arc<RwLock<f64>>,
    wallet_balances: Arc<RwLock<Vec<WalletBalance>>>,
    wallet_open_orders: Arc<RwLock<Vec<CoinbaseOrderSummary>>>,
    coinbase_orderbooks: Arc<RwLock<HashMap<String, CoinbaseOrderBookState>>>,
    coinbase_user_events: Arc<RwLock<Vec<UserOrderEvent>>>,
    route_opportunities: Arc<RwLock<Vec<RouteOpportunity>>>,
    route_executions: Arc<RwLock<Vec<RouteExecutionPlan>>>,
    venue_capabilities: Arc<RwLock<Vec<VenueCapability>>>,
    coinbase_fee_summary: Arc<RwLock<Option<CoinbaseTransactionSummary>>>,
    wallet_drifts: Arc<RwLock<Vec<AllocationDrift>>>,
    rebalance_plan: Arc<RwLock<Option<RebalancePlan>>>,
    rebalance_approval: Arc<RwLock<Option<ApprovalToken>>>,
    force_unwind: Arc<RwLock<bool>>,
}

impl SharedState {
    fn new(policy: ExecutionPolicy) -> Self {
        Self {
            selected_markets: Arc::new(RwLock::new(Vec::new())),
            latest_books: Arc::new(RwLock::new(HashMap::new())),
            market_history: Arc::new(RwLock::new(HashMap::new())),
            recent_executions: Arc::new(RwLock::new(Vec::new())),
            execution_events: Arc::new(RwLock::new(Vec::new())),
            execution_costs: Arc::new(RwLock::new(Vec::new())),
            execution_policy: Arc::new(RwLock::new(policy)),
            fused_bias: Arc::new(RwLock::new(HashMap::new())),
            tv_bias: Arc::new(RwLock::new(None)),
            risk_state: Arc::new(RwLock::new(RiskState::default())),
            kill_switch: Arc::new(RwLock::new(KillSwitchState::Running)),
            inventory_usd: Arc::new(RwLock::new(0.0)),
            wallet_balances: Arc::new(RwLock::new(Vec::new())),
            wallet_open_orders: Arc::new(RwLock::new(Vec::new())),
            coinbase_orderbooks: Arc::new(RwLock::new(HashMap::new())),
            coinbase_user_events: Arc::new(RwLock::new(Vec::new())),
            route_opportunities: Arc::new(RwLock::new(Vec::new())),
            route_executions: Arc::new(RwLock::new(Vec::new())),
            venue_capabilities: Arc::new(RwLock::new(Vec::new())),
            coinbase_fee_summary: Arc::new(RwLock::new(None)),
            wallet_drifts: Arc::new(RwLock::new(Vec::new())),
            rebalance_plan: Arc::new(RwLock::new(None)),
            rebalance_approval: Arc::new(RwLock::new(None)),
            force_unwind: Arc::new(RwLock::new(false)),
        }
    }
}

struct Storage {
    conn: Mutex<Connection>,
    portfolio_id: String,
    snapshot_roll_secs: u64,
    parquet_dir: String,
    snapshot_buffer: RwLock<Vec<MarketSnapshot>>,
    last_roll_ms: RwLock<i64>,
}

impl Storage {
    fn new(
        sqlite_path: &str,
        parquet_dir: &str,
        snapshot_roll_secs: u64,
        portfolio_id: &str,
    ) -> PtResult<Self> {
        if let Some(parent) = Path::new(sqlite_path).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|e| PtError::Io(e.to_string()))?;
            }
        }
        fs::create_dir_all(parquet_dir).map_err(|e| PtError::Io(e.to_string()))?;

        let conn = Connection::open(sqlite_path).map_err(|e| PtError::Io(e.to_string()))?;
        conn.pragma_update(None, "journal_mode", "WAL")
            .map_err(|e| PtError::Io(e.to_string()))?;

        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS market_snapshots (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts_ms INTEGER NOT NULL,
                market_id TEXT NOT NULL,
                token_id TEXT NOT NULL,
                bid REAL NOT NULL,
                ask REAL NOT NULL,
                spread REAL NOT NULL,
                liquidity REAL NOT NULL
            );
            CREATE TABLE IF NOT EXISTS execution_reports (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts_ms INTEGER NOT NULL,
                venue TEXT NOT NULL,
                order_id TEXT NOT NULL,
                market_id TEXT,
                side TEXT NOT NULL,
                status TEXT NOT NULL,
                filled_qty REAL NOT NULL,
                avg_px REAL NOT NULL,
                details TEXT
            );
            CREATE TABLE IF NOT EXISTS risk_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts_ms INTEGER NOT NULL,
                payload TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS coinbase_balances (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts_ms INTEGER NOT NULL,
                venue TEXT NOT NULL,
                account_id TEXT NOT NULL,
                asset TEXT NOT NULL,
                available REAL NOT NULL,
                hold REAL NOT NULL,
                usd_value REAL NOT NULL
            );
            CREATE TABLE IF NOT EXISTS coinbase_orders (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts_ms INTEGER NOT NULL,
                order_id TEXT NOT NULL,
                product_id TEXT NOT NULL,
                side TEXT NOT NULL,
                status TEXT NOT NULL,
                order_type TEXT NOT NULL,
                average_filled_price REAL NOT NULL,
                filled_size REAL NOT NULL
            );
            CREATE TABLE IF NOT EXISTS rebalance_plans (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts_ms INTEGER NOT NULL,
                plan_id TEXT NOT NULL,
                status TEXT NOT NULL,
                payload TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS rebalance_actions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts_ms INTEGER NOT NULL,
                plan_id TEXT NOT NULL,
                action TEXT NOT NULL,
                payload TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS execution_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts_ms INTEGER NOT NULL,
                order_id TEXT NOT NULL,
                venue TEXT NOT NULL,
                market_id TEXT,
                product_id TEXT,
                side TEXT NOT NULL,
                state TEXT NOT NULL,
                qty REAL NOT NULL,
                price REAL NOT NULL,
                details TEXT
            );
            CREATE TABLE IF NOT EXISTS execution_costs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts_ms INTEGER NOT NULL,
                execution_id TEXT NOT NULL,
                venue TEXT NOT NULL,
                market_id TEXT,
                side TEXT NOT NULL,
                qty REAL NOT NULL,
                avg_px REAL NOT NULL,
                reference_px REAL NOT NULL,
                fee_bps REAL NOT NULL,
                fee_est REAL NOT NULL,
                slippage_bps REAL NOT NULL,
                slippage_est REAL NOT NULL,
                rebate_bps_est REAL NOT NULL,
                rebate_est REAL NOT NULL,
                effective_edge REAL NOT NULL,
                strategy_class TEXT,
                route_id TEXT
            );
            CREATE TABLE IF NOT EXISTS replay_acceptance_reports (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts_ms INTEGER NOT NULL,
                artifact_path TEXT NOT NULL,
                passed INTEGER NOT NULL,
                fail_reasons TEXT NOT NULL,
                total_reports INTEGER NOT NULL,
                reject_error_rate REAL NOT NULL,
                max_unhedged_delta REAL NOT NULL,
                killswitch TEXT NOT NULL,
                effective_fee_bps_avg REAL NOT NULL,
                payload TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS coinbase_l2_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts_ms INTEGER NOT NULL,
                product_id TEXT NOT NULL,
                sequence_num INTEGER NOT NULL,
                side TEXT NOT NULL,
                price_level REAL NOT NULL,
                new_quantity REAL NOT NULL,
                event_time_ms INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS coinbase_user_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts_ms INTEGER NOT NULL,
                order_id TEXT NOT NULL,
                product_id TEXT NOT NULL,
                status TEXT NOT NULL,
                side TEXT NOT NULL,
                post_only INTEGER NOT NULL,
                avg_price REAL NOT NULL,
                filled_qty REAL NOT NULL,
                total_fees REAL NOT NULL,
                payload TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS order_manager_transitions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts_ms INTEGER NOT NULL,
                market_id TEXT,
                order_id TEXT,
                action TEXT NOT NULL,
                reason TEXT NOT NULL,
                target_price REAL NOT NULL,
                target_size REAL NOT NULL
            );
            CREATE TABLE IF NOT EXISTS route_opportunities (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts_ms INTEGER NOT NULL,
                route_id TEXT NOT NULL,
                strategy_class TEXT NOT NULL,
                gross_edge_bps REAL NOT NULL,
                expected_net_bps REAL NOT NULL,
                expected_usd_profit REAL NOT NULL,
                capital_required_usd REAL NOT NULL,
                payload TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS route_executions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts_ms INTEGER NOT NULL,
                route_id TEXT NOT NULL,
                approved INTEGER NOT NULL,
                reason TEXT,
                payload TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS fee_tier_snapshots (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts_ms INTEGER NOT NULL,
                maker_fee_rate TEXT,
                taker_fee_rate TEXT,
                total_fees REAL NOT NULL,
                payload TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS auth_key_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts_ms INTEGER NOT NULL,
                portfolio_id TEXT NOT NULL,
                venue TEXT NOT NULL,
                action TEXT NOT NULL,
                profile_id TEXT,
                key_id_suffix TEXT,
                source TEXT,
                status TEXT NOT NULL,
                reason TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS wallet_intel_snapshots (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ts_ms INTEGER NOT NULL,
                portfolio_id TEXT NOT NULL,
                asset TEXT NOT NULL,
                horizon TEXT NOT NULL,
                bias REAL NOT NULL,
                confidence REAL NOT NULL
            );
            ",
        )
        .map_err(|e| PtError::Io(e.to_string()))?;
        ensure_sqlite_column(&conn, "execution_costs", "strategy_class", "TEXT")?;
        ensure_sqlite_column(&conn, "execution_costs", "route_id", "TEXT")?;
        ensure_sqlite_column(&conn, "execution_reports", "portfolio_id", "TEXT")?;
        ensure_sqlite_column(&conn, "coinbase_balances", "portfolio_id", "TEXT")?;
        ensure_sqlite_column(&conn, "coinbase_orders", "portfolio_id", "TEXT")?;
        ensure_sqlite_column(&conn, "rebalance_plans", "portfolio_id", "TEXT")?;
        ensure_sqlite_column(&conn, "rebalance_actions", "portfolio_id", "TEXT")?;
        ensure_sqlite_column(&conn, "execution_events", "portfolio_id", "TEXT")?;
        ensure_sqlite_column(&conn, "execution_costs", "portfolio_id", "TEXT")?;
        ensure_sqlite_column(&conn, "coinbase_l2_events", "portfolio_id", "TEXT")?;
        ensure_sqlite_column(&conn, "coinbase_user_events", "portfolio_id", "TEXT")?;
        ensure_sqlite_column(&conn, "order_manager_transitions", "portfolio_id", "TEXT")?;
        ensure_sqlite_column(&conn, "route_opportunities", "portfolio_id", "TEXT")?;
        ensure_sqlite_column(&conn, "route_executions", "portfolio_id", "TEXT")?;
        ensure_sqlite_column(&conn, "fee_tier_snapshots", "portfolio_id", "TEXT")?;
        ensure_sqlite_column(&conn, "auth_key_events", "portfolio_id", "TEXT")?;
        ensure_sqlite_column(&conn, "wallet_intel_snapshots", "portfolio_id", "TEXT")?;

        Ok(Self {
            conn: Mutex::new(conn),
            portfolio_id: portfolio_id.to_string(),
            snapshot_roll_secs,
            parquet_dir: parquet_dir.to_string(),
            snapshot_buffer: RwLock::new(Vec::new()),
            last_roll_ms: RwLock::new(Utc::now().timestamp_millis()),
        })
    }

    fn insert_snapshot(&self, snap: &MarketSnapshot) -> PtResult<()> {
        self.conn
            .lock()
            .execute(
                "INSERT INTO market_snapshots (ts_ms, market_id, token_id, bid, ask, spread, liquidity) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    snap.ts.timestamp_millis(),
                    snap.market_id,
                    snap.token_id,
                    snap.bid,
                    snap.ask,
                    snap.spread,
                    snap.liquidity,
                ],
            )
            .map_err(|e| PtError::Io(e.to_string()))?;

        self.snapshot_buffer.write().push(snap.clone());
        Ok(())
    }

    fn insert_execution_report(&self, report: &ExecutionReport) -> PtResult<()> {
        self.conn
            .lock()
            .execute(
                "INSERT INTO execution_reports (ts_ms, portfolio_id, venue, order_id, market_id, side, status, filled_qty, avg_px, details) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    report.ts.timestamp_millis(),
                    &self.portfolio_id,
                    format!("{:?}", report.venue),
                    report.order_id,
                    report.market_id,
                    format!("{:?}", report.side),
                    format!("{:?}", report.status),
                    report.filled_qty,
                    report.avg_px,
                    report.details,
                ],
            )
            .map_err(|e| PtError::Io(e.to_string()))?;
        Ok(())
    }

    fn insert_risk_state(&self, risk: &RiskState) -> PtResult<()> {
        let payload = serde_json::to_string(risk).map_err(|e| PtError::Serde(e.to_string()))?;
        self.conn
            .lock()
            .execute(
                "INSERT INTO risk_events (ts_ms, payload) VALUES (?1, ?2)",
                params![Utc::now().timestamp_millis(), payload],
            )
            .map_err(|e| PtError::Io(e.to_string()))?;
        Ok(())
    }

    fn insert_wallet_balances(&self, balances: &[WalletBalance]) -> PtResult<()> {
        let conn = self.conn.lock();
        let tx = conn
            .unchecked_transaction()
            .map_err(|e| PtError::Io(e.to_string()))?;
        tx.execute("DELETE FROM coinbase_balances", [])
            .map_err(|e| PtError::Io(e.to_string()))?;
        for b in balances {
            tx.execute(
                "INSERT INTO coinbase_balances (ts_ms, portfolio_id, venue, account_id, asset, available, hold, usd_value) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    b.ts.timestamp_millis(),
                    &self.portfolio_id,
                    format!("{:?}", b.venue),
                    b.account_id,
                    b.asset,
                    b.available,
                    b.hold,
                    b.usd_value
                ],
            )
            .map_err(|e| PtError::Io(e.to_string()))?;
        }
        tx.commit().map_err(|e| PtError::Io(e.to_string()))?;
        Ok(())
    }

    fn insert_coinbase_orders(&self, orders: &[CoinbaseOrderSummary]) -> PtResult<()> {
        let conn = self.conn.lock();
        let tx = conn
            .unchecked_transaction()
            .map_err(|e| PtError::Io(e.to_string()))?;
        tx.execute("DELETE FROM coinbase_orders", [])
            .map_err(|e| PtError::Io(e.to_string()))?;
        for o in orders {
            tx.execute(
                "INSERT INTO coinbase_orders (ts_ms, portfolio_id, order_id, product_id, side, status, order_type, average_filled_price, filled_size) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    Utc::now().timestamp_millis(),
                    &self.portfolio_id,
                    o.order_id,
                    o.product_id,
                    o.side,
                    o.status,
                    o.order_type,
                    o.average_filled_price.parse::<f64>().unwrap_or(0.0),
                    o.filled_size.parse::<f64>().unwrap_or(0.0)
                ],
            )
            .map_err(|e| PtError::Io(e.to_string()))?;
        }
        tx.commit().map_err(|e| PtError::Io(e.to_string()))?;
        Ok(())
    }

    fn insert_rebalance_plan(&self, plan: &RebalancePlan) -> PtResult<()> {
        let payload = serde_json::to_string(plan).map_err(|e| PtError::Serde(e.to_string()))?;
        self.conn
            .lock()
            .execute(
                "INSERT INTO rebalance_plans (ts_ms, portfolio_id, plan_id, status, payload) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    plan.created_ts.timestamp_millis(),
                    &self.portfolio_id,
                    plan.plan_id,
                    format!("{:?}", plan.status),
                    payload
                ],
            )
            .map_err(|e| PtError::Io(e.to_string()))?;
        Ok(())
    }

    fn insert_rebalance_action(&self, plan_id: &str, action: &str, payload: &str) -> PtResult<()> {
        self.conn
            .lock()
            .execute(
                "INSERT INTO rebalance_actions (ts_ms, portfolio_id, plan_id, action, payload) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    Utc::now().timestamp_millis(),
                    &self.portfolio_id,
                    plan_id,
                    action,
                    payload
                ],
            )
            .map_err(|e| PtError::Io(e.to_string()))?;
        Ok(())
    }

    fn insert_execution_event(&self, event: &ExecutionEvent) -> PtResult<()> {
        self.conn
            .lock()
            .execute(
                "INSERT INTO execution_events (ts_ms, portfolio_id, order_id, venue, market_id, product_id, side, state, qty, price, details) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    event.ts.timestamp_millis(),
                    &self.portfolio_id,
                    event.order_id,
                    format!("{:?}", event.venue),
                    event.market_id,
                    event.product_id,
                    format!("{:?}", event.side),
                    format!("{:?}", event.state),
                    event.qty,
                    event.price,
                    event.details,
                ],
            )
            .map_err(|e| PtError::Io(e.to_string()))?;
        Ok(())
    }

    fn insert_execution_cost(&self, cost: &ExecutionCostAttribution) -> PtResult<()> {
        self.conn
            .lock()
            .execute(
                "INSERT INTO execution_costs (ts_ms, portfolio_id, execution_id, venue, market_id, side, qty, avg_px, reference_px, fee_bps, fee_est, slippage_bps, slippage_est, rebate_bps_est, rebate_est, effective_edge, strategy_class, route_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
                params![
                    cost.ts.timestamp_millis(),
                    &self.portfolio_id,
                    cost.execution_id,
                    format!("{:?}", cost.venue),
                    cost.market_id,
                    format!("{:?}", cost.side),
                    cost.qty,
                    cost.avg_px,
                    cost.reference_px,
                    cost.fee_bps,
                    cost.fee_est,
                    cost.slippage_bps,
                    cost.slippage_est,
                    cost.rebate_bps_est,
                    cost.rebate_est,
                    cost.effective_edge,
                    cost.strategy_class.as_ref().map(|v| format!("{:?}", v)),
                    cost.route_id,
                ],
            )
            .map_err(|e| PtError::Io(e.to_string()))?;
        Ok(())
    }

    fn insert_coinbase_l2_event(&self, update: &CoinbaseL2Update) -> PtResult<()> {
        self.conn
            .lock()
            .execute(
                "INSERT INTO coinbase_l2_events (ts_ms, portfolio_id, product_id, sequence_num, side, price_level, new_quantity, event_time_ms) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    Utc::now().timestamp_millis(),
                    &self.portfolio_id,
                    update.product_id,
                    update.sequence_num,
                    update.side,
                    update.price_level,
                    update.new_quantity,
                    update.event_time.timestamp_millis(),
                ],
            )
            .map_err(|e| PtError::Io(e.to_string()))?;
        Ok(())
    }

    fn insert_coinbase_user_event(&self, event: &UserOrderEvent) -> PtResult<()> {
        let payload = serde_json::to_string(event).map_err(|e| PtError::Serde(e.to_string()))?;
        self.conn
            .lock()
            .execute(
                "INSERT INTO coinbase_user_events (ts_ms, portfolio_id, order_id, product_id, status, side, post_only, avg_price, filled_qty, total_fees, payload) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    Utc::now().timestamp_millis(),
                    &self.portfolio_id,
                    event.order_id,
                    event.product_id,
                    event.status,
                    event.side,
                    if event.post_only { 1 } else { 0 },
                    event.avg_price,
                    event.filled_qty,
                    event.total_fees,
                    payload,
                ],
            )
            .map_err(|e| PtError::Io(e.to_string()))?;
        Ok(())
    }

    fn insert_order_manager_transition(
        &self,
        market_id: Option<&str>,
        order_id: Option<&str>,
        action: &str,
        reason: &str,
        target_price: f64,
        target_size: f64,
    ) -> PtResult<()> {
        self.conn
            .lock()
            .execute(
                "INSERT INTO order_manager_transitions (ts_ms, portfolio_id, market_id, order_id, action, reason, target_price, target_size) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    Utc::now().timestamp_millis(),
                    &self.portfolio_id,
                    market_id,
                    order_id,
                    action,
                    reason,
                    target_price,
                    target_size,
                ],
            )
            .map_err(|e| PtError::Io(e.to_string()))?;
        Ok(())
    }

    fn insert_route_opportunities(&self, opportunities: &[RouteOpportunity]) -> PtResult<()> {
        if opportunities.is_empty() {
            return Ok(());
        }
        let conn = self.conn.lock();
        let tx = conn
            .unchecked_transaction()
            .map_err(|e| PtError::Io(e.to_string()))?;
        for opp in opportunities {
            let payload = serde_json::to_string(opp).map_err(|e| PtError::Serde(e.to_string()))?;
            tx.execute(
                "INSERT INTO route_opportunities (ts_ms, portfolio_id, route_id, strategy_class, gross_edge_bps, expected_net_bps, expected_usd_profit, capital_required_usd, payload) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    opp.ts.timestamp_millis(),
                    &self.portfolio_id,
                    opp.route_id,
                    format!("{:?}", opp.strategy_class),
                    opp.gross_edge_bps,
                    opp.expected_net_bps,
                    opp.expected_usd_profit,
                    opp.capital_required_usd,
                    payload,
                ],
            )
            .map_err(|e| PtError::Io(e.to_string()))?;
        }
        tx.commit().map_err(|e| PtError::Io(e.to_string()))?;
        Ok(())
    }

    fn insert_route_execution(&self, plan: &RouteExecutionPlan) -> PtResult<()> {
        let payload = serde_json::to_string(plan).map_err(|e| PtError::Serde(e.to_string()))?;
        self.conn
            .lock()
            .execute(
                "INSERT INTO route_executions (ts_ms, portfolio_id, route_id, approved, reason, payload) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    plan.ts.timestamp_millis(),
                    &self.portfolio_id,
                    plan.route_id,
                    if plan.approved { 1 } else { 0 },
                    plan.reason,
                    payload,
                ],
            )
            .map_err(|e| PtError::Io(e.to_string()))?;
        Ok(())
    }

    fn insert_fee_tier_snapshot(&self, summary: &CoinbaseTransactionSummary) -> PtResult<()> {
        let payload = serde_json::to_string(summary).map_err(|e| PtError::Serde(e.to_string()))?;
        self.conn
            .lock()
            .execute(
                "INSERT INTO fee_tier_snapshots (ts_ms, portfolio_id, maker_fee_rate, taker_fee_rate, total_fees, payload) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    Utc::now().timestamp_millis(),
                    &self.portfolio_id,
                    summary.maker_fee_rate,
                    summary.taker_fee_rate,
                    summary.total_fees,
                    payload,
                ],
            )
            .map_err(|e| PtError::Io(e.to_string()))?;
        Ok(())
    }

    fn insert_auth_key_event(
        &self,
        action: &str,
        profile_id: Option<&str>,
        key_id_suffix: Option<&str>,
        source: Option<&str>,
        ok: bool,
        reason: &str,
    ) -> PtResult<()> {
        self.conn
            .lock()
            .execute(
                "INSERT INTO auth_key_events (ts_ms, portfolio_id, venue, action, profile_id, key_id_suffix, source, status, reason) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    Utc::now().timestamp_millis(),
                    &self.portfolio_id,
                    "Coinbase",
                    action,
                    profile_id,
                    key_id_suffix,
                    source,
                    if ok { "ok" } else { "error" },
                    reason,
                ],
            )
            .map_err(|e| PtError::Io(e.to_string()))?;
        Ok(())
    }

    fn insert_wallet_intel_signals(&self, signals: &[WalletSignal]) -> PtResult<()> {
        if signals.is_empty() {
            return Ok(());
        }
        let conn = self.conn.lock();
        let tx = conn
            .unchecked_transaction()
            .map_err(|e| PtError::Io(e.to_string()))?;
        for s in signals {
            tx.execute(
                "INSERT INTO wallet_intel_snapshots (ts_ms, portfolio_id, asset, horizon, bias, confidence) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    s.ts.timestamp_millis(),
                    &self.portfolio_id,
                    s.asset.as_str(),
                    format!("{:?}", s.horizon),
                    s.bias,
                    s.confidence
                ],
            )
            .map_err(|e| PtError::Io(e.to_string()))?;
        }
        tx.commit().map_err(|e| PtError::Io(e.to_string()))?;
        Ok(())
    }

    fn roll_snapshots_if_due(&self) -> PtResult<()> {
        let now_ms = Utc::now().timestamp_millis();
        let last = *self.last_roll_ms.read();
        if now_ms - last < (self.snapshot_roll_secs as i64 * 1000) {
            return Ok(());
        }

        let batch = {
            let mut lock = self.snapshot_buffer.write();
            if lock.is_empty() {
                *self.last_roll_ms.write() = now_ms;
                return Ok(());
            }
            std::mem::take(&mut *lock)
        };

        let file_path = format!(
            "{}/snapshots-{}.parquet",
            self.parquet_dir,
            Utc::now().format("%Y%m%d-%H%M%S")
        );
        write_snapshots_parquet(&file_path, &batch)?;
        *self.last_roll_ms.write() = now_ms;
        Ok(())
    }
}

fn ensure_sqlite_column(
    conn: &Connection,
    table: &str,
    column: &str,
    column_def: &str,
) -> PtResult<()> {
    let pragma = format!("PRAGMA table_info({})", table);
    let mut stmt = conn
        .prepare(&pragma)
        .map_err(|e| PtError::Io(e.to_string()))?;
    let existing = stmt
        .query_map([], |r| r.get::<_, String>(1))
        .map_err(|e| PtError::Io(e.to_string()))?
        .filter_map(|row| row.ok())
        .any(|name| name.eq_ignore_ascii_case(column));
    if existing {
        return Ok(());
    }

    let alter = format!("ALTER TABLE {} ADD COLUMN {} {}", table, column, column_def);
    conn.execute(&alter, [])
        .map_err(|e| PtError::Io(e.to_string()))?;
    Ok(())
}

fn write_snapshots_parquet(path: &str, snapshots: &[MarketSnapshot]) -> PtResult<()> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("market_id", DataType::Utf8, false),
        Field::new("token_id", DataType::Utf8, false),
        Field::new("bid", DataType::Float64, false),
        Field::new("ask", DataType::Float64, false),
        Field::new("spread", DataType::Float64, false),
        Field::new("liquidity", DataType::Float64, false),
        Field::new(
            "ts_ms",
            DataType::Timestamp(TimeUnit::Millisecond, None),
            false,
        ),
    ]));

    let market_id = StringArray::from_iter_values(snapshots.iter().map(|s| s.market_id.clone()));
    let token_id = StringArray::from_iter_values(snapshots.iter().map(|s| s.token_id.clone()));
    let bid = Float64Array::from_iter_values(snapshots.iter().map(|s| s.bid));
    let ask = Float64Array::from_iter_values(snapshots.iter().map(|s| s.ask));
    let spread = Float64Array::from_iter_values(snapshots.iter().map(|s| s.spread));
    let liquidity = Float64Array::from_iter_values(snapshots.iter().map(|s| s.liquidity));
    let ts_ms = TimestampMillisecondArray::from_iter_values(
        snapshots.iter().map(|s| s.ts.timestamp_millis()),
    );

    let arrays: Vec<ArrayRef> = vec![
        Arc::new(market_id),
        Arc::new(token_id),
        Arc::new(bid),
        Arc::new(ask),
        Arc::new(spread),
        Arc::new(liquidity),
        Arc::new(ts_ms),
    ];

    let batch =
        RecordBatch::try_new(schema.clone(), arrays).map_err(|e| PtError::Io(e.to_string()))?;

    let file = std::fs::File::create(path).map_err(|e| PtError::Io(e.to_string()))?;
    let mut writer =
        ArrowWriter::try_new(file, schema, None).map_err(|e| PtError::Io(e.to_string()))?;
    writer
        .write(&batch)
        .map_err(|e| PtError::Io(e.to_string()))?;
    writer.close().map_err(|e| PtError::Io(e.to_string()))?;

    Ok(())
}

#[derive(Clone)]
struct TvWebhookState {
    tv_bias: Arc<RwLock<Option<TradingViewBias>>>,
    secret: Option<String>,
    metrics: Arc<MetricsRegistry>,
}

#[derive(Clone)]
struct EngineCoinbaseAuthController {
    manager: Arc<CoinbaseAuthManager>,
    storage: Arc<Storage>,
}

impl CoinbaseAuthController for EngineCoinbaseAuthController {
    fn status(&self) -> pt_coinbase::CoinbaseAuthStatus {
        self.manager.status()
    }

    fn reload(&self) -> pt_core::AuthReloadResult {
        let result = self.manager.reload_active_profile();
        let source = result.source.as_ref().map(|s| format!("{:?}", s));
        if let Err(e) = self.storage.insert_auth_key_event(
            "reload",
            result.profile_id.as_deref(),
            result.key_id_suffix.as_deref(),
            source.as_deref(),
            result.ok,
            &result.reason,
        ) {
            error!(%e, "persist auth reload event failed");
        }
        result
    }

    fn switch_profile(&self, profile_id: &str) -> pt_core::AuthReloadResult {
        let result = self.manager.switch_profile(profile_id);
        let source = result.source.as_ref().map(|s| format!("{:?}", s));
        if let Err(e) = self.storage.insert_auth_key_event(
            "switch_profile",
            result.profile_id.as_deref(),
            result.key_id_suffix.as_deref(),
            source.as_deref(),
            result.ok,
            &result.reason,
        ) {
            error!(%e, "persist auth switch event failed");
        }
        result
    }
}

pub struct TradingEngine {
    cfg: AppConfig,
    portfolio_id: String,
    metrics: Arc<MetricsRegistry>,
    state: SharedState,
    market_discovery: MarketDiscoveryClient,
    polymarket: PolymarketClient,
    wallet_intel: WalletIntelClient,
    signal_fusion: SignalFusionEngine,
    risk: Arc<RiskEngine>,
    quote_cfg: QuoteConfig,
    poly_exec: Arc<dyn PolymarketExecution>,
    hedger: Arc<dyn HedgeExecutor>,
    coinbase_auth: Option<Arc<CoinbaseAuthManager>>,
    coinbase_wallet: Option<Arc<CoinbaseWalletClient>>,
    storage: Arc<Storage>,
}

fn parse_timeout_streak(message: &str) -> Option<f64> {
    let key = "streak=";
    let start = message.find(key)? + key.len();
    let tail = &message[start..];
    let end = tail.find(')').unwrap_or(tail.len());
    tail[..end].trim().parse::<f64>().ok()
}

impl TradingEngine {
    pub fn new(cfg: AppConfig) -> PtResult<Self> {
        let portfolio_id = cfg
            .engine
            .portfolio_id
            .clone()
            .filter(|v| !v.trim().is_empty())
            .unwrap_or_else(|| "default".to_string());
        let metrics = Arc::new(MetricsRegistry::default());
        let state = SharedState::new(policy_from_config(&cfg));
        *state.venue_capabilities.write() = venue_capabilities_from_config(&cfg);
        let min_expected_net = cfg.risk.min_expected_net;

        let market_discovery = MarketDiscoveryClient::new(
            cfg.venues.polymarket.gamma_api.clone(),
            cfg.venues.polymarket.filters.clone(),
        );

        let polymarket = PolymarketClient::new(
            cfg.venues.polymarket.clob_api.clone(),
            cfg.venues.polymarket.clob_ws.clone(),
        );

        let wallet_intel = WalletIntelClient::new(
            cfg.venues.polymarket.data_api.clone(),
            cfg.venues.polymarket.gamma_api.clone(),
            cfg.signals.wallet.top_n,
            cfg.signals.wallet.allowlist_path.clone(),
            cfg.signals.wallet.enforce_allowlist,
        );

        let signal_fusion = SignalFusionEngine::new(
            cfg.signals.tradingview.k_wallet,
            cfg.signals.tradingview.k_tv,
        );

        let risk = Arc::new(RiskEngine::new(cfg.risk.clone(), 50.0));

        let coinbase_auth = if should_attempt_coinbase_auth(&cfg) {
            match CoinbaseAuthManager::new(cfg.venues.coinbase.clone(), cfg.engine.mode.clone()) {
                Ok(v) => Some(Arc::new(v)),
                Err(e) => {
                    if matches!(cfg.engine.mode, EngineMode::Live)
                        && cfg.venues.coinbase.auth.strict_live_auth
                    {
                        return Err(PtError::Config(format!(
                            "coinbase auth initialization failed in live mode: {e}"
                        )));
                    }
                    warn!(%e, "coinbase auth init failed; continuing without auth manager");
                    None
                }
            }
        } else {
            None
        };

        let poly_exec: Arc<dyn PolymarketExecution> = match cfg.engine.mode {
            EngineMode::Replay | EngineMode::Paper => Arc::new(PaperPolymarketExecutor),
            EngineMode::Live => Arc::new(LivePolymarketExecutor::new(LivePolymarketConfig {
                clob_api: cfg.venues.polymarket.clob_api.clone(),
                private_key: cfg
                    .venues
                    .polymarket
                    .private_key
                    .clone()
                    .unwrap_or_default(),
                chain_id: cfg.venues.polymarket.chain_id,
                use_server_time: cfg.venues.polymarket.use_server_time.unwrap_or(true),
            })?),
        };

        let hedger: Arc<dyn HedgeExecutor> = match cfg.engine.mode {
            EngineMode::Replay | EngineMode::Paper => Arc::new(PaperCoinbaseHedger::default()),
            EngineMode::Live => {
                if let Some(auth) = coinbase_auth.clone() {
                    Arc::new(CoinbaseSpotHedger::new_with_auth_manager(
                        cfg.venues.coinbase.api_base.clone(),
                        auth,
                        cfg.venues.coinbase.passphrase.clone(),
                    ))
                } else {
                    if cfg.venues.coinbase.api_key.is_none()
                        || cfg.venues.coinbase.api_secret.is_none()
                    {
                        return Err(PtError::Config(
                            "live mode requires coinbase auth material (profile or venues.coinbase.api_key/api_secret)"
                                .to_string(),
                        ));
                    }
                    Arc::new(CoinbaseSpotHedger::new(
                        cfg.venues.coinbase.api_base.clone(),
                        cfg.venues.coinbase.api_key.clone(),
                        cfg.venues.coinbase.api_secret.clone(),
                        cfg.venues.coinbase.passphrase.clone(),
                    ))
                }
            }
        };

        let coinbase_wallet = if cfg.wallet.enabled {
            if let Some(auth) = coinbase_auth.clone() {
                Some(Arc::new(CoinbaseWalletClient::new_with_auth_manager(
                    cfg.venues.coinbase.api_base.clone(),
                    auth,
                    cfg.venues.coinbase.passphrase.clone(),
                )))
            } else {
                Some(Arc::new(CoinbaseWalletClient::new(
                    cfg.venues.coinbase.api_base.clone(),
                    cfg.venues.coinbase.api_key.clone(),
                    cfg.venues.coinbase.api_secret.clone(),
                    cfg.venues.coinbase.passphrase.clone(),
                )))
            }
        } else {
            None
        };

        let sqlite_path = apply_portfolio_to_sqlite_path(&cfg.storage.sqlite_path, &portfolio_id);
        let parquet_dir = apply_portfolio_to_parquet_dir(&cfg.storage.parquet_dir, &portfolio_id);

        let storage = Arc::new(Storage::new(
            &sqlite_path,
            &parquet_dir,
            cfg.storage.snapshot_roll_secs,
            &portfolio_id,
        )?);

        if let Some(auth) = &coinbase_auth {
            let status = auth.status();
            let source = status.source.as_ref().map(|v| format!("{:?}", v));
            if let Err(e) = storage.insert_auth_key_event(
                "startup_init",
                status.profile_id.as_deref(),
                status.key_id_suffix.as_deref(),
                source.as_deref(),
                status.ok,
                &status.reason,
            ) {
                warn!(%e, "failed to persist startup auth event");
            }
        }

        Ok(Self {
            cfg,
            portfolio_id,
            metrics,
            state,
            market_discovery,
            polymarket,
            wallet_intel,
            signal_fusion,
            risk,
            quote_cfg: QuoteConfig {
                min_expected_net,
                ..QuoteConfig::default()
            },
            poly_exec,
            hedger,
            coinbase_auth,
            coinbase_wallet,
            storage,
        })
    }

    pub async fn run(&self) -> PtResult<()> {
        info!(
            mode = ?self.cfg.engine.mode,
            portfolio_id = %self.portfolio_id,
            "starting trading engine"
        );

        if let EngineMode::Replay = self.cfg.engine.mode {
            return self.run_replay_mode().await;
        }

        let mut tasks: Vec<JoinHandle<()>> = Vec::new();
        tasks.push(self.spawn_dashboard_server());
        if self.cfg.signals.tradingview.enabled {
            tasks.push(self.spawn_tradingview_server());
        }
        self.push_data_plane_tasks(&mut tasks, true);
        self.await_shutdown(tasks).await
    }

    pub async fn run_homebase(&self) -> PtResult<()> {
        info!(
            mode = ?self.cfg.engine.mode,
            portfolio_id = %self.portfolio_id,
            "starting trading engine homebase mode"
        );

        if let EngineMode::Replay = self.cfg.engine.mode {
            return self.run_replay_mode().await;
        }

        let mut tasks: Vec<JoinHandle<()>> = Vec::new();
        tasks.push(self.spawn_dashboard_server());
        if self.cfg.signals.tradingview.enabled {
            tasks.push(self.spawn_tradingview_server());
        }
        self.push_data_plane_tasks(&mut tasks, false);
        self.await_shutdown(tasks).await
    }

    pub async fn run_exec(&self) -> PtResult<()> {
        info!(
            mode = ?self.cfg.engine.mode,
            portfolio_id = %self.portfolio_id,
            "starting trading engine exec mode"
        );

        if let EngineMode::Replay = self.cfg.engine.mode {
            return self.run_replay_mode().await;
        }

        let mut tasks: Vec<JoinHandle<()>> = Vec::new();
        self.push_data_plane_tasks(&mut tasks, true);
        self.await_shutdown(tasks).await
    }

    fn push_data_plane_tasks(&self, tasks: &mut Vec<JoinHandle<()>>, with_trading: bool) {
        tasks.push(self.spawn_market_refresh_loop());
        tasks.push(self.spawn_wallet_refresh_loop());
        if self.cfg.wallet.enabled {
            tasks.push(self.spawn_coinbase_wallet_sync_loop());
            tasks.push(self.spawn_coinbase_ws_loop());
            tasks.push(self.spawn_route_loop());
            tasks.push(self.spawn_fee_tier_loop());
            if with_trading {
                tasks.push(self.spawn_rebalance_planner_loop());
            }
        }
        tasks.push(self.spawn_orderbook_loop());
        if with_trading {
            tasks.push(self.spawn_quote_loop());
            tasks.push(self.spawn_watchdog_loop());
        }
    }

    async fn await_shutdown(&self, tasks: Vec<JoinHandle<()>>) -> PtResult<()> {
        info!("engine running; press Ctrl+C to stop");
        tokio::signal::ctrl_c()
            .await
            .map_err(|e| PtError::Io(e.to_string()))?;
        info!("shutdown signal received");
        for task in tasks {
            task.abort();
        }
        Ok(())
    }

    async fn run_replay_mode(&self) -> PtResult<()> {
        let replay_path = self
            .cfg
            .engine
            .replay_path
            .clone()
            .unwrap_or_else(|| "data/replay/sample.ndjson".to_string());

        let frames = load_replay_frames(&replay_path)
            .map_err(|e| PtError::Io(format!("failed to load replay: {e}")))?;
        let markets = self.market_discovery.fetch_all_markets().await?;
        let market_map: HashMap<String, MarketSelection> = markets
            .into_iter()
            .map(|m| (m.market_id.clone(), m))
            .collect();

        let mut simulator = PaperSimulator::default();

        for frame in frames {
            let Some(market) = market_map.get(&frame.snapshot.market_id) else {
                continue;
            };

            self.storage.insert_snapshot(&frame.snapshot)?;

            let costs = CostInputs {
                rebate_est: if market.fees_enabled { 0.001 } else { 0.0 },
                adverse_sel_est: 0.003,
                hedge_cost_est: 0.001,
                gas_amortized_est: 0.0005,
            };

            let quote = build_quote_intent(
                market,
                &frame.snapshot,
                frame.bias * 0.005,
                0.0,
                &costs,
                &self.quote_cfg,
            );

            if let Some(q) = quote {
                let age_ms = (Utc::now() - frame.snapshot.ts).num_milliseconds().max(0) as u64;
                let decision = self.risk.evaluate_quote(&q, age_ms);
                if decision.allow {
                    self.risk.reserve_quote_exposure(&q);
                    for report in simulator.apply_quote(&q, &frame.snapshot) {
                        self.apply_execution_effects(&report).await?;
                        self.storage.insert_execution_report(&report)?;
                    }
                    self.risk.release_market_exposure(&q.market_id);
                }
            }

            let risk = self.risk.snapshot();
            self.storage.insert_risk_state(&risk)?;
        }

        self.storage.roll_snapshots_if_due()?;
        info!("replay mode completed");
        Ok(())
    }

    async fn apply_execution_effects(&self, report: &ExecutionReport) -> PtResult<()> {
        push_recent_execution(&self.state.recent_executions, report.clone());
        let event = ExecutionEvent {
            order_id: report.order_id.clone(),
            venue: report.venue.clone(),
            market_id: report.market_id.clone(),
            product_id: report.market_id.clone(),
            side: report.side.clone(),
            state: status_to_lifecycle(&report.status),
            qty: report.filled_qty,
            price: report.avg_px,
            ts: Utc::now(),
            details: report.details.clone(),
            reason_code: None,
            unwind_flag: false,
        };
        push_execution_event(&self.state.execution_events, event.clone());
        self.storage.insert_execution_event(&event)?;

        let policy = self.state.execution_policy.read().clone();
        let fee_bps = default_fee_bps_for_venue(
            &report.venue,
            policy.polymarket_fees.maker_bps,
            policy.coinbase_fees.taker_bps,
        );
        let rebate_bps = if matches!(report.venue, Venue::Polymarket) {
            policy.polymarket_fees.rebate_bps_est
        } else {
            0.0
        };
        let cost =
            estimate_execution_cost(&report.order_id, report, report.avg_px, fee_bps, rebate_bps);
        push_execution_cost(&self.state.execution_costs, cost.clone());
        self.storage.insert_execution_cost(&cost)?;

        apply_fill_to_inventory(&self.state.inventory_usd, report)?;
        let current_inv = *self.state.inventory_usd.read();
        self.risk.update_unhedged_delta(current_inv);

        if matches!(report.status, pt_core::ExecutionStatus::Filled)
            && current_inv.abs() >= self.cfg.venues.coinbase.hedge_threshold_usd
        {
            let side = if current_inv > 0.0 {
                Side::Sell
            } else {
                Side::Buy
            };
            let intent = HedgeIntent {
                asset: Asset::Btc,
                side,
                usd_notional: self.cfg.venues.coinbase.hedge_threshold_usd,
                max_slippage_bps: self.cfg.venues.coinbase.hedge_max_slippage_bps,
                risk_unwind: true,
            };
            let hedge_report = self.hedger.hedge(intent).await?;
            self.storage.insert_execution_report(&hedge_report)?;
            push_recent_execution(&self.state.recent_executions, hedge_report);
        }

        Ok(())
    }

    fn spawn_dashboard_server(&self) -> JoinHandle<()> {
        let bind = self.cfg.ops.dashboard_bind.clone();
        let coinbase_auth_controller: Option<Arc<dyn CoinbaseAuthController>> =
            self.coinbase_auth.clone().map(|manager| {
                Arc::new(EngineCoinbaseAuthController {
                    manager,
                    storage: self.storage.clone(),
                }) as Arc<dyn CoinbaseAuthController>
            });
        let state = DashboardState::new(
            self.metrics.clone(),
            self.state.risk_state.clone(),
            self.state.kill_switch.clone(),
            self.state.selected_markets.clone(),
            self.state.latest_books.clone(),
            self.state.market_history.clone(),
            self.state.recent_executions.clone(),
            self.state.execution_events.clone(),
            self.state.execution_costs.clone(),
            self.state.execution_policy.clone(),
            self.state.fused_bias.clone(),
            self.state.inventory_usd.clone(),
            self.state.wallet_balances.clone(),
            self.state.wallet_drifts.clone(),
            self.state.wallet_open_orders.clone(),
            self.state.coinbase_orderbooks.clone(),
            self.state.route_opportunities.clone(),
            self.state.route_executions.clone(),
            self.state.venue_capabilities.clone(),
            self.state.coinbase_fee_summary.clone(),
            self.state.rebalance_plan.clone(),
            self.state.rebalance_approval.clone(),
            self.state.force_unwind.clone(),
            coinbase_auth_controller,
            self.coinbase_wallet.clone(),
            self.cfg.venues.coinbase.products.clone(),
            self.cfg.engine.mode.clone(),
        );

        tokio::spawn(async move {
            let app = dashboard_router(state);
            let addr: SocketAddr = match bind.parse() {
                Ok(v) => v,
                Err(e) => {
                    error!(%e, "invalid dashboard bind addr");
                    return;
                }
            };

            let listener = match tokio::net::TcpListener::bind(addr).await {
                Ok(l) => l,
                Err(e) => {
                    error!(%e, "failed to bind dashboard");
                    return;
                }
            };

            if let Err(e) = axum::serve(listener, app).await {
                error!(%e, "dashboard server failed");
            }
        })
    }

    fn spawn_tradingview_server(&self) -> JoinHandle<()> {
        let bind = self.cfg.signals.tradingview.bind_addr.clone();
        let tv_state = TvWebhookState {
            tv_bias: self.state.tv_bias.clone(),
            secret: self.cfg.signals.tradingview.endpoint_secret.clone(),
            metrics: self.metrics.clone(),
        };

        tokio::spawn(async move {
            let app = Router::new()
                .route("/tradingview", post(tradingview_webhook))
                .with_state(tv_state);

            let addr: SocketAddr = match bind.parse() {
                Ok(v) => v,
                Err(e) => {
                    error!(%e, "invalid tradingview bind addr");
                    return;
                }
            };

            let listener = match tokio::net::TcpListener::bind(addr).await {
                Ok(l) => l,
                Err(e) => {
                    error!(%e, "failed to bind tradingview listener");
                    return;
                }
            };

            if let Err(e) = axum::serve(listener, app).await {
                error!(%e, "tradingview listener failed");
            }
        })
    }

    fn spawn_market_refresh_loop(&self) -> JoinHandle<()> {
        let client = self.market_discovery.clone();
        let selected = self.state.selected_markets.clone();
        let metrics = self.metrics.clone();
        let refresh = self.cfg.ops.market_refresh_secs.max(5);

        tokio::spawn(async move {
            loop {
                let start = Instant::now();
                match client.fetch_all_markets().await {
                    Ok(markets) => {
                        let total = markets.len() as f64;
                        let tier_a = markets
                            .iter()
                            .filter(|m| matches!(m.tier, pt_core::MarketTier::TierA))
                            .count() as f64;
                        *selected.write() = markets;
                        metrics.set_gauge("markets_total", total);
                        metrics.set_gauge("markets_tier_a", tier_a);
                        metrics.inc_counter("market_refresh_ok", 1.0);
                    }
                    Err(e) => {
                        error!(%e, "market refresh failed");
                        metrics.inc_counter("market_refresh_error", 1.0);
                    }
                }
                metrics.set_gauge("market_refresh_ms", start.elapsed().as_millis() as f64);
                tokio::time::sleep(Duration::from_secs(refresh)).await;
            }
        })
    }

    fn spawn_wallet_refresh_loop(&self) -> JoinHandle<()> {
        let wallet = self.wallet_intel.clone();
        let fusion = self.signal_fusion.clone();
        let fused_bias = self.state.fused_bias.clone();
        let tv_bias = self.state.tv_bias.clone();
        let metrics = self.metrics.clone();
        let storage = self.storage.clone();
        let refresh = self.cfg.ops.wallet_refresh_secs.max(10);

        tokio::spawn(async move {
            loop {
                let start = Instant::now();
                match wallet.compute_wallet_biases().await {
                    Ok(wallet_signals) => {
                        let tv = tv_bias.read().clone();
                        let map = fusion.fuse(&wallet_signals, tv);
                        *fused_bias.write() = map;
                        if let Err(e) = storage.insert_wallet_intel_signals(&wallet_signals) {
                            error!(%e, "persist wallet intel snapshots failed");
                        }
                        metrics.set_gauge("wallet_signals_count", wallet_signals.len() as f64);
                        metrics.inc_counter("wallet_refresh_ok", 1.0);
                    }
                    Err(e) => {
                        error!(%e, "wallet refresh failed");
                        metrics.inc_counter("wallet_refresh_error", 1.0);
                    }
                }
                metrics.set_gauge("wallet_refresh_ms", start.elapsed().as_millis() as f64);
                tokio::time::sleep(Duration::from_secs(refresh)).await;
            }
        })
    }

    fn spawn_coinbase_wallet_sync_loop(&self) -> JoinHandle<()> {
        let Some(wallet_client) = self.coinbase_wallet.clone() else {
            return tokio::spawn(async {});
        };

        let balances_state = self.state.wallet_balances.clone();
        let open_orders_state = self.state.wallet_open_orders.clone();
        let drifts_state = self.state.wallet_drifts.clone();
        let metrics = self.metrics.clone();
        let storage = self.storage.clone();
        let products = self.cfg.venues.coinbase.products.clone();
        let targets = self.cfg.wallet.targets.clone();
        let refresh_secs = self.cfg.wallet.sync_secs.max(5);

        tokio::spawn(async move {
            loop {
                let start = Instant::now();

                match wallet_client.fetch_wallet_balances(&products).await {
                    Ok(balances) => {
                        let drifts = compute_allocation_drifts(&balances, &targets);
                        *balances_state.write() = balances.clone();
                        *drifts_state.write() = drifts;
                        if let Err(e) = storage.insert_wallet_balances(&balances) {
                            error!(%e, "persist wallet balances failed");
                        }
                        metrics.inc_counter("wallet_sync_ok", 1.0);
                        metrics.set_gauge("wallet_balance_assets", balances.len() as f64);
                    }
                    Err(e) => {
                        error!(%e, "coinbase wallet sync failed");
                        metrics.inc_counter("wallet_sync_error", 1.0);
                    }
                }

                match wallet_client.fetch_open_orders().await {
                    Ok(open_orders) => {
                        *open_orders_state.write() = open_orders.clone();
                        if let Err(e) = storage.insert_coinbase_orders(&open_orders) {
                            error!(%e, "persist coinbase orders failed");
                        }
                    }
                    Err(e) => {
                        warn!(%e, "coinbase open orders fetch failed");
                    }
                }

                metrics.set_gauge("wallet_sync_ms", start.elapsed().as_millis() as f64);
                tokio::time::sleep(Duration::from_secs(refresh_secs)).await;
            }
        })
    }

    fn spawn_coinbase_ws_loop(&self) -> JoinHandle<()> {
        let Some(wallet_client) = self.coinbase_wallet.clone() else {
            return tokio::spawn(async {});
        };

        let orderbooks_state = self.state.coinbase_orderbooks.clone();
        let user_events_state = self.state.coinbase_user_events.clone();
        let metrics = self.metrics.clone();
        let storage = self.storage.clone();
        let products = self.cfg.venues.coinbase.products.clone();
        let ws_cfg = self.cfg.venues.coinbase.ws.clone();

        tokio::spawn(async move {
            loop {
                let run_cfg = CoinbaseWsRunConfig {
                    ws_url: ws_cfg.url.clone(),
                    channels: ws_cfg.channels.clone(),
                    product_ids: products.clone(),
                    heartbeat_timeout_ms: ws_cfg.heartbeat_timeout_ms,
                    resync_on_gap: ws_cfg.resync_on_gap,
                };

                let mut rx = match wallet_client.spawn_ws_event_loop(run_cfg) {
                    Ok(rx) => rx,
                    Err(e) => {
                        error!(%e, "coinbase ws spawn failed");
                        tokio::time::sleep(Duration::from_secs(2)).await;
                        continue;
                    }
                };

                while let Some(ev) = rx.recv().await {
                    match ev {
                        CoinbaseWsEvent::Reconnected => {
                            metrics.inc_counter("coinbase_ws_reconnects", 1.0);
                            metrics.set_gauge("coinbase_ws_timeout_streak", 0.0);
                        }
                        CoinbaseWsEvent::Subscribed { channel } => {
                            metrics.inc_counter(
                                &format!("coinbase_ws_subscribed_{}", channel.to_ascii_lowercase()),
                                1.0,
                            );
                        }
                        CoinbaseWsEvent::Heartbeat {
                            sequence_num,
                            heartbeat_counter: _,
                            ts: _,
                        } => {
                            metrics
                                .set_gauge("coinbase_ws_last_heartbeat_seq", sequence_num as f64);
                            metrics.set_gauge(
                                "coinbase_ws_last_heartbeat_ts_ms",
                                Utc::now().timestamp_millis() as f64,
                            );
                            metrics.set_gauge("coinbase_ws_timeout_streak", 0.0);
                        }
                        CoinbaseWsEvent::L2 { update } => {
                            apply_coinbase_l2_update(&orderbooks_state, &update);
                            if let Err(e) = storage.insert_coinbase_l2_event(&update) {
                                error!(%e, "persist coinbase l2 event failed");
                            }
                            metrics.inc_counter("coinbase_ws_l2_updates", 1.0);
                            metrics.set_gauge(
                                "coinbase_ws_last_l2_ts_ms",
                                update.event_time.timestamp_millis() as f64,
                            );
                        }
                        CoinbaseWsEvent::User { update } => {
                            push_coinbase_user_event(&user_events_state, update.clone());
                            if let Err(e) = storage.insert_coinbase_user_event(&update) {
                                error!(%e, "persist coinbase user event failed");
                            }
                            metrics.inc_counter("coinbase_ws_user_updates", 1.0);
                        }
                        CoinbaseWsEvent::Gap {
                            product_id,
                            expected_sequence: _,
                            received_sequence: _,
                        } => {
                            metrics.inc_counter("coinbase_ws_sequence_gap", 1.0);
                            if ws_cfg.resync_on_gap {
                                match wallet_client.get_product_book(&product_id, 200).await {
                                    Ok(book) => {
                                        let mut state =
                                            CoinbaseWalletClient::product_book_to_state(&book);
                                        state.last_event_ts = Some(Utc::now());
                                        orderbooks_state.write().insert(product_id.clone(), state);
                                    }
                                    Err(e) => {
                                        warn!(%e, product_id, "coinbase ws resync snapshot failed");
                                    }
                                }
                            }
                        }
                        CoinbaseWsEvent::Error { message } => {
                            warn!(%message, "coinbase ws event error");
                            metrics.inc_counter("coinbase_ws_errors", 1.0);
                            if let Some(streak) = parse_timeout_streak(&message) {
                                metrics.inc_counter("coinbase_ws_read_timeouts", 1.0);
                                metrics.set_gauge("coinbase_ws_timeout_streak", streak);
                            }
                            if message.contains("ping failed") {
                                metrics.inc_counter("coinbase_ws_ping_failures", 1.0);
                            }
                            if message.contains("heartbeat timeout") {
                                metrics.inc_counter("coinbase_ws_heartbeat_timeouts", 1.0);
                            }
                            if message.contains("closed by remote") {
                                metrics.inc_counter("coinbase_ws_remote_closes", 1.0);
                            }
                            if message.contains("read error") {
                                metrics.inc_counter("coinbase_ws_read_errors", 1.0);
                            }
                            if message.contains("connect failed") {
                                metrics.inc_counter("coinbase_ws_connect_failures", 1.0);
                            }
                        }
                    }
                }

                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        })
    }

    fn spawn_route_loop(&self) -> JoinHandle<()> {
        let orderbooks = self.state.coinbase_orderbooks.clone();
        let opportunities_state = self.state.route_opportunities.clone();
        let executions_state = self.state.route_executions.clone();
        let balances_state = self.state.wallet_balances.clone();
        let execution_policy = self.state.execution_policy.clone();
        let metrics = self.metrics.clone();
        let storage = self.storage.clone();
        let refresh_ms = self.cfg.engine.loop_ms.max(250);
        let wallet_cfg = self.cfg.wallet.clone();
        let mode = self.cfg.engine.mode.clone();

        tokio::spawn(async move {
            let mut last_route_id = String::new();
            loop {
                let books_snapshot = orderbooks.read().clone();
                if books_snapshot.len() < 2 {
                    tokio::time::sleep(Duration::from_millis(refresh_ms)).await;
                    continue;
                }

                let route_books: HashMap<String, RouteBook> = books_snapshot
                    .iter()
                    .filter_map(|(product, book)| {
                        let best_bid = book.bids.first().map(|x| x.0).unwrap_or(0.0);
                        let best_ask = book.asks.first().map(|x| x.0).unwrap_or(0.0);
                        if best_bid > 0.0 && best_ask > 0.0 && best_bid < best_ask {
                            Some((product.clone(), RouteBook { best_bid, best_ask }))
                        } else {
                            None
                        }
                    })
                    .collect();

                if route_books.len() < 2 {
                    tokio::time::sleep(Duration::from_millis(refresh_ms)).await;
                    continue;
                }

                let wallet_total_usd = balances_state
                    .read()
                    .iter()
                    .map(|b| b.usd_value.max(0.0))
                    .sum::<f64>();
                let reserve_floor = wallet_cfg.targets.usd.clamp(0.0, 0.95);
                let allocatable = (wallet_total_usd * (1.0 - reserve_floor)).max(0.0);
                let capital_slice = allocatable.min(2.5).max(0.0);
                if capital_slice <= 0.0 {
                    tokio::time::sleep(Duration::from_millis(refresh_ms)).await;
                    continue;
                }

                let policy = execution_policy.read().clone();
                let opportunities = find_route_opportunities(
                    &route_books,
                    capital_slice,
                    policy.coinbase_fees.maker_bps,
                    2.0,
                    1.0,
                    &policy.edge_profiles,
                );
                let top: Vec<RouteOpportunity> = opportunities.into_iter().take(40).collect();
                *opportunities_state.write() = top.clone();
                if let Err(e) = storage.insert_route_opportunities(&top) {
                    error!(%e, "persist route opportunities failed");
                }
                metrics.set_gauge("routes_opportunity_count", top.len() as f64);

                if let Some(best) = top.first() {
                    if best.route_id != last_route_id {
                        let approved = matches!(mode, EngineMode::Live)
                            && matches!(wallet_cfg.mode, WalletModeConfig::Auto);
                        let reason = if approved {
                            Some("auto_mode_route_candidate".to_string())
                        } else {
                            Some("assist_mode_route_candidate".to_string())
                        };
                        let plan = RouteExecutionPlan {
                            route_id: best.route_id.clone(),
                            legs: best.legs.clone(),
                            approved,
                            reason,
                            ts: Utc::now(),
                        };
                        push_route_execution(&executions_state, plan.clone());
                        if let Err(e) = storage.insert_route_execution(&plan) {
                            error!(%e, "persist route execution plan failed");
                        }
                        last_route_id = best.route_id.clone();
                    }
                }

                tokio::time::sleep(Duration::from_millis(refresh_ms)).await;
            }
        })
    }

    fn spawn_fee_tier_loop(&self) -> JoinHandle<()> {
        let Some(wallet_client) = self.coinbase_wallet.clone() else {
            return tokio::spawn(async {});
        };

        let fee_state = self.state.coinbase_fee_summary.clone();
        let metrics = self.metrics.clone();
        let storage = self.storage.clone();

        tokio::spawn(async move {
            loop {
                match wallet_client.get_transaction_summary().await {
                    Ok(summary) => {
                        *fee_state.write() = Some(summary.clone());
                        if let Err(e) = storage.insert_fee_tier_snapshot(&summary) {
                            error!(%e, "persist fee summary failed");
                        }
                        metrics.inc_counter("coinbase_fee_summary_ok", 1.0);
                    }
                    Err(e) => {
                        warn!(%e, "coinbase fee summary fetch failed");
                        metrics.inc_counter("coinbase_fee_summary_error", 1.0);
                    }
                }
                tokio::time::sleep(Duration::from_secs(300)).await;
            }
        })
    }

    fn spawn_rebalance_planner_loop(&self) -> JoinHandle<()> {
        let Some(wallet_client) = self.coinbase_wallet.clone() else {
            return tokio::spawn(async {});
        };

        let balances_state = self.state.wallet_balances.clone();
        let open_orders_state = self.state.wallet_open_orders.clone();
        let drifts_state = self.state.wallet_drifts.clone();
        let plan_state = self.state.rebalance_plan.clone();
        let approval_state = self.state.rebalance_approval.clone();
        let capabilities_state = self.state.venue_capabilities.clone();
        let recent_executions = self.state.recent_executions.clone();
        let event_state = self.state.execution_events.clone();
        let cost_state = self.state.execution_costs.clone();
        let storage = self.storage.clone();
        let metrics = self.metrics.clone();
        let cfg_wallet = self.cfg.wallet.clone();
        let cfg_exec = self.cfg.execution.clone();
        let mode = self.cfg.engine.mode.clone();

        tokio::spawn(async move {
            let mut last_plan_id = String::new();
            let order_manager = OrderManager::new(RepriceManagerConfig {
                preview_required: cfg_exec.order_manager.preview_required,
                max_reprice_attempts: cfg_exec.order_manager.max_reprice_attempts,
                edit_vs_replace_threshold_bps: cfg_exec.order_manager.edit_vs_replace_threshold_bps,
                cancel_replace_cooldown_ms: cfg_exec.order_manager.cancel_replace_cooldown_ms,
                min_rest_ms: cfg_exec.order_manager.min_rest_ms,
            });
            let mut resting_orders: HashMap<String, RestingOrder> = HashMap::new();
            loop {
                let balances = balances_state.read().clone();
                let drifts = drifts_state.read().clone();
                if balances.is_empty() {
                    tokio::time::sleep(Duration::from_secs(cfg_wallet.sync_secs.max(5))).await;
                    continue;
                }

                let maybe_plan =
                    build_rebalance_plan(&wallet_client, &drifts, &cfg_wallet, &cfg_exec).await;

                if let Some(mut plan) = maybe_plan {
                    if plan.plan_id != last_plan_id {
                        if let Err(e) = storage.insert_rebalance_plan(&plan) {
                            error!(%e, "persist rebalance plan failed");
                        }
                        let approval = ApprovalToken {
                            token_id: format!("approval-{}", Utc::now().timestamp_millis()),
                            plan_id: plan.plan_id.clone(),
                            approved: !cfg_wallet.approval.required
                                || matches!(cfg_wallet.mode, WalletModeConfig::Auto),
                            created_ts: Utc::now(),
                            expires_ts: Utc::now()
                                + chrono::Duration::seconds(
                                    cfg_wallet.approval.token_ttl_secs as i64,
                                ),
                        };
                        *approval_state.write() = Some(approval);
                        last_plan_id = plan.plan_id.clone();
                    }

                    // Approval-driven execution
                    let approval = approval_state.read().clone();
                    let should_execute = approval
                        .as_ref()
                        .map(|a| {
                            a.approved
                                && a.plan_id == plan.plan_id
                                && a.expires_ts > Utc::now()
                                && !matches!(cfg_wallet.mode, WalletModeConfig::Monitor)
                        })
                        .unwrap_or(false);

                    if should_execute {
                        plan.status = RebalancePlanStatus::Approved;
                        if let Err(e) = storage.insert_rebalance_plan(&plan) {
                            error!(%e, "persist approved rebalance plan failed");
                        }

                        let coinbase_supports_amend = capabilities_state
                            .read()
                            .iter()
                            .find(|c| matches!(c.venue, Venue::Coinbase))
                            .map(|c| c.supports_amend)
                            .unwrap_or(true);

                        let open_orders_snapshot = open_orders_state.read().clone();

                        for intent in &plan.intents {
                            let desired_size = if intent.limit_price > 0.0 {
                                intent.usd_notional / intent.limit_price
                            } else {
                                0.0
                            };
                            if desired_size <= 0.0 {
                                continue;
                            }

                            let top = wallet_client
                                .fetch_top_of_book(&intent.product_id)
                                .await
                                .unwrap_or(CoinbaseTopOfBook {
                                    product_id: intent.product_id.clone(),
                                    best_bid: intent.limit_price.max(0.00000001),
                                    best_ask: intent.limit_price.max(0.00000001) + 0.01,
                                });

                            let now_ms = Utc::now().timestamp_millis();
                            let slot_key = order_manager_slot(&intent.product_id, &intent.side);
                            let mut existing = resting_orders.get(&slot_key).cloned();
                            if existing.is_none() {
                                if let Some(order) = open_orders_snapshot.iter().find(|o| {
                                    let side_matches = match intent.side {
                                        Side::Buy => o.side.eq_ignore_ascii_case("BUY"),
                                        Side::Sell => o.side.eq_ignore_ascii_case("SELL"),
                                    };
                                    o.product_id.eq_ignore_ascii_case(&intent.product_id)
                                        && side_matches
                                        && o.is_open_like()
                                }) {
                                    if let (Some(px), Some(sz)) =
                                        (order.resting_price(), order.resting_size())
                                    {
                                        let submitted_ts = order.created_ts_ms().unwrap_or(now_ms);
                                        existing = Some(RestingOrder {
                                            order_id: order.order_id.clone(),
                                            side: intent.side.clone(),
                                            price: px,
                                            size: sz,
                                            submitted_ts_ms: submitted_ts,
                                            last_replace_ts_ms: submitted_ts,
                                        });
                                    }
                                }
                            }

                            let om_decision = order_manager.decide(
                                existing.as_ref(),
                                intent.side.clone(),
                                intent.limit_price,
                                desired_size,
                                coinbase_supports_amend,
                                TopOfBook {
                                    best_bid: top.best_bid,
                                    best_ask: top.best_ask,
                                    tick_size: 0.01,
                                },
                                now_ms,
                            );
                            if let Err(e) = storage.insert_order_manager_transition(
                                Some(&intent.product_id),
                                existing.as_ref().map(|o| o.order_id.as_str()),
                                &om_decision.action,
                                &om_decision.reason,
                                om_decision.target_price,
                                om_decision.target_size,
                            ) {
                                error!(%e, "persist order manager transition failed");
                            }

                            if om_decision.action == "hold" {
                                continue;
                            }

                            if cfg_exec.order_manager.preview_required
                                && matches!(
                                    om_decision.action.as_str(),
                                    "submit" | "edit" | "cancel_replace"
                                )
                            {
                                match wallet_client
                                    .preview_order_post_only(
                                        &intent.product_id,
                                        intent.side.clone(),
                                        om_decision.target_size,
                                        om_decision.target_price,
                                    )
                                    .await
                                {
                                    Ok(preview) => {
                                        if !preview.success {
                                            metrics.inc_counter("rebalance_preview_rejected", 1.0);
                                            let reason = preview
                                                .failure_reason
                                                .unwrap_or_else(|| "preview_rejected".to_string());
                                            if let Err(e) = storage.insert_order_manager_transition(
                                                Some(&intent.product_id),
                                                existing.as_ref().map(|o| o.order_id.as_str()),
                                                "preview_reject",
                                                &reason,
                                                om_decision.target_price,
                                                om_decision.target_size,
                                            ) {
                                                error!(
                                                    %e,
                                                    "persist preview reject transition failed"
                                                );
                                            }
                                            continue;
                                        }
                                    }
                                    Err(e) => {
                                        metrics.inc_counter("rebalance_preview_error", 1.0);
                                        error!(%e, "rebalance preview failed");
                                        continue;
                                    }
                                }
                            }

                            let exec = if matches!(mode, EngineMode::Live) {
                                match om_decision.action.as_str() {
                                    "edit" => {
                                        if let Some(resting) = existing.as_ref() {
                                            wallet_client
                                                .edit_order(
                                                    &resting.order_id,
                                                    om_decision.target_price,
                                                    om_decision.target_size,
                                                )
                                                .await
                                        } else {
                                            wallet_client
                                                .place_limit_post_only(
                                                    &intent.product_id,
                                                    intent.side.clone(),
                                                    om_decision.target_size,
                                                    om_decision.target_price,
                                                )
                                                .await
                                        }
                                    }
                                    "cancel_replace" => {
                                        if let Some(resting) = existing.as_ref() {
                                            let _ = wallet_client
                                                .cancel_orders_batch(&[resting.order_id.clone()])
                                                .await;
                                        }
                                        wallet_client
                                            .place_limit_post_only(
                                                &intent.product_id,
                                                intent.side.clone(),
                                                om_decision.target_size,
                                                om_decision.target_price,
                                            )
                                            .await
                                    }
                                    _ => {
                                        wallet_client
                                            .place_limit_post_only(
                                                &intent.product_id,
                                                intent.side.clone(),
                                                om_decision.target_size,
                                                om_decision.target_price,
                                            )
                                            .await
                                    }
                                }
                            } else {
                                Ok(ExecutionReport {
                                    venue: Venue::Sim,
                                    order_id: existing
                                        .as_ref()
                                        .map(|o| o.order_id.clone())
                                        .unwrap_or_else(|| {
                                            format!("rebalance-sim-{}", intent.intent_id)
                                        }),
                                    market_id: Some(intent.product_id.clone()),
                                    status: if om_decision.action == "cancel_replace" {
                                        pt_core::ExecutionStatus::Canceled
                                    } else {
                                        pt_core::ExecutionStatus::New
                                    },
                                    side: intent.side.clone(),
                                    filled_qty: om_decision.target_size,
                                    avg_px: om_decision.target_price,
                                    ts: Utc::now(),
                                    details: Some(format!(
                                        "rebalance simulated action={}",
                                        om_decision.action
                                    )),
                                })
                            };

                            match exec {
                                Ok(report) => {
                                    if let Err(e) = storage.insert_execution_report(&report) {
                                        error!(%e, "persist rebalance execution report failed");
                                    }
                                    push_recent_execution(&recent_executions, report.clone());

                                    let event = ExecutionEvent {
                                        order_id: report.order_id.clone(),
                                        venue: report.venue.clone(),
                                        market_id: report.market_id.clone(),
                                        product_id: report.market_id.clone(),
                                        side: report.side.clone(),
                                        state: status_to_lifecycle(&report.status),
                                        qty: report.filled_qty,
                                        price: report.avg_px,
                                        ts: Utc::now(),
                                        details: report.details.clone(),
                                        reason_code: Some("rebalance".to_string()),
                                        unwind_flag: false,
                                    };
                                    push_execution_event(&event_state, event.clone());
                                    if let Err(e) = storage.insert_execution_event(&event) {
                                        error!(%e, "persist rebalance execution event failed");
                                    }

                                    let maker_fee = cfg_exec.fees.coinbase.maker_bps;
                                    let cost = estimate_execution_cost(
                                        &report.order_id,
                                        &report,
                                        report.avg_px,
                                        maker_fee,
                                        cfg_exec.fees.coinbase.rebate_bps_est,
                                    );
                                    push_execution_cost(&cost_state, cost.clone());
                                    if let Err(e) = storage.insert_execution_cost(&cost) {
                                        error!(%e, "persist rebalance execution cost failed");
                                    }

                                    let refresh_now = Utc::now().timestamp_millis();
                                    if matches!(mode, EngineMode::Live) {
                                        if let Ok(order_state) =
                                            wallet_client.get_order(&report.order_id).await
                                        {
                                            if let Some(order_state) = order_state {
                                                if order_state.is_open_like() {
                                                    if let (Some(px), Some(sz)) = (
                                                        order_state.resting_price(),
                                                        order_state.resting_size(),
                                                    ) {
                                                        resting_orders.insert(
                                                            slot_key.clone(),
                                                            RestingOrder {
                                                                order_id: order_state
                                                                    .order_id
                                                                    .clone(),
                                                                side: intent.side.clone(),
                                                                price: px,
                                                                size: sz,
                                                                submitted_ts_ms: order_state
                                                                    .created_ts_ms()
                                                                    .unwrap_or(refresh_now),
                                                                last_replace_ts_ms: if om_decision
                                                                    .action
                                                                    == "cancel_replace"
                                                                {
                                                                    refresh_now
                                                                } else {
                                                                    existing
                                                                        .as_ref()
                                                                        .map(|o| {
                                                                            o.last_replace_ts_ms
                                                                        })
                                                                        .unwrap_or(refresh_now)
                                                                },
                                                            },
                                                        );
                                                    }
                                                } else {
                                                    resting_orders.remove(&slot_key);
                                                }
                                            }
                                        }

                                        if let Ok(fills) = wallet_client
                                            .list_fills(
                                                Some(&intent.product_id),
                                                Some(&report.order_id),
                                            )
                                            .await
                                        {
                                            if !fills.is_empty() {
                                                metrics.inc_counter(
                                                    "rebalance_reconcile_fills_seen",
                                                    fills.len() as f64,
                                                );
                                            }
                                        }
                                    } else if om_decision.action == "submit"
                                        || om_decision.action == "edit"
                                        || om_decision.action == "cancel_replace"
                                    {
                                        resting_orders.insert(
                                            slot_key.clone(),
                                            RestingOrder {
                                                order_id: report.order_id.clone(),
                                                side: intent.side.clone(),
                                                price: om_decision.target_price,
                                                size: om_decision.target_size,
                                                submitted_ts_ms: refresh_now,
                                                last_replace_ts_ms: if om_decision.action
                                                    == "cancel_replace"
                                                {
                                                    refresh_now
                                                } else {
                                                    existing
                                                        .as_ref()
                                                        .map(|o| o.last_replace_ts_ms)
                                                        .unwrap_or(refresh_now)
                                                },
                                            },
                                        );
                                    }
                                }
                                Err(e) => {
                                    error!(%e, "rebalance execution failed");
                                    metrics.inc_counter("rebalance_execute_error", 1.0);
                                }
                            }
                        }

                        plan.status = RebalancePlanStatus::Executed;
                        if let Err(e) = storage.insert_rebalance_plan(&plan) {
                            error!(%e, "persist executed rebalance plan failed");
                        }
                        if let Ok(payload) = serde_json::to_string(&plan) {
                            let _ = storage.insert_rebalance_action(
                                &plan.plan_id,
                                "executed",
                                &payload,
                            );
                        }
                        *plan_state.write() = Some(plan);
                    } else {
                        *plan_state.write() = Some(plan);
                    }
                } else {
                    *plan_state.write() = None;
                }

                metrics.set_gauge(
                    "rebalance_plan_intents",
                    plan_state
                        .read()
                        .as_ref()
                        .map(|p| p.intents.len() as f64)
                        .unwrap_or(0.0),
                );

                tokio::time::sleep(Duration::from_secs(cfg_wallet.sync_secs.max(5))).await;
            }
        })
    }

    fn spawn_orderbook_loop(&self) -> JoinHandle<()> {
        let selected = self.state.selected_markets.clone();
        let latest = self.state.latest_books.clone();
        let market_history = self.state.market_history.clone();
        let metrics = self.metrics.clone();
        let poly = self.polymarket.clone();
        let storage = self.storage.clone();
        let loop_ms = self.cfg.engine.loop_ms.max(100);
        let max_books = self
            .cfg
            .risk
            .max_markets_quoted_simultaneously
            .saturating_mul(4)
            .max(4);

        tokio::spawn(async move {
            loop {
                let markets = selected.read().clone();
                let mut active: Vec<MarketSelection> = markets
                    .into_iter()
                    .filter(|m| {
                        matches!(
                            m.tier,
                            pt_core::MarketTier::TierA | pt_core::MarketTier::TierB
                        )
                    })
                    .take(max_books)
                    .collect();

                active.sort_by(|a, b| {
                    b.volume24h
                        .partial_cmp(&a.volume24h)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });

                for m in active {
                    match poly.get_best_book(&m.token_id_yes).await {
                        Ok(best) => {
                            let snap = MarketSnapshot {
                                market_id: m.market_id.clone(),
                                token_id: m.token_id_yes.clone(),
                                bid: best.best_bid,
                                ask: best.best_ask,
                                spread: best.spread,
                                liquidity: m.liquidity,
                                ts: DateTime::<Utc>::from_timestamp_millis(best.ts_ms)
                                    .unwrap_or_else(Utc::now),
                            };
                            latest.write().insert(m.market_id.clone(), snap.clone());
                            push_market_history(&market_history, &snap);
                            if let Err(e) = storage.insert_snapshot(&snap) {
                                error!(%e, "failed to persist snapshot");
                            }
                            metrics.inc_counter("book_poll_ok", 1.0);
                        }
                        Err(e) => {
                            warn!(market_id = %m.market_id, %e, "book poll failed");
                            metrics.inc_counter("book_poll_error", 1.0);
                        }
                    }
                }

                if let Err(e) = storage.roll_snapshots_if_due() {
                    error!(%e, "snapshot parquet roll failed");
                }

                tokio::time::sleep(Duration::from_millis(loop_ms)).await;
            }
        })
    }

    fn spawn_quote_loop(&self) -> JoinHandle<()> {
        let selected = self.state.selected_markets.clone();
        let latest = self.state.latest_books.clone();
        let shared_recent_executions = self.state.recent_executions.clone();
        let shared_execution_events = self.state.execution_events.clone();
        let shared_execution_costs = self.state.execution_costs.clone();
        let execution_policy = self.state.execution_policy.clone();
        let force_unwind = self.state.force_unwind.clone();
        let biases = self.state.fused_bias.clone();
        let inv = self.state.inventory_usd.clone();
        let metrics = self.metrics.clone();

        let risk = self.risk.clone();
        let poly_exec = self.poly_exec.clone();
        let hedger = self.hedger.clone();
        let storage = self.storage.clone();
        let quote_cfg = self.quote_cfg.clone();
        let loop_ms = self.cfg.engine.loop_ms.max(100);
        let risk_cfg = self.cfg.risk.clone();
        let hedge_cfg = self.cfg.venues.coinbase.clone();
        let mode = self.cfg.engine.mode.clone();

        tokio::spawn(async move {
            let mut simulator = PaperSimulator::default();
            let mut last_cancel_ms: HashMap<String, i64> = HashMap::new();
            let mut last_submit_ms: HashMap<String, i64> = HashMap::new();
            loop {
                let markets = selected.read().clone();
                let books = latest.read().clone();
                let bias_map = biases.read().clone();
                let policy = execution_policy.read().clone();
                let unwind_requested = *force_unwind.read();

                let mut active: Vec<MarketSelection> = markets
                    .into_iter()
                    .filter(|m| matches!(m.tier, pt_core::MarketTier::TierA))
                    .take(risk_cfg.max_markets_quoted_simultaneously)
                    .collect();

                active.sort_by(|a, b| {
                    b.volume24h
                        .partial_cmp(&a.volume24h)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });

                for market in active {
                    let Some(book) = books.get(&market.market_id) else {
                        continue;
                    };

                    let bias = *bias_map.get(&market.asset).unwrap_or(&0.0);
                    let bias_shift = bias * 0.005;
                    let inv_penalty = (*inv.read() / 100.0).clamp(-0.01, 0.01);

                    let costs = CostInputs {
                        rebate_est: if market.fees_enabled { 0.001 } else { 0.0 },
                        adverse_sel_est: 0.003,
                        hedge_cost_est: 0.001,
                        gas_amortized_est: 0.0005,
                    };

                    let Some(quote) = build_quote_intent(
                        &market,
                        book,
                        bias_shift,
                        inv_penalty,
                        &costs,
                        &quote_cfg,
                    ) else {
                        continue;
                    };

                    if let Err(err) = vector_gate(&quote, book, &policy.vectors) {
                        metrics.inc_counter("quote_vector_blocked", 1.0);
                        warn!(market_id = %market.market_id, %err, "quote blocked by vector gate");
                        continue;
                    }

                    let stale_ms = (Utc::now() - book.ts).num_milliseconds().max(0) as u64;
                    let decision = risk.evaluate_quote(&quote, stale_ms);
                    if !decision.allow {
                        metrics.inc_counter("quote_blocked", 1.0);
                        continue;
                    }

                    let now_ms = Utc::now().timestamp_millis();
                    if let Some(last_submit) = last_submit_ms.get(&market.market_id) {
                        if now_ms - *last_submit < policy.min_rest_ms as i64 {
                            metrics.inc_counter("quote_min_rest_skip", 1.0);
                            continue;
                        }
                    }

                    let can_cancel = last_cancel_ms
                        .get(&market.market_id)
                        .map(|v| now_ms - *v >= policy.cancel_replace_cooldown_ms as i64)
                        .unwrap_or(true);
                    if can_cancel {
                        if let Err(e) = poly_exec.cancel_stale(&market.market_id).await {
                            error!(market_id = %market.market_id, %e, "cancel stale quote failed");
                            metrics.inc_counter("quote_cancel_error", 1.0);
                            continue;
                        }
                        last_cancel_ms.insert(market.market_id.clone(), now_ms);
                    }

                    risk.reserve_quote_exposure(&quote);

                    let mut reports: Vec<ExecutionReport> = Vec::new();
                    let planned_event = ExecutionEvent {
                        order_id: format!("planned-{}-{}", quote.market_id, now_ms),
                        venue: Venue::Polymarket,
                        market_id: Some(quote.market_id.clone()),
                        product_id: None,
                        side: Side::Buy,
                        state: OrderLifecycleState::Planned,
                        qty: quote.bid_sz + quote.ask_sz,
                        price: (quote.bid_px + quote.ask_px) / 2.0,
                        ts: Utc::now(),
                        details: Some(format!(
                            "expected_net={} policy={:?}",
                            quote.expected_net, policy.mode
                        )),
                        reason_code: Some("planned_quote".to_string()),
                        unwind_flag: false,
                    };
                    push_execution_event(&shared_execution_events, planned_event.clone());
                    if let Err(e) = storage.insert_execution_event(&planned_event) {
                        error!(%e, "persist planned execution event failed");
                    }

                    match poly_exec.post_quote(&quote).await {
                        Ok(r) => {
                            reports.extend(r);
                            last_submit_ms.insert(market.market_id.clone(), now_ms);
                        }
                        Err(e) => {
                            error!(%e, "post quote failed");
                            metrics.inc_counter("quote_post_error", 1.0);
                        }
                    }

                    if matches!(mode, EngineMode::Paper | EngineMode::Replay) {
                        reports.extend(simulator.apply_quote(&quote, book));
                    }

                    for report in reports {
                        if let Err(e) = storage.insert_execution_report(&report) {
                            error!(%e, "persist report failed");
                        }
                        push_recent_execution(&shared_recent_executions, report.clone());

                        let lifecycle = status_to_lifecycle(&report.status);
                        let event = ExecutionEvent {
                            order_id: report.order_id.clone(),
                            venue: report.venue.clone(),
                            market_id: report.market_id.clone(),
                            product_id: report.market_id.clone(),
                            side: report.side.clone(),
                            state: lifecycle,
                            qty: report.filled_qty,
                            price: report.avg_px,
                            ts: Utc::now(),
                            details: report.details.clone(),
                            reason_code: None,
                            unwind_flag: false,
                        };
                        push_execution_event(&shared_execution_events, event.clone());
                        if let Err(e) = storage.insert_execution_event(&event) {
                            error!(%e, "persist execution event failed");
                        }

                        let reference_px = (book.bid + book.ask) / 2.0;
                        let fee_bps = default_fee_bps_for_venue(
                            &report.venue,
                            policy.polymarket_fees.maker_bps,
                            policy.coinbase_fees.taker_bps,
                        );
                        let rebate_bps = if matches!(report.venue, Venue::Polymarket) {
                            policy.polymarket_fees.rebate_bps_est
                        } else {
                            0.0
                        };
                        let cost = estimate_execution_cost(
                            &report.order_id,
                            &report,
                            reference_px,
                            fee_bps,
                            rebate_bps,
                        );
                        push_execution_cost(&shared_execution_costs, cost.clone());
                        if let Err(e) = storage.insert_execution_cost(&cost) {
                            error!(%e, "persist execution cost failed");
                        }

                        if let Err(e) = apply_fill_to_inventory(&inv, &report) {
                            error!(%e, "inventory apply failed");
                        }

                        if matches!(report.status, pt_core::ExecutionStatus::Filled) {
                            metrics.inc_counter("fills_total", 1.0);
                        }
                    }

                    // hedge if unhedged delta breaches threshold
                    let current_inv = *inv.read();
                    risk.update_unhedged_delta(current_inv);

                    if current_inv.abs() >= hedge_cfg.hedge_threshold_usd {
                        let risk_unwind =
                            unwind_requested || current_inv.abs() >= hedge_cfg.hedge_threshold_usd;
                        if policy.allow_taker_on_unwind_only && !risk_unwind {
                            metrics.inc_counter("hedge_blocked_taker_policy", 1.0);
                            continue;
                        }
                        let hedge_side = if current_inv > 0.0 {
                            Side::Sell
                        } else {
                            Side::Buy
                        };
                        let intent = HedgeIntent {
                            asset: market.asset.clone(),
                            side: hedge_side,
                            usd_notional: current_inv.abs().min(hedge_cfg.hedge_threshold_usd),
                            max_slippage_bps: hedge_cfg.hedge_max_slippage_bps,
                            risk_unwind,
                        };

                        match hedger.hedge(intent).await {
                            Ok(report) => {
                                if let Err(e) = storage.insert_execution_report(&report) {
                                    error!(%e, "persist hedge report failed");
                                }
                                push_recent_execution(&shared_recent_executions, report.clone());
                                // Assume hedge execution offsets delta by filled qty in USD terms.
                                if matches!(report.status, pt_core::ExecutionStatus::Filled) {
                                    let mut inv_lock = inv.write();
                                    if current_inv > 0.0 {
                                        *inv_lock -= report.filled_qty;
                                    } else {
                                        *inv_lock += report.filled_qty;
                                    }
                                }
                                metrics.inc_counter("hedge_ok", 1.0);
                            }
                            Err(e) => {
                                error!(%e, "hedge failed, entering safe mode");
                                risk.flatten_safe_mode();
                                metrics.inc_counter("hedge_error", 1.0);
                            }
                        }
                    }

                    risk.release_market_exposure(&market.market_id);
                }

                tokio::time::sleep(Duration::from_millis(loop_ms)).await;
            }
        })
    }

    fn spawn_watchdog_loop(&self) -> JoinHandle<()> {
        let kill_switch = self.state.kill_switch.clone();
        let risk_state = self.state.risk_state.clone();
        let risk = self.risk.clone();
        let storage = self.storage.clone();
        let metrics = self.metrics.clone();
        let interval = self.cfg.ops.risk_watchdog_ms.max(50);

        tokio::spawn(async move {
            loop {
                match *kill_switch.read() {
                    KillSwitchState::ManualHalt => risk.manual_halt(),
                    KillSwitchState::Running => {
                        let _ = risk.resume();
                    }
                    KillSwitchState::SafeMode => risk.flatten_safe_mode(),
                    KillSwitchState::AutoHalt => {
                        // AutoHalt is controlled by risk engine internals.
                    }
                }

                let snap = risk.snapshot();
                metrics.set_gauge("risk_daily_pnl", snap.daily_pnl);
                metrics.set_gauge("risk_open_notional", snap.open_notional);
                metrics.set_gauge("risk_unhedged_delta", snap.unhedged_delta);
                metrics.set_gauge(
                    "risk_killswitch_running",
                    if snap.killswitch == "Running" {
                        1.0
                    } else {
                        0.0
                    },
                );

                *risk_state.write() = snap.clone();
                if let Err(e) = storage.insert_risk_state(&snap) {
                    error!(%e, "persist risk state failed");
                }

                tokio::time::sleep(Duration::from_millis(interval)).await;
            }
        })
    }
}

fn apply_fill_to_inventory(inv: &Arc<RwLock<f64>>, report: &ExecutionReport) -> PtResult<()> {
    if !matches!(report.status, pt_core::ExecutionStatus::Filled) {
        return Ok(());
    }

    let notional = report.filled_qty * report.avg_px;
    let mut lock = inv.write();
    match report.side {
        Side::Buy => *lock += notional,
        Side::Sell => *lock -= notional,
    }
    Ok(())
}

fn push_recent_execution(
    recent_executions: &Arc<RwLock<Vec<ExecutionReport>>>,
    report: ExecutionReport,
) {
    const MAX_RECENT_EXECUTIONS: usize = 500;
    let mut lock = recent_executions.write();
    lock.push(report);
    if lock.len() > MAX_RECENT_EXECUTIONS {
        let overflow = lock.len() - MAX_RECENT_EXECUTIONS;
        lock.drain(0..overflow);
    }
}

fn push_market_history(
    market_history: &Arc<RwLock<HashMap<String, Vec<MarketHistoryPoint>>>>,
    snap: &MarketSnapshot,
) {
    const MAX_MARKET_HISTORY_POINTS: usize = 1200;
    let mut lock = market_history.write();
    let points = lock.entry(snap.market_id.clone()).or_default();
    points.push(MarketHistoryPoint {
        market_id: snap.market_id.clone(),
        mid: (snap.bid + snap.ask) / 2.0,
        spread: snap.spread,
        bid: snap.bid,
        ask: snap.ask,
        ts: snap.ts,
    });

    if points.len() > MAX_MARKET_HISTORY_POINTS {
        let overflow = points.len() - MAX_MARKET_HISTORY_POINTS;
        points.drain(0..overflow);
    }
}

fn push_execution_event(events: &Arc<RwLock<Vec<ExecutionEvent>>>, event: ExecutionEvent) {
    const MAX_EVENTS: usize = 1000;
    let mut lock = events.write();
    lock.push(event);
    if lock.len() > MAX_EVENTS {
        let overflow = lock.len() - MAX_EVENTS;
        lock.drain(0..overflow);
    }
}

fn push_execution_cost(
    costs: &Arc<RwLock<Vec<ExecutionCostAttribution>>>,
    cost: ExecutionCostAttribution,
) {
    const MAX_COSTS: usize = 1000;
    let mut lock = costs.write();
    lock.push(cost);
    if lock.len() > MAX_COSTS {
        let overflow = lock.len() - MAX_COSTS;
        lock.drain(0..overflow);
    }
}

fn push_coinbase_user_event(events: &Arc<RwLock<Vec<UserOrderEvent>>>, event: UserOrderEvent) {
    const MAX_USER_EVENTS: usize = 1000;
    let mut lock = events.write();
    lock.push(event);
    if lock.len() > MAX_USER_EVENTS {
        let overflow = lock.len() - MAX_USER_EVENTS;
        lock.drain(0..overflow);
    }
}

fn push_route_execution(plans: &Arc<RwLock<Vec<RouteExecutionPlan>>>, plan: RouteExecutionPlan) {
    const MAX_ROUTE_EXECUTIONS: usize = 400;
    let mut lock = plans.write();
    lock.push(plan);
    if lock.len() > MAX_ROUTE_EXECUTIONS {
        let overflow = lock.len() - MAX_ROUTE_EXECUTIONS;
        lock.drain(0..overflow);
    }
}

fn apply_coinbase_l2_update(
    orderbooks: &Arc<RwLock<HashMap<String, CoinbaseOrderBookState>>>,
    update: &CoinbaseL2Update,
) {
    let mut books = orderbooks.write();
    let state = books
        .entry(update.product_id.clone())
        .or_insert_with(|| CoinbaseOrderBookState {
            product_id: update.product_id.clone(),
            sequence_num: 0,
            bids: Vec::new(),
            asks: Vec::new(),
            last_event_ts: None,
        });

    if update.sequence_num <= state.sequence_num {
        return;
    }
    state.sequence_num = update.sequence_num;
    state.last_event_ts = Some(update.event_time);

    let side = update.side.to_ascii_lowercase();
    if side.contains("bid") {
        apply_level_update(
            &mut state.bids,
            update.price_level,
            update.new_quantity,
            true,
        );
    } else {
        apply_level_update(
            &mut state.asks,
            update.price_level,
            update.new_quantity,
            false,
        );
    }
}

fn apply_level_update(levels: &mut Vec<(f64, f64)>, price: f64, qty: f64, is_bid: bool) {
    if let Some(idx) = levels.iter().position(|(px, _)| (*px - price).abs() < 1e-9) {
        if qty <= 0.0 {
            levels.remove(idx);
        } else {
            levels[idx].1 = qty;
        }
    } else if qty > 0.0 {
        levels.push((price, qty));
    }

    if is_bid {
        levels.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    } else {
        levels.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    }
    if levels.len() > 200 {
        levels.truncate(200);
    }
}

fn status_to_lifecycle(status: &pt_core::ExecutionStatus) -> OrderLifecycleState {
    match status {
        pt_core::ExecutionStatus::New => OrderLifecycleState::Submitted,
        pt_core::ExecutionStatus::PartiallyFilled => OrderLifecycleState::PartiallyFilled,
        pt_core::ExecutionStatus::Filled => OrderLifecycleState::Filled,
        pt_core::ExecutionStatus::Canceled => OrderLifecycleState::Canceled,
        pt_core::ExecutionStatus::Rejected => OrderLifecycleState::Rejected,
        pt_core::ExecutionStatus::Error => OrderLifecycleState::Rejected,
    }
}

fn order_manager_slot(product_id: &str, side: &Side) -> String {
    let side_key = match side {
        Side::Buy => "buy",
        Side::Sell => "sell",
    };
    format!("{}:{}", product_id.to_ascii_uppercase(), side_key)
}

fn policy_from_config(cfg: &AppConfig) -> ExecutionPolicy {
    ExecutionPolicy {
        mode: match cfg.execution.mode {
            pt_core::ExecutionModeConfig::MakerFirst => ExecutionMode::MakerFirst,
        },
        allow_taker_on_unwind_only: cfg.execution.allow_taker_on_unwind_only,
        post_only: cfg.execution.post_only,
        cancel_replace_cooldown_ms: cfg.execution.cancel_replace_cooldown_ms,
        min_rest_ms: cfg.execution.min_rest_ms,
        stale_book_ms: cfg.execution.stale_book_ms,
        vectors: pt_core::EntryExitVector {
            entry_max_slippage_bps: cfg.execution.vectors.entry_max_slippage_bps,
            exit_max_slippage_bps: cfg.execution.vectors.exit_max_slippage_bps,
            entry_offset_bps: cfg.execution.vectors.entry_offset_bps,
            exit_offset_bps: cfg.execution.vectors.exit_offset_bps,
            max_cross_bps_unwind: cfg.execution.vectors.max_cross_bps_unwind,
        },
        coinbase_fees: pt_core::VenueFeeSchedule {
            maker_bps: cfg.execution.fees.coinbase.maker_bps,
            taker_bps: cfg.execution.fees.coinbase.taker_bps,
            rebate_bps_est: cfg.execution.fees.coinbase.rebate_bps_est,
        },
        polymarket_fees: pt_core::VenueFeeSchedule {
            maker_bps: cfg.execution.fees.polymarket.maker_bps,
            taker_bps: cfg.execution.fees.polymarket.taker_bps,
            rebate_bps_est: cfg.execution.fees.polymarket.rebate_bps_est,
        },
        edge_profiles: pt_core::EdgeProfile {
            maker_mm_spot_min_bps: cfg.execution.edge_profiles.maker_mm_spot_min_bps,
            conversion_cycle_min_bps: cfg.execution.edge_profiles.conversion_cycle_min_bps,
            position_reentry_min_bps: cfg.execution.edge_profiles.position_reentry_min_bps,
            per_asset_overrides_bps: cfg.execution.edge_profiles.per_asset_overrides_bps.clone(),
        },
    }
}

fn venue_capabilities_from_config(cfg: &AppConfig) -> Vec<VenueCapability> {
    let mut out = vec![
        VenueCapability {
            venue: Venue::Coinbase,
            supports_post_only: true,
            supports_amend: true,
            supports_fix: false,
            min_tick: 0.01,
            min_size: 0.00000001,
            fee_model: "tiered_maker_taker".to_string(),
        },
        VenueCapability {
            venue: Venue::Polymarket,
            supports_post_only: true,
            supports_amend: false,
            supports_fix: false,
            min_tick: cfg
                .venues
                .polymarket
                .filters
                .max_spread
                .min(0.01)
                .max(0.001),
            min_size: 1.0,
            fee_model: "maker_reward_taker_fee".to_string(),
        },
    ];

    if cfg.venues.kraken.enabled {
        out.push(VenueCapability {
            venue: Venue::Kraken,
            supports_post_only: true,
            supports_amend: false,
            supports_fix: false,
            min_tick: 0.01,
            min_size: 0.00000001,
            fee_model: "maker_taker".to_string(),
        });
    }
    if cfg.venues.gemini.enabled {
        out.push(VenueCapability {
            venue: Venue::Gemini,
            supports_post_only: true,
            supports_amend: false,
            supports_fix: false,
            min_tick: 0.01,
            min_size: 0.00000001,
            fee_model: "maker_taker".to_string(),
        });
    }
    out
}

fn should_attempt_coinbase_auth(cfg: &AppConfig) -> bool {
    let has_legacy = cfg
        .venues
        .coinbase
        .api_key
        .as_deref()
        .map(|v| !v.trim().is_empty())
        .unwrap_or(false)
        && cfg
            .venues
            .coinbase
            .api_secret
            .as_deref()
            .map(|v| !v.trim().is_empty())
            .unwrap_or(false);
    if has_legacy {
        return true;
    }
    cfg.venues
        .coinbase
        .auth
        .active_profile
        .as_deref()
        .map(|v| !v.trim().is_empty())
        .unwrap_or(false)
}

fn apply_portfolio_to_sqlite_path(path: &str, portfolio_id: &str) -> String {
    if path.contains("{portfolio_id}") {
        return path.replace("{portfolio_id}", portfolio_id);
    }
    if portfolio_id.eq_ignore_ascii_case("default") {
        return path.to_string();
    }
    if let Some(stripped) = path.strip_suffix(".sqlite") {
        return format!("{}_{}.sqlite", stripped, portfolio_id);
    }
    format!("{}_{}", path, portfolio_id)
}

fn apply_portfolio_to_parquet_dir(path: &str, portfolio_id: &str) -> String {
    if path.contains("{portfolio_id}") {
        return path.replace("{portfolio_id}", portfolio_id);
    }
    if portfolio_id.eq_ignore_ascii_case("default") {
        return path.to_string();
    }
    format!("{}/{}", path.trim_end_matches('/'), portfolio_id)
}

fn normalize_wallet_asset(asset: &str) -> String {
    if asset.eq_ignore_ascii_case("USDC") {
        "USD".to_string()
    } else {
        asset.to_ascii_uppercase()
    }
}

fn compute_allocation_drifts(
    balances: &[WalletBalance],
    targets: &pt_core::WalletTargetsConfig,
) -> Vec<AllocationDrift> {
    let mut by_asset: HashMap<String, f64> = HashMap::new();
    for b in balances {
        let k = normalize_wallet_asset(&b.asset);
        *by_asset.entry(k).or_insert(0.0) += b.usd_value.max(0.0);
    }

    let total_usd: f64 = by_asset.values().copied().sum::<f64>().max(0.0);
    if total_usd <= 0.0 {
        return Vec::new();
    }

    let target_map = vec![
        ("BTC".to_string(), targets.btc),
        ("ETH".to_string(), targets.eth),
        ("SOL".to_string(), targets.sol),
        ("XRP".to_string(), targets.xrp),
        ("USD".to_string(), targets.usd),
    ];

    let mut out = Vec::new();
    for (asset, target_weight) in target_map {
        let current_usd = *by_asset.get(&asset).unwrap_or(&0.0);
        let current_weight = current_usd / total_usd;
        let target_usd = total_usd * target_weight;
        let drift_usd = current_usd - target_usd;
        out.push(AllocationDrift {
            asset,
            current_weight,
            target_weight,
            drift_weight: current_weight - target_weight,
            current_usd,
            target_usd,
            drift_usd,
        });
    }

    out.sort_by(|a, b| {
        b.drift_usd
            .abs()
            .partial_cmp(&a.drift_usd.abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    out
}

async fn build_rebalance_plan(
    wallet_client: &CoinbaseWalletClient,
    drifts: &[AllocationDrift],
    wallet_cfg: &pt_core::WalletConfig,
    exec_cfg: &pt_core::ExecutionConfig,
) -> Option<RebalancePlan> {
    let mut intents = Vec::new();
    let mut drifts_sorted = drifts.to_vec();
    drifts_sorted.sort_by(|a, b| {
        b.drift_usd
            .abs()
            .partial_cmp(&a.drift_usd.abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for drift in drifts_sorted.iter() {
        if drift.asset == "USD" {
            continue;
        }
        let drift_pct = drift.drift_weight.abs();
        if drift_pct < wallet_cfg.rebalance.drift_trigger_pct {
            continue;
        }

        let usd_notional = drift
            .drift_usd
            .abs()
            .min(wallet_cfg.rebalance.max_order_usd);
        if usd_notional < wallet_cfg.rebalance.min_order_usd {
            continue;
        }
        if intents.len() >= wallet_cfg.rebalance.max_orders_per_cycle {
            break;
        }

        let asset = Asset::from_symbol(&drift.asset);
        let Some(product_id) = asset.as_product_id() else {
            continue;
        };
        let top = match wallet_client.fetch_top_of_book(product_id).await {
            Ok(v) => v,
            Err(_) => CoinbaseTopOfBook {
                product_id: product_id.to_string(),
                best_bid: 0.0,
                best_ask: 0.0,
            },
        };
        let side = if drift.drift_usd > 0.0 {
            Side::Sell
        } else {
            Side::Buy
        };
        let limit_price = match side {
            Side::Buy => {
                if top.best_bid > 0.0 {
                    top.best_bid * (1.0 - exec_cfg.vectors.entry_offset_bps / 10_000.0)
                } else {
                    0.0
                }
            }
            Side::Sell => {
                if top.best_ask > 0.0 {
                    top.best_ask * (1.0 + exec_cfg.vectors.exit_offset_bps / 10_000.0)
                } else {
                    0.0
                }
            }
        };
        if limit_price <= 0.0 {
            continue;
        }

        intents.push(RebalanceIntent {
            intent_id: format!("intent-{}-{}", drift.asset, Utc::now().timestamp_millis()),
            product_id: product_id.to_string(),
            asset,
            side,
            usd_notional,
            limit_price,
            max_slippage_bps: wallet_cfg.rebalance.max_slippage_bps,
        });
    }

    if intents.is_empty() {
        return None;
    }

    let total_drift_abs_usd = drifts.iter().map(|d| d.drift_usd.abs()).sum::<f64>();
    Some(RebalancePlan {
        plan_id: format!("plan-{}", Utc::now().timestamp_millis()),
        status: RebalancePlanStatus::Planned,
        intents,
        drifts: drifts.to_vec(),
        total_drift_abs_usd,
        created_ts: Utc::now(),
        expires_ts: Utc::now()
            + chrono::Duration::seconds(wallet_cfg.approval.token_ttl_secs as i64),
    })
}

async fn tradingview_webhook(
    State(state): State<TvWebhookState>,
    headers: HeaderMap,
    body: String,
) -> impl IntoResponse {
    if let Some(secret) = &state.secret {
        let provided = headers
            .get("x-tv-secret")
            .and_then(|h| h.to_str().ok())
            .unwrap_or_default();
        if provided != secret {
            state.metrics.inc_counter("tv_webhook_unauthorized", 1.0);
            return (axum::http::StatusCode::UNAUTHORIZED, "invalid secret");
        }
    }

    match parse_tradingview_bias(&body) {
        Some(bias) => {
            *state.tv_bias.write() = Some(bias);
            state.metrics.inc_counter("tv_webhook_ok", 1.0);
            (axum::http::StatusCode::OK, "ok")
        }
        None => {
            state.metrics.inc_counter("tv_webhook_parse_error", 1.0);
            (axum::http::StatusCode::BAD_REQUEST, "failed to parse")
        }
    }
}
