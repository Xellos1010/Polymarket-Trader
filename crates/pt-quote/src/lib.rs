use pt_core::{
    clamp, round_down, round_up, Asset, MarketSelection, MarketSnapshot, MarketTier, QuoteIntent,
    TimeBucket,
use chrono::Utc;
use pt_core::{
    clamp, round_down, round_up, EntryExitVector, ExecutionCostAttribution, ExecutionReport,
    MarketSelection, MarketSnapshot, QuoteIntent, Side, Venue,
};

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

pub fn vector_gate(
    quote: &QuoteIntent,
    book: &MarketSnapshot,
    vectors: &EntryExitVector,
) -> Result<(), String> {
    let mid = (book.bid + book.ask) / 2.0;
    if mid <= 0.0 {
        return Err("invalid mid".to_string());
    }

    let target_bid = mid * (1.0 - vectors.entry_offset_bps / 10_000.0);
    let target_ask = mid * (1.0 + vectors.exit_offset_bps / 10_000.0);

    let bid_slippage_bps = ((target_bid - quote.bid_px).abs() / mid) * 10_000.0;
    let ask_slippage_bps = ((quote.ask_px - target_ask).abs() / mid) * 10_000.0;

    if bid_slippage_bps > vectors.entry_max_slippage_bps {
        return Err(format!(
            "entry vector breach: {:.4}bps > {:.4}bps",
            bid_slippage_bps, vectors.entry_max_slippage_bps
        ));
    }
    if ask_slippage_bps > vectors.exit_max_slippage_bps {
        return Err(format!(
            "exit vector breach: {:.4}bps > {:.4}bps",
            ask_slippage_bps, vectors.exit_max_slippage_bps
        ));
    }

    Ok(())
}

pub fn estimate_execution_cost(
    execution_id: &str,
    report: &ExecutionReport,
    reference_px: f64,
    fee_bps: f64,
    rebate_bps_est: f64,
) -> ExecutionCostAttribution {
    let qty = report.filled_qty.max(0.0);
    let px = report.avg_px.max(0.0);
    let notional = qty * px;
    let fee_est = notional * fee_bps / 10_000.0;
    let rebate_est = notional * rebate_bps_est / 10_000.0;

    let slippage_bps = if reference_px > 0.0 {
        match report.side {
            Side::Buy => ((px - reference_px) / reference_px) * 10_000.0,
            Side::Sell => ((reference_px - px) / reference_px) * 10_000.0,
        }
    } else {
        0.0
    };
    let slippage_est = if reference_px > 0.0 {
        notional * slippage_bps / 10_000.0
    } else {
        0.0
    };

    ExecutionCostAttribution {
        execution_id: execution_id.to_string(),
        venue: report.venue.clone(),
        market_id: report.market_id.clone(),
        side: report.side.clone(),
        qty,
        avg_px: px,
        reference_px,
        fee_bps,
        fee_est,
        slippage_bps,
        slippage_est,
        rebate_bps_est,
        rebate_est,
        effective_edge: rebate_est - fee_est - slippage_est,
        ts: Utc::now(),
        strategy_class: None,
        route_id: None,
    }
}

pub fn default_fee_bps_for_venue(venue: &Venue, maker_fee_bps: f64, taker_fee_bps: f64) -> f64 {
    match venue {
        Venue::Polymarket => maker_fee_bps,
        Venue::Coinbase | Venue::Kraken | Venue::Gemini => taker_fee_bps,
        Venue::Sim => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn market(tick_size: f64, min_order_size: f64) -> MarketSelection {
        MarketSelection {
            market_id: "m1".into(),
            question: "test".into(),
            slug: "updown-5m-btc".into(),
            token_id_yes: "yes".into(),
            token_id_no: "no".into(),
            asset: Asset::Btc,
            bucket: TimeBucket::FiveMinute,
            tier: MarketTier::TierA,
            fees_enabled: true,
            spread: 0.02,
            liquidity: 1_000.0,
            volume24h: 100.0,
            tick_size,
            min_order_size,
            end_date: Utc::now(),
        }
    }

    fn book(bid: f64, ask: f64) -> MarketSnapshot {
        MarketSnapshot {
            market_id: "m1".into(),
            token_id: "yes".into(),
            bid,
            ask,
            spread: ask - bid,
            liquidity: 1_000.0,
            ts: Utc::now(),
        }
    }
    use pt_core::{ExecutionStatus, Side, Venue};

    #[test]
    fn expected_net_math() {
        let n = expected_net(0.02, 0.001, 0.005, 0.002, 0.001);
        assert!((n - 0.013).abs() < 1e-6);
    }

    #[test]
    fn invalid_books_are_rejected() {
        let market = market(0.01, 1.0);
        let cfg = QuoteConfig::default();
        let costs = CostInputs::default();

        for snapshot in [book(0.50, 0.50), book(0.51, 0.50), book(0.0, 0.50), book(0.49, 0.0)] {
            assert!(build_quote_intent(&market, &snapshot, 0.0, 0.0, &costs, &cfg).is_none());
        }
    }

    #[test]
    fn expected_net_threshold_is_inclusive_at_boundary() {
        let market = market(0.01, 1.0);
        let snapshot = book(0.49, 0.51);
        let costs = CostInputs::default();

        let boundary_cfg = QuoteConfig {
            min_expected_net: 0.02,
            ..QuoteConfig::default()
        };
        let boundary = build_quote_intent(&market, &snapshot, 0.0, 0.0, &costs, &boundary_cfg)
            .expect("boundary quote");
        assert!((boundary.expected_net - 0.02).abs() < 1e-9);

        let reject_cfg = QuoteConfig {
            min_expected_net: 0.0201,
            ..QuoteConfig::default()
        };
        assert!(build_quote_intent(&market, &snapshot, 0.0, 0.0, &costs, &reject_cfg).is_none());
    }

    #[test]
    fn tick_rounding_and_clamp_are_deterministic() {
        let market = market(0.05, 1.0);
        let snapshot = book(0.48, 0.52);
        let cfg = QuoteConfig {
            base_half: 0.01,
            max_half: 0.20,
            ..QuoteConfig::default()
        };

        let quote = build_quote_intent(&market, &snapshot, 0.01, 0.0, &CostInputs::default(), &cfg)
            .expect("quote");
        assert!((quote.bid_px - 0.45).abs() < 1e-9);
        assert!((quote.ask_px - 0.60).abs() < 1e-9);
    }

    #[test]
    fn min_order_size_sets_quote_floor() {
        let market = market(0.01, 3.0);
        let snapshot = book(0.49, 0.51);
        let cfg = QuoteConfig {
            base_size: 1.0,
            ..QuoteConfig::default()
        };

        let quote = build_quote_intent(&market, &snapshot, 0.0, 0.0, &CostInputs::default(), &cfg)
            .expect("quote");
        assert_eq!(quote.bid_sz, 3.0);
        assert_eq!(quote.ask_sz, 3.0);
    }

    #[test]
    fn large_negative_shift_invalidates_quote() {
        let market = market(0.01, 1.0);
        let snapshot = book(0.49, 0.51);

        assert!(build_quote_intent(
            &market,
            &snapshot,
            -1.0,
            0.0,
            &CostInputs::default(),
            &QuoteConfig::default(),
        )
        .is_none());
    fn vector_gate_allows_in_range() {
        let quote = QuoteIntent {
            market_id: "m1".to_string(),
            token_id: "t1".to_string(),
            bid_px: 99.9,
            ask_px: 100.1,
            bid_sz: 1.0,
            ask_sz: 1.0,
            ttl_ms: 1000,
            expected_net: 0.01,
        };
        let book = MarketSnapshot {
            market_id: "m1".to_string(),
            token_id: "t1".to_string(),
            bid: 99.95,
            ask: 100.05,
            spread: 0.10,
            liquidity: 1_000.0,
            ts: Utc::now(),
        };
        let vectors = EntryExitVector {
            entry_max_slippage_bps: 20.0,
            exit_max_slippage_bps: 20.0,
            entry_offset_bps: 10.0,
            exit_offset_bps: 10.0,
            max_cross_bps_unwind: 25.0,
        };
        assert!(vector_gate(&quote, &book, &vectors).is_ok());
    }

    #[test]
    fn estimate_execution_cost_shapes_output() {
        let report = ExecutionReport {
            venue: Venue::Coinbase,
            order_id: "o1".to_string(),
            market_id: Some("m1".to_string()),
            status: ExecutionStatus::Filled,
            side: Side::Buy,
            filled_qty: 2.0,
            avg_px: 101.0,
            ts: Utc::now(),
            details: None,
        };
        let out = estimate_execution_cost("x1", &report, 100.0, 10.0, 0.0);
        assert_eq!(out.execution_id, "x1");
        assert!(out.fee_est >= 0.0);
    }
}
