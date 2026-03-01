use crate::{PtError, PtResult};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

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
    #[serde(default)]
    pub portfolio_id: Option<String>,
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
    #[serde(default)]
    pub auth: CoinbaseAuthConfig,
    pub products: Vec<String>,
    pub hedge_threshold_usd: f64,
    pub hedge_max_slippage_bps: f64,
    #[serde(default)]
    pub ws: CoinbaseWsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericVenueConfig {
    pub enabled: bool,
    pub api_base: String,
    pub ws_url: String,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub passphrase: Option<String>,
    pub products: Vec<String>,
}

impl Default for GenericVenueConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            api_base: String::new(),
            ws_url: String::new(),
            api_key: None,
            api_secret: None,
            passphrase: None,
            products: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CoinbaseAuthProfileConfig {
    #[serde(default)]
    pub cdp_key_file: Option<String>,
    #[serde(default)]
    pub cdp_secret_id: Option<String>,
    #[serde(default)]
    pub expected_key_id: Option<String>,
    #[serde(default)]
    pub strategy_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinbaseAuthConfig {
    #[serde(default)]
    pub active_profile: Option<String>,
    #[serde(default)]
    pub allow_hot_reload: bool,
    #[serde(default)]
    pub strict_live_auth: bool,
    #[serde(default)]
    pub profiles: HashMap<String, CoinbaseAuthProfileConfig>,
}

impl Default for CoinbaseAuthConfig {
    fn default() -> Self {
        Self {
            active_profile: None,
            allow_hot_reload: true,
            strict_live_auth: true,
            profiles: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinbaseWsConfig {
    pub url: String,
    pub channels: Vec<String>,
    pub heartbeat_timeout_ms: u64,
    pub resync_on_gap: bool,
}

impl Default for CoinbaseWsConfig {
    fn default() -> Self {
        Self {
            url: "wss://advanced-trade-ws.coinbase.com".to_string(),
            channels: vec![
                "heartbeats".to_string(),
                "level2".to_string(),
                "user".to_string(),
            ],
            heartbeat_timeout_ms: 8_000,
            resync_on_gap: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VenuesConfig {
    pub polymarket: PolymarketConfig,
    pub coinbase: CoinbaseConfig,
    #[serde(default)]
    pub kraken: GenericVenueConfig,
    #[serde(default)]
    pub gemini: GenericVenueConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeAffinityConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub feed_cores: Vec<usize>,
    #[serde(default)]
    pub strategy_cores: Vec<usize>,
    #[serde(default)]
    pub execution_cores: Vec<usize>,
    #[serde(default)]
    pub persistence_cores: Vec<usize>,
}

impl Default for RuntimeAffinityConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            feed_cores: vec![0, 1],
            strategy_cores: vec![2, 3],
            execution_cores: vec![4, 5],
            persistence_cores: vec![6, 7],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeJitterControlsConfig {
    #[serde(default)]
    pub enabled: bool,
    pub nic_polling: bool,
    pub disable_cpu_powersave: bool,
    pub isolate_cores: bool,
    pub rcu_nocbs: bool,
    pub nohz_full: bool,
}

impl Default for RuntimeJitterControlsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            nic_polling: true,
            disable_cpu_powersave: true,
            isolate_cores: true,
            rcu_nocbs: true,
            nohz_full: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuntimeConfig {
    #[serde(default)]
    pub affinity: RuntimeAffinityConfig,
    #[serde(default)]
    pub jitter_controls: RuntimeJitterControlsConfig,
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
    #[serde(default)]
    pub pilot: RiskPilotConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskPilotConfig {
    pub enforce_ultra_tight_live: bool,
    pub ultra_tight_daily_loss_limit_pct: f64,
    pub ultra_tight_max_notional_per_market: f64,
    pub ultra_tight_max_total_open_notional: f64,
    pub ultra_tight_max_markets_quoted: usize,
}

impl Default for RiskPilotConfig {
    fn default() -> Self {
        Self {
            enforce_ultra_tight_live: true,
            ultra_tight_daily_loss_limit_pct: 0.01,
            ultra_tight_max_notional_per_market: 2.5,
            ultra_tight_max_total_open_notional: 10.0,
            ultra_tight_max_markets_quoted: 1,
        }
    }
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
#[serde(rename_all = "snake_case")]
pub enum ExecutionModeConfig {
    MakerFirst,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionVectorsConfig {
    pub entry_max_slippage_bps: f64,
    pub exit_max_slippage_bps: f64,
    pub entry_offset_bps: f64,
    pub exit_offset_bps: f64,
    pub max_cross_bps_unwind: f64,
}

impl Default for ExecutionVectorsConfig {
    fn default() -> Self {
        Self {
            entry_max_slippage_bps: 8.0,
            exit_max_slippage_bps: 10.0,
            entry_offset_bps: 2.0,
            exit_offset_bps: 2.0,
            max_cross_bps_unwind: 20.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionFeeVenueConfig {
    pub maker_bps: f64,
    pub taker_bps: f64,
    pub rebate_bps_est: f64,
}

impl Default for ExecutionFeeVenueConfig {
    fn default() -> Self {
        Self {
            maker_bps: 0.0,
            taker_bps: 10.0,
            rebate_bps_est: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionFeesConfig {
    pub coinbase: ExecutionFeeVenueConfig,
    pub polymarket: ExecutionFeeVenueConfig,
}

impl Default for ExecutionFeesConfig {
    fn default() -> Self {
        Self {
            coinbase: ExecutionFeeVenueConfig {
                maker_bps: 6.0,
                taker_bps: 12.0,
                rebate_bps_est: 0.0,
            },
            polymarket: ExecutionFeeVenueConfig {
                maker_bps: 0.0,
                taker_bps: 5.0,
                rebate_bps_est: 1.0,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionConfig {
    pub mode: ExecutionModeConfig,
    pub allow_taker_on_unwind_only: bool,
    pub post_only: bool,
    pub cancel_replace_cooldown_ms: u64,
    pub min_rest_ms: u64,
    pub stale_book_ms: u64,
    pub vectors: ExecutionVectorsConfig,
    pub fees: ExecutionFeesConfig,
    #[serde(default)]
    pub edge_profiles: ExecutionEdgeProfilesConfig,
    #[serde(default)]
    pub order_manager: ExecutionOrderManagerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionEdgeProfilesConfig {
    pub maker_mm_spot_min_bps: f64,
    pub conversion_cycle_min_bps: f64,
    pub position_reentry_min_bps: f64,
    #[serde(default)]
    pub per_asset_overrides_bps: HashMap<String, f64>,
}

impl Default for ExecutionEdgeProfilesConfig {
    fn default() -> Self {
        Self {
            maker_mm_spot_min_bps: 8.0,
            conversion_cycle_min_bps: 100.0,
            position_reentry_min_bps: 40.0,
            per_asset_overrides_bps: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionOrderManagerConfig {
    pub preview_required: bool,
    pub max_reprice_attempts: u32,
    pub edit_vs_replace_threshold_bps: f64,
    pub cancel_replace_cooldown_ms: u64,
    pub min_rest_ms: u64,
}

impl Default for ExecutionOrderManagerConfig {
    fn default() -> Self {
        Self {
            preview_required: true,
            max_reprice_attempts: 3,
            edit_vs_replace_threshold_bps: 5.0,
            cancel_replace_cooldown_ms: 250,
            min_rest_ms: 400,
        }
    }
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            mode: ExecutionModeConfig::MakerFirst,
            allow_taker_on_unwind_only: true,
            post_only: true,
            cancel_replace_cooldown_ms: 250,
            min_rest_ms: 400,
            stale_book_ms: 400,
            vectors: ExecutionVectorsConfig::default(),
            fees: ExecutionFeesConfig::default(),
            edge_profiles: ExecutionEdgeProfilesConfig::default(),
            order_manager: ExecutionOrderManagerConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WalletModeConfig {
    Monitor,
    Assist,
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTargetsConfig {
    pub btc: f64,
    pub eth: f64,
    pub sol: f64,
    pub xrp: f64,
    pub usd: f64,
}

impl Default for WalletTargetsConfig {
    fn default() -> Self {
        Self {
            btc: 0.25,
            eth: 0.20,
            sol: 0.10,
            xrp: 0.05,
            usd: 0.40,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletRebalanceConfig {
    pub drift_trigger_pct: f64,
    pub min_order_usd: f64,
    pub max_order_usd: f64,
    pub max_slippage_bps: f64,
    pub max_orders_per_cycle: usize,
}

impl Default for WalletRebalanceConfig {
    fn default() -> Self {
        Self {
            drift_trigger_pct: 0.03,
            min_order_usd: 5.0,
            max_order_usd: 25.0,
            max_slippage_bps: 10.0,
            max_orders_per_cycle: 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletApprovalConfig {
    pub required: bool,
    pub token_ttl_secs: u64,
}

impl Default for WalletApprovalConfig {
    fn default() -> Self {
        Self {
            required: true,
            token_ttl_secs: 300,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConfig {
    pub enabled: bool,
    pub mode: WalletModeConfig,
    pub sync_secs: u64,
    pub targets: WalletTargetsConfig,
    pub rebalance: WalletRebalanceConfig,
    pub approval: WalletApprovalConfig,
}

impl Default for WalletConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: WalletModeConfig::Assist,
            sync_secs: 10,
            targets: WalletTargetsConfig::default(),
            rebalance: WalletRebalanceConfig::default(),
            approval: WalletApprovalConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptanceReplayConfig {
    pub max_reject_error_rate: f64,
    pub max_effective_fee_bps_avg: f64,
    pub max_unhedged_delta: f64,
    pub require_killswitch_running: bool,
}

impl Default for AcceptanceReplayConfig {
    fn default() -> Self {
        Self {
            max_reject_error_rate: 0.05,
            max_effective_fee_bps_avg: 25.0,
            max_unhedged_delta: 10.0,
            require_killswitch_running: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AcceptanceConfig {
    pub replay: AcceptanceReplayConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletIntelConfig {
    pub enabled: bool,
    pub top_wallets: usize,
    pub refresh_secs: u64,
    pub include_coinbase: bool,
    pub include_polymarket: bool,
}

impl Default for WalletIntelConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            top_wallets: 50,
            refresh_secs: 60,
            include_coinbase: true,
            include_polymarket: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotpathBenchmarkConfig {
    pub enabled: bool,
    pub sample_size: usize,
    pub warmup_iters: usize,
}

impl Default for HotpathBenchmarkConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            sample_size: 10_000,
            warmup_iters: 1_000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BenchmarkConfig {
    #[serde(default)]
    pub hotpath: HotpathBenchmarkConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub engine: EngineConfig,
    pub venues: VenuesConfig,
    pub signals: SignalsConfig,
    pub risk: RiskConfig,
    pub storage: StorageConfig,
    pub ops: OpsConfig,
    #[serde(default)]
    pub execution: ExecutionConfig,
    #[serde(default)]
    pub wallet: WalletConfig,
    #[serde(default)]
    pub acceptance: AcceptanceConfig,
    #[serde(default)]
    pub runtime: RuntimeConfig,
    #[serde(default)]
    pub wallet_intel: WalletIntelConfig,
    #[serde(default)]
    pub benchmark: BenchmarkConfig,
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
        if let Some(v) = env_nonempty("COINBASE_AUTH_PROFILE") {
            self.venues.coinbase.auth.active_profile = Some(v);
        }
        if let Some(v) = env_nonempty("COINBASE_CDP_KEY_FILE") {
            self.coinbase_env_profile_mut().cdp_key_file = Some(v);
        }
        if let Some(v) = env_nonempty("COINBASE_CDP_SECRET_ID") {
            self.coinbase_env_profile_mut().cdp_secret_id = Some(v);
        }
        if let Some(v) = env_nonempty("COINBASE_EXPECTED_KEY_ID") {
            self.coinbase_env_profile_mut().expected_key_id = Some(v);
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
        if let Some(portfolio_id) = self.engine.portfolio_id.as_deref() {
            if portfolio_id.trim().is_empty() {
                return Err(PtError::Config(
                    "engine.portfolio_id must not be empty when set".to_string(),
                ));
            }
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
        if !is_websocket_url(&self.venues.coinbase.ws.url) {
            return Err(PtError::Config(
                "venues.coinbase.ws.url must be a ws(s) URL".to_string(),
            ));
        }
        if self.venues.coinbase.ws.channels.is_empty() {
            return Err(PtError::Config(
                "venues.coinbase.ws.channels must not be empty".to_string(),
            ));
        }
        if self.venues.coinbase.ws.heartbeat_timeout_ms == 0 {
            return Err(PtError::Config(
                "venues.coinbase.ws.heartbeat_timeout_ms must be > 0".to_string(),
            ));
        }
        if self.venues.kraken.enabled {
            if !is_http_url(&self.venues.kraken.api_base) {
                return Err(PtError::Config(
                    "venues.kraken.api_base must be an http(s) URL when enabled".to_string(),
                ));
            }
            if !is_websocket_url(&self.venues.kraken.ws_url) {
                return Err(PtError::Config(
                    "venues.kraken.ws_url must be a ws(s) URL when enabled".to_string(),
                ));
            }
        }
        if self.venues.gemini.enabled {
            if !is_http_url(&self.venues.gemini.api_base) {
                return Err(PtError::Config(
                    "venues.gemini.api_base must be an http(s) URL when enabled".to_string(),
                ));
            }
            if !is_websocket_url(&self.venues.gemini.ws_url) {
                return Err(PtError::Config(
                    "venues.gemini.ws_url must be a ws(s) URL when enabled".to_string(),
                ));
            }
        }
        if let Some(active_profile) = self.venues.coinbase.auth.active_profile.as_deref() {
            if active_profile.trim().is_empty() {
                return Err(PtError::Config(
                    "venues.coinbase.auth.active_profile must not be empty".to_string(),
                ));
            }
            if !self
                .venues
                .coinbase
                .auth
                .profiles
                .contains_key(active_profile.trim())
            {
                return Err(PtError::Config(format!(
                    "venues.coinbase.auth.active_profile '{}' missing in venues.coinbase.auth.profiles",
                    active_profile
                )));
            }
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
        if self.runtime.affinity.enabled {
            if self.runtime.affinity.feed_cores.is_empty()
                || self.runtime.affinity.strategy_cores.is_empty()
                || self.runtime.affinity.execution_cores.is_empty()
            {
                return Err(PtError::Config(
                    "runtime.affinity enabled requires non-empty feed_cores/strategy_cores/execution_cores"
                        .to_string(),
                ));
            }
        }
        if self.wallet_intel.refresh_secs == 0 {
            return Err(PtError::Config(
                "wallet_intel.refresh_secs must be > 0".to_string(),
            ));
        }
        if self.wallet_intel.top_wallets == 0 {
            return Err(PtError::Config(
                "wallet_intel.top_wallets must be > 0".to_string(),
            ));
        }
        if self.benchmark.hotpath.sample_size == 0 {
            return Err(PtError::Config(
                "benchmark.hotpath.sample_size must be > 0".to_string(),
            ));
        }

        if self.execution.cancel_replace_cooldown_ms == 0 {
            return Err(PtError::Config(
                "execution.cancel_replace_cooldown_ms must be > 0".to_string(),
            ));
        }
        if self.execution.min_rest_ms == 0 {
            return Err(PtError::Config(
                "execution.min_rest_ms must be > 0".to_string(),
            ));
        }
        if self.execution.stale_book_ms == 0 {
            return Err(PtError::Config(
                "execution.stale_book_ms must be > 0".to_string(),
            ));
        }
        if self.execution.vectors.entry_max_slippage_bps <= 0.0
            || self.execution.vectors.exit_max_slippage_bps <= 0.0
        {
            return Err(PtError::Config(
                "execution vectors max slippage bps must be > 0".to_string(),
            ));
        }
        if self.execution.vectors.max_cross_bps_unwind <= 0.0 {
            return Err(PtError::Config(
                "execution.vectors.max_cross_bps_unwind must be > 0".to_string(),
            ));
        }
        if self.execution.order_manager.max_reprice_attempts == 0 {
            return Err(PtError::Config(
                "execution.order_manager.max_reprice_attempts must be > 0".to_string(),
            ));
        }
        if self.execution.order_manager.edit_vs_replace_threshold_bps < 0.0 {
            return Err(PtError::Config(
                "execution.order_manager.edit_vs_replace_threshold_bps must be >= 0".to_string(),
            ));
        }
        if self.execution.order_manager.cancel_replace_cooldown_ms == 0 {
            return Err(PtError::Config(
                "execution.order_manager.cancel_replace_cooldown_ms must be > 0".to_string(),
            ));
        }
        if self.execution.order_manager.min_rest_ms == 0 {
            return Err(PtError::Config(
                "execution.order_manager.min_rest_ms must be > 0".to_string(),
            ));
        }
        if self.execution.edge_profiles.maker_mm_spot_min_bps < 0.0
            || self.execution.edge_profiles.conversion_cycle_min_bps < 0.0
            || self.execution.edge_profiles.position_reentry_min_bps < 0.0
        {
            return Err(PtError::Config(
                "execution.edge_profiles values must be >= 0".to_string(),
            ));
        }

        if self.wallet.sync_secs == 0 {
            return Err(PtError::Config("wallet.sync_secs must be > 0".to_string()));
        }
        if self.wallet.rebalance.min_order_usd <= 0.0 {
            return Err(PtError::Config(
                "wallet.rebalance.min_order_usd must be > 0".to_string(),
            ));
        }
        if self.wallet.rebalance.max_order_usd < self.wallet.rebalance.min_order_usd {
            return Err(PtError::Config(
                "wallet.rebalance.max_order_usd must be >= min_order_usd".to_string(),
            ));
        }
        if self.wallet.rebalance.max_orders_per_cycle == 0 {
            return Err(PtError::Config(
                "wallet.rebalance.max_orders_per_cycle must be > 0".to_string(),
            ));
        }
        if self.wallet.approval.token_ttl_secs == 0 {
            return Err(PtError::Config(
                "wallet.approval.token_ttl_secs must be > 0".to_string(),
            ));
        }

        let targets_sum = self.wallet.targets.btc
            + self.wallet.targets.eth
            + self.wallet.targets.sol
            + self.wallet.targets.xrp
            + self.wallet.targets.usd;
        if !(targets_sum > 0.0 && targets_sum <= 1.0001) {
            return Err(PtError::Config(
                "wallet.targets must sum to a positive value <= 1.0".to_string(),
            ));
        }

        if self.acceptance.replay.max_reject_error_rate < 0.0
            || self.acceptance.replay.max_reject_error_rate > 1.0
        {
            return Err(PtError::Config(
                "acceptance.replay.max_reject_error_rate must be in [0,1]".to_string(),
            ));
        }
        if self.acceptance.replay.max_effective_fee_bps_avg < 0.0 {
            return Err(PtError::Config(
                "acceptance.replay.max_effective_fee_bps_avg must be >= 0".to_string(),
            ));
        }
        if self.acceptance.replay.max_unhedged_delta <= 0.0 {
            return Err(PtError::Config(
                "acceptance.replay.max_unhedged_delta must be > 0".to_string(),
            ));
        }

        if self.risk.pilot.ultra_tight_daily_loss_limit_pct <= 0.0
            || self.risk.pilot.ultra_tight_max_notional_per_market <= 0.0
            || self.risk.pilot.ultra_tight_max_total_open_notional <= 0.0
            || self.risk.pilot.ultra_tight_max_markets_quoted == 0
        {
            return Err(PtError::Config(
                "risk.pilot ultra-tight limits must all be positive".to_string(),
            ));
        }

        if matches!(self.engine.mode, EngineMode::Live) {
            if is_empty_opt(self.venues.polymarket.private_key.as_deref()) {
                return Err(PtError::Config(
                    "engine.mode=live requires venues.polymarket.private_key".to_string(),
                ));
            }
            if self.venues.coinbase.auth.strict_live_auth
                && !self.has_usable_coinbase_auth_material()
            {
                return Err(PtError::Config(
                    "engine.mode=live requires coinbase auth material: either venues.coinbase.api_key/api_secret or an active profile with cdp_key_file/cdp_secret_id"
                        .to_string(),
                ));
            }
            if self.risk.pilot.enforce_ultra_tight_live {
                if self.risk.daily_loss_limit_pct > self.risk.pilot.ultra_tight_daily_loss_limit_pct
                    || self.risk.max_notional_per_market
                        > self.risk.pilot.ultra_tight_max_notional_per_market
                    || self.risk.max_total_open_notional
                        > self.risk.pilot.ultra_tight_max_total_open_notional
                    || self.risk.max_markets_quoted_simultaneously
                        > self.risk.pilot.ultra_tight_max_markets_quoted
                {
                    return Err(PtError::Config(
                        "engine.mode=live exceeds risk.pilot ultra-tight limits; lower risk caps or disable enforce_ultra_tight_live"
                            .to_string(),
                    ));
                }
            }
        }

        Ok(())
    }

    fn coinbase_env_profile_mut(&mut self) -> &mut CoinbaseAuthProfileConfig {
        let profile_id = self
            .venues
            .coinbase
            .auth
            .active_profile
            .clone()
            .filter(|v| !v.trim().is_empty())
            .unwrap_or_else(|| "default".to_string());
        self.venues.coinbase.auth.active_profile = Some(profile_id.clone());
        self.venues
            .coinbase
            .auth
            .profiles
            .entry(profile_id)
            .or_default()
    }

    fn has_usable_coinbase_auth_material(&self) -> bool {
        let legacy = !is_empty_opt(self.venues.coinbase.api_key.as_deref())
            && !is_empty_opt(self.venues.coinbase.api_secret.as_deref());
        if legacy {
            return true;
        }

        let Some(profile_id) = self.venues.coinbase.auth.active_profile.as_deref() else {
            return false;
        };
        let Some(profile) = self.venues.coinbase.auth.profiles.get(profile_id.trim()) else {
            return false;
        };

        !is_empty_opt(profile.cdp_key_file.as_deref())
            || !is_empty_opt(profile.cdp_secret_id.as_deref())
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
        std::env::set_var("COINBASE_AUTH_PROFILE", "ops");
        std::env::set_var("COINBASE_CDP_KEY_FILE", "config/cdp_api_key.json");
        std::env::set_var("COINBASE_CDP_SECRET_ID", "my/secret/id");
        std::env::set_var("COINBASE_EXPECTED_KEY_ID", "abc-123");
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
            cfg.venues.coinbase.auth.active_profile.as_deref(),
            Some("ops")
        );
        let profile = cfg
            .venues
            .coinbase
            .auth
            .profiles
            .get("ops")
            .expect("ops profile");
        assert_eq!(
            profile.cdp_key_file.as_deref(),
            Some("config/cdp_api_key.json")
        );
        assert_eq!(profile.cdp_secret_id.as_deref(), Some("my/secret/id"));
        assert_eq!(profile.expected_key_id.as_deref(), Some("abc-123"));
        assert_eq!(
            cfg.signals.tradingview.endpoint_secret.as_deref(),
            Some("tv_secret")
        );

        std::env::remove_var("POLYMARKET_PRIVATE_KEY");
        std::env::remove_var("COINBASE_API_KEY");
        std::env::remove_var("COINBASE_API_SECRET");
        std::env::remove_var("COINBASE_PASSPHRASE");
        std::env::remove_var("COINBASE_AUTH_PROFILE");
        std::env::remove_var("COINBASE_CDP_KEY_FILE");
        std::env::remove_var("COINBASE_CDP_SECRET_ID");
        std::env::remove_var("COINBASE_EXPECTED_KEY_ID");
        std::env::remove_var("TRADINGVIEW_ENDPOINT_SECRET");
    }

    #[test]
    fn live_mode_accepts_profile_based_coinbase_auth() {
        let raw = include_str!("../../../config/config.example.toml");
        let mut cfg = toml::from_str::<AppConfig>(raw).expect("parse config example");
        cfg.engine.mode = EngineMode::Live;
        cfg.venues.polymarket.private_key = Some("0xabc".to_string());
        cfg.venues.coinbase.api_key = None;
        cfg.venues.coinbase.api_secret = None;
        cfg.venues.coinbase.auth.active_profile = Some("pilot".to_string());
        cfg.venues.coinbase.auth.profiles.insert(
            "pilot".to_string(),
            CoinbaseAuthProfileConfig {
                cdp_key_file: Some("config/cdp_api_key.json".to_string()),
                cdp_secret_id: None,
                expected_key_id: Some("a5602f09-8a12-422a-b55d-b6981c5a0776".to_string()),
                strategy_tags: vec!["maker_mm_spot".to_string()],
            },
        );
        assert!(cfg.validate().is_ok());
    }
}
