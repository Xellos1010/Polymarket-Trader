use chrono::{Datelike, Utc};
use parking_lot::RwLock;
use pt_core::{
    KillSwitchState, PtError, PtResult, QuoteIntent, RiskConfig, RiskDecision, RiskState,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct InternalRiskState {
    kill: KillSwitchState,
    day_key: (i32, u32, u32),
    daily_pnl: f64,
    open_notional_total: f64,
    open_notional_per_market: HashMap<String, f64>,
    unhedged_delta: f64,
    stale_books: usize,
    open_markets: usize,
}

impl Default for InternalRiskState {
    fn default() -> Self {
        let now = Utc::now().date_naive();
        Self {
            kill: KillSwitchState::Running,
            day_key: (now.year(), now.month(), now.day()),
            daily_pnl: 0.0,
            open_notional_total: 0.0,
            open_notional_per_market: HashMap::new(),
            unhedged_delta: 0.0,
            stale_books: 0,
            open_markets: 0,
        }
    }
}

#[derive(Debug)]
pub struct RiskEngine {
    cfg: RiskConfig,
    deployed_capital: f64,
    state: RwLock<InternalRiskState>,
}

impl RiskEngine {
    pub fn new(cfg: RiskConfig, deployed_capital: f64) -> Self {
        Self {
            cfg,
            deployed_capital,
            state: RwLock::new(InternalRiskState::default()),
        }
    }

    pub fn evaluate_quote(&self, quote: &QuoteIntent, stale_book_ms: u64) -> RiskDecision {
        self.reset_day_if_needed();

        let now = Utc::now();
        let mut st = self.state.write();

        if st.kill != KillSwitchState::Running {
            return deny("KILLSWITCH_ACTIVE", Some("killswitch"));
        }

        if stale_book_ms > self.cfg.stale_book_threshold_ms {
            st.stale_books += 1;
            return deny("STALE_ORDERBOOK", Some("stale_book_threshold_ms"));
        }

        let max_daily_loss = self.deployed_capital * self.cfg.daily_loss_limit_pct;
        if st.daily_pnl <= -max_daily_loss {
            st.kill = KillSwitchState::AutoHalt;
            return deny("DAILY_LOSS_LIMIT", Some("daily_loss_limit_pct"));
        }

        if st.open_notional_total + quote.bid_sz > self.cfg.max_total_open_notional {
            return deny("MAX_TOTAL_OPEN_NOTIONAL", Some("max_total_open_notional"));
        }

        let current_market_notional = *st
            .open_notional_per_market
            .get(&quote.market_id)
            .unwrap_or(&0.0);
        if current_market_notional + quote.bid_sz > self.cfg.max_notional_per_market {
            return deny("MAX_NOTIONAL_PER_MARKET", Some("max_notional_per_market"));
        }

        if st.open_markets >= self.cfg.max_markets_quoted_simultaneously
            && !st.open_notional_per_market.contains_key(&quote.market_id)
        {
            return deny(
                "MAX_MARKETS_QUOTED",
                Some("max_markets_quoted_simultaneously"),
            );
        }

        if st.unhedged_delta.abs() > self.cfg.max_unhedged_delta {
            st.kill = KillSwitchState::SafeMode;
            return deny("MAX_UNHEDGED_DELTA", Some("max_unhedged_delta"));
        }

        RiskDecision {
            allow: true,
            reason_code: "ALLOW".to_string(),
            limit_name: None,
            ts: now,
        }
    }

    pub fn reserve_quote_exposure(&self, quote: &QuoteIntent) {
        let mut st = self.state.write();
        st.open_notional_total += quote.bid_sz;
        let is_new_market = !st.open_notional_per_market.contains_key(&quote.market_id);
        if is_new_market {
            st.open_markets += 1;
        }
        let e = st
            .open_notional_per_market
            .entry(quote.market_id.clone())
            .or_insert(0.0);
        *e += quote.bid_sz;
    }

    pub fn release_market_exposure(&self, market_id: &str) {
        let mut st = self.state.write();
        if let Some(v) = st.open_notional_per_market.remove(market_id) {
            st.open_notional_total = (st.open_notional_total - v).max(0.0);
            st.open_markets = st.open_markets.saturating_sub(1);
        }
    }

    pub fn update_unhedged_delta(&self, delta: f64) {
        self.state.write().unhedged_delta = delta;
    }

    pub fn apply_realized_pnl(&self, pnl_delta: f64) {
        self.reset_day_if_needed();
        self.state.write().daily_pnl += pnl_delta;
    }

    pub fn manual_halt(&self) {
        self.state.write().kill = KillSwitchState::ManualHalt;
    }

    pub fn resume(&self) -> PtResult<()> {
        let mut st = self.state.write();
        if st.kill == KillSwitchState::AutoHalt {
            return Err(PtError::Risk(
                "cannot resume from AUTO_HALT without explicit reset".to_string(),
            ));
        }
        st.kill = KillSwitchState::Running;
        Ok(())
    }

    pub fn flatten_safe_mode(&self) {
        self.state.write().kill = KillSwitchState::SafeMode;
    }

    pub fn force_reset(&self) {
        let mut st = self.state.write();
        st.kill = KillSwitchState::Running;
        st.daily_pnl = 0.0;
    }

    pub fn snapshot(&self) -> RiskState {
        let st = self.state.read();
        RiskState {
            killswitch: format!("{:?}", st.kill),
            daily_pnl: st.daily_pnl,
            max_daily_loss: self.deployed_capital * self.cfg.daily_loss_limit_pct,
            open_notional: st.open_notional_total,
            unhedged_delta: st.unhedged_delta,
            open_markets: st.open_markets,
            stale_books: st.stale_books,
            last_update_ms: Utc::now().timestamp_millis(),
        }
    }

    fn reset_day_if_needed(&self) {
        let now = Utc::now().date_naive();
        let key = (now.year(), now.month(), now.day());
        let mut st = self.state.write();
        if st.day_key != key {
            st.day_key = key;
            st.daily_pnl = 0.0;
            st.stale_books = 0;
        }
    }
}

fn deny(reason: &str, limit: Option<&str>) -> RiskDecision {
    RiskDecision {
        allow: false,
        reason_code: reason.to_string(),
        limit_name: limit.map(str::to_string),
        ts: Utc::now(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_when_stale() {
        let cfg = RiskConfig {
            daily_loss_limit_pct: 0.02,
            max_notional_per_market: 5.0,
            max_total_open_notional: 20.0,
            max_markets_quoted_simultaneously: 2,
            max_unhedged_delta: 10.0,
            max_order_age_secs: 20,
            stale_book_threshold_ms: 400,
            min_expected_net: 0.002,
        };

        let engine = RiskEngine::new(cfg, 50.0);
        let q = QuoteIntent {
            market_id: "m1".into(),
            token_id: "t1".into(),
            bid_px: 0.49,
            ask_px: 0.51,
            bid_sz: 5.0,
            ask_sz: 5.0,
            ttl_ms: 20_000,
            expected_net: 0.003,
        };

        let d = engine.evaluate_quote(&q, 500);
        assert!(!d.allow);
    }
}
