use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
