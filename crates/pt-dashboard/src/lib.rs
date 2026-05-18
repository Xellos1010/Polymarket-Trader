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
use pt_ai_agent::{AgentProposal, ProposalKind, ProposalQueue, ProposalStatus};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
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
    pub strategy_candidates: Arc<RwLock<Vec<StrategyCandidateReviewView>>>,
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
            strategy_candidates: Arc::new(RwLock::new(Vec::new())),
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
    pub proposal_queue: ProposalQueue,
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
            proposal_queue: ProposalQueue::new(),
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
    pub strategy_candidates: Arc<RwLock<Vec<StrategyCandidateReviewView>>>,
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
    pub proposal_queue: ProposalQueue,
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
            proposal_queue: handles.proposal_queue,
            coinbase: CoinbaseDashboardState {
                mode: handles.coinbase.mode,
                live_arm: handles.coinbase.live_arm,
                products: handles.coinbase.products,
                scanner: handles.coinbase.scanner,
                product_details: handles.coinbase.product_details,
                orders: handles.coinbase.orders,
                strategies: handles.coinbase.strategies,
                imports: handles.coinbase.imports,
                strategy_candidates: handles.coinbase.strategy_candidates,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    artifact_id: Option<String>,
    path: String,
    product_id: String,
    market: String,
    variant: String,
    imported_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_run_id: Option<String>,
    promotion_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    replay_acceptance_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    objective_score: Option<f64>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StrategyCandidateObjectiveBreakdown {
    pub net_return_after_costs: f64,
    pub drawdown_penalty: f64,
    pub turnover_penalty: f64,
    pub stability_penalty: f64,
    pub final_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StrategyCandidateStabilityView {
    pub splits_requested: usize,
    pub score_stddev: f64,
    pub return_stddev: f64,
    pub penalty: f64,
    pub positive_windows: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StrategyCandidateRiskGateView {
    pub status: String,
    pub failure_count: usize,
    #[serde(default)]
    pub reason_codes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StrategyCandidatePromotionGateView {
    pub status: String,
    pub requires_replay_acceptance: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_acceptance_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_run_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StrategyCandidateReviewView {
    pub rank: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_market: Option<String>,
    pub variant: String,
    #[serde(default)]
    pub params: Map<String, Value>,
    pub score: f64,
    pub objective_breakdown: StrategyCandidateObjectiveBreakdown,
    pub stability: StrategyCandidateStabilityView,
    pub risk_gate: StrategyCandidateRiskGateView,
    pub promotion_gate: StrategyCandidatePromotionGateView,
    #[serde(default)]
    pub rejection_reasons: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_report_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle_summary_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct StrategyCandidatesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_report_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle_summary_path: Option<String>,
    pub candidates: Vec<StrategyCandidateReviewView>,
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
struct StrategyCandidatesQuery {
    product_id: Option<String>,
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
        .route("/api/v1/strategy-candidates", get(get_strategy_candidates))
        .route("/api/v1/mode", post(post_mode))
        .route("/api/v1/live/arm", post(post_live_arm))
        .route("/api/v1/live/disarm", post(post_live_disarm))
        .route("/api/v1/orders/:order_id/cancel", post(post_cancel_order))
        .route(
            "/api/v1/strategy-lab/import",
            post(post_strategy_lab_import),
        )
        .route("/api/v1/agent/console", get(get_agent_console))
        .route(
            "/api/v1/agent/proposals",
            get(get_agent_proposals).post(post_agent_proposal),
        )
        .route(
            "/api/v1/agent/proposals/:id/resolve",
            post(post_resolve_proposal),
        )
        .route("/api/v1/ai-metrics", get(get_ai_metrics))
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
    let existing_detail = state
        .coinbase
        .product_details
        .read()
        .get(&product_id)
        .cloned();
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

async fn get_strategy_candidates(
    State(state): State<DashboardState>,
    Query(query): Query<StrategyCandidatesQuery>,
) -> Json<StrategyCandidatesResponse> {
    Json(load_strategy_candidates(
        &state,
        query.product_id.as_deref(),
    ))
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
    arm.reason = payload
        .reason
        .clone()
        .or_else(|| Some("manual disarm".to_string()));
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
        client_order_id: Some(format!(
            "manual-{}",
            now.timestamp_nanos_opt().unwrap_or_default()
        )),
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

fn lab_json_str<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str)
}

fn lab_meta_object(payload: &Value) -> Option<&serde_json::Map<String, Value>> {
    payload.get("meta").and_then(Value::as_object)
}

fn extract_source_run_id(payload: &Value) -> Option<String> {
    lab_json_str(payload, "source_run_id")
        .map(str::to_string)
        .or_else(|| {
            lab_meta_object(payload)?
                .get("journal_run_id")
                .and_then(Value::as_str)
                .map(str::to_string)
        })
        .or_else(|| {
            lab_meta_object(payload)?
                .get("run_id")
                .and_then(Value::as_str)
                .map(str::to_string)
        })
}

fn extract_artifact_id_from_lab(payload: &Value) -> Option<String> {
    lab_json_str(payload, "artifact_id")
        .map(str::to_string)
        .or_else(|| {
            lab_meta_object(payload)?
                .get("artifact_id")
                .and_then(Value::as_str)
                .map(str::to_string)
        })
}

fn extract_timeframe(payload: &Value) -> Option<String> {
    let meta = lab_meta_object(payload)?;
    let g = meta
        .get("granularity_sec")
        .and_then(|v| v.as_u64())
        .or_else(|| {
            meta.get("granularity_sec")
                .and_then(|v| v.as_i64())
                .map(|n| n as u64)
        })
        .or_else(|| {
            meta.get("granularity_sec")
                .and_then(Value::as_f64)
                .map(|f| f as u64)
        })?;
    Some(format!("{g}s_candles"))
}

fn extract_objective_preview(payload: &Value) -> Option<f64> {
    let markets = payload.get("markets").and_then(Value::as_object)?;
    let mut best: Option<f64> = None;
    for mv in markets.values() {
        let default_variant = mv.get("default_variant").and_then(Value::as_str)?;
        let metrics = mv
            .get("variants")?
            .get(default_variant)?
            .get("metrics")?
            .as_object()?;
        let score = metrics
            .get("sharpe_like")
            .and_then(Value::as_f64)
            .or_else(|| metrics.get("total_return").and_then(Value::as_f64))?;
        best = Some(best.map(|b| b.max(score)).unwrap_or(score));
    }
    best
}

fn extract_promotion_status(payload: &Value) -> String {
    lab_json_str(payload, "promotion_status")
        .map(str::to_string)
        .or_else(|| {
            payload
                .get("promotion")
                .and_then(|p| p.get("status"))
                .and_then(Value::as_str)
                .map(str::to_string)
        })
        .unwrap_or_else(|| "imported_only".to_string())
}

fn extract_replay_acceptance_status(payload: &Value) -> Option<String> {
    payload
        .get("replay_acceptance")
        .and_then(|r| r.get("status"))
        .and_then(Value::as_str)
        .map(str::to_string)
        .or_else(|| lab_json_str(payload, "replay_acceptance_status").map(str::to_string))
}

fn extract_confidence_preview(payload: &Value) -> Option<f64> {
    lab_meta_object(payload)?
        .get("confidence")
        .and_then(Value::as_f64)
        .or_else(|| lab_json_str(payload, "confidence").and_then(|s| s.parse().ok()))
}

fn summarize_strategy_lab_import(path: &str) -> Result<StrategyLabImportSummary, String> {
    let raw = fs::read_to_string(path).map_err(|e| format!("failed to read {path}: {e}"))?;
    let payload: Value = serde_json::from_str(&raw)
        .map_err(|e| format!("failed to parse strategy-lab JSON: {e}"))?;
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

    let import_id = format!("import-{}", Utc::now().timestamp_millis());
    let artifact_id = extract_artifact_id_from_lab(&payload).or_else(|| Some(import_id.clone()));

    Ok(StrategyLabImportSummary {
        import_id,
        artifact_id,
        path: path.to_string(),
        imported_at: Some(Utc::now()),
        markets,
        best_variants,
        source_run_id: extract_source_run_id(&payload),
        promotion_status: extract_promotion_status(&payload),
        replay_acceptance_status: extract_replay_acceptance_status(&payload),
        objective_score: extract_objective_preview(&payload),
        confidence: extract_confidence_preview(&payload),
        timeframe: extract_timeframe(&payload),
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
                    artifact_id: summary.artifact_id.clone(),
                    path: summary.path.clone(),
                    product_id: product_id.to_string(),
                    market: market.to_string(),
                    variant: variant.to_string(),
                    imported_at: summary.imported_at,
                    source_run_id: summary.source_run_id.clone(),
                    promotion_status: summary.promotion_status.clone(),
                    replay_acceptance_status: summary.replay_acceptance_status.clone(),
                    objective_score: summary.objective_score,
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

fn load_strategy_candidates(
    state: &DashboardState,
    product_filter: Option<&str>,
) -> StrategyCandidatesResponse {
    let in_memory = state.coinbase.strategy_candidates.read().clone();
    if !in_memory.is_empty() {
        let filtered = filter_strategy_candidates(in_memory, product_filter);
        return StrategyCandidatesResponse {
            product_id: product_filter.map(str::to_string),
            source_report_path: None,
            cycle_summary_path: None,
            candidates: filtered,
        };
    }

    load_strategy_candidates_from_files(product_filter)
}

fn filter_strategy_candidates(
    candidates: Vec<StrategyCandidateReviewView>,
    product_filter: Option<&str>,
) -> Vec<StrategyCandidateReviewView> {
    match product_filter.map(str::to_ascii_lowercase) {
        Some(product) => candidates
            .into_iter()
            .filter(|candidate| {
                candidate
                    .product_id
                    .as_ref()
                    .map(|value| value.eq_ignore_ascii_case(&product))
                    .unwrap_or(false)
                    || candidate
                        .selected_market
                        .as_ref()
                        .map(|value| value.eq_ignore_ascii_case(&product))
                        .unwrap_or(false)
            })
            .collect(),
        None => candidates,
    }
}

fn load_strategy_candidates_from_files(product_filter: Option<&str>) -> StrategyCandidatesResponse {
    let Some(optimize_path) =
        latest_matching_file(PathBuf::from("data/strategy_lab"), "optimize-", ".json")
    else {
        return StrategyCandidatesResponse {
            product_id: product_filter.map(str::to_string),
            source_report_path: None,
            cycle_summary_path: None,
            candidates: Vec::new(),
        };
    };

    let optimize_payload = fs::read_to_string(&optimize_path)
        .ok()
        .and_then(|raw| serde_json::from_str::<Value>(&raw).ok());
    let cycle_summary_path =
        latest_non_acceptance_cycle_file(PathBuf::from("data/strategy_lab/hourly_optimizer_runs"));
    let cycle_summary = cycle_summary_path
        .as_ref()
        .and_then(|path| fs::read_to_string(path).ok())
        .and_then(|raw| serde_json::from_str::<Value>(&raw).ok());

    let mut candidates = optimize_payload
        .as_ref()
        .and_then(|payload| payload.get("top"))
        .and_then(Value::as_array)
        .map(|rows| {
            rows.iter()
                .map(|row| {
                    candidate_review_from_json(
                        row,
                        &optimize_path,
                        cycle_summary_path.as_deref(),
                        cycle_summary.as_ref(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    candidates = filter_strategy_candidates(candidates, product_filter);

    StrategyCandidatesResponse {
        product_id: product_filter.map(str::to_string),
        source_report_path: Some(optimize_path.display().to_string()),
        cycle_summary_path: cycle_summary_path.map(|path| path.display().to_string()),
        candidates,
    }
}

fn latest_matching_file(dir: PathBuf, prefix: &str, suffix: &str) -> Option<PathBuf> {
    let mut files = fs::read_dir(dir)
        .ok()?
        .filter_map(|entry| entry.ok().map(|row| row.path()))
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.starts_with(prefix) && name.ends_with(suffix))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    files.sort();
    files.pop()
}

fn latest_non_acceptance_cycle_file(dir: PathBuf) -> Option<PathBuf> {
    let mut files = fs::read_dir(dir)
        .ok()?
        .filter_map(|entry| entry.ok().map(|row| row.path()))
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| {
                    name.starts_with("cycle-")
                        && name.ends_with(".json")
                        && !name.contains(".acceptance.")
                })
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    files.sort();
    files.pop()
}

fn candidate_review_from_json(
    row: &Value,
    optimize_path: &std::path::Path,
    cycle_summary_path: Option<&std::path::Path>,
    cycle_summary: Option<&Value>,
) -> StrategyCandidateReviewView {
    let selected_market = row
        .get("per_market")
        .and_then(Value::as_array)
        .and_then(|rows| {
            rows.iter().max_by(|a, b| {
                let a_score = a
                    .get("score")
                    .and_then(Value::as_f64)
                    .unwrap_or(f64::NEG_INFINITY);
                let b_score = b
                    .get("score")
                    .and_then(Value::as_f64)
                    .unwrap_or(f64::NEG_INFINITY);
                a_score
                    .partial_cmp(&b_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
        })
        .and_then(|entry| entry.get("market"))
        .and_then(Value::as_str)
        .map(str::to_string);

    let cycle_candidate = cycle_summary.and_then(|summary| summary.get("candidate"));
    let is_selected = cycle_candidate
        .map(|candidate| candidate_matches_review(candidate, row))
        .unwrap_or(false);
    let decision = cycle_summary.and_then(|summary| summary.get("decision"));
    let acceptance_status = cycle_summary
        .and_then(|summary| summary.get("acceptance"))
        .and_then(|value| value.get("status"))
        .and_then(Value::as_str)
        .map(str::to_string);

    let promotion_status = if is_selected {
        cycle_summary
            .and_then(|summary| summary.get("status"))
            .and_then(Value::as_str)
            .map(str::to_string)
    } else {
        Some("not_selected".to_string())
    };

    let mut reason_codes = row
        .get("rejection_reasons")
        .and_then(Value::as_array)
        .map(|rows| {
            rows.iter()
                .filter_map(Value::as_str)
                .map(|value| {
                    value
                        .split('@')
                        .next()
                        .unwrap_or(value)
                        .replace("risk:", "")
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if is_selected {
        if let Some(code) = decision
            .and_then(|value| value.get("reason_code"))
            .and_then(Value::as_str)
        {
            if !reason_codes.iter().any(|existing| existing == code) {
                reason_codes.push(code.to_string());
            }
        }
    }

    StrategyCandidateReviewView {
        rank: row.get("rank").and_then(Value::as_u64).unwrap_or(0) as usize,
        product_id: selected_market.clone(),
        selected_market,
        variant: row
            .get("variant")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
            .to_string(),
        params: row
            .get("params")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default(),
        score: row.get("score").and_then(Value::as_f64).unwrap_or_default(),
        objective_breakdown: serde_json::from_value(
            row.get("objective_breakdown")
                .cloned()
                .unwrap_or_else(|| Value::Object(Map::new())),
        )
        .unwrap_or_default(),
        stability: serde_json::from_value(
            row.get("stability")
                .cloned()
                .unwrap_or_else(|| Value::Object(Map::new())),
        )
        .unwrap_or_default(),
        risk_gate: StrategyCandidateRiskGateView {
            status: row
                .get("risk_gate")
                .and_then(|value| value.get("status"))
                .and_then(Value::as_str)
                .unwrap_or("unknown")
                .to_string(),
            failure_count: row
                .get("risk_gate")
                .and_then(|value| value.get("failures"))
                .and_then(Value::as_array)
                .map(Vec::len)
                .unwrap_or(0),
            reason_codes,
        },
        promotion_gate: StrategyCandidatePromotionGateView {
            status: row
                .get("promotion_gate")
                .and_then(|value| value.get("status"))
                .and_then(Value::as_str)
                .unwrap_or("unknown")
                .to_string(),
            requires_replay_acceptance: row
                .get("promotion_gate")
                .and_then(|value| value.get("requires_replay_acceptance"))
                .and_then(Value::as_bool)
                .unwrap_or(true),
            replay_acceptance_status: acceptance_status,
            promotion_status,
            source_run_id: cycle_summary
                .and_then(|summary| summary.get("cycle_id"))
                .and_then(Value::as_str)
                .map(str::to_string),
        },
        rejection_reasons: row
            .get("rejection_reasons")
            .and_then(Value::as_array)
            .map(|rows| {
                rows.iter()
                    .filter_map(Value::as_str)
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default(),
        source_report_path: Some(optimize_path.display().to_string()),
        cycle_summary_path: cycle_summary_path.map(|path| path.display().to_string()),
    }
}

fn candidate_matches_review(candidate: &Value, review: &Value) -> bool {
    let variant_match = candidate.get("variant").and_then(Value::as_str)
        == review.get("variant").and_then(Value::as_str);
    let params_match = candidate.get("params") == review.get("params");
    variant_match && params_match
}

// ── Agent Console & Proposal API ─────────────────────────────────────────────

#[derive(Debug, Serialize)]
struct AgentApprovalItemView {
    id: String,
    title: String,
    description: String,
    severity: String,
    status: String,
    product_id: Option<String>,
}

#[derive(Debug, Serialize)]
struct AgentConsoleView {
    autonomy_tier: String,
    live_arm: LiveArmState,
    next_action: String,
    blocked_markets: usize,
    imports_loaded: usize,
    recommended_products: Vec<String>,
    approvals: Vec<AgentApprovalItemView>,
}

fn proposal_to_item(p: &AgentProposal) -> AgentApprovalItemView {
    let (title, description, severity, product_id) = match &p.kind {
        ProposalKind::StrategyAdjustment { parameter, value } => (
            format!("Strategy adjustment: {parameter}"),
            format!("Proposed value: {value}"),
            "medium".to_string(),
            None,
        ),
        ProposalKind::MarketSelection { market_id, action } => (
            format!("Market selection: {market_id}"),
            format!("Action: {action}"),
            "low".to_string(),
            Some(market_id.clone()),
        ),
        ProposalKind::RiskParameterChange { parameter, value } => (
            format!("Risk parameter: {parameter}"),
            format!("Proposed value: {value}"),
            "high".to_string(),
            None,
        ),
        ProposalKind::Alert { message } => (
            "Agent alert".to_string(),
            message.clone(),
            "low".to_string(),
            None,
        ),
    };
    AgentApprovalItemView {
        id: p.id.clone(),
        title,
        description,
        severity,
        status: format!("{:?}", p.status).to_ascii_lowercase(),
        product_id,
    }
}

async fn get_agent_console(State(state): State<DashboardState>) -> Json<AgentConsoleView> {
    let live_arm = state.coinbase.live_arm.read().clone();
    let imports_loaded = state.coinbase.imports.read().len();
    let proposals = state.proposal_queue.list();
    let pending: Vec<AgentApprovalItemView> = proposals
        .iter()
        .filter(|p| p.status == ProposalStatus::Pending)
        .map(proposal_to_item)
        .collect();
    let autonomy_tier = if live_arm.armed {
        "bounded_execute"
    } else {
        "recommend_only"
    };
    Json(AgentConsoleView {
        autonomy_tier: autonomy_tier.to_string(),
        live_arm,
        next_action: if pending.is_empty() {
            "Waiting for operator context".to_string()
        } else {
            format!("{} proposal(s) pending review", pending.len())
        },
        blocked_markets: 0,
        imports_loaded,
        recommended_products: vec![],
        approvals: pending,
    })
}

async fn get_agent_proposals(State(state): State<DashboardState>) -> Json<Vec<AgentApprovalItemView>> {
    let proposals = state.proposal_queue.list();
    Json(proposals.iter().map(proposal_to_item).collect())
}

#[derive(Debug, Deserialize)]
struct NewProposalPayload {
    kind: String,
    parameter: Option<String>,
    value: Option<Value>,
    market_id: Option<String>,
    action: Option<String>,
    message: Option<String>,
    reasoning: String,
    context: Option<Value>,
    model_source: Option<String>,
}

async fn post_agent_proposal(
    State(state): State<DashboardState>,
    Json(payload): Json<NewProposalPayload>,
) -> impl IntoResponse {
    let kind = match payload.kind.as_str() {
        "strategy_adjustment" => ProposalKind::StrategyAdjustment {
            parameter: payload.parameter.unwrap_or_default(),
            value: payload.value.unwrap_or(Value::Null),
        },
        "market_selection" => ProposalKind::MarketSelection {
            market_id: payload.market_id.unwrap_or_default(),
            action: payload.action.unwrap_or_default(),
        },
        "risk_parameter_change" => ProposalKind::RiskParameterChange {
            parameter: payload.parameter.unwrap_or_default(),
            value: payload.value.unwrap_or(Value::Null),
        },
        "alert" => ProposalKind::Alert {
            message: payload.message.unwrap_or_default(),
        },
        other => {
            return (StatusCode::BAD_REQUEST, format!("unknown proposal kind: {other}"))
                .into_response()
        }
    };
    let proposal = AgentProposal::new(
        kind,
        payload.reasoning,
        payload.context.unwrap_or(Value::Null),
        payload.model_source.unwrap_or_else(|| "operator".to_string()),
    );
    match state.proposal_queue.push(proposal, 50) {
        Ok(()) => StatusCode::CREATED.into_response(),
        Err(e) => (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()).into_response(),
    }
}

#[derive(Debug, Deserialize)]
struct ResolveProposalPayload {
    accepted: bool,
    operator_note: Option<String>,
}

async fn post_resolve_proposal(
    State(state): State<DashboardState>,
    Path(id): Path<String>,
    Json(payload): Json<ResolveProposalPayload>,
) -> impl IntoResponse {
    let _ = payload.operator_note; // stored in future SQLite persistence slice
    match state.proposal_queue.resolve(&id, payload.accepted) {
        Ok(resolved) => Json(proposal_to_item(&resolved)).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, e.to_string()).into_response(),
    }
}

// ── AI Metrics (issue #92) ────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
struct AiMetricsView {
    local_requests_total: f64,
    openrouter_requests_total: f64,
    openrouter_spend_today_usd: f64,
    openrouter_cap_usd: f64,
    routing_policy: String,
}

async fn get_ai_metrics(State(state): State<DashboardState>) -> Json<AiMetricsView> {
    Json(AiMetricsView {
        local_requests_total: state.metrics.get_counter("ai_local_requests"),
        openrouter_requests_total: state.metrics.get_counter("ai_openrouter_requests"),
        openrouter_spend_today_usd: state.metrics.get_gauge("ai_openrouter_spend_usd"),
        openrouter_cap_usd: state.metrics.get_gauge("ai_openrouter_cap_usd"),
        routing_policy: if state.metrics.get_gauge("ai_routing_local_first").is_finite() {
            "local_first".to_string()
        } else {
            "unknown".to_string()
        },
    })
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
    response
        .headers_mut()
        .insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
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
