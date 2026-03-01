use axum::{
    extract::{Json as AxumJson, Query, State},
    response::Html,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Duration as ChronoDuration, Utc};
use parking_lot::RwLock;
use pt_core::{
    AllocationDrift, ApprovalToken, Asset, AuthReloadResult, CoinbaseOrderBookState, EngineMode,
    ExecutionCostAttribution, ExecutionEvent, ExecutionPolicy, ExecutionReport, KillSwitchState,
    MarketHistoryPoint, MarketSelection, MarketSnapshot, MetricsRegistry, RebalancePlan,
    RebalancePlanStatus, RiskState, RouteExecutionPlan, RouteOpportunity, Side, VenueCapability,
    VenueFillQualityStats, VenueLatencyStats, WalletBalance, WalletIntelSnapshot,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

pub trait CoinbaseAuthController: Send + Sync {
    fn status(&self) -> pt_coinbase::CoinbaseAuthStatus;
    fn reload(&self) -> AuthReloadResult;
    fn switch_profile(&self, profile_id: &str) -> AuthReloadResult;
}

#[derive(Clone)]
pub struct DashboardState {
    pub metrics: Arc<MetricsRegistry>,
    pub risk_state: Arc<RwLock<RiskState>>,
    pub kill_switch: Arc<RwLock<KillSwitchState>>,
    pub selected_markets: Arc<RwLock<Vec<MarketSelection>>>,
    pub latest_books: Arc<RwLock<HashMap<String, MarketSnapshot>>>,
    pub market_history: Arc<RwLock<HashMap<String, Vec<MarketHistoryPoint>>>>,
    pub recent_executions: Arc<RwLock<Vec<ExecutionReport>>>,
    pub execution_events: Arc<RwLock<Vec<ExecutionEvent>>>,
    pub execution_costs: Arc<RwLock<Vec<ExecutionCostAttribution>>>,
    pub execution_policy: Arc<RwLock<ExecutionPolicy>>,
    pub fused_bias: Arc<RwLock<HashMap<Asset, f64>>>,
    pub inventory_usd: Arc<RwLock<f64>>,
    pub wallet_balances: Arc<RwLock<Vec<WalletBalance>>>,
    pub wallet_drifts: Arc<RwLock<Vec<AllocationDrift>>>,
    pub wallet_open_orders: Arc<RwLock<Vec<pt_coinbase::CoinbaseOrderSummary>>>,
    pub coinbase_orderbooks: Arc<RwLock<HashMap<String, CoinbaseOrderBookState>>>,
    pub route_opportunities: Arc<RwLock<Vec<RouteOpportunity>>>,
    pub route_executions: Arc<RwLock<Vec<RouteExecutionPlan>>>,
    pub venue_capabilities: Arc<RwLock<Vec<VenueCapability>>>,
    pub coinbase_fee_summary: Arc<RwLock<Option<pt_coinbase::CoinbaseTransactionSummary>>>,
    pub rebalance_plan: Arc<RwLock<Option<RebalancePlan>>>,
    pub rebalance_approval: Arc<RwLock<Option<ApprovalToken>>>,
    pub force_unwind: Arc<RwLock<bool>>,
    pub coinbase_auth_controller: Option<Arc<dyn CoinbaseAuthController>>,
    pub coinbase_wallet_client: Option<Arc<pt_coinbase::CoinbaseWalletClient>>,
    pub coinbase_products: Vec<String>,
    pub engine_mode: EngineMode,
}

impl DashboardState {
    pub fn new(
        metrics: Arc<MetricsRegistry>,
        risk_state: Arc<RwLock<RiskState>>,
        kill_switch: Arc<RwLock<KillSwitchState>>,
        selected_markets: Arc<RwLock<Vec<MarketSelection>>>,
        latest_books: Arc<RwLock<HashMap<String, MarketSnapshot>>>,
        market_history: Arc<RwLock<HashMap<String, Vec<MarketHistoryPoint>>>>,
        recent_executions: Arc<RwLock<Vec<ExecutionReport>>>,
        execution_events: Arc<RwLock<Vec<ExecutionEvent>>>,
        execution_costs: Arc<RwLock<Vec<ExecutionCostAttribution>>>,
        execution_policy: Arc<RwLock<ExecutionPolicy>>,
        fused_bias: Arc<RwLock<HashMap<Asset, f64>>>,
        inventory_usd: Arc<RwLock<f64>>,
        wallet_balances: Arc<RwLock<Vec<WalletBalance>>>,
        wallet_drifts: Arc<RwLock<Vec<AllocationDrift>>>,
        wallet_open_orders: Arc<RwLock<Vec<pt_coinbase::CoinbaseOrderSummary>>>,
        coinbase_orderbooks: Arc<RwLock<HashMap<String, CoinbaseOrderBookState>>>,
        route_opportunities: Arc<RwLock<Vec<RouteOpportunity>>>,
        route_executions: Arc<RwLock<Vec<RouteExecutionPlan>>>,
        venue_capabilities: Arc<RwLock<Vec<VenueCapability>>>,
        coinbase_fee_summary: Arc<RwLock<Option<pt_coinbase::CoinbaseTransactionSummary>>>,
        rebalance_plan: Arc<RwLock<Option<RebalancePlan>>>,
        rebalance_approval: Arc<RwLock<Option<ApprovalToken>>>,
        force_unwind: Arc<RwLock<bool>>,
        coinbase_auth_controller: Option<Arc<dyn CoinbaseAuthController>>,
        coinbase_wallet_client: Option<Arc<pt_coinbase::CoinbaseWalletClient>>,
        coinbase_products: Vec<String>,
        engine_mode: EngineMode,
    ) -> Self {
        Self {
            metrics,
            risk_state,
            kill_switch,
            selected_markets,
            latest_books,
            market_history,
            recent_executions,
            execution_events,
            execution_costs,
            execution_policy,
            fused_bias,
            inventory_usd,
            wallet_balances,
            wallet_drifts,
            wallet_open_orders,
            coinbase_orderbooks,
            route_opportunities,
            route_executions,
            venue_capabilities,
            coinbase_fee_summary,
            rebalance_plan,
            rebalance_approval,
            force_unwind,
            coinbase_auth_controller,
            coinbase_wallet_client,
            coinbase_products,
            engine_mode,
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
struct ExecutionVectorsView {
    entry_max_slippage_bps: f64,
    exit_max_slippage_bps: f64,
    entry_offset_bps: f64,
    exit_offset_bps: f64,
    max_cross_bps_unwind: f64,
}

#[derive(Debug, Clone, Serialize)]
struct RebalancePlanView {
    plan_id: String,
    status: String,
    intent_count: usize,
    total_drift_abs_usd: f64,
    created_ts: String,
    expires_ts: String,
    approval_token_id: Option<String>,
    approval_required: bool,
    approval_expires_ts: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct RebalanceApprovalRequest {
    token_id: String,
}

#[derive(Debug, Clone, Serialize)]
struct RebalanceApprovalResponse {
    ok: bool,
    plan_id: Option<String>,
    token_id: Option<String>,
    status: String,
}

#[derive(Debug, Clone, Deserialize)]
struct CoinbaseAuthSwitchRequest {
    profile_id: String,
}

#[derive(Debug, Clone, Serialize)]
struct MarketView {
    market_id: String,
    token_id: String,
    display_name: String,
    pair_product: String,
    asset: String,
    bucket: String,
    question: String,
    bid: f64,
    ask: f64,
    spread: f64,
    mid: f64,
    ts: String,
}

#[derive(Debug, Clone, Deserialize)]
struct CoinbaseConvertRequest {
    account_id: Option<String>,
    from_asset: String,
    to_asset: String,
    amount: Option<f64>,
    live: Option<bool>,
    confirm: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct CoinbaseConvertResponse {
    ok: bool,
    mode: String,
    from_asset: String,
    to_asset: String,
    account_id: Option<String>,
    product_id: Option<String>,
    side: Option<String>,
    amount_from: f64,
    amount_base: f64,
    limit_price: f64,
    est_quote: f64,
    order_id: Option<String>,
    preview_ok: bool,
    message: String,
}

#[derive(Debug, Clone, Deserialize)]
struct CoinbaseMakerTestRequest {
    product_id: String,
    side: String,
    base_size: f64,
    live: Option<bool>,
    confirm: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct CoinbaseMakerTestResponse {
    ok: bool,
    mode: String,
    product_id: String,
    side: String,
    preview_ok: bool,
    order_id: Option<String>,
    limit_price: f64,
    preview_ms: f64,
    post_ms: f64,
    cancel_ms: f64,
    total_ms: f64,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
struct MarketHistoryResponse {
    market_id: Option<String>,
    points: Vec<MarketHistoryPoint>,
}

#[derive(Debug, Clone, Deserialize)]
struct HistoryQuery {
    market_id: Option<String>,
    limit: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
struct ListingCandidatesQuery {
    window: Option<String>,
    granularity_sec: Option<u32>,
    max_scan: Option<usize>,
    max_results: Option<usize>,
    min_candles: Option<usize>,
    min_gap_candles: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
struct ListingManualAnchor {
    product_id: String,
    anchor_time: String,
    label: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ListingOverlayRequest {
    product_ids: Option<Vec<String>>,
    window_preset: Option<String>,
    granularity_sec: Option<u32>,
    alignment_mode: Option<String>,
    normalization: Option<String>,
    max_scan: Option<usize>,
    min_candles: Option<usize>,
    min_gap_candles: Option<usize>,
    manual_anchors: Option<Vec<ListingManualAnchor>>,
}

#[derive(Debug, Clone, Serialize)]
struct ListingCandidateView {
    product_id: String,
    label: String,
    quote_currency: String,
    source: String,
    anchor_time: String,
    first_window_volume: f64,
    confidence_score: f64,
}

#[derive(Debug, Clone, Serialize)]
struct ListingCandidatesResponse {
    window_preset: String,
    granularity_sec: u32,
    candidates: Vec<ListingCandidateView>,
    diagnostics: Vec<String>,
    fetched_ts: String,
}

#[derive(Debug, Clone, Serialize)]
struct ListingOverlaySeries {
    product_id: String,
    label: String,
    source: String,
    anchor_time: Option<String>,
    values: Vec<f64>,
    ts: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct ListingSummaryRow {
    product_id: String,
    label: String,
    source: String,
    anchor_time: Option<String>,
    ret_1: f64,
    ret_3: f64,
    ret_10: f64,
}

#[derive(Debug, Clone, Serialize)]
struct ListingOverlayResponse {
    ok: bool,
    window_preset: String,
    granularity_sec: u32,
    alignment_mode: String,
    normalization: String,
    x_axis: Vec<String>,
    series: Vec<ListingOverlaySeries>,
    summary_rows: Vec<ListingSummaryRow>,
    diagnostics: Vec<String>,
    errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct L2ArchiveRow {
    product_id: String,
    sequence_num: i64,
    bid_levels: usize,
    ask_levels: usize,
    last_event_ts: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct L2ArchiveResponse {
    rows: Vec<L2ArchiveRow>,
    l2_update_count: f64,
    sequence_gap_count: f64,
}

#[derive(Debug, Clone, Serialize)]
struct FeedHealthResponse {
    ws_healthy: bool,
    heartbeat_seq: f64,
    heartbeat_age_ms: Option<i64>,
    l2_updates: f64,
    user_updates: f64,
    reconnects: f64,
    sequence_gaps: f64,
    errors: f64,
    timeout_streak: f64,
    read_timeouts: f64,
    ping_failures: f64,
    heartbeat_timeouts: f64,
    remote_closes: f64,
    read_errors: f64,
    connect_failures: f64,
    reject_events_10m: usize,
    cancel_events_10m: usize,
}

#[derive(Debug, Clone, Serialize)]
struct FeedDiagnosticsResponse {
    ws_healthy: bool,
    stale_threshold_ms: u64,
    heartbeat_age_ms: Option<i64>,
    heartbeat_seq: f64,
    l2_updates: f64,
    user_updates: f64,
    reconnects: f64,
    sequence_gaps: f64,
    errors: f64,
    timeout_streak: f64,
    read_timeouts: f64,
    ping_failures: f64,
    heartbeat_timeouts: f64,
    remote_closes: f64,
    read_errors: f64,
    connect_failures: f64,
    reject_events_10m: usize,
    cancel_events_10m: usize,
    reject_rate_10m: f64,
    cancel_rate_10m: f64,
}

#[derive(Debug, Clone, Serialize)]
struct ParityMonitorRow {
    route_id: String,
    path: String,
    strategy_class: String,
    gross_edge_bps: f64,
    expected_net_bps: f64,
    min_required_bps: f64,
    pass: bool,
    reasons: Vec<String>,
    expected_usd_profit: f64,
    capital_required_usd: f64,
    ts: String,
}

#[derive(Debug, Clone, Serialize)]
struct ParityMonitorResponse {
    feed_stale: bool,
    rows: Vec<ParityMonitorRow>,
}

#[derive(Debug, Clone, Deserialize)]
struct ParityCsvExportRequest {
    limit: Option<usize>,
    include_failures: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
struct ParityCsvExportResponse {
    ok: bool,
    row_count: usize,
    file_path: Option<String>,
    message: String,
}

#[derive(Debug, Clone, Deserialize)]
struct RouteCsvExportRequest {
    limit: Option<usize>,
    min_expected_net_bps: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
struct RouteCsvExportResponse {
    ok: bool,
    row_count: usize,
    file_path: Option<String>,
    message: String,
}

#[derive(Debug, Clone, Deserialize)]
struct WalletIntelExportRequest {
    source: Option<String>,
    limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
struct WalletIntelExportResponse {
    ok: bool,
    row_count: usize,
    file_path: Option<String>,
    message: String,
}

#[derive(Debug, Clone)]
struct ListingCandidateInternal {
    product_id: String,
    label: String,
    quote_currency: String,
    source: String,
    anchor_time: DateTime<Utc>,
    first_window_volume: f64,
    confidence_score: f64,
}

#[derive(Debug, Clone)]
struct CoinbasePublicCandle {
    ts: DateTime<Utc>,
    close: f64,
    volume: f64,
}

pub fn router(state: DashboardState) -> Router {
    Router::new()
        .route("/", get(get_dashboard))
        .route("/health", get(get_health))
        .route("/healthz", get(get_health))
        .route("/ready", get(get_health))
        .route("/metrics", get(get_metrics))
        .route("/state/risk", get(get_risk_state))
        .route("/state/books", get(get_books))
        .route("/state/markets", get(get_markets))
        .route("/state/history", get(get_market_history))
        .route("/state/listings/candidates", get(get_listing_candidates))
        .route("/state/listings/overlay", post(post_listing_overlay))
        .route("/state/listings/l2-archive", get(get_listing_l2_archive))
        .route("/state/feed/health", get(get_feed_health))
        .route("/state/feed/diagnostics", get(get_feed_diagnostics))
        .route("/state/parity/monitor", get(get_parity_monitor))
        .route("/state/parity/export-csv", post(post_parity_export_csv))
        .route("/state/venues/capabilities", get(get_venue_capabilities))
        .route("/state/venues/latency", get(get_venue_latency))
        .route("/state/venues/fill-quality", get(get_venue_fill_quality))
        .route("/state/venues/rejects", get(get_venue_fill_quality))
        .route("/state/executions", get(get_executions))
        .route("/state/execution/orders", get(get_execution_orders))
        .route("/state/execution/costs", get(get_execution_costs))
        .route("/state/execution/vectors", get(get_execution_vectors))
        .route("/state/bias", get(get_bias))
        .route("/state/inventory", get(get_inventory))
        .route("/state/coinbase/wallet", get(get_coinbase_wallet))
        .route("/state/coinbase/allocations", get(get_coinbase_allocations))
        .route("/state/coinbase/orderbook", get(get_coinbase_orderbook))
        .route("/state/coinbase/auth", get(get_coinbase_auth))
        .route(
            "/state/coinbase/rebalance-plan",
            get(get_coinbase_rebalance_plan),
        )
        .route("/state/coinbase/orders", get(get_coinbase_orders))
        .route("/state/routes/opportunities", get(get_route_opportunities))
        .route("/state/routes/executions", get(get_route_executions))
        .route("/state/routes/export-csv", post(post_routes_export_csv))
        .route("/state/fees/summary", get(get_fees_summary))
        .route(
            "/state/wallet-intel/coinbase",
            get(get_wallet_intel_coinbase),
        )
        .route(
            "/state/wallet-intel/polymarket",
            get(get_wallet_intel_polymarket),
        )
        .route(
            "/state/wallet-intel/leaderboard",
            get(get_wallet_intel_leaderboard),
        )
        .route(
            "/state/wallet-intel/export-csv",
            post(post_wallet_intel_export_csv),
        )
        .route("/ops/halt", post(post_halt))
        .route("/ops/resume", post(post_resume))
        .route("/ops/flatten", post(post_flatten))
        .route(
            "/ops/profile/pilot-ultra-tight",
            post(post_profile_pilot_ultra_tight),
        )
        .route(
            "/ops/coinbase/rebalance/approve",
            post(post_coinbase_rebalance_approve),
        )
        .route(
            "/ops/coinbase/rebalance/reject",
            post(post_coinbase_rebalance_reject),
        )
        .route("/ops/coinbase/auth/reload", post(post_coinbase_auth_reload))
        .route(
            "/ops/coinbase/auth/switch-profile",
            post(post_coinbase_auth_switch_profile),
        )
        .route(
            "/ops/coinbase/convert/preview",
            post(post_coinbase_convert_preview),
        )
        .route(
            "/ops/coinbase/convert/execute",
            post(post_coinbase_convert_execute),
        )
        .route("/ops/coinbase/maker-test", post(post_coinbase_maker_test))
        .route("/ops/execution/unwind", post(post_execution_unwind))
        .route("/ops/unwind/now", post(post_execution_unwind))
        .with_state(state)
}

async fn get_dashboard() -> Html<&'static str> {
    Html(DASHBOARD_HTML)
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
    let selected = state.selected_markets.read();
    let meta_by_market: HashMap<String, &MarketSelection> =
        selected.iter().map(|m| (m.market_id.clone(), m)).collect();

    let mut rows: Vec<MarketView> = state
        .latest_books
        .read()
        .values()
        .map(|b| {
            let meta = meta_by_market.get(&b.market_id).copied();
            let pair_product = meta
                .and_then(|m| m.asset.as_product_id().map(str::to_string))
                .unwrap_or_else(|| "UNKNOWN-USD".to_string());
            let asset = meta
                .map(|m| m.asset.as_str().to_string())
                .unwrap_or_else(|| "UNK".to_string());
            let bucket = meta
                .map(bucket_label)
                .unwrap_or_else(|| "other".to_string());
            let question = meta
                .map(|m| m.question.clone())
                .unwrap_or_else(|| b.market_id.clone());
            let display_name = format!("{pair_product} | {bucket}");
            MarketView {
                market_id: b.market_id.clone(),
                token_id: b.token_id.clone(),
                display_name,
                pair_product,
                asset,
                bucket,
                question,
                bid: b.bid,
                ask: b.ask,
                spread: b.spread,
                mid: (b.bid + b.ask) / 2.0,
                ts: b.ts.to_rfc3339(),
            }
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

#[derive(Debug, Clone, Deserialize)]
struct CoinbaseExchangeProduct {
    id: String,
    quote_currency: Option<String>,
    status: Option<String>,
    trading_disabled: Option<bool>,
}

fn parse_window_preset(raw: Option<&str>) -> (String, i64) {
    let text = raw.unwrap_or("90d").trim().to_ascii_lowercase();
    if text.is_empty() {
        return ("90d".to_string(), 90);
    }
    let unit = text.chars().last().unwrap_or('d');
    let num_text = text
        .trim_end_matches(|c: char| c.is_ascii_alphabetic())
        .trim();
    if let Ok(n) = num_text.parse::<i64>() {
        let days = match unit {
            'd' => n,
            'w' => n * 7,
            'm' => n * 30,
            _ => n,
        }
        .clamp(1, 720);
        return (format!("{n}{unit}"), days);
    }
    ("90d".to_string(), 90)
}

fn normalize_granularity(sec: Option<u32>) -> u32 {
    let requested = sec.unwrap_or(14_400).clamp(60, 86_400);
    let allowed = [60_u32, 300, 900, 3_600, 21_600, 86_400];
    let mut best = allowed[0];
    let mut best_delta = i64::MAX;
    for candidate in allowed {
        let delta = (requested as i64 - candidate as i64).abs();
        if delta < best_delta {
            best = candidate;
            best_delta = delta;
        }
    }
    best
}

fn normalize_alignment_mode(raw: Option<&str>) -> String {
    match raw
        .unwrap_or("entry_aligned")
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "calendar" | "calendar_aligned" => "calendar_aligned".to_string(),
        "cohort" | "cohort_start" | "cohort_start_aligned" | "start_all" => {
            "cohort_start_aligned".to_string()
        }
        _ => "entry_aligned".to_string(),
    }
}

fn normalize_series_mode(raw: Option<&str>) -> String {
    match raw
        .unwrap_or("indexed")
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "return" | "returns" | "pct" | "percent" => "returns".to_string(),
        _ => "indexed".to_string(),
    }
}

fn overlay_values(candles: &[CoinbasePublicCandle], normalization: &str) -> Option<Vec<f64>> {
    let base = candles.first()?.close.abs();
    if base <= 1e-12 {
        return None;
    }
    Some(
        candles
            .iter()
            .map(|c| {
                let ratio = c.close / base;
                if normalization == "returns" {
                    ratio - 1.0
                } else {
                    ratio
                }
            })
            .collect(),
    )
}

fn sample_return(candles: &[CoinbasePublicCandle], idx: usize) -> Option<f64> {
    let base = candles.first()?.close.abs();
    if base <= 1e-12 {
        return None;
    }
    let i = idx.min(candles.len().saturating_sub(1));
    Some(candles.get(i)?.close / base - 1.0)
}

fn summary_returns(candles: &[CoinbasePublicCandle]) -> Option<(f64, f64, f64)> {
    Some((
        sample_return(candles, 1)?,
        sample_return(candles, 3)?,
        sample_return(candles, 10)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        normalize_alignment_mode, normalize_granularity, normalize_series_mode, overlay_values,
        parse_window_preset, sample_return, summary_returns, CoinbasePublicCandle,
    };
    use chrono::{DateTime, Utc};

    #[test]
    fn window_preset_defaults_and_aliases() {
        assert_eq!(parse_window_preset(None), ("90d".to_string(), 90));
        assert_eq!(parse_window_preset(Some("30d")), ("30d".to_string(), 30));
        assert_eq!(parse_window_preset(Some("1m")), ("1m".to_string(), 30));
        assert_eq!(parse_window_preset(Some("3m")), ("3m".to_string(), 90));
        assert_eq!(parse_window_preset(Some("6m")), ("6m".to_string(), 180));
        assert_eq!(
            parse_window_preset(Some("unknown")),
            ("90d".to_string(), 90)
        );
    }

    #[test]
    fn granularity_normalization_clamps_to_supported_values() {
        assert_eq!(normalize_granularity(None), 21_600);
        assert_eq!(normalize_granularity(Some(3_599)), 3_600);
        assert_eq!(normalize_granularity(Some(3_600)), 3_600);
        assert_eq!(normalize_granularity(Some(14_401)), 21_600);
        assert_eq!(normalize_granularity(Some(90_000)), 86_400);
    }

    #[test]
    fn alignment_mode_normalization_aliases() {
        assert_eq!(
            normalize_alignment_mode(Some("calendar")),
            "calendar_aligned".to_string()
        );
        assert_eq!(
            normalize_alignment_mode(Some("start_all")),
            "cohort_start_aligned".to_string()
        );
        assert_eq!(
            normalize_alignment_mode(Some("cohort_start")),
            "cohort_start_aligned".to_string()
        );
        assert_eq!(
            normalize_alignment_mode(Some("entry_aligned")),
            "entry_aligned".to_string()
        );
        assert_eq!(
            normalize_alignment_mode(Some("bad-value")),
            "entry_aligned".to_string()
        );
    }

    #[test]
    fn series_mode_normalization_aliases() {
        assert_eq!(
            normalize_series_mode(Some("returns")),
            "returns".to_string()
        );
        assert_eq!(
            normalize_series_mode(Some("percent")),
            "returns".to_string()
        );
        assert_eq!(
            normalize_series_mode(Some("indexed")),
            "indexed".to_string()
        );
        assert_eq!(
            normalize_series_mode(Some("unknown")),
            "indexed".to_string()
        );
    }

    #[test]
    fn overlay_helpers_produce_deterministic_values() {
        let mk = |ts: i64, close: f64| CoinbasePublicCandle {
            ts: DateTime::<Utc>::from_timestamp(ts, 0).expect("ts"),
            close,
            volume: 1.0,
        };
        let candles = vec![
            mk(1_700_000_000, 100.0),
            mk(1_700_000_060, 102.0),
            mk(1_700_000_120, 101.0),
            mk(1_700_000_180, 105.0),
            mk(1_700_000_240, 108.0),
        ];

        let indexed = overlay_values(&candles, "indexed").expect("indexed");
        let returns = overlay_values(&candles, "returns").expect("returns");
        assert_eq!(indexed[0], 1.0);
        assert!((indexed[3] - 1.05).abs() < 1e-9);
        assert_eq!(returns[0], 0.0);
        assert!((returns[4] - 0.08).abs() < 1e-9);
        assert!((sample_return(&candles, 1).expect("r1") - 0.02).abs() < 1e-9);

        let (r1, r3, r10) = summary_returns(&candles).expect("summary");
        assert!((r1 - 0.02).abs() < 1e-9);
        assert!((r3 - 0.05).abs() < 1e-9);
        assert!((r10 - 0.08).abs() < 1e-9);
    }
}

async fn fetch_coinbase_public_products(
    client: &Client,
) -> Result<Vec<CoinbaseExchangeProduct>, String> {
    let resp = client
        .get("https://api.exchange.coinbase.com/products")
        .header("user-agent", "Polymarket-Trader-Dashboard/1.0")
        .send()
        .await
        .map_err(|e| format!("products request failed: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("products request status {}", resp.status()));
    }
    resp.json::<Vec<CoinbaseExchangeProduct>>()
        .await
        .map_err(|e| format!("products decode failed: {e}"))
}

async fn fetch_coinbase_public_candles(
    client: &Client,
    product_id: &str,
    granularity_sec: u32,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    limit: usize,
) -> Result<Vec<CoinbasePublicCandle>, String> {
    let url = format!(
        "https://api.exchange.coinbase.com/products/{}/candles?granularity={}&start={}&end={}",
        product_id,
        granularity_sec,
        start.to_rfc3339(),
        end.to_rfc3339()
    );
    let resp = client
        .get(url)
        .header("user-agent", "Polymarket-Trader-Dashboard/1.0")
        .send()
        .await
        .map_err(|e| format!("candles request failed for {product_id}: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!(
            "candles request status {} for {}",
            resp.status(),
            product_id
        ));
    }
    let raw = resp
        .json::<Vec<Vec<f64>>>()
        .await
        .map_err(|e| format!("candles decode failed for {product_id}: {e}"))?;

    let mut rows = Vec::new();
    for item in raw {
        if item.len() < 6 {
            continue;
        }
        let ts_s = item[0] as i64;
        let close = item[4];
        let volume = item[5];
        let Some(ts) = DateTime::<Utc>::from_timestamp(ts_s, 0) else {
            continue;
        };
        rows.push(CoinbasePublicCandle { ts, close, volume });
    }
    rows.sort_by(|a, b| a.ts.cmp(&b.ts));
    if rows.len() > limit {
        let keep_from = rows.len() - limit;
        rows = rows.split_off(keep_from);
    }
    Ok(rows)
}

async fn discover_listing_candidates(
    window_days: i64,
    granularity_sec: u32,
    max_scan: usize,
    max_results: usize,
    min_candles: usize,
    min_gap_candles: usize,
) -> (Vec<ListingCandidateInternal>, Vec<String>) {
    let client = Client::new();
    let mut diagnostics = Vec::new();
    let products = match fetch_coinbase_public_products(&client).await {
        Ok(v) => v,
        Err(e) => return (Vec::new(), vec![e]),
    };

    let mut eligible: Vec<CoinbaseExchangeProduct> = products
        .into_iter()
        .filter(|p| p.id.contains('-'))
        .filter(|p| !p.trading_disabled.unwrap_or(false))
        .collect();
    eligible.sort_by(|a, b| a.id.cmp(&b.id));

    let now = Utc::now();
    let start = now - ChronoDuration::days(window_days);
    let min_gap_ms = (min_gap_candles as i64) * (granularity_sec as i64) * 1000;
    let threshold = start + ChronoDuration::milliseconds(min_gap_ms);

    let mut out = Vec::new();
    let mut scanned = 0usize;
    let candles_limit =
        (((window_days as i64 * 86_400) / granularity_sec as i64) + 32).clamp(32, 1_200) as usize;

    for product in eligible.into_iter().take(max_scan) {
        scanned += 1;
        let candles = match fetch_coinbase_public_candles(
            &client,
            &product.id,
            granularity_sec,
            start,
            now,
            candles_limit,
        )
        .await
        {
            Ok(v) => v,
            Err(_) => continue,
        };
        if candles.len() < min_candles {
            continue;
        }
        let Some(first) = candles.first() else {
            continue;
        };
        if first.ts <= threshold {
            continue;
        }
        let first_window_volume = candles.iter().take(5).map(|c| c.volume).sum::<f64>();
        let age_days = ((now - first.ts).num_hours() as f64 / 24.0).max(0.0);
        let recency = (1.0 - (age_days / window_days.max(1) as f64)).clamp(0.0, 1.0);
        let volume_score = ((first_window_volume.abs() + 1.0).ln() / 10.0).clamp(0.0, 1.0);
        let confidence_score = (0.65 * recency + 0.35 * volume_score).clamp(0.0, 1.0);
        out.push(ListingCandidateInternal {
            product_id: product.id.clone(),
            label: product.id.split('-').next().unwrap_or("UNK").to_string(),
            quote_currency: product
                .quote_currency
                .unwrap_or_else(|| "UNK".to_string())
                .to_ascii_uppercase(),
            source: "auto_discovery".to_string(),
            anchor_time: first.ts,
            first_window_volume,
            confidence_score,
        });
    }

    out.sort_by(|a, b| {
        b.anchor_time
            .cmp(&a.anchor_time)
            .then_with(|| b.first_window_volume.total_cmp(&a.first_window_volume))
    });
    if out.len() > max_results {
        out.truncate(max_results);
    }

    diagnostics.push(format!(
        "listing auto-discovery scanned={}, selected={}",
        scanned,
        out.len()
    ));
    (out, diagnostics)
}

async fn get_listing_candidates(
    Query(query): Query<ListingCandidatesQuery>,
) -> Json<ListingCandidatesResponse> {
    let (window_preset, window_days) = parse_window_preset(query.window.as_deref());
    let granularity_sec = normalize_granularity(query.granularity_sec);
    let max_scan = query.max_scan.unwrap_or(320).clamp(10, 1_500);
    let max_results = query.max_results.unwrap_or(80).clamp(5, 400);
    let min_candles = query.min_candles.unwrap_or(20).clamp(3, 1_000);
    let min_gap = query.min_gap_candles.unwrap_or(2).clamp(0, 500);

    let (candidates, diagnostics) = discover_listing_candidates(
        window_days,
        granularity_sec,
        max_scan,
        max_results,
        min_candles,
        min_gap,
    )
    .await;

    Json(ListingCandidatesResponse {
        window_preset,
        granularity_sec,
        candidates: candidates
            .into_iter()
            .map(|c| ListingCandidateView {
                product_id: c.product_id,
                label: c.label,
                quote_currency: c.quote_currency,
                source: c.source,
                anchor_time: c.anchor_time.to_rfc3339(),
                first_window_volume: c.first_window_volume,
                confidence_score: c.confidence_score,
            })
            .collect(),
        diagnostics,
        fetched_ts: Utc::now().to_rfc3339(),
    })
}

async fn post_listing_overlay(
    AxumJson(req): AxumJson<ListingOverlayRequest>,
) -> Json<ListingOverlayResponse> {
    let (window_preset, window_days) = parse_window_preset(req.window_preset.as_deref());
    let granularity_sec = normalize_granularity(req.granularity_sec);
    let alignment_mode = normalize_alignment_mode(req.alignment_mode.as_deref());
    let normalization = normalize_series_mode(req.normalization.as_deref());
    let max_scan = req.max_scan.unwrap_or(320).clamp(10, 1_500);
    let min_candles = req.min_candles.unwrap_or(20).clamp(3, 1_000);
    let min_gap = req.min_gap_candles.unwrap_or(2).clamp(0, 500);

    let (candidates, mut diagnostics) = discover_listing_candidates(
        window_days,
        granularity_sec,
        max_scan,
        200,
        min_candles,
        min_gap,
    )
    .await;
    let mut anchor_map: HashMap<String, ListingCandidateInternal> = candidates
        .iter()
        .map(|c| (c.product_id.clone(), c.clone()))
        .collect();

    if let Some(manual) = req.manual_anchors.as_ref() {
        for m in manual {
            let ts = match DateTime::parse_from_rfc3339(m.anchor_time.trim()) {
                Ok(v) => v.with_timezone(&Utc),
                Err(_) => continue,
            };
            let product_id = m.product_id.trim().to_ascii_uppercase();
            if product_id.is_empty() {
                continue;
            }
            let label = m
                .label
                .clone()
                .unwrap_or_else(|| product_id.split('-').next().unwrap_or("UNK").to_string());
            anchor_map.insert(
                product_id.clone(),
                ListingCandidateInternal {
                    product_id,
                    label,
                    quote_currency: "UNK".to_string(),
                    source: "manual".to_string(),
                    anchor_time: ts,
                    first_window_volume: 0.0,
                    confidence_score: 1.0,
                },
            );
        }
    }

    let selected_products = req.product_ids.unwrap_or_else(|| {
        candidates
            .iter()
            .take(12)
            .map(|c| c.product_id.clone())
            .collect()
    });
    if selected_products.is_empty() {
        return Json(ListingOverlayResponse {
            ok: false,
            window_preset,
            granularity_sec,
            alignment_mode,
            normalization,
            x_axis: Vec::new(),
            series: Vec::new(),
            summary_rows: Vec::new(),
            diagnostics,
            errors: vec!["no products selected".to_string()],
        });
    }

    let now = Utc::now();
    let window_span = ChronoDuration::days(window_days);
    let mut errors = Vec::new();
    let mut series_rows = Vec::new();
    let mut summary_rows = Vec::new();
    let client = Client::new();

    let cohort_start = if alignment_mode == "cohort_start_aligned" {
        selected_products
            .iter()
            .filter_map(|p| {
                anchor_map
                    .get(&p.to_ascii_uppercase())
                    .map(|c| c.anchor_time)
            })
            .max()
    } else {
        None
    };
    if alignment_mode == "cohort_start_aligned" && cohort_start.is_none() {
        errors.push("cohort_start_aligned requires anchor times for selected products".to_string());
    }

    let mut max_len = 0usize;
    for product in selected_products {
        let product_id = product.trim().to_ascii_uppercase();
        if product_id.is_empty() {
            continue;
        }
        let candidate = anchor_map.get(&product_id).cloned();
        let anchor = candidate.as_ref().map(|c| c.anchor_time);
        let (start, end) = match alignment_mode.as_str() {
            "calendar_aligned" => (now - window_span, now),
            "cohort_start_aligned" => {
                if let Some(c) = cohort_start {
                    (c, c + window_span)
                } else {
                    continue;
                }
            }
            _ => {
                let a = anchor.unwrap_or(now - window_span);
                (a, a + window_span)
            }
        };
        let limit =
            (((window_days * 86_400) / granularity_sec as i64) + 32).clamp(32, 1_200) as usize;
        let candles = match fetch_coinbase_public_candles(
            &client,
            &product_id,
            granularity_sec,
            start,
            end,
            limit,
        )
        .await
        {
            Ok(v) => v,
            Err(e) => {
                errors.push(format!("{product_id}: {e}"));
                continue;
            }
        };
        if candles.len() < 3 {
            errors.push(format!("{product_id}: insufficient candles"));
            continue;
        }
        let Some(values) = overlay_values(&candles, &normalization) else {
            errors.push(format!("{product_id}: invalid base price"));
            continue;
        };
        let ts: Vec<String> = candles.iter().map(|c| c.ts.to_rfc3339()).collect();
        max_len = max_len.max(values.len());

        let (ret_1, ret_3, ret_10) = match summary_returns(&candles) {
            Some(v) => v,
            None => {
                errors.push(format!("{product_id}: summary return computation failed"));
                continue;
            }
        };
        summary_rows.push(ListingSummaryRow {
            product_id: product_id.clone(),
            label: candidate
                .as_ref()
                .map(|c| c.label.clone())
                .unwrap_or_else(|| product_id.split('-').next().unwrap_or("UNK").to_string()),
            source: candidate
                .as_ref()
                .map(|c| c.source.clone())
                .unwrap_or_else(|| "fallback".to_string()),
            anchor_time: anchor.map(|a| a.to_rfc3339()),
            ret_1,
            ret_3,
            ret_10,
        });
        series_rows.push(ListingOverlaySeries {
            product_id: product_id.clone(),
            label: candidate
                .as_ref()
                .map(|c| c.label.clone())
                .unwrap_or_else(|| product_id.split('-').next().unwrap_or("UNK").to_string()),
            source: candidate
                .as_ref()
                .map(|c| c.source.clone())
                .unwrap_or_else(|| "fallback".to_string()),
            anchor_time: anchor.map(|a| a.to_rfc3339()),
            values,
            ts,
        });
    }

    let x_axis = if alignment_mode == "entry_aligned" {
        (0..max_len).map(|i| i.to_string()).collect::<Vec<_>>()
    } else {
        let start = if alignment_mode == "calendar_aligned" {
            now - window_span
        } else {
            cohort_start.unwrap_or(now - window_span)
        };
        (0..max_len)
            .map(|i| {
                (start + ChronoDuration::seconds((i as i64) * granularity_sec as i64)).to_rfc3339()
            })
            .collect::<Vec<_>>()
    };

    diagnostics.push(format!(
        "overlay built products={} series={} alignment={} normalization={}",
        series_rows.len(),
        series_rows.len(),
        alignment_mode,
        normalization
    ));

    Json(ListingOverlayResponse {
        ok: !series_rows.is_empty(),
        window_preset,
        granularity_sec,
        alignment_mode,
        normalization,
        x_axis,
        series: series_rows,
        summary_rows,
        diagnostics,
        errors,
    })
}

async fn get_listing_l2_archive(State(state): State<DashboardState>) -> Json<L2ArchiveResponse> {
    let mut rows: Vec<L2ArchiveRow> = state
        .coinbase_orderbooks
        .read()
        .values()
        .map(|book| L2ArchiveRow {
            product_id: book.product_id.clone(),
            sequence_num: book.sequence_num,
            bid_levels: book.bids.len(),
            ask_levels: book.asks.len(),
            last_event_ts: book.last_event_ts.map(|ts| ts.to_rfc3339()),
        })
        .collect();
    rows.sort_by(|a, b| a.product_id.cmp(&b.product_id));
    Json(L2ArchiveResponse {
        rows,
        l2_update_count: state.metrics.get_counter("coinbase_ws_l2_updates"),
        sequence_gap_count: state.metrics.get_counter("coinbase_ws_sequence_gap"),
    })
}

fn compute_feed_stale(
    metrics: &MetricsRegistry,
    stale_threshold_ms: u64,
) -> (bool, Option<i64>, f64) {
    let seq = metrics.get_gauge("coinbase_ws_last_heartbeat_seq");
    let heartbeat_ts_ms = metrics.get_gauge("coinbase_ws_last_heartbeat_ts_ms");
    let l2_ts_ms = metrics.get_gauge("coinbase_ws_last_l2_ts_ms");
    let ts_ms = heartbeat_ts_ms.max(l2_ts_ms);
    if ts_ms <= 0.0 {
        return (true, None, seq);
    }
    let age_ms = (Utc::now().timestamp_millis() as f64 - ts_ms).max(0.0) as i64;
    (
        age_ms as u64 > stale_threshold_ms.max(1_000),
        Some(age_ms),
        seq,
    )
}

fn recent_reject_cancel_stats(state: &DashboardState) -> (usize, usize, f64, f64) {
    let cutoff = Utc::now() - ChronoDuration::minutes(10);
    let mut reject_events = 0usize;
    let mut cancel_events = 0usize;
    let mut total = 0usize;
    for ev in state.execution_events.read().iter() {
        if ev.ts < cutoff {
            continue;
        }
        total += 1;
        match ev.state {
            pt_core::OrderLifecycleState::Rejected => reject_events += 1,
            pt_core::OrderLifecycleState::Canceled => cancel_events += 1,
            _ => {}
        }
    }
    if total == 0 {
        return (reject_events, cancel_events, 0.0, 0.0);
    }
    (
        reject_events,
        cancel_events,
        reject_events as f64 / total as f64,
        cancel_events as f64 / total as f64,
    )
}

async fn get_feed_health(State(state): State<DashboardState>) -> Json<FeedHealthResponse> {
    let stale_threshold_ms = state.execution_policy.read().stale_book_ms;
    let (stale, heartbeat_age_ms, heartbeat_seq) =
        compute_feed_stale(&state.metrics, stale_threshold_ms);

    let (reject_events, cancel_events, _, _) = recent_reject_cancel_stats(&state);

    Json(FeedHealthResponse {
        ws_healthy: !stale,
        heartbeat_seq,
        heartbeat_age_ms,
        l2_updates: state.metrics.get_counter("coinbase_ws_l2_updates"),
        user_updates: state.metrics.get_counter("coinbase_ws_user_updates"),
        reconnects: state.metrics.get_counter("coinbase_ws_reconnects"),
        sequence_gaps: state.metrics.get_counter("coinbase_ws_sequence_gap"),
        errors: state.metrics.get_counter("coinbase_ws_errors"),
        timeout_streak: state.metrics.get_gauge("coinbase_ws_timeout_streak"),
        read_timeouts: state.metrics.get_counter("coinbase_ws_read_timeouts"),
        ping_failures: state.metrics.get_counter("coinbase_ws_ping_failures"),
        heartbeat_timeouts: state.metrics.get_counter("coinbase_ws_heartbeat_timeouts"),
        remote_closes: state.metrics.get_counter("coinbase_ws_remote_closes"),
        read_errors: state.metrics.get_counter("coinbase_ws_read_errors"),
        connect_failures: state.metrics.get_counter("coinbase_ws_connect_failures"),
        reject_events_10m: reject_events,
        cancel_events_10m: cancel_events,
    })
}

async fn get_feed_diagnostics(
    State(state): State<DashboardState>,
) -> Json<FeedDiagnosticsResponse> {
    let stale_threshold_ms = state.execution_policy.read().stale_book_ms;
    let (stale, heartbeat_age_ms, heartbeat_seq) =
        compute_feed_stale(&state.metrics, stale_threshold_ms);
    let (reject_events, cancel_events, reject_rate, cancel_rate) =
        recent_reject_cancel_stats(&state);

    Json(FeedDiagnosticsResponse {
        ws_healthy: !stale,
        stale_threshold_ms,
        heartbeat_age_ms,
        heartbeat_seq,
        l2_updates: state.metrics.get_counter("coinbase_ws_l2_updates"),
        user_updates: state.metrics.get_counter("coinbase_ws_user_updates"),
        reconnects: state.metrics.get_counter("coinbase_ws_reconnects"),
        sequence_gaps: state.metrics.get_counter("coinbase_ws_sequence_gap"),
        errors: state.metrics.get_counter("coinbase_ws_errors"),
        timeout_streak: state.metrics.get_gauge("coinbase_ws_timeout_streak"),
        read_timeouts: state.metrics.get_counter("coinbase_ws_read_timeouts"),
        ping_failures: state.metrics.get_counter("coinbase_ws_ping_failures"),
        heartbeat_timeouts: state.metrics.get_counter("coinbase_ws_heartbeat_timeouts"),
        remote_closes: state.metrics.get_counter("coinbase_ws_remote_closes"),
        read_errors: state.metrics.get_counter("coinbase_ws_read_errors"),
        connect_failures: state.metrics.get_counter("coinbase_ws_connect_failures"),
        reject_events_10m: reject_events,
        cancel_events_10m: cancel_events,
        reject_rate_10m: reject_rate,
        cancel_rate_10m: cancel_rate,
    })
}

fn build_parity_monitor(state: &DashboardState) -> ParityMonitorResponse {
    let opportunities = state.route_opportunities.read().clone();
    let policy = state.execution_policy.read().clone();
    let (feed_stale, _, _) = compute_feed_stale(&state.metrics, policy.stale_book_ms);

    let mut rows: Vec<ParityMonitorRow> = opportunities
        .into_iter()
        .map(|opp| {
            let min_required_bps = match opp.strategy_class {
                pt_core::StrategyClass::MakerMmSpot => policy.edge_profiles.maker_mm_spot_min_bps,
                pt_core::StrategyClass::ConversionCycle => {
                    policy.edge_profiles.conversion_cycle_min_bps
                }
                pt_core::StrategyClass::PositionReentry => {
                    policy.edge_profiles.position_reentry_min_bps
                }
            };
            let mut reasons = Vec::new();
            if opp.expected_net_bps < min_required_bps {
                reasons.push("expected_net_below_strategy_threshold".to_string());
            }
            if feed_stale {
                reasons.push("feed_stale".to_string());
            }
            let pass = reasons.is_empty();
            let path = if opp.legs.is_empty() {
                "-".to_string()
            } else {
                opp.legs
                    .iter()
                    .map(|l| format!("{}:{}->{}", l.product_id, l.input_asset, l.output_asset))
                    .collect::<Vec<_>>()
                    .join(" | ")
            };
            ParityMonitorRow {
                route_id: opp.route_id,
                path,
                strategy_class: format!("{:?}", opp.strategy_class),
                gross_edge_bps: opp.gross_edge_bps,
                expected_net_bps: opp.expected_net_bps,
                min_required_bps,
                pass,
                reasons,
                expected_usd_profit: opp.expected_usd_profit,
                capital_required_usd: opp.capital_required_usd,
                ts: opp.ts.to_rfc3339(),
            }
        })
        .collect();
    rows.sort_by(|a, b| b.expected_net_bps.total_cmp(&a.expected_net_bps));
    ParityMonitorResponse { feed_stale, rows }
}

async fn get_parity_monitor(State(state): State<DashboardState>) -> Json<ParityMonitorResponse> {
    Json(build_parity_monitor(&state))
}

fn csv_cell(input: &str) -> String {
    let escaped = input.replace('\"', "\"\"");
    if escaped.contains(',') || escaped.contains('\n') || escaped.contains('\"') {
        format!("\"{escaped}\"")
    } else {
        escaped
    }
}

fn parity_rows_to_csv(rows: &[ParityMonitorRow]) -> String {
    let mut out = String::from(
        "route_id,strategy_class,gross_edge_bps,expected_net_bps,cost_bps,min_required_bps,expected_usd_profit,capital_required_usd,pass,reasons,ts,path\n",
    );
    for row in rows {
        let cost_bps = row.gross_edge_bps - row.expected_net_bps;
        let gate = if row.pass { "PASS" } else { "FAIL" };
        let reasons = row.reasons.join("|");
        let line = format!(
            "{},{},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{},{},{},{}\n",
            csv_cell(&row.route_id),
            csv_cell(&row.strategy_class),
            row.gross_edge_bps,
            row.expected_net_bps,
            cost_bps,
            row.min_required_bps,
            row.expected_usd_profit,
            row.capital_required_usd,
            gate,
            csv_cell(&reasons),
            csv_cell(&row.ts),
            csv_cell(&row.path),
        );
        out.push_str(&line);
    }
    out
}

async fn post_parity_export_csv(
    State(state): State<DashboardState>,
    AxumJson(req): AxumJson<ParityCsvExportRequest>,
) -> Json<ParityCsvExportResponse> {
    let mut rows = build_parity_monitor(&state).rows;
    if !req.include_failures.unwrap_or(true) {
        rows.retain(|r| r.pass);
    }
    let limit = req.limit.unwrap_or(200).clamp(1, 5_000);
    if rows.len() > limit {
        rows.truncate(limit);
    }
    if rows.is_empty() {
        return Json(ParityCsvExportResponse {
            ok: false,
            row_count: 0,
            file_path: None,
            message: "no parity rows to export".to_string(),
        });
    }

    let csv = parity_rows_to_csv(&rows);
    let output_dir = std::env::var("PT_OUTPUT_DIR").unwrap_or_else(|_| "data/output".to_string());
    if let Err(e) = std::fs::create_dir_all(&output_dir) {
        return Json(ParityCsvExportResponse {
            ok: false,
            row_count: rows.len(),
            file_path: None,
            message: format!("create output dir failed: {e}"),
        });
    }
    let filename = format!("parity_monitor_{}.csv", Utc::now().format("%Y%m%d_%H%M%S"));
    let path = format!("{}/{}", output_dir.trim_end_matches('/'), filename);
    if let Err(e) = std::fs::write(&path, csv) {
        return Json(ParityCsvExportResponse {
            ok: false,
            row_count: rows.len(),
            file_path: None,
            message: format!("write parity csv failed: {e}"),
        });
    }

    Json(ParityCsvExportResponse {
        ok: true,
        row_count: rows.len(),
        file_path: Some(path),
        message: "parity csv exported".to_string(),
    })
}

async fn get_executions(State(state): State<DashboardState>) -> Json<Vec<ExecutionReport>> {
    Json(state.recent_executions.read().clone())
}

async fn get_execution_orders(State(state): State<DashboardState>) -> Json<Vec<ExecutionEvent>> {
    Json(state.execution_events.read().clone())
}

async fn get_execution_costs(
    State(state): State<DashboardState>,
) -> Json<Vec<ExecutionCostAttribution>> {
    Json(state.execution_costs.read().clone())
}

async fn get_execution_vectors(State(state): State<DashboardState>) -> Json<ExecutionVectorsView> {
    let p = state.execution_policy.read().clone();
    Json(ExecutionVectorsView {
        entry_max_slippage_bps: p.vectors.entry_max_slippage_bps,
        exit_max_slippage_bps: p.vectors.exit_max_slippage_bps,
        entry_offset_bps: p.vectors.entry_offset_bps,
        exit_offset_bps: p.vectors.exit_offset_bps,
        max_cross_bps_unwind: p.vectors.max_cross_bps_unwind,
    })
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

async fn get_coinbase_wallet(State(state): State<DashboardState>) -> Json<Vec<WalletBalance>> {
    Json(state.wallet_balances.read().clone())
}

async fn get_coinbase_allocations(
    State(state): State<DashboardState>,
) -> Json<Vec<AllocationDrift>> {
    Json(state.wallet_drifts.read().clone())
}

async fn get_coinbase_rebalance_plan(
    State(state): State<DashboardState>,
) -> Json<Option<RebalancePlanView>> {
    let plan = state.rebalance_plan.read().clone();
    let approval = state.rebalance_approval.read().clone();
    let out = plan.map(|p| RebalancePlanView {
        plan_id: p.plan_id,
        status: format!("{:?}", p.status),
        intent_count: p.intents.len(),
        total_drift_abs_usd: p.total_drift_abs_usd,
        created_ts: p.created_ts.to_rfc3339(),
        expires_ts: p.expires_ts.to_rfc3339(),
        approval_token_id: approval.as_ref().map(|a| a.token_id.clone()),
        approval_required: approval.is_some(),
        approval_expires_ts: approval.map(|a| a.expires_ts.to_rfc3339()),
    });
    Json(out)
}

async fn get_coinbase_orders(
    State(state): State<DashboardState>,
) -> Json<Vec<pt_coinbase::CoinbaseOrderSummary>> {
    Json(state.wallet_open_orders.read().clone())
}

async fn get_coinbase_orderbook(
    State(state): State<DashboardState>,
) -> Json<Vec<CoinbaseOrderBookState>> {
    let mut rows: Vec<CoinbaseOrderBookState> =
        state.coinbase_orderbooks.read().values().cloned().collect();
    rows.sort_by(|a, b| a.product_id.cmp(&b.product_id));
    Json(rows)
}

async fn get_coinbase_auth(
    State(state): State<DashboardState>,
) -> Json<pt_coinbase::CoinbaseAuthStatus> {
    if let Some(ctrl) = &state.coinbase_auth_controller {
        Json(ctrl.status())
    } else {
        Json(pt_coinbase::CoinbaseAuthStatus {
            ok: false,
            profile_id: None,
            key_id_suffix: None,
            source: None,
            loaded_at: None,
            reason: "coinbase auth controller unavailable".to_string(),
            allow_hot_reload: false,
        })
    }
}

async fn get_route_opportunities(
    State(state): State<DashboardState>,
) -> Json<Vec<RouteOpportunity>> {
    Json(state.route_opportunities.read().clone())
}

async fn get_route_executions(
    State(state): State<DashboardState>,
) -> Json<Vec<RouteExecutionPlan>> {
    Json(state.route_executions.read().clone())
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let rank = ((sorted.len() - 1) as f64 * p).round() as usize;
    sorted[rank.min(sorted.len() - 1)]
}

fn build_venue_latency_stats(state: &DashboardState) -> Vec<VenueLatencyStats> {
    let events = state.execution_events.read().clone();
    let mut starts: HashMap<(String, String), DateTime<Utc>> = HashMap::new();
    let mut ends: HashMap<(String, String), DateTime<Utc>> = HashMap::new();
    for ev in &events {
        let key = (ev.venue.as_str().to_string(), ev.order_id.clone());
        starts
            .entry(key.clone())
            .and_modify(|v| {
                if ev.ts < *v {
                    *v = ev.ts;
                }
            })
            .or_insert(ev.ts);
        if matches!(
            ev.state,
            pt_core::OrderLifecycleState::PartiallyFilled
                | pt_core::OrderLifecycleState::Filled
                | pt_core::OrderLifecycleState::Canceled
                | pt_core::OrderLifecycleState::Rejected
        ) {
            ends.entry(key)
                .and_modify(|v| {
                    if ev.ts > *v {
                        *v = ev.ts;
                    }
                })
                .or_insert(ev.ts);
        }
    }

    let mut by_venue: HashMap<String, Vec<f64>> = HashMap::new();
    for (key, start_ts) in starts {
        if let Some(end_ts) = ends.get(&key) {
            let ms = (*end_ts - start_ts).num_milliseconds().max(0) as f64;
            by_venue.entry(key.0).or_default().push(ms);
        }
    }

    let now = Utc::now();
    let mut rows: Vec<VenueLatencyStats> = Vec::new();
    for (venue_name, mut samples) in by_venue {
        samples.sort_by(|a, b| a.total_cmp(b));
        let venue = match venue_name.as_str() {
            "coinbase" => pt_core::Venue::Coinbase,
            "polymarket" => pt_core::Venue::Polymarket,
            "kraken" => pt_core::Venue::Kraken,
            "gemini" => pt_core::Venue::Gemini,
            _ => pt_core::Venue::Sim,
        };
        rows.push(VenueLatencyStats {
            venue,
            p50_ms: percentile(&samples, 0.50),
            p95_ms: percentile(&samples, 0.95),
            p99_ms: percentile(&samples, 0.99),
            samples: samples.len(),
            ts: now,
        });
    }
    rows.sort_by(|a, b| a.venue.as_str().cmp(b.venue.as_str()));
    rows
}

fn build_venue_fill_quality(state: &DashboardState) -> Vec<VenueFillQualityStats> {
    let mut agg: HashMap<String, VenueFillQualityStats> = HashMap::new();
    let now = Utc::now();
    for ev in state.execution_events.read().iter() {
        let k = ev.venue.as_str().to_string();
        let row = agg.entry(k).or_insert_with(|| VenueFillQualityStats {
            venue: ev.venue.clone(),
            fills: 0,
            rejects: 0,
            cancels: 0,
            reject_ratio: 0.0,
            avg_slippage_bps: 0.0,
            avg_effective_edge: 0.0,
            ts: now,
        });
        match ev.state {
            pt_core::OrderLifecycleState::Filled
            | pt_core::OrderLifecycleState::PartiallyFilled => {
                row.fills += 1;
            }
            pt_core::OrderLifecycleState::Rejected => row.rejects += 1,
            pt_core::OrderLifecycleState::Canceled => row.cancels += 1,
            _ => {}
        }
    }

    let mut slip_sum: HashMap<String, (f64, f64, usize)> = HashMap::new();
    for cost in state.execution_costs.read().iter() {
        let key = cost.venue.as_str().to_string();
        let ent = slip_sum.entry(key).or_insert((0.0, 0.0, 0));
        ent.0 += cost.slippage_bps;
        ent.1 += cost.effective_edge;
        ent.2 += 1;
    }

    for (k, row) in &mut agg {
        let denom = (row.fills + row.rejects + row.cancels).max(1) as f64;
        row.reject_ratio = row.rejects as f64 / denom;
        if let Some((slip_total, edge_total, n)) = slip_sum.get(k) {
            let d = (*n).max(1) as f64;
            row.avg_slippage_bps = *slip_total / d;
            row.avg_effective_edge = *edge_total / d;
        }
    }

    let mut rows: Vec<VenueFillQualityStats> = agg.into_values().collect();
    rows.sort_by(|a, b| a.venue.as_str().cmp(b.venue.as_str()));
    rows
}

fn build_wallet_intel_coinbase(state: &DashboardState) -> Vec<WalletIntelSnapshot> {
    let mut rows = Vec::new();
    for bal in state.wallet_balances.read().iter() {
        if bal.venue.as_str() != "coinbase" {
            continue;
        }
        let subject = format!("{}:{}", bal.account_id, bal.asset.to_ascii_uppercase());
        rows.push(WalletIntelSnapshot {
            source: "coinbase".to_string(),
            subject: subject.clone(),
            metric: "available".to_string(),
            value: bal.available,
            ts: bal.ts,
        });
        rows.push(WalletIntelSnapshot {
            source: "coinbase".to_string(),
            subject: subject.clone(),
            metric: "hold".to_string(),
            value: bal.hold,
            ts: bal.ts,
        });
        rows.push(WalletIntelSnapshot {
            source: "coinbase".to_string(),
            subject,
            metric: "usd_value".to_string(),
            value: bal.usd_value,
            ts: bal.ts,
        });
    }
    for drift in state.wallet_drifts.read().iter() {
        rows.push(WalletIntelSnapshot {
            source: "coinbase".to_string(),
            subject: drift.asset.to_ascii_uppercase(),
            metric: "drift_usd".to_string(),
            value: drift.drift_usd,
            ts: Utc::now(),
        });
    }
    rows
}

fn build_wallet_intel_polymarket(state: &DashboardState) -> Vec<WalletIntelSnapshot> {
    let now = Utc::now();
    let mut by_asset_count: HashMap<String, usize> = HashMap::new();
    let mut by_asset_spread: HashMap<String, (f64, usize)> = HashMap::new();

    for m in state.selected_markets.read().iter() {
        let asset = m.asset.as_str().to_string();
        *by_asset_count.entry(asset.clone()).or_insert(0) += 1;
        let ent = by_asset_spread.entry(asset).or_insert((0.0, 0));
        ent.0 += m.spread;
        ent.1 += 1;
    }

    let mut rows = Vec::new();
    rows.push(WalletIntelSnapshot {
        source: "polymarket".to_string(),
        subject: "market_universe".to_string(),
        metric: "active_markets".to_string(),
        value: state.selected_markets.read().len() as f64,
        ts: now,
    });
    for (asset, count) in by_asset_count {
        rows.push(WalletIntelSnapshot {
            source: "polymarket".to_string(),
            subject: asset.clone(),
            metric: "active_markets".to_string(),
            value: count as f64,
            ts: now,
        });
        if let Some((spread_total, n)) = by_asset_spread.get(&asset) {
            rows.push(WalletIntelSnapshot {
                source: "polymarket".to_string(),
                subject: asset,
                metric: "avg_spread".to_string(),
                value: if *n > 0 {
                    *spread_total / *n as f64
                } else {
                    0.0
                },
                ts: now,
            });
        }
    }
    for (asset, bias) in state.fused_bias.read().iter() {
        rows.push(WalletIntelSnapshot {
            source: "polymarket".to_string(),
            subject: asset.as_str().to_string(),
            metric: "wallet_bias".to_string(),
            value: *bias,
            ts: now,
        });
    }
    rows
}

fn build_wallet_intel_leaderboard(state: &DashboardState) -> Vec<WalletIntelSnapshot> {
    let mut balances = state.wallet_balances.read().clone();
    balances.sort_by(|a, b| b.usd_value.total_cmp(&a.usd_value));

    let mut rows = Vec::new();
    for (rank, bal) in balances.into_iter().take(20).enumerate() {
        rows.push(WalletIntelSnapshot {
            source: "leaderboard".to_string(),
            subject: format!("#{}:{}:{}", rank + 1, bal.account_id, bal.asset),
            metric: "usd_value".to_string(),
            value: bal.usd_value,
            ts: bal.ts,
        });
    }

    for (rank, opp) in state
        .route_opportunities
        .read()
        .iter()
        .cloned()
        .take(20)
        .enumerate()
    {
        rows.push(WalletIntelSnapshot {
            source: "leaderboard".to_string(),
            subject: format!("#{}:route:{}", rank + 1, opp.route_id),
            metric: "expected_usd_profit".to_string(),
            value: opp.expected_usd_profit,
            ts: opp.ts,
        });
    }
    rows
}

async fn get_venue_latency(State(state): State<DashboardState>) -> Json<Vec<VenueLatencyStats>> {
    Json(build_venue_latency_stats(&state))
}

async fn get_venue_capabilities(State(state): State<DashboardState>) -> Json<Vec<VenueCapability>> {
    Json(state.venue_capabilities.read().clone())
}

async fn get_venue_fill_quality(
    State(state): State<DashboardState>,
) -> Json<Vec<VenueFillQualityStats>> {
    Json(build_venue_fill_quality(&state))
}

async fn get_wallet_intel_coinbase(
    State(state): State<DashboardState>,
) -> Json<Vec<WalletIntelSnapshot>> {
    Json(build_wallet_intel_coinbase(&state))
}

async fn get_wallet_intel_polymarket(
    State(state): State<DashboardState>,
) -> Json<Vec<WalletIntelSnapshot>> {
    Json(build_wallet_intel_polymarket(&state))
}

async fn get_wallet_intel_leaderboard(
    State(state): State<DashboardState>,
) -> Json<Vec<WalletIntelSnapshot>> {
    Json(build_wallet_intel_leaderboard(&state))
}

fn route_rows_to_csv(rows: &[RouteOpportunity]) -> String {
    let mut out = String::from(
        "route_id,strategy_class,gross_edge_bps,expected_net_bps,expected_usd_profit,capital_required_usd,legs,ts\n",
    );
    for row in rows {
        let legs = row
            .legs
            .iter()
            .map(|l| format!("{}:{}->{}", l.product_id, l.input_asset, l.output_asset))
            .collect::<Vec<_>>()
            .join(" | ");
        let line = format!(
            "{},{},{:.6},{:.6},{:.6},{:.6},{},{}\n",
            csv_cell(&row.route_id),
            csv_cell(&format!("{:?}", row.strategy_class)),
            row.gross_edge_bps,
            row.expected_net_bps,
            row.expected_usd_profit,
            row.capital_required_usd,
            csv_cell(&legs),
            csv_cell(&row.ts.to_rfc3339()),
        );
        out.push_str(&line);
    }
    out
}

async fn post_routes_export_csv(
    State(state): State<DashboardState>,
    AxumJson(req): AxumJson<RouteCsvExportRequest>,
) -> Json<RouteCsvExportResponse> {
    let limit = req.limit.unwrap_or(200).clamp(1, 5_000);
    let min_net = req.min_expected_net_bps.unwrap_or(f64::NEG_INFINITY);
    let mut rows: Vec<RouteOpportunity> = state
        .route_opportunities
        .read()
        .iter()
        .filter(|r| r.expected_net_bps >= min_net)
        .cloned()
        .collect();
    rows.sort_by(|a, b| b.expected_net_bps.total_cmp(&a.expected_net_bps));
    if rows.len() > limit {
        rows.truncate(limit);
    }
    if rows.is_empty() {
        return Json(RouteCsvExportResponse {
            ok: false,
            row_count: 0,
            file_path: None,
            message: "no route rows to export".to_string(),
        });
    }

    let csv = route_rows_to_csv(&rows);
    let output_dir = std::env::var("PT_OUTPUT_DIR").unwrap_or_else(|_| "data/output".to_string());
    if let Err(e) = std::fs::create_dir_all(&output_dir) {
        return Json(RouteCsvExportResponse {
            ok: false,
            row_count: rows.len(),
            file_path: None,
            message: format!("create output dir failed: {e}"),
        });
    }
    let filename = format!("routes_{}.csv", Utc::now().format("%Y%m%d_%H%M%S"));
    let path = format!("{}/{}", output_dir.trim_end_matches('/'), filename);
    if let Err(e) = std::fs::write(&path, csv) {
        return Json(RouteCsvExportResponse {
            ok: false,
            row_count: rows.len(),
            file_path: None,
            message: format!("write routes csv failed: {e}"),
        });
    }

    Json(RouteCsvExportResponse {
        ok: true,
        row_count: rows.len(),
        file_path: Some(path),
        message: "routes csv exported".to_string(),
    })
}

fn wallet_intel_rows_to_csv(rows: &[WalletIntelSnapshot]) -> String {
    let mut out = String::from("source,subject,metric,value,ts\n");
    for row in rows {
        let line = format!(
            "{},{},{},{:.10},{}\n",
            csv_cell(&row.source),
            csv_cell(&row.subject),
            csv_cell(&row.metric),
            row.value,
            csv_cell(&row.ts.to_rfc3339())
        );
        out.push_str(&line);
    }
    out
}

async fn post_wallet_intel_export_csv(
    State(state): State<DashboardState>,
    AxumJson(req): AxumJson<WalletIntelExportRequest>,
) -> Json<WalletIntelExportResponse> {
    let source = req
        .source
        .unwrap_or_else(|| "all".to_string())
        .trim()
        .to_ascii_lowercase();
    let mut rows = match source.as_str() {
        "coinbase" => build_wallet_intel_coinbase(&state),
        "polymarket" => build_wallet_intel_polymarket(&state),
        "leaderboard" => build_wallet_intel_leaderboard(&state),
        _ => {
            let mut all = build_wallet_intel_coinbase(&state);
            all.extend(build_wallet_intel_polymarket(&state));
            all.extend(build_wallet_intel_leaderboard(&state));
            all
        }
    };
    rows.sort_by(|a, b| b.ts.cmp(&a.ts).then_with(|| a.subject.cmp(&b.subject)));
    let limit = req.limit.unwrap_or(5_000).clamp(1, 50_000);
    if rows.len() > limit {
        rows.truncate(limit);
    }
    if rows.is_empty() {
        return Json(WalletIntelExportResponse {
            ok: false,
            row_count: 0,
            file_path: None,
            message: "no wallet intel rows to export".to_string(),
        });
    }

    let csv = wallet_intel_rows_to_csv(&rows);
    let output_dir = std::env::var("PT_OUTPUT_DIR").unwrap_or_else(|_| "data/output".to_string());
    if let Err(e) = std::fs::create_dir_all(&output_dir) {
        return Json(WalletIntelExportResponse {
            ok: false,
            row_count: rows.len(),
            file_path: None,
            message: format!("create output dir failed: {e}"),
        });
    }
    let filename = format!("wallet_intel_{}.csv", Utc::now().format("%Y%m%d_%H%M%S"));
    let path = format!("{}/{}", output_dir.trim_end_matches('/'), filename);
    if let Err(e) = std::fs::write(&path, csv) {
        return Json(WalletIntelExportResponse {
            ok: false,
            row_count: rows.len(),
            file_path: None,
            message: format!("write wallet intel csv failed: {e}"),
        });
    }

    Json(WalletIntelExportResponse {
        ok: true,
        row_count: rows.len(),
        file_path: Some(path),
        message: "wallet intel csv exported".to_string(),
    })
}

async fn get_fees_summary(
    State(state): State<DashboardState>,
) -> Json<Option<pt_coinbase::CoinbaseTransactionSummary>> {
    Json(state.coinbase_fee_summary.read().clone())
}

async fn post_halt(State(state): State<DashboardState>) -> Json<Health> {
    *state.kill_switch.write() = KillSwitchState::ManualHalt;
    *state.force_unwind.write() = false;
    Json(Health {
        status: "ok",
        kill_switch: "ManualHalt".to_string(),
    })
}

async fn post_resume(State(state): State<DashboardState>) -> Json<Health> {
    *state.kill_switch.write() = KillSwitchState::Running;
    *state.force_unwind.write() = false;
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

async fn post_profile_pilot_ultra_tight(State(state): State<DashboardState>) -> Json<Health> {
    let mut policy = state.execution_policy.write();
    policy.allow_taker_on_unwind_only = true;
    policy.post_only = true;
    policy.min_rest_ms = policy.min_rest_ms.max(400);
    policy.cancel_replace_cooldown_ms = policy.cancel_replace_cooldown_ms.max(250);
    policy.vectors.entry_max_slippage_bps = policy.vectors.entry_max_slippage_bps.min(8.0);
    policy.vectors.exit_max_slippage_bps = policy.vectors.exit_max_slippage_bps.min(10.0);
    policy.vectors.max_cross_bps_unwind = policy.vectors.max_cross_bps_unwind.min(20.0);
    Json(Health {
        status: "pilot_ultra_tight_applied",
        kill_switch: format!("{:?}", *state.kill_switch.read()),
    })
}

async fn post_coinbase_rebalance_approve(
    State(state): State<DashboardState>,
    AxumJson(req): AxumJson<RebalanceApprovalRequest>,
) -> Json<RebalanceApprovalResponse> {
    let plan = state.rebalance_plan.read().clone();
    let mut approval = state.rebalance_approval.write();
    if let (Some(plan), Some(current)) = (plan, approval.as_mut()) {
        if current.token_id == req.token_id
            && current.plan_id == plan.plan_id
            && current.expires_ts > Utc::now()
        {
            current.approved = true;
            return Json(RebalanceApprovalResponse {
                ok: true,
                plan_id: Some(current.plan_id.clone()),
                token_id: Some(current.token_id.clone()),
                status: "approved".to_string(),
            });
        }
    }

    Json(RebalanceApprovalResponse {
        ok: false,
        plan_id: None,
        token_id: None,
        status: "invalid_or_expired_token".to_string(),
    })
}

async fn post_coinbase_rebalance_reject(
    State(state): State<DashboardState>,
) -> Json<RebalanceApprovalResponse> {
    if let Some(plan) = state.rebalance_plan.write().as_mut() {
        plan.status = RebalancePlanStatus::Rejected;
    }
    *state.rebalance_approval.write() = None;
    Json(RebalanceApprovalResponse {
        ok: true,
        plan_id: None,
        token_id: None,
        status: "rejected".to_string(),
    })
}

async fn post_coinbase_auth_reload(State(state): State<DashboardState>) -> Json<AuthReloadResult> {
    if let Some(ctrl) = &state.coinbase_auth_controller {
        return Json(ctrl.reload());
    }
    Json(AuthReloadResult {
        ok: false,
        profile_id: None,
        key_id_suffix: None,
        source: None,
        reason: "coinbase auth controller unavailable".to_string(),
        ts: Utc::now(),
    })
}

async fn post_coinbase_auth_switch_profile(
    State(state): State<DashboardState>,
    AxumJson(req): AxumJson<CoinbaseAuthSwitchRequest>,
) -> Json<AuthReloadResult> {
    if req.profile_id.trim().is_empty() {
        return Json(AuthReloadResult {
            ok: false,
            profile_id: None,
            key_id_suffix: None,
            source: None,
            reason: "profile_id must not be empty".to_string(),
            ts: Utc::now(),
        });
    }
    if let Some(ctrl) = &state.coinbase_auth_controller {
        return Json(ctrl.switch_profile(req.profile_id.trim()));
    }
    Json(AuthReloadResult {
        ok: false,
        profile_id: None,
        key_id_suffix: None,
        source: None,
        reason: "coinbase auth controller unavailable".to_string(),
        ts: Utc::now(),
    })
}

async fn post_execution_unwind(State(state): State<DashboardState>) -> Json<Health> {
    *state.force_unwind.write() = true;
    *state.kill_switch.write() = KillSwitchState::SafeMode;
    Json(Health {
        status: "ok",
        kill_switch: "SafeMode".to_string(),
    })
}

fn bucket_label(selection: &MarketSelection) -> String {
    match selection.bucket {
        pt_core::TimeBucket::FiveMinute => "5m".to_string(),
        pt_core::TimeBucket::FifteenMinute => "15m".to_string(),
        pt_core::TimeBucket::Other => "other".to_string(),
    }
}

fn normalize_asset_symbol(asset: &str) -> String {
    let upper = asset.trim().to_ascii_uppercase();
    if upper == "USDC" {
        "USD".to_string()
    } else {
        upper
    }
}

fn conversion_product_and_side(
    from_asset: &str,
    to_asset: &str,
    products: &[String],
) -> Result<(String, Side), String> {
    if from_asset == to_asset {
        return Err("from_asset and to_asset are the same".to_string());
    }
    if from_asset == "USD" {
        let product = format!("{to_asset}-USD");
        return Ok((product, Side::Buy));
    }
    if to_asset == "USD" {
        let product = format!("{from_asset}-USD");
        return Ok((product, Side::Sell));
    }

    let direct = format!("{from_asset}-{to_asset}");
    if products.iter().any(|p| p.eq_ignore_ascii_case(&direct)) {
        return Ok((direct, Side::Sell));
    }
    let inverse = format!("{to_asset}-{from_asset}");
    if products.iter().any(|p| p.eq_ignore_ascii_case(&inverse)) {
        return Ok((inverse, Side::Buy));
    }

    Err("no direct conversion pair found (use USD bridge conversion in two steps)".to_string())
}

async fn post_coinbase_convert_preview(
    State(state): State<DashboardState>,
    AxumJson(req): AxumJson<CoinbaseConvertRequest>,
) -> Json<CoinbaseConvertResponse> {
    post_coinbase_convert_inner(state, req, false).await
}

async fn post_coinbase_convert_execute(
    State(state): State<DashboardState>,
    AxumJson(req): AxumJson<CoinbaseConvertRequest>,
) -> Json<CoinbaseConvertResponse> {
    post_coinbase_convert_inner(state, req, true).await
}

async fn post_coinbase_convert_inner(
    state: DashboardState,
    req: CoinbaseConvertRequest,
    execute: bool,
) -> Json<CoinbaseConvertResponse> {
    const LIVE_CONFIRM: &str = "I_UNDERSTAND_LIVE_CONVERT";
    let from_asset = normalize_asset_symbol(&req.from_asset);
    let to_asset = normalize_asset_symbol(&req.to_asset);
    let live = req.live.unwrap_or(false) && matches!(state.engine_mode, EngineMode::Live);

    let Some(wallet_client) = state.coinbase_wallet_client.as_ref() else {
        return Json(CoinbaseConvertResponse {
            ok: false,
            mode: if live { "live" } else { "paper" }.to_string(),
            from_asset,
            to_asset,
            account_id: req.account_id,
            product_id: None,
            side: None,
            amount_from: 0.0,
            amount_base: 0.0,
            limit_price: 0.0,
            est_quote: 0.0,
            order_id: None,
            preview_ok: false,
            message: "coinbase wallet client unavailable".to_string(),
        });
    };

    let balances = state.wallet_balances.read().clone();
    let available_total: f64 = balances
        .iter()
        .filter(|b| {
            b.asset.eq_ignore_ascii_case(&from_asset)
                && req
                    .account_id
                    .as_ref()
                    .map(|id| b.account_id.eq_ignore_ascii_case(id))
                    .unwrap_or(true)
        })
        .map(|b| b.available.max(0.0))
        .sum();
    let amount_from = req
        .amount
        .unwrap_or(available_total)
        .max(0.0)
        .min(available_total);

    if amount_from <= 0.0 {
        return Json(CoinbaseConvertResponse {
            ok: false,
            mode: if live { "live" } else { "paper" }.to_string(),
            from_asset,
            to_asset,
            account_id: req.account_id,
            product_id: None,
            side: None,
            amount_from,
            amount_base: 0.0,
            limit_price: 0.0,
            est_quote: 0.0,
            order_id: None,
            preview_ok: false,
            message: "insufficient available balance for selected source asset/account".to_string(),
        });
    }

    let (product_id, side) =
        match conversion_product_and_side(&from_asset, &to_asset, &state.coinbase_products) {
            Ok(v) => v,
            Err(e) => {
                return Json(CoinbaseConvertResponse {
                    ok: false,
                    mode: if live { "live" } else { "paper" }.to_string(),
                    from_asset,
                    to_asset,
                    account_id: req.account_id,
                    product_id: None,
                    side: None,
                    amount_from,
                    amount_base: 0.0,
                    limit_price: 0.0,
                    est_quote: 0.0,
                    order_id: None,
                    preview_ok: false,
                    message: e,
                })
            }
        };

    let top = match wallet_client.fetch_top_of_book(&product_id).await {
        Ok(v) => v,
        Err(e) => {
            return Json(CoinbaseConvertResponse {
                ok: false,
                mode: if live { "live" } else { "paper" }.to_string(),
                from_asset,
                to_asset,
                account_id: req.account_id,
                product_id: Some(product_id),
                side: Some(format!("{:?}", side)),
                amount_from,
                amount_base: 0.0,
                limit_price: 0.0,
                est_quote: 0.0,
                order_id: None,
                preview_ok: false,
                message: format!("top-of-book fetch failed: {e}"),
            })
        }
    };

    let offsets = state.execution_policy.read().vectors.clone();
    let limit_price = match side {
        Side::Buy => top.best_bid * (1.0 - offsets.entry_offset_bps / 10_000.0),
        Side::Sell => top.best_ask * (1.0 + offsets.exit_offset_bps / 10_000.0),
    }
    .max(0.00000001);
    let amount_base = match side {
        Side::Sell => amount_from,
        Side::Buy => amount_from / limit_price,
    }
    .max(0.00000001);
    let est_quote = amount_base * limit_price;

    let preview = wallet_client
        .preview_order_post_only(&product_id, side.clone(), amount_base, limit_price)
        .await;
    let preview_ok = matches!(preview, Ok(ref p) if p.success);
    if !preview_ok {
        let reason = match preview {
            Ok(p) => p
                .failure_reason
                .unwrap_or_else(|| "preview rejected".to_string()),
            Err(e) => e.to_string(),
        };
        return Json(CoinbaseConvertResponse {
            ok: false,
            mode: if live { "live" } else { "paper" }.to_string(),
            from_asset,
            to_asset,
            account_id: req.account_id,
            product_id: Some(product_id),
            side: Some(format!("{:?}", side)),
            amount_from,
            amount_base,
            limit_price,
            est_quote,
            order_id: None,
            preview_ok: false,
            message: reason,
        });
    }

    if !execute {
        return Json(CoinbaseConvertResponse {
            ok: true,
            mode: if live { "live" } else { "paper" }.to_string(),
            from_asset,
            to_asset,
            account_id: req.account_id,
            product_id: Some(product_id),
            side: Some(format!("{:?}", side)),
            amount_from,
            amount_base,
            limit_price,
            est_quote,
            order_id: Some("preview-only".to_string()),
            preview_ok: true,
            message: "preview succeeded (no order placed)".to_string(),
        });
    }

    if live && req.confirm.as_deref().unwrap_or_default() != LIVE_CONFIRM {
        return Json(CoinbaseConvertResponse {
            ok: false,
            mode: "live".to_string(),
            from_asset,
            to_asset,
            account_id: req.account_id,
            product_id: Some(product_id),
            side: Some(format!("{:?}", side)),
            amount_from,
            amount_base,
            limit_price,
            est_quote,
            order_id: None,
            preview_ok: true,
            message: format!("live convert blocked: provide confirm='{LIVE_CONFIRM}'"),
        });
    }

    if !live {
        return Json(CoinbaseConvertResponse {
            ok: true,
            mode: "paper".to_string(),
            from_asset,
            to_asset,
            account_id: req.account_id,
            product_id: Some(product_id),
            side: Some(format!("{:?}", side)),
            amount_from,
            amount_base,
            limit_price,
            est_quote,
            order_id: Some(format!("paper-convert-{}", Utc::now().timestamp_millis())),
            preview_ok: true,
            message: "paper conversion simulated".to_string(),
        });
    }

    match wallet_client
        .create_order_post_only(&product_id, side.clone(), amount_base, limit_price)
        .await
    {
        Ok(report) => Json(CoinbaseConvertResponse {
            ok: true,
            mode: "live".to_string(),
            from_asset,
            to_asset,
            account_id: req.account_id,
            product_id: Some(product_id),
            side: Some(format!("{:?}", side)),
            amount_from,
            amount_base,
            limit_price,
            est_quote,
            order_id: Some(report.order_id),
            preview_ok: true,
            message: "live maker order posted".to_string(),
        }),
        Err(e) => Json(CoinbaseConvertResponse {
            ok: false,
            mode: "live".to_string(),
            from_asset,
            to_asset,
            account_id: req.account_id,
            product_id: Some(product_id),
            side: Some(format!("{:?}", side)),
            amount_from,
            amount_base,
            limit_price,
            est_quote,
            order_id: None,
            preview_ok: true,
            message: format!("order create failed: {e}"),
        }),
    }
}

async fn post_coinbase_maker_test(
    State(state): State<DashboardState>,
    AxumJson(req): AxumJson<CoinbaseMakerTestRequest>,
) -> Json<CoinbaseMakerTestResponse> {
    const LIVE_CONFIRM: &str = "I_UNDERSTAND_LIVE_MAKER_TEST";
    let side = if req.side.eq_ignore_ascii_case("sell") {
        Side::Sell
    } else {
        Side::Buy
    };
    let live = req.live.unwrap_or(false) && matches!(state.engine_mode, EngineMode::Live);
    let product_id = req.product_id.trim().to_ascii_uppercase();
    let Some(wallet_client) = state.coinbase_wallet_client.as_ref() else {
        return Json(CoinbaseMakerTestResponse {
            ok: false,
            mode: if live { "live" } else { "paper" }.to_string(),
            product_id,
            side: format!("{:?}", side),
            preview_ok: false,
            order_id: None,
            limit_price: 0.0,
            preview_ms: 0.0,
            post_ms: 0.0,
            cancel_ms: 0.0,
            total_ms: 0.0,
            message: "coinbase wallet client unavailable".to_string(),
        });
    };

    let start = std::time::Instant::now();
    let top = match wallet_client.fetch_top_of_book(&product_id).await {
        Ok(v) => v,
        Err(e) => {
            return Json(CoinbaseMakerTestResponse {
                ok: false,
                mode: if live { "live" } else { "paper" }.to_string(),
                product_id,
                side: format!("{:?}", side),
                preview_ok: false,
                order_id: None,
                limit_price: 0.0,
                preview_ms: 0.0,
                post_ms: 0.0,
                cancel_ms: 0.0,
                total_ms: start.elapsed().as_secs_f64() * 1000.0,
                message: format!("top-of-book failed: {e}"),
            })
        }
    };
    let offsets = state.execution_policy.read().vectors.clone();
    let limit_price = match side {
        Side::Buy => top.best_bid * (1.0 - offsets.entry_offset_bps / 10_000.0),
        Side::Sell => top.best_ask * (1.0 + offsets.exit_offset_bps / 10_000.0),
    }
    .max(0.00000001);

    let preview_start = std::time::Instant::now();
    let preview_ok = wallet_client
        .preview_order_post_only(
            &product_id,
            side.clone(),
            req.base_size.max(0.00000001),
            limit_price,
        )
        .await
        .map(|p| p.success)
        .unwrap_or(false);
    let preview_ms = preview_start.elapsed().as_secs_f64() * 1000.0;

    if !preview_ok {
        return Json(CoinbaseMakerTestResponse {
            ok: false,
            mode: if live { "live" } else { "paper" }.to_string(),
            product_id,
            side: format!("{:?}", side),
            preview_ok,
            order_id: None,
            limit_price,
            preview_ms,
            post_ms: 0.0,
            cancel_ms: 0.0,
            total_ms: start.elapsed().as_secs_f64() * 1000.0,
            message: "preview rejected".to_string(),
        });
    }

    if !live {
        return Json(CoinbaseMakerTestResponse {
            ok: true,
            mode: "paper".to_string(),
            product_id,
            side: format!("{:?}", side),
            preview_ok: true,
            order_id: Some(format!("paper-maker-{}", Utc::now().timestamp_millis())),
            limit_price,
            preview_ms,
            post_ms: 0.0,
            cancel_ms: 0.0,
            total_ms: start.elapsed().as_secs_f64() * 1000.0,
            message: "paper maker test simulated".to_string(),
        });
    }

    if req.confirm.as_deref().unwrap_or_default() != LIVE_CONFIRM {
        return Json(CoinbaseMakerTestResponse {
            ok: false,
            mode: "live".to_string(),
            product_id,
            side: format!("{:?}", side),
            preview_ok: true,
            order_id: None,
            limit_price,
            preview_ms,
            post_ms: 0.0,
            cancel_ms: 0.0,
            total_ms: start.elapsed().as_secs_f64() * 1000.0,
            message: format!("live maker test blocked: provide confirm='{LIVE_CONFIRM}'"),
        });
    }

    let post_start = std::time::Instant::now();
    let create = wallet_client
        .create_order_post_only(
            &product_id,
            side.clone(),
            req.base_size.max(0.00000001),
            limit_price,
        )
        .await;
    let post_ms = post_start.elapsed().as_secs_f64() * 1000.0;
    match create {
        Ok(report) => {
            let cancel_start = std::time::Instant::now();
            let _ = wallet_client
                .cancel_orders_batch(std::slice::from_ref(&report.order_id))
                .await;
            let cancel_ms = cancel_start.elapsed().as_secs_f64() * 1000.0;
            Json(CoinbaseMakerTestResponse {
                ok: true,
                mode: "live".to_string(),
                product_id,
                side: format!("{:?}", side),
                preview_ok: true,
                order_id: Some(report.order_id),
                limit_price,
                preview_ms,
                post_ms,
                cancel_ms,
                total_ms: start.elapsed().as_secs_f64() * 1000.0,
                message: "live maker order posted and canceled".to_string(),
            })
        }
        Err(e) => Json(CoinbaseMakerTestResponse {
            ok: false,
            mode: "live".to_string(),
            product_id,
            side: format!("{:?}", side),
            preview_ok: true,
            order_id: None,
            limit_price,
            preview_ms,
            post_ms,
            cancel_ms: 0.0,
            total_ms: start.elapsed().as_secs_f64() * 1000.0,
            message: format!("order create failed: {e}"),
        }),
    }
}

const DASHBOARD_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width,initial-scale=1" />
  <title>Polymarket Trader Dashboard</title>
  <style>
    :root {
      --bg: #0f172a;
      --panel: #111827;
      --panel2: #1f2937;
      --text: #e5e7eb;
      --muted: #94a3b8;
      --buy: #10b981;
      --sell: #ef4444;
      --warn: #f59e0b;
      --accent: #38bdf8;
    }
    * { box-sizing: border-box; }
    body {
      margin: 0;
      font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
      color: var(--text);
      background: radial-gradient(1200px 800px at 20% -10%, #1e293b, var(--bg));
    }
    .wrap { max-width: 1280px; margin: 0 auto; padding: 16px; }
    .title {
      display: flex; justify-content: space-between; align-items: center;
      gap: 12px; margin-bottom: 16px;
    }
    .title h1 { margin: 0; font-size: 20px; letter-spacing: 0.5px; color: var(--accent); }
    .status { color: var(--muted); font-size: 12px; }
    .grid {
      display: grid;
      grid-template-columns: repeat(12, 1fr);
      gap: 12px;
    }
    .card {
      background: linear-gradient(180deg, var(--panel), var(--panel2));
      border: 1px solid #243042;
      border-radius: 10px;
      padding: 12px;
      box-shadow: 0 8px 24px rgba(0,0,0,0.25);
    }
    .kpis { grid-column: span 12; display: grid; grid-template-columns: repeat(8, 1fr); gap: 8px; }
    .kpi { background: #0b1220; border: 1px solid #1e293b; border-radius: 8px; padding: 8px; }
    .kpi .label { color: var(--muted); font-size: 11px; }
    .kpi .value { font-size: 16px; margin-top: 4px; }
    .chart { grid-column: span 6; }
    .hidden { display: none; }
    .controls { grid-column: span 12; display: flex; gap: 8px; align-items: center; flex-wrap: wrap; }
    .tab-btn.active { border-color: var(--accent); color: var(--accent); }
    button {
      background: #0b1220; color: var(--text); border: 1px solid #334155;
      border-radius: 8px; padding: 8px 12px; cursor: pointer; font-size: 12px;
    }
    button:hover { border-color: var(--accent); }
    select {
      background: #0b1220;
      color: var(--text);
      border: 1px solid #334155;
      border-radius: 8px;
      padding: 8px 12px;
      font-size: 12px;
      min-width: 340px;
      max-width: 100%;
    }
    input {
      background: #0b1220;
      color: var(--text);
      border: 1px solid #334155;
      border-radius: 8px;
      padding: 8px 12px;
      font-size: 12px;
      min-width: 220px;
    }
    .table-card { grid-column: span 6; }
    table { width: 100%; border-collapse: collapse; font-size: 12px; }
    th, td { text-align: left; padding: 6px; border-bottom: 1px solid #1e293b; }
    th { color: var(--muted); position: sticky; top: 0; background: #0f172a; }
    .scroll { max-height: 360px; overflow: auto; }
    .buy { color: var(--buy); }
    .sell { color: var(--sell); }
    .warn { color: var(--warn); }
    .tiny { color: var(--muted); font-size: 11px; }
    canvas {
      width: 100%;
      height: 200px;
      background: #0b1220;
      border: 1px solid #1e293b;
      border-radius: 8px;
    }
    @media (max-width: 960px) {
      .kpis { grid-template-columns: repeat(2, 1fr); }
      .chart, .controls, .table-card { grid-column: span 12; }
      select { min-width: 100%; }
    }
  </style>
</head>
<body>
  <div class="wrap">
    <div class="title">
      <h1>Polymarket Trader</h1>
      <div class="status" id="status">Loading...</div>
    </div>
    <div class="grid">
      <div class="kpis">
        <div class="kpi"><div class="label">Kill Switch</div><div class="value" id="k_kill">-</div></div>
        <div class="kpi"><div class="label">Daily PnL</div><div class="value" id="k_pnl">-</div></div>
        <div class="kpi"><div class="label">Open Notional</div><div class="value" id="k_open">-</div></div>
        <div class="kpi"><div class="label">Unhedged Delta</div><div class="value" id="k_delta">-</div></div>
        <div class="kpi"><div class="label">Open Markets</div><div class="value" id="k_markets">-</div></div>
        <div class="kpi"><div class="label">Inventory USD</div><div class="value" id="k_inv">-</div></div>
        <div class="kpi"><div class="label">Route Opps</div><div class="value" id="k_routes">-</div></div>
        <div class="kpi"><div class="label">Fees M/T</div><div class="value" id="k_fees">-</div></div>
      </div>

      <div class="card chart">
        <div class="tiny">Daily PnL (rolling)</div>
        <canvas id="pnlChart" width="640" height="220"></canvas>
      </div>

      <div class="card chart" id="marketChartCard">
        <div class="tiny">Selected Market Mid-Price (rolling)</div>
        <canvas id="marketChart" width="640" height="220"></canvas>
        <div class="tiny" style="margin-top:8px;">Selected Market Delta Bars (granularity)</div>
        <canvas id="marketBarChart" width="640" height="140"></canvas>
        <div class="tiny" id="marketMeta">No market selected</div>
      </div>

      <div class="card controls">
        <button onclick="op('/ops/halt')">HALT</button>
        <button onclick="op('/ops/resume')">RESUME</button>
        <button onclick="op('/ops/flatten')">FLATTEN</button>
        <button onclick="op('/ops/execution/unwind')">UNWIND</button>
        <button onclick="op('/ops/profile/pilot-ultra-tight')">PILOT PROFILE</button>
        <button class="tab-btn active" id="chartTabBtn">CHART</button>
        <button class="tab-btn" id="backtestTabBtn">BACKTESTER</button>
        <button class="tab-btn" id="listingTabBtn">LISTING PATTERN</button>
        <label class="tiny" for="marketSelect">Market</label>
        <select id="marketSelect"></select>
        <label class="tiny" for="granularitySelect">Granularity</label>
        <select id="granularitySelect" style="min-width:120px;">
          <option value="1">1x</option>
          <option value="3">3x</option>
          <option value="5">5x</option>
          <option value="10">10x</option>
        </select>
        <div class="tiny" id="vectorInfo">vectors: -</div>
        <div class="tiny" id="opsResult"></div>
      </div>

      <div class="card table-card" id="selectedBookCard">
        <div class="tiny">Selected Pair Orderbook Depth</div>
        <div class="tiny" id="selectedBookMeta">No pair selected</div>
        <div class="scroll">
          <table>
            <thead><tr><th>Bid Px</th><th>Bid Sz</th><th>Ask Px</th><th>Ask Sz</th></tr></thead>
            <tbody id="selectedBookBody"></tbody>
          </table>
        </div>
      </div>

      <div class="card table-card hidden" id="backtesterCard">
        <div class="tiny">Strategy Backtester</div>
        <div class="tiny">Served from <code>http://127.0.0.1:9090</code></div>
        <iframe
          id="backtesterFrame"
          src="http://127.0.0.1:9090"
          title="Strategy Lab Backtester"
          style="width:100%; height:360px; border:1px solid #1e293b; border-radius:8px; background:#0b1220;"
        ></iframe>
      </div>

      <div class="card table-card hidden" id="listingPatternCard" style="grid-column: span 12;">
        <div class="tiny">Listing Pattern Overlay (Recently Entered Markets)</div>
        <div style="display:flex; gap:8px; margin-top:8px; flex-wrap:wrap;">
          <label class="tiny" for="listingWindow">Window</label>
          <select id="listingWindow" style="min-width:100px;">
            <option value="30d">30D</option>
            <option value="90d" selected>90D</option>
            <option value="180d">180D</option>
          </select>
          <label class="tiny" for="listingGranularity">Granularity</label>
          <select id="listingGranularity" style="min-width:100px;">
            <option value="3600">1H</option>
            <option value="14400" selected>4H</option>
            <option value="86400">1D</option>
          </select>
          <label class="tiny" for="listingAlignment">Alignment</label>
          <select id="listingAlignment" style="min-width:170px;">
            <option value="entry_aligned" selected>Time Entered</option>
            <option value="cohort_start_aligned">Start All</option>
            <option value="calendar_aligned">Calendar</option>
          </select>
          <label class="tiny" for="listingNormalization">Scale</label>
          <select id="listingNormalization" style="min-width:120px;">
            <option value="indexed" selected>Indexed</option>
            <option value="returns">Returns</option>
          </select>
          <button id="listingRefreshBtn">REFRESH</button>
          <button id="listingExportCsvBtn">EXPORT CSV</button>
          <span class="tiny" id="listingMeta">Idle</span>
        </div>
        <div style="display:grid; grid-template-columns: 320px 1fr; gap:12px; margin-top:10px;">
          <div class="scroll" style="max-height:420px;">
            <div class="tiny">Candidate Products (multi-select)</div>
            <select id="listingProducts" multiple style="min-width:100%; height:180px;"></select>
            <div class="tiny" style="margin-top:8px;">Candidates</div>
            <table>
              <thead><tr><th>Pair</th><th>Anchor</th><th>Src</th><th>Conf</th></tr></thead>
              <tbody id="listingCandidatesBody"></tbody>
            </table>
          </div>
          <div>
            <canvas id="listingOverlayChart" width="920" height="300"></canvas>
            <div class="tiny" id="listingSummary"></div>
            <div class="tiny warn" id="listingErrors"></div>
          </div>
        </div>
      </div>

      <div class="card table-card">
        <div class="tiny">Current Books</div>
        <div class="scroll">
          <table>
            <thead><tr><th>Market</th><th>Bid</th><th>Ask</th><th>Spread</th><th>TS</th></tr></thead>
            <tbody id="booksBody"></tbody>
          </table>
        </div>
      </div>

      <div class="card table-card">
        <div class="tiny">Recent Executions</div>
        <div class="scroll">
          <table>
            <thead><tr><th>TS</th><th>Venue</th><th>Status</th><th>Side</th><th>Qty</th><th>Px</th></tr></thead>
            <tbody id="execBody"></tbody>
          </table>
        </div>
      </div>

      <div class="card table-card">
        <div class="tiny">Execution Costs</div>
        <div class="scroll">
          <table>
            <thead><tr><th>TS</th><th>Exec</th><th>Venue</th><th>Fee bps</th><th>Slip bps</th><th>Edge</th></tr></thead>
            <tbody id="costBody"></tbody>
          </table>
        </div>
      </div>

      <div class="card table-card">
        <div class="tiny">Coinbase Wallet</div>
        <div class="scroll">
          <table>
            <thead><tr><th>Asset</th><th>Avail</th><th>Hold</th><th>USD</th><th>Drift USD</th></tr></thead>
            <tbody id="walletBody"></tbody>
          </table>
        </div>
      </div>

      <div class="card table-card">
        <div class="tiny">Coinbase Open Orders</div>
        <div class="scroll">
          <table>
            <thead><tr><th>Order</th><th>Product</th><th>Side</th><th>Status</th><th>Filled</th></tr></thead>
            <tbody id="cbOrdersBody"></tbody>
          </table>
        </div>
      </div>

      <div class="card table-card">
        <div class="tiny">Coinbase L2 Top</div>
        <div class="scroll">
          <table>
            <thead><tr><th>Product</th><th>Best Bid</th><th>Best Ask</th><th>Seq</th></tr></thead>
            <tbody id="cbBookBody"></tbody>
          </table>
        </div>
      </div>

      <div class="card table-card">
        <div class="tiny">Route Opportunities</div>
        <div class="scroll">
          <table>
            <thead><tr><th>Route</th><th>Net bps</th><th>Gross bps</th><th>USD</th></tr></thead>
            <tbody id="routesBody"></tbody>
          </table>
        </div>
      </div>

      <div class="card table-card">
        <div class="tiny">Feed Health</div>
        <div class="tiny" id="feedHealthSummary">feed health: -</div>
      </div>

      <div class="card table-card">
        <div class="tiny">Parity Monitor</div>
        <div style="display:flex; gap:8px; margin-top:8px; flex-wrap:wrap;">
          <button id="parityExportCsvBtn">EXPORT PARITY CSV</button>
          <button id="parityExportServerBtn">EXPORT PARITY CSV (SERVER)</button>
          <span class="tiny" id="parityExportResult"></span>
        </div>
        <div class="scroll">
          <table>
            <thead><tr><th>Route</th><th>Strategy</th><th>Gross bps</th><th>Net bps</th><th>Cost bps</th><th>Min bps</th><th>USD</th><th>Gate</th><th>Reasons</th></tr></thead>
            <tbody id="parityBody"></tbody>
          </table>
        </div>
      </div>

      <div class="card table-card">
        <div class="tiny">Rebalance Plan (Assist)</div>
        <div class="tiny" id="rebalanceSummary">No active plan</div>
        <div style="display:flex; gap:8px; margin-top:8px; flex-wrap:wrap;">
          <input id="rebalanceToken" placeholder="approval token id" />
          <button id="approveRebalanceBtn">APPROVE PLAN</button>
          <button id="rejectRebalanceBtn">REJECT PLAN</button>
        </div>
        <div class="tiny" id="rebalanceOpsResult"></div>
      </div>

      <div class="card table-card">
        <div class="tiny">Coinbase Auth</div>
        <div class="tiny" id="coinbaseAuthSummary">auth: unknown</div>
        <div style="display:flex; gap:8px; margin-top:8px; flex-wrap:wrap;">
          <input id="coinbaseAuthProfile" placeholder="profile id (e.g. primary)" />
          <button id="reloadAuthBtn">RELOAD AUTH</button>
          <button id="switchAuthBtn">SWITCH PROFILE</button>
        </div>
        <div class="tiny" id="coinbaseAuthOpsResult"></div>
      </div>

      <div class="card table-card">
        <div class="tiny">Wallet Convert (Maker-First)</div>
        <div style="display:flex; gap:8px; margin-top:8px; flex-wrap:wrap;">
          <select id="convertAccountSelect" style="min-width:220px;"></select>
          <select id="convertFromAssetSelect" style="min-width:120px;"></select>
          <select id="convertToAssetSelect" style="min-width:120px;"></select>
          <input id="convertAmount" placeholder="amount (blank=all available)" style="min-width:200px;" />
          <input id="convertConfirm" placeholder="live confirm phrase" style="min-width:220px;" />
          <button id="convertPreviewBtn">PREVIEW</button>
          <button id="convertPaperBtn">PAPER EXEC</button>
          <button id="convertLiveBtn">LIVE EXEC</button>
        </div>
        <div class="tiny" id="convertResult"></div>
      </div>

      <div class="card table-card">
        <div class="tiny">Maker Orderbook Speed Test</div>
        <div style="display:flex; gap:8px; margin-top:8px; flex-wrap:wrap;">
          <select id="makerProductSelect" style="min-width:160px;"></select>
          <select id="makerSideSelect" style="min-width:120px;">
            <option value="buy">BUY</option>
            <option value="sell">SELL</option>
          </select>
          <input id="makerBaseSize" value="0.0001" style="min-width:140px;" />
          <input id="makerConfirm" placeholder="live confirm phrase" style="min-width:220px;" />
          <button id="makerPaperBtn">PAPER TEST</button>
          <button id="makerLiveBtn">LIVE TEST</button>
        </div>
        <div class="tiny" id="makerResult"></div>
      </div>

      <div class="card table-card">
        <div class="tiny">Route Execution Plans</div>
        <div class="scroll">
          <table>
            <thead><tr><th>TS</th><th>Route</th><th>Approved</th><th>Reason</th></tr></thead>
            <tbody id="routeExecBody"></tbody>
          </table>
        </div>
      </div>

      <div class="card table-card" style="grid-column: span 12;">
        <div class="tiny">Asset Bias</div>
        <div class="scroll">
          <table>
            <thead><tr><th>Asset</th><th>Bias</th></tr></thead>
            <tbody id="biasBody"></tbody>
          </table>
        </div>
      </div>
    </div>
  </div>

  <script>
    const pnlSeries = [];
    let selectedMarketId = null;
    let marketSignature = '';
    let marketsById = new Map();
    let activeTab = 'chart';
    let lastListingFetchMs = 0;
    let listingCandidates = [];
    let lastListingOverlay = null;
    let lastParityRows = [];

    function fmtNum(n) {
      const v = Number(n || 0);
      if (!Number.isFinite(v)) return '-';
      return v.toFixed(4);
    }

    function escapeHtml(value) {
      return String(value || '')
        .replaceAll('&', '&amp;')
        .replaceAll('<', '&lt;')
        .replaceAll('>', '&gt;');
    }

    function drawSeries(canvasId, series, color) {
      const canvas = document.getElementById(canvasId);
      const ctx = canvas.getContext('2d');
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      ctx.strokeStyle = '#334155';
      ctx.lineWidth = 1;
      for (let i = 0; i < 5; i++) {
        const y = 20 + i * 45;
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(canvas.width, y);
        ctx.stroke();
      }

      if (series.length < 2) return;

      const min = Math.min(...series);
      const max = Math.max(...series);
      const span = (max - min) || 1;

      ctx.strokeStyle = color;
      ctx.lineWidth = 2;
      ctx.beginPath();
      series.forEach((v, i) => {
        const x = (i / (series.length - 1)) * (canvas.width - 20) + 10;
        const y = canvas.height - 15 - ((v - min) / span) * (canvas.height - 30);
        if (i === 0) {
          ctx.moveTo(x, y);
        } else {
          ctx.lineTo(x, y);
        }
      });
      ctx.stroke();
    }

    function drawBars(canvasId, series) {
      const canvas = document.getElementById(canvasId);
      const ctx = canvas.getContext('2d');
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      if (!series.length) return;
      const maxAbs = Math.max(...series.map(v => Math.abs(v))) || 1;
      const zeroY = canvas.height / 2;

      ctx.strokeStyle = '#334155';
      ctx.beginPath();
      ctx.moveTo(0, zeroY);
      ctx.lineTo(canvas.width, zeroY);
      ctx.stroke();

      const barW = Math.max(2, Math.floor(canvas.width / Math.max(series.length, 1)));
      series.forEach((v, i) => {
        const h = (Math.abs(v) / maxAbs) * (canvas.height * 0.45);
        const x = i * barW;
        const y = v >= 0 ? zeroY - h : zeroY;
        ctx.fillStyle = v >= 0 ? '#10b981' : '#ef4444';
        ctx.fillRect(x, y, Math.max(1, barW - 1), h);
      });
    }

    function drawMultiSeries(canvasId, seriesRows) {
      const canvas = document.getElementById(canvasId);
      const ctx = canvas.getContext('2d');
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      if (!Array.isArray(seriesRows) || seriesRows.length === 0) return;

      const palette = ['#38bdf8', '#10b981', '#ef4444', '#f59e0b', '#a78bfa', '#22d3ee', '#fb7185', '#facc15', '#34d399', '#60a5fa'];
      const all = [];
      for (const row of seriesRows) {
        for (const v of (row.values || [])) {
          const n = Number(v);
          if (Number.isFinite(n)) all.push(n);
        }
      }
      if (all.length < 2) return;
      const min = Math.min(...all);
      const max = Math.max(...all);
      const span = (max - min) || 1;

      ctx.strokeStyle = '#334155';
      ctx.lineWidth = 1;
      for (let i = 0; i < 5; i++) {
        const y = 20 + i * ((canvas.height - 40) / 4);
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(canvas.width, y);
        ctx.stroke();
      }

      seriesRows.forEach((row, idx) => {
        const vals = (row.values || []).map(Number).filter(x => Number.isFinite(x));
        if (vals.length < 2) return;
        ctx.strokeStyle = palette[idx % palette.length];
        ctx.lineWidth = 2;
        ctx.beginPath();
        vals.forEach((v, i) => {
          const x = (i / (vals.length - 1)) * (canvas.width - 20) + 10;
          const y = canvas.height - 15 - ((v - min) / span) * (canvas.height - 30);
          if (i === 0) ctx.moveTo(x, y);
          else ctx.lineTo(x, y);
        });
        ctx.stroke();
      });
    }

    function aggregatePoints(points, step) {
      const out = [];
      if (!Array.isArray(points) || points.length === 0) return out;
      const chunkSize = Math.max(1, Number(step || 1));
      for (let i = 0; i < points.length; i += chunkSize) {
        const chunk = points.slice(i, i + chunkSize);
        if (!chunk.length) continue;
        const first = chunk[0];
        const last = chunk[chunk.length - 1];
        out.push({
          market_id: last.market_id,
          ts: last.ts,
          mid: Number(last.mid || 0),
          spread: Number(last.spread || 0),
          delta: Number(last.mid || 0) - Number(first.mid || 0),
        });
      }
      return out;
    }

    function syncSelect(selectId, values, fallback) {
      const select = document.getElementById(selectId);
      const prev = select.value;
      select.innerHTML = '';
      values.forEach(({ value, label }) => {
        const opt = document.createElement('option');
        opt.value = value;
        opt.textContent = label;
        select.appendChild(opt);
      });
      if (prev && values.some(x => x.value === prev)) {
        select.value = prev;
      } else if (fallback && values.some(x => x.value === fallback)) {
        select.value = fallback;
      }
    }

    function getSelectedValues(selectId) {
      const select = document.getElementById(selectId);
      return Array.from(select.selectedOptions || []).map(opt => opt.value);
    }

    function setMultiSelectValues(selectId, wanted) {
      const want = new Set((wanted || []).map(x => String(x)));
      const select = document.getElementById(selectId);
      Array.from(select.options || []).forEach((opt) => {
        opt.selected = want.has(opt.value);
      });
    }

    function renderListingCandidates(candidates) {
      const body = document.getElementById('listingCandidatesBody');
      body.innerHTML = (candidates || []).slice(0, 200).map((c) => {
        return `<tr><td>${escapeHtml(c.product_id)}</td><td>${escapeHtml(c.anchor_time || '')}</td><td>${escapeHtml(c.source || '')}</td><td>${fmtNum(c.confidence_score)}</td></tr>`;
      }).join('');

      const select = document.getElementById('listingProducts');
      const prevSelected = getSelectedValues('listingProducts');
      const prevSet = new Set(prevSelected);
      select.innerHTML = '';
      (candidates || []).forEach((c, idx) => {
        const opt = document.createElement('option');
        opt.value = c.product_id;
        const label = c.label ? `${c.label} (${c.product_id})` : c.product_id;
        opt.textContent = `${label} [conf=${fmtNum(c.confidence_score)}]`;
        opt.selected = prevSet.size ? prevSet.has(c.product_id) : idx < 8;
        select.appendChild(opt);
      });
    }

    async function refreshListingPanel(force) {
      if (activeTab !== 'listing' && !force) {
        return;
      }
      const now = Date.now();
      if (!force && now - lastListingFetchMs < 15000) {
        return;
      }
      lastListingFetchMs = now;

      const windowPreset = document.getElementById('listingWindow').value || '90d';
      const granularity = Number(document.getElementById('listingGranularity').value || '14400');
      const alignment = document.getElementById('listingAlignment').value || 'entry_aligned';
      const normalization = document.getElementById('listingNormalization').value || 'indexed';

      const errorsNode = document.getElementById('listingErrors');
      const summaryNode = document.getElementById('listingSummary');
      const metaNode = document.getElementById('listingMeta');
      errorsNode.textContent = '';
      metaNode.textContent = 'Loading listing data...';

      try {
        const candidatesUrl = `/state/listings/candidates?window=${encodeURIComponent(windowPreset)}&granularity_sec=${granularity}&max_scan=400&max_results=120&min_candles=20&min_gap_candles=2`;
        const candidatesResp = await fetch(candidatesUrl).then(x => x.json());
        listingCandidates = candidatesResp.candidates || [];
        renderListingCandidates(listingCandidates);

        let selectedProducts = getSelectedValues('listingProducts');
        if (!selectedProducts.length) {
          selectedProducts = listingCandidates.slice(0, 8).map(x => x.product_id);
          setMultiSelectValues('listingProducts', selectedProducts);
        }

        const overlayBody = {
          window_preset: windowPreset,
          granularity_sec: granularity,
          alignment_mode: alignment,
          normalization,
          product_ids: selectedProducts,
          max_scan: 400,
          min_candles: 20,
          min_gap_candles: 2,
        };
        const overlay = await postJsonData('/state/listings/overlay', overlayBody);
        lastListingOverlay = overlay;
        const rows = (overlay.series || []).map((s) => ({ label: s.label || s.product_id, values: s.values || [] }));
        drawMultiSeries('listingOverlayChart', rows);

        const diagnostics = overlay.diagnostics || [];
        const summaryRows = overlay.summary_rows || [];
        const top = summaryRows.slice(0, 6).map((r) => {
          return `${r.product_id}: r1=${fmtNum((r.ret_1 || 0) * 100)}% r3=${fmtNum((r.ret_3 || 0) * 100)}% r10=${fmtNum((r.ret_10 || 0) * 100)}%`;
        });
        summaryNode.textContent =
          `series=${rows.length} selected=${selectedProducts.length} mode=${overlay.alignment_mode || alignment} scale=${overlay.normalization || normalization}` +
          (top.length ? ` | ${top.join(' | ')}` : '');
        errorsNode.textContent = (overlay.errors || []).join(' | ');
        metaNode.textContent = `Updated ${new Date().toLocaleTimeString()} | ${diagnostics.join(' | ')}`;
      } catch (err) {
        errorsNode.textContent = String(err);
        metaNode.textContent = 'Listing refresh failed';
      }
    }

    function toCsvCell(value) {
      const s = String(value ?? '');
      if (s.includes(',') || s.includes('"') || s.includes('\n')) {
        return `"${s.replaceAll('"', '""')}"`;
      }
      return s;
    }

    function downloadTextFile(filename, text, contentType) {
      const blob = new Blob([text], { type: contentType || 'text/plain;charset=utf-8' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = filename;
      document.body.appendChild(a);
      a.click();
      a.remove();
      URL.revokeObjectURL(url);
    }

    function exportListingCsv() {
      const overlay = lastListingOverlay;
      if (!overlay || !Array.isArray(overlay.series) || overlay.series.length === 0) {
        document.getElementById('listingErrors').textContent = 'no listing overlay data to export';
        return;
      }

      const rows = [];
      rows.push('product_id,label,source,anchor_time,index,ts,value');
      for (const s of overlay.series) {
        const vals = Array.isArray(s.values) ? s.values : [];
        const ts = Array.isArray(s.ts) ? s.ts : [];
        for (let i = 0; i < vals.length; i++) {
          rows.push([
            toCsvCell(s.product_id),
            toCsvCell(s.label),
            toCsvCell(s.source),
            toCsvCell(s.anchor_time || ''),
            toCsvCell(i),
            toCsvCell(ts[i] || ''),
            toCsvCell(vals[i]),
          ].join(','));
        }
      }

      const now = new Date();
      const y = now.getUTCFullYear();
      const m = String(now.getUTCMonth() + 1).padStart(2, '0');
      const d = String(now.getUTCDate()).padStart(2, '0');
      const hh = String(now.getUTCHours()).padStart(2, '0');
      const mm = String(now.getUTCMinutes()).padStart(2, '0');
      const ss = String(now.getUTCSeconds()).padStart(2, '0');
      const filename = `listing_overlay_${y}${m}${d}_${hh}${mm}${ss}.csv`;
      downloadTextFile(filename, rows.join('\n'), 'text/csv;charset=utf-8');
      document.getElementById('listingMeta').textContent = `Exported CSV ${filename} with ${rows.length - 1} rows`;
    }

    function exportParityCsv() {
      const rows = Array.isArray(lastParityRows) ? lastParityRows : [];
      if (!rows.length) {
        document.getElementById('status').textContent = 'No parity rows to export';
        return;
      }
      const lines = [];
      lines.push('route_id,strategy_class,gross_edge_bps,expected_net_bps,cost_bps,min_required_bps,expected_usd_profit,capital_required_usd,pass,reasons,ts,path');
      for (const r of rows) {
        const gross = Number(r.gross_edge_bps || 0);
        const net = Number(r.expected_net_bps || 0);
        const cost = gross - net;
        lines.push([
          toCsvCell(r.route_id),
          toCsvCell(r.strategy_class),
          toCsvCell(gross),
          toCsvCell(net),
          toCsvCell(cost),
          toCsvCell(r.min_required_bps),
          toCsvCell(r.expected_usd_profit),
          toCsvCell(r.capital_required_usd),
          toCsvCell(r.pass ? 'PASS' : 'FAIL'),
          toCsvCell((r.reasons || []).join('|')),
          toCsvCell(r.ts || ''),
          toCsvCell(r.path || ''),
        ].join(','));
      }
      const now = new Date();
      const y = now.getUTCFullYear();
      const m = String(now.getUTCMonth() + 1).padStart(2, '0');
      const d = String(now.getUTCDate()).padStart(2, '0');
      const hh = String(now.getUTCHours()).padStart(2, '0');
      const mm = String(now.getUTCMinutes()).padStart(2, '0');
      const ss = String(now.getUTCSeconds()).padStart(2, '0');
      const filename = `parity_monitor_${y}${m}${d}_${hh}${mm}${ss}.csv`;
      downloadTextFile(filename, lines.join('\n'), 'text/csv;charset=utf-8');
      document.getElementById('status').textContent = `Exported parity CSV ${filename} with ${rows.length} rows`;
    }

    async function exportParityCsvServer() {
      const res = await postJsonData('/state/parity/export-csv', {
        limit: 500,
        include_failures: true,
      });
      const resultNode = document.getElementById('parityExportResult');
      if (res && res.ok) {
        resultNode.textContent = `server export ok rows=${res.row_count} file=${res.file_path || '-'}`;
      } else {
        resultNode.textContent = `server export failed: ${res && res.message ? res.message : 'unknown error'}`;
      }
    }

    function setActiveTab(tab) {
      activeTab = tab;
      const chartBtn = document.getElementById('chartTabBtn');
      const backtestBtn = document.getElementById('backtestTabBtn');
      const listingBtn = document.getElementById('listingTabBtn');
      const marketChartCard = document.getElementById('marketChartCard');
      const chartCard = document.getElementById('selectedBookCard');
      const backtesterCard = document.getElementById('backtesterCard');
      const listingCard = document.getElementById('listingPatternCard');

      if (tab === 'backtest') {
        backtestBtn.classList.add('active');
        chartBtn.classList.remove('active');
        listingBtn.classList.remove('active');
        marketChartCard.classList.add('hidden');
        chartCard.classList.add('hidden');
        backtesterCard.classList.remove('hidden');
        listingCard.classList.add('hidden');
      } else if (tab === 'listing') {
        backtestBtn.classList.remove('active');
        chartBtn.classList.remove('active');
        listingBtn.classList.add('active');
        marketChartCard.classList.add('hidden');
        chartCard.classList.remove('hidden');
        backtesterCard.classList.add('hidden');
        listingCard.classList.remove('hidden');
        refreshListingPanel(true);
      } else {
        chartBtn.classList.add('active');
        backtestBtn.classList.remove('active');
        listingBtn.classList.remove('active');
        marketChartCard.classList.remove('hidden');
        chartCard.classList.remove('hidden');
        backtesterCard.classList.add('hidden');
        listingCard.classList.add('hidden');
      }
    }

    function syncMarketSelect(markets) {
      marketsById = new Map((markets || []).map(x => [x.market_id, x]));
      const select = document.getElementById('marketSelect');
      const ids = (markets || []).map(x => x.market_id);
      const signature = ids.join('|');

      if (!selectedMarketId || !ids.includes(selectedMarketId)) {
        selectedMarketId = ids.length > 0 ? ids[0] : null;
      }

      if (signature !== marketSignature) {
        marketSignature = signature;
        select.innerHTML = '';
        ids.forEach(id => {
          const market = marketsById.get(id);
          const opt = document.createElement('option');
          opt.value = id;
          const name = market ? market.display_name : id;
          const question = market && market.question ? ` | ${market.question}` : '';
          opt.textContent = `${name}${question}`;
          select.appendChild(opt);
        });
      }

      if (selectedMarketId) {
        select.value = selectedMarketId;
      }
    }

    function drawMarket(points) {
      const step = Number(document.getElementById('granularitySelect').value || '1');
      const aggregated = aggregatePoints(points, step);
      const mids = aggregated.map(p => Number(p.mid || 0));
      const deltas = aggregated.map(p => Number(p.delta || 0));
      drawSeries('marketChart', mids, '#10b981');
      drawBars('marketBarChart', deltas);
      const meta = document.getElementById('marketMeta');
      if (aggregated.length === 0) {
        meta.textContent = 'No market history points available yet';
        return;
      }

      const last = aggregated[aggregated.length - 1];
      const marketId = selectedMarketId || last.market_id;
      const market = marketsById.get(marketId);
      const name = market ? market.display_name : marketId;
      meta.textContent = `${name} mid=${fmtNum(last.mid)} spread=${fmtNum(last.spread)} ts=${new Date(last.ts).toLocaleTimeString()} step=${step}x`;
    }

    async function op(path) {
      try {
        const r = await fetch(path, { method: 'POST' });
        document.getElementById('opsResult').textContent = await r.text();
      } catch (e) {
        document.getElementById('opsResult').textContent = String(e);
      }
    }

    async function postJson(path, body) {
      const r = await fetch(path, {
        method: 'POST',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify(body),
      });
      return r.text();
    }

    async function postJsonData(path, body) {
      const r = await fetch(path, {
        method: 'POST',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify(body),
      });
      const text = await r.text();
      try {
        return JSON.parse(text);
      } catch (_) {
        return { ok: false, message: text };
      }
    }

    function syncConvertControls(wallet, markets, cbBook) {
      const balances = (wallet || []).filter(x => Number(x.available || 0) > 0);
      const byAccount = new Map();
      balances.forEach(b => {
        const key = b.account_id || 'unknown';
        byAccount.set(key, (byAccount.get(key) || 0) + Number(b.usd_value || 0));
      });

      const accountOptions = [{ value: '', label: 'All Wallets' }];
      [...byAccount.entries()]
        .sort((a, b) => b[1] - a[1])
        .forEach(([accountId, usd]) => {
          accountOptions.push({ value: accountId, label: `${accountId} ($${fmtNum(usd)})` });
        });
      syncSelect('convertAccountSelect', accountOptions, '');

      const assets = [...new Set(balances.map(x => String(x.asset || '').toUpperCase()).filter(Boolean))];
      assets.sort();
      const assetOptions = assets.map(x => ({ value: x, label: x }));
      if (!assetOptions.length) {
        assetOptions.push({ value: 'USD', label: 'USD' });
      }
      syncSelect('convertFromAssetSelect', assetOptions, assetOptions[0].value);

      const toSet = new Set(['USD', 'USDC', 'BTC', 'ETH', 'SOL', 'XRP']);
      assetOptions.forEach(x => toSet.add(x.value));
      const toOptions = [...toSet].sort().map(x => ({ value: x, label: x }));
      syncSelect('convertToAssetSelect', toOptions, 'USD');

      const marketProducts = new Set((markets || []).map(x => x.pair_product).filter(Boolean));
      (cbBook || []).forEach(x => marketProducts.add(x.product_id));
      const productOptions = [...marketProducts].sort().map(x => ({ value: x, label: x }));
      if (!productOptions.length) {
        productOptions.push({ value: 'BTC-USD', label: 'BTC-USD' });
      }
      syncSelect('makerProductSelect', productOptions, productOptions[0].value);
    }

    function renderSelectedOrderbook(cbBook) {
      const body = document.getElementById('selectedBookBody');
      const meta = document.getElementById('selectedBookMeta');
      const selected = selectedMarketId ? marketsById.get(selectedMarketId) : null;
      const selectedProduct = selected ? selected.pair_product : null;
      if (!selectedProduct) {
        meta.textContent = 'No market selected';
        body.innerHTML = '';
        return;
      }

      const book = (cbBook || []).find(x => x.product_id === selectedProduct);
      if (!book) {
        meta.textContent = `${selectedProduct} orderbook unavailable`;
        body.innerHTML = '';
        return;
      }

      meta.textContent = `${selectedProduct} seq=${book.sequence_num || 0}`;
      const bids = (book.bids || []).slice(0, 12);
      const asks = (book.asks || []).slice(0, 12);
      const depth = Math.max(bids.length, asks.length, 12);
      const rows = [];
      for (let i = 0; i < depth; i++) {
        const bid = bids[i];
        const ask = asks[i];
        rows.push(
          `<tr><td class='buy'>${bid ? fmtNum(bid[0]) : ''}</td><td>${bid ? fmtNum(bid[1]) : ''}</td><td class='sell'>${ask ? fmtNum(ask[0]) : ''}</td><td>${ask ? fmtNum(ask[1]) : ''}</td></tr>`,
        );
      }
      body.innerHTML = rows.join('');
    }

    async function runConvert(execute, live) {
      const accountId = (document.getElementById('convertAccountSelect').value || '').trim();
      const fromAsset = (document.getElementById('convertFromAssetSelect').value || '').trim();
      const toAsset = (document.getElementById('convertToAssetSelect').value || '').trim();
      const amountRaw = (document.getElementById('convertAmount').value || '').trim();
      const confirm = (document.getElementById('convertConfirm').value || '').trim();
      const amount = amountRaw === '' ? null : Number(amountRaw);
      if (!fromAsset || !toAsset) {
        document.getElementById('convertResult').textContent = 'select source and target assets';
        return;
      }

      const payload = {
        account_id: accountId || null,
        from_asset: fromAsset,
        to_asset: toAsset,
        amount: Number.isFinite(amount) ? amount : null,
        live,
        confirm,
      };
      const path = execute ? '/ops/coinbase/convert/execute' : '/ops/coinbase/convert/preview';
      const res = await postJsonData(path, payload);
      document.getElementById('convertResult').textContent =
        `${res.mode || (live ? 'live' : 'paper')} ok=${!!res.ok} preview=${!!res.preview_ok} ` +
        `product=${res.product_id || '-'} side=${res.side || '-'} amount=${fmtNum(res.amount_from)} ` +
        `price=${fmtNum(res.limit_price)} order=${res.order_id || '-'} msg=${res.message || ''}`;
    }

    async function runMakerTest(live) {
      const productId = (document.getElementById('makerProductSelect').value || '').trim();
      const side = (document.getElementById('makerSideSelect').value || 'buy').trim();
      const baseSize = Number((document.getElementById('makerBaseSize').value || '0').trim());
      const confirm = (document.getElementById('makerConfirm').value || '').trim();
      if (!productId || !Number.isFinite(baseSize) || baseSize <= 0) {
        document.getElementById('makerResult').textContent = 'invalid product or base size';
        return;
      }
      const res = await postJsonData('/ops/coinbase/maker-test', {
        product_id: productId,
        side,
        base_size: baseSize,
        live,
        confirm,
      });
      document.getElementById('makerResult').textContent =
        `${res.mode || (live ? 'live' : 'paper')} ok=${!!res.ok} preview=${!!res.preview_ok} ` +
        `product=${res.product_id || '-'} side=${res.side || '-'} price=${fmtNum(res.limit_price)} ` +
        `ms(total=${fmtNum(res.total_ms)}, preview=${fmtNum(res.preview_ms)}, post=${fmtNum(res.post_ms)}, cancel=${fmtNum(res.cancel_ms)}) ` +
        `order=${res.order_id || '-'} msg=${res.message || ''}`;
    }

    async function tick() {
      try {
        const [h, r, b, e, exCosts, vectors, wallet, allocations, rebalance, cbOrders, cbBook, cbAuth, routes, routeExec, feeSummary, bias, inv, markets, feedHealth, parity] = await Promise.all([
          fetch('/health').then(x => x.json()),
          fetch('/state/risk').then(x => x.json()),
          fetch('/state/books').then(x => x.json()),
          fetch('/state/executions').then(x => x.json()),
          fetch('/state/execution/costs').then(x => x.json()),
          fetch('/state/execution/vectors').then(x => x.json()),
          fetch('/state/coinbase/wallet').then(x => x.json()),
          fetch('/state/coinbase/allocations').then(x => x.json()),
          fetch('/state/coinbase/rebalance-plan').then(x => x.json()),
          fetch('/state/coinbase/orders').then(x => x.json()),
          fetch('/state/coinbase/orderbook').then(x => x.json()),
          fetch('/state/coinbase/auth').then(x => x.json()),
          fetch('/state/routes/opportunities').then(x => x.json()),
          fetch('/state/routes/executions').then(x => x.json()),
          fetch('/state/fees/summary').then(x => x.json()),
          fetch('/state/bias').then(x => x.json()),
          fetch('/state/inventory').then(x => x.json()),
          fetch('/state/markets').then(x => x.json()),
          fetch('/state/feed/health').then(x => x.json()),
          fetch('/state/parity/monitor').then(x => x.json()),
        ]);

        syncMarketSelect(markets);
        syncConvertControls(wallet, markets, cbBook);

        let historyUrl = '/state/history?limit=360';
        if (selectedMarketId) {
          historyUrl += `&market_id=${encodeURIComponent(selectedMarketId)}`;
        }
        const history = await fetch(historyUrl).then(x => x.json());
        if (history.market_id && history.market_id !== selectedMarketId) {
          selectedMarketId = history.market_id;
          syncMarketSelect(markets);
        }

        document.getElementById('status').textContent = `Updated ${new Date().toLocaleTimeString()}`;
        document.getElementById('k_kill').textContent = h.kill_switch;
        document.getElementById('k_pnl').textContent = fmtNum(r.daily_pnl);
        document.getElementById('k_open').textContent = fmtNum(r.open_notional);
        document.getElementById('k_delta').textContent = fmtNum(r.unhedged_delta);
        document.getElementById('k_markets').textContent = r.open_markets;
        document.getElementById('k_inv').textContent = fmtNum(inv.inventory_usd);
        document.getElementById('k_routes').textContent = (routes || []).length;
        const makerFee = feeSummary && feeSummary.maker_fee_rate ? Number(feeSummary.maker_fee_rate) * 10000 : 0;
        const takerFee = feeSummary && feeSummary.taker_fee_rate ? Number(feeSummary.taker_fee_rate) * 10000 : 0;
        document.getElementById('k_fees').textContent = `${fmtNum(makerFee)}/${fmtNum(takerFee)} bps`;
        document.getElementById('vectorInfo').textContent =
          `vectors: entry<=${fmtNum(vectors.entry_max_slippage_bps)}bps exit<=${fmtNum(vectors.exit_max_slippage_bps)}bps unwind<=${fmtNum(vectors.max_cross_bps_unwind)}bps`;

        pnlSeries.push(Number(r.daily_pnl || 0));
        if (pnlSeries.length > 240) pnlSeries.shift();
        drawSeries('pnlChart', pnlSeries, '#38bdf8');
        drawMarket(history.points || []);
        renderSelectedOrderbook(cbBook || []);

        const booksBody = document.getElementById('booksBody');
        booksBody.innerHTML = b.slice(0, 120).map(x => {
          const market = marketsById.get(x.market_id);
          const label = market ? market.display_name : x.market_id;
          return `<tr><td title='${escapeHtml(x.market_id)}'>${escapeHtml(label)}</td><td>${fmtNum(x.bid)}</td><td>${fmtNum(x.ask)}</td><td>${fmtNum(x.spread)}</td><td>${new Date(x.ts).toLocaleTimeString()}</td></tr>`;
        }).join('');

        const execBody = document.getElementById('execBody');
        execBody.innerHTML = e.slice(0, 160).map(x => {
          const sideClass = x.side === 'Buy' ? 'buy' : 'sell';
          const statusClass = (x.status === 'Rejected' || x.status === 'Error') ? 'warn' : '';
          return `<tr><td>${new Date(x.ts).toLocaleTimeString()}</td><td>${x.venue}</td><td class="${statusClass}">${x.status}</td><td class="${sideClass}">${x.side}</td><td>${fmtNum(x.filled_qty)}</td><td>${fmtNum(x.avg_px)}</td></tr>`;
        }).join('');

        const costBody = document.getElementById('costBody');
        costBody.innerHTML = exCosts.slice(0, 160).map(x =>
          `<tr><td>${new Date(x.ts).toLocaleTimeString()}</td><td>${x.execution_id}</td><td>${x.venue}</td><td>${fmtNum(x.fee_bps)}</td><td>${fmtNum(x.slippage_bps)}</td><td>${fmtNum(x.effective_edge)}</td></tr>`
        ).join('');

        const drifts = new Map((allocations || []).map(x => [x.asset, x]));
        const walletBody = document.getElementById('walletBody');
        walletBody.innerHTML = wallet.slice(0, 64).map(x => {
          const drift = drifts.get(x.asset);
          const driftUsd = drift ? drift.drift_usd : 0;
          return `<tr><td>${x.asset}</td><td>${fmtNum(x.available)}</td><td>${fmtNum(x.hold)}</td><td>${fmtNum(x.usd_value)}</td><td>${fmtNum(driftUsd)}</td></tr>`;
        }).join('');

        const cbOrdersBody = document.getElementById('cbOrdersBody');
        cbOrdersBody.innerHTML = cbOrders.slice(0, 120).map(x =>
          `<tr><td>${x.order_id}</td><td>${x.product_id}</td><td>${x.side}</td><td>${x.status}</td><td>${x.filled_size}</td></tr>`
        ).join('');

        const cbBookBody = document.getElementById('cbBookBody');
        cbBookBody.innerHTML = (cbBook || []).slice(0, 80).map(x => {
          const bid = (x.bids && x.bids.length) ? Number(x.bids[0][0]) : 0;
          const ask = (x.asks && x.asks.length) ? Number(x.asks[0][0]) : 0;
          return `<tr><td>${x.product_id}</td><td>${fmtNum(bid)}</td><td>${fmtNum(ask)}</td><td>${x.sequence_num || 0}</td></tr>`;
        }).join('');

        const routesBody = document.getElementById('routesBody');
        routesBody.innerHTML = (routes || []).slice(0, 80).map(x =>
          `<tr><td>${x.route_id}</td><td>${fmtNum(x.expected_net_bps)}</td><td>${fmtNum(x.gross_edge_bps)}</td><td>${fmtNum(x.expected_usd_profit)}</td></tr>`
        ).join('');

        const feedHealthSummary = document.getElementById('feedHealthSummary');
        feedHealthSummary.textContent =
          `ws=${feedHealth.ws_healthy ? 'healthy' : 'stale'} hb_age_ms=${feedHealth.heartbeat_age_ms || '-'} ` +
          `seq=${fmtNum(feedHealth.heartbeat_seq)} l2=${feedHealth.l2_updates || 0} gaps=${feedHealth.sequence_gaps || 0} ` +
          `timeouts=${feedHealth.read_timeouts || 0} streak=${fmtNum(feedHealth.timeout_streak || 0)} ` +
          `ping_fail=${feedHealth.ping_failures || 0} hb_to=${feedHealth.heartbeat_timeouts || 0} ` +
          `rejects_10m=${feedHealth.reject_events_10m || 0}`;

        const parityBody = document.getElementById('parityBody');
        lastParityRows = (parity.rows || []).slice(0, 200);
        parityBody.innerHTML = lastParityRows.slice(0, 80).map(x => {
          const gross = Number(x.gross_edge_bps || 0);
          const net = Number(x.expected_net_bps || 0);
          const cost = gross - net;
          const gateClass = x.pass ? 'buy' : 'warn';
          return `<tr><td>${x.route_id}</td><td>${x.strategy_class}</td><td>${fmtNum(gross)}</td><td>${fmtNum(net)}</td><td>${fmtNum(cost)}</td><td>${fmtNum(x.min_required_bps)}</td><td>${fmtNum(x.expected_usd_profit)}</td><td class='${gateClass}'>${x.pass ? 'PASS' : 'FAIL'}</td><td>${escapeHtml((x.reasons || []).join('|'))}</td></tr>`;
        }).join('');

        const routeExecBody = document.getElementById('routeExecBody');
        routeExecBody.innerHTML = (routeExec || []).slice(0, 80).map(x =>
          `<tr><td>${new Date(x.ts).toLocaleTimeString()}</td><td>${x.route_id}</td><td>${x.approved ? 'yes' : 'no'}</td><td>${x.reason || ''}</td></tr>`
        ).join('');

        const summary = document.getElementById('rebalanceSummary');
        const tokenInput = document.getElementById('rebalanceToken');
        if (rebalance && rebalance.plan_id) {
          summary.textContent = `plan=${rebalance.plan_id} status=${rebalance.status} intents=${rebalance.intent_count} drift_usd=${fmtNum(rebalance.total_drift_abs_usd)} expires=${new Date(rebalance.expires_ts).toLocaleTimeString()}`;
          if (rebalance.approval_token_id && !tokenInput.value) {
            tokenInput.value = rebalance.approval_token_id;
          }
        } else {
          summary.textContent = 'No active plan';
        }

        const authSummary = document.getElementById('coinbaseAuthSummary');
        const authProfileInput = document.getElementById('coinbaseAuthProfile');
        if (cbAuth && cbAuth.ok) {
          authSummary.textContent = `ok profile=${cbAuth.profile_id || 'legacy'} key=*${cbAuth.key_id_suffix || '-'} source=${cbAuth.source || '-'} loaded=${cbAuth.loaded_at ? new Date(cbAuth.loaded_at).toLocaleTimeString() : '-'}`;
          if (cbAuth.profile_id && !authProfileInput.value) {
            authProfileInput.value = cbAuth.profile_id;
          }
        } else {
          authSummary.textContent = `error ${cbAuth && cbAuth.reason ? cbAuth.reason : 'unavailable'}`;
        }

        const biasBody = document.getElementById('biasBody');
        biasBody.innerHTML = bias.map(x => `<tr><td>${x.asset}</td><td>${fmtNum(x.bias)}</td></tr>`).join('');

        if (activeTab === 'listing') {
          await refreshListingPanel(false);
        }
      } catch (err) {
        document.getElementById('status').textContent = `Error: ${err}`;
      }
    }

    document.getElementById('marketSelect').addEventListener('change', (ev) => {
      selectedMarketId = ev.target.value || null;
    });

    document.getElementById('granularitySelect').addEventListener('change', () => {
      // Re-render on next tick with the selected aggregation step.
    });

    document.getElementById('chartTabBtn').addEventListener('click', () => setActiveTab('chart'));
    document.getElementById('backtestTabBtn').addEventListener('click', () => setActiveTab('backtest'));
    document.getElementById('listingTabBtn').addEventListener('click', () => setActiveTab('listing'));
    document.getElementById('listingRefreshBtn').addEventListener('click', async () => {
      await refreshListingPanel(true);
    });
    document.getElementById('listingExportCsvBtn').addEventListener('click', () => {
      exportListingCsv();
    });
    document.getElementById('parityExportCsvBtn').addEventListener('click', () => {
      exportParityCsv();
    });
    document.getElementById('parityExportServerBtn').addEventListener('click', async () => {
      await exportParityCsvServer();
    });
    document.getElementById('listingProducts').addEventListener('change', async () => {
      await refreshListingPanel(true);
    });
    document.getElementById('listingWindow').addEventListener('change', async () => {
      await refreshListingPanel(true);
    });
    document.getElementById('listingGranularity').addEventListener('change', async () => {
      await refreshListingPanel(true);
    });
    document.getElementById('listingAlignment').addEventListener('change', async () => {
      await refreshListingPanel(true);
    });
    document.getElementById('listingNormalization').addEventListener('change', async () => {
      await refreshListingPanel(true);
    });

    document.getElementById('convertPreviewBtn').addEventListener('click', async () => {
      await runConvert(false, false);
    });
    document.getElementById('convertPaperBtn').addEventListener('click', async () => {
      await runConvert(true, false);
    });
    document.getElementById('convertLiveBtn').addEventListener('click', async () => {
      await runConvert(true, true);
    });

    document.getElementById('makerPaperBtn').addEventListener('click', async () => {
      await runMakerTest(false);
    });
    document.getElementById('makerLiveBtn').addEventListener('click', async () => {
      await runMakerTest(true);
    });

    document.getElementById('approveRebalanceBtn').addEventListener('click', async () => {
      const token = (document.getElementById('rebalanceToken').value || '').trim();
      if (!token) {
        document.getElementById('rebalanceOpsResult').textContent = 'missing token_id';
        return;
      }
      try {
        const res = await postJson('/ops/coinbase/rebalance/approve', { token_id: token });
        document.getElementById('rebalanceOpsResult').textContent = res;
      } catch (e) {
        document.getElementById('rebalanceOpsResult').textContent = String(e);
      }
    });

    document.getElementById('rejectRebalanceBtn').addEventListener('click', async () => {
      try {
        const res = await fetch('/ops/coinbase/rebalance/reject', { method: 'POST' }).then(x => x.text());
        document.getElementById('rebalanceOpsResult').textContent = res;
      } catch (e) {
        document.getElementById('rebalanceOpsResult').textContent = String(e);
      }
    });

    document.getElementById('reloadAuthBtn').addEventListener('click', async () => {
      try {
        const res = await fetch('/ops/coinbase/auth/reload', { method: 'POST' }).then(x => x.text());
        document.getElementById('coinbaseAuthOpsResult').textContent = res;
      } catch (e) {
        document.getElementById('coinbaseAuthOpsResult').textContent = String(e);
      }
    });

    document.getElementById('switchAuthBtn').addEventListener('click', async () => {
      const profile = (document.getElementById('coinbaseAuthProfile').value || '').trim();
      if (!profile) {
        document.getElementById('coinbaseAuthOpsResult').textContent = 'missing profile_id';
        return;
      }
      try {
        const res = await postJson('/ops/coinbase/auth/switch-profile', { profile_id: profile });
        document.getElementById('coinbaseAuthOpsResult').textContent = res;
      } catch (e) {
        document.getElementById('coinbaseAuthOpsResult').textContent = String(e);
      }
    });

    setActiveTab(activeTab);
    tick();
    setInterval(tick, 1000);
  </script>
</body>
</html>
"#;
