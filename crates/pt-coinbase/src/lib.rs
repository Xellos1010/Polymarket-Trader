use async_trait::async_trait;
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use parking_lot::RwLock;
use pt_core::{Asset, ExecutionReport, ExecutionStatus, PtError, PtResult, Side, Venue};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct HedgeIntent {
    pub asset: Asset,
    pub side: Side,
    pub usd_notional: f64,
    pub max_slippage_bps: f64,
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

#[derive(Debug, Clone)]
pub struct CoinbaseSpotHedger {
    client: Client,
    api_base: String,
    jwt_host_path: String,
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
            api_key,
            api_secret,
            passphrase,
        }
    }

    fn credentials(&self) -> PtResult<(&str, &str)> {
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

fn normalize_multiline_secret(input: &str) -> String {
    input.replace("\\n", "\n")
}

fn derive_host_path_for_jwt(api_base: &str) -> String {
    if let Ok(url) = Url::parse(api_base) {
        if let Some(host) = url.host_str() {
            return format!("{}{}", host, url.path().trim_end_matches('/'));
        }
    }
    "api.coinbase.com/api/v3/brokerage".to_string()
}
