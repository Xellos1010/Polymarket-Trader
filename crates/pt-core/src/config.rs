use crate::{PtError, PtResult};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EngineMode {
    Replay,
    Paper,
    Live,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    pub mode: EngineMode,
    pub loop_ms: u64,
    pub replay_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolymarketFilterConfig {
    pub tag_id: u64,
    pub max_spread: f64,
    pub min_liquidity: f64,
    pub min_volume24h: f64,
    pub require_fee_enabled: bool,
    pub require_orderbook: bool,
    pub allowed_slugs: Vec<String>,
    pub assets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolymarketConfig {
    pub gamma_api: String,
    pub data_api: String,
    pub clob_api: String,
    pub clob_ws: String,
    pub private_key: Option<String>,
    pub use_server_time: Option<bool>,
    pub chain_id: u64,
    pub filters: PolymarketFilterConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinbaseConfig {
    pub api_base: String,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub passphrase: Option<String>,
    pub products: Vec<String>,
    pub hedge_threshold_usd: f64,
    pub hedge_max_slippage_bps: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VenuesConfig {
    pub polymarket: PolymarketConfig,
    pub coinbase: CoinbaseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletSignalConfig {
    pub refresh_secs: u64,
    pub top_n: usize,
    pub allowlist_path: String,
    pub enforce_allowlist: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingViewSignalConfig {
    pub enabled: bool,
    pub bind_addr: String,
    pub endpoint_secret: Option<String>,
    pub k_wallet: f64,
    pub k_tv: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalsConfig {
    pub wallet: WalletSignalConfig,
    pub tradingview: TradingViewSignalConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskConfig {
    pub daily_loss_limit_pct: f64,
    pub max_notional_per_market: f64,
    pub max_total_open_notional: f64,
    pub max_markets_quoted_simultaneously: usize,
    pub max_unhedged_delta: f64,
    pub max_order_age_secs: u64,
    pub stale_book_threshold_ms: u64,
    pub min_expected_net: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub sqlite_path: String,
    pub parquet_dir: String,
    pub snapshot_roll_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpsConfig {
    pub log_level: String,
    pub dashboard_bind: String,
    pub metrics_flush_secs: u64,
    pub market_refresh_secs: u64,
    pub wallet_refresh_secs: u64,
    pub risk_watchdog_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub engine: EngineConfig,
    pub venues: VenuesConfig,
    pub signals: SignalsConfig,
    pub risk: RiskConfig,
    pub storage: StorageConfig,
    pub ops: OpsConfig,
}

impl AppConfig {
    pub fn from_file(path: impl AsRef<Path>) -> PtResult<Self> {
        let path = path.as_ref();
        let raw = fs::read_to_string(path).map_err(|e| PtError::Io(e.to_string()))?;
        let cfg = toml::from_str::<AppConfig>(&raw).map_err(|e| PtError::Config(e.to_string()))?;
        Ok(cfg)
    }
}
