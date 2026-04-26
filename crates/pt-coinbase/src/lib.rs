use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::{SinkExt, StreamExt};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use p256::pkcs8::{EncodePrivateKey, LineEnding};
use p256::SecretKey;
use parking_lot::RwLock;
use pt_core::{
    Asset, AuthReloadResult, CoinbaseAuthProfileConfig, CoinbaseAuthSource, CoinbaseConfig,
    CoinbaseL2Update, CoinbaseOrderBookState, EngineMode, ExecutionReport, ExecutionStatus,
    PtError, PtResult, ResolvedCoinbaseAuth, Side, UserOrderEvent, Venue, WalletBalance,
};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
    Asset, ExecutionReport, ExecutionStatus, OrderRoute, PtError, PtResult, Side, Venue,
};
use reqwest::header::{
    HeaderMap, HeaderValue, AUTHORIZATION, CACHE_CONTROL, CONTENT_TYPE,
};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    fs,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
use tokio::sync::mpsc;
use tokio::time::{sleep, timeout};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct HedgeIntent {
    pub asset: Asset,
    pub side: Side,
    pub usd_notional: f64,
    pub max_slippage_bps: f64,
    pub risk_unwind: bool,
}

#[async_trait]
pub trait HedgeExecutor: Send + Sync {
    async fn hedge(&self, intent: HedgeIntent) -> PtResult<ExecutionReport>;
}

#[derive(Debug, Default)]
pub struct NoopCoinbaseHedger;

#[async_trait]
impl HedgeExecutor for NoopCoinbaseHedger {
    async fn hedge(&self, intent: HedgeIntent) -> PtResult<ExecutionReport> {
        Ok(ExecutionReport {
            venue: Venue::Coinbase,
            order_id: format!("noop-hedge-{:?}", intent.asset),
            market_id: None,
            status: ExecutionStatus::New,
            side: intent.side,
            filled_qty: 0.0,
            avg_px: 0.0,
            ts: Utc::now(),
            details: Some("noop hedge executor".to_string()),
        })
    }
}

#[derive(Debug, Default)]
pub struct PaperCoinbaseHedger {
    pub net_hedged_usd: RwLock<f64>,
}

#[async_trait]
impl HedgeExecutor for PaperCoinbaseHedger {
    async fn hedge(&self, intent: HedgeIntent) -> PtResult<ExecutionReport> {
        let signed = match intent.side {
            Side::Buy => intent.usd_notional,
            Side::Sell => -intent.usd_notional,
        };
        *self.net_hedged_usd.write() += signed;

        Ok(ExecutionReport {
            venue: Venue::Sim,
            order_id: format!("paper-hedge-{}", Utc::now().timestamp_millis()),
            market_id: None,
            status: ExecutionStatus::Filled,
            side: intent.side,
            filled_qty: intent.usd_notional,
            avg_px: 1.0,
            ts: Utc::now(),
            details: Some("paper hedge fill".to_string()),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinbaseAuthStatus {
    pub ok: bool,
    pub profile_id: Option<String>,
    pub key_id_suffix: Option<String>,
    pub source: Option<CoinbaseAuthSource>,
    pub loaded_at: Option<DateTime<Utc>>,
    pub reason: String,
    pub allow_hot_reload: bool,
}

#[derive(Debug)]
pub struct CoinbaseAuthManager {
    cfg: CoinbaseConfig,
    mode: EngineMode,
    active_profile: RwLock<Option<String>>,
    resolved: RwLock<Option<ResolvedCoinbaseAuth>>,
    generation: AtomicU64,
}

impl CoinbaseAuthManager {
    pub fn new(cfg: CoinbaseConfig, mode: EngineMode) -> PtResult<Self> {
        let manager = Self {
            active_profile: RwLock::new(
                cfg.auth
                    .active_profile
                    .clone()
                    .filter(|v| !v.trim().is_empty()),
            ),
            resolved: RwLock::new(None),
            generation: AtomicU64::new(0),
            cfg,
            mode,
        };
        let auth = manager.resolve_selected_auth()?;
        *manager.resolved.write() = Some(auth);
        manager.generation.store(1, Ordering::SeqCst);
        Ok(manager)
    }

    pub fn current_auth(&self) -> PtResult<ResolvedCoinbaseAuth> {
        if let Some(v) = self.resolved.read().clone() {
            return Ok(v);
        }
        let auth = self.resolve_selected_auth()?;
        *self.resolved.write() = Some(auth.clone());
        Ok(auth)
    }

    pub fn generation(&self) -> u64 {
        self.generation.load(Ordering::SeqCst)
    }

    pub fn active_profile(&self) -> Option<String> {
        self.active_profile.read().clone()
    }

    pub fn status(&self) -> CoinbaseAuthStatus {
        match self.current_auth() {
            Ok(auth) => CoinbaseAuthStatus {
                ok: true,
                profile_id: auth.profile_id,
                key_id_suffix: Some(key_suffix(&auth.key_id)),
                source: Some(auth.source),
                loaded_at: Some(auth.loaded_at),
                reason: "ready".to_string(),
                allow_hot_reload: self.cfg.auth.allow_hot_reload,
            },
            Err(e) => CoinbaseAuthStatus {
                ok: false,
                profile_id: self.active_profile(),
                key_id_suffix: None,
                source: None,
                loaded_at: None,
                reason: e.to_string(),
                allow_hot_reload: self.cfg.auth.allow_hot_reload,
            },
        }
    }

    pub fn reload_active_profile(&self) -> AuthReloadResult {
        if !self.cfg.auth.allow_hot_reload {
            return AuthReloadResult {
                ok: false,
                profile_id: self.active_profile(),
                key_id_suffix: None,
                source: None,
                reason: "coinbase auth hot reload is disabled".to_string(),
                ts: Utc::now(),
            };
        }

        match self.resolve_selected_auth() {
            Ok(auth) => {
                let profile_id = auth.profile_id.clone();
                let key_id_suffix = key_suffix(&auth.key_id);
                let source = auth.source.clone();
                *self.resolved.write() = Some(auth);
                self.generation.fetch_add(1, Ordering::SeqCst);
                AuthReloadResult {
                    ok: true,
                    profile_id,
                    key_id_suffix: Some(key_id_suffix),
                    source: Some(source),
                    reason: "reloaded active coinbase auth profile".to_string(),
                    ts: Utc::now(),
                }
            }
            Err(e) => AuthReloadResult {
                ok: false,
                profile_id: self.active_profile(),
                key_id_suffix: None,
                source: None,
                reason: e.to_string(),
                ts: Utc::now(),
            },
        }
    }

    pub fn switch_profile(&self, profile_id: &str) -> AuthReloadResult {
        if !self.cfg.auth.allow_hot_reload {
            return AuthReloadResult {
                ok: false,
                profile_id: self.active_profile(),
                key_id_suffix: None,
                source: None,
                reason: "coinbase auth hot reload is disabled".to_string(),
                ts: Utc::now(),
            };
        }
        if profile_id.trim().is_empty() {
            return AuthReloadResult {
                ok: false,
                profile_id: self.active_profile(),
                key_id_suffix: None,
                source: None,
                reason: "profile_id must not be empty".to_string(),
                ts: Utc::now(),
            };
        }
        if !self.cfg.auth.profiles.contains_key(profile_id.trim()) {
            return AuthReloadResult {
                ok: false,
                profile_id: Some(profile_id.trim().to_string()),
                key_id_suffix: None,
                source: None,
                reason: "profile_id not found in venues.coinbase.auth.profiles".to_string(),
                ts: Utc::now(),
            };
        }
        *self.active_profile.write() = Some(profile_id.trim().to_string());
        self.reload_active_profile()
    }

    fn resolve_selected_auth(&self) -> PtResult<ResolvedCoinbaseAuth> {
        if let Some(auth) = self.resolve_legacy_auth()? {
            return Ok(auth);
        }

        let profile_id = self
            .active_profile()
            .or_else(|| self.cfg.auth.active_profile.clone())
            .ok_or_else(|| {
                PtError::Config(
                    "missing coinbase auth: no active profile and no legacy api_key/api_secret"
                        .to_string(),
                )
            })?;
        let profile = self
            .cfg
            .auth
            .profiles
            .get(profile_id.trim())
            .ok_or_else(|| {
                PtError::Config(format!(
                    "coinbase active profile '{}' is missing in config",
                    profile_id
                ))
            })?;

        self.resolve_profile_auth(profile_id.trim(), profile)
    }

    fn resolve_legacy_auth(&self) -> PtResult<Option<ResolvedCoinbaseAuth>> {
        let key = self.cfg.api_key.as_deref().unwrap_or_default().trim();
        let secret = self.cfg.api_secret.as_deref().unwrap_or_default().trim();
        if key.is_empty() || secret.is_empty() {
            return Ok(None);
        }
        let key_id = extract_key_id(key);
        Ok(Some(ResolvedCoinbaseAuth {
            profile_id: None,
            key_name: key.to_string(),
            key_id,
            private_key_pem: normalize_multiline_secret(secret),
            source: CoinbaseAuthSource::LegacyInline,
            loaded_at: Utc::now(),
        }))
    }

    fn resolve_profile_auth(
        &self,
        profile_id: &str,
        profile: &CoinbaseAuthProfileConfig,
    ) -> PtResult<ResolvedCoinbaseAuth> {
        let from_file = profile
            .cdp_key_file
            .as_deref()
            .filter(|v| !v.trim().is_empty());
        let from_secret = profile
            .cdp_secret_id
            .as_deref()
            .filter(|v| !v.trim().is_empty());

        let (payload, source) = if let Some(path) = from_file {
            let raw = fs::read_to_string(path)
                .map_err(|e| PtError::Io(format!("coinbase cdp_key_file read failed: {e}")))?;
            (raw, CoinbaseAuthSource::CdpKeyFile)
        } else if let Some(secret_id) = from_secret {
            (
                fetch_secret_string_blocking(secret_id)?,
                CoinbaseAuthSource::AwsSecretsManager,
            )
        } else {
            return Err(PtError::Config(format!(
                "coinbase profile '{}' is missing cdp_key_file/cdp_secret_id",
                profile_id
            )));
        };

        let parsed: CoinbaseCdpKeyPayload =
            serde_json::from_str(&payload).map_err(|e| PtError::Serde(e.to_string()))?;
        if parsed.name.trim().is_empty() || parsed.private_key.trim().is_empty() {
            return Err(PtError::Config(
                "coinbase cdp payload must include non-empty name/privateKey".to_string(),
            ));
        }
        let key_id = extract_key_id(parsed.name.trim());
        if matches!(self.mode, EngineMode::Live) {
            if let Some(expected) = profile.expected_key_id.as_deref() {
                let expected_trimmed = expected.trim();
                if !expected_trimmed.is_empty() && !key_id.eq_ignore_ascii_case(expected_trimmed) {
                    return Err(PtError::Config(format!(
                        "coinbase key_id mismatch for profile '{}': expected {} got {}",
                        profile_id, expected_trimmed, key_id
                    )));
                }
            }
        }

        Ok(ResolvedCoinbaseAuth {
            profile_id: Some(profile_id.to_string()),
            key_name: parsed.name.trim().to_string(),
            key_id,
            private_key_pem: normalize_multiline_secret(parsed.private_key.trim()),
            source,
            loaded_at: Utc::now(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct CoinbaseCdpKeyPayload {
    name: String,
    #[serde(rename = "privateKey")]
    private_key: String,
}

#[derive(Debug, Clone)]
pub struct CoinbaseSpotHedger {
    client: Client,
    api_base: String,
    jwt_host_path: String,
    auth_manager: Option<Arc<CoinbaseAuthManager>>,
    api_key: Option<String>,
    api_secret: Option<String>,
    passphrase: Option<String>,
}

impl CoinbaseSpotHedger {
    pub fn new(
        api_base: impl Into<String>,
        api_key: Option<String>,
        api_secret: Option<String>,
        passphrase: Option<String>,
    ) -> Self {
        let api_base = api_base.into();
        let jwt_host_path = derive_host_path_for_jwt(&api_base);
        Self {
            client: Client::new(),
            api_base,
            jwt_host_path,
            auth_manager: None,
            api_key,
            api_secret,
            passphrase,
        }
    }

    pub fn new_with_auth_manager(
        api_base: impl Into<String>,
        auth_manager: Arc<CoinbaseAuthManager>,
        passphrase: Option<String>,
    ) -> Self {
        let api_base = api_base.into();
        let jwt_host_path = derive_host_path_for_jwt(&api_base);
        Self {
            client: Client::new(),
            api_base,
            jwt_host_path,
            auth_manager: Some(auth_manager),
            api_key: None,
            api_secret: None,
            passphrase,
        }
    }

    pub fn auth_status(&self) -> Option<CoinbaseAuthStatus> {
        self.auth_manager.as_ref().map(|m| m.status())
    }

    pub fn reload_auth(&self) -> Option<AuthReloadResult> {
        self.auth_manager
            .as_ref()
            .map(|m| m.reload_active_profile())
    }

    pub fn switch_auth_profile(&self, profile_id: &str) -> Option<AuthReloadResult> {
        self.auth_manager
            .as_ref()
            .map(|m| m.switch_profile(profile_id))
    }

    fn credentials(&self) -> PtResult<(String, String)> {
        if let Some(manager) = &self.auth_manager {
            let auth = manager.current_auth()?;
            return Ok((auth.key_name, auth.private_key_pem));
        }

        let key = self
            .api_key
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| {
                PtError::Config("missing venues.coinbase.api_key for live hedging".to_string())
            })?;

        let secret = self
            .api_secret
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| {
                PtError::Config("missing venues.coinbase.api_secret for live hedging".to_string())
            })?;

        Ok((key.to_string(), secret.to_string()))
    }

    fn build_jwt(&self, method: &str, path: &str) -> PtResult<String> {
        let (api_key, api_secret_raw) = self.credentials()?;
        let now = Utc::now().timestamp();
        let uri = format!("{method} {}{path}", self.jwt_host_path);

        let claims = CoinbaseJwtClaims {
            iss: "cdp".to_string(),
            nbf: now,
            exp: now + 120,
            sub: api_key.to_string(),
            uri: Some(uri),
        };

        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(api_key.clone());

        let key = build_ec_encoding_key(&api_secret_raw)?;

        encode(&header, &claims, &key)
            .map_err(|e| PtError::Http(format!("coinbase jwt generation failed: {e}")))
    }

    fn signed_headers(&self, method: &str, path: &str) -> PtResult<HeaderMap> {
        let token = self.build_jwt(method, path)?;
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {token}"))
                .map_err(|e| PtError::Http(e.to_string()))?,
        );
        if let Some(passphrase) = &self.passphrase {
            if !passphrase.trim().is_empty() {
                headers.insert(
                    "CB-ACCESS-PASSPHRASE",
                    HeaderValue::from_str(passphrase).map_err(|e| PtError::Http(e.to_string()))?,
                );
            }
        }
        Ok(headers)
    }

    fn product_for_asset(asset: &Asset) -> PtResult<&'static str> {
        match asset {
            Asset::Btc => Ok("BTC-USD"),
            Asset::Eth => Ok("ETH-USD"),
            Asset::Sol => Ok("SOL-USD"),
            Asset::Xrp => Ok("XRP-USD"),
            Asset::Other => Err(PtError::Unsupported(
                "unsupported asset for coinbase hedge".to_string(),
            )),
        }
    }

    fn endpoint_url(&self, path: &str) -> String {
        format!("{}/{}", self.api_base.trim_end_matches('/'), path)
    }

    async fn fetch_ticker_price(&self, product: &str) -> PtResult<f64> {
        let path = format!("/products/{product}/ticker");
        let url = self.endpoint_url(path.trim_start_matches('/'));
        let headers = self.signed_headers("GET", &path)?;

        let resp = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        let status = resp.status();
        let body = resp
            .text()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;

        if !status.is_success() {
            return Err(PtError::Http(format!(
                "coinbase ticker failed status={} body={body}",
                status
            )));
        }

        let parsed: CoinbaseTickerResponse =
            serde_json::from_str(&body).map_err(|e| PtError::Serde(e.to_string()))?;
        let price = parsed
            .price
            .and_then(|v| v.parse::<f64>().ok())
            .filter(|p| p.is_finite() && *p > 0.0)
            .ok_or_else(|| PtError::Http(format!("coinbase ticker missing price body={body}")))?;

        Ok(price)
    }
}

#[async_trait]
impl HedgeExecutor for CoinbaseSpotHedger {
    async fn hedge(&self, intent: HedgeIntent) -> PtResult<ExecutionReport> {
        self.credentials()?;
        if !intent.usd_notional.is_finite() || intent.usd_notional <= 0.0 {
            return Err(PtError::InvalidInput(format!(
                "invalid hedge notional: {}",
                intent.usd_notional
            )));
        }
        if !intent.risk_unwind {
            return Err(PtError::Risk(
                "taker hedge blocked: risk_unwind=false".to_string(),
            ));
        }

        let product = Self::product_for_asset(&intent.asset)?;
        let side = match intent.side {
            Side::Buy => "BUY",
            Side::Sell => "SELL",
        };

        let px = self.fetch_ticker_price(product).await?;
        let base_size = (intent.usd_notional / px).max(0.00000001);

        let body = CoinbaseCreateOrderRequest {
            client_order_id: Uuid::new_v4().to_string(),
            product_id: product.to_string(),
            side: side.to_string(),
            preview_id: None,
            order_configuration: serde_json::json!({
                "market_market_ioc": {
                    "base_size": format!("{base_size:.8}")
                }
            }),
        };

        let path = "/orders";
        let url = self.endpoint_url(path.trim_start_matches('/'));
        let headers = self.signed_headers("POST", path)?;

        let resp = self
            .client
            .post(url)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        let status = resp.status();
        let raw = resp
            .text()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;

        if !status.is_success() {
            return Err(PtError::Http(format!(
                "coinbase create order failed status={} body={raw}",
                status
            )));
        }

        let parsed: CoinbaseCreateOrderResponse =
            serde_json::from_str(&raw).map_err(|e| PtError::Serde(e.to_string()))?;

        let order_id = parsed
            .order_id
            .or_else(|| parsed.success_response.and_then(|v| v.order_id))
            .unwrap_or_else(|| format!("cb-order-{}", Utc::now().timestamp_millis()));
        let exec_status = if parsed.success {
            ExecutionStatus::New
        } else {
            ExecutionStatus::Rejected
        };
        let rejected = matches!(exec_status, ExecutionStatus::Rejected);

        Ok(ExecutionReport {
            venue: Venue::Coinbase,
            order_id,
            market_id: None,
            status: exec_status,
            side: intent.side,
            filled_qty: if rejected { 0.0 } else { intent.usd_notional },
            avg_px: px,
            ts: Utc::now(),
            details: Some(format!(
                "success={} response_status={} max_slippage_bps={} body={raw}",
                parsed.success, status, intent.max_slippage_bps
            )),
        })
    }
}

#[derive(Debug, Clone)]
pub struct CoinbaseAdvancedTradeClient {
    client: Client,
    api_base: String,
    jwt_host_path: String,
    api_key: Option<String>,
    api_secret: Option<String>,
    passphrase: Option<String>,
}

impl CoinbaseAdvancedTradeClient {
    pub fn new(
        api_base: impl Into<String>,
        api_key: Option<String>,
        api_secret: Option<String>,
        passphrase: Option<String>,
    ) -> Self {
        let api_base = api_base.into();
        let jwt_host_path = derive_host_path_for_jwt(&api_base);
        Self {
            client: Client::new(),
            api_base,
            jwt_host_path,
            api_key,
            api_secret,
            passphrase,
        }
    }

    pub fn credentials_available(&self) -> bool {
        self.api_key
            .as_deref()
            .map(|v| !v.trim().is_empty())
            .unwrap_or(false)
            && self
                .api_secret
                .as_deref()
                .map(|v| !v.trim().is_empty())
                .unwrap_or(false)
    }

    fn credentials(&self) -> PtResult<(&str, &str)> {
        let key = self
            .api_key
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| PtError::Config("missing venues.coinbase.api_key".to_string()))?;
        let secret = self
            .api_secret
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| PtError::Config("missing venues.coinbase.api_secret".to_string()))?;
        Ok((key, secret))
    }

    fn build_jwt(&self, method: &str, path: &str) -> PtResult<String> {
        let (api_key, api_secret_raw) = self.credentials()?;
        let now = Utc::now().timestamp();
        let uri = format!("{method} {}{path}", self.jwt_host_path);

        let claims = CoinbaseJwtClaims {
            iss: "cdp".to_string(),
            nbf: now,
            exp: now + 120,
            sub: api_key.to_string(),
            uri: Some(uri),
        };

        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(api_key.to_string());

        let normalized_secret = normalize_multiline_secret(api_secret_raw);
        let key = EncodingKey::from_ec_pem(normalized_secret.as_bytes())
            .map_err(|e| PtError::Config(format!("invalid coinbase EC private key: {e}")))?;

        encode(&header, &claims, &key)
            .map_err(|e| PtError::Http(format!("coinbase jwt generation failed: {e}")))
    }

    fn public_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
        headers
    }

    fn signed_headers(&self, method: &str, path: &str) -> PtResult<HeaderMap> {
        let token = self.build_jwt(method, path)?;
        let mut headers = self.public_headers();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {token}"))
                .map_err(|e| PtError::Http(e.to_string()))?,
        );
        if let Some(passphrase) = &self.passphrase {
            if !passphrase.trim().is_empty() {
                headers.insert(
                    "CB-ACCESS-PASSPHRASE",
                    HeaderValue::from_str(passphrase).map_err(|e| PtError::Http(e.to_string()))?,
                );
            }
        }
        Ok(headers)
    }

    fn endpoint_url(&self, path: &str) -> String {
        format!("{}/{}", self.api_base.trim_end_matches('/'), path)
    }

    async fn get_json<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, String)],
        auth_required: bool,
    ) -> PtResult<T> {
        let method = "GET";
        let url = self.endpoint_url(path.trim_start_matches('/'));
        let headers = if auth_required {
            self.signed_headers(method, path)?
        } else {
            self.public_headers()
        };

        let resp = self
            .client
            .get(url)
            .headers(headers)
            .query(query)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        let status = resp.status();
        let raw = resp
            .text()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        if !status.is_success() {
            return Err(PtError::Http(format!(
                "coinbase GET {} failed status={} body={raw}",
                path, status
            )));
        }

        serde_json::from_str::<T>(&raw).map_err(|e| PtError::Serde(e.to_string()))
    }

    async fn post_json<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> PtResult<T> {
        let method = "POST";
        let url = self.endpoint_url(path.trim_start_matches('/'));
        let headers = self.signed_headers(method, path)?;

        let resp = self
            .client
            .post(url)
            .headers(headers)
            .json(body)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        let status = resp.status();
        let raw = resp
            .text()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        if !status.is_success() {
            return Err(PtError::Http(format!(
                "coinbase POST {} failed status={} body={raw}",
                path, status
            )));
        }

        serde_json::from_str::<T>(&raw).map_err(|e| PtError::Serde(e.to_string()))
    }

    pub async fn list_public_products(&self, limit: usize) -> PtResult<Vec<CoinbaseProduct>> {
        let mut products = self
            .get_json::<CoinbaseProductsResponse>(
                "/market/products",
                &[
                    ("limit", limit.max(1).to_string()),
                    ("get_all_products", "true".to_string()),
                ],
                false,
            )
            .await?;
        products.products.sort_by(|a, b| a.product_id.cmp(&b.product_id));
        Ok(products.products)
    }

    pub async fn get_public_product_book(
        &self,
        product_id: &str,
        limit: usize,
    ) -> PtResult<CoinbasePriceBook> {
        let response = self
            .get_json::<CoinbaseProductBookResponse>(
                &format!("/market/product_book"),
                &[
                    ("product_id", product_id.to_string()),
                    ("limit", limit.max(1).to_string()),
                ],
                false,
            )
            .await?;

        response
            .pricebook
            .or_else(|| response.pricebooks.into_iter().next())
            .ok_or_else(|| PtError::Http("coinbase product book missing pricebook".to_string()))
    }

    pub async fn get_public_market_trades(
        &self,
        product_id: &str,
        limit: usize,
    ) -> PtResult<Vec<CoinbaseMarketTrade>> {
        let response = self
            .get_json::<CoinbaseMarketTradesResponse>(
                &format!("/market/products/{product_id}/ticker"),
                &[("limit", limit.max(1).to_string())],
                false,
            )
            .await?;
        Ok(response.trades)
    }

    pub async fn get_public_candles(
        &self,
        product_id: &str,
        granularity_sec: u64,
        limit: usize,
    ) -> PtResult<Vec<CoinbaseCandle>> {
        let response = self
            .get_json::<CoinbaseCandlesResponse>(
                &format!("/market/products/{product_id}/candles"),
                &[
                    ("granularity", granularity_sec.to_string()),
                    ("limit", limit.max(1).to_string()),
                ],
                false,
            )
            .await?;
        Ok(response.candles)
    }

    pub async fn list_orders(&self, product_id: Option<&str>) -> PtResult<Vec<CoinbaseOrderSummary>> {
        let mut query = vec![("limit", "50".to_string())];
        if let Some(product_id) = product_id {
            if !product_id.trim().is_empty() {
                query.push(("product_id", product_id.to_string()));
            }
        }
        let response = self
            .get_json::<CoinbaseListOrdersResponse>("/orders/historical/batch", &query, true)
            .await?;
        Ok(response.orders)
    }

    pub async fn preview_order(
        &self,
        request: &CoinbaseAdvancedTradeOrderRequest,
    ) -> PtResult<CoinbaseOrderPreview> {
        let body = request.as_preview_body();
        let response = self
            .post_json::<CoinbaseCreateOrderRequest, CoinbaseOrderPreviewResponse>(
                "/orders/preview",
                &body,
            )
            .await?;
        Ok(CoinbaseOrderPreview {
            success: response.success,
            preview_id: response.preview_id,
            raw_status: response.raw_status,
        })
    }

    pub async fn create_order(
        &self,
        request: &CoinbaseAdvancedTradeOrderRequest,
    ) -> PtResult<CoinbaseSubmittedOrder> {
        let body = request.as_create_body();
        let response = self
            .post_json::<CoinbaseCreateOrderRequest, CoinbaseCreateOrderResponse>("/orders", &body)
            .await?;
        let order_id = response
            .order_id
            .or_else(|| response.success_response.and_then(|v| v.order_id))
            .unwrap_or_else(|| format!("cb-order-{}", Utc::now().timestamp_millis()));
        Ok(CoinbaseSubmittedOrder {
            order_id,
            client_order_id: body.client_order_id,
            success: response.success,
            raw_status: if response.success {
                "accepted".to_string()
            } else {
                "rejected".to_string()
            },
        })
    }

    pub async fn cancel_orders(&self, order_ids: &[String]) -> PtResult<Vec<CoinbaseCancelResult>> {
        let body = CoinbaseBatchCancelRequest {
            order_ids: order_ids.to_vec(),
        };
        let response = self
            .post_json::<CoinbaseBatchCancelRequest, CoinbaseBatchCancelResponse>(
                "/orders/batch_cancel",
                &body,
            )
            .await?;
        Ok(response.results)
    }
}

#[derive(Debug, Clone)]
pub struct CoinbaseAdvancedTradeOrderRequest {
    pub product_id: String,
    pub side: Side,
    pub route: OrderRoute,
    pub base_size: f64,
    pub quote_size: Option<f64>,
    pub limit_price: Option<f64>,
    pub post_only: bool,
    pub preview_id: Option<String>,
}

impl CoinbaseAdvancedTradeOrderRequest {
    fn as_preview_body(&self) -> CoinbaseCreateOrderRequest {
        self.as_order_body(None)
    }

    fn as_create_body(&self) -> CoinbaseCreateOrderRequest {
        self.as_order_body(self.preview_id.as_deref())
    }

    fn as_order_body(&self, preview_id: Option<&str>) -> CoinbaseCreateOrderRequest {
        let side = match self.side {
            Side::Buy => "BUY",
            Side::Sell => "SELL",
        };

        let order_configuration = match self.route {
            OrderRoute::Maker => serde_json::json!({
                "limit_limit_gtc": {
                    "base_size": format!("{:.8}", self.base_size.max(0.00000001)),
                    "limit_price": format!("{:.8}", self.limit_price.unwrap_or_default()),
                    "post_only": self.post_only,
                }
            }),
            OrderRoute::Taker => {
                if let Some(quote_size) = self.quote_size {
                    serde_json::json!({
                        "market_market_ioc": {
                            "quote_size": format!("{:.8}", quote_size.max(0.01)),
                        }
                    })
                } else {
                    serde_json::json!({
                        "market_market_ioc": {
                            "base_size": format!("{:.8}", self.base_size.max(0.00000001)),
                        }
                    })
                }
            }
            OrderRoute::ScanOnly => serde_json::json!({}),
        };

        CoinbaseCreateOrderRequest {
            client_order_id: Uuid::new_v4().to_string(),
            product_id: self.product_id.clone(),
            side: side.to_string(),
            preview_id: preview_id.map(str::to_string),
            order_configuration,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoinbaseProductsResponse {
    #[serde(default)]
    pub products: Vec<CoinbaseProduct>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoinbaseProduct {
    pub product_id: String,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(default)]
    pub volume_24h: Option<String>,
    #[serde(default)]
    pub base_currency_id: Option<String>,
    #[serde(default)]
    pub quote_currency_id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub trading_disabled: bool,
    #[serde(default)]
    pub cancel_only: bool,
    #[serde(default)]
    pub post_only: bool,
    #[serde(default)]
    pub auction_mode: bool,
    #[serde(default)]
    pub product_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoinbaseProductBookResponse {
    #[serde(default)]
    pub pricebook: Option<CoinbasePriceBook>,
    #[serde(default)]
    pub pricebooks: Vec<CoinbasePriceBook>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoinbasePriceBook {
    #[serde(default)]
    pub bids: Vec<CoinbaseBookLevel>,
    #[serde(default)]
    pub asks: Vec<CoinbaseBookLevel>,
    #[serde(default)]
    pub time: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoinbaseBookLevel {
    pub price: String,
    #[serde(default)]
    pub size: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoinbaseMarketTradesResponse {
    #[serde(default)]
    pub trades: Vec<CoinbaseMarketTrade>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoinbaseMarketTrade {
    #[serde(default)]
    pub side: Option<String>,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(default)]
    pub size: Option<String>,
    #[serde(default)]
    pub time: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoinbaseCandlesResponse {
    #[serde(default)]
    pub candles: Vec<CoinbaseCandle>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoinbaseCandle {
    #[serde(default)]
    pub start: Option<String>,
    #[serde(default)]
    pub open: Option<String>,
    #[serde(default)]
    pub high: Option<String>,
    #[serde(default)]
    pub low: Option<String>,
    #[serde(default)]
    pub close: Option<String>,
    #[serde(default)]
    pub volume: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CoinbaseOrderPreview {
    pub success: bool,
    pub preview_id: Option<String>,
    pub raw_status: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CoinbaseSubmittedOrder {
    pub order_id: String,
    pub client_order_id: String,
    pub success: bool,
    pub raw_status: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoinbaseListOrdersResponse {
    #[serde(default)]
    pub orders: Vec<CoinbaseOrderSummary>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoinbaseOrderSummary {
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub client_order_id: Option<String>,
    #[serde(default)]
    pub product_id: Option<String>,
    #[serde(default)]
    pub side: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub limit_price: Option<String>,
    #[serde(default)]
    pub base_size: Option<String>,
    #[serde(default)]
    pub filled_size: Option<String>,
    #[serde(default)]
    pub average_filled_price: Option<String>,
    #[serde(default)]
    pub created_time: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoinbaseCancelResult {
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub failure_reason: Option<String>,
}

#[derive(Debug, Serialize)]
struct CoinbaseBatchCancelRequest {
    order_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct CoinbaseBatchCancelResponse {
    #[serde(default)]
    results: Vec<CoinbaseCancelResult>,
}

#[derive(Debug, Deserialize)]
struct CoinbaseOrderPreviewResponse {
    #[serde(default)]
    success: bool,
    #[serde(default)]
    preview_id: Option<String>,
    #[serde(default)]
    raw_status: Option<String>,
}

#[derive(Debug, Serialize)]
struct CoinbaseJwtClaims {
    iss: String,
    nbf: i64,
    exp: i64,
    sub: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<String>,
}

#[derive(Debug, Serialize)]
struct CoinbaseCreateOrderRequest {
    client_order_id: String,
    product_id: String,
    side: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    preview_id: Option<String>,
    order_configuration: Value,
}

#[derive(Debug, Deserialize)]
struct CoinbaseTickerResponse {
    price: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CoinbaseCreateOrderResponse {
    #[serde(default)]
    success: bool,
    #[serde(default)]
    order_id: Option<String>,
    #[serde(default)]
    success_response: Option<CoinbaseCreateOrderSuccess>,
}

#[derive(Debug, Deserialize)]
struct CoinbaseCreateOrderSuccess {
    #[serde(default)]
    order_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct CoinbaseAccountsResponse {
    #[serde(default)]
    accounts: Vec<CoinbaseAccount>,
}

#[derive(Debug, Clone, Deserialize)]
struct CoinbaseAccount {
    #[serde(default)]
    uuid: String,
    #[serde(default)]
    currency: String,
    #[serde(default)]
    available_balance: CoinbaseBalanceValue,
    #[serde(default)]
    hold: CoinbaseBalanceValue,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct CoinbaseBalanceValue {
    #[serde(default)]
    value: String,
    #[serde(default, rename = "currency")]
    _currency: String,
}

#[derive(Debug, Clone, Deserialize)]
struct CoinbaseOrdersResponse {
    #[serde(default)]
    orders: Vec<CoinbaseOrderSummary>,
}

#[derive(Debug, Clone, Deserialize)]
struct CoinbaseOrderEnvelope {
    #[serde(default)]
    order: Option<CoinbaseOrderSummary>,
}

#[derive(Debug, Clone, Deserialize)]
struct CoinbaseCancelsResponse {
    #[serde(default)]
    results: Vec<CoinbaseCancelResult>,
}

#[derive(Debug, Clone, Deserialize)]
struct CoinbaseFillsResponse {
    #[serde(default)]
    fills: Vec<CoinbaseFill>,
}

#[derive(Debug, Clone, Deserialize)]
struct CoinbaseProductBookResponse {
    #[serde(default)]
    pricebook: Option<CoinbaseProductBook>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoinbaseOrderSummary {
    #[serde(default)]
    pub order_id: String,
    #[serde(default)]
    pub product_id: String,
    #[serde(default)]
    pub side: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub order_type: String,
    #[serde(default)]
    pub average_filled_price: String,
    #[serde(default)]
    pub filled_size: String,
    #[serde(default)]
    pub order_configuration: Value,
    #[serde(default)]
    pub created_time: String,
    #[serde(default)]
    pub last_update_time: String,
}

#[derive(Debug, Clone, Deserialize)]
struct CoinbaseBestBidAskResponse {
    #[serde(default)]
    pricebooks: Vec<CoinbasePriceBook>,
}

#[derive(Debug, Clone, Deserialize)]
struct CoinbasePriceBook {
    #[serde(default)]
    product_id: String,
    #[serde(default)]
    bids: Vec<CoinbaseBestPriceLevel>,
    #[serde(default)]
    asks: Vec<CoinbaseBestPriceLevel>,
}

#[derive(Debug, Clone, Deserialize)]
struct CoinbaseBestPriceLevel {
    #[serde(default)]
    price: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoinbaseTopOfBook {
    pub product_id: String,
    pub best_bid: f64,
    pub best_ask: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoinbasePreviewResult {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub preview_id: Option<String>,
    #[serde(default)]
    pub failure_reason: Option<String>,
    #[serde(default)]
    pub raw: Value,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoinbaseCancelResult {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub failure_reason: String,
    #[serde(default)]
    pub order_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoinbaseFill {
    #[serde(default)]
    pub order_id: String,
    #[serde(default)]
    pub product_id: String,
    #[serde(default)]
    pub price: String,
    #[serde(default)]
    pub size: String,
    #[serde(default)]
    pub commission: String,
    #[serde(default)]
    pub side: String,
    #[serde(default)]
    pub liquidity_indicator: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoinbaseProduct {
    #[serde(default)]
    pub product_id: String,
    #[serde(default)]
    pub base_increment: String,
    #[serde(default)]
    pub quote_increment: String,
    #[serde(default)]
    pub base_min_size: String,
    #[serde(default)]
    pub quote_min_size: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub cancel_only: bool,
    #[serde(default)]
    pub limit_only: bool,
    #[serde(default)]
    pub post_only: bool,
    #[serde(default)]
    pub trading_disabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoinbaseBookLevel {
    #[serde(default)]
    pub price: String,
    #[serde(default)]
    pub size: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoinbaseProductBook {
    #[serde(default)]
    pub product_id: String,
    #[serde(default)]
    pub bids: Vec<CoinbaseBookLevel>,
    #[serde(default)]
    pub asks: Vec<CoinbaseBookLevel>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoinbaseTransactionSummary {
    #[serde(default)]
    pub total_fees: f64,
    #[serde(default)]
    pub maker_fee_rate: Option<String>,
    #[serde(default)]
    pub taker_fee_rate: Option<String>,
    #[serde(default)]
    pub raw: Value,
}

#[derive(Debug, Clone)]
pub struct CoinbaseWsRunConfig {
    pub ws_url: String,
    pub channels: Vec<String>,
    pub product_ids: Vec<String>,
    pub heartbeat_timeout_ms: u64,
    pub resync_on_gap: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum CoinbaseWsEvent {
    Subscribed {
        channel: String,
    },
    Reconnected,
    Heartbeat {
        sequence_num: i64,
        heartbeat_counter: Option<i64>,
        ts: DateTime<Utc>,
    },
    L2 {
        update: CoinbaseL2Update,
    },
    User {
        update: UserOrderEvent,
    },
    Gap {
        product_id: String,
        expected_sequence: i64,
        received_sequence: i64,
    },
    Error {
        message: String,
    },
}

impl CoinbaseOrderSummary {
    pub fn side_enum(&self) -> Option<Side> {
        match self.side.trim().to_ascii_uppercase().as_str() {
            "BUY" => Some(Side::Buy),
            "SELL" => Some(Side::Sell),
            _ => None,
        }
    }

    pub fn created_ts_ms(&self) -> Option<i64> {
        let raw = self.created_time.trim();
        if raw.is_empty() {
            return None;
        }
        DateTime::parse_from_rfc3339(raw)
            .ok()
            .map(|dt| dt.with_timezone(&Utc).timestamp_millis())
    }

    pub fn resting_price(&self) -> Option<f64> {
        parse_order_configuration_number(&self.order_configuration, "limit_price")
            .or_else(|| self.average_filled_price.parse::<f64>().ok())
            .filter(|v| v.is_finite() && *v > 0.0)
    }

    pub fn resting_size(&self) -> Option<f64> {
        parse_order_configuration_number(&self.order_configuration, "base_size")
            .or_else(|| self.filled_size.parse::<f64>().ok())
            .filter(|v| v.is_finite() && *v > 0.0)
    }

    pub fn is_open_like(&self) -> bool {
        matches!(
            self.status.trim().to_ascii_uppercase().as_str(),
            "OPEN" | "PENDING" | "WORKING" | "ACTIVE"
        )
    }
}

fn parse_order_configuration_number(order_configuration: &Value, field: &str) -> Option<f64> {
    if order_configuration.is_null() {
        return None;
    }
    let variants = [
        "limit_limit_gtc",
        "limit_limit_gtd",
        "limit_limit_fok",
        "sor_limit_ioc",
        "market_market_ioc",
        "market_market_fok",
    ];
    for key in variants {
        if let Some(v) = order_configuration.get(key).and_then(|cfg| cfg.get(field)) {
            if let Some(parsed) = parse_f64_from_value(v) {
                return Some(parsed);
            }
        }
    }
    parse_f64_from_value(order_configuration.get(field)?)
}

fn parse_f64_from_value(value: &Value) -> Option<f64> {
    let parsed = match value {
        Value::Number(n) => n.as_f64(),
        Value::String(s) => s.parse::<f64>().ok(),
        _ => None,
    }?;
    if parsed.is_finite() {
        Some(parsed)
    } else {
        None
    }
}

#[derive(Debug, Clone)]
pub struct CoinbaseWalletClient {
    client: Client,
    api_base: String,
    jwt_host_path: String,
    auth_manager: Option<Arc<CoinbaseAuthManager>>,
    api_key: Option<String>,
    api_secret: Option<String>,
    passphrase: Option<String>,
}

impl CoinbaseWalletClient {
    pub fn new(
        api_base: impl Into<String>,
        api_key: Option<String>,
        api_secret: Option<String>,
        passphrase: Option<String>,
    ) -> Self {
        let api_base = api_base.into();
        let jwt_host_path = derive_host_path_for_jwt(&api_base);
        Self {
            client: Client::new(),
            api_base,
            jwt_host_path,
            auth_manager: None,
            api_key,
            api_secret,
            passphrase,
        }
    }

    pub fn new_with_auth_manager(
        api_base: impl Into<String>,
        auth_manager: Arc<CoinbaseAuthManager>,
        passphrase: Option<String>,
    ) -> Self {
        let api_base = api_base.into();
        let jwt_host_path = derive_host_path_for_jwt(&api_base);
        Self {
            client: Client::new(),
            api_base,
            jwt_host_path,
            auth_manager: Some(auth_manager),
            api_key: None,
            api_secret: None,
            passphrase,
        }
    }

    pub fn auth_status(&self) -> Option<CoinbaseAuthStatus> {
        self.auth_manager.as_ref().map(|m| m.status())
    }

    pub fn reload_auth(&self) -> Option<AuthReloadResult> {
        self.auth_manager
            .as_ref()
            .map(|m| m.reload_active_profile())
    }

    pub fn switch_auth_profile(&self, profile_id: &str) -> Option<AuthReloadResult> {
        self.auth_manager
            .as_ref()
            .map(|m| m.switch_profile(profile_id))
    }

    fn credentials(&self) -> PtResult<(String, String)> {
        if let Some(manager) = &self.auth_manager {
            let auth = manager.current_auth()?;
            return Ok((auth.key_name, auth.private_key_pem));
        }

        let key = self
            .api_key
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| PtError::Config("missing venues.coinbase.api_key".to_string()))?;

        let secret = self
            .api_secret
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| PtError::Config("missing venues.coinbase.api_secret".to_string()))?;

        Ok((key.to_string(), secret.to_string()))
    }

    fn endpoint_url(&self, path: &str) -> String {
        format!("{}/{}", self.api_base.trim_end_matches('/'), path)
    }

    fn build_jwt(&self, method: &str, path: &str) -> PtResult<String> {
        let (api_key, api_secret_raw) = self.credentials()?;
        let now = Utc::now().timestamp();
        let uri = format!("{method} {}{path}", self.jwt_host_path);

        let claims = CoinbaseJwtClaims {
            iss: "cdp".to_string(),
            nbf: now,
            exp: now + 120,
            sub: api_key.to_string(),
            uri: Some(uri),
        };

        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(api_key.clone());

        let key = build_ec_encoding_key(&api_secret_raw)?;

        encode(&header, &claims, &key)
            .map_err(|e| PtError::Http(format!("coinbase jwt generation failed: {e}")))
    }

    fn build_ws_jwt(&self) -> PtResult<String> {
        let (api_key, api_secret_raw) = self.credentials()?;
        let now = Utc::now().timestamp();

        let claims = CoinbaseJwtClaims {
            iss: "cdp".to_string(),
            nbf: now,
            exp: now + 120,
            sub: api_key.to_string(),
            uri: None,
        };

        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(api_key.clone());

        let key = build_ec_encoding_key(&api_secret_raw)?;

        encode(&header, &claims, &key)
            .map_err(|e| PtError::Http(format!("coinbase ws jwt generation failed: {e}")))
    }

    pub fn auth_token_self_test(&self) -> PtResult<(String, String)> {
        let rest = self.build_jwt("GET", "/accounts")?;
        let ws = self.build_ws_jwt()?;
        Ok((rest, ws))
    }

    pub async fn probe_authenticated_accounts(&self) -> PtResult<usize> {
        let accounts = self.fetch_accounts().await?;
        Ok(accounts.len())
    }

    fn signed_headers(&self, method: &str, path: &str) -> PtResult<HeaderMap> {
        let token = self.build_jwt(method, path)?;
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {token}"))
                .map_err(|e| PtError::Http(e.to_string()))?,
        );
        if let Some(passphrase) = &self.passphrase {
            if !passphrase.trim().is_empty() {
                headers.insert(
                    "CB-ACCESS-PASSPHRASE",
                    HeaderValue::from_str(passphrase).map_err(|e| PtError::Http(e.to_string()))?,
                );
            }
        }
        Ok(headers)
    }

    async fn fetch_accounts(&self) -> PtResult<Vec<CoinbaseAccount>> {
        self.credentials()?;
        let path = "/accounts";
        let url = self.endpoint_url(path.trim_start_matches('/'));
        let headers = self.signed_headers("GET", path)?;

        let resp = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        let status = resp.status();
        let body = resp
            .text()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;

        if !status.is_success() {
            return Err(PtError::Http(format!(
                "coinbase accounts failed status={} body={body}",
                status
            )));
        }

        let parsed: CoinbaseAccountsResponse =
            serde_json::from_str(&body).map_err(|e| PtError::Serde(e.to_string()))?;
        Ok(parsed.accounts)
    }

    pub async fn fetch_open_orders(&self) -> PtResult<Vec<CoinbaseOrderSummary>> {
        self.credentials()?;
        let path = "/orders/historical/batch?order_status=OPEN";
        let url = self.endpoint_url(path.trim_start_matches('/'));
        let headers = self.signed_headers("GET", "/orders/historical/batch")?;

        let resp = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        let status = resp.status();
        let body = resp
            .text()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;

        if !status.is_success() {
            return Err(PtError::Http(format!(
                "coinbase open orders failed status={} body={body}",
                status
            )));
        }

        let parsed: CoinbaseOrdersResponse =
            serde_json::from_str(&body).map_err(|e| PtError::Serde(e.to_string()))?;
        Ok(parsed.orders)
    }

    pub async fn fetch_ticker_price(&self, product: &str) -> PtResult<f64> {
        self.credentials()?;
        let path = format!("/products/{product}/ticker");
        let url = self.endpoint_url(path.trim_start_matches('/'));
        let headers = self.signed_headers("GET", &path)?;

        let resp = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        let status = resp.status();
        let body = resp
            .text()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;

        if !status.is_success() {
            return Err(PtError::Http(format!(
                "coinbase ticker failed status={} body={body}",
                status
            )));
        }

        let parsed: CoinbaseTickerResponse =
            serde_json::from_str(&body).map_err(|e| PtError::Serde(e.to_string()))?;
        parsed
            .price
            .and_then(|v| v.parse::<f64>().ok())
            .filter(|p| p.is_finite() && *p > 0.0)
            .ok_or_else(|| PtError::Http(format!("coinbase ticker missing price body={body}")))
    }

    pub async fn fetch_top_of_book(&self, product: &str) -> PtResult<CoinbaseTopOfBook> {
        self.credentials()?;
        let path = format!("/best_bid_ask?product_ids={product}");
        let url = self.endpoint_url(path.trim_start_matches('/'));
        let headers = self.signed_headers("GET", "/best_bid_ask")?;

        let resp = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        let status = resp.status();
        let body = resp
            .text()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;

        if !status.is_success() {
            return Err(PtError::Http(format!(
                "coinbase best_bid_ask failed status={} body={body}",
                status
            )));
        }

        let parsed: CoinbaseBestBidAskResponse =
            serde_json::from_str(&body).map_err(|e| PtError::Serde(e.to_string()))?;
        let Some(book) = parsed
            .pricebooks
            .into_iter()
            .find(|b| b.product_id.eq_ignore_ascii_case(product))
        else {
            return Err(PtError::Http(format!(
                "coinbase best_bid_ask missing product {product}"
            )));
        };

        let best_bid = book
            .bids
            .first()
            .and_then(|x| x.price.parse::<f64>().ok())
            .unwrap_or(0.0);
        let best_ask = book
            .asks
            .first()
            .and_then(|x| x.price.parse::<f64>().ok())
            .unwrap_or(0.0);
        if best_bid <= 0.0 || best_ask <= 0.0 || best_bid >= best_ask {
            return Err(PtError::Http(format!(
                "invalid top of book for {product}: bid={best_bid} ask={best_ask}"
            )));
        }

        Ok(CoinbaseTopOfBook {
            product_id: product.to_string(),
            best_bid,
            best_ask,
        })
    }

    pub async fn fetch_wallet_balances(&self, products: &[String]) -> PtResult<Vec<WalletBalance>> {
        let accounts = self.fetch_accounts().await?;
        let mut out = Vec::new();

        for account in accounts {
            let available = account
                .available_balance
                .value
                .parse::<f64>()
                .unwrap_or(0.0);
            let hold = account.hold.value.parse::<f64>().unwrap_or(0.0);
            let total_units = available + hold;
            if total_units <= 0.0 {
                continue;
            }

            let asset = account.currency.to_ascii_uppercase();
            let usd_value = if asset == "USD" || asset == "USDC" {
                total_units
            } else {
                let preferred = format!("{asset}-USD");
                let product = products
                    .iter()
                    .find(|p| p.eq_ignore_ascii_case(&preferred))
                    .cloned()
                    .unwrap_or(preferred);
                match self.fetch_ticker_price(&product).await {
                    Ok(px) => total_units * px,
                    Err(_) => 0.0,
                }
            };

            out.push(WalletBalance {
                venue: Venue::Coinbase,
                account_id: account.uuid.clone(),
                asset,
                available,
                hold,
                usd_value,
                ts: Utc::now(),
            });
        }

        out.sort_by(|a, b| {
            b.usd_value
                .partial_cmp(&a.usd_value)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Ok(out)
    }

    pub async fn place_limit_post_only(
        &self,
        product_id: &str,
        side: Side,
        base_size: f64,
        limit_price: f64,
    ) -> PtResult<ExecutionReport> {
        self.credentials()?;
        if base_size <= 0.0 || limit_price <= 0.0 {
            return Err(PtError::InvalidInput(
                "limit post-only requires positive base_size and limit_price".to_string(),
            ));
        }
        let side_str = match side {
            Side::Buy => "BUY",
            Side::Sell => "SELL",
        };

        let body = serde_json::json!({
            "client_order_id": Uuid::new_v4().to_string(),
            "product_id": product_id,
            "side": side_str,
            "order_configuration": {
                "limit_limit_gtc": {
                    "base_size": format!("{base_size:.8}"),
                    "limit_price": format!("{limit_price:.8}"),
                    "post_only": true
                }
            }
        });

        let path = "/orders";
        let url = self.endpoint_url(path.trim_start_matches('/'));
        let headers = self.signed_headers("POST", path)?;

        let resp = self
            .client
            .post(url)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        let status = resp.status();
        let raw = resp
            .text()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        if !status.is_success() {
            return Err(PtError::Http(format!(
                "coinbase post-only order failed status={} body={raw}",
                status
            )));
        }

        let parsed: CoinbaseCreateOrderResponse =
            serde_json::from_str(&raw).map_err(|e| PtError::Serde(e.to_string()))?;
        let order_id = parsed
            .order_id
            .or_else(|| parsed.success_response.and_then(|v| v.order_id))
            .unwrap_or_else(|| format!("cb-order-{}", Utc::now().timestamp_millis()));
        let exec_status = if parsed.success {
            ExecutionStatus::New
        } else {
            ExecutionStatus::Rejected
        };

        Ok(ExecutionReport {
            venue: Venue::Coinbase,
            order_id,
            market_id: Some(product_id.to_string()),
            status: exec_status,
            side,
            filled_qty: 0.0,
            avg_px: limit_price,
            ts: Utc::now(),
            details: Some(format!(
                "post_only=true response_status={} body={raw}",
                status
            )),
        })
    }

    pub async fn preview_order_post_only(
        &self,
        product_id: &str,
        side: Side,
        base_size: f64,
        limit_price: f64,
    ) -> PtResult<CoinbasePreviewResult> {
        self.credentials()?;
        let side_str = match side {
            Side::Buy => "BUY",
            Side::Sell => "SELL",
        };

        let body = serde_json::json!({
            "product_id": product_id,
            "side": side_str,
            "order_configuration": {
                "limit_limit_gtc": {
                    "base_size": format!("{base_size:.8}"),
                    "limit_price": format!("{limit_price:.8}"),
                    "post_only": true
                }
            }
        });

        let path = "/orders/preview";
        let url = self.endpoint_url(path.trim_start_matches('/'));
        let headers = self.signed_headers("POST", path)?;
        let resp = self
            .client
            .post(url)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        let status = resp.status();
        let raw = resp
            .text()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        if !status.is_success() {
            return Err(PtError::Http(format!(
                "coinbase preview failed status={} body={raw}",
                status
            )));
        }
        let value: Value = serde_json::from_str(&raw).map_err(|e| PtError::Serde(e.to_string()))?;
        let preview_id = value
            .get("preview_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let failure_reason = value
            .get("preview_failure_reason")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .or_else(|| {
                value
                    .get("error")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            });
        let success = failure_reason.is_none();
        Ok(CoinbasePreviewResult {
            success,
            preview_id,
            failure_reason,
            raw: value,
        })
    }

    pub async fn create_order_post_only(
        &self,
        product_id: &str,
        side: Side,
        base_size: f64,
        limit_price: f64,
    ) -> PtResult<ExecutionReport> {
        self.place_limit_post_only(product_id, side, base_size, limit_price)
            .await
    }

    pub async fn edit_order(
        &self,
        order_id: &str,
        new_price: f64,
        new_size: f64,
    ) -> PtResult<ExecutionReport> {
        self.credentials()?;
        let path = "/orders/edit";
        let url = self.endpoint_url(path.trim_start_matches('/'));
        let headers = self.signed_headers("POST", path)?;
        let body = serde_json::json!({
            "order_id": order_id,
            "price": format!("{new_price:.8}"),
            "size": format!("{new_size:.8}")
        });

        let resp = self
            .client
            .post(url)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        let status = resp.status();
        let raw = resp
            .text()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        if !status.is_success() {
            return Err(PtError::Http(format!(
                "coinbase edit order failed status={} body={raw}",
                status
            )));
        }

        Ok(ExecutionReport {
            venue: Venue::Coinbase,
            order_id: order_id.to_string(),
            market_id: None,
            status: ExecutionStatus::New,
            side: Side::Buy,
            filled_qty: 0.0,
            avg_px: new_price,
            ts: Utc::now(),
            details: Some(format!("edited size={} body={}", new_size, raw)),
        })
    }

    pub async fn cancel_orders_batch(
        &self,
        order_ids: &[String],
    ) -> PtResult<Vec<CoinbaseCancelResult>> {
        self.credentials()?;
        if order_ids.is_empty() {
            return Ok(Vec::new());
        }
        let path = "/orders/batch_cancel";
        let url = self.endpoint_url(path.trim_start_matches('/'));
        let headers = self.signed_headers("POST", path)?;
        let body = serde_json::json!({ "order_ids": order_ids });
        let resp = self
            .client
            .post(url)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        let status = resp.status();
        let raw = resp
            .text()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        if !status.is_success() {
            return Err(PtError::Http(format!(
                "coinbase batch_cancel failed status={} body={raw}",
                status
            )));
        }
        let parsed: CoinbaseCancelsResponse =
            serde_json::from_str(&raw).map_err(|e| PtError::Serde(e.to_string()))?;
        Ok(parsed.results)
    }

    pub async fn get_order(&self, order_id: &str) -> PtResult<Option<CoinbaseOrderSummary>> {
        self.credentials()?;
        let path = format!("/orders/historical/{order_id}");
        let url = self.endpoint_url(path.trim_start_matches('/'));
        let headers = self.signed_headers("GET", &path)?;
        let resp = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        let status = resp.status();
        let raw = resp
            .text()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        if !status.is_success() {
            return Err(PtError::Http(format!(
                "coinbase get order failed status={} body={raw}",
                status
            )));
        }
        let parsed: CoinbaseOrderEnvelope =
            serde_json::from_str(&raw).map_err(|e| PtError::Serde(e.to_string()))?;
        Ok(parsed.order)
    }

    pub async fn list_fills(
        &self,
        product_id: Option<&str>,
        order_id: Option<&str>,
    ) -> PtResult<Vec<CoinbaseFill>> {
        self.credentials()?;
        let mut query = Vec::new();
        if let Some(p) = product_id {
            query.push(format!("product_id={p}"));
        }
        if let Some(o) = order_id {
            query.push(format!("order_id={o}"));
        }
        let path = if query.is_empty() {
            "/orders/historical/fills".to_string()
        } else {
            format!("/orders/historical/fills?{}", query.join("&"))
        };
        let url = self.endpoint_url(path.trim_start_matches('/'));
        let headers = self.signed_headers("GET", "/orders/historical/fills")?;
        let resp = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        let status = resp.status();
        let raw = resp
            .text()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        if !status.is_success() {
            return Err(PtError::Http(format!(
                "coinbase list fills failed status={} body={raw}",
                status
            )));
        }
        let parsed: CoinbaseFillsResponse =
            serde_json::from_str(&raw).map_err(|e| PtError::Serde(e.to_string()))?;
        Ok(parsed.fills)
    }

    pub async fn get_product(&self, product_id: &str) -> PtResult<CoinbaseProduct> {
        self.credentials()?;
        let path = format!("/products/{product_id}");
        let url = self.endpoint_url(path.trim_start_matches('/'));
        let headers = self.signed_headers("GET", &path)?;
        let resp = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        let status = resp.status();
        let raw = resp
            .text()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        if !status.is_success() {
            return Err(PtError::Http(format!(
                "coinbase get product failed status={} body={raw}",
                status
            )));
        }
        serde_json::from_str(&raw).map_err(|e| PtError::Serde(e.to_string()))
    }

    pub async fn get_product_book(
        &self,
        product_id: &str,
        limit: usize,
    ) -> PtResult<CoinbaseProductBook> {
        self.credentials()?;
        let path = format!(
            "/product_book?product_id={product_id}&limit={}",
            limit.max(1)
        );
        let url = self.endpoint_url(path.trim_start_matches('/'));
        let headers = self.signed_headers("GET", "/product_book")?;
        let resp = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        let status = resp.status();
        let raw = resp
            .text()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        if !status.is_success() {
            return Err(PtError::Http(format!(
                "coinbase product_book failed status={} body={raw}",
                status
            )));
        }

        let parsed: CoinbaseProductBookResponse =
            serde_json::from_str(&raw).map_err(|e| PtError::Serde(e.to_string()))?;
        parsed.pricebook.ok_or_else(|| {
            PtError::Http(format!(
                "coinbase product_book missing pricebook for product {product_id}"
            ))
        })
    }

    pub async fn get_best_bid_ask(&self, products: &[String]) -> PtResult<Vec<CoinbaseTopOfBook>> {
        let mut out = Vec::new();
        for p in products {
            if let Ok(top) = self.fetch_top_of_book(p).await {
                out.push(top);
            }
        }
        Ok(out)
    }

    pub async fn get_transaction_summary(&self) -> PtResult<CoinbaseTransactionSummary> {
        self.credentials()?;
        let path = "/transaction_summary";
        let url = self.endpoint_url(path.trim_start_matches('/'));
        let headers = self.signed_headers("GET", path)?;
        let resp = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        let status = resp.status();
        let raw = resp
            .text()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?;
        if !status.is_success() {
            return Err(PtError::Http(format!(
                "coinbase transaction summary failed status={} body={raw}",
                status
            )));
        }
        let value: Value = serde_json::from_str(&raw).map_err(|e| PtError::Serde(e.to_string()))?;
        let maker_fee_rate = value
            .get("fee_tier")
            .and_then(|f| f.get("maker_fee_rate"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let taker_fee_rate = value
            .get("fee_tier")
            .and_then(|f| f.get("taker_fee_rate"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        Ok(CoinbaseTransactionSummary {
            total_fees: value
                .get("total_fees")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0),
            maker_fee_rate,
            taker_fee_rate,
            raw: value,
        })
    }

    pub fn product_book_to_state(book: &CoinbaseProductBook) -> CoinbaseOrderBookState {
        let mut bids: Vec<(f64, f64)> = book
            .bids
            .iter()
            .filter_map(|l| {
                let px = l.price.parse::<f64>().ok()?;
                let sz = l.size.parse::<f64>().ok()?;
                if px > 0.0 && sz > 0.0 {
                    Some((px, sz))
                } else {
                    None
                }
            })
            .collect();
        bids.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        let mut asks: Vec<(f64, f64)> = book
            .asks
            .iter()
            .filter_map(|l| {
                let px = l.price.parse::<f64>().ok()?;
                let sz = l.size.parse::<f64>().ok()?;
                if px > 0.0 && sz > 0.0 {
                    Some((px, sz))
                } else {
                    None
                }
            })
            .collect();
        asks.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        CoinbaseOrderBookState {
            product_id: book.product_id.clone(),
            sequence_num: 0,
            bids,
            asks,
            last_event_ts: Some(Utc::now()),
        }
    }

    pub fn spawn_ws_event_loop(
        &self,
        cfg: CoinbaseWsRunConfig,
    ) -> PtResult<mpsc::Receiver<CoinbaseWsEvent>> {
        if cfg.channels.is_empty() {
            return Err(PtError::Config(
                "coinbase ws config must include at least one channel".to_string(),
            ));
        }

        let (tx, rx) = mpsc::channel(4096);
        let client = self.clone();
        tokio::spawn(async move {
            let mut seq_by_product: HashMap<String, i64> = HashMap::new();
            let has_heartbeat_channel = cfg
                .channels
                .iter()
                .any(|c| c.eq_ignore_ascii_case("heartbeats"));
            let heartbeat_timeout = Duration::from_millis(cfg.heartbeat_timeout_ms.max(1_000));
            let reconnect_deadline =
                Duration::from_millis(cfg.heartbeat_timeout_ms.max(1_000).saturating_mul(2));
            const MAX_TIMEOUT_STREAK: u32 = 3;

            loop {
                let connection_generation = client
                    .auth_manager
                    .as_ref()
                    .map(|m| m.generation())
                    .unwrap_or(0);
                let ws = connect_async(&cfg.ws_url).await;
                let Ok((stream, _)) = ws else {
                    let _ = tx
                        .send(CoinbaseWsEvent::Error {
                            message: "coinbase ws connect failed".to_string(),
                        })
                        .await;
                    sleep(Duration::from_secs(1)).await;
                    continue;
                };
                let _ = tx.send(CoinbaseWsEvent::Reconnected).await;
                let (mut write, mut read) = stream.split();
                let has_auth = client.credentials().is_ok();
                let mut last_message_at = Instant::now();
                let mut last_heartbeat_at = Instant::now();
                let mut timeout_streak: u32 = 0;

                for channel in &cfg.channels {
                    if channel.eq_ignore_ascii_case("user") && !has_auth {
                        let _ = tx
                            .send(CoinbaseWsEvent::Error {
                                message:
                                    "coinbase ws user channel skipped: missing api credentials"
                                        .to_string(),
                            })
                            .await;
                        continue;
                    }
                    let mut sub = serde_json::json!({
                        "type": "subscribe",
                        "channel": channel,
                    });
                    if !cfg.product_ids.is_empty() {
                        sub["product_ids"] = serde_json::json!(cfg.product_ids);
                    }

                    if let Ok(jwt) = client.build_ws_jwt() {
                        sub["jwt"] = serde_json::json!(jwt);
                    }

                    if let Err(e) = write.send(Message::Text(sub.to_string())).await {
                        let _ = tx
                            .send(CoinbaseWsEvent::Error {
                                message: format!("coinbase ws subscribe send failed: {e}"),
                            })
                            .await;
                        break;
                    }

                    let _ = tx
                        .send(CoinbaseWsEvent::Subscribed {
                            channel: channel.to_string(),
                        })
                        .await;
                }

                let mut disconnect = false;
                while !disconnect {
                    let latest_generation = client
                        .auth_manager
                        .as_ref()
                        .map(|m| m.generation())
                        .unwrap_or(connection_generation);
                    if latest_generation != connection_generation {
                        let _ = tx
                            .send(CoinbaseWsEvent::Error {
                                message: "coinbase ws auth changed, reconnecting".to_string(),
                            })
                            .await;
                        disconnect = true;
                        continue;
                    }
                    let next = timeout(heartbeat_timeout, read.next()).await;

                    match next {
                        Ok(Some(Ok(Message::Text(text)))) => {
                            last_message_at = Instant::now();
                            timeout_streak = 0;
                            let events =
                                parse_ws_events(&text, &mut seq_by_product, cfg.resync_on_gap);
                            for ev in events {
                                if let CoinbaseWsEvent::Heartbeat { .. } = ev {
                                    last_heartbeat_at = Instant::now();
                                }
                                if tx.send(ev).await.is_err() {
                                    return;
                                }
                            }
                        }
                        Ok(Some(Ok(Message::Ping(payload)))) => {
                            last_message_at = Instant::now();
                            timeout_streak = 0;
                            if let Err(e) = write.send(Message::Pong(payload)).await {
                                let _ = tx
                                    .send(CoinbaseWsEvent::Error {
                                        message: format!("coinbase ws pong failed: {e}"),
                                    })
                                    .await;
                                disconnect = true;
                            }
                        }
                        Ok(Some(Ok(Message::Close(_)))) | Ok(None) => {
                            let _ = tx
                                .send(CoinbaseWsEvent::Error {
                                    message: "coinbase ws closed by remote".to_string(),
                                })
                                .await;
                            disconnect = true;
                        }
                        Ok(Some(Ok(_))) => {
                            last_message_at = Instant::now();
                            timeout_streak = 0;
                        }
                        Ok(Some(Err(e))) => {
                            let _ = tx
                                .send(CoinbaseWsEvent::Error {
                                    message: format!("coinbase ws read error: {e}"),
                                })
                                .await;
                            disconnect = true;
                        }
                        Err(_) => {
                            timeout_streak = timeout_streak.saturating_add(1);
                            let _ = tx
                                .send(CoinbaseWsEvent::Error {
                                    message: format!(
                                        "coinbase ws read timeout (streak={})",
                                        timeout_streak
                                    ),
                                })
                                .await;
                            // Keepalive ping before deciding to reconnect.
                            if let Err(e) = write.send(Message::Ping(Vec::new().into())).await {
                                let _ = tx
                                    .send(CoinbaseWsEvent::Error {
                                        message: format!("coinbase ws ping failed: {e}"),
                                    })
                                    .await;
                                disconnect = true;
                                continue;
                            }

                            let liveness_age = if has_heartbeat_channel {
                                last_heartbeat_at.elapsed()
                            } else {
                                last_message_at.elapsed()
                            };
                            if timeout_streak >= MAX_TIMEOUT_STREAK
                                && liveness_age > reconnect_deadline
                            {
                                let _ = tx
                                    .send(CoinbaseWsEvent::Error {
                                        message: "coinbase ws heartbeat timeout".to_string(),
                                    })
                                    .await;
                                disconnect = true;
                            }
                            continue;
                        }
                    }
                }

                sleep(Duration::from_secs(1)).await;
            }
        });

        Ok(rx)
    }
}

fn parse_ws_events(
    text: &str,
    seq_by_product: &mut HashMap<String, i64>,
    emit_gaps: bool,
) -> Vec<CoinbaseWsEvent> {
    let mut out = Vec::new();
    let value: Value = match serde_json::from_str(text) {
        Ok(v) => v,
        Err(e) => {
            out.push(CoinbaseWsEvent::Error {
                message: format!("coinbase ws json parse error: {e}"),
            });
            return out;
        }
    };

    let top_type = value
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_ascii_lowercase();
    let channel = value
        .get("channel")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_ascii_lowercase();
    let sequence_num = value
        .get("sequence_num")
        .and_then(Value::as_i64)
        .unwrap_or(0);
    let ts = parse_datetime(value.get("timestamp")).unwrap_or_else(Utc::now);

    if top_type == "subscriptions" || channel == "subscriptions" {
        out.push(CoinbaseWsEvent::Subscribed {
            channel: "subscriptions".to_string(),
        });
        return out;
    }

    if channel == "heartbeats" {
        let heartbeat_counter = value
            .get("events")
            .and_then(Value::as_array)
            .and_then(|events| events.first())
            .and_then(|event| event.get("heartbeat_counter"))
            .and_then(Value::as_str)
            .and_then(|v| v.parse::<i64>().ok());
        out.push(CoinbaseWsEvent::Heartbeat {
            sequence_num,
            heartbeat_counter,
            ts,
        });
        return out;
    }

    let is_level2 = channel == "l2_data" || channel == "level2";
    let is_user = channel == "user";
    if !is_level2 && !is_user {
        return out;
    }

    let Some(events) = value.get("events").and_then(Value::as_array) else {
        return out;
    };

    for event in events {
        if is_level2 {
            let product_id = event
                .get("product_id")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();
            if product_id.is_empty() {
                continue;
            }

            if let Some(last_seq) = seq_by_product.get(&product_id).copied() {
                let expected = last_seq + 1;
                if sequence_num > expected && emit_gaps {
                    out.push(CoinbaseWsEvent::Gap {
                        product_id: product_id.clone(),
                        expected_sequence: expected,
                        received_sequence: sequence_num,
                    });
                }
                if sequence_num <= last_seq {
                    continue;
                }
            }
            seq_by_product.insert(product_id.clone(), sequence_num);

            let Some(updates) = event.get("updates").and_then(Value::as_array) else {
                continue;
            };
            for update in updates {
                let price_level = parse_f64(update.get("price_level"));
                let new_quantity = parse_f64(update.get("new_quantity"));
                if !price_level.is_finite() || price_level <= 0.0 || !new_quantity.is_finite() {
                    continue;
                }

                let side = update
                    .get("side")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_ascii_lowercase();
                let event_time = parse_datetime(update.get("event_time")).unwrap_or(ts);

                out.push(CoinbaseWsEvent::L2 {
                    update: CoinbaseL2Update {
                        sequence_num,
                        product_id: product_id.clone(),
                        side,
                        price_level,
                        new_quantity,
                        event_time,
                    },
                });
            }
        } else if is_user {
            let Some(orders) = event.get("orders").and_then(Value::as_array) else {
                continue;
            };

            for order in orders {
                let order_id = order
                    .get("order_id")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();
                if order_id.is_empty() {
                    continue;
                }
                let product_id = order
                    .get("product_id")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();
                let status = order
                    .get("status")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();
                let side = order
                    .get("order_side")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();
                let post_only = parse_bool(order.get("post_only"));
                let avg_price = parse_f64(order.get("avg_price"));
                let filled_qty = parse_f64(order.get("cumulative_quantity"));
                let total_fees = parse_f64(order.get("total_fees"));
                let order_ts =
                    parse_datetime(order.get("creation_time")).unwrap_or_else(|| ts.to_owned());

                out.push(CoinbaseWsEvent::User {
                    update: UserOrderEvent {
                        order_id,
                        product_id,
                        status,
                        side,
                        post_only,
                        avg_price,
                        filled_qty,
                        total_fees,
                        ts: order_ts,
                    },
                });
            }
        }
    }

    out
}

fn parse_datetime(value: Option<&Value>) -> Option<DateTime<Utc>> {
    let raw = value.and_then(Value::as_str)?;
    if let Ok(dt) = DateTime::parse_from_rfc3339(raw) {
        return Some(dt.with_timezone(&Utc));
    }
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(raw, "%Y-%m-%d %H:%M:%S%.f") {
        return Some(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc));
    }
    None
}

fn parse_f64(value: Option<&Value>) -> f64 {
    match value {
        Some(Value::Number(n)) => n.as_f64().unwrap_or(0.0),
        Some(Value::String(s)) => s.parse::<f64>().unwrap_or(0.0),
        Some(Value::Bool(v)) => {
            if *v {
                1.0
            } else {
                0.0
            }
        }
        _ => 0.0,
    }
}

fn parse_bool(value: Option<&Value>) -> bool {
    match value {
        Some(Value::Bool(v)) => *v,
        Some(Value::String(s)) => s.eq_ignore_ascii_case("true"),
        Some(Value::Number(n)) => n.as_i64().unwrap_or(0) != 0,
        _ => false,
    }
}

fn normalize_multiline_secret(input: &str) -> String {
    let mut out = input.trim().trim_start_matches('\u{feff}').to_string();
    out = out.replace("\r\n", "\n").replace('\r', "\n");
    out = out.replace("\\r\\n", "\n").replace("\\n", "\n");
    if out.contains("BEGIN ") && !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

fn build_ec_encoding_key(secret_raw: &str) -> PtResult<EncodingKey> {
    let normalized = normalize_multiline_secret(secret_raw);
    if normalized.trim().is_empty() {
        return Err(PtError::Config(
            "invalid coinbase EC private key: empty secret".to_string(),
        ));
    }

    if let Ok(key) = EncodingKey::from_ec_pem(normalized.as_bytes()) {
        return Ok(key);
    }

    if normalized.contains("BEGIN EC PRIVATE KEY") {
        let sec1_key = SecretKey::from_sec1_pem(&normalized).map_err(|e| {
            PtError::Config(format!(
                "invalid coinbase EC private key: SEC1 parse failed ({})",
                redact_secret_error(&e.to_string())
            ))
        })?;
        let pkcs8_pem = sec1_key.to_pkcs8_pem(LineEnding::LF).map_err(|e| {
            PtError::Config(format!(
                "invalid coinbase EC private key: PKCS8 conversion failed ({})",
                redact_secret_error(&e.to_string())
            ))
        })?;
        return EncodingKey::from_ec_pem(pkcs8_pem.as_bytes()).map_err(|e| {
            PtError::Config(format!(
                "invalid coinbase EC private key: {}",
                redact_secret_error(&e.to_string())
            ))
        });
    }

    Err(PtError::Config(
        "invalid coinbase EC private key: unsupported PEM format".to_string(),
    ))
}

fn redact_secret_error(err: &str) -> String {
    err.replace("BEGIN EC PRIVATE KEY", "BEGIN_REDACTED")
        .replace("END EC PRIVATE KEY", "END_REDACTED")
        .replace("BEGIN PRIVATE KEY", "BEGIN_REDACTED")
        .replace("END PRIVATE KEY", "END_REDACTED")
}

fn extract_key_id(key_name: &str) -> String {
    key_name
        .trim()
        .rsplit('/')
        .next()
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn key_suffix(key_id: &str) -> String {
    let trimmed = key_id.trim();
    let chars: Vec<char> = trimmed.chars().collect();
    if chars.len() <= 8 {
        return trimmed.to_string();
    }
    chars[chars.len() - 8..].iter().collect()
}

fn fetch_secret_string_blocking(secret_id: &str) -> PtResult<String> {
    let secret_id = secret_id.trim().to_string();
    if secret_id.is_empty() {
        return Err(PtError::Config(
            "coinbase cdp_secret_id must not be empty".to_string(),
        ));
    }

    async fn fetch(secret_id: String) -> PtResult<String> {
        let cfg = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .load()
            .await;
        let client = aws_sdk_secretsmanager::Client::new(&cfg);
        let out = client
            .get_secret_value()
            .secret_id(secret_id.clone())
            .send()
            .await
            .map_err(|e| {
                PtError::Http(format!(
                    "coinbase secrets manager get_secret_value failed for '{}': {e}",
                    secret_id
                ))
            })?;
        let secret = out.secret_string().ok_or_else(|| {
            PtError::Config(format!(
                "coinbase secrets manager secret '{}' has no secret_string payload",
                secret_id
            ))
        })?;
        Ok(secret.to_string())
    }

    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        tokio::task::block_in_place(|| handle.block_on(fetch(secret_id)))
    } else {
        let rt = tokio::runtime::Runtime::new().map_err(|e| PtError::Io(e.to_string()))?;
        rt.block_on(fetch(secret_id))
    }
}

fn derive_host_path_for_jwt(api_base: &str) -> String {
    if let Ok(url) = Url::parse(api_base) {
        if let Some(host) = url.host_str() {
            return format!("{}{}", host, url.path().trim_end_matches('/'));
        }
    }
    "api.coinbase.com/api/v3/brokerage".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pt_core::{CoinbaseAuthConfig, CoinbaseWsConfig};

    fn fixture_coinbase_config() -> CoinbaseConfig {
        CoinbaseConfig {
            api_base: "https://api.coinbase.com/api/v3/brokerage".to_string(),
            api_key: None,
            api_secret: None,
            passphrase: None,
            auth: CoinbaseAuthConfig::default(),
            products: vec!["BTC-USD".to_string()],
            hedge_threshold_usd: 5.0,
            hedge_max_slippage_bps: 10.0,
            ws: CoinbaseWsConfig::default(),
        }
    }

    #[test]
    fn key_id_extraction_uses_name_tail_segment() {
        let name = "organizations/org-id/apiKeys/a5602f09-8a12-422a-b55d-b6981c5a0776";
        let key_id = extract_key_id(name);
        assert_eq!(key_id, "a5602f09-8a12-422a-b55d-b6981c5a0776");
        assert_eq!(key_suffix(&key_id), "1c5a0776");
    }

    #[test]
    fn normalize_secret_unescapes_newlines() {
        let raw = "-----BEGIN KEY-----\\nline\\n-----END KEY-----";
        let normalized = normalize_multiline_secret(raw);
        assert!(normalized.contains('\n'));
        assert!(!normalized.contains("\\n"));
    }

    #[test]
    fn resolve_profile_from_cdp_key_file() {
        let temp_path = std::env::temp_dir().join(format!(
            "pt-coinbase-cdp-{}.json",
            Utc::now().timestamp_nanos_opt().unwrap_or_default()
        ));
        let payload = serde_json::json!({
            "name": "organizations/test-org/apiKeys/a5602f09-8a12-422a-b55d-b6981c5a0776",
            "privateKey": "-----BEGIN EC PRIVATE KEY-----\\nabc\\n-----END EC PRIVATE KEY-----"
        });
        fs::write(
            &temp_path,
            serde_json::to_vec(&payload).expect("serialize payload"),
        )
        .expect("write temp file");

        let mut cfg = fixture_coinbase_config();
        cfg.auth.active_profile = Some("primary".to_string());
        cfg.auth.profiles.insert(
            "primary".to_string(),
            CoinbaseAuthProfileConfig {
                cdp_key_file: Some(temp_path.to_string_lossy().to_string()),
                cdp_secret_id: None,
                expected_key_id: Some("a5602f09-8a12-422a-b55d-b6981c5a0776".to_string()),
                strategy_tags: vec![],
            },
        );

        let manager = CoinbaseAuthManager::new(cfg, EngineMode::Live).expect("manager");
        let auth = manager.current_auth().expect("auth");
        assert_eq!(auth.profile_id.as_deref(), Some("primary"));
        assert_eq!(auth.key_id, "a5602f09-8a12-422a-b55d-b6981c5a0776");
        assert_eq!(auth.source, CoinbaseAuthSource::CdpKeyFile);
        assert!(auth.private_key_pem.contains('\n'));

        let _ = fs::remove_file(temp_path);
    }

    #[test]
    fn live_profile_expected_key_id_mismatch_fails() {
        let temp_path = std::env::temp_dir().join(format!(
            "pt-coinbase-cdp-mismatch-{}.json",
            Utc::now().timestamp_nanos_opt().unwrap_or_default()
        ));
        let payload = serde_json::json!({
            "name": "organizations/test-org/apiKeys/a5602f09-8a12-422a-b55d-b6981c5a0776",
            "privateKey": "-----BEGIN EC PRIVATE KEY-----\\nabc\\n-----END EC PRIVATE KEY-----"
        });
        fs::write(
            &temp_path,
            serde_json::to_vec(&payload).expect("serialize payload"),
        )
        .expect("write temp file");

        let mut cfg = fixture_coinbase_config();
        cfg.auth.active_profile = Some("primary".to_string());
        cfg.auth.profiles.insert(
            "primary".to_string(),
            CoinbaseAuthProfileConfig {
                cdp_key_file: Some(temp_path.to_string_lossy().to_string()),
                cdp_secret_id: None,
                expected_key_id: Some("different-key-id".to_string()),
                strategy_tags: vec![],
            },
        );

        let err =
            CoinbaseAuthManager::new(cfg, EngineMode::Live).expect_err("expected mismatch to fail");
        assert!(
            err.to_string().contains("key_id mismatch"),
            "unexpected error: {}",
            err
        );
        let _ = fs::remove_file(temp_path);
    }
}
