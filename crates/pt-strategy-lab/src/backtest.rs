use crate::indicators::max_drawdown;
use crate::signals::build_decisions;
use crate::types::{
    Candle, EquityPoint, StrategyProfile, StrategyRunReport, TradeAction, TradeFill,
};
use chrono::Utc;

fn apply_costs(notional: f64, fee_bps: f64, slippage_bps: f64, fixed: f64) -> (f64, f64) {
    let fee = notional * (fee_bps / 10_000.0) + fixed;
    let slippage = notional * (slippage_bps / 10_000.0);
    (fee, slippage)
}

pub fn run_backtest(profile: &StrategyProfile, candles: &[Candle]) -> StrategyRunReport {
    let decisions = build_decisions(candles, profile);

    let mut cash = profile.starting_equity;
    let mut qty = 0.0_f64;
    let mut fills: Vec<TradeFill> = Vec::new();
    let mut equity_curve: Vec<EquityPoint> = Vec::new();

    for i in 0..candles.len() {
        if i + 1 < candles.len() {
            let next_open = candles[i + 1].open;
            match decisions[i].action {
                TradeAction::Buy => {
                    if qty <= 0.0 && cash > 0.0 {
                        let notional = cash;
                        let (fee, slippage) = apply_costs(
                            notional,
                            profile.costs.fee_bps,
                            profile.costs.slippage_bps,
                            profile.costs.fixed_trade_cost,
                        );
                        let effective_cash = (cash - fee - slippage).max(0.0);
                        let buy_qty = if next_open > 0.0 {
                            effective_cash / next_open
                        } else {
                            0.0
                        };
                        if buy_qty > 0.0 {
                            qty = buy_qty;
                            cash = 0.0;
                            fills.push(TradeFill {
                                ts_ms: candles[i + 1].ts_ms,
                                action: TradeAction::Buy,
                                price: next_open,
                                qty: buy_qty,
                                notional,
                                fee_cost: fee,
                                slippage_cost: slippage,
                            });
                        }
                    }
                }
                TradeAction::Sell => {
                    if qty > 0.0 {
                        let notional = qty * next_open;
                        let (fee, slippage) = apply_costs(
                            notional,
                            profile.costs.fee_bps,
                            profile.costs.slippage_bps,
                            profile.costs.fixed_trade_cost,
                        );
                        cash = (notional - fee - slippage).max(0.0);
                        fills.push(TradeFill {
                            ts_ms: candles[i + 1].ts_ms,
                            action: TradeAction::Sell,
                            price: next_open,
                            qty,
                            notional,
                            fee_cost: fee,
                            slippage_cost: slippage,
                        });
                        qty = 0.0;
                    }
                }
                TradeAction::Hold => {}
            }
        }

        let mark = if qty > 0.0 {
            qty * candles[i].close
        } else {
            cash
        };
        equity_curve.push(EquityPoint {
            ts_ms: candles[i].ts_ms,
            equity: mark,
        });
    }

    if qty > 0.0 {
        let last = candles.last().cloned();
        if let Some(last) = last {
            let notional = qty * last.close;
            let (fee, slippage) = apply_costs(
                notional,
                profile.costs.fee_bps,
                profile.costs.slippage_bps,
                profile.costs.fixed_trade_cost,
            );
            cash = (notional - fee - slippage).max(0.0);
            fills.push(TradeFill {
                ts_ms: last.ts_ms,
                action: TradeAction::Sell,
                price: last.close,
                qty,
                notional,
                fee_cost: fee,
                slippage_cost: slippage,
            });
            if let Some(last_point) = equity_curve.last_mut() {
                last_point.equity = cash;
            }
        }
    }

    let equity_values: Vec<f64> = equity_curve.iter().map(|p| p.equity).collect();
    let final_equity = equity_values
        .last()
        .copied()
        .unwrap_or(profile.starting_equity);
    let pnl = final_equity - profile.starting_equity;
    let total_return_pct = if profile.starting_equity > 0.0 {
        pnl / profile.starting_equity
    } else {
        0.0
    };

    let mut wins = 0usize;
    let mut total_closed = 0usize;
    let mut last_buy: Option<&TradeFill> = None;
    for fill in &fills {
        match fill.action {
            TradeAction::Buy => last_buy = Some(fill),
            TradeAction::Sell => {
                if let Some(buy) = last_buy {
                    total_closed += 1;
                    if fill.price > buy.price {
                        wins += 1;
                    }
                }
                last_buy = None;
            }
            TradeAction::Hold => {}
        }
    }

    StrategyRunReport {
        run_id: format!("run-{}", Utc::now().timestamp_millis()),
        profile_id: profile.profile_id.clone(),
        product_id: profile.product_id.clone(),
        granularity_sec: profile.granularity_sec,
        started_ts_ms: Utc::now().timestamp_millis(),
        total_return_pct,
        max_drawdown_pct: max_drawdown(&equity_values),
        trades: fills.len(),
        win_rate: if total_closed > 0 {
            wins as f64 / total_closed as f64
        } else {
            0.0
        },
        pnl,
        candles: candles.to_vec(),
        decisions,
        fills,
        equity_curve,
    }
}
