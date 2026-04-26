use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use parking_lot::RwLock;
use pt_core::{
    AgentApprovalItem, AgentConsoleView, Asset, ExecutionReport, KillSwitchState,
    ListingLifecycleStage, ListingRadarDetailView, ListingRadarRow, ListingVenueRoute,
    LiveArmState, MarketHistoryPoint, MarketSnapshot, MetricsRegistry, OrderRoute,
    ProductDetailView, ProductId, ProductStrategyConfigView, ProviderInsight, RiskOverviewView,
    RiskState, ScannerRow, Side, StrategyLabImportSummary, TradeAction, TradingEligibility,
    WorkstationOrder, WorkstationOrderStatus, WorkstationProduct,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};

#[derive(Debug, Clone)]
pub struct StoredPolicyEvent {
    pub event_id: String,
    pub event_type: String,
    pub outcome: String,
    pub summary: String,
    pub product_id: Option<ProductId>,
    pub created_at: chrono::DateTime<Utc>,
}

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
    pub policy_events: Arc<RwLock<Vec<StoredPolicyEvent>>>,
    pub approval_queue: Arc<RwLock<Vec<AgentApprovalItem>>>,
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
            policy_events: Arc::new(RwLock::new(Vec::new())),
            approval_queue: Arc::new(RwLock::new(Vec::new())),
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
    pub policy_events: Arc<RwLock<Vec<StoredPolicyEvent>>>,
    pub approval_queue: Arc<RwLock<Vec<AgentApprovalItem>>>,
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
                policy_events: handles.coinbase.policy_events,
                approval_queue: handles.coinbase.approval_queue,
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
struct StrategiesResponse {
    mode: String,
    live_arm: LiveArmState,
    strategies: Vec<ProductStrategyConfigView>,
    imports: Vec<StrategyLabImportSummary>,
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
        .route("/api/v1/listings", get(get_listing_radar))
        .route("/api/v1/listings/:product_id", get(get_listing_radar_detail))
        .route("/api/v1/risk/overview", get(get_risk_overview))
        .route("/api/v1/agent/console", get(get_agent_console))
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
    record_policy_event(
        &state,
        "kill_switch",
        "manual_halt",
        "Operator placed the workstation into manual halt.",
        None,
    );
    Json(Health {
        status: "ok",
        kill_switch: "ManualHalt".to_string(),
    })
}

async fn post_resume(State(state): State<DashboardState>) -> Json<Health> {
    *state.kill_switch.write() = KillSwitchState::Running;
    record_policy_event(
        &state,
        "kill_switch",
        "resume",
        "Operator resumed the workstation from a halted posture.",
        None,
    );
    Json(Health {
        status: "ok",
        kill_switch: "Running".to_string(),
    })
}

async fn post_flatten(State(state): State<DashboardState>) -> Json<Health> {
    *state.kill_switch.write() = KillSwitchState::SafeMode;
    record_policy_event(
        &state,
        "kill_switch",
        "safe_mode",
        "Operator requested safe mode and flatten behavior.",
        None,
    );
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
) -> Result<Json<ProductDetailView>, StatusCode> {
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
    if let Some(mut detail) = existing_detail {
        detail.orders = current_orders;
        detail.imports = current_imports;
        return Ok(Json(detail));
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

    Ok(Json(detail))
}

async fn get_listing_radar(State(state): State<DashboardState>) -> Json<Vec<ListingRadarRow>> {
    Json(build_listing_radar_rows(&state))
}

async fn get_listing_radar_detail(
    State(state): State<DashboardState>,
    Path(product_id): Path<String>,
) -> Result<Json<ListingRadarDetailView>, StatusCode> {
    build_listing_radar_detail(&state, &product_id)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn get_risk_overview(State(state): State<DashboardState>) -> Json<RiskOverviewView> {
    Json(build_risk_overview(&state))
}

async fn get_agent_console(State(state): State<DashboardState>) -> Json<AgentConsoleView> {
    Json(build_agent_console(&state))
}

async fn get_orders(State(state): State<DashboardState>) -> Json<Vec<WorkstationOrder>> {
    let mut rows = state.coinbase.orders.read().clone();
    rows.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Json(rows)
}

async fn get_strategies(State(state): State<DashboardState>) -> Json<StrategiesResponse> {
    Json(StrategiesResponse {
        mode: state.coinbase.mode.read().clone(),
        live_arm: state.coinbase.live_arm.read().clone(),
        strategies: state.coinbase.strategies.read().clone(),
        imports: state.coinbase.imports.read().clone(),
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
    drop(arm);

    record_policy_event(
        &state,
        "mode_change",
        "accepted",
        format!("Workstation mode changed to {mode} by the operator."),
        None,
    );

    let response = ModeResponse {
        mode,
        live_arm: state.coinbase.live_arm.read().clone(),
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
    drop(arm);
    *state.kill_switch.write() = KillSwitchState::Running;

    set_approval_status(
        &state.coinbase.approval_queue,
        "approval-live-arm",
        "approved",
        Some("Operator explicitly armed live routing posture.".to_string()),
    );
    record_policy_event(
        &state,
        "live_arm",
        "approved",
        format!(
            "Operator armed live routing while workstation mode was {}.",
            mode
        ),
        None,
    );

    Json(ModeResponse {
        mode,
        live_arm: state.coinbase.live_arm.read().clone(),
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
    drop(arm);
    *state.kill_switch.write() = KillSwitchState::ManualHalt;

    record_policy_event(
        &state,
        "live_arm",
        "disarmed",
        format!("Operator disarmed live routing while workstation mode was {}.", mode),
        None,
    );

    Json(ModeResponse {
        mode,
        live_arm: state.coinbase.live_arm.read().clone(),
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
    let now = Utc::now();
    let status = if matches!(route, OrderRoute::ScanOnly) {
        WorkstationOrderStatus::ScanOnly
    } else {
        WorkstationOrderStatus::Draft
    };
    let order = WorkstationOrder {
        order_id: format!("manual-{}", now.timestamp_millis()),
        client_order_id: Some(format!("manual-{}", now.timestamp_nanos_opt().unwrap_or_default())),
        product_id: ProductId::from(payload.product_id),
        instrument: None,
        side: Some(side),
        route: Some(route.clone()),
        status: Some(status),
        live: state.coinbase.mode.read().as_str() == "live",
        post_only: payload.post_only.unwrap_or(true),
        limit_price: payload.limit_price,
        base_size: payload.base_size.unwrap_or(0.0),
        quote_notional: payload.quote_notional.unwrap_or(0.0),
        expected_net_bps: payload.expected_net_bps.unwrap_or(0.0),
        reason: payload
            .strategy_name
            .clone()
            .map(|strategy| {
                let priority_fill = if payload.priority_fill.unwrap_or(false) {
                    " priority_fill"
                } else {
                    ""
                };
                format!("queued from dashboard ({strategy}{priority_fill})")
            }),
        created_at: Some(now),
        updated_at: Some(now),
    };
    state.coinbase.orders.write().push(order.clone());

    record_policy_event(
        &state,
        "order_queue",
        if matches!(route, OrderRoute::Taker) {
            "review_required"
        } else {
            "queued"
        },
        format!(
            "Queued {} order for {} via {} routing.",
            side_label(order.side.as_ref()),
            order.product_id.as_str(),
            route_label(Some(&route))
        ),
        Some(order.product_id.clone()),
    );

    if order.live || matches!(route, OrderRoute::Taker) {
        upsert_approval_item(
            &state.coinbase.approval_queue,
            AgentApprovalItem {
                id: format!("approval-order-{}", order.order_id),
                title: if order.live {
                    "Review live order escalation".to_string()
                } else {
                    "Review taker route escalation".to_string()
                },
                description: format!(
                    "Order {} for {} was queued through {} routing and should be reviewed against replay and paper evidence.",
                    order.order_id,
                    order.product_id.as_str(),
                    route_label(order.route.as_ref())
                ),
                severity: if order.live {
                    "high".to_string()
                } else {
                    "medium".to_string()
                },
                status: "pending".to_string(),
                product_id: Some(order.product_id.clone()),
            },
        );
    }

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
    let product_id = order.product_id.clone();
    drop(orders);

    set_approval_status(
        &state.coinbase.approval_queue,
        &format!("approval-order-{order_id}"),
        "resolved",
        Some("Order was canceled or marked for cancellation by the operator.".to_string()),
    );
    record_policy_event(
        &state,
        "order_cancel",
        "operator_requested",
        format!("Cancellation was requested for order {order_id} by the operator."),
        Some(product_id),
    );

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

    set_approval_status(
        &state.coinbase.approval_queue,
        "approval-imports",
        "approved",
        Some("Strategy evidence is now attached to the workstation session.".to_string()),
    );
    record_policy_event(
        &state,
        "strategy_import",
        "attached",
        format!(
            "Imported strategy-lab evidence from {} covering {} markets.",
            summary.path,
            summary.markets.len()
        ),
        None,
    );

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

fn listing_stage_for(product: &WorkstationProduct) -> ListingLifecycleStage {
    let status = product.status.to_ascii_lowercase();
    if product.live_tradable {
        ListingLifecycleStage::FullTrading
    } else if status.contains("auction") {
        ListingLifecycleStage::Auction
    } else if product.scan_only {
        ListingLifecycleStage::TransferOnly
    } else if product.trading_disabled {
        ListingLifecycleStage::Monitoring
    } else {
        ListingLifecycleStage::Research
    }
}

fn listing_stage_label(stage: &ListingLifecycleStage) -> &'static str {
    match stage {
        ListingLifecycleStage::Research => "research",
        ListingLifecycleStage::Monitoring => "monitoring",
        ListingLifecycleStage::TransferOnly => "transfer only",
        ListingLifecycleStage::Auction => "auction",
        ListingLifecycleStage::FullTrading => "full trading",
    }
}

fn scanner_map(state: &DashboardState) -> HashMap<String, ScannerRow> {
    state
        .coinbase
        .scanner
        .read()
        .iter()
        .cloned()
        .map(|row| (row.product_id.as_str().to_string(), row))
        .collect()
}

fn imports_for_product(
    imports: &[StrategyLabImportSummary],
    product_id: &str,
) -> Vec<StrategyLabImportSummary> {
    imports
        .iter()
        .filter(|item| {
            item.markets.iter().any(|market| market == product_id)
                || item
                    .best_variants
                    .iter()
                    .any(|variant| variant.starts_with(&format!("{product_id}:")))
        })
        .cloned()
        .collect()
}

fn build_listing_radar_rows(state: &DashboardState) -> Vec<ListingRadarRow> {
    let scanner = scanner_map(state);
    let imports = state.coinbase.imports.read().clone();
    let mut rows: Vec<ListingRadarRow> = state
        .coinbase
        .products
        .read()
        .iter()
        .cloned()
        .map(|product| {
            let key = product.product_id.as_str().to_string();
            let scanner_row = scanner.get(&key);
            let stage = listing_stage_for(&product);
            let composite_score = scanner_row.map(|row| row.score).unwrap_or_default();
            let liquidity_score = scanner_row
                .map(|row| ((row.fill_rate_estimate * 0.7) + (1.0 / (row.spread_bps + 1.0)) * 0.3).clamp(0.0, 1.0))
                .unwrap_or(0.15);
            let sentiment_score = scanner_row
                .map(|row| ((row.tape_direction + row.imbalance) / 2.0).clamp(-1.0, 1.0))
                .unwrap_or_default();
            let unlock_risk_score = if product.scan_only { 0.72 } else { 0.28 };
            let route_ready = product.live_tradable || !imports_for_product(&imports, &key).is_empty();
            let priority_fill = scanner_row.map(|row| row.priority_fill).unwrap_or(false);
            let headline = format!(
                "{} is in {} with {} route posture",
                product.product_id.as_str(),
                listing_stage_label(&stage),
                if route_ready { "ready" } else { "research" }
            );
            let mut tags = vec![product.base_currency.clone(), product.quote_currency.clone()];
            tags.push(listing_stage_label(&stage).to_string());
            if priority_fill {
                tags.push("priority_fill".to_string());
            }
            if product.scan_only {
                tags.push("scan_only".to_string());
            }
            if let Some(row) = scanner_row {
                tags.push(row.active_strategy.clone());
            }
            ListingRadarRow {
                product_id: product.product_id.clone(),
                asset_symbol: product.base_currency.clone(),
                base_currency: product.base_currency.clone(),
                quote_currency: product.quote_currency.clone(),
                stage,
                headline,
                composite_score,
                liquidity_score,
                sentiment_score,
                unlock_risk_score,
                route_ready,
                live_tradable: product.live_tradable,
                scan_only: product.scan_only,
                priority_fill,
                tags,
            }
        })
        .collect();
    rows.sort_by(|a, b| {
        b.composite_score
            .partial_cmp(&a.composite_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    rows
}

fn build_listing_radar_detail(
    state: &DashboardState,
    product_id: &str,
) -> Option<ListingRadarDetailView> {
    let product = state
        .coinbase
        .products
        .read()
        .iter()
        .find(|product| product.product_id.as_str() == product_id)
        .cloned()?;
    let scanner = scanner_map(state);
    let scanner_row = scanner.get(product_id).cloned();
    let imports = imports_for_product(&state.coinbase.imports.read().clone(), product_id);
    let stage = listing_stage_for(&product);
    let composite_score = scanner_row.as_ref().map(|row| row.score).unwrap_or_default();
    let liquidity_score = scanner_row
        .as_ref()
        .map(|row| ((row.fill_rate_estimate * 0.7) + (1.0 / (row.spread_bps + 1.0)) * 0.3).clamp(0.0, 1.0))
        .unwrap_or(0.15);
    let sentiment_score = scanner_row
        .as_ref()
        .map(|row| ((row.tape_direction + row.imbalance) / 2.0).clamp(-1.0, 1.0))
        .unwrap_or_default();
    let unlock_risk_score = if product.scan_only { 0.72 } else { 0.28 };
    let route_ready = product.live_tradable || !imports.is_empty();
    let priority_fill = scanner_row.as_ref().map(|row| row.priority_fill).unwrap_or(false);
    let eligibility = scanner_row
        .as_ref()
        .map(|row| row.current_risk_eligibility.clone())
        .unwrap_or_else(|| TradingEligibility {
            product_id: product.product_id.clone(),
            live_tradable: product.live_tradable,
            scan_only: product.scan_only,
            eligible: false,
            reasons: vec!["listing radar item not yet scored by scanner".to_string()],
        });
    let headline = format!(
        "{} is currently in {}",
        product.product_id.as_str(),
        listing_stage_label(&stage)
    );
    let summary = format!(
        "{} routes through a {} execution posture with {} imports loaded.",
        product.product_id.as_str(),
        if route_ready { "ready" } else { "research" },
        imports.len()
    );
    let mut catalysts = vec![format!(
        "Current trading stage is {}.",
        listing_stage_label(&stage)
    )];
    if priority_fill {
        catalysts.push("Scanner currently flags this market for priority fill conditions.".to_string());
    }
    if product.scan_only {
        catalysts.push("Market remains scan-only until venue state and route policy advance.".to_string());
    }
    if !imports.is_empty() {
        catalysts.push(format!(
            "{} strategy-lab imports already reference this market.",
            imports.len()
        ));
    }
    let insights = vec![
        ProviderInsight {
            provider: "Coinbase/CDP".to_string(),
            category: "venue_state".to_string(),
            summary: format!(
                "Venue status is '{}' and live_tradable is {}.",
                product.status,
                product.live_tradable
            ),
            freshness_label: "realtime".to_string(),
            status: if product.live_tradable {
                "strong".to_string()
            } else {
                "watch".to_string()
            },
        },
        ProviderInsight {
            provider: "TradingView/Strategy Lab".to_string(),
            category: "signal_context".to_string(),
            summary: scanner_row
                .as_ref()
                .map(|row| format!(
                    "Active strategy '{}' with score {:.3} and spread {:.2} bps.",
                    row.active_strategy, row.score, row.spread_bps
                ))
                .unwrap_or_else(|| "No scanner vector has been attached yet.".to_string()),
            freshness_label: "polling".to_string(),
            status: if composite_score >= 0.35 {
                "favorable".to_string()
            } else {
                "neutral".to_string()
            },
        },
        ProviderInsight {
            provider: "Dune/DeFiLlama".to_string(),
            category: "research_lane".to_string(),
            summary: "Research adapters are planned to contribute TVL, fee, and wallet growth context here.".to_string(),
            freshness_label: "planned".to_string(),
            status: "planned".to_string(),
        },
    ];
    let routes = vec![
        ListingVenueRoute {
            venue: "Coinbase".to_string(),
            route_type: "cex_primary".to_string(),
            readiness: if product.live_tradable {
                "ready".to_string()
            } else {
                "monitoring".to_string()
            },
            tradable: product.live_tradable,
            notes: if product.live_tradable {
                "Primary route available under current workstation policy.".to_string()
            } else {
                "Await full venue state before live execution.".to_string()
            },
        },
        ListingVenueRoute {
            venue: "DEX simulation".to_string(),
            route_type: "pre_listing".to_string(),
            readiness: if product.scan_only {
                "research".to_string()
            } else {
                "watch".to_string()
            },
            tradable: false,
            notes: "0x and Jupiter adapters should feed this lane in simulation before any multi-chain capital deployment.".to_string(),
        },
        ListingVenueRoute {
            venue: "Strategy replay".to_string(),
            route_type: "validation".to_string(),
            readiness: if !imports.is_empty() {
                "ready".to_string()
            } else {
                "missing_evidence".to_string()
            },
            tradable: false,
            notes: "Replay promotion remains the approval gate before any route can escalate.".to_string(),
        },
    ];

    Some(ListingRadarDetailView {
        product,
        stage,
        headline,
        summary,
        composite_score,
        liquidity_score,
        sentiment_score,
        unlock_risk_score,
        route_ready,
        priority_fill,
        catalysts,
        insights,
        routes,
        eligibility,
        imports,
    })
}

fn build_risk_overview(state: &DashboardState) -> RiskOverviewView {
    ensure_policy_seed(state);
    let approvals = sync_approval_queue(state);
    let risk = state.risk_state.read().clone();
    let scanner = state.coinbase.scanner.read().clone();
    let orders = state.coinbase.orders.read().clone();
    let blocked_markets = scanner
        .iter()
        .filter(|row| !row.current_risk_eligibility.eligible)
        .count();
    let live_eligible_markets = scanner
        .iter()
        .filter(|row| row.live_tradable && row.current_risk_eligibility.eligible && !row.scan_only)
        .count();
    let queued_notional = orders.iter().map(|order| order.quote_notional).sum::<f64>();
    let live_orders = orders.iter().filter(|order| order.live).count();
    let taker_orders = orders
        .iter()
        .filter(|order| matches!(order.route, Some(OrderRoute::Taker)))
        .count();
    let mut policy_breaches = Vec::new();
    if blocked_markets > 0 {
        policy_breaches.push(format!("{blocked_markets} markets are currently blocked by scanner policy."));
    }
    if risk.killswitch != "Running" {
        policy_breaches.push(format!("Kill switch is currently {}.", risk.killswitch));
    }
    if !state.coinbase.live_arm.read().armed {
        policy_breaches.push("Live arm is disarmed; only bounded recommendation flows should proceed.".to_string());
    }
    if taker_orders > 0 {
        policy_breaches.push(format!("{taker_orders} taker orders are queued and should be reviewed against replay evidence."));
    }
    let pending_approvals = approvals.iter().filter(|item| item.status == "pending").count();
    if pending_approvals > 0 {
        policy_breaches.push(format!("{pending_approvals} approval queue items still need operator review."));
    }
    policy_breaches.extend(
        state
            .coinbase
            .policy_events
            .read()
            .iter()
            .rev()
            .take(6)
            .map(format_policy_event_summary),
    );
    RiskOverviewView {
        killswitch: risk.killswitch,
        daily_pnl: risk.daily_pnl,
        open_notional: risk.open_notional,
        unhedged_delta: risk.unhedged_delta,
        blocked_markets,
        live_eligible_markets,
        queued_notional,
        live_orders,
        taker_orders,
        policy_breaches,
    }
}

fn build_agent_console(state: &DashboardState) -> AgentConsoleView {
    ensure_policy_seed(state);
    let live_arm = state.coinbase.live_arm.read().clone();
    let scanner = state.coinbase.scanner.read().clone();
    let imports = state.coinbase.imports.read().clone();
    let approvals = sync_approval_queue(state);
    let blocked_markets = scanner
        .iter()
        .filter(|row| !row.current_risk_eligibility.eligible)
        .count();
    let recommended_products = scanner
        .iter()
        .take(3)
        .map(|row| row.product_id.clone())
        .collect::<Vec<_>>();
    let pending_approvals = approvals.iter().filter(|item| item.status == "pending").count();
    let next_action = if pending_approvals > 0 {
        format!(
            "Work the approval queue first: {} item(s) still require explicit operator review.",
            pending_approvals
        )
    } else if blocked_markets > 0 {
        "Review blocked markets and route them back through replay or scanner policy adjustments.".to_string()
    } else if imports.is_empty() {
        "Import strategy-lab evidence and promote a replay candidate.".to_string()
    } else if !live_arm.armed {
        "Stay in recommend-only mode until a live-arm decision is explicitly recorded.".to_string()
    } else {
        "Monitor top-ranked markets and keep execution inside bounded policy.".to_string()
    };
    AgentConsoleView {
        autonomy_tier: if live_arm.armed {
            "bounded_execute".to_string()
        } else {
            "recommend_only".to_string()
        },
        live_arm,
        next_action,
        blocked_markets,
        imports_loaded: imports.len(),
        recommended_products,
        approvals: approvals.into_iter().take(8).collect(),
    }
}

fn ensure_policy_seed(state: &DashboardState) {
    if !state.coinbase.policy_events.read().is_empty() {
        return;
    }
    record_policy_event(
        state,
        "session_bootstrap",
        "observed",
        format!(
            "Dashboard session initialized in {} mode with kill switch {:?}.",
            state.coinbase.mode.read().clone(),
            *state.kill_switch.read()
        ),
        None,
    );
}

fn record_policy_event(
    state: &DashboardState,
    event_type: &str,
    outcome: &str,
    summary: impl Into<String>,
    product_id: Option<ProductId>,
) {
    const MAX_POLICY_EVENTS: usize = 240;
    let now = Utc::now();
    let mut events = state.coinbase.policy_events.write();
    events.push(StoredPolicyEvent {
        event_id: format!("policy-{}", now.timestamp_millis()),
        event_type: event_type.to_string(),
        outcome: outcome.to_string(),
        summary: summary.into(),
        product_id,
        created_at: now,
    });
    if events.len() > MAX_POLICY_EVENTS {
        let overflow = events.len() - MAX_POLICY_EVENTS;
        events.drain(0..overflow);
    }
}

fn format_policy_event_summary(event: &StoredPolicyEvent) -> String {
    let product = event
        .product_id
        .as_ref()
        .map(|id| format!(" for {}", id.as_str()))
        .unwrap_or_default();
    format!(
        "{} [{}:{}] {}{}",
        event.created_at.format("%H:%M:%S"),
        event.event_type,
        event.outcome,
        event.summary,
        product
    )
}

fn sync_approval_queue(state: &DashboardState) -> Vec<AgentApprovalItem> {
    const MAX_APPROVAL_ITEMS: usize = 120;
    let blocked_products = state
        .coinbase
        .scanner
        .read()
        .iter()
        .filter(|row| !row.current_risk_eligibility.eligible)
        .map(|row| row.product_id.clone())
        .collect::<Vec<_>>();
    let live_armed = state.coinbase.live_arm.read().armed;
    let imports_loaded = !state.coinbase.imports.read().is_empty();

    let mut queue = state.coinbase.approval_queue.write();

    if !live_armed {
        upsert_approval_locked(
            &mut queue,
            AgentApprovalItem {
                id: "approval-live-arm".to_string(),
                title: "Review live arming posture".to_string(),
                description: "Keep autonomy in recommend-only mode until replay, paper, and route evidence are complete.".to_string(),
                severity: "high".to_string(),
                status: "pending".to_string(),
                product_id: None,
            },
        );
    } else {
        set_approval_status_locked(
            &mut queue,
            "approval-live-arm",
            "approved",
            Some("Live-arm posture has already been explicitly approved in-session.".to_string()),
        );
    }

    if !imports_loaded {
        upsert_approval_locked(
            &mut queue,
            AgentApprovalItem {
                id: "approval-imports".to_string(),
                title: "Load strategy evidence".to_string(),
                description: "Import strategy-lab outputs so agent recommendations can point to replayable evidence.".to_string(),
                severity: "medium".to_string(),
                status: "pending".to_string(),
                product_id: None,
            },
        );
    } else {
        set_approval_status_locked(
            &mut queue,
            "approval-imports",
            "approved",
            Some("Strategy-lab evidence is now attached to this workstation session.".to_string()),
        );
    }

    for product in &blocked_products {
        let approval_id = format!("approval-policy-{}", product.as_str());
        upsert_approval_locked(
            &mut queue,
            AgentApprovalItem {
                id: approval_id,
                title: "Resolve policy-blocked market".to_string(),
                description: format!(
                    "{} remains blocked by scanner policy and needs explicit review before any strategy escalation or route change.",
                    product.as_str()
                ),
                severity: "medium".to_string(),
                status: "pending".to_string(),
                product_id: Some(product.clone()),
            },
        );
    }

    for item in queue.iter_mut() {
        if item.id.starts_with("approval-policy-") {
            let still_blocked = item
                .product_id
                .as_ref()
                .map(|product| blocked_products.iter().any(|blocked| blocked == product))
                .unwrap_or(false);
            if !still_blocked && item.status == "pending" {
                item.status = "resolved".to_string();
                item.description = "Scanner no longer reports this market as policy-blocked.".to_string();
            }
        }
    }

    if queue.len() > MAX_APPROVAL_ITEMS {
        let overflow = queue.len() - MAX_APPROVAL_ITEMS;
        queue.drain(0..overflow);
    }

    let mut snapshot = queue.clone();
    snapshot.sort_by_key(|item| approval_rank(&item.status));
    snapshot
}

fn upsert_approval_item(store: &Arc<RwLock<Vec<AgentApprovalItem>>>, item: AgentApprovalItem) {
    let mut queue = store.write();
    upsert_approval_locked(&mut queue, item);
}

fn upsert_approval_locked(queue: &mut Vec<AgentApprovalItem>, item: AgentApprovalItem) {
    if let Some(existing) = queue.iter_mut().find(|current| current.id == item.id) {
        if existing.status == "pending" || item.status == "pending" {
            *existing = item;
        }
    } else {
        queue.push(item);
    }
}

fn set_approval_status(
    store: &Arc<RwLock<Vec<AgentApprovalItem>>>,
    approval_id: &str,
    status: &str,
    description: Option<String>,
) {
    let mut queue = store.write();
    set_approval_status_locked(&mut queue, approval_id, status, description);
}

fn set_approval_status_locked(
    queue: &mut Vec<AgentApprovalItem>,
    approval_id: &str,
    status: &str,
    description: Option<String>,
) {
    if let Some(existing) = queue.iter_mut().find(|current| current.id == approval_id) {
        existing.status = status.to_string();
        if let Some(next_description) = description {
            existing.description = next_description;
        }
    }
}

fn approval_rank(status: &str) -> usize {
    match status {
        "pending" => 0,
        "approved" => 1,
        _ => 2,
    }
}

fn side_label(side: Option<&Side>) -> &'static str {
    match side {
        Some(Side::Buy) => "buy",
        Some(Side::Sell) => "sell",
        None => "unknown",
    }
}

fn route_label(route: Option<&OrderRoute>) -> &'static str {
    match route {
        Some(OrderRoute::Maker) => "maker",
        Some(OrderRoute::Taker) => "taker",
        Some(OrderRoute::ScanOnly) => "scan_only",
        None => "unknown",
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
      <p><a href="/api/v1/listings">/api/v1/listings</a></p>
      <p><a href="/api/v1/risk/overview">/api/v1/risk/overview</a></p>
      <p><a href="/api/v1/agent/console">/api/v1/agent/console</a></p>
    </div>
  </main>
</body>
</html>
"#;
