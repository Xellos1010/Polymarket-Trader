use chrono::Utc;
use pt_core::{EdgeProfile, RouteLeg, RouteOpportunity, Side, StrategyClass};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct RouteBook {
    pub best_bid: f64,
    pub best_ask: f64,
}

#[derive(Debug, Clone)]
struct DirectedEdge {
    venue: String,
    from: String,
    to: String,
    product_id: String,
    side: Side,
    px: f64,
}

#[allow(clippy::too_many_arguments)] // Route scan takes independent fee and penalty knobs from callers.
pub fn find_route_opportunities(
    books: &HashMap<String, RouteBook>,
    capital_usd: f64,
    venue_maker_fees_bps: &HashMap<String, f64>,
    expected_slippage_bps_per_leg: f64,
    cancel_churn_bps_per_leg: f64,
    reject_penalty_bps: f64,
    latency_decay_penalty_bps: f64,
    edge_profile: &EdgeProfile,
) -> Vec<RouteOpportunity> {
    if capital_usd <= 0.0 {
        return Vec::new();
    }

    let edges = build_edges(books);
    let mut out = Vec::new();

    // 2-leg cycles: A->B->A
    for i in 0..edges.len() {
        for j in 0..edges.len() {
            if i == j {
                continue;
            }
            let e1 = &edges[i];
            let e2 = &edges[j];
            if e1.to != e2.from || e2.to != e1.from {
                continue;
            }
            let multiplier = e1.px * e2.px;
            let gross_bps = (multiplier - 1.0) * 10_000.0;
            let legs_cost_bps = edge_cost_bps(
                e1,
                venue_maker_fees_bps,
                expected_slippage_bps_per_leg,
                cancel_churn_bps_per_leg,
            ) + edge_cost_bps(
                e2,
                venue_maker_fees_bps,
                expected_slippage_bps_per_leg,
                cancel_churn_bps_per_leg,
            );
            let net_bps =
                gross_bps - legs_cost_bps - reject_penalty_bps - latency_decay_penalty_bps;
            let min_bps = min_edge_for_cycle(&e1.from, edge_profile);
            if net_bps < min_bps {
                continue;
            }

            let route_id = format!("route2-{}-{}-{}", e1.from, e1.to, i);
            let legs = vec![
                RouteLeg {
                    product_id: e1.product_id.clone(),
                    side: e1.side.clone(),
                    input_asset: e1.from.clone(),
                    output_asset: e1.to.clone(),
                    price: e1.px,
                    size: capital_usd,
                },
                RouteLeg {
                    product_id: e2.product_id.clone(),
                    side: e2.side.clone(),
                    input_asset: e2.from.clone(),
                    output_asset: e2.to.clone(),
                    price: e2.px,
                    size: capital_usd,
                },
            ];

            out.push(RouteOpportunity {
                route_id,
                legs,
                gross_edge_bps: gross_bps,
                expected_net_bps: net_bps,
                expected_usd_profit: capital_usd * (net_bps / 10_000.0),
                capital_required_usd: capital_usd,
                strategy_class: StrategyClass::ConversionCycle,
                ts: Utc::now(),
            });
        }
    }

    // 3-leg cycles: A->B->C->A
    for i in 0..edges.len() {
        for j in 0..edges.len() {
            if i == j {
                continue;
            }
            for k in 0..edges.len() {
                if i == k || j == k {
                    continue;
                }
                let e1 = &edges[i];
                let e2 = &edges[j];
                let e3 = &edges[k];
                if e1.to != e2.from || e2.to != e3.from || e3.to != e1.from {
                    continue;
                }

                let multiplier = e1.px * e2.px * e3.px;
                let gross_bps = (multiplier - 1.0) * 10_000.0;
                let legs_cost_bps = edge_cost_bps(
                    e1,
                    venue_maker_fees_bps,
                    expected_slippage_bps_per_leg,
                    cancel_churn_bps_per_leg,
                ) + edge_cost_bps(
                    e2,
                    venue_maker_fees_bps,
                    expected_slippage_bps_per_leg,
                    cancel_churn_bps_per_leg,
                ) + edge_cost_bps(
                    e3,
                    venue_maker_fees_bps,
                    expected_slippage_bps_per_leg,
                    cancel_churn_bps_per_leg,
                );
                let net_bps =
                    gross_bps - legs_cost_bps - reject_penalty_bps - latency_decay_penalty_bps;
                let min_bps = min_edge_for_cycle(&e1.from, edge_profile);
                if net_bps < min_bps {
                    continue;
                }

                let route_id = format!("route3-{}-{}-{}-{}", e1.from, e1.to, e2.to, i);
                let legs = vec![
                    RouteLeg {
                        product_id: e1.product_id.clone(),
                        side: e1.side.clone(),
                        input_asset: e1.from.clone(),
                        output_asset: e1.to.clone(),
                        price: e1.px,
                        size: capital_usd,
                    },
                    RouteLeg {
                        product_id: e2.product_id.clone(),
                        side: e2.side.clone(),
                        input_asset: e2.from.clone(),
                        output_asset: e2.to.clone(),
                        price: e2.px,
                        size: capital_usd,
                    },
                    RouteLeg {
                        product_id: e3.product_id.clone(),
                        side: e3.side.clone(),
                        input_asset: e3.from.clone(),
                        output_asset: e3.to.clone(),
                        price: e3.px,
                        size: capital_usd,
                    },
                ];

                out.push(RouteOpportunity {
                    route_id,
                    legs,
                    gross_edge_bps: gross_bps,
                    expected_net_bps: net_bps,
                    expected_usd_profit: capital_usd * (net_bps / 10_000.0),
                    capital_required_usd: capital_usd,
                    strategy_class: StrategyClass::ConversionCycle,
                    ts: Utc::now(),
                });
            }
        }
    }

    out.sort_by(|a, b| {
        b.expected_net_bps
            .partial_cmp(&a.expected_net_bps)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    out
}

fn min_edge_for_cycle(seed_asset: &str, edge_profile: &EdgeProfile) -> f64 {
    edge_profile
        .per_asset_overrides_bps
        .get(&seed_asset.to_ascii_uppercase())
        .copied()
        .unwrap_or(edge_profile.conversion_cycle_min_bps)
}

fn edge_cost_bps(
    edge: &DirectedEdge,
    venue_maker_fees_bps: &HashMap<String, f64>,
    expected_slippage_bps_per_leg: f64,
    cancel_churn_bps_per_leg: f64,
) -> f64 {
    maker_fee_for_venue_bps(&edge.venue, venue_maker_fees_bps)
        + expected_slippage_bps_per_leg
        + cancel_churn_bps_per_leg
}

fn maker_fee_for_venue_bps(venue: &str, venue_maker_fees_bps: &HashMap<String, f64>) -> f64 {
    let key = venue.to_ascii_lowercase();
    if let Some(v) = venue_maker_fees_bps.get(&key) {
        return *v;
    }
    venue_maker_fees_bps
        .get("coinbase")
        .copied()
        .unwrap_or_default()
}

fn build_edges(books: &HashMap<String, RouteBook>) -> Vec<DirectedEdge> {
    let mut edges = Vec::new();
    let mut product_ids: Vec<&String> = books.keys().collect();
    product_ids.sort();
    for product_id in product_ids {
        let Some(book) = books.get(product_id) else {
            continue;
        };
        let (venue, raw_product_id) = split_venue_product(product_id);
        let Some((base, quote)) = split_product(raw_product_id) else {
            continue;
        };
        if book.best_bid <= 0.0 || book.best_ask <= 0.0 || book.best_bid >= book.best_ask {
            continue;
        }

        // BUY base with quote at ask (quote -> base)
        edges.push(DirectedEdge {
            venue: venue.to_string(),
            from: quote.to_string(),
            to: base.to_string(),
            product_id: product_id.clone(),
            side: Side::Buy,
            px: 1.0 / book.best_ask,
        });

        // SELL base for quote at bid (base -> quote)
        edges.push(DirectedEdge {
            venue: venue.to_string(),
            from: base.to_string(),
            to: quote.to_string(),
            product_id: product_id.clone(),
            side: Side::Sell,
            px: book.best_bid,
        });
    }
    edges
}

fn split_product(product_id: &str) -> Option<(&str, &str)> {
    let mut parts = product_id.split('-').filter(|p| !p.is_empty());
    let base = parts.next()?;
    let quote = parts.next()?;
    if base.is_empty() || quote.is_empty() {
        return None;
    }
    Some((base, quote))
}

fn split_venue_product(product_id: &str) -> (&str, &str) {
    if let Some((venue, pair)) = product_id.split_once(':') {
        if !venue.trim().is_empty() && !pair.trim().is_empty() {
            return (venue.trim(), pair.trim());
        }
    }
    ("coinbase", product_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_detection_returns_positive_net_cycle() {
        let mut books = HashMap::new();
        books.insert(
            "coinbase:BTC-USD".to_string(),
            RouteBook {
                best_bid: 100.0,
                best_ask: 100.1,
            },
        );
        books.insert(
            "kraken:BTC-USDC".to_string(),
            RouteBook {
                best_bid: 101.3,
                best_ask: 101.5,
            },
        );
        books.insert(
            "gemini:USDC-USD".to_string(),
            RouteBook {
                best_bid: 1.002,
                best_ask: 1.003,
            },
        );

        let opportunities = find_route_opportunities(
            &books,
            20.0,
            &HashMap::from([
                ("coinbase".to_string(), 1.0),
                ("kraken".to_string(), 1.0),
                ("gemini".to_string(), 1.0),
            ]),
            1.0,
            1.0,
            0.0,
            0.0,
            &EdgeProfile {
                maker_mm_spot_min_bps: 8.0,
                conversion_cycle_min_bps: 5.0,
                position_reentry_min_bps: 40.0,
                per_asset_overrides_bps: HashMap::new(),
            },
        );

        assert!(!opportunities.is_empty());
        assert!(opportunities[0].expected_net_bps > 0.0);
    }

    #[test]
    fn split_product_supports_prefixed_pairs() {
        let (v1, p1) = split_venue_product("coinbase:BTC-USD");
        assert_eq!(v1, "coinbase");
        assert_eq!(split_product(p1), Some(("BTC", "USD")));

        let (v2, p2) = split_venue_product("kraken:XBT-USD");
        assert_eq!(v2, "kraken");
        assert_eq!(split_product(p2), Some(("XBT", "USD")));

        let (v3, p3) = split_venue_product("gemini:BTC-USD");
        assert_eq!(v3, "gemini");
        assert_eq!(split_product(p3), Some(("BTC", "USD")));
    }

    #[test]
    fn venue_specific_fees_change_net_edge() {
        let mut books = HashMap::new();
        books.insert(
            "coinbase:BTC-USD".to_string(),
            RouteBook {
                best_bid: 100.0,
                best_ask: 100.1,
            },
        );
        books.insert(
            "kraken:BTC-USDC".to_string(),
            RouteBook {
                best_bid: 101.3,
                best_ask: 101.5,
            },
        );
        books.insert(
            "gemini:USDC-USD".to_string(),
            RouteBook {
                best_bid: 1.002,
                best_ask: 1.003,
            },
        );

        let low_fees = HashMap::from([
            ("coinbase".to_string(), 1.0),
            ("kraken".to_string(), 1.0),
            ("gemini".to_string(), 1.0),
        ]);
        let high_fees = HashMap::from([
            ("coinbase".to_string(), 25.0),
            ("kraken".to_string(), 30.0),
            ("gemini".to_string(), 35.0),
        ]);

        let low = find_route_opportunities(
            &books,
            20.0,
            &low_fees,
            1.0,
            1.0,
            0.0,
            0.0,
            &EdgeProfile {
                maker_mm_spot_min_bps: 8.0,
                conversion_cycle_min_bps: 0.0,
                position_reentry_min_bps: 40.0,
                per_asset_overrides_bps: HashMap::new(),
            },
        );
        let high = find_route_opportunities(
            &books,
            20.0,
            &high_fees,
            1.0,
            1.0,
            0.0,
            0.0,
            &EdgeProfile {
                maker_mm_spot_min_bps: 8.0,
                conversion_cycle_min_bps: 0.0,
                position_reentry_min_bps: 40.0,
                per_asset_overrides_bps: HashMap::new(),
            },
        );

        assert!(!low.is_empty());
        assert!(!high.is_empty());
        assert!(low[0].expected_net_bps > high[0].expected_net_bps);
    }

    #[test]
    fn reject_and_latency_penalties_reduce_net_edge() {
        let mut books = HashMap::new();
        books.insert(
            "coinbase:BTC-USD".to_string(),
            RouteBook {
                best_bid: 100.0,
                best_ask: 100.1,
            },
        );
        books.insert(
            "kraken:BTC-USDC".to_string(),
            RouteBook {
                best_bid: 101.3,
                best_ask: 101.5,
            },
        );
        books.insert(
            "gemini:USDC-USD".to_string(),
            RouteBook {
                best_bid: 1.002,
                best_ask: 1.003,
            },
        );
        let fees = HashMap::from([
            ("coinbase".to_string(), 1.0),
            ("kraken".to_string(), 1.0),
            ("gemini".to_string(), 1.0),
        ]);
        let baseline = find_route_opportunities(
            &books,
            20.0,
            &fees,
            1.0,
            1.0,
            0.0,
            0.0,
            &EdgeProfile {
                maker_mm_spot_min_bps: 8.0,
                conversion_cycle_min_bps: 0.0,
                position_reentry_min_bps: 40.0,
                per_asset_overrides_bps: HashMap::new(),
            },
        );
        let penalized = find_route_opportunities(
            &books,
            20.0,
            &fees,
            1.0,
            1.0,
            25.0,
            10.0,
            &EdgeProfile {
                maker_mm_spot_min_bps: 0.0,
                conversion_cycle_min_bps: 0.0,
                position_reentry_min_bps: 40.0,
                per_asset_overrides_bps: HashMap::new(),
            },
        );
        assert!(!baseline.is_empty());
        assert!(!penalized.is_empty());
        assert!(baseline[0].expected_net_bps > penalized[0].expected_net_bps);
    }
}
