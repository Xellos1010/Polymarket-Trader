use chrono::Utc;
use pt_core::{ExecutionReport, ExecutionStatus, MarketSnapshot, QuoteIntent, Side, Venue};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayFrame {
    pub snapshot: MarketSnapshot,
    #[serde(default)]
    pub bias: f64,
}

pub fn load_replay_frames(path: &str) -> anyhow::Result<Vec<ReplayFrame>> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let mut out = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        out.push(serde_json::from_str::<ReplayFrame>(&line)?);
    }
    Ok(out)
}

#[derive(Debug, Default)]
pub struct PaperSimulator {
    pub inventory_by_market: HashMap<String, f64>,
    pub realized_pnl: f64,
}

impl PaperSimulator {
    pub fn apply_quote(
        &mut self,
        quote: &QuoteIntent,
        frame: &MarketSnapshot,
    ) -> Vec<ExecutionReport> {
        let mut reports = Vec::new();

        // Fill model: if bid is aggressive enough against top ask, fill buy.
        if quote.bid_px >= frame.ask {
            let qty = quote.bid_sz;
            *self
                .inventory_by_market
                .entry(quote.market_id.clone())
                .or_insert(0.0) += qty;
            reports.push(ExecutionReport {
                venue: Venue::Sim,
                order_id: format!("replay-buy-{}", Utc::now().timestamp_millis()),
                market_id: Some(quote.market_id.clone()),
                status: ExecutionStatus::Filled,
                side: Side::Buy,
                filled_qty: qty,
                avg_px: frame.ask,
                ts: Utc::now(),
                details: Some("paper buy fill".to_string()),
            });
        }

        // Fill model: if ask is aggressive enough against top bid, fill sell.
        if quote.ask_px <= frame.bid {
            let qty = quote.ask_sz;
            let inv = self
                .inventory_by_market
                .entry(quote.market_id.clone())
                .or_insert(0.0);
            let sell_qty = qty.min((*inv).max(0.0));
            *inv -= sell_qty;
            self.realized_pnl += sell_qty * frame.bid;

            reports.push(ExecutionReport {
                venue: Venue::Sim,
                order_id: format!("replay-sell-{}", Utc::now().timestamp_millis()),
                market_id: Some(quote.market_id.clone()),
                status: ExecutionStatus::Filled,
                side: Side::Sell,
                filled_qty: sell_qty,
                avg_px: frame.bid,
                ts: Utc::now(),
                details: Some("paper sell fill".to_string()),
            });
        }

        reports
    }
}
