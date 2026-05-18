use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaType {
    Ema,
    Sma,
    Wma,
    Hma,
    Dema,
    Tema,
    Vwma,
    Rma,
    Zlema,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorSettings {
    pub ma_type: MaType,
    pub ma_fast: usize,
    pub ma_slow: usize,
    pub rsi_len: usize,
    pub rsi_oversold: f64,
    pub rsi_overbought: f64,
    pub bb_len: usize,
    pub bb_fib_multiplier: f64,
    pub ichimoku_conv: usize,
    pub ichimoku_base: usize,
    pub ichimoku_span_b: usize,
    pub ichimoku_displacement: usize,
    pub macd_fast: usize,
    pub macd_slow: usize,
    pub macd_signal: usize,
    pub adx_len: usize,
    pub atr_len: usize,
    pub stoch_rsi_len: usize,
    pub stoch_rsi_smooth: usize,
    pub volume_lookback: usize,
    pub vwap_dev_lookback: usize,
}

impl Default for IndicatorSettings {
    fn default() -> Self {
        Self {
            ma_type: MaType::Ema,
            ma_fast: 50,
            ma_slow: 200,
            rsi_len: 14,
            rsi_oversold: 30.0,
            rsi_overbought: 70.0,
            bb_len: 20,
            bb_fib_multiplier: 1.618,
            ichimoku_conv: 9,
            ichimoku_base: 26,
            ichimoku_span_b: 52,
            ichimoku_displacement: 26,
            macd_fast: 12,
            macd_slow: 26,
            macd_signal: 9,
            adx_len: 14,
            atr_len: 14,
            stoch_rsi_len: 14,
            stoch_rsi_smooth: 3,
            volume_lookback: 20,
            vwap_dev_lookback: 20,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionSettings {
    pub buy_threshold: f64,
    pub sell_threshold: f64,
    pub min_confluence: usize,
    pub neutral_regime_multiplier: f64,
}

impl Default for FusionSettings {
    fn default() -> Self {
        Self {
            buy_threshold: 0.60,
            sell_threshold: -0.60,
            min_confluence: 2,
            neutral_regime_multiplier: 1.20,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestCosts {
    pub fee_bps: f64,
    pub slippage_bps: f64,
    pub fixed_trade_cost: f64,
}

impl Default for BacktestCosts {
    fn default() -> Self {
        Self {
            fee_bps: 8.0,
            slippage_bps: 2.0,
            fixed_trade_cost: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyProfile {
    pub profile_id: String,
    pub name: String,
    pub version: u32,
    pub product_id: String,
    pub granularity_sec: u32,
    pub candle_limit: usize,
    pub starting_equity: f64,
    pub indicators: IndicatorSettings,
    pub fusion: FusionSettings,
    pub costs: BacktestCosts,
    pub weights: HashMap<String, f64>,
}

impl Default for StrategyProfile {
    fn default() -> Self {
        let mut weights = HashMap::new();
        weights.insert("ma_regime".to_string(), 1.0);
        weights.insert("rsi".to_string(), 0.8);
        weights.insert("fib_bb".to_string(), 0.7);
        weights.insert("ichimoku".to_string(), 0.8);
        weights.insert("macd".to_string(), 0.7);
        weights.insert("adx".to_string(), 0.6);
        weights.insert("atr".to_string(), 0.5);
        weights.insert("volume".to_string(), 0.6);
        weights.insert("vwap_dev".to_string(), 0.6);
        weights.insert("stoch_rsi".to_string(), 0.7);
        Self {
            profile_id: "default".to_string(),
            name: "Default Strategy".to_string(),
            version: 1,
            product_id: "BTC-USD".to_string(),
            granularity_sec: 300,
            candle_limit: 600,
            starting_equity: 1000.0,
            indicators: IndicatorSettings::default(),
            fusion: FusionSettings::default(),
            costs: BacktestCosts::default(),
            weights,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    pub ts_ms: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegimeState {
    Bull,
    Bear,
    Neutral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorSignal {
    pub name: String,
    pub bias: f64,
    pub confidence: f64,
    pub regime_vote: RegimeState,
    pub metadata: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TradeAction {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionDecision {
    pub ts_ms: i64,
    pub score: f64,
    pub regime: RegimeState,
    pub action: TradeAction,
    pub confluence: usize,
    pub indicators: Vec<IndicatorSignal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeFill {
    pub ts_ms: i64,
    pub action: TradeAction,
    pub price: f64,
    pub qty: f64,
    pub notional: f64,
    pub fee_cost: f64,
    pub slippage_cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityPoint {
    pub ts_ms: i64,
    pub equity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyRunReport {
    /// Deterministic: hex(sha256(params_hash + candle_start_ms as string)).
    pub run_id: String,
    /// SHA-256 of JSON-serialized StrategyProfile (hex).
    pub params_hash: String,
    pub candle_start_ms: i64,
    pub candle_end_ms: i64,
    pub profile_id: String,
    pub product_id: String,
    pub granularity_sec: u32,
    pub started_ts_ms: i64,
    pub total_return_pct: f64,
    pub max_drawdown_pct: f64,
    pub trades: usize,
    pub win_rate: f64,
    pub pnl: f64,
    pub candles: Vec<Candle>,
    pub decisions: Vec<FusionDecision>,
    pub fills: Vec<TradeFill>,
    pub equity_curve: Vec<EquityPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningCandidate {
    pub rank: usize,
    pub score: f64,
    pub profile: StrategyProfile,
    pub report: StrategyRunReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningReport {
    pub created_ts_ms: i64,
    pub iterations: usize,
    pub walk_forward_splits: usize,
    pub top: Vec<TuningCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperEndpointReport {
    pub created_ts_ms: i64,
    pub profile_id: String,
    pub simulated_orders: usize,
    pub simulated_edits: usize,
    pub simulated_cancel_replace: usize,
    pub estimated_reject_rate: f64,
    pub notes: String,
}
