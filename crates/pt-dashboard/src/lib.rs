use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use pt_core::{
    Asset, ExecutionReport, KillSwitchState, LiveArmState, MarketHistoryPoint, MarketSnapshot,
    MetricsRegistry, OrderRoute, ProductDetailView, ProductId, ProductStrategyConfigView,
    RiskState, ScannerRow, Side, StrategyLabImportSummary, TradeAction, TradingEligibility,
    WorkstationOrder, WorkstationOrderStatus, WorkstationProduct,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};

#[derive(Clone)]
pub struct CoinbaseDashboardHandles {
    pub mode: Arc<RwLock<String>>,
    pub live_arm: Arc<RwLock<LiveArmState>>,
    pub products: Arc<RwLock<Vec<WorkstationProduct>>>,
    pub scanner: Arc<RwLock<Vec<ScannerRow>>>,
    pub product_details: Arc<RwLock<HashMap<String, ProductDetailView>>>,
    pub orders: Arc<RwLock<Vec<WorkstationOrder>>>,
    pub strategies: Arc<RwLock<Vec<ProductStrategyConfigView>>>,
    pub imports: Arc<RwLock<Vec<StrategyLabImportSummary>>>,
}

impl Default for CoinbaseDashboardHandles {
    fn default() -> Self {
        Self {
            mode: Arc::new(RwLock::new("paper".to_string())),
            live_arm: Arc::new(RwLock::new(LiveArmState::default())),
            products: Arc::new(RwLock::new(Vec::new())),
            scanner: Arc::new(RwLock::new(Vec::new())),
            product_details: Arc::new(RwLock::new(HashMap::new())),
            orders: Arc::new(RwLock::new(Vec::new())),
            strategies: Arc::new(RwLock::new(Vec::new())),
            imports: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[derive(Clone)]
pub struct DashboardHandles {
    pub metrics: Arc<MetricsRegistry>,
    pub risk_state: Arc<RwLock<RiskState>>,
    pub kill_switch: Arc<RwLock<KillSwitchState>>,
    pub latest_books: Arc<RwLock<HashMap<String, MarketSnapshot>>>,
    pub market_history: Arc<RwLock<HashMap<String, Vec<MarketHistoryPoint>>>>,
    pub recent_executions: Arc<RwLock<Vec<ExecutionReport>>>,
    pub fused_bias: Arc<RwLock<HashMap<Asset, f64>>>,
    pub inventory_usd: Arc<RwLock<f64>>,
    pub coinbase: CoinbaseDashboardHandles,
}

impl Default for DashboardHandles {
    fn default() -> Self {
        Self {
            metrics: Arc::new(MetricsRegistry::default()),
            risk_state: Arc::new(RwLock::new(RiskState::default())),
            kill_switch: Arc::new(RwLock::new(KillSwitchState::Running)),
            latest_books: Arc::new(RwLock::new(HashMap::new())),
            market_history: Arc::new(RwLock::new(HashMap::new())),
            recent_executions: Arc::new(RwLock::new(Vec::new())),
            fused_bias: Arc::new(RwLock::new(HashMap::new())),
            inventory_usd: Arc::new(RwLock::new(0.0)),
            coinbase: CoinbaseDashboardHandles::default(),
        }
    }
}

#[derive(Clone)]
pub struct CoinbaseDashboardState {
    pub mode: Arc<RwLock<String>>,
    pub live_arm: Arc<RwLock<LiveArmState>>,
    pub products: Arc<RwLock<Vec<WorkstationProduct>>>,
    pub scanner: Arc<RwLock<Vec<ScannerRow>>>,
    pub product_details: Arc<RwLock<HashMap<String, ProductDetailView>>>,
    pub orders: Arc<RwLock<Vec<WorkstationOrder>>>,
    pub strategies: Arc<RwLock<Vec<ProductStrategyConfigView>>>,
    pub imports: Arc<RwLock<Vec<StrategyLabImportSummary>>>,
}

#[derive(Clone)]
pub struct DashboardState {
    pub metrics: Arc<MetricsRegistry>,
    pub risk_state: Arc<RwLock<RiskState>>,
    pub kill_switch: Arc<RwLock<KillSwitchState>>,
    pub latest_books: Arc<RwLock<HashMap<String, MarketSnapshot>>>,
    pub market_history: Arc<RwLock<HashMap<String, Vec<MarketHistoryPoint>>>>,
    pub recent_executions: Arc<RwLock<Vec<ExecutionReport>>>,
    pub fused_bias: Arc<RwLock<HashMap<Asset, f64>>>,
    pub inventory_usd: Arc<RwLock<f64>>,
    pub coinbase: CoinbaseDashboardState,
}

impl DashboardState {
    pub fn new(handles: DashboardHandles) -> Self {
        Self {
            metrics: handles.metrics,
            risk_state: handles.risk_state,
            kill_switch: handles.kill_switch,
            latest_books: handles.latest_books,
            market_history: handles.market_history,
            recent_executions: handles.recent_executions,
            fused_bias: handles.fused_bias,
            inventory_usd: handles.inventory_usd,
            coinbase: CoinbaseDashboardState {
                mode: handles.coinbase.mode,
                live_arm: handles.coinbase.live_arm,
                products: handles.coinbase.products,
                scanner: handles.coinbase.scanner,
                product_details: handles.coinbase.product_details,
                orders: handles.coinbase.orders,
                strategies: handles.coinbase.strategies,
                imports: handles.coinbase.imports,
            },
        }
    }
}

#[derive(Debug, Serialize)]
struct Health {
    status: &'static str,
    kill_switch: String,
}

#[derive(Debug, Clone, Serialize)]
struct BiasView {
    asset: String,
    bias: f64,
}

#[derive(Debug, Clone, Serialize)]
struct InventoryView {
    inventory_usd: f64,
}

#[derive(Debug, Clone, Serialize)]
struct MarketView {
    market_id: String,
    token_id: String,
    bid: f64,
    ask: f64,
    spread: f64,
    mid: f64,
    ts: String,
}

#[derive(Debug, Clone, Serialize)]
struct MarketHistoryResponse {
    market_id: Option<String>,
    points: Vec<MarketHistoryPoint>,
}

#[derive(Debug, Clone, Serialize)]
struct ModeResponse {
    mode: String,
    live_arm: LiveArmState,
}

#[derive(Debug, Clone, Serialize)]
struct ActiveImportView {
    import_id: String,
    path: String,
    product_id: String,
    market: String,
    variant: String,
    imported_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize)]
struct ProductImportActivationView {
    product_id: String,
    imports: Vec<ActiveImportView>,
}

#[derive(Debug, Clone, Serialize)]
struct ProductDetailResponse {
    #[serde(flatten)]
    detail: ProductDetailView,
    active_imports: Vec<ActiveImportView>,
}

#[derive(Debug, Clone, Serialize)]
struct StrategiesResponse {
    mode: String,
    live_arm: LiveArmState,
    strategies: Vec<ProductStrategyConfigView>,
    imports: Vec<StrategyLabImportSummary>,
    active_imports: Vec<ProductImportActivationView>,
}

#[derive(Debug, Clone, Serialize)]
struct ActionResponse {
    ok: bool,
    message: String,
}

#[derive(Debug, Clone, Deserialize)]
struct HistoryQuery {
    market_id: Option<String>,
    limit: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
struct ModeRequest {
    mode: String,
}

#[derive(Debug, Clone, Deserialize)]
struct LiveArmRequest {
    reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct StrategyLabImportRequest {
    path: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ManualOrderRequest {
    product_id: String,
    side: String,
    route: Option<String>,
    quote_notional: Option<f64>,
    base_size: Option<f64>,
    limit_price: Option<f64>,
    post_only: Option<bool>,
    priority_fill: Option<bool>,
    strategy_name: Option<String>,
    expected_net_bps: Option<f64>,
}

pub fn router(state: DashboardState) -> Router {
    Router::new()
        .route("/", get(get_dashboard))
        .route("/assets/*path", get(get_frontend_asset))
        .route("/favicon.ico", get(get_favicon))
        .route("/health", get(get_health))
        .route("/healthz", get(get_health))
        .route("/ready", get(get_health))
        .route("/metrics", get(get_metrics))
        .route("/state/risk", get(get_risk_state))
        .route("/state/books", get(get_books))
        .route("/state/markets", get(get_markets))
        .route("/state/history", get(get_market_history))
        .route("/state/executions", get(get_executions))
        .route("/state/bias", get(get_bias))
        .route("/state/inventory", get(get_inventory))
        .route("/ops/halt", post(post_halt))
        .route("/ops/resume", post(post_resume))
        .route("/ops/flatten", post(post_flatten))
        .route("/api/v1/products", get(get_products))
        .route("/api/v1/scanner", get(get_scanner))
        .route("/api/v1/products/:product_id", get(get_product_detail))
        .route("/api/v1/orders", get(get_orders).post(post_order))
        .route("/api/v1/strategies", get(get_strategies))
        .route("/api/v1/mode", post(post_mode))
        .route("/api/v1/live/arm", post(post_live_arm))
        .route("/api/v1/live/disarm", post(post_live_disarm))
        .route("/api/v1/orders/:order_id/cancel", post(post_cancel_order))
        .route("/api/v1/strategy-lab/import", post(post_strategy_lab_import))
        .with_state(state)
}

async fn get_dashboard() -> Html<String> {
    let html = read_frontend_file("index.html")
        .and_then(|bytes| String::from_utf8(bytes).ok())
        .unwrap_or_else(|| FRONTEND_FALLBACK_HTML.to_string());
    Html(html)
}

async fn get_frontend_asset(Path(path): Path<String>) -> Response {
    let requested = path;
    match read_frontend_file(&requested) {
        Some(bytes) => bytes_response(bytes, content_type_for(&requested)),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn get_favicon() -> Response {
    match read_frontend_file("favicon.ico") {
        Some(bytes) => bytes_response(bytes, "image/x-icon"),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn get_health(State(state): State<DashboardState>) -> Json<Health> {
    let k = format!("{:?}", *state.kill_switch.read());
    Json(Health {
        status: "ok",
        kill_switch: k,
    })
}

async fn get_metrics(State(state): State<DashboardState>) -> String {
    state.metrics.render_prometheus()
}

async fn get_risk_state(State(state): State<DashboardState>) -> Json<RiskState> {
    Json(state.risk_state.read().clone())
}

async fn get_books(State(state): State<DashboardState>) -> Json<Vec<MarketSnapshot>> {
    let mut books: Vec<MarketSnapshot> = state.latest_books.read().values().cloned().collect();
    books.sort_by(|a, b| b.ts.cmp(&a.ts));
    Json(books)
}

async fn get_markets(State(state): State<DashboardState>) -> Json<Vec<MarketView>> {
    let mut rows: Vec<MarketView> = state
        .latest_books
        .read()
        .values()
        .map(|b| MarketView {
            market_id: b.market_id.clone(),
            token_id: b.token_id.clone(),
            bid: b.bid,
            ask: b.ask,
            spread: b.spread,
            mid: (b.bid + b.ask) / 2.0,
            ts: b.ts.to_rfc3339(),
        })
        .collect();

    rows.sort_by(|a, b| b.ts.cmp(&a.ts));
    Json(rows)
}

async fn get_market_history(
    State(state): State<DashboardState>,
    Query(query): Query<HistoryQuery>,
) -> Json<MarketHistoryResponse> {
    let limit = query.limit.unwrap_or(240).clamp(10, 1200);

    let selected_market = query
        .market_id
        .filter(|m| !m.trim().is_empty())
        .or_else(|| {
            let mut books: Vec<MarketSnapshot> =
                state.latest_books.read().values().cloned().collect();
            books.sort_by(|a, b| b.ts.cmp(&a.ts));
            books.first().map(|b| b.market_id.clone())
        });

    let mut points = selected_market
        .as_ref()
        .and_then(|market_id| state.market_history.read().get(market_id).cloned())
        .unwrap_or_default();

    if points.len() > limit {
        let keep_from = points.len() - limit;
        points = points.split_off(keep_from);
    }

    Json(MarketHistoryResponse {
        market_id: selected_market,
        points,
    })
}

async fn get_executions(State(state): State<DashboardState>) -> Json<Vec<ExecutionReport>> {
    Json(state.recent_executions.read().clone())
}

async fn get_bias(State(state): State<DashboardState>) -> Json<Vec<BiasView>> {
    let mut rows: Vec<BiasView> = state
        .fused_bias
        .read()
        .iter()
        .map(|(asset, bias)| BiasView {
            asset: asset.as_str().to_string(),
            bias: *bias,
        })
        .collect();
    rows.sort_by(|a, b| a.asset.cmp(&b.asset));
    Json(rows)
}

async fn get_inventory(State(state): State<DashboardState>) -> Json<InventoryView> {
    Json(InventoryView {
        inventory_usd: *state.inventory_usd.read(),
    })
}

async fn post_halt(State(state): State<DashboardState>) -> Json<Health> {
    *state.kill_switch.write() = KillSwitchState::ManualHalt;
    Json(Health {
        status: "ok",
        kill_switch: "ManualHalt".to_string(),
    })
}

async fn post_resume(State(state): State<DashboardState>) -> Json<Health> {
    *state.kill_switch.write() = KillSwitchState::Running;
    Json(Health {
        status: "ok",
        kill_switch: "Running".to_string(),
    })
}

async fn post_flatten(State(state): State<DashboardState>) -> Json<Health> {
    *state.kill_switch.write() = KillSwitchState::SafeMode;
    Json(Health {
        status: "ok",
        kill_switch: "SafeMode".to_string(),
    })
}

async fn get_products(State(state): State<DashboardState>) -> Json<Vec<WorkstationProduct>> {
    let mut rows = state.coinbase.products.read().clone();
    rows.sort_by(|a, b| a.product_id.as_str().cmp(b.product_id.as_str()));
    Json(rows)
}

async fn get_scanner(State(state): State<DashboardState>) -> Json<Vec<ScannerRow>> {
    let mut rows = state.coinbase.scanner.read().clone();
    rows.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    Json(rows)
}

async fn get_product_detail(
    State(state): State<DashboardState>,
    Path(product_id): Path<String>,
) -> Result<Json<ProductDetailResponse>, StatusCode> {
    let existing_detail = state.coinbase.product_details.read().get(&product_id).cloned();
    let current_orders = state
        .coinbase
        .orders
        .read()
        .iter()
        .filter(|order| order.product_id.as_str() == product_id)
        .cloned()
        .collect::<Vec<_>>();
    let current_imports = state.coinbase.imports.read().clone();
    let active_imports = active_imports_for_product(&current_imports, &product_id);
    if let Some(mut detail) = existing_detail {
        detail.orders = current_orders;
        detail.imports = current_imports;
        return Ok(Json(ProductDetailResponse {
            detail,
            active_imports,
        }));
    }

    let products = state.coinbase.products.read();
    let Some(product) = products
        .iter()
        .find(|product| product.product_id.as_str() == product_id)
        .cloned()
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    drop(products);

    let scanner = state.coinbase.scanner.read();
    let scanner_row = scanner
        .iter()
        .find(|row| row.product_id.as_str() == product_id)
        .cloned();
    drop(scanner);

    let strategies = state.coinbase.strategies.read();
    let strategy = strategies
        .iter()
        .find(|row| row.product_id.as_str() == product_id)
        .cloned();
    drop(strategies);

    let detail = ProductDetailView {
        product,
        microstructure: scanner_row
            .as_ref()
            .map(scanner_row_to_microstructure)
            .unwrap_or_default(),
        strategy: strategy
            .as_ref()
            .map(strategy_view_to_vector)
            .unwrap_or_default(),
        eligibility: scanner_row
            .as_ref()
            .map(|row| row.current_risk_eligibility.clone())
            .unwrap_or_else(|| TradingEligibility {
                product_id: ProductId::from(product_id.clone()),
                live_tradable: false,
                scan_only: true,
                eligible: false,
                reasons: vec!["product detail not yet ranked by scanner".to_string()],
            }),
        orders: current_orders,
        imports: current_imports,
    };

    Ok(Json(ProductDetailResponse {
        detail,
        active_imports,
    }))
}

async fn get_orders(State(state): State<DashboardState>) -> Json<Vec<WorkstationOrder>> {
    let mut rows = state.coinbase.orders.read().clone();
    rows.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Json(rows)
}

async fn get_strategies(State(state): State<DashboardState>) -> Json<StrategiesResponse> {
    let strategies = state.coinbase.strategies.read().clone();
    let imports = state.coinbase.imports.read().clone();
    let active_imports = collect_active_imports(&strategies, &imports);
    Json(StrategiesResponse {
        mode: state.coinbase.mode.read().clone(),
        live_arm: state.coinbase.live_arm.read().clone(),
        strategies,
        imports,
        active_imports,
    })
}

async fn post_mode(
    State(state): State<DashboardState>,
    Json(payload): Json<ModeRequest>,
) -> Result<Json<ModeResponse>, (StatusCode, Json<ActionResponse>)> {
    let mode = payload.mode.trim().to_ascii_lowercase();
    if !matches!(mode.as_str(), "replay" | "paper" | "live") {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ActionResponse {
                ok: false,
                message: "mode must be one of replay|paper|live".to_string(),
            }),
        ));
    }

    *state.coinbase.mode.write() = mode.clone();
    let mut arm = state.coinbase.live_arm.write();
    arm.mode = Some(mode.clone());
    arm.updated_at = Some(Utc::now());
    let response = ModeResponse {
        mode,
        live_arm: arm.clone(),
    };
    Ok(Json(response))
}

async fn post_live_arm(
    State(state): State<DashboardState>,
    Json(payload): Json<LiveArmRequest>,
) -> Json<ModeResponse> {
    let mode = state.coinbase.mode.read().clone();
    let now = Utc::now();
    let mut arm = state.coinbase.live_arm.write();
    arm.armed = true;
    arm.mode = Some(mode.clone());
    arm.reason = payload.reason.clone();
    arm.auto_disarm_reason = None;
    arm.armed_at = Some(now);
    arm.updated_at = Some(now);
    *state.kill_switch.write() = KillSwitchState::Running;

    Json(ModeResponse {
        mode,
        live_arm: arm.clone(),
    })
}

async fn post_live_disarm(
    State(state): State<DashboardState>,
    Json(payload): Json<LiveArmRequest>,
) -> Json<ModeResponse> {
    let mode = state.coinbase.mode.read().clone();
    let now = Utc::now();
    let mut arm = state.coinbase.live_arm.write();
    arm.armed = false;
    arm.mode = Some(mode.clone());
    arm.reason = payload.reason.clone().or_else(|| Some("manual disarm".to_string()));
    arm.updated_at = Some(now);
    *state.kill_switch.write() = KillSwitchState::ManualHalt;

    Json(ModeResponse {
        mode,
        live_arm: arm.clone(),
    })
}

async fn post_order(
    State(state): State<DashboardState>,
    Json(payload): Json<ManualOrderRequest>,
) -> Result<Json<WorkstationOrder>, (StatusCode, Json<ActionResponse>)> {
    let side = parse_side(&payload.side).ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ActionResponse {
                ok: false,
                message: "side must be buy or sell".to_string(),
            }),
        )
    })?;
    let route = parse_route(payload.route.as_deref()).unwrap_or(OrderRoute::Maker);
    let priority_fill = payload.priority_fill.unwrap_or(false);
    let active_import = {
        let imports = state.coinbase.imports.read();
        active_imports_for_product(&imports, &payload.product_id)
            .into_iter()
            .next()
    };
    let now = Utc::now();
    let status = if matches!(route, OrderRoute::ScanOnly) {
        WorkstationOrderStatus::ScanOnly
    } else {
        WorkstationOrderStatus::Draft
    };
    let order = WorkstationOrder {
        order_id: format!("manual-{}", now.timestamp_millis()),
        client_order_id: Some(format!("manual-{}", now.timestamp_nanos_opt().unwrap_or_default())),
        product_id: ProductId::from(payload.product_id.clone()),
        instrument: None,
        side: Some(side),
        route: Some(route),
        status: Some(status),
        live: state.coinbase.mode.read().as_str() == "live",
        post_only: payload.post_only.unwrap_or(true),
        limit_price: payload.limit_price,
        base_size: payload.base_size.unwrap_or(0.0),
        quote_notional: payload.quote_notional.unwrap_or(0.0),
        expected_net_bps: payload.expected_net_bps.unwrap_or(0.0),
        reason: Some(order_reason(
            payload.strategy_name.as_deref(),
            priority_fill,
            active_import.as_ref(),
        )),
        created_at: Some(now),
        updated_at: Some(now),
    };
    state.coinbase.orders.write().push(order.clone());
    Ok(Json(order))
}

async fn post_cancel_order(
    State(state): State<DashboardState>,
    Path(order_id): Path<String>,
) -> Result<Json<ActionResponse>, (StatusCode, Json<ActionResponse>)> {
    let mut orders = state.coinbase.orders.write();
    let Some(order) = orders.iter_mut().find(|order| order.order_id == order_id) else {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ActionResponse {
                ok: false,
                message: format!("unknown order_id {order_id}"),
            }),
        ));
    };

    order.status = Some(match order.status {
        Some(WorkstationOrderStatus::Draft) => WorkstationOrderStatus::Canceled,
        _ => WorkstationOrderStatus::CancelRequested,
    });
    order.reason = Some("cancel requested from dashboard".to_string());
    order.updated_at = Some(Utc::now());

    Ok(Json(ActionResponse {
        ok: true,
        message: format!("cancel queued for {order_id}"),
    }))
}

async fn post_strategy_lab_import(
    State(state): State<DashboardState>,
    Json(payload): Json<StrategyLabImportRequest>,
) -> Result<Json<StrategyLabImportSummary>, (StatusCode, Json<ActionResponse>)> {
    let summary = summarize_strategy_lab_import(&payload.path).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ActionResponse {
                ok: false,
                message: e,
            }),
        )
    })?;
    state.coinbase.imports.write().push(summary.clone());
    Ok(Json(summary))
}

fn summarize_strategy_lab_import(path: &str) -> Result<StrategyLabImportSummary, String> {
    let raw = fs::read_to_string(path).map_err(|e| format!("failed to read {path}: {e}"))?;
    let payload: Value =
        serde_json::from_str(&raw).map_err(|e| format!("failed to parse strategy-lab JSON: {e}"))?;
    let markets = payload
        .get("markets")
        .and_then(Value::as_object)
        .map(|rows| rows.keys().cloned().collect::<Vec<_>>())
        .unwrap_or_default();

    let mut best_variants = Vec::new();
    if let Some(rows) = payload.get("markets").and_then(Value::as_object) {
        for (market, value) in rows {
            if let Some(default_variant) = value.get("default_variant").and_then(Value::as_str) {
                best_variants.push(format!("{market}:{default_variant}"));
                continue;
            }
            if let Some(variants) = value.get("variants").and_then(Value::as_object) {
                if let Some((name, _)) = variants.iter().next() {
                    best_variants.push(format!("{market}:{name}"));
                }
            }
        }
    }

    Ok(StrategyLabImportSummary {
        import_id: format!("import-{}", Utc::now().timestamp_millis()),
        path: path.to_string(),
        imported_at: Some(Utc::now()),
        markets,
        best_variants,
    })
}

fn active_imports_for_product(
    imports: &[StrategyLabImportSummary],
    product_id: &str,
) -> Vec<ActiveImportView> {
    let mut rows = Vec::new();
    for summary in imports {
        for best_variant in &summary.best_variants {
            let Some((market, variant)) = best_variant.split_once(':') else {
                continue;
            };
            if market.eq_ignore_ascii_case(product_id) {
                rows.push(ActiveImportView {
                    import_id: summary.import_id.clone(),
                    path: summary.path.clone(),
                    product_id: product_id.to_string(),
                    market: market.to_string(),
                    variant: variant.to_string(),
                    imported_at: summary.imported_at,
                });
            }
        }
    }
    rows.sort_by(|a, b| {
        b.imported_at
            .cmp(&a.imported_at)
            .then_with(|| a.import_id.cmp(&b.import_id))
    });
    rows
}

fn collect_active_imports(
    strategies: &[ProductStrategyConfigView],
    imports: &[StrategyLabImportSummary],
) -> Vec<ProductImportActivationView> {
    strategies
        .iter()
        .filter_map(|strategy| {
            let product_imports = active_imports_for_product(imports, strategy.product_id.as_str());
            if product_imports.is_empty() {
                None
            } else {
                Some(ProductImportActivationView {
                    product_id: strategy.product_id.as_str().to_string(),
                    imports: product_imports,
                })
            }
        })
        .collect()
}

fn order_reason(
    strategy_name: Option<&str>,
    priority_fill: bool,
    active_import: Option<&ActiveImportView>,
) -> String {
    let mut parts = Vec::new();
    if let Some(strategy_name) = strategy_name {
        if !strategy_name.trim().is_empty() {
            parts.push(format!("strategy={strategy_name}"));
        }
    }
    if priority_fill {
        parts.push("priority_fill".to_string());
    }
    if let Some(active_import) = active_import {
        parts.push(format!(
            "import={} {}:{}",
            active_import.import_id, active_import.market, active_import.variant
        ));
    }

    if parts.is_empty() {
        "queued from dashboard".to_string()
    } else {
        format!("queued from dashboard ({})", parts.join(" "))
    }
}

fn scanner_row_to_microstructure(row: &ScannerRow) -> pt_core::MarketMicrostructureSnapshot {
    pt_core::MarketMicrostructureSnapshot {
        product_id: row.product_id.clone(),
        instrument: row.instrument.clone(),
        best_bid: row.best_bid,
        best_ask: row.best_ask,
        mid_price: row.mid_price,
        spread_bps: row.spread_bps,
        imbalance: row.imbalance,
        tape_direction: row.tape_direction,
        realized_volatility: row.realized_volatility,
        fill_rate_estimate: row.fill_rate_estimate,
        one_way_persistence: row.one_way_persistence,
        ts: row.ts,
    }
}

fn strategy_view_to_vector(view: &ProductStrategyConfigView) -> pt_core::StrategyVector {
    pt_core::StrategyVector {
        product_id: view.product_id.clone(),
        strategy_name: view.strategy_name.clone(),
        plugin_score: view.plugin_signal,
        action: Some(TradeAction::Hold),
        ..pt_core::StrategyVector::default()
    }
}

fn parse_side(raw: &str) -> Option<Side> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "buy" => Some(Side::Buy),
        "sell" => Some(Side::Sell),
        _ => None,
    }
}

fn parse_route(raw: Option<&str>) -> Option<OrderRoute> {
    match raw.unwrap_or("maker").trim().to_ascii_lowercase().as_str() {
        "maker" => Some(OrderRoute::Maker),
        "taker" => Some(OrderRoute::Taker),
        "scan_only" | "scan-only" => Some(OrderRoute::ScanOnly),
        _ => None,
    }
}

fn read_frontend_file(relative_path: &str) -> Option<Vec<u8>> {
    let path = frontend_dist_dir().join(relative_path);
    fs::read(path).ok()
}

fn frontend_dist_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("frontend")
        .join("dist")
}

fn content_type_for(path: &str) -> &'static str {
    if path.ends_with(".js") {
        "application/javascript; charset=utf-8"
    } else if path.ends_with(".css") {
        "text/css; charset=utf-8"
    } else if path.ends_with(".svg") {
        "image/svg+xml"
    } else if path.ends_with(".ico") {
        "image/x-icon"
    } else if path.ends_with(".json") {
        "application/json; charset=utf-8"
    } else {
        "application/octet-stream"
    }
}

fn bytes_response(bytes: Vec<u8>, content_type: &'static str) -> Response {
    let mut response = bytes.into_response();
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static(content_type),
    );
    response
}

const FRONTEND_FALLBACK_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Coinbase Workstation</title>
  <style>
    body { margin: 0; font-family: ui-sans-serif, system-ui, sans-serif; background: #071018; color: #f3f6fb; }
    main { max-width: 960px; margin: 0 auto; padding: 32px 20px 64px; }
    h1 { margin: 0 0 8px; font-size: 28px; }
    p { color: #98abc0; line-height: 1.5; }
    .card { background: #0f1b29; border: 1px solid #203346; border-radius: 16px; padding: 20px; margin-top: 20px; }
    code { color: #9dd8ff; }
    a { color: #9dd8ff; }
  </style>
</head>
<body>
  <main>
    <h1>Coinbase Workstation</h1>
    <p>The React/Vite frontend bundle is not present in <code>crates/pt-dashboard/frontend/dist</code> yet, but the API is live.</p>
    <div class="card">
      <p>Core endpoints:</p>
      <p><a href="/api/v1/scanner">/api/v1/scanner</a></p>
      <p><a href="/api/v1/products">/api/v1/products</a></p>
      <p><a href="/api/v1/orders">/api/v1/orders</a></p>
      <p><a href="/api/v1/strategies">/api/v1/strategies</a></p>
    </div>
  </main>
</body>
</html>
"#;
