use pt_core::{OrderManagerDecision, Side};

#[derive(Debug, Clone)]
pub struct OrderManagerConfig {
    pub preview_required: bool,
    pub max_reprice_attempts: u32,
    pub edit_vs_replace_threshold_bps: f64,
    pub cancel_replace_cooldown_ms: u64,
    pub min_rest_ms: u64,
}

#[derive(Debug, Clone)]
pub struct RestingOrder {
    pub order_id: String,
    pub side: Side,
    pub price: f64,
    pub size: f64,
    pub submitted_ts_ms: i64,
    pub last_replace_ts_ms: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct TopOfBook {
    pub best_bid: f64,
    pub best_ask: f64,
    pub tick_size: f64,
}

#[derive(Debug, Clone)]
pub struct OrderManager {
    cfg: OrderManagerConfig,
}

impl OrderManager {
    pub fn new(cfg: OrderManagerConfig) -> Self {
        Self { cfg }
    }

    pub fn decide(
        &self,
        existing: Option<&RestingOrder>,
        desired_side: Side,
        desired_price: f64,
        desired_size: f64,
        book: TopOfBook,
        now_ms: i64,
    ) -> OrderManagerDecision {
        let safe_price = reprice_post_only(
            desired_side.clone(),
            desired_price,
            book.best_bid,
            book.best_ask,
            book.tick_size,
            self.cfg.max_reprice_attempts,
        )
        .unwrap_or(desired_price);

        let Some(order) = existing else {
            return OrderManagerDecision {
                action: "submit".to_string(),
                reason: if self.cfg.preview_required {
                    "new_order_preview_then_submit".to_string()
                } else {
                    "new_order_submit".to_string()
                },
                should_edit: false,
                should_cancel_replace: false,
                target_price: safe_price,
                target_size: desired_size,
            };
        };

        if now_ms - order.submitted_ts_ms < self.cfg.min_rest_ms as i64 {
            return OrderManagerDecision {
                action: "hold".to_string(),
                reason: "min_rest_window".to_string(),
                should_edit: false,
                should_cancel_replace: false,
                target_price: order.price,
                target_size: order.size,
            };
        }

        let px_diff_bps = if order.price > 0.0 {
            ((safe_price - order.price).abs() / order.price) * 10_000.0
        } else {
            f64::INFINITY
        };

        if px_diff_bps <= self.cfg.edit_vs_replace_threshold_bps {
            return OrderManagerDecision {
                action: "edit".to_string(),
                reason: format!(
                    "price_delta_within_{:.2}_bps",
                    self.cfg.edit_vs_replace_threshold_bps
                ),
                should_edit: true,
                should_cancel_replace: false,
                target_price: safe_price,
                target_size: desired_size,
            };
        }

        if now_ms - order.last_replace_ts_ms < self.cfg.cancel_replace_cooldown_ms as i64 {
            return OrderManagerDecision {
                action: "hold".to_string(),
                reason: "cancel_replace_cooldown".to_string(),
                should_edit: false,
                should_cancel_replace: false,
                target_price: order.price,
                target_size: order.size,
            };
        }

        OrderManagerDecision {
            action: "cancel_replace".to_string(),
            reason: "price_delta_above_threshold".to_string(),
            should_edit: false,
            should_cancel_replace: true,
            target_price: safe_price,
            target_size: desired_size,
        }
    }
}

pub fn reprice_post_only(
    side: Side,
    desired_price: f64,
    best_bid: f64,
    best_ask: f64,
    tick: f64,
    max_attempts: u32,
) -> Option<f64> {
    if desired_price <= 0.0 || best_bid <= 0.0 || best_ask <= 0.0 || tick <= 0.0 {
        return None;
    }

    let mut px = desired_price;
    for _ in 0..max_attempts.max(1) {
        match side {
            Side::Buy => {
                if px < best_ask {
                    return Some(round_to_tick(px, tick, false));
                }
                px -= tick;
            }
            Side::Sell => {
                if px > best_bid {
                    return Some(round_to_tick(px, tick, true));
                }
                px += tick;
            }
        }
    }

    match side {
        Side::Buy => Some(round_to_tick((best_ask - tick).max(tick), tick, false)),
        Side::Sell => Some(round_to_tick(best_bid + tick, tick, true)),
    }
}

fn round_to_tick(value: f64, tick: f64, up: bool) -> f64 {
    if up {
        (value / tick).ceil() * tick
    } else {
        (value / tick).floor() * tick
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> OrderManagerConfig {
        OrderManagerConfig {
            preview_required: true,
            max_reprice_attempts: 3,
            edit_vs_replace_threshold_bps: 5.0,
            cancel_replace_cooldown_ms: 250,
            min_rest_ms: 400,
        }
    }

    #[test]
    fn reprice_buy_does_not_cross_ask() {
        let px = reprice_post_only(Side::Buy, 100.5, 100.0, 100.1, 0.01, 3).unwrap();
        assert!(px < 100.1);
    }

    #[test]
    fn reprice_sell_does_not_cross_bid() {
        let px = reprice_post_only(Side::Sell, 99.5, 100.0, 100.1, 0.01, 3).unwrap();
        assert!(px > 100.0);
    }

    #[test]
    fn decision_is_hold_in_min_rest_window() {
        let manager = OrderManager::new(cfg());
        let existing = RestingOrder {
            order_id: "o1".to_string(),
            side: Side::Buy,
            price: 100.0,
            size: 1.0,
            submitted_ts_ms: 1_000,
            last_replace_ts_ms: 1_000,
        };
        let d = manager.decide(
            Some(&existing),
            Side::Buy,
            100.0,
            1.0,
            TopOfBook {
                best_bid: 99.9,
                best_ask: 100.1,
                tick_size: 0.01,
            },
            1_100,
        );
        assert_eq!(d.action, "hold");
    }
}
