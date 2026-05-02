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

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Btc => "BTC",
            Self::Eth => "ETH",
            Self::Sol => "SOL",
            Self::Xrp => "XRP",
            Self::Other => "OTHER",
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
    Sim,
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

fn default_strategy_import_promotion_status() -> String {
    "imported_only".to_string()
}

/// Summary of a strategy-lab JSON file imported into the Coinbase paper workstation.
///
/// `import_id` is assigned at import time. `artifact_id` (when present) is the stable
/// trace handle operators should correlate with replay and paper evidence; it may match
/// `import_id` or come from the lab JSON (`artifact_id` / `meta.artifact_id`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyLabImportSummary {
    pub import_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<String>,
    pub path: String,
    pub imported_at: Option<DateTime<Utc>>,
    pub markets: Vec<String>,
    pub best_variants: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_run_id: Option<String>,
    #[serde(default = "default_strategy_import_promotion_status")]
    pub promotion_status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replay_acceptance_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objective_score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeframe: Option<String>,
}

impl Default for StrategyLabImportSummary {
    fn default() -> Self {
        Self {
            import_id: String::new(),
            artifact_id: None,
            path: String::new(),
            imported_at: None,
            markets: Vec::new(),
            best_variants: Vec::new(),
            source_run_id: None,
            promotion_status: default_strategy_import_promotion_status(),
            replay_acceptance_status: None,
            objective_score: None,
            confidence: None,
            timeframe: None,
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StrategyClass {
    MakerMmSpot,
    ConversionCycle,
    PositionReentry,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EdgeProfile {
    pub maker_mm_spot_min_bps: f64,
    pub conversion_cycle_min_bps: f64,
    pub position_reentry_min_bps: f64,
    #[serde(default)]
    pub per_asset_overrides_bps: HashMap<String, f64>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderManagerDecision {
    pub action: String,
    pub reason: String,
    pub should_edit: bool,
    pub should_cancel_replace: bool,
    pub target_price: f64,
    pub target_size: f64,
}
