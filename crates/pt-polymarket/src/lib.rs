use std::collections::HashMap;
use std::str::FromStr;

use alloy::signers::local::PrivateKeySigner;
use alloy::signers::Signer as _;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use parking_lot::RwLock;
use polymarket_client_sdk::auth::state::Authenticated;
use polymarket_client_sdk::auth::Normal;
use polymarket_client_sdk::clob::types::{OrderType, Side as PolySide};
use polymarket_client_sdk::clob::{Client as PolyClient, Config as PolyConfig};
use polymarket_client_sdk::types::{Decimal, U256};
use pt_core::{ExecutionReport, ExecutionStatus, PtError, PtResult, QuoteIntent, Side, Venue};
use reqwest::Client;
use serde::Deserialize;
use tokio::sync::OnceCell;

pub mod ws;
pub use ws::{BookUpdate, PolymarketWsClient};

#[derive(Debug, Clone)]
pub struct PolymarketClient {
    client: Client,
    clob_api: String,
    clob_ws: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PriceLevel {
    pub price: String,
    pub size: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrderBookResponse {
    pub market: String,
    pub asset_id: String,
    pub timestamp: String,
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
}

#[derive(Debug, Clone, Deserialize)]
struct SpreadResponse {
    spread: String,
}

#[derive(Debug, Clone)]
pub struct BestBook {
    pub market_id: String,
    pub token_id: String,
    pub best_bid: f64,
    pub best_ask: f64,
    pub spread: f64,
    pub ts_ms: i64,
}

impl PolymarketClient {
    pub fn new(clob_api: impl Into<String>, clob_ws: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            clob_api: clob_api.into(),
            clob_ws: clob_ws.into(),
        }
    }

    pub async fn get_book(&self, token_id: &str) -> PtResult<OrderBookResponse> {
        let url = format!("{}/book?token_id={}", self.clob_api, token_id);
        self.client
            .get(url)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?
            .error_for_status()
            .map_err(|e| PtError::Http(e.to_string()))?
            .json::<OrderBookResponse>()
            .await
            .map_err(|e| PtError::Serde(e.to_string()))
    }

    pub async fn get_spread(&self, token_id: &str) -> PtResult<f64> {
        let url = format!("{}/spread?token_id={}", self.clob_api, token_id);
        let resp = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?
            .error_for_status()
            .map_err(|e| PtError::Http(e.to_string()))?
            .json::<SpreadResponse>()
            .await
            .map_err(|e| PtError::Serde(e.to_string()))?;

        Ok(resp.spread.parse::<f64>().unwrap_or(1.0))
    }

    pub async fn get_best_book(&self, token_id: &str) -> PtResult<BestBook> {
        let book = self.get_book(token_id).await?;

        let best_bid = book
            .bids
            .iter()
            .filter_map(|l| l.price.parse::<f64>().ok())
            .fold(0.0_f64, |acc, x| acc.max(x));

        let best_ask = book
            .asks
            .iter()
            .filter_map(|l| l.price.parse::<f64>().ok())
            .fold(1.0_f64, |acc, x| acc.min(x));

        if best_ask <= 0.0 || best_bid < 0.0 || best_bid >= best_ask {
            return Err(PtError::InvalidInput(format!(
                "invalid book top: bid={best_bid} ask={best_ask}"
            )));
        }

        let ts_ms = book
            .timestamp
            .parse::<i64>()
            .unwrap_or(Utc::now().timestamp_millis());

        Ok(BestBook {
            market_id: book.market,
            token_id: book.asset_id,
            best_bid,
            best_ask,
            spread: best_ask - best_bid,
            ts_ms,
        })
    }

    pub fn clob_ws(&self) -> &str {
        &self.clob_ws
    }
}

#[async_trait]
pub trait PolymarketExecution: Send + Sync {
    async fn post_quote(&self, quote: &QuoteIntent) -> PtResult<Vec<ExecutionReport>>;
    async fn cancel_stale(&self, market_id: &str) -> PtResult<()>;
}

#[derive(Debug, Clone)]
pub struct LivePolymarketConfig {
    pub clob_api: String,
    pub private_key: String,
    pub chain_id: u64,
    pub use_server_time: bool,
}

#[derive(Debug)]
struct LivePolymarketState {
    client: PolyClient<Authenticated<Normal>>,
    signer: PrivateKeySigner,
}

#[derive(Debug)]
pub struct LivePolymarketExecutor {
    cfg: LivePolymarketConfig,
    state: OnceCell<LivePolymarketState>,
    active_orders: RwLock<HashMap<String, Vec<String>>>,
}

impl LivePolymarketExecutor {
    pub fn new(cfg: LivePolymarketConfig) -> PtResult<Self> {
        if cfg.private_key.trim().is_empty() {
            return Err(PtError::Config(
                "polymarket live executor requires venues.polymarket.private_key".to_string(),
            ));
        }

        Ok(Self {
            cfg,
            state: OnceCell::new(),
            active_orders: RwLock::new(HashMap::new()),
        })
    }

    async fn ensure_state(&self) -> PtResult<&LivePolymarketState> {
        self.state
            .get_or_try_init(|| async {
                let signer = PrivateKeySigner::from_str(self.cfg.private_key.trim())
                    .map_err(|e| PtError::Config(format!("invalid polymarket private key: {e}")))?
                    .with_chain_id(Some(self.cfg.chain_id));

                let cfg = PolyConfig::builder()
                    .use_server_time(self.cfg.use_server_time)
                    .build();

                let client = PolyClient::new(&self.cfg.clob_api, cfg)
                    .map_err(|e| PtError::Http(format!("polymarket client init failed: {e}")))?
                    .authentication_builder(&signer)
                    .authenticate()
                    .await
                    .map_err(|e| PtError::Http(format!("polymarket authentication failed: {e}")))?;

                Ok(LivePolymarketState { client, signer })
            })
            .await
    }

    async fn post_limit_side(
        &self,
        quote: &QuoteIntent,
        price: f64,
        size: f64,
        side: PolySide,
        side_report: Side,
    ) -> PtResult<ExecutionReport> {
        let state = self.ensure_state().await?;
        let token_id = U256::from_str(&quote.token_id).map_err(|e| {
            PtError::InvalidInput(format!("invalid token_id {}: {e}", quote.token_id))
        })?;
        let price_dec = decimal_from_f64(price, "price")?;
        let size_dec = decimal_from_f64(size, "size")?;
        let expiration = Utc::now() + Duration::milliseconds(quote.ttl_ms as i64);

        let order = state
            .client
            .limit_order()
            .token_id(token_id)
            .side(side)
            .order_type(OrderType::GTD)
            .expiration(expiration)
            .post_only(true)
            .price(price_dec)
            .size(size_dec)
            .build()
            .await
            .map_err(|e| PtError::Http(format!("polymarket build order failed: {e}")))?;

        let signed = state
            .client
            .sign(&state.signer, order)
            .await
            .map_err(|e| PtError::Http(format!("polymarket sign order failed: {e}")))?;

        let response = state
            .client
            .post_order(signed)
            .await
            .map_err(|e| PtError::Http(format!("polymarket post order failed: {e}")))?;

        let filled_qty = response
            .taking_amount
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0);
        let status = map_post_status(response.success, response.taking_amount);
        let details = format!(
            "success={} status={:?} making={} taking={} trades={}",
            response.success,
            response.status,
            response.making_amount,
            response.taking_amount,
            response.trade_ids.len()
        );

        Ok(ExecutionReport {
            venue: Venue::Polymarket,
            order_id: response.order_id,
            market_id: Some(quote.market_id.clone()),
            status,
            side: side_report,
            filled_qty,
            avg_px: price,
            ts: Utc::now(),
            details: Some(details),
        })
    }
}

#[async_trait]
impl PolymarketExecution for LivePolymarketExecutor {
    async fn post_quote(&self, quote: &QuoteIntent) -> PtResult<Vec<ExecutionReport>> {
        let bid_result = self
            .post_limit_side(quote, quote.bid_px, quote.bid_sz, PolySide::Buy, Side::Buy)
            .await;
        let ask_result = self
            .post_limit_side(
                quote,
                quote.ask_px,
                quote.ask_sz,
                PolySide::Sell,
                Side::Sell,
            )
            .await;

        let mut order_ids = Vec::new();
        if let Ok(report) = &bid_result {
            order_ids.push(report.order_id.clone());
        }
        if let Ok(report) = &ask_result {
            order_ids.push(report.order_id.clone());
        }
        if !order_ids.is_empty() {
            self.active_orders
                .write()
                .insert(quote.market_id.clone(), order_ids);
        }

        let bid = bid_result?;
        let ask = ask_result?;
        Ok(vec![bid, ask])
    }

    async fn cancel_stale(&self, market_id: &str) -> PtResult<()> {
        let order_ids = self
            .active_orders
            .write()
            .remove(market_id)
            .unwrap_or_default();
        if order_ids.is_empty() {
            return Ok(());
        }

        let state = self.ensure_state().await?;
        let refs: Vec<&str> = order_ids.iter().map(String::as_str).collect();
        state
            .client
            .cancel_orders(&refs)
            .await
            .map_err(|e| PtError::Http(format!("polymarket cancel orders failed: {e}")))?;

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct NoopPolymarketExecutor;

#[async_trait]
impl PolymarketExecution for NoopPolymarketExecutor {
    async fn post_quote(&self, quote: &QuoteIntent) -> PtResult<Vec<ExecutionReport>> {
        let now = Utc::now();
        Ok(vec![
            ExecutionReport {
                venue: Venue::Polymarket,
                order_id: format!("noop-bid-{}", quote.market_id),
                market_id: Some(quote.market_id.clone()),
                status: ExecutionStatus::New,
                side: Side::Buy,
                filled_qty: 0.0,
                avg_px: quote.bid_px,
                ts: now,
                details: Some("noop executor".to_string()),
            },
            ExecutionReport {
                venue: Venue::Polymarket,
                order_id: format!("noop-ask-{}", quote.market_id),
                market_id: Some(quote.market_id.clone()),
                status: ExecutionStatus::New,
                side: Side::Sell,
                filled_qty: 0.0,
                avg_px: quote.ask_px,
                ts: now,
                details: Some("noop executor".to_string()),
            },
        ])
    }

    async fn cancel_stale(&self, _market_id: &str) -> PtResult<()> {
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct PaperPolymarketExecutor;

#[async_trait]
impl PolymarketExecution for PaperPolymarketExecutor {
    async fn post_quote(&self, quote: &QuoteIntent) -> PtResult<Vec<ExecutionReport>> {
        let now = Utc::now();
        Ok(vec![
            ExecutionReport {
                venue: Venue::Sim,
                order_id: format!("paper-bid-{}", quote.market_id),
                market_id: Some(quote.market_id.clone()),
                status: ExecutionStatus::New,
                side: Side::Buy,
                filled_qty: 0.0,
                avg_px: quote.bid_px,
                ts: now,
                details: Some("paper quote posted".to_string()),
            },
            ExecutionReport {
                venue: Venue::Sim,
                order_id: format!("paper-ask-{}", quote.market_id),
                market_id: Some(quote.market_id.clone()),
                status: ExecutionStatus::New,
                side: Side::Sell,
                filled_qty: 0.0,
                avg_px: quote.ask_px,
                ts: now,
                details: Some("paper quote posted".to_string()),
            },
        ])
    }

    async fn cancel_stale(&self, _market_id: &str) -> PtResult<()> {
        Ok(())
    }
}

fn decimal_from_f64(value: f64, field: &str) -> PtResult<Decimal> {
    if !value.is_finite() || value <= 0.0 {
        return Err(PtError::InvalidInput(format!(
            "invalid decimal for {field}: {value}"
        )));
    }
    Decimal::from_str(&value.to_string())
        .map_err(|e| PtError::InvalidInput(format!("invalid decimal for {field}: {e}")))
}

fn map_post_status(success: bool, taking_amount: Decimal) -> ExecutionStatus {
    if !success {
        ExecutionStatus::Rejected
    } else if taking_amount > Decimal::ZERO {
        ExecutionStatus::PartiallyFilled
    } else {
        ExecutionStatus::New
    }
}
