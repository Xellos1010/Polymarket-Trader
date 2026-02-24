use chrono::Utc;
use pt_core::{Asset, PtError, PtResult, TimeBucket, WalletSignal};
use reqwest::Client;
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone)]
pub struct WalletIntelClient {
    client: Client,
    data_api: String,
    gamma_api: String,
    top_n: usize,
    allowlist_path: String,
    enforce_allowlist: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LeaderboardEntry {
    #[serde(rename = "proxyWallet")]
    pub proxy_wallet: String,
    #[serde(default)]
    pub pnl: f64,
    #[serde(default)]
    pub vol: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PositionRow {
    pub title: Option<String>,
    #[serde(default)]
    pub size: f64,
    #[serde(default, rename = "avgPrice")]
    pub avg_price: f64,
    #[serde(default, rename = "cashPnl")]
    pub cash_pnl: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TradeRow {
    pub title: Option<String>,
    pub side: Option<String>,
    #[serde(default)]
    pub price: f64,
    #[serde(default)]
    pub size: f64,
}

impl WalletIntelClient {
    pub fn new(
        data_api: impl Into<String>,
        gamma_api: impl Into<String>,
        top_n: usize,
        allowlist_path: impl Into<String>,
        enforce_allowlist: bool,
    ) -> Self {
        Self {
            client: Client::new(),
            data_api: data_api.into(),
            gamma_api: gamma_api.into(),
            top_n,
            allowlist_path: allowlist_path.into(),
            enforce_allowlist,
        }
    }

    pub async fn compute_wallet_biases(&self) -> PtResult<Vec<WalletSignal>> {
        let leaderboard = self.fetch_leaderboard().await?;
        let allowlist = self.load_allowlist();

        let candidates: Vec<LeaderboardEntry> = leaderboard
            .into_iter()
            .take(self.top_n)
            .filter(|w| {
                if !self.enforce_allowlist || allowlist.is_empty() {
                    return true;
                }
                allowlist.contains(&w.proxy_wallet.to_ascii_lowercase())
            })
            .collect();

        let mut score_by_asset: HashMap<Asset, f64> = HashMap::new();
        let mut confidence_by_asset: HashMap<Asset, f64> = HashMap::new();

        for wallet in candidates {
            let _profile = self.fetch_profile(&wallet.proxy_wallet).await.ok();
            let positions = self
                .fetch_positions(&wallet.proxy_wallet)
                .await
                .unwrap_or_default();
            let trades = self
                .fetch_trades(&wallet.proxy_wallet)
                .await
                .unwrap_or_default();
            let wallet_weight = score_wallet_weight(&wallet, &positions);

            for trade in trades {
                let title = trade.title.unwrap_or_default();
                let asset = infer_asset_from_text(&title);
                let signed = if trade
                    .side
                    .as_deref()
                    .unwrap_or_default()
                    .eq_ignore_ascii_case("BUY")
                {
                    1.0
                } else {
                    -1.0
                };
                let edge = signed * (trade.size * trade.price.max(0.01));
                *score_by_asset.entry(asset.clone()).or_insert(0.0) += edge * wallet_weight;
                *confidence_by_asset.entry(asset).or_insert(0.0) += trade.size.abs();
            }
        }

        let mut out = Vec::new();
        for (asset, raw_score) in score_by_asset {
            let confidence = confidence_by_asset.get(&asset).copied().unwrap_or(0.0);
            let normalized = (raw_score / 10_000.0).clamp(-1.0, 1.0);
            out.push(WalletSignal {
                asset,
                horizon: TimeBucket::Other,
                bias: normalized,
                confidence: (confidence / 1_000.0).clamp(0.0, 1.0),
                ts: Utc::now(),
            });
        }

        Ok(out)
    }

    async fn fetch_leaderboard(&self) -> PtResult<Vec<LeaderboardEntry>> {
        let url = format!("{}/v1/leaderboard?category=CRYPTO", self.data_api);
        self.client
            .get(url)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?
            .error_for_status()
            .map_err(|e| PtError::Http(e.to_string()))?
            .json::<Vec<LeaderboardEntry>>()
            .await
            .map_err(|e| PtError::Serde(e.to_string()))
    }

    async fn fetch_profile(&self, address: &str) -> PtResult<serde_json::Value> {
        let url = format!("{}/public-profile?address={}", self.gamma_api, address);
        self.client
            .get(url)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?
            .error_for_status()
            .map_err(|e| PtError::Http(e.to_string()))?
            .json::<serde_json::Value>()
            .await
            .map_err(|e| PtError::Serde(e.to_string()))
    }

    async fn fetch_positions(&self, user: &str) -> PtResult<Vec<PositionRow>> {
        let url = format!("{}/positions?user={}", self.data_api, user);
        self.client
            .get(url)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?
            .error_for_status()
            .map_err(|e| PtError::Http(e.to_string()))?
            .json::<Vec<PositionRow>>()
            .await
            .map_err(|e| PtError::Serde(e.to_string()))
    }

    async fn fetch_trades(&self, user: &str) -> PtResult<Vec<TradeRow>> {
        let url = format!("{}/trades?user={}&limit=100", self.data_api, user);
        self.client
            .get(url)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?
            .error_for_status()
            .map_err(|e| PtError::Http(e.to_string()))?
            .json::<Vec<TradeRow>>()
            .await
            .map_err(|e| PtError::Serde(e.to_string()))
    }

    fn load_allowlist(&self) -> HashSet<String> {
        let path = std::path::Path::new(&self.allowlist_path);
        if !path.exists() {
            return HashSet::new();
        }

        fs::read_to_string(path)
            .unwrap_or_default()
            .lines()
            .map(|line| line.trim().to_ascii_lowercase())
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .collect()
    }
}

fn score_wallet_weight(wallet: &LeaderboardEntry, positions: &[PositionRow]) -> f64 {
    let pnl_component = (wallet.pnl / 10_000.0).clamp(-1.0, 2.0);
    let vol_component = (wallet.vol / 1_000_000.0).clamp(0.0, 2.0);

    let net_cash_pnl: f64 = positions.iter().map(|p| p.cash_pnl).sum();
    let inventory_load: f64 = positions
        .iter()
        .map(|p| p.size * p.avg_price.max(0.01))
        .sum();

    let quality = 1.0 + pnl_component + (net_cash_pnl / 10_000.0).clamp(-0.5, 0.5);
    let turnover_penalty = if inventory_load > 1_000_000.0 {
        0.8
    } else {
        1.0
    };

    (quality + vol_component).clamp(0.1, 4.0) * turnover_penalty
}

fn infer_asset_from_text(text: &str) -> Asset {
    let lower = text.to_ascii_lowercase();
    if lower.contains("bitcoin") || lower.contains(" btc") {
        Asset::Btc
    } else if lower.contains("ethereum") || lower.contains(" eth") {
        Asset::Eth
    } else if lower.contains("solana") || lower.contains(" sol") {
        Asset::Sol
    } else if lower.contains("xrp") {
        Asset::Xrp
    } else {
        Asset::Other
    }
}
