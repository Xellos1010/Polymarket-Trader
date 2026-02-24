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
        let mut cfg =
            toml::from_str::<AppConfig>(&raw).map_err(|e| PtError::Config(e.to_string()))?;
        cfg.apply_env_overrides();
        cfg.validate()?;
        Ok(cfg)
    }

    fn apply_env_overrides(&mut self) {
        if let Some(v) = env_nonempty("POLYMARKET_PRIVATE_KEY") {
            self.venues.polymarket.private_key = Some(v);
        }
        if let Some(v) = env_nonempty("COINBASE_API_KEY") {
            self.venues.coinbase.api_key = Some(v);
        }
        if let Some(v) = env_nonempty("COINBASE_API_SECRET") {
            self.venues.coinbase.api_secret = Some(v);
        }
        if let Some(v) = env_nonempty("COINBASE_PASSPHRASE") {
            self.venues.coinbase.passphrase = Some(v);
        }
        if let Some(v) = env_nonempty("TRADINGVIEW_ENDPOINT_SECRET") {
            self.signals.tradingview.endpoint_secret = Some(v);
        }
    }

    pub fn validate(&self) -> PtResult<()> {
        if self.engine.loop_ms < 50 {
            return Err(PtError::Config(
                "engine.loop_ms must be >= 50ms".to_string(),
            ));
        }

        if self.venues.polymarket.chain_id == 0 {
            return Err(PtError::Config(
                "venues.polymarket.chain_id must be > 0".to_string(),
            ));
        }

        if !is_http_url(&self.venues.polymarket.gamma_api) {
            return Err(PtError::Config(
                "venues.polymarket.gamma_api must be an http(s) URL".to_string(),
            ));
        }
        if !is_http_url(&self.venues.polymarket.data_api) {
            return Err(PtError::Config(
                "venues.polymarket.data_api must be an http(s) URL".to_string(),
            ));
        }
        if !is_http_url(&self.venues.polymarket.clob_api) {
            return Err(PtError::Config(
                "venues.polymarket.clob_api must be an http(s) URL".to_string(),
            ));
        }
        if !is_websocket_url(&self.venues.polymarket.clob_ws) {
            return Err(PtError::Config(
                "venues.polymarket.clob_ws must be a ws(s) URL".to_string(),
            ));
        }
        if !is_http_url(&self.venues.coinbase.api_base) {
            return Err(PtError::Config(
                "venues.coinbase.api_base must be an http(s) URL".to_string(),
            ));
        }

        if self.venues.polymarket.filters.max_spread <= 0.0
            || self.venues.polymarket.filters.max_spread > 1.0
        {
            return Err(PtError::Config(
                "venues.polymarket.filters.max_spread must be in (0,1]".to_string(),
            ));
        }

        if self.venues.polymarket.filters.min_liquidity < 0.0 {
            return Err(PtError::Config(
                "venues.polymarket.filters.min_liquidity must be >= 0".to_string(),
            ));
        }
        if self.venues.polymarket.filters.min_volume24h < 0.0 {
            return Err(PtError::Config(
                "venues.polymarket.filters.min_volume24h must be >= 0".to_string(),
            ));
        }
        if self.venues.polymarket.filters.allowed_slugs.is_empty() {
            return Err(PtError::Config(
                "venues.polymarket.filters.allowed_slugs must not be empty".to_string(),
            ));
        }
        if self.venues.polymarket.filters.assets.is_empty() {
            return Err(PtError::Config(
                "venues.polymarket.filters.assets must not be empty".to_string(),
            ));
        }

        if self.risk.daily_loss_limit_pct <= 0.0 {
            return Err(PtError::Config(
                "risk.daily_loss_limit_pct must be > 0".to_string(),
            ));
        }
        if self.risk.max_notional_per_market <= 0.0 {
            return Err(PtError::Config(
                "risk.max_notional_per_market must be > 0".to_string(),
            ));
        }
        if self.risk.max_total_open_notional <= 0.0 {
            return Err(PtError::Config(
                "risk.max_total_open_notional must be > 0".to_string(),
            ));
        }
        if self.risk.max_markets_quoted_simultaneously == 0 {
            return Err(PtError::Config(
                "risk.max_markets_quoted_simultaneously must be > 0".to_string(),
            ));
        }
        if self.risk.max_unhedged_delta <= 0.0 {
            return Err(PtError::Config(
                "risk.max_unhedged_delta must be > 0".to_string(),
            ));
        }
        if self.risk.max_order_age_secs == 0 {
            return Err(PtError::Config(
                "risk.max_order_age_secs must be > 0".to_string(),
            ));
        }
        if self.risk.stale_book_threshold_ms == 0 {
            return Err(PtError::Config(
                "risk.stale_book_threshold_ms must be > 0".to_string(),
            ));
        }
        if self.risk.min_expected_net < 0.0 {
            return Err(PtError::Config(
                "risk.min_expected_net must be >= 0".to_string(),
            ));
        }

        if self.signals.tradingview.k_wallet.is_nan() || self.signals.tradingview.k_tv.is_nan() {
            return Err(PtError::Config(
                "signals.tradingview weights must be finite".to_string(),
            ));
        }
        if self.signals.tradingview.k_wallet.abs() + self.signals.tradingview.k_tv.abs() <= 0.0 {
            return Err(PtError::Config(
                "signals.tradingview weights must not both be zero".to_string(),
            ));
        }

        if self.ops.dashboard_bind.trim().is_empty() {
            return Err(PtError::Config(
                "ops.dashboard_bind must not be empty".to_string(),
            ));
        }
        if self.ops.market_refresh_secs == 0 {
            return Err(PtError::Config(
                "ops.market_refresh_secs must be > 0".to_string(),
            ));
        }
        if self.ops.wallet_refresh_secs == 0 {
            return Err(PtError::Config(
                "ops.wallet_refresh_secs must be > 0".to_string(),
            ));
        }
        if self.ops.risk_watchdog_ms == 0 {
            return Err(PtError::Config(
                "ops.risk_watchdog_ms must be > 0".to_string(),
            ));
        }

        if matches!(self.engine.mode, EngineMode::Live) {
            if is_empty_opt(self.venues.polymarket.private_key.as_deref()) {
                return Err(PtError::Config(
                    "engine.mode=live requires venues.polymarket.private_key".to_string(),
                ));
            }
            if is_empty_opt(self.venues.coinbase.api_key.as_deref()) {
                return Err(PtError::Config(
                    "engine.mode=live requires venues.coinbase.api_key".to_string(),
                ));
            }
            if is_empty_opt(self.venues.coinbase.api_secret.as_deref()) {
                return Err(PtError::Config(
                    "engine.mode=live requires venues.coinbase.api_secret".to_string(),
                ));
            }
        }

        Ok(())
    }
}

fn is_http_url(v: &str) -> bool {
    v.starts_with("http://") || v.starts_with("https://")
}

fn is_websocket_url(v: &str) -> bool {
    v.starts_with("ws://") || v.starts_with("wss://")
}

fn is_empty_opt(v: Option<&str>) -> bool {
    v.map(|s| s.trim().is_empty()).unwrap_or(true)
}

fn env_nonempty(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .and_then(|v| if v.trim().is_empty() { None } else { Some(v) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_example_is_valid() {
        let raw = include_str!("../../../config/config.example.toml");
        let cfg = toml::from_str::<AppConfig>(raw).expect("parse config example");
        cfg.validate().expect("validate config example");
    }

    #[test]
    fn live_mode_requires_credentials() {
        let raw = include_str!("../../../config/config.example.toml");
        let mut cfg = toml::from_str::<AppConfig>(raw).expect("parse config example");
        cfg.engine.mode = EngineMode::Live;
        cfg.venues.polymarket.private_key = Some(String::new());
        cfg.venues.coinbase.api_key = Some(String::new());
        cfg.venues.coinbase.api_secret = Some(String::new());
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn env_overrides_apply_for_secrets() {
        let raw = include_str!("../../../config/config.example.toml");
        let mut cfg = toml::from_str::<AppConfig>(raw).expect("parse config example");

        std::env::set_var("POLYMARKET_PRIVATE_KEY", "poly_key");
        std::env::set_var("COINBASE_API_KEY", "cb_key");
        std::env::set_var("COINBASE_API_SECRET", "cb_secret");
        std::env::set_var("COINBASE_PASSPHRASE", "cb_pass");
        std::env::set_var("TRADINGVIEW_ENDPOINT_SECRET", "tv_secret");

        cfg.apply_env_overrides();

        assert_eq!(
            cfg.venues.polymarket.private_key.as_deref(),
            Some("poly_key")
        );
        assert_eq!(cfg.venues.coinbase.api_key.as_deref(), Some("cb_key"));
        assert_eq!(cfg.venues.coinbase.api_secret.as_deref(), Some("cb_secret"));
        assert_eq!(cfg.venues.coinbase.passphrase.as_deref(), Some("cb_pass"));
        assert_eq!(
            cfg.signals.tradingview.endpoint_secret.as_deref(),
            Some("tv_secret")
        );

        std::env::remove_var("POLYMARKET_PRIVATE_KEY");
        std::env::remove_var("COINBASE_API_KEY");
        std::env::remove_var("COINBASE_API_SECRET");
        std::env::remove_var("COINBASE_PASSPHRASE");
        std::env::remove_var("TRADINGVIEW_ENDPOINT_SECRET");
    }
}
