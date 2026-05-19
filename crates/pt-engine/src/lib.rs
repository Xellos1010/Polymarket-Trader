use arrow_array::{ArrayRef, Float64Array, RecordBatch, StringArray, TimestampMillisecondArray};
use arrow_schema::{DataType, Field, Schema, TimeUnit};
use axum::{
    extract::{ConnectInfo, State},
    http::HeaderMap,
    response::IntoResponse,
    routing::post,
    Router,
};
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use dashmap::DashMap;
use parking_lot::{Mutex, RwLock};
use parquet::arrow::ArrowWriter;
use pt_ai_agent::{
    summarize_positions, AgentProposal, MonitoringConfig, PositionInput, ProposalKind,
    ProposalQueue,
};
use pt_coinbase::{CoinbaseSpotHedger, HedgeExecutor, HedgeIntent, PaperCoinbaseHedger};
use pt_core::{
    AppConfig, Asset, EngineMode, ExecutionReport, KillSwitchState, MarketHistoryPoint,
    MarketSelection, MarketSnapshot, MetricsRegistry, PtError, PtResult, RiskState, Side,
    TradingViewBias,
};
use pt_dashboard::{router as dashboard_router, DashboardHandles, DashboardState};
use pt_market_discovery::MarketDiscoveryClient;
use pt_polymarket::{
    LivePolymarketConfig, LivePolymarketExecutor, PaperPolymarketExecutor, PolymarketClient,
    PolymarketExecution,
};
use pt_quote::{build_quote_intent, CostInputs, QuoteConfig};
use pt_replay::{load_replay_frames, PaperSimulator};
use pt_risk::RiskEngine;
use pt_signal::{parse_tradingview_bias, SignalFusionEngine};
use pt_wallet_intel::WalletIntelClient;
use rusqlite::{params, Connection};
use sha2::Sha256;
use std::{
    collections::HashMap,
    fs,
    net::{IpAddr, SocketAddr},
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::task::JoinHandle;
use tracing::{error, info, warn};

#[derive(Clone)]
struct SharedState {
    selected_markets: Arc<RwLock<Vec<MarketSelection>>>,
    latest_books: Arc<DashMap<String, MarketSnapshot>>,
    market_history: Arc<RwLock<HashMap<String, Vec<MarketHistoryPoint>>>>,
    recent_executions: Arc<RwLock<Vec<ExecutionReport>>>,
    fused_bias: Arc<RwLock<HashMap<Asset, f64>>>,
    tv_bias: Arc<RwLock<Option<TradingViewBias>>>,
    risk_state: Arc<RwLock<RiskState>>,
    kill_switch: Arc<RwLock<KillSwitchState>>,
    inventory_usd: Arc<RwLock<f64>>,
}

impl SharedState {
    fn new() -> Self {
        Self {
            selected_markets: Arc::new(RwLock::new(Vec::new())),
            latest_books: Arc::new(DashMap::new()),
            market_history: Arc::new(RwLock::new(HashMap::new())),
            recent_executions: Arc::new(RwLock::new(Vec::new())),
            fused_bias: Arc::new(RwLock::new(HashMap::new())),
            tv_bias: Arc::new(RwLock::new(None)),
            risk_state: Arc::new(RwLock::new(RiskState::default())),
            kill_switch: Arc::new(RwLock::new(KillSwitchState::Running)),
            inventory_usd: Arc::new(RwLock::new(0.0)),
        }
    }
}

struct Storage {
    conn: Mutex<Connection>,
    snapshot_roll_secs: u64,
    parquet_dir: String,
    snapshot_buffer: RwLock<Vec<MarketSnapshot>>,
    last_roll_ms: RwLock<i64>,
}

impl Storage {
    fn new(sqlite_path: &str, parquet_dir: &str, snapshot_roll_secs: u64) -> PtResult<Self> {
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
            ",
        )
        .map_err(|e| PtError::Io(e.to_string()))?;

        Ok(Self {
            conn: Mutex::new(conn),
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
                "INSERT INTO execution_reports (ts_ms, venue, order_id, market_id, side, status, filled_qty, avg_px, details) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    report.ts.timestamp_millis(),
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
    ip_allowlist: Vec<IpAddr>,
    seen_nonces: Arc<Mutex<HashMap<String, Instant>>>,
    nonce_window: Duration,
}

pub struct TradingEngine {
    cfg: AppConfig,
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
    storage: Arc<Storage>,
    proposal_queue: Arc<ProposalQueue>,
}

impl TradingEngine {
    pub fn new(cfg: AppConfig) -> PtResult<Self> {
        let metrics = Arc::new(MetricsRegistry::default());
        let state = SharedState::new();

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

        let risk = Arc::new(RiskEngine::new(
            cfg.risk.clone(),
            cfg.risk.deployed_capital_usd,
        ));

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
                if cfg.venues.coinbase.api_key.is_none() || cfg.venues.coinbase.api_secret.is_none()
                {
                    return Err(PtError::Config(
                        "live mode requires venues.coinbase.api_key and venues.coinbase.api_secret"
                            .to_string(),
                    ));
                }
                Arc::new(CoinbaseSpotHedger::new(
                    cfg.venues.coinbase.api_base.clone(),
                    cfg.venues.coinbase.api_key.clone(),
                    cfg.venues.coinbase.api_secret.clone(),
                ))
            }
        };

        let storage = Arc::new(Storage::new(
            &cfg.storage.sqlite_path,
            &cfg.storage.parquet_dir,
            cfg.storage.snapshot_roll_secs,
        )?);

        Ok(Self {
            cfg,
            metrics,
            state,
            market_discovery,
            polymarket,
            wallet_intel,
            signal_fusion,
            risk,
            quote_cfg: QuoteConfig::default(),
            poly_exec,
            hedger,
            storage,
            proposal_queue: Arc::new(ProposalQueue::new()),
        })
    }

    pub async fn run(&self) -> PtResult<()> {
        info!(mode = ?self.cfg.engine.mode, "starting trading engine");

        if let EngineMode::Replay = self.cfg.engine.mode {
            return self.run_replay_mode().await;
        }

        let mut tasks: Vec<JoinHandle<()>> = Vec::new();

        tasks.push(self.spawn_dashboard_server());
        if self.cfg.signals.tradingview.enabled {
            tasks.push(self.spawn_tradingview_server());
        }

        tasks.push(self.spawn_market_refresh_loop());
        tasks.push(self.spawn_wallet_refresh_loop());
        tasks.push(self.spawn_orderbook_loop());
        tasks.push(self.spawn_quote_loop());
        tasks.push(self.spawn_watchdog_loop());
        if self.cfg.agent.enabled {
            tasks.push(self.spawn_ai_monitor_loop());
        }

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
            };
            let hedge_report = self.hedger.hedge(intent).await?;
            self.storage.insert_execution_report(&hedge_report)?;
            push_recent_execution(&self.state.recent_executions, hedge_report);
        }

        Ok(())
    }

    fn spawn_dashboard_server(&self) -> JoinHandle<()> {
        let bind = self.cfg.ops.dashboard_bind.clone();
        let state = DashboardState::new(DashboardHandles {
            metrics: self.metrics.clone(),
            risk_state: self.state.risk_state.clone(),
            kill_switch: self.state.kill_switch.clone(),
            latest_books: self.state.latest_books.clone(),
            market_history: self.state.market_history.clone(),
            recent_executions: self.state.recent_executions.clone(),
            fused_bias: self.state.fused_bias.clone(),
            inventory_usd: self.state.inventory_usd.clone(),
            coinbase: Default::default(),
            proposal_queue: ProposalQueue::new(),
            last_backtest: Arc::new(parking_lot::RwLock::new(None)),
            tsdb: None,
        });

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
        let ip_allowlist: Vec<IpAddr> = self
            .cfg
            .signals
            .tradingview
            .ip_allowlist
            .iter()
            .filter_map(|s| s.parse().ok())
            .collect();
        let nonce_window = Duration::from_secs(self.cfg.signals.tradingview.nonce_window_secs);
        let tv_state = TvWebhookState {
            tv_bias: self.state.tv_bias.clone(),
            secret: self.cfg.signals.tradingview.endpoint_secret.clone(),
            metrics: self.metrics.clone(),
            ip_allowlist,
            seen_nonces: Arc::new(Mutex::new(HashMap::new())),
            nonce_window,
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

            if let Err(e) = axum::serve(
                listener,
                app.into_make_service_with_connect_info::<SocketAddr>(),
            )
            .await
            {
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
        let refresh = self.cfg.ops.wallet_refresh_secs.max(10);

        tokio::spawn(async move {
            loop {
                let start = Instant::now();
                match wallet.compute_wallet_biases().await {
                    Ok(wallet_signals) => {
                        let tv = tv_bias.read().clone();
                        let map = fusion.fuse(&wallet_signals, tv);
                        *fused_bias.write() = map;
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
                            latest.insert(m.market_id.clone(), snap.clone());
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
            loop {
                let markets = selected.read().clone();
                let bias_map = biases.read().clone();

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
                    let Some(book) = latest.get(&market.market_id) else {
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
                        &*book,
                        bias_shift,
                        inv_penalty,
                        &costs,
                        &quote_cfg,
                    ) else {
                        continue;
                    };

                    let stale_ms = (Utc::now() - book.ts).num_milliseconds().max(0) as u64;
                    let decision = risk.evaluate_quote(&quote, stale_ms);
                    if !decision.allow {
                        metrics.inc_counter("quote_blocked", 1.0);
                        continue;
                    }

                    if let Err(e) = poly_exec.cancel_stale(&market.market_id).await {
                        error!(market_id = %market.market_id, %e, "cancel stale quote failed");
                        metrics.inc_counter("quote_cancel_error", 1.0);
                        continue;
                    }

                    risk.reserve_quote_exposure(&quote);

                    let mut reports: Vec<ExecutionReport> = Vec::new();
                    match poly_exec.post_quote(&quote).await {
                        Ok(r) => reports.extend(r),
                        Err(e) => {
                            error!(%e, "post quote failed");
                            metrics.inc_counter("quote_post_error", 1.0);
                        }
                    }

                    if matches!(mode, EngineMode::Paper | EngineMode::Replay) {
                        reports.extend(simulator.apply_quote(&quote, &*book));
                    }

                    for report in reports {
                        if let Err(e) = storage.insert_execution_report(&report) {
                            error!(%e, "persist report failed");
                        }
                        push_recent_execution(&shared_recent_executions, report.clone());

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

    fn spawn_ai_monitor_loop(&self) -> JoinHandle<()> {
        let state = self.state.latest_books.clone();
        let risk_state = self.state.risk_state.clone();
        let queue = self.proposal_queue.clone();
        let max_pending = 50usize;
        let interval_secs = self.cfg.agent.monitor_interval_secs.max(60);

        tokio::spawn(async move {
            info!("ai monitor loop started (interval={}s)", interval_secs);
            loop {
                tokio::time::sleep(Duration::from_secs(interval_secs)).await;

                let snap = risk_state.read().clone();
                let book_len = state.len().max(1);
                let inputs: Vec<PositionInput> = state
                    .iter()
                    .map(|e| PositionInput {
                        market_id: e.value().market_id.clone(),
                        position_usd: snap.open_notional / book_len as f64,
                        pnl_usd: snap.daily_pnl / book_len as f64,
                        age_secs: 0,
                    })
                    .collect();

                let summary = summarize_positions(&inputs, &MonitoringConfig::default());

                if summary.anomalous_count > 0 {
                    let notes = summary.notes.join("; ");
                    let proposal = AgentProposal::new(
                        ProposalKind::Alert {
                            message: format!(
                                "AI monitor: {} anomalous position(s). {}",
                                summary.anomalous_count, notes
                            ),
                        },
                        format!(
                            "{} position(s) outside normal bounds; operator review recommended.",
                            summary.anomalous_count
                        ),
                        serde_json::json!({
                            "anomalous_count": summary.anomalous_count,
                            "total_exposure_usd": summary.total_exposure_usd,
                            "total_pnl_usd": summary.total_pnl_usd,
                            "notes": summary.notes,
                        }),
                        "ai-monitor",
                    );
                    if let Err(e) = queue.push(proposal, max_pending) {
                        warn!("ai monitor: proposal queue full or error: {e}");
                    }
                }

                info!(
                    anomalous = summary.anomalous_count,
                    exposure_usd = summary.total_exposure_usd,
                    pnl_usd = summary.total_pnl_usd,
                    "ai monitor cycle complete"
                );
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

async fn tradingview_webhook(
    State(state): State<TvWebhookState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    body: String,
) -> impl IntoResponse {
    // #84: IP allowlist — reject if configured and peer not in list.
    if !state.ip_allowlist.is_empty() && !state.ip_allowlist.contains(&peer.ip()) {
        state.metrics.inc_counter("tv_webhook_ip_rejected", 1.0);
        return (axum::http::StatusCode::UNAUTHORIZED, "ip not allowed");
    }

    // Auth: prefer HMAC-SHA256 signature over plain secret when both header and secret present.
    if let Some(secret) = &state.secret {
        let sig_header = headers.get("x-tv-signature").and_then(|h| h.to_str().ok());

        if let Some(sig_hex) = sig_header {
            // #85: HMAC-SHA256 verification.
            let sig_bytes = match hex::decode(sig_hex) {
                Ok(b) => b,
                Err(_) => {
                    state.metrics.inc_counter("tv_webhook_unauthorized", 1.0);
                    return (
                        axum::http::StatusCode::UNAUTHORIZED,
                        "invalid signature encoding",
                    );
                }
            };
            let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
                .expect("HMAC accepts any key length");
            mac.update(body.as_bytes());
            if mac.verify_slice(&sig_bytes).is_err() {
                state.metrics.inc_counter("tv_webhook_unauthorized", 1.0);
                return (axum::http::StatusCode::UNAUTHORIZED, "invalid signature");
            }
        } else {
            // Fallback: plain x-tv-secret header comparison.
            let provided = headers
                .get("x-tv-secret")
                .and_then(|h| h.to_str().ok())
                .unwrap_or_default();
            if provided != secret {
                state.metrics.inc_counter("tv_webhook_unauthorized", 1.0);
                return (axum::http::StatusCode::UNAUTHORIZED, "invalid secret");
            }
        }
    }

    // #85: Nonce replay protection — reject duplicate nonces within the window.
    if !state.nonce_window.is_zero() {
        if let Some(nonce) = headers.get("x-tv-nonce").and_then(|h| h.to_str().ok()) {
            let now = Instant::now();
            let mut nonces = state.seen_nonces.lock();
            // Prune expired entries.
            nonces.retain(|_, seen_at| now.duration_since(*seen_at) < state.nonce_window);
            if nonces.contains_key(nonce) {
                state.metrics.inc_counter("tv_webhook_replay_rejected", 1.0);
                return (axum::http::StatusCode::UNAUTHORIZED, "replay detected");
            }
            nonces.insert(nonce.to_owned(), now);
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::connect_info::MockConnectInfo;
    use http_body_util::BodyExt;
    use std::time::{SystemTime, UNIX_EPOCH};
    use tower::ServiceExt;

    fn make_webhook_state(
        secret: Option<&str>,
        ip_allowlist: Vec<&str>,
        nonce_window_secs: u64,
    ) -> TvWebhookState {
        TvWebhookState {
            tv_bias: Arc::new(RwLock::new(None)),
            secret: secret.map(|s| s.to_owned()),
            metrics: Arc::new(MetricsRegistry::default()),
            ip_allowlist: ip_allowlist.iter().filter_map(|s| s.parse().ok()).collect(),
            seen_nonces: Arc::new(Mutex::new(HashMap::new())),
            nonce_window: Duration::from_secs(nonce_window_secs),
        }
    }

    fn webhook_app(state: TvWebhookState, peer: SocketAddr) -> axum::Router {
        Router::new()
            .route("/tradingview", post(tradingview_webhook))
            .with_state(state)
            .layer(MockConnectInfo(peer))
    }

    async fn post_webhook(
        app: axum::Router,
        headers: Vec<(&str, &str)>,
        body: &str,
    ) -> (u16, String) {
        let mut req = axum::http::Request::builder()
            .method("POST")
            .uri("/tradingview");
        for (k, v) in headers {
            req = req.header(k, v);
        }
        let req = req.body(axum::body::Body::from(body.to_owned())).unwrap();
        let resp = app.oneshot(req).await.unwrap();
        let status = resp.status().as_u16();
        let bytes = resp.into_body().collect().await.unwrap().to_bytes();
        (status, String::from_utf8_lossy(&bytes).to_string())
    }

    fn hmac_hex(secret: &str, body: &str) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(body.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    const VALID_BODY: &str = r#"{"order_action":"buy","contracts":"0.5","ticker":"BTC-USD"}"#;
    const PEER: &str = "127.0.0.1:12345";

    #[tokio::test]
    async fn webhook_no_secret_accepts_any_request() {
        let state = make_webhook_state(None, vec![], 0);
        let app = webhook_app(state, PEER.parse().unwrap());
        let (status, _) = post_webhook(app, vec![], VALID_BODY).await;
        assert_eq!(status, 200);
    }

    #[tokio::test]
    async fn webhook_plain_secret_accepted() {
        let state = make_webhook_state(Some("mysecret"), vec![], 0);
        let app = webhook_app(state, PEER.parse().unwrap());
        let (status, _) = post_webhook(app, vec![("x-tv-secret", "mysecret")], VALID_BODY).await;
        assert_eq!(status, 200);
    }

    #[tokio::test]
    async fn webhook_plain_secret_rejected() {
        let state = make_webhook_state(Some("mysecret"), vec![], 0);
        let app = webhook_app(state, PEER.parse().unwrap());
        let (status, _) = post_webhook(app, vec![("x-tv-secret", "wrong")], VALID_BODY).await;
        assert_eq!(status, 401);
    }

    #[tokio::test]
    async fn webhook_hmac_signature_accepted() {
        let body = VALID_BODY;
        let sig = hmac_hex("mysecret", body);
        let state = make_webhook_state(Some("mysecret"), vec![], 0);
        let app = webhook_app(state, PEER.parse().unwrap());
        let (status, _) = post_webhook(app, vec![("x-tv-signature", &sig)], body).await;
        assert_eq!(status, 200);
    }

    #[tokio::test]
    async fn webhook_hmac_signature_rejected_wrong_key() {
        let body = VALID_BODY;
        let sig = hmac_hex("wrongkey", body);
        let state = make_webhook_state(Some("mysecret"), vec![], 0);
        let app = webhook_app(state, PEER.parse().unwrap());
        let (status, _) = post_webhook(app, vec![("x-tv-signature", &sig)], body).await;
        assert_eq!(status, 401);
    }

    #[tokio::test]
    async fn webhook_hmac_invalid_hex_encoding_rejected() {
        let state = make_webhook_state(Some("mysecret"), vec![], 0);
        let app = webhook_app(state, PEER.parse().unwrap());
        let (status, _) =
            post_webhook(app, vec![("x-tv-signature", "not-valid-hex!")], VALID_BODY).await;
        assert_eq!(status, 401);
    }

    #[tokio::test]
    async fn webhook_nonce_replay_rejected() {
        let state = make_webhook_state(None, vec![], 300);
        let nonce = "unique-nonce-abc";
        // First request accepted.
        let app = webhook_app(state.clone(), PEER.parse().unwrap());
        let (status, _) = post_webhook(app, vec![("x-tv-nonce", nonce)], VALID_BODY).await;
        assert_eq!(status, 200);
        // Second request with same nonce rejected.
        let app2 = webhook_app(state, PEER.parse().unwrap());
        let (status2, _) = post_webhook(app2, vec![("x-tv-nonce", nonce)], VALID_BODY).await;
        assert_eq!(status2, 401);
    }

    #[tokio::test]
    async fn webhook_burst_with_unique_nonces_stays_reliable() {
        let state = make_webhook_state(None, vec!["127.0.0.1"], 300);
        for i in 0..50 {
            let nonce = format!("burst-nonce-{i}");
            let app = webhook_app(state.clone(), "127.0.0.1:9999".parse().unwrap());
            let (status, _) = post_webhook(app, vec![("x-tv-nonce", &nonce)], VALID_BODY).await;
            assert_eq!(status, 200, "burst request {i} failed");
        }
        assert_eq!(state.metrics.get_counter("tv_webhook_ok"), 50.0);
        assert_eq!(state.metrics.get_counter("tv_webhook_replay_rejected"), 0.0);
    }

    #[tokio::test]
    async fn webhook_ip_allowlist_accepted() {
        let state = make_webhook_state(None, vec!["127.0.0.1"], 0);
        let app = webhook_app(state, "127.0.0.1:9999".parse().unwrap());
        let (status, _) = post_webhook(app, vec![], VALID_BODY).await;
        assert_eq!(status, 200);
    }

    #[tokio::test]
    async fn webhook_ip_allowlist_rejected() {
        let state = make_webhook_state(None, vec!["10.0.0.1"], 0);
        let app = webhook_app(state, "192.168.1.50:9999".parse().unwrap());
        let (status, _) = post_webhook(app, vec![], VALID_BODY).await;
        assert_eq!(status, 401);
    }

    fn test_config() -> AppConfig {
        let raw = include_str!("../../../config/config.example.toml");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let base = std::env::temp_dir().join(format!("pt-engine-risk-{nonce}"));
        std::fs::create_dir_all(&base).expect("create temp base");
        let config_path = base.join("config.toml");
        std::fs::write(&config_path, raw).expect("write temp config");
        let mut cfg = AppConfig::from_file(&config_path).expect("load config example");
        cfg.storage.sqlite_path = base.join("engine.sqlite").display().to_string();
        cfg.storage.parquet_dir = base.join("parquet").display().to_string();
        cfg
    }

    #[tokio::test]
    async fn engine_startup_uses_configured_deployed_capital() {
        let cfg = test_config();
        let expected_max_daily_loss = cfg.risk.deployed_capital_usd * cfg.risk.daily_loss_limit_pct;
        let engine = TradingEngine::new(cfg).expect("engine");

        assert_eq!(
            engine.risk.snapshot().max_daily_loss,
            expected_max_daily_loss
        );

        let watchdog = engine.spawn_watchdog_loop();
        tokio::time::sleep(Duration::from_millis(75)).await;

        let shared_max_daily_loss = engine.state.risk_state.read().max_daily_loss;
        watchdog.abort();

        assert_eq!(shared_max_daily_loss, expected_max_daily_loss);
    }
}
