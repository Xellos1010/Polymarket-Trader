pub mod external;
pub use external::{
    ExternalSignalAdapter, FearGreedAdapter, NormalizedExternalSignal, WebhookSignalAdapter,
};

use chrono::Utc;
use pt_core::{Asset, TradingViewBias, WalletSignal};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SignalFusionEngine {
    pub k_wallet: f64,
    pub k_tv: f64,
}

impl SignalFusionEngine {
    pub fn new(k_wallet: f64, k_tv: f64) -> Self {
        Self { k_wallet, k_tv }
    }

    pub fn fuse(
        &self,
        wallet_signals: &[WalletSignal],
        tv_bias: Option<TradingViewBias>,
    ) -> HashMap<Asset, f64> {
        let mut wallet_component: HashMap<Asset, (f64, f64)> = HashMap::new();

        for s in wallet_signals {
            let e = wallet_component
                .entry(s.asset.clone())
                .or_insert((0.0, 0.0));
            e.0 += s.bias * s.confidence;
            e.1 += s.confidence.max(1e-6);
        }

        let mut out = HashMap::new();
        for (asset, (weighted_sum, sum_conf)) in wallet_component {
            let wb = (weighted_sum / sum_conf).clamp(-1.0, 1.0);
            let tb = tv_bias.as_ref().map(|b| b.bias).unwrap_or(0.0);
            let combined = (self.k_wallet * wb + self.k_tv * tb).clamp(-1.0, 1.0);
            out.insert(asset, combined);
        }

        // Ensure all core assets have an explicit bias output.
        for asset in [Asset::Btc, Asset::Eth, Asset::Sol, Asset::Xrp] {
            out.entry(asset).or_insert_with(|| {
                let tb = tv_bias.as_ref().map(|b| b.bias).unwrap_or(0.0);
                (self.k_tv * tb).clamp(-1.0, 1.0)
            });
        }

        out
    }
}

#[derive(Debug, Clone, Deserialize)]
struct TradingViewJson {
    strategy_name: Option<String>,
    order_action: Option<String>,
    contracts: Option<String>,
    position_size: Option<String>,
    ticker: Option<String>,
}

pub fn parse_tradingview_bias(raw: &str) -> Option<TradingViewBias> {
    if let Ok(json_payload) = serde_json::from_str::<TradingViewJson>(raw) {
        return build_bias(
            json_payload.order_action.as_deref(),
            json_payload.contracts.as_deref(),
            json_payload.position_size.as_deref(),
            json_payload.ticker.as_deref(),
            json_payload.strategy_name.as_deref(),
        );
    }

    let map = parse_colon_payload(raw);
    build_bias(
        map.get("order_action").map(String::as_str),
        map.get("contracts").map(String::as_str),
        map.get("position_size").map(String::as_str),
        map.get("ticker").map(String::as_str),
        map.get("strategy_name").map(String::as_str),
    )
}

fn parse_colon_payload(raw: &str) -> HashMap<String, String> {
    raw.split(',')
        .filter_map(|pair| {
            let mut it = pair.splitn(2, ':');
            let key = it.next()?.trim().trim_matches('"').to_ascii_lowercase();
            let val = it.next()?.trim().trim_matches('"').to_string();
            Some((key, val))
        })
        .collect()
}

fn build_bias(
    action: Option<&str>,
    contracts: Option<&str>,
    position_size: Option<&str>,
    ticker: Option<&str>,
    strategy_name: Option<&str>,
) -> Option<TradingViewBias> {
    let action = action?.to_ascii_uppercase();
    let mut bias = if action == "BUY" { 1.0 } else { -1.0 };

    let contracts_val = contracts
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0)
        .abs();
    let pos_val = position_size
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0)
        .abs();

    let confidence = ((contracts_val + pos_val) / 2.0).clamp(0.05, 1.0);
    bias *= confidence;

    Some(TradingViewBias {
        bias: bias.clamp(-1.0, 1.0),
        confidence,
        source: format!(
            "tv:{}:{}",
            strategy_name.unwrap_or("unknown"),
            ticker.unwrap_or("unknown")
        ),
        ts: Utc::now(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_colon_style_payload() {
        let payload =
            "strategy_name:X,order_action:buy,contracts:0.5,ticker:BTCUSD,position_size:0.3";
        let b = parse_tradingview_bias(payload).unwrap();
        assert!(b.bias > 0.0);
    }
}
