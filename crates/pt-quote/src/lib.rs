use pt_core::{clamp, round_down, round_up, MarketSelection, MarketSnapshot, QuoteIntent};

#[derive(Debug, Clone)]
pub struct QuoteConfig {
    pub base_half: f64,
    pub max_half: f64,
    pub ttl_ms: u64,
    pub base_size: f64,
    pub min_expected_net: f64,
}

impl Default for QuoteConfig {
    fn default() -> Self {
        Self {
            base_half: 0.01,
            max_half: 0.02,
            ttl_ms: 20_000,
            base_size: 5.0,
            min_expected_net: 0.002,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CostInputs {
    pub rebate_est: f64,
    pub adverse_sel_est: f64,
    pub hedge_cost_est: f64,
    pub gas_amortized_est: f64,
}

pub fn build_quote_intent(
    market: &MarketSelection,
    book: &MarketSnapshot,
    bias_shift: f64,
    inv_penalty: f64,
    costs: &CostInputs,
    cfg: &QuoteConfig,
) -> Option<QuoteIntent> {
    if book.ask <= 0.0 || book.bid <= 0.0 || book.ask <= book.bid {
        return None;
    }

    let mid = (book.bid + book.ask) / 2.0;
    let half = clamp(cfg.base_half, market.tick_size, cfg.max_half);

    let bid = round_down(mid - half + bias_shift - inv_penalty, market.tick_size);
    let ask = round_up(mid + half + bias_shift + inv_penalty, market.tick_size);

    if bid <= 0.0 || ask <= bid {
        return None;
    }

    let maker_edge = ask - bid;
    let expected_net = expected_net(
        maker_edge,
        costs.rebate_est,
        costs.adverse_sel_est,
        costs.hedge_cost_est,
        costs.gas_amortized_est,
    );

    if expected_net < cfg.min_expected_net {
        return None;
    }

    Some(QuoteIntent {
        market_id: market.market_id.clone(),
        token_id: market.token_id_yes.clone(),
        bid_px: bid,
        ask_px: ask,
        bid_sz: cfg.base_size.max(market.min_order_size),
        ask_sz: cfg.base_size.max(market.min_order_size),
        ttl_ms: cfg.ttl_ms,
        expected_net,
    })
}

pub fn expected_net(
    maker_edge: f64,
    rebate_est: f64,
    adverse_sel_est: f64,
    hedge_cost_est: f64,
    gas_amortized_est: f64,
) -> f64 {
    maker_edge + rebate_est - adverse_sel_est - hedge_cost_est - gas_amortized_est
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use pt_core::{Asset, MarketSelection, MarketSnapshot, MarketTier, TimeBucket};

    fn market() -> MarketSelection {
        MarketSelection {
            market_id: "test-market".into(),
            question: "Test?".into(),
            slug: "test-market".into(),
            token_id_yes: "tok-yes".into(),
            token_id_no: "tok-no".into(),
            asset: Asset::Btc,
            bucket: TimeBucket::FiveMinute,
            tier: MarketTier::TierA,
            fees_enabled: false,
            spread: 0.02,
            liquidity: 1000.0,
            volume24h: 500.0,
            tick_size: 0.01,
            min_order_size: 5.0,
            end_date: Utc::now() + chrono::Duration::days(30),
        }
    }

    fn book(bid: f64, ask: f64) -> MarketSnapshot {
        MarketSnapshot {
            market_id: "test-market".into(),
            token_id: "tok-yes".into(),
            bid,
            ask,
            spread: ask - bid,
            liquidity: 1000.0,
            ts: Utc::now(),
        }
    }

    #[test]
    fn expected_net_math() {
        let n = expected_net(0.02, 0.001, 0.005, 0.002, 0.001);
        assert!((n - 0.013).abs() < 1e-6);
    }

    #[test]
    fn invalid_book_ask_lte_bid_returns_none() {
        let m = market();
        let costs = CostInputs::default();
        let cfg = QuoteConfig::default();
        assert!(build_quote_intent(&m, &book(0.50, 0.50), 0.0, 0.0, &costs, &cfg).is_none());
        assert!(build_quote_intent(&m, &book(0.55, 0.50), 0.0, 0.0, &costs, &cfg).is_none());
    }

    #[test]
    fn invalid_book_non_positive_prices_returns_none() {
        let m = market();
        let costs = CostInputs::default();
        let cfg = QuoteConfig::default();
        assert!(build_quote_intent(&m, &book(0.0, 0.55), 0.0, 0.0, &costs, &cfg).is_none());
        assert!(build_quote_intent(&m, &book(-0.1, 0.55), 0.0, 0.0, &costs, &cfg).is_none());
    }

    #[test]
    fn expected_net_threshold_boundary() {
        let m = market();
        let costs = CostInputs::default();
        // Wide spread guarantees expected_net >= min_expected_net
        let cfg_pass = QuoteConfig {
            base_half: 0.05,
            min_expected_net: 0.002,
            ..QuoteConfig::default()
        };
        assert!(build_quote_intent(&m, &book(0.40, 0.60), 0.0, 0.0, &costs, &cfg_pass).is_some());

        // min_expected_net higher than possible edge → None
        let cfg_fail = QuoteConfig {
            base_half: 0.001,
            min_expected_net: 0.50,
            ..QuoteConfig::default()
        };
        assert!(build_quote_intent(&m, &book(0.49, 0.51), 0.0, 0.0, &costs, &cfg_fail).is_none());
    }

    #[test]
    fn min_order_size_floor_applied() {
        let mut m = market();
        m.min_order_size = 20.0;
        let costs = CostInputs::default();
        let cfg = QuoteConfig {
            base_size: 5.0, // below min_order_size
            base_half: 0.05,
            ..QuoteConfig::default()
        };
        let intent = build_quote_intent(&m, &book(0.40, 0.60), 0.0, 0.0, &costs, &cfg).unwrap();
        assert_eq!(intent.bid_sz, 20.0);
        assert_eq!(intent.ask_sz, 20.0);
    }

    #[test]
    fn bias_shift_creating_invalid_output_returns_none() {
        let m = market();
        let costs = CostInputs::default();
        let cfg = QuoteConfig {
            base_half: 0.01,
            ..QuoteConfig::default()
        };
        // Extreme negative bias pushes bid below zero → None
        assert!(build_quote_intent(&m, &book(0.02, 0.04), -0.10, 0.0, &costs, &cfg).is_none());
    }

    #[test]
    fn tick_rounding_determinism() {
        let m = market(); // tick_size = 0.01
        let costs = CostInputs::default();
        let cfg = QuoteConfig {
            base_half: 0.015,
            min_expected_net: 0.001,
            ..QuoteConfig::default()
        };
        let intent = build_quote_intent(&m, &book(0.48, 0.52), 0.0, 0.0, &costs, &cfg).unwrap();
        // bid and ask must be multiples of tick_size
        let tick = m.tick_size;
        assert!((intent.bid_px / tick).fract().abs() < 1e-9);
        assert!((intent.ask_px / tick).fract().abs() < 1e-9);
        // ask > bid always
        assert!(intent.ask_px > intent.bid_px);
    }
}
