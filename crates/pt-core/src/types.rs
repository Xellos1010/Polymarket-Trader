use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum Asset {
    Btc,
    Eth,
    Sol,
    Xrp,
    Other,
}

impl Asset {
    pub fn from_slug(slug: &str) -> Self {
        if slug.starts_with("btc-") {
            Self::Btc
        } else if slug.starts_with("eth-") {
            Self::Eth
        } else if slug.starts_with("sol-") {
            Self::Sol
        } else if slug.starts_with("xrp-") {
            Self::Xrp
        } else {
            Self::Other
        }
    }

    pub fn from_symbol(symbol: &str) -> Self {
        match symbol.to_ascii_uppercase().as_str() {
            "BTC" => Self::Btc,
            "ETH" => Self::Eth,
            "SOL" => Self::Sol,
            "XRP" => Self::Xrp,
            _ => Self::Other,
        }
    }

    pub fn from_product_id(product_id: &str) -> Self {
        let base = product_id.split('-').next().unwrap_or_default();
        Self::from_symbol(base)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Btc => "BTC",
            Self::Eth => "ETH",
            Self::Sol => "SOL",
            Self::Xrp => "XRP",
            Self::Other => "OTHER",
        }
    }

    pub fn as_product_id(&self) -> Option<&'static str> {
        match self {
            Self::Btc => Some("BTC-USD"),
            Self::Eth => Some("ETH-USD"),
            Self::Sol => Some("SOL-USD"),
            Self::Xrp => Some("XRP-USD"),
            Self::Other => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TimeBucket {
    FiveMinute,
    FifteenMinute,
    Other,
}

impl TimeBucket {
    pub fn from_slug(slug: &str) -> Self {
        if slug.contains("updown-5m") {
            Self::FiveMinute
        } else if slug.contains("updown-15m") {
            Self::FifteenMinute
        } else {
            Self::Other
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MarketTier {
    TierA,
    TierB,
    TierC,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketSnapshot {
    pub market_id: String,
    pub token_id: String,
    pub bid: f64,
    pub ask: f64,
    pub spread: f64,
    pub liquidity: f64,
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketHistoryPoint {
    pub market_id: String,
    pub mid: f64,
    pub spread: f64,
    pub bid: f64,
    pub ask: f64,
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletSignal {
    pub asset: Asset,
    pub horizon: TimeBucket,
    pub bias: f64,
    pub confidence: f64,
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteIntent {
    pub market_id: String,
    pub token_id: String,
    pub bid_px: f64,
    pub ask_px: f64,
    pub bid_sz: f64,
    pub ask_sz: f64,
    pub ttl_ms: u64,
    pub expected_net: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Venue {
    Polymarket,
    Coinbase,
    Kraken,
    Gemini,
    Sim,
}

impl Venue {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Polymarket => "polymarket",
            Self::Coinbase => "coinbase",
            Self::Kraken => "kraken",
            Self::Gemini => "gemini",
            Self::Sim => "sim",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReport {
    pub venue: Venue,
    pub order_id: String,
    pub market_id: Option<String>,
    pub status: ExecutionStatus,
    pub side: Side,
    pub filled_qty: f64,
    pub avg_px: f64,
    pub ts: DateTime<Utc>,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KillSwitchState {
    Running,
    AutoHalt,
    ManualHalt,
    SafeMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskDecision {
    pub allow: bool,
    pub reason_code: String,
    pub limit_name: Option<String>,
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RiskState {
    pub killswitch: String,
    pub daily_pnl: f64,
    pub max_daily_loss: f64,
    pub open_notional: f64,
    pub unhedged_delta: f64,
    pub open_markets: usize,
    pub stale_books: usize,
    pub last_update_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub market_id: String,
    pub token_id: String,
    pub qty: f64,
    pub avg_price: f64,
    pub asset: Asset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingViewBias {
    pub bias: f64,
    pub confidence: f64,
    pub source: String,
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketSelection {
    pub market_id: String,
    pub question: String,
    pub slug: String,
    pub token_id_yes: String,
    pub token_id_no: String,
    pub asset: Asset,
    pub bucket: TimeBucket,
    pub tier: MarketTier,
    pub fees_enabled: bool,
    pub spread: f64,
    pub liquidity: f64,
    pub volume24h: f64,
    pub tick_size: f64,
    pub min_order_size: f64,
    pub end_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionMode {
    MakerFirst,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryExitVector {
    pub entry_max_slippage_bps: f64,
    pub exit_max_slippage_bps: f64,
    pub entry_offset_bps: f64,
    pub exit_offset_bps: f64,
    pub max_cross_bps_unwind: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VenueFeeSchedule {
    pub maker_bps: f64,
    pub taker_bps: f64,
    pub rebate_bps_est: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPolicy {
    pub mode: ExecutionMode,
    pub allow_taker_on_unwind_only: bool,
    pub post_only: bool,
    pub cancel_replace_cooldown_ms: u64,
    pub min_rest_ms: u64,
    pub stale_book_ms: u64,
    pub vectors: EntryExitVector,
    pub coinbase_fees: VenueFeeSchedule,
    pub kraken_fees: VenueFeeSchedule,
    pub gemini_fees: VenueFeeSchedule,
    pub polymarket_fees: VenueFeeSchedule,
    pub edge_profiles: EdgeProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StrategyClass {
    MakerMmSpot,
    ConversionCycle,
    PositionReentry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeProfile {
    pub maker_mm_spot_min_bps: f64,
    pub conversion_cycle_min_bps: f64,
    pub position_reentry_min_bps: f64,
    #[serde(default)]
    pub per_asset_overrides_bps: HashMap<String, f64>,
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[serde(transparent)]
pub struct ProductId(pub String);

impl ProductId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.0.trim().is_empty()
    }
}

impl From<&str> for ProductId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for ProductId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OrderLifecycleState {
    Planned,
    Submitted,
    Resting,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionEvent {
    pub order_id: String,
    pub venue: Venue,
    pub market_id: Option<String>,
    pub product_id: Option<String>,
    pub side: Side,
    pub state: OrderLifecycleState,
    pub qty: f64,
    pub price: f64,
    pub ts: DateTime<Utc>,
    pub details: Option<String>,
    #[serde(default)]
    pub reason_code: Option<String>,
    #[serde(default)]
    pub unwind_flag: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletBalance {
    pub venue: Venue,
    pub account_id: String,
    pub asset: String,
    pub available: f64,
    pub hold: f64,
    pub usd_value: f64,
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationDrift {
    pub asset: String,
    pub current_weight: f64,
    pub target_weight: f64,
    pub drift_weight: f64,
    pub current_usd: f64,
    pub target_usd: f64,
    pub drift_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebalanceIntent {
    pub intent_id: String,
    pub product_id: String,
    pub asset: Asset,
    pub side: Side,
    pub usd_notional: f64,
    pub limit_price: f64,
    pub max_slippage_bps: f64,
pub enum Instrument {
    Spot,
    Derivative,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TradeAction {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OrderRoute {
    Maker,
    Taker,
    ScanOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RebalancePlanStatus {
    Planned,
    Approved,
    Rejected,
    Executed,
    Expired,
    Canceled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebalancePlan {
    pub plan_id: String,
    pub status: RebalancePlanStatus,
    pub intents: Vec<RebalanceIntent>,
    pub drifts: Vec<AllocationDrift>,
    pub total_drift_abs_usd: f64,
    pub created_ts: DateTime<Utc>,
    pub expires_ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalToken {
    pub token_id: String,
    pub plan_id: String,
    pub approved: bool,
    pub created_ts: DateTime<Utc>,
    pub expires_ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionCostAttribution {
    pub execution_id: String,
    pub venue: Venue,
    pub market_id: Option<String>,
    pub side: Side,
    pub qty: f64,
    pub avg_px: f64,
    pub reference_px: f64,
    pub fee_bps: f64,
    pub fee_est: f64,
    pub slippage_bps: f64,
    pub slippage_est: f64,
    pub rebate_bps_est: f64,
    pub rebate_est: f64,
    pub effective_edge: f64,
    pub ts: DateTime<Utc>,
    #[serde(default)]
    pub strategy_class: Option<StrategyClass>,
    #[serde(default)]
    pub route_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinbaseL2Update {
    pub sequence_num: i64,
    pub product_id: String,
    pub side: String,
    pub price_level: f64,
    pub new_quantity: f64,
    pub event_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CoinbaseOrderBookState {
    pub product_id: String,
    pub sequence_num: i64,
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
    pub last_event_ts: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOrderEvent {
    pub order_id: String,
    pub product_id: String,
    pub status: String,
    pub side: String,
    pub post_only: bool,
    pub avg_price: f64,
    pub filled_qty: f64,
    pub total_fees: f64,
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CoinbaseAuthSource {
    LegacyInline,
    CdpKeyFile,
    AwsSecretsManager,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedCoinbaseAuth {
    pub profile_id: Option<String>,
    pub key_name: String,
    pub key_id: String,
    pub private_key_pem: String,
    pub source: CoinbaseAuthSource,
    pub loaded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthReloadResult {
    pub ok: bool,
    pub profile_id: Option<String>,
    pub key_id_suffix: Option<String>,
    pub source: Option<CoinbaseAuthSource>,
    pub reason: String,
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderManagerDecision {
    pub action: String,
    pub reason: String,
    pub should_edit: bool,
    pub should_cancel_replace: bool,
    pub target_price: f64,
    pub target_size: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteLeg {
    pub product_id: String,
    pub side: Side,
    pub input_asset: String,
    pub output_asset: String,
    pub price: f64,
    pub size: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteOpportunity {
    pub route_id: String,
    pub legs: Vec<RouteLeg>,
    pub gross_edge_bps: f64,
    pub expected_net_bps: f64,
    pub expected_usd_profit: f64,
    pub capital_required_usd: f64,
    pub strategy_class: StrategyClass,
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteExecutionPlan {
    pub route_id: String,
    pub legs: Vec<RouteLeg>,
    pub approved: bool,
    pub reason: Option<String>,
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VenueCapability {
    pub venue: Venue,
    pub supports_post_only: bool,
    pub supports_amend: bool,
    pub supports_fix: bool,
    pub min_tick: f64,
    pub min_size: f64,
    pub fee_model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VenueLatencyStats {
    pub venue: Venue,
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
    pub samples: usize,
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VenueFillQualityStats {
    pub venue: Venue,
    pub fills: usize,
    pub rejects: usize,
    pub cancels: usize,
    pub reject_ratio: f64,
    pub avg_slippage_bps: f64,
    pub avg_effective_edge: f64,
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossVenueRouteOpportunity {
    pub route_id: String,
    pub venues: Vec<Venue>,
    pub legs: Vec<RouteLeg>,
    pub gross_edge_bps: f64,
    pub expected_net_bps: f64,
    pub expected_usd_profit: f64,
    pub capital_required_usd: f64,
    pub strategy_class: StrategyClass,
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletIntelSnapshot {
    pub source: String,
    pub subject: String,
    pub metric: String,
    pub value: f64,
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotPathBenchmarkReport {
    pub implementation: String,
    pub throughput_msgs_per_sec: f64,
    pub p50_ns: f64,
    pub p95_ns: f64,
    pub p99_ns: f64,
    pub cpu_pct: f64,
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayAcceptanceReport {
    pub artifact_path: String,
    pub passed: bool,
    pub fail_reasons: Vec<String>,
    pub total_reports: usize,
    pub reject_error_rate: f64,
    pub max_unhedged_delta: f64,
    pub killswitch: String,
    pub effective_fee_bps_avg: f64,
    pub created_ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UiMode {
    Basic,
    Advanced,
}

impl UiMode {
    pub fn from_str(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "advanced" => Self::Advanced,
            _ => Self::Basic,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Basic => "basic",
            Self::Advanced => "advanced",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapitalTierRule {
    pub min_equity_usd: f64,
    pub reserve_pct: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyReserveRecommendation {
    pub ts: DateTime<Utc>,
    pub equity_usd: f64,
    pub realized_pnl_usd: f64,
    pub reserve_pct: f64,
    pub reserve_recommendation_usd: f64,
    pub reinvest_recommendation_usd: f64,
    pub daily_contribution_usd: f64,
    pub projected_next_equity_usd: f64,
    pub tier_label: String,
    pub blocked_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapitalLedgerEntry {
    pub entry_id: String,
    pub portfolio_id: String,
    pub day_utc: String,
    pub ts: DateTime<Utc>,
    pub contribution_usd: f64,
    pub realized_pnl_usd: f64,
    pub reserve_transfer_usd: f64,
    pub reinvested_usd: f64,
    pub equity_before_usd: f64,
    pub equity_after_usd: f64,
    pub status: String,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EquitySessionState {
    Open,
    Closed,
    Halted,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityProductSnapshot {
    pub symbol: String,
    pub product_id: String,
    pub tradable: bool,
    pub session_state: EquitySessionState,
    pub min_order_size: f64,
    pub quote_increment: f64,
    pub ts: DateTime<Utc>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityPaperRun {
    pub run_id: String,
    pub symbol: String,
    pub bars: usize,
    pub trades: usize,
    pub net_pnl_usd: f64,
    pub max_drawdown_pct: f64,
    pub ts: DateTime<Utc>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownturnAnalysisSummary {
    pub ts: DateTime<Utc>,
    pub regime_window: String,
    pub bearish_score: f64,
    pub volatility_score: f64,
    pub drawdown_pct: f64,
    pub risk_off: bool,
    pub baseline_net_edge_bps: f64,
    pub candidate_net_edge_bps: f64,
    pub pass: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossVenueShadowSummary {
    pub ts: DateTime<Utc>,
    pub total_opportunities: usize,
    pub coinbase_only_count: usize,
    pub cross_venue_count: usize,
    pub venues_seen: Vec<String>,
    pub best_expected_net_bps: f64,
    pub best_route_id: Option<String>,
pub enum WorkstationOrderStatus {
    Draft,
    CancelRequested,
    Open,
    Filled,
    Canceled,
    Rejected,
    AutoCanceled,
    ScanOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TradingEligibility {
    pub product_id: ProductId,
    pub live_tradable: bool,
    pub scan_only: bool,
    pub eligible: bool,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketMicrostructureSnapshot {
    pub product_id: ProductId,
    pub instrument: Option<Instrument>,
    pub best_bid: f64,
    pub best_ask: f64,
    pub mid_price: f64,
    pub spread_bps: f64,
    pub imbalance: f64,
    pub tape_direction: f64,
    pub realized_volatility: f64,
    pub fill_rate_estimate: f64,
    pub one_way_persistence: u64,
    pub ts: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StrategyVector {
    pub product_id: ProductId,
    pub strategy_name: String,
    pub microstructure_score: f64,
    pub momentum_score: f64,
    pub volatility_score: f64,
    pub plugin_score: f64,
    pub composite_score: f64,
    pub action: Option<TradeAction>,
    pub priority_fill: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderIntent {
    pub product_id: ProductId,
    pub instrument: Option<Instrument>,
    pub side: Option<Side>,
    pub route: Option<OrderRoute>,
    pub limit_price: Option<f64>,
    pub base_size: f64,
    pub quote_notional: f64,
    pub post_only: bool,
    pub reduce_only: bool,
    pub priority_fill: bool,
    pub strategy_name: String,
    pub expected_net_bps: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderDecision {
    pub product_id: ProductId,
    pub action: Option<TradeAction>,
    pub allow: bool,
    pub reason: String,
    pub route: Option<OrderRoute>,
    pub expected_net_bps: f64,
    pub taker_fallback_allowed: bool,
    pub intent: Option<OrderIntent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LiveArmState {
    pub armed: bool,
    pub mode: Option<String>,
    pub reason: Option<String>,
    pub auto_disarm_reason: Option<String>,
    pub armed_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkstationProduct {
    pub product_id: ProductId,
    pub instrument: Option<Instrument>,
    pub base_currency: String,
    pub quote_currency: String,
    pub status: String,
    pub price: f64,
    pub volume_24h: f64,
    pub live_tradable: bool,
    pub scan_only: bool,
    pub trading_disabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScannerRow {
    pub product_id: ProductId,
    pub instrument: Option<Instrument>,
    pub live_tradable: bool,
    pub scan_only: bool,
    pub spread_bps: f64,
    pub imbalance: f64,
    pub tape_direction: f64,
    pub realized_volatility: f64,
    pub fill_rate_estimate: f64,
    pub active_strategy: String,
    pub score: f64,
    pub current_risk_eligibility: TradingEligibility,
    pub best_bid: f64,
    pub best_ask: f64,
    pub mid_price: f64,
    pub action: Option<TradeAction>,
    pub priority_fill: bool,
    pub one_way_persistence: u64,
    pub ts: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkstationOrder {
    pub order_id: String,
    pub client_order_id: Option<String>,
    pub product_id: ProductId,
    pub instrument: Option<Instrument>,
    pub side: Option<Side>,
    pub route: Option<OrderRoute>,
    pub status: Option<WorkstationOrderStatus>,
    pub live: bool,
    pub post_only: bool,
    pub limit_price: Option<f64>,
    pub base_size: f64,
    pub quote_notional: f64,
    pub expected_net_bps: f64,
    pub reason: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductStrategyConfigView {
    pub product_id: ProductId,
    pub strategy_name: String,
    pub enabled: bool,
    pub live_enabled: bool,
    pub score_threshold: f64,
    pub quote_size_usd: f64,
    pub plugin_signal: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StrategyLabImportSummary {
    pub import_id: String,
    pub path: String,
    pub imported_at: Option<DateTime<Utc>>,
    pub markets: Vec<String>,
    pub best_variants: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductDetailView {
    pub product: WorkstationProduct,
    pub microstructure: MarketMicrostructureSnapshot,
    pub strategy: StrategyVector,
    pub eligibility: TradingEligibility,
    pub orders: Vec<WorkstationOrder>,
    pub imports: Vec<StrategyLabImportSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ListingLifecycleStage {
    #[default]
    Research,
    Monitoring,
    TransferOnly,
    Auction,
    FullTrading,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderInsight {
    pub provider: String,
    pub category: String,
    pub summary: String,
    pub freshness_label: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListingVenueRoute {
    pub venue: String,
    pub route_type: String,
    pub readiness: String,
    pub tradable: bool,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListingRadarRow {
    pub product_id: ProductId,
    pub asset_symbol: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub stage: ListingLifecycleStage,
    pub headline: String,
    pub composite_score: f64,
    pub liquidity_score: f64,
    pub sentiment_score: f64,
    pub unlock_risk_score: f64,
    pub route_ready: bool,
    pub live_tradable: bool,
    pub scan_only: bool,
    pub priority_fill: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListingRadarDetailView {
    pub product: WorkstationProduct,
    pub stage: ListingLifecycleStage,
    pub headline: String,
    pub summary: String,
    pub composite_score: f64,
    pub liquidity_score: f64,
    pub sentiment_score: f64,
    pub unlock_risk_score: f64,
    pub route_ready: bool,
    pub priority_fill: bool,
    pub catalysts: Vec<String>,
    pub insights: Vec<ProviderInsight>,
    pub routes: Vec<ListingVenueRoute>,
    pub eligibility: TradingEligibility,
    pub imports: Vec<StrategyLabImportSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RiskOverviewView {
    pub killswitch: String,
    pub daily_pnl: f64,
    pub open_notional: f64,
    pub unhedged_delta: f64,
    pub blocked_markets: usize,
    pub live_eligible_markets: usize,
    pub queued_notional: f64,
    pub live_orders: usize,
    pub taker_orders: usize,
    pub policy_breaches: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentApprovalItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub severity: String,
    pub status: String,
    pub product_id: Option<ProductId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentConsoleView {
    pub autonomy_tier: String,
    pub live_arm: LiveArmState,
    pub next_action: String,
    pub blocked_markets: usize,
    pub imports_loaded: usize,
    pub recommended_products: Vec<ProductId>,
    pub approvals: Vec<AgentApprovalItem>,
}
