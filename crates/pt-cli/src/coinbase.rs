use chrono::Utc;
use parking_lot::RwLock;
use pt_coinbase::{
    CoinbaseAdvancedTradeClient, CoinbaseAdvancedTradeOrderRequest, CoinbaseBookLevel,
    CoinbaseCandle, CoinbaseMarketTrade, CoinbaseOrderSummary, CoinbasePriceBook, CoinbaseProduct,
};
use pt_core::{
    AppConfig, EngineMode, ExecutionReport, ExecutionStatus, Instrument, KillSwitchState,
    LiveArmState, MarketHistoryPoint, MarketSnapshot, MetricsRegistry, OrderRoute,
    ProductDetailView, ProductId, ProductStrategyConfig, ProductStrategyConfigView, RiskState,
    ScannerRow, Side, StrategyLabImportSummary, StrategyVector, TradeAction, TradingEligibility,
    Venue, WorkstationOrder, WorkstationOrderStatus, WorkstationProduct,
};
use pt_dashboard::{
    router as dashboard_router, CoinbaseDashboardHandles, DashboardHandles, DashboardState,
};
use std::{
    collections::{HashMap, HashSet},
    net::TcpListener,
    sync::Arc,
    time::Duration,
};
use tracing::{error, info, warn};

#[derive(Default)]
struct ProductTelemetry {
    last_direction: i8,
    persistence: u64,
    last_auto_submit_ms: i64,
}

#[derive(Default)]
struct RuntimeMeta {
    products: HashMap<String, ProductTelemetry>,
    total_order_attempts: u64,
    total_order_rejects: u64,
    taker_budget_used_usd: f64,
}

pub async fn coinbase_up(config_path: &str, mode_override: Option<&str>) -> Result<(), String> {
    let cfg = AppConfig::from_file(config_path).map_err(|e| e.to_string())?;
    let runtime = CoinbaseWorkstationRuntime::new(cfg, mode_override)?;
    runtime.run().await
}

pub async fn coinbase_preflight(
    config_path: &str,
    mode_override: Option<&str>,
    _timeout_ms: u64,
) -> Result<(), String> {
    let cfg = AppConfig::from_file(config_path).map_err(|e| e.to_string())?;
    let mode = resolved_mode(mode_override, &cfg.engine.mode)?;
    let client = CoinbaseAdvancedTradeClient::new(
        cfg.venues.coinbase.api_base.clone(),
        cfg.venues.coinbase.api_key.clone(),
        cfg.venues.coinbase.api_secret.clone(),
    );

    let mut checks = vec![
        check_bind("ops.dashboard_bind", &cfg.ops.dashboard_bind),
        check_nonempty("venues.coinbase.api_base", &cfg.venues.coinbase.api_base),
        check_nonempty("venues.coinbase.ws.url", &cfg.venues.coinbase.ws.url),
        check_pass("mode", &mode),
    ];

    match client.list_public_products(8).await {
        Ok(products) if !products.is_empty() => checks.push(check_pass(
            "public products",
            &format!("fetched {} products", products.len()),
        )),
        Ok(_) => checks.push(check_fail("public products", "no products returned")),
        Err(e) => checks.push(check_fail("public products", &e.to_string())),
    }

    if mode == "live" {
        checks.push(if client.credentials_available() {
            check_pass("coinbase credentials", "loaded")
        } else {
            check_fail("coinbase credentials", "missing api_key/api_secret")
        });

        if client.credentials_available() {
            let sample_product = cfg
                .strategy
                .products
                .first()
                .map(|row| row.product_id.clone())
                .or_else(|| cfg.venues.coinbase.products.first().cloned())
                .unwrap_or_else(|| "BTC-USD".to_string());
            let preview_request = CoinbaseAdvancedTradeOrderRequest {
                product_id: sample_product.clone(),
                side: Side::Buy,
                route: OrderRoute::Maker,
                base_size: 0.0001,
                quote_size: Some(10.0),
                limit_price: Some(1.0),
                post_only: true,
                preview_id: None,
            };
            match client.preview_order(&preview_request).await {
                Ok(preview) if preview.success => checks.push(check_pass(
                    "orders/preview",
                    &format!("preview ok for {sample_product}"),
                )),
                Ok(preview) => checks.push(check_fail(
                    "orders/preview",
                    &format!("preview rejected {:?}", preview.raw_status),
                )),
                Err(e) => checks.push(check_fail("orders/preview", &e.to_string())),
            }
        }
    }

    println!("coinbase preflight report ({})", config_path);
    let mut failures = 0usize;
    for check in checks {
        let status = if check.ok { "PASS" } else { "FAIL" };
        if !check.ok {
            failures += 1;
        }
        println!("[{}] {}: {}", status, check.name, check.detail);
    }
    if failures > 0 {
        return Err(format!(
            "coinbase preflight blocked by {} failing checks",
            failures
        ));
    }
    Ok(())
}

struct PreflightRow {
    ok: bool,
    name: String,
    detail: String,
}

fn check_pass(name: &str, detail: &str) -> PreflightRow {
    PreflightRow {
        ok: true,
        name: name.to_string(),
        detail: detail.to_string(),
    }
}

fn check_fail(name: &str, detail: &str) -> PreflightRow {
    PreflightRow {
        ok: false,
        name: name.to_string(),
        detail: detail.to_string(),
    }
}

fn check_nonempty(name: &str, value: &str) -> PreflightRow {
    if value.trim().is_empty() {
        check_fail(name, "missing")
    } else {
        check_pass(name, "set")
    }
}

fn check_bind(name: &str, bind_addr: &str) -> PreflightRow {
    match TcpListener::bind(bind_addr) {
        Ok(listener) => {
            drop(listener);
            check_pass(name, "bind available")
        }
        Err(e) => check_fail(name, &format!("bind failed: {e}")),
    }
}

struct CoinbaseWorkstationRuntime {
    cfg: AppConfig,
    state: DashboardState,
    client: CoinbaseAdvancedTradeClient,
    meta: Arc<RwLock<RuntimeMeta>>,
}

impl CoinbaseWorkstationRuntime {
    fn new(cfg: AppConfig, mode_override: Option<&str>) -> Result<Self, String> {
        let mode = resolved_mode(mode_override, &cfg.engine.mode)?;
        let metrics = Arc::new(MetricsRegistry::default());
        let risk_state = Arc::new(RwLock::new(RiskState {
            killswitch: "Running".to_string(),
            daily_pnl: 0.0,
            max_daily_loss: cfg.risk.daily_loss_limit_pct,
            open_notional: 0.0,
            unhedged_delta: 0.0,
            open_markets: 0,
            stale_books: 0,
            last_update_ms: Utc::now().timestamp_millis(),
        }));
        let kill_switch = Arc::new(RwLock::new(KillSwitchState::Running));
        let coinbase_handles = CoinbaseDashboardHandles {
            mode: Arc::new(RwLock::new(mode)),
            live_arm: Arc::new(RwLock::new(LiveArmState::default())),
            ..CoinbaseDashboardHandles::default()
        };
        let state = DashboardState::new(DashboardHandles {
            metrics,
            risk_state,
            kill_switch,
            latest_books: Arc::new(RwLock::new(HashMap::new())),
            market_history: Arc::new(RwLock::new(HashMap::new())),
            recent_executions: Arc::new(RwLock::new(Vec::new())),
            fused_bias: Arc::new(RwLock::new(HashMap::new())),
            inventory_usd: Arc::new(RwLock::new(0.0)),
            coinbase: coinbase_handles,
        });
        let client = CoinbaseAdvancedTradeClient::new(
            cfg.venues.coinbase.api_base.clone(),
            cfg.venues.coinbase.api_key.clone(),
            cfg.venues.coinbase.api_secret.clone(),
        );

        Ok(Self {
            cfg,
            state,
            client,
            meta: Arc::new(RwLock::new(RuntimeMeta::default())),
        })
    }

    async fn run(self) -> Result<(), String> {
        let runtime = Arc::new(self);
        let tasks = vec![
            runtime.clone().spawn_dashboard_server(),
            runtime.clone().spawn_product_refresh_loop(),
            runtime.clone().spawn_scanner_loop(),
            runtime.clone().spawn_order_loop(),
            runtime.clone().spawn_live_order_sync_loop(),
        ];

        info!(
            bind = %runtime.cfg.ops.dashboard_bind,
            mode = %runtime.state.coinbase.mode.read().clone(),
            "starting coinbase workstation"
        );

        tokio::signal::ctrl_c()
            .await
            .map_err(|e| format!("shutdown signal failed: {e}"))?;

        for task in tasks {
            task.abort();
        }
        Ok(())
    }

    fn spawn_dashboard_server(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let app = dashboard_router(self.state.clone());
            let addr: std::net::SocketAddr = match self.cfg.ops.dashboard_bind.parse() {
                Ok(addr) => addr,
                Err(e) => {
                    error!(%e, "invalid dashboard bind address");
                    return;
                }
            };

            let listener = match tokio::net::TcpListener::bind(addr).await {
                Ok(listener) => listener,
                Err(e) => {
                    error!(%e, "failed to bind coinbase workstation");
                    return;
                }
            };

            if let Err(e) = axum::serve(listener, app).await {
                error!(%e, "coinbase workstation server failed");
            }
        })
    }

    fn spawn_product_refresh_loop(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let refresh = Duration::from_secs(self.cfg.scanner.product_refresh_secs.max(15));
            loop {
                match self
                    .client
                    .list_public_products(self.cfg.scanner.max_products * 3)
                    .await
                {
                    Ok(products) => {
                        let views = self.filtered_products(&products);
                        let strategies = views
                            .iter()
                            .map(|product| self.strategy_view_for_product(product))
                            .collect::<Vec<_>>();
                        *self.state.coinbase.products.write() = views;
                        *self.state.coinbase.strategies.write() = strategies;
                    }
                    Err(e) => {
                        warn!(%e, "coinbase product refresh failed");
                    }
                }

                tokio::time::sleep(refresh).await;
            }
        })
    }

    fn spawn_scanner_loop(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let refresh = Duration::from_millis(self.cfg.scanner.refresh_ms.max(500));
            loop {
                let products = self.state.coinbase.products.read().clone();
                let imports = self.state.coinbase.imports.read().clone();
                let mut detail_map = HashMap::new();
                let mut rows = Vec::new();

                for product in products
                    .iter()
                    .take(self.cfg.scanner.top_n.min(self.cfg.scanner.max_products))
                {
                    match self.scan_product(product, &imports).await {
                        Ok((row, detail, latest_book)) => {
                            detail_map.insert(product.product_id.as_str().to_string(), detail);
                            rows.push(row);
                            if let Some(snap) = latest_book {
                                self.state
                                    .latest_books
                                    .write()
                                    .insert(snap.market_id.clone(), snap.clone());
                                push_market_history(&self.state.market_history, &snap);
                            }
                        }
                        Err(e) => {
                            warn!(product_id = %product.product_id.as_str(), %e, "scan product failed");
                            if e.contains("429") {
                                tokio::time::sleep(Duration::from_millis(750)).await;
                            }
                        }
                    }
                    tokio::time::sleep(Duration::from_millis(175)).await;
                }

                rows.sort_by(|a, b| {
                    b.score
                        .partial_cmp(&a.score)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
                *self.state.coinbase.product_details.write() = detail_map;
                *self.state.coinbase.scanner.write() = rows.clone();
                self.refresh_risk_state(&rows);
                tokio::time::sleep(refresh).await;
            }
        })
    }

    fn spawn_order_loop(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let refresh = Duration::from_millis(self.cfg.execution.min_rest_ms.max(200));
            loop {
                self.process_cancel_requests().await;
                self.process_draft_orders().await;
                self.advance_paper_orders();
                self.maybe_submit_auto_orders();
                self.enforce_live_guards();
                self.refresh_order_risk();
                tokio::time::sleep(refresh).await;
            }
        })
    }

    fn spawn_live_order_sync_loop(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                let mode = self.state.coinbase.mode.read().clone();
                let armed = self.state.coinbase.live_arm.read().armed;
                if mode == "live" && armed && self.client.credentials_available() {
                    match self.client.list_orders(None).await {
                        Ok(orders) => self.merge_live_orders(&orders),
                        Err(e) => warn!(%e, "coinbase live order sync failed"),
                    }
                }
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        })
    }

    fn filtered_products(&self, products: &[CoinbaseProduct]) -> Vec<WorkstationProduct> {
        let allowed_quotes: HashSet<String> = self
            .cfg
            .scanner
            .quote_currencies
            .iter()
            .map(|row| row.to_ascii_uppercase())
            .collect();

        let mut out = products
            .iter()
            .filter_map(|product| {
                let quote = product
                    .quote_currency_id
                    .clone()
                    .unwrap_or_default()
                    .to_ascii_uppercase();
                if !allowed_quotes.is_empty() && !allowed_quotes.contains(&quote) {
                    return None;
                }

                let instrument = instrument_for_product(product);
                if matches!(instrument, Instrument::Derivative)
                    && !self.cfg.scanner.include_derivatives
                {
                    return None;
                }

                let live_tradable = matches!(instrument, Instrument::Spot)
                    && !product.trading_disabled
                    && !product.cancel_only;

                Some(WorkstationProduct {
                    product_id: ProductId::from(product.product_id.clone()),
                    instrument: Some(instrument.clone()),
                    base_currency: product.base_currency_id.clone().unwrap_or_default(),
                    quote_currency: quote,
                    status: product
                        .status
                        .clone()
                        .unwrap_or_else(|| "unknown".to_string()),
                    price: parse_num(product.price.as_deref()),
                    volume_24h: parse_num(product.volume_24h.as_deref()),
                    live_tradable,
                    scan_only: !matches!(instrument, Instrument::Spot),
                    trading_disabled: product.trading_disabled,
                })
            })
            .collect::<Vec<_>>();

        out.sort_by(|a, b| {
            b.volume_24h
                .partial_cmp(&a.volume_24h)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        out.truncate(self.cfg.scanner.max_products);
        out
    }

    async fn scan_product(
        &self,
        product: &WorkstationProduct,
        imports: &[StrategyLabImportSummary],
    ) -> Result<(ScannerRow, ProductDetailView, Option<MarketSnapshot>), String> {
        let product_id = product.product_id.as_str().to_string();
        let (book_res, trades_res, candles_res) = tokio::join!(
            self.client
                .get_public_product_book(&product_id, self.cfg.scanner.book_levels),
            self.client
                .get_public_market_trades(&product_id, self.cfg.scanner.trade_limit),
            self.client.get_public_candles(
                &product_id,
                self.cfg.scanner.candle_granularity_sec,
                self.cfg.scanner.candle_limit,
            ),
        );

        let book = book_res.map_err(|e| e.to_string())?;
        let trades = trades_res.unwrap_or_default();
        let candles = candles_res.unwrap_or_default();

        let strategy_cfg = self.strategy_cfg_for_product(&product_id);
        let plugin_signal =
            plugin_signal_for_product(imports, &product_id, strategy_cfg.plugin_signal);
        let lab_artifacts: Vec<&str> = imports
            .iter()
            .filter(|summary| import_matches_product(summary, &product_id))
            .map(|summary| {
                summary
                    .artifact_id
                    .as_deref()
                    .unwrap_or(summary.import_id.as_str())
            })
            .collect();
        if !lab_artifacts.is_empty() {
            info!(
                product_id = %product_id,
                plugin_signal,
                ?lab_artifacts,
                "strategy-lab artifact handoff influencing plugin signal"
            );
        }
        let (micro, momentum_score) =
            self.build_microstructure(&product_id, &book, &trades, &candles);
        let strategy = self.build_strategy_vector(
            &product_id,
            &strategy_cfg,
            &micro,
            momentum_score,
            plugin_signal,
        );
        let eligibility = self.build_eligibility(product, &strategy);
        let row = ScannerRow {
            product_id: ProductId::from(product_id.clone()),
            instrument: product.instrument.clone(),
            live_tradable: product.live_tradable,
            scan_only: product.scan_only,
            spread_bps: micro.spread_bps,
            imbalance: micro.imbalance,
            tape_direction: micro.tape_direction,
            realized_volatility: micro.realized_volatility,
            fill_rate_estimate: micro.fill_rate_estimate,
            active_strategy: strategy.strategy_name.clone(),
            score: strategy.composite_score,
            current_risk_eligibility: eligibility.clone(),
            best_bid: micro.best_bid,
            best_ask: micro.best_ask,
            mid_price: micro.mid_price,
            action: strategy.action.clone(),
            priority_fill: strategy.priority_fill,
            one_way_persistence: micro.one_way_persistence,
            ts: micro.ts,
        };

        let detail = ProductDetailView {
            product: product.clone(),
            microstructure: micro.clone(),
            strategy: strategy.clone(),
            eligibility,
            orders: self.orders_for_product(&product_id),
            imports: imports.to_vec(),
        };

        let latest_book = if micro.best_bid > 0.0 && micro.best_ask > 0.0 {
            Some(MarketSnapshot {
                market_id: product_id.clone(),
                token_id: product_id,
                bid: micro.best_bid,
                ask: micro.best_ask,
                spread: micro.best_ask - micro.best_bid,
                liquidity: sum_notional(&book.bids) + sum_notional(&book.asks),
                ts: micro.ts.unwrap_or_else(Utc::now),
            })
        } else {
            None
        };

        Ok((row, detail, latest_book))
    }

    fn build_microstructure(
        &self,
        product_id: &str,
        book: &CoinbasePriceBook,
        trades: &[CoinbaseMarketTrade],
        candles: &[CoinbaseCandle],
    ) -> (pt_core::MarketMicrostructureSnapshot, f64) {
        let bid = parse_num(book.bids.first().map(|row| row.price.as_str()));
        let ask = parse_num(book.asks.first().map(|row| row.price.as_str()));
        let mid = if bid > 0.0 && ask > 0.0 {
            (bid + ask) / 2.0
        } else {
            0.0
        };
        let spread_bps = if mid > 0.0 {
            ((ask - bid) / mid) * 10_000.0
        } else {
            0.0
        };
        let bid_notional = sum_notional(&book.bids);
        let ask_notional = sum_notional(&book.asks);
        let imbalance = if bid_notional + ask_notional > 0.0 {
            (bid_notional - ask_notional) / (bid_notional + ask_notional)
        } else {
            0.0
        };
        let tape_direction = trade_direction_score(trades);
        let (realized_volatility, momentum_score) = candle_stats(candles);
        let fill_rate_estimate = (0.55 + imbalance * 0.25 + momentum_score * 0.15
            - (spread_bps / 100.0))
            .clamp(0.0, 1.0);

        let direction = direction_sign(imbalance + tape_direction);
        let mut meta = self.meta.write();
        let telemetry = meta.products.entry(product_id.to_string()).or_default();
        telemetry.persistence = if direction != 0 && telemetry.last_direction == direction {
            telemetry.persistence + 1
        } else if direction != 0 {
            1
        } else {
            0
        };
        telemetry.last_direction = direction;
        let persistence = telemetry.persistence;
        drop(meta);

        (
            pt_core::MarketMicrostructureSnapshot {
                product_id: ProductId::from(product_id.to_string()),
                instrument: None,
                best_bid: bid,
                best_ask: ask,
                mid_price: mid,
                spread_bps,
                imbalance,
                tape_direction,
                realized_volatility,
                fill_rate_estimate,
                one_way_persistence: persistence,
                ts: Some(Utc::now()),
            },
            momentum_score,
        )
    }

    fn build_strategy_vector(
        &self,
        product_id: &str,
        strategy_cfg: &ProductStrategyConfig,
        micro: &pt_core::MarketMicrostructureSnapshot,
        momentum_score: f64,
        plugin_signal: f64,
    ) -> StrategyVector {
        let imbalance_weight = strategy_cfg
            .imbalance_weight
            .unwrap_or(self.cfg.strategy.imbalance_weight);
        let momentum_weight = strategy_cfg
            .momentum_weight
            .unwrap_or(self.cfg.strategy.momentum_weight);
        let volatility_weight = strategy_cfg
            .volatility_weight
            .unwrap_or(self.cfg.strategy.volatility_weight);

        let microstructure_score = micro.imbalance;
        let volatility_score = -micro.realized_volatility;
        let composite_score = microstructure_score * imbalance_weight
            + momentum_score * momentum_weight
            + volatility_score * volatility_weight
            + plugin_signal * self.cfg.strategy.plugin_weight;

        let action = if composite_score >= strategy_cfg.score_threshold {
            Some(TradeAction::Buy)
        } else if composite_score <= -strategy_cfg.score_threshold {
            Some(TradeAction::Sell)
        } else {
            Some(TradeAction::Hold)
        };

        StrategyVector {
            product_id: ProductId::from(product_id.to_string()),
            strategy_name: strategy_cfg.strategy_name.clone(),
            microstructure_score,
            momentum_score,
            volatility_score,
            plugin_score: plugin_signal,
            composite_score,
            action,
            priority_fill: composite_score.abs() >= self.cfg.strategy.priority_fill_threshold
                && micro.one_way_persistence >= self.cfg.live_arming.one_way_confirmation_ticks,
        }
    }

    fn build_eligibility(
        &self,
        product: &WorkstationProduct,
        strategy: &StrategyVector,
    ) -> TradingEligibility {
        let mode = self.state.coinbase.mode.read().clone();
        let live_arm = self.state.coinbase.live_arm.read().clone();
        let mut reasons = Vec::new();

        if !product.live_tradable {
            reasons.push("product is not live tradable".to_string());
        }
        if product.scan_only {
            reasons.push("derivatives remain scan-only in phase 1".to_string());
        }
        if matches!(strategy.action, Some(TradeAction::Hold) | None) {
            reasons.push("strategy score below threshold".to_string());
        }
        if mode == "live" && !live_arm.armed {
            reasons.push("live workstation is disarmed".to_string());
        }

        TradingEligibility {
            product_id: product.product_id.clone(),
            live_tradable: product.live_tradable,
            scan_only: product.scan_only,
            eligible: reasons.is_empty(),
            reasons,
        }
    }

    fn strategy_cfg_for_product(&self, product_id: &str) -> ProductStrategyConfig {
        self.cfg
            .strategy
            .products
            .iter()
            .find(|row| row.product_id == product_id)
            .cloned()
            .unwrap_or_else(|| ProductStrategyConfig {
                product_id: product_id.to_string(),
                enabled: true,
                live_enabled: true,
                strategy_name: self.cfg.strategy.default_strategy_name.clone(),
                quote_size_usd: 25.0,
                score_threshold: self.cfg.strategy.score_threshold,
                plugin_signal: 0.0,
                imbalance_weight: None,
                momentum_weight: None,
                volatility_weight: None,
            })
    }

    fn strategy_view_for_product(&self, product: &WorkstationProduct) -> ProductStrategyConfigView {
        let cfg = self.strategy_cfg_for_product(product.product_id.as_str());
        ProductStrategyConfigView {
            product_id: product.product_id.clone(),
            strategy_name: cfg.strategy_name,
            enabled: cfg.enabled,
            live_enabled: cfg.live_enabled,
            score_threshold: cfg.score_threshold,
            quote_size_usd: cfg.quote_size_usd,
            plugin_signal: cfg.plugin_signal,
        }
    }

    fn orders_for_product(&self, product_id: &str) -> Vec<WorkstationOrder> {
        self.state
            .coinbase
            .orders
            .read()
            .iter()
            .filter(|order| order.product_id.as_str() == product_id)
            .cloned()
            .collect()
    }

    async fn process_cancel_requests(&self) {
        let pending = self
            .state
            .coinbase
            .orders
            .read()
            .iter()
            .filter(|order| matches!(order.status, Some(WorkstationOrderStatus::CancelRequested)))
            .cloned()
            .collect::<Vec<_>>();

        for order in pending {
            let mode = self.state.coinbase.mode.read().clone();
            if mode == "live" && self.client.credentials_available() {
                let result = self
                    .client
                    .cancel_orders(std::slice::from_ref(&order.order_id))
                    .await;
                match result {
                    Ok(_) => self.update_order_status(
                        &order.order_id,
                        WorkstationOrderStatus::Canceled,
                        Some("canceled on coinbase".to_string()),
                    ),
                    Err(e) => self.update_order_status(
                        &order.order_id,
                        WorkstationOrderStatus::Rejected,
                        Some(format!("cancel failed: {e}")),
                    ),
                }
            } else {
                self.update_order_status(
                    &order.order_id,
                    WorkstationOrderStatus::Canceled,
                    Some("canceled locally".to_string()),
                );
            }
        }
    }

    async fn process_draft_orders(&self) {
        let draft_orders = self
            .state
            .coinbase
            .orders
            .read()
            .iter()
            .filter(|order| matches!(order.status, Some(WorkstationOrderStatus::Draft)))
            .cloned()
            .collect::<Vec<_>>();

        for draft in draft_orders {
            let mode = self.state.coinbase.mode.read().clone();
            let live_arm = self.state.coinbase.live_arm.read().clone();

            if mode == "live" && !live_arm.armed {
                self.update_order_status(
                    &draft.order_id,
                    WorkstationOrderStatus::Rejected,
                    Some("live workstation is not armed".to_string()),
                );
                continue;
            }

            let route = draft.route.clone().unwrap_or(OrderRoute::Maker);
            let side = draft.side.clone().unwrap_or(Side::Buy);
            let limit_price = draft
                .limit_price
                .or_else(|| self.derived_limit_price(draft.product_id.as_str(), &side));
            let base_size = if draft.base_size > 0.0 {
                draft.base_size
            } else if let Some(limit_price) = limit_price {
                (draft.quote_notional.max(1.0) / limit_price).max(0.00000001)
            } else {
                0.0001
            };

            {
                let mut meta = self.meta.write();
                meta.total_order_attempts += 1;
            }

            match mode.as_str() {
                "live" => {
                    let mut request = CoinbaseAdvancedTradeOrderRequest {
                        product_id: draft.product_id.as_str().to_string(),
                        side,
                        route: route.clone(),
                        base_size,
                        quote_size: if draft.quote_notional > 0.0 {
                            Some(draft.quote_notional)
                        } else {
                            None
                        },
                        limit_price,
                        post_only: draft.post_only,
                        preview_id: None,
                    };

                    if self.cfg.execution.order_manager.preview_required {
                        match self.client.preview_order(&request).await {
                            Ok(preview) if preview.success => {
                                request.preview_id = preview.preview_id;
                            }
                            Ok(preview) => {
                                self.reject_order(
                                    &draft.order_id,
                                    format!("preview rejected {:?}", preview.raw_status),
                                );
                                continue;
                            }
                            Err(e) => {
                                self.reject_order(&draft.order_id, format!("preview failed: {e}"));
                                continue;
                            }
                        }
                    }

                    match self.client.create_order(&request).await {
                        Ok(submitted) => {
                            let status = if matches!(route, OrderRoute::Taker) {
                                WorkstationOrderStatus::Filled
                            } else {
                                WorkstationOrderStatus::Open
                            };
                            self.complete_order_submission(
                                &draft.order_id,
                                &submitted.order_id,
                                status,
                                Some(submitted.raw_status),
                                base_size,
                                limit_price,
                            );
                            if matches!(route, OrderRoute::Taker) {
                                self.meta.write().taker_budget_used_usd +=
                                    draft.quote_notional.max(0.0);
                            }
                        }
                        Err(e) => self.reject_order(&draft.order_id, format!("submit failed: {e}")),
                    }
                }
                _ => {
                    let status = if matches!(route, OrderRoute::Taker) {
                        WorkstationOrderStatus::Filled
                    } else {
                        WorkstationOrderStatus::Open
                    };
                    self.complete_order_submission(
                        &draft.order_id,
                        &format!("paper-{}", Utc::now().timestamp_millis()),
                        status,
                        Some(format!("simulated {mode} order")),
                        base_size,
                        limit_price,
                    );
                }
            }
        }
    }

    fn advance_paper_orders(&self) {
        let mode = self.state.coinbase.mode.read().clone();
        if mode == "live" {
            return;
        }

        let rows = self.state.coinbase.scanner.read().clone();
        let by_product = rows
            .into_iter()
            .map(|row| (row.product_id.as_str().to_string(), row))
            .collect::<HashMap<_, _>>();
        let mut orders = self.state.coinbase.orders.write();
        for order in orders.iter_mut() {
            if !matches!(order.status, Some(WorkstationOrderStatus::Open)) {
                continue;
            }
            let Some(scanner) = by_product.get(order.product_id.as_str()) else {
                continue;
            };
            let should_fill = match order.side {
                Some(Side::Buy) => order
                    .limit_price
                    .map(|limit| scanner.best_ask <= limit || scanner.priority_fill)
                    .unwrap_or(scanner.priority_fill),
                Some(Side::Sell) => order
                    .limit_price
                    .map(|limit| scanner.best_bid >= limit || scanner.priority_fill)
                    .unwrap_or(scanner.priority_fill),
                None => false,
            };
            if should_fill {
                order.status = Some(WorkstationOrderStatus::Filled);
                order.updated_at = Some(Utc::now());
                order.reason = Some("paper order filled from scanner state".to_string());
                push_execution(
                    &self.state.recent_executions,
                    execution_from_order(order, ExecutionStatus::Filled),
                );
            }
        }
    }

    fn maybe_submit_auto_orders(&self) {
        let mode = self.state.coinbase.mode.read().clone();
        let live_arm = self.state.coinbase.live_arm.read().clone();
        if mode == "live" && !live_arm.armed {
            return;
        }

        let kill_switch = self.state.kill_switch.read().clone();
        if !matches!(kill_switch, KillSwitchState::Running) {
            return;
        }

        let rows = self.state.coinbase.scanner.read().clone();
        let mut orders = self.state.coinbase.orders.write();
        for row in rows {
            if !row.current_risk_eligibility.eligible {
                continue;
            }
            let Some(action) = row.action.clone() else {
                continue;
            };
            if matches!(action, TradeAction::Hold) {
                continue;
            }
            if orders.iter().any(|order| {
                order.product_id == row.product_id
                    && matches!(
                        order.status,
                        Some(WorkstationOrderStatus::Draft | WorkstationOrderStatus::Open)
                    )
            }) {
                continue;
            }

            let mut meta = self.meta.write();
            let telemetry = meta
                .products
                .entry(row.product_id.as_str().to_string())
                .or_default();
            let now_ms = Utc::now().timestamp_millis();
            if now_ms - telemetry.last_auto_submit_ms < self.cfg.execution.min_rest_ms as i64 {
                continue;
            }
            telemetry.last_auto_submit_ms = now_ms;
            drop(meta);

            let strategy_cfg = self.strategy_cfg_for_product(row.product_id.as_str());
            let route = if row.priority_fill
                && row.one_way_persistence >= self.cfg.live_arming.one_way_confirmation_ticks
                && self.meta.read().taker_budget_used_usd < self.cfg.live_arming.taker_budget_usd
            {
                OrderRoute::Taker
            } else {
                OrderRoute::Maker
            };

            let side = match action {
                TradeAction::Buy => Side::Buy,
                TradeAction::Sell => Side::Sell,
                TradeAction::Hold => continue,
            };
            let now = Utc::now();
            orders.push(WorkstationOrder {
                order_id: format!("auto-{}", now.timestamp_millis()),
                client_order_id: Some(format!(
                    "auto-{}",
                    now.timestamp_nanos_opt().unwrap_or_default()
                )),
                product_id: row.product_id.clone(),
                instrument: row.instrument.clone(),
                side: Some(side.clone()),
                route: Some(route.clone()),
                status: Some(WorkstationOrderStatus::Draft),
                live: mode == "live",
                post_only: matches!(route, OrderRoute::Maker),
                limit_price: derived_limit_price_from_row(&row, &side, &self.cfg),
                base_size: 0.0,
                quote_notional: strategy_cfg.quote_size_usd,
                expected_net_bps: row.score * 100.0,
                reason: Some(format!("auto {}", row.active_strategy)),
                created_at: Some(now),
                updated_at: Some(now),
            });
            break;
        }
    }

    fn enforce_live_guards(&self) {
        let mode = self.state.coinbase.mode.read().clone();
        let armed = self.state.coinbase.live_arm.read().armed;
        if mode != "live" || !armed {
            return;
        }

        let rows = self.state.coinbase.scanner.read().clone();
        let stale = rows
            .iter()
            .filter(|row| {
                row.ts
                    .map(|ts| {
                        Utc::now().signed_duration_since(ts).num_milliseconds()
                            > self.cfg.live_arming.auto_disarm_stale_data_ms as i64
                    })
                    .unwrap_or(true)
            })
            .count();

        let meta = self.meta.read();
        let reject_rate = if meta.total_order_attempts > 0 {
            meta.total_order_rejects as f64 / meta.total_order_attempts as f64
        } else {
            0.0
        };
        drop(meta);

        if stale > 0 || reject_rate > self.cfg.live_arming.auto_disarm_reject_rate {
            let reason = if stale > 0 {
                format!("auto disarm: {} stale scanner rows", stale)
            } else {
                format!("auto disarm: reject rate {:.2}", reject_rate)
            };
            let now = Utc::now();
            *self.state.kill_switch.write() = KillSwitchState::AutoHalt;
            let mut arm = self.state.coinbase.live_arm.write();
            arm.armed = false;
            arm.auto_disarm_reason = Some(reason);
            arm.updated_at = Some(now);
        }
    }

    fn merge_live_orders(&self, orders: &[CoinbaseOrderSummary]) {
        let mut local_orders = self.state.coinbase.orders.write();
        for remote in orders {
            let remote_order_id = remote.order_id.clone().unwrap_or_default();
            let remote_client_id = remote.client_order_id.clone().unwrap_or_default();
            let Some(local) = local_orders.iter_mut().find(|order| {
                order.order_id == remote_order_id
                    || order
                        .client_order_id
                        .as_deref()
                        .map(|client_id| client_id == remote_client_id)
                        .unwrap_or(false)
            }) else {
                continue;
            };

            local.order_id = remote_order_id.clone();
            local.status = Some(remote_status(remote.status.as_deref()));
            local.limit_price = remote
                .limit_price
                .as_deref()
                .map(|value| parse_num(Some(value)))
                .filter(|value| *value > 0.0);
            local.base_size = parse_num(remote.base_size.as_deref());
            local.updated_at = Some(Utc::now());
            local.reason = remote.status.clone();
        }
    }

    fn refresh_risk_state(&self, rows: &[ScannerRow]) {
        let stale_books = rows
            .iter()
            .filter(|row| {
                row.ts
                    .map(|ts| {
                        Utc::now().signed_duration_since(ts).num_milliseconds()
                            > self.cfg.live_arming.auto_disarm_stale_data_ms as i64
                    })
                    .unwrap_or(true)
            })
            .count();
        let mut risk = self.state.risk_state.write();
        risk.stale_books = stale_books;
        risk.last_update_ms = Utc::now().timestamp_millis();
        risk.killswitch = format!("{:?}", *self.state.kill_switch.read());
    }

    fn refresh_order_risk(&self) {
        let orders = self.state.coinbase.orders.read().clone();
        let open_orders = orders
            .iter()
            .filter(|order| {
                matches!(
                    order.status,
                    Some(WorkstationOrderStatus::Draft | WorkstationOrderStatus::Open)
                )
            })
            .collect::<Vec<_>>();

        let open_notional = open_orders
            .iter()
            .map(|order| order.quote_notional)
            .sum::<f64>();
        let open_markets = open_orders
            .iter()
            .map(|order| order.product_id.as_str().to_string())
            .collect::<HashSet<_>>()
            .len();
        let unhedged_delta = orders
            .iter()
            .filter(|order| matches!(order.status, Some(WorkstationOrderStatus::Filled)))
            .map(|order| match order.side {
                Some(Side::Buy) => order.quote_notional,
                Some(Side::Sell) => -order.quote_notional,
                None => 0.0,
            })
            .sum::<f64>();

        let mut inventory = self.state.inventory_usd.write();
        *inventory = unhedged_delta;

        let mut risk = self.state.risk_state.write();
        risk.open_notional = open_notional;
        risk.open_markets = open_markets;
        risk.unhedged_delta = unhedged_delta;
        risk.killswitch = format!("{:?}", *self.state.kill_switch.read());
        risk.last_update_ms = Utc::now().timestamp_millis();
    }

    fn update_order_status(
        &self,
        order_id: &str,
        status: WorkstationOrderStatus,
        reason: Option<String>,
    ) {
        let mut orders = self.state.coinbase.orders.write();
        if let Some(order) = orders.iter_mut().find(|order| order.order_id == order_id) {
            order.status = Some(status.clone());
            order.updated_at = Some(Utc::now());
            order.reason = reason;
            if matches!(status, WorkstationOrderStatus::Rejected) {
                self.meta.write().total_order_rejects += 1;
            }
        }
    }

    fn complete_order_submission(
        &self,
        local_order_id: &str,
        remote_order_id: &str,
        status: WorkstationOrderStatus,
        reason: Option<String>,
        base_size: f64,
        limit_price: Option<f64>,
    ) {
        let mut orders = self.state.coinbase.orders.write();
        if let Some(order) = orders
            .iter_mut()
            .find(|order| order.order_id == local_order_id)
        {
            order.order_id = remote_order_id.to_string();
            order.status = Some(status.clone());
            order.base_size = base_size;
            order.limit_price = limit_price;
            order.updated_at = Some(Utc::now());
            order.reason = reason;
            push_execution(
                &self.state.recent_executions,
                execution_from_order(
                    order,
                    if matches!(status, WorkstationOrderStatus::Filled) {
                        ExecutionStatus::Filled
                    } else {
                        ExecutionStatus::New
                    },
                ),
            );
        }
    }

    fn reject_order(&self, order_id: &str, reason: String) {
        self.meta.write().total_order_rejects += 1;
        self.update_order_status(order_id, WorkstationOrderStatus::Rejected, Some(reason));
    }

    fn derived_limit_price(&self, product_id: &str, side: &Side) -> Option<f64> {
        let rows = self.state.coinbase.scanner.read();
        rows.iter()
            .find(|row| row.product_id.as_str() == product_id)
            .and_then(|row| derived_limit_price_from_row(row, side, &self.cfg))
    }
}

fn resolved_mode(mode_override: Option<&str>, fallback: &EngineMode) -> Result<String, String> {
    match mode_override.map(|mode| mode.trim().to_ascii_lowercase()) {
        Some(mode) if matches!(mode.as_str(), "replay" | "paper" | "live") => Ok(mode),
        Some(mode) => Err(format!("unsupported mode {mode}")),
        None => Ok(match fallback {
            EngineMode::Replay => "replay",
            EngineMode::Paper => "paper",
            EngineMode::Live => "live",
        }
        .to_string()),
    }
}

fn instrument_for_product(product: &CoinbaseProduct) -> Instrument {
    let product_type = product
        .product_type
        .clone()
        .unwrap_or_default()
        .to_ascii_lowercase();
    if product_type.contains("future")
        || product_type.contains("perpetual")
        || product_type.contains("derivative")
    {
        Instrument::Derivative
    } else {
        Instrument::Spot
    }
}

fn parse_num(value: Option<&str>) -> f64 {
    value
        .and_then(|value| value.parse::<f64>().ok())
        .filter(|value| value.is_finite())
        .unwrap_or(0.0)
}

fn sum_notional(levels: &[CoinbaseBookLevel]) -> f64 {
    levels
        .iter()
        .map(|level| parse_num(Some(level.price.as_str())) * parse_num(Some(level.size.as_str())))
        .sum()
}

fn trade_direction_score(trades: &[CoinbaseMarketTrade]) -> f64 {
    if trades.is_empty() {
        return 0.0;
    }
    let mut sum = 0.0;
    for trade in trades {
        let score = match trade
            .side
            .as_deref()
            .unwrap_or_default()
            .to_ascii_uppercase()
            .as_str()
        {
            "BUY" => 1.0,
            "SELL" => -1.0,
            _ => 0.0,
        };
        sum += score;
    }
    (sum / trades.len() as f64).clamp(-1.0, 1.0)
}

fn candle_stats(candles: &[CoinbaseCandle]) -> (f64, f64) {
    let closes = candles
        .iter()
        .filter_map(|row| {
            row.close
                .as_deref()
                .and_then(|value| value.parse::<f64>().ok())
        })
        .collect::<Vec<_>>();
    if closes.len() < 3 {
        return (0.0, 0.0);
    }

    let mut returns = Vec::new();
    for pair in closes.windows(2) {
        if pair[0] > 0.0 && pair[1] > 0.0 {
            returns.push((pair[1] / pair[0]).ln());
        }
    }
    if returns.is_empty() {
        return (0.0, 0.0);
    }
    let mean = returns.iter().sum::<f64>() / returns.len() as f64;
    let variance = returns
        .iter()
        .map(|value| {
            let diff = value - mean;
            diff * diff
        })
        .sum::<f64>()
        / returns.len() as f64;
    let realized_volatility = variance.sqrt().clamp(0.0, 1.0);
    let momentum = ((closes.last().copied().unwrap_or_default()
        / closes.first().copied().unwrap_or(1.0))
        - 1.0)
        .tanh()
        .clamp(-1.0, 1.0);
    (realized_volatility, momentum)
}

fn direction_sign(value: f64) -> i8 {
    if value > 0.05 {
        1
    } else if value < -0.05 {
        -1
    } else {
        0
    }
}

fn import_matches_product(summary: &StrategyLabImportSummary, product_id: &str) -> bool {
    summary.best_variants.iter().any(|row| {
        row.split_once(':')
            .map(|(market, _)| market.eq_ignore_ascii_case(product_id))
            .unwrap_or(false)
    })
}

fn plugin_signal_for_product(
    imports: &[StrategyLabImportSummary],
    product_id: &str,
    base_signal: f64,
) -> f64 {
    let mut bias = 0.0;
    for summary in imports
        .iter()
        .filter(|s| import_matches_product(s, product_id))
    {
        bias += 0.1;
        if let Some(score) = summary.objective_score {
            bias += (score.clamp(-3.0, 3.0) / 30.0).clamp(-0.15, 0.15);
        } else if let Some(c) = summary.confidence {
            bias += (c.clamp(0.0, 1.0) / 40.0).clamp(0.0, 0.1);
        }
    }
    (base_signal + bias).clamp(-1.0, 1.0)
}

fn derived_limit_price_from_row(row: &ScannerRow, side: &Side, cfg: &AppConfig) -> Option<f64> {
    if row.best_bid <= 0.0 || row.best_ask <= 0.0 {
        return None;
    }
    let mid = row.mid_price.max(0.0);
    let offset = cfg.execution.vectors.entry_offset_bps / 10_000.0;
    let px = match side {
        Side::Buy => (mid * (1.0 - offset)).min(row.best_bid.max(0.0)),
        Side::Sell => (mid * (1.0 + offset)).max(row.best_ask.max(0.0)),
    };
    Some(px)
}

fn remote_status(raw: Option<&str>) -> WorkstationOrderStatus {
    match raw.unwrap_or_default().to_ascii_uppercase().as_str() {
        "OPEN" | "PENDING" => WorkstationOrderStatus::Open,
        "FILLED" | "COMPLETED" => WorkstationOrderStatus::Filled,
        "CANCELLED" | "CANCELED" => WorkstationOrderStatus::Canceled,
        "REJECTED" | "FAILED" => WorkstationOrderStatus::Rejected,
        _ => WorkstationOrderStatus::Open,
    }
}

fn execution_from_order(order: &WorkstationOrder, status: ExecutionStatus) -> ExecutionReport {
    ExecutionReport {
        venue: if order.live {
            Venue::Coinbase
        } else {
            Venue::Sim
        },
        order_id: order.order_id.clone(),
        market_id: Some(order.product_id.as_str().to_string()),
        status,
        side: order.side.clone().unwrap_or(Side::Buy),
        filled_qty: order.base_size,
        avg_px: order.limit_price.unwrap_or_default(),
        ts: order.updated_at.unwrap_or_else(Utc::now),
        details: order.reason.clone(),
    }
}

fn push_execution(target: &Arc<RwLock<Vec<ExecutionReport>>>, report: ExecutionReport) {
    let mut lock = target.write();
    lock.push(report);
    if lock.len() > 200 {
        let trim = lock.len() - 200;
        lock.drain(0..trim);
    }
}

fn push_market_history(
    target: &Arc<RwLock<HashMap<String, Vec<MarketHistoryPoint>>>>,
    snapshot: &MarketSnapshot,
) {
    let mut lock = target.write();
    let points = lock.entry(snapshot.market_id.clone()).or_default();
    points.push(MarketHistoryPoint {
        market_id: snapshot.market_id.clone(),
        mid: (snapshot.bid + snapshot.ask) / 2.0,
        spread: snapshot.spread,
        bid: snapshot.bid,
        ask: snapshot.ask,
        ts: snapshot.ts,
    });
    if points.len() > 360 {
        let trim = points.len() - 360;
        points.drain(0..trim);
    }
}
