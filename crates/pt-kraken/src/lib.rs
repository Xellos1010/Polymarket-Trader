use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use pt_core::{PtError, PtResult, Side};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256, Sha512};
use std::collections::BTreeMap;
use url::form_urlencoded;

type HmacSha512 = Hmac<Sha512>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KrakenTopOfBook {
    pub product_id: String,
    pub bid: f64,
    pub ask: f64,
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KrakenOrderResult {
    pub txid: Option<String>,
    pub description: Option<String>,
    pub raw: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KrakenCancelResult {
    pub count: usize,
    pub pending: bool,
    pub raw: Value,
}

#[derive(Debug, Clone)]
struct KrakenCredentials {
    api_key: String,
    api_secret: String,
}

#[derive(Clone)]
pub struct KrakenClient {
    http: reqwest::Client,
    api_base: String,
    credentials: Option<KrakenCredentials>,
}

impl KrakenClient {
    pub fn new(
        api_base: impl Into<String>,
        api_key: Option<String>,
        api_secret: Option<String>,
    ) -> Self {
        let credentials = match (api_key, api_secret) {
            (Some(k), Some(s)) if !k.trim().is_empty() && !s.trim().is_empty() => {
                Some(KrakenCredentials {
                    api_key: k,
                    api_secret: s,
                })
            }
            _ => None,
        };

        Self {
            http: reqwest::Client::new(),
            api_base: api_base.into(),
            credentials,
        }
    }

    pub async fn ping(&self) -> PtResult<()> {
        let url = format!(
            "{}/0/public/SystemStatus",
            self.api_base.trim_end_matches('/')
        );
        let resp = self
            .http
            .get(url)
            .send()
            .await
            .map_err(|e| PtError::Http(format!("kraken ping failed: {e}")))?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(PtError::Http(format!(
                "kraken ping status {}",
                resp.status()
            )))
        }
    }

    pub async fn fetch_top_of_book(&self, pair: &str) -> PtResult<KrakenTopOfBook> {
        let pair_q = urlencoding(pair);
        let url = format!(
            "{}/0/public/Ticker?pair={}",
            self.api_base.trim_end_matches('/'),
            pair_q
        );
        let resp = self
            .http
            .get(url)
            .send()
            .await
            .map_err(|e| PtError::Http(format!("kraken ticker request failed: {e}")))?;
        let status = resp.status();
        let raw = resp
            .text()
            .await
            .map_err(|e| PtError::Http(format!("kraken ticker decode failed: {e}")))?;
        if !status.is_success() {
            return Err(PtError::Http(format!(
                "kraken ticker status={} body={raw}",
                status
            )));
        }

        let v: Value = serde_json::from_str(&raw).map_err(|e| PtError::Serde(e.to_string()))?;
        parse_kraken_errors(&v)?;
        let result = v
            .get("result")
            .and_then(Value::as_object)
            .ok_or_else(|| PtError::Serde("kraken ticker missing result object".to_string()))?;

        let (_, book) = result
            .iter()
            .next()
            .ok_or_else(|| PtError::Serde("kraken ticker result empty".to_string()))?;

        let bid = book
            .get("b")
            .and_then(Value::as_array)
            .and_then(|a| a.first())
            .and_then(Value::as_str)
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| PtError::Serde("kraken ticker missing bid".to_string()))?;
        let ask = book
            .get("a")
            .and_then(Value::as_array)
            .and_then(|a| a.first())
            .and_then(Value::as_str)
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| PtError::Serde("kraken ticker missing ask".to_string()))?;

        Ok(KrakenTopOfBook {
            product_id: pair.to_string(),
            bid,
            ask,
            ts: Utc::now(),
        })
    }

    pub async fn add_order_post_only_limit(
        &self,
        pair: &str,
        side: Side,
        volume: f64,
        price: f64,
    ) -> PtResult<KrakenOrderResult> {
        let mut params = BTreeMap::new();
        params.insert("pair".to_string(), pair.to_string());
        params.insert(
            "type".to_string(),
            match side {
                Side::Buy => "buy".to_string(),
                Side::Sell => "sell".to_string(),
            },
        );
        params.insert("ordertype".to_string(), "limit".to_string());
        params.insert("volume".to_string(), format!("{:.10}", volume.max(0.0)));
        params.insert("price".to_string(), format!("{:.10}", price.max(0.0)));
        params.insert("oflags".to_string(), "post".to_string());

        let v = self.private_post("/0/private/AddOrder", params).await?;
        let txid = v
            .get("result")
            .and_then(|x| x.get("txid"))
            .and_then(Value::as_array)
            .and_then(|a| a.first())
            .and_then(Value::as_str)
            .map(str::to_string);
        let description = v
            .get("result")
            .and_then(|x| x.get("descr"))
            .and_then(|x| x.get("order"))
            .and_then(Value::as_str)
            .map(str::to_string);

        Ok(KrakenOrderResult {
            txid,
            description,
            raw: v,
        })
    }

    pub async fn cancel_order(&self, txid: &str) -> PtResult<KrakenCancelResult> {
        let mut params = BTreeMap::new();
        params.insert("txid".to_string(), txid.to_string());
        let v = self.private_post("/0/private/CancelOrder", params).await?;
        let count = v
            .get("result")
            .and_then(|x| x.get("count"))
            .and_then(Value::as_u64)
            .unwrap_or(0) as usize;
        let pending = v
            .get("result")
            .and_then(|x| x.get("pending"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        Ok(KrakenCancelResult {
            count,
            pending,
            raw: v,
        })
    }

    pub async fn open_orders(&self) -> PtResult<Value> {
        self.private_post("/0/private/OpenOrders", BTreeMap::new())
            .await
    }

    async fn private_post(
        &self,
        path: &str,
        mut params: BTreeMap<String, String>,
    ) -> PtResult<Value> {
        let creds = self
            .credentials
            .as_ref()
            .ok_or_else(|| PtError::Config("kraken credentials not configured".to_string()))?;
        let nonce = Utc::now().timestamp_millis().to_string();
        params.insert("nonce".to_string(), nonce.clone());

        let body = form_urlencoded::Serializer::new(String::new())
            .extend_pairs(params.iter().map(|(k, v)| (k.as_str(), v.as_str())))
            .finish();

        let sig = sign_kraken(path, &nonce, &body, &creds.api_secret)?;
        let mut headers = HeaderMap::new();
        headers.insert(
            "API-Key",
            HeaderValue::from_str(&creds.api_key).map_err(|e| {
                PtError::InvalidInput(format!("invalid kraken api key header: {e}"))
            })?,
        );
        headers.insert(
            "API-Sign",
            HeaderValue::from_str(&sig).map_err(|e| {
                PtError::InvalidInput(format!("invalid kraken api sign header: {e}"))
            })?,
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        let url = format!("{}{}", self.api_base.trim_end_matches('/'), path);
        let resp = self
            .http
            .post(url)
            .headers(headers)
            .body(body)
            .send()
            .await
            .map_err(|e| PtError::Http(format!("kraken private request failed: {e}")))?;
        let status = resp.status();
        let raw = resp
            .text()
            .await
            .map_err(|e| PtError::Http(format!("kraken private decode failed: {e}")))?;
        if !status.is_success() {
            return Err(PtError::Http(format!(
                "kraken private status={} body={raw}",
                status
            )));
        }

        let v: Value = serde_json::from_str(&raw).map_err(|e| PtError::Serde(e.to_string()))?;
        parse_kraken_errors(&v)?;
        Ok(v)
    }
}

fn urlencoding(value: &str) -> String {
    form_urlencoded::byte_serialize(value.as_bytes()).collect::<String>()
}

fn parse_kraken_errors(v: &Value) -> PtResult<()> {
    let errors = v
        .get("error")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let list: Vec<String> = errors
        .into_iter()
        .filter_map(|e| e.as_str().map(str::to_string))
        .collect();
    if list.is_empty() {
        Ok(())
    } else {
        Err(PtError::Http(format!(
            "kraken api errors: {}",
            list.join(" | ")
        )))
    }
}

fn sign_kraken(path: &str, nonce: &str, body: &str, secret_b64: &str) -> PtResult<String> {
    let mut hasher = Sha256::new();
    hasher.update(nonce.as_bytes());
    hasher.update(body.as_bytes());
    let body_hash = hasher.finalize();

    let mut payload = Vec::with_capacity(path.len() + body_hash.len());
    payload.extend_from_slice(path.as_bytes());
    payload.extend_from_slice(&body_hash);

    let secret = B64
        .decode(secret_b64.trim())
        .map_err(|e| PtError::InvalidInput(format!("kraken secret must be base64: {e}")))?;

    let mut mac = HmacSha512::new_from_slice(&secret)
        .map_err(|e| PtError::InvalidInput(format!("kraken hmac init failed: {e}")))?;
    mac.update(&payload);
    let out = mac.finalize().into_bytes();
    Ok(B64.encode(out))
}

#[cfg(test)]
mod tests {
    use super::urlencoding;

    #[test]
    fn urlencoding_escapes_slash_pair() {
        assert_eq!(urlencoding("BTC/USD"), "BTC%2FUSD");
    }
}
