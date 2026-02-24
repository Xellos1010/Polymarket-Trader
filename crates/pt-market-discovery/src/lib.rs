use chrono::{DateTime, Utc};
use pt_core::{
    Asset, MarketSelection, MarketTier, PolymarketFilterConfig, PtError, PtResult, TimeBucket,
};
use reqwest::Client;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct MarketDiscoveryClient {
    client: Client,
    gamma_api: String,
    filters: PolymarketFilterConfig,
}

#[derive(Debug, Clone, Deserialize)]
struct GammaMarket {
    id: String,
    question: String,
    slug: String,
    #[serde(default, rename = "feesEnabled", deserialize_with = "de_bool_default")]
    fees_enabled: bool,
    #[serde(default, deserialize_with = "de_opt_f64")]
    spread: Option<f64>,
    #[serde(default, deserialize_with = "de_string_default")]
    liquidity: String,
    #[serde(default, deserialize_with = "de_opt_f64")]
    volume24hr: Option<f64>,
    #[serde(
        default,
        rename = "enableOrderBook",
        deserialize_with = "de_bool_default"
    )]
    enable_order_book: bool,
    #[serde(
        default,
        rename = "clobTokenIds",
        deserialize_with = "de_token_ids_default"
    )]
    clob_token_ids: String,
    #[serde(
        default,
        rename = "orderPriceMinTickSize",
        deserialize_with = "de_opt_f64"
    )]
    order_price_min_tick_size: Option<f64>,
    #[serde(default, rename = "orderMinSize", deserialize_with = "de_opt_f64")]
    order_min_size: Option<f64>,
    #[serde(rename = "endDate")]
    end_date: Option<String>,
}

impl MarketDiscoveryClient {
    pub fn new(gamma_api: impl Into<String>, filters: PolymarketFilterConfig) -> Self {
        Self {
            client: Client::new(),
            gamma_api: gamma_api.into(),
            filters,
        }
    }

    pub async fn fetch_all_markets(&self) -> PtResult<Vec<MarketSelection>> {
        let mut offset = 0_u64;
        let mut selected = Vec::new();

        loop {
            let page = self.fetch_page(offset, 500).await?;
            if page.is_empty() {
                break;
            }

            for market in page {
                if let Some(sel) = self.to_selection(market)? {
                    selected.push(sel);
                }
            }
            offset += 500;
            if offset > 20_000 {
                break;
            }
        }

        selected.sort_by(|a, b| {
            b.volume24h
                .partial_cmp(&a.volume24h)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(selected)
    }

    async fn fetch_page(&self, offset: u64, limit: u64) -> PtResult<Vec<GammaMarket>> {
        let url = format!(
            "{}/markets?tag_id={}&active=true&closed=false&limit={}&offset={}",
            self.gamma_api, self.filters.tag_id, limit, offset
        );

        let resp = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| PtError::Http(e.to_string()))?
            .error_for_status()
            .map_err(|e| PtError::Http(e.to_string()))?;

        resp.json::<Vec<GammaMarket>>()
            .await
            .map_err(|e| PtError::Serde(e.to_string()))
    }

    fn to_selection(&self, market: GammaMarket) -> PtResult<Option<MarketSelection>> {
        let spread = market.spread.unwrap_or(1.0);
        let liquidity = market.liquidity.parse::<f64>().unwrap_or(0.0);
        let volume24h = market.volume24hr.unwrap_or(0.0);

        if self.filters.require_fee_enabled && !market.fees_enabled {
            return Ok(None);
        }

        if self.filters.require_orderbook && !market.enable_order_book {
            return Ok(None);
        }

        if !self.filters.allowed_slugs.is_empty()
            && !self
                .filters
                .allowed_slugs
                .iter()
                .any(|needle| market.slug.contains(needle))
        {
            return Ok(None);
        }

        let asset = Asset::from_slug(&market.slug);
        if !self.filters.assets.is_empty() {
            let allowed = self
                .filters
                .assets
                .iter()
                .any(|a| a.eq_ignore_ascii_case(asset.as_str()));
            if !allowed {
                return Ok(None);
            }
        }

        let Some(end_date_raw) = market.end_date else {
            return Ok(None);
        };
        let end_date = DateTime::parse_from_rfc3339(&end_date_raw)
            .map_err(|e| PtError::InvalidInput(e.to_string()))?
            .with_timezone(&Utc);

        if end_date <= Utc::now() {
            return Ok(None);
        }

        let tokens = parse_token_ids(&market.clob_token_ids)?;
        if tokens.len() < 2 {
            return Ok(None);
        }

        let tier = if spread <= self.filters.max_spread
            && liquidity >= self.filters.min_liquidity
            && volume24h >= self.filters.min_volume24h
        {
            MarketTier::TierA
        } else if spread <= self.filters.max_spread * 1.5
            && liquidity >= self.filters.min_liquidity * 0.5
        {
            MarketTier::TierB
        } else {
            MarketTier::TierC
        };

        Ok(Some(MarketSelection {
            market_id: market.id,
            question: market.question,
            slug: market.slug.clone(),
            token_id_yes: tokens[0].clone(),
            token_id_no: tokens[1].clone(),
            asset,
            bucket: TimeBucket::from_slug(&market.slug),
            tier,
            fees_enabled: market.fees_enabled,
            spread,
            liquidity,
            volume24h,
            tick_size: market.order_price_min_tick_size.unwrap_or(0.01),
            min_order_size: market.order_min_size.unwrap_or(5.0),
            end_date,
        }))
    }
}

fn parse_token_ids(raw: &str) -> PtResult<Vec<String>> {
    serde_json::from_str::<Vec<String>>(raw).map_err(|e| PtError::Serde(e.to_string()))
}

fn de_bool_default<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let val = Value::deserialize(deserializer).unwrap_or(Value::Null);
    let parsed = match val {
        Value::Bool(b) => b,
        Value::Number(n) => n.as_i64().unwrap_or(0) != 0,
        Value::String(s) => matches!(s.to_ascii_lowercase().as_str(), "1" | "true" | "yes"),
        _ => false,
    };
    Ok(parsed)
}

fn de_opt_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let val = Value::deserialize(deserializer).unwrap_or(Value::Null);
    let parsed = match val {
        Value::Null => None,
        Value::Number(n) => n.as_f64(),
        Value::String(s) => s.parse::<f64>().ok(),
        _ => None,
    };
    Ok(parsed)
}

fn de_string_default<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let val = Value::deserialize(deserializer).unwrap_or(Value::Null);
    let s = match val {
        Value::String(s) => s,
        Value::Number(n) => n.to_string(),
        _ => "0".to_string(),
    };
    Ok(s)
}

fn de_token_ids_default<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let val = Value::deserialize(deserializer).unwrap_or(Value::Null);
    let out = match val {
        Value::String(s) => s,
        Value::Array(arr) => serde_json::to_string(&arr).unwrap_or_else(|_| "[]".to_string()),
        _ => "[]".to_string(),
    };
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tokens() {
        let t = parse_token_ids("[\"1\",\"2\"]").unwrap();
        assert_eq!(t.len(), 2);
    }
}
