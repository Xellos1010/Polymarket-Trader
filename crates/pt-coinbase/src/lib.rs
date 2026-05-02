use async_trait::async_trait;
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use parking_lot::RwLock;
use pt_core::{
    Asset, ExecutionReport, ExecutionStatus, OrderRoute, PtError, PtResult, Side, Venue,
};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CACHE_CONTROL, CONTENT_TYPE};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
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
        products
            .products
            .sort_by(|a, b| a.product_id.cmp(&b.product_id));
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

    pub async fn list_orders(
        &self,
        product_id: Option<&str>,
    ) -> PtResult<Vec<CoinbaseOrderSummary>> {
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
