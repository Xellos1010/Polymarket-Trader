use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use chrono::{DateTime, Utc};
use hex::encode as hex_encode;
use hmac::{Hmac, Mac};
use pt_core::{PtError, PtResult, Side};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::Sha384;

type HmacSha384 = Hmac<Sha384>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiTopOfBook {
    pub product_id: String,
    pub bid: f64,
    pub ask: f64,
    pub ts: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiOrderResult {
    pub order_id: Option<String>,
    pub is_live: bool,
    pub is_cancelled: bool,
    pub raw: Value,
}

#[derive(Debug, Clone)]
struct GeminiCredentials {
    api_key: String,
    api_secret: String,
}

#[derive(Clone)]
pub struct GeminiClient {
    http: reqwest::Client,
    api_base: String,
    credentials: Option<GeminiCredentials>,
}

impl GeminiClient {
    pub fn new(
        api_base: impl Into<String>,
        api_key: Option<String>,
        api_secret: Option<String>,
    ) -> Self {
        let credentials = match (api_key, api_secret) {
            (Some(k), Some(s)) if !k.trim().is_empty() && !s.trim().is_empty() => {
                Some(GeminiCredentials {
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
        let url = format!("{}/v1/symbols", self.api_base.trim_end_matches('/'));
        let resp = self
            .http
            .get(url)
            .send()
            .await
            .map_err(|e| PtError::Http(format!("gemini ping failed: {e}")))?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(PtError::Http(format!(
                "gemini ping status {}",
                resp.status()
            )))
        }
    }

    pub async fn fetch_top_of_book(&self, symbol: &str) -> PtResult<GeminiTopOfBook> {
        let normalized = symbol.to_ascii_lowercase();
        let url = format!(
            "{}/v1/pubticker/{}",
            self.api_base.trim_end_matches('/'),
            normalized
        );
        let resp = self
            .http
            .get(url)
            .send()
            .await
            .map_err(|e| PtError::Http(format!("gemini ticker request failed: {e}")))?;
        let status = resp.status();
        let raw = resp
            .text()
            .await
            .map_err(|e| PtError::Http(format!("gemini ticker decode failed: {e}")))?;
        if !status.is_success() {
            return Err(PtError::Http(format!(
                "gemini ticker status={} body={raw}",
                status
            )));
        }
        let v: Value = serde_json::from_str(&raw).map_err(|e| PtError::Serde(e.to_string()))?;
        let bid = v
            .get("bid")
            .and_then(Value::as_str)
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| PtError::Serde("gemini ticker missing bid".to_string()))?;
        let ask = v
            .get("ask")
            .and_then(Value::as_str)
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| PtError::Serde("gemini ticker missing ask".to_string()))?;

        Ok(GeminiTopOfBook {
            product_id: symbol.to_string(),
            bid,
            ask,
            ts: Utc::now(),
        })
    }

    pub async fn new_order_post_only_limit(
        &self,
        symbol: &str,
        side: Side,
        amount: f64,
        price: f64,
        client_order_id: Option<&str>,
    ) -> PtResult<GeminiOrderResult> {
        let mut payload = json!({
            "symbol": symbol.to_ascii_lowercase(),
            "amount": format!("{:.10}", amount.max(0.0)),
            "price": format!("{:.10}", price.max(0.0)),
            "side": match side {
                Side::Buy => "buy",
                Side::Sell => "sell",
            },
            "type": "exchange limit",
            "options": ["maker-or-cancel"],
        });
        if let Some(id) = client_order_id {
            payload["client_order_id"] = Value::String(id.to_string());
        }

        let v = self.private_post("/v1/order/new", payload).await?;
        Ok(GeminiOrderResult {
            order_id: v
                .get("order_id")
                .or_else(|| v.get("id"))
                .map(|x| x.to_string().trim_matches('"').to_string()),
            is_live: v.get("is_live").and_then(Value::as_bool).unwrap_or(false),
            is_cancelled: v
                .get("is_cancelled")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            raw: v,
        })
    }

    pub async fn cancel_order(&self, order_id: &str) -> PtResult<GeminiOrderResult> {
        let v = self
            .private_post("/v1/order/cancel", json!({"order_id": order_id}))
            .await?;
        Ok(GeminiOrderResult {
            order_id: v
                .get("order_id")
                .or_else(|| v.get("id"))
                .map(|x| x.to_string().trim_matches('"').to_string()),
            is_live: v.get("is_live").and_then(Value::as_bool).unwrap_or(false),
            is_cancelled: v
                .get("is_cancelled")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            raw: v,
        })
    }

    pub async fn list_open_orders(&self) -> PtResult<Vec<Value>> {
        let v = self.private_post("/v1/orders", json!({})).await?;
        let rows = v.as_array().cloned().ok_or_else(|| {
            PtError::Serde("gemini open orders response must be array".to_string())
        })?;
        Ok(rows)
    }

    async fn private_post(&self, path: &str, payload: Value) -> PtResult<Value> {
        let creds = self
            .credentials
            .as_ref()
            .ok_or_else(|| PtError::Config("gemini credentials not configured".to_string()))?;

        let nonce = Utc::now().timestamp_millis().to_string();
        let mut body = payload;
        body["request"] = Value::String(path.to_string());
        body["nonce"] = Value::String(nonce);

        let serialized = serde_json::to_vec(&body).map_err(|e| PtError::Serde(e.to_string()))?;
        let payload_b64 = B64.encode(serialized);
        let signature = sign_gemini(&payload_b64, &creds.api_secret)?;

        let mut headers = HeaderMap::new();
        headers.insert(
            "X-GEMINI-APIKEY",
            HeaderValue::from_str(&creds.api_key).map_err(|e| {
                PtError::InvalidInput(format!("invalid gemini api key header: {e}"))
            })?,
        );
        headers.insert(
            "X-GEMINI-PAYLOAD",
            HeaderValue::from_str(&payload_b64).map_err(|e| {
                PtError::InvalidInput(format!("invalid gemini payload header: {e}"))
            })?,
        );
        headers.insert(
            "X-GEMINI-SIGNATURE",
            HeaderValue::from_str(&signature).map_err(|e| {
                PtError::InvalidInput(format!("invalid gemini signature header: {e}"))
            })?,
        );
        headers.insert("Content-Type", HeaderValue::from_static("text/plain"));
        headers.insert("Content-Length", HeaderValue::from_static("0"));

        let url = format!("{}{}", self.api_base.trim_end_matches('/'), path);
        let resp = self
            .http
            .post(url)
            .headers(headers)
            .body(String::new())
            .send()
            .await
            .map_err(|e| PtError::Http(format!("gemini private request failed: {e}")))?;
        let status = resp.status();
        let raw = resp
            .text()
            .await
            .map_err(|e| PtError::Http(format!("gemini private decode failed: {e}")))?;
        if !status.is_success() {
            return Err(PtError::Http(format!(
                "gemini private status={} body={raw}",
                status
            )));
        }
        serde_json::from_str::<Value>(&raw).map_err(|e| PtError::Serde(e.to_string()))
    }
}

fn sign_gemini(payload_b64: &str, api_secret: &str) -> PtResult<String> {
    let mut mac = HmacSha384::new_from_slice(api_secret.as_bytes())
        .map_err(|e| PtError::InvalidInput(format!("gemini hmac init failed: {e}")))?;
    mac.update(payload_b64.as_bytes());
    Ok(hex_encode(mac.finalize().into_bytes()))
}

#[cfg(test)]
mod tests {
    use super::sign_gemini;

    #[test]
    fn gemini_signature_is_hex() {
        let sig = sign_gemini("eyJ0ZXN0IjoidHJ1ZSJ9", "secret").expect("sign");
        assert!(sig.chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!(sig.len(), 96);
    }
}
