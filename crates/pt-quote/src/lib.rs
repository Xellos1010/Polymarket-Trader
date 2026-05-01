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

    #[test]
    fn expected_net_math() {
        let n = expected_net(0.02, 0.001, 0.005, 0.002, 0.001);
        assert!((n - 0.013).abs() < 1e-6);
    }
}
