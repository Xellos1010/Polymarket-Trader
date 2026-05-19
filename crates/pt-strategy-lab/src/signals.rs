use crate::indicators::{
    adx, atr, closes, cumulative_vwap, ichimoku, ma, macd, rolling_std, rsi, session_vwap, sma,
    stoch_rsi, volumes,
};
use crate::types::{
    Candle, FusionDecision, IndicatorSignal, RegimeState, StrategyProfile, TradeAction,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

// ---------------------------------------------------------------------------
// SignalConfig — configuration for standalone signal evaluation functions
// ---------------------------------------------------------------------------

fn default_session_vwap_threshold() -> f64 {
    0.005
}
fn default_session_start_hour() -> u32 {
    0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalConfig {
    #[serde(default = "default_session_vwap_threshold")]
    pub session_vwap_threshold: f64,
    #[serde(default = "default_session_start_hour")]
    pub session_start_hour_utc: u32,
}

impl Default for SignalConfig {
    fn default() -> Self {
        Self {
            session_vwap_threshold: default_session_vwap_threshold(),
            session_start_hour_utc: default_session_start_hour(),
        }
    }
}

// ---------------------------------------------------------------------------
// session_vwap_signal — mean-reversion signal vs. session VWAP
// ---------------------------------------------------------------------------

/// Returns an `IndicatorSignal` when the last candle's close deviates from its
/// session VWAP by more than `cfg.session_vwap_threshold` (a fractional value,
/// e.g. 0.005 = 0.5%).  Returns `None` when the deviation is within threshold
/// or there are no candles with a valid VWAP.
///
/// Bias convention (mean-reversion):
///   price above VWAP → bearish bias (negative)
///   price below VWAP → bullish bias (positive)
pub fn session_vwap_signal(candles: &[Candle], cfg: &SignalConfig) -> Option<IndicatorSignal> {
    if candles.is_empty() {
        return None;
    }
    let vwaps = session_vwap(candles, cfg.session_start_hour_utc);
    let last_vwap = vwaps.last()?.as_ref()?;
    let last_close = candles.last()?.close;
    let deviation = (last_close - last_vwap) / last_vwap;
    if deviation.abs() < cfg.session_vwap_threshold {
        return None;
    }
    // Mean-reversion: above VWAP is bearish, below is bullish.
    let bias = if deviation > 0.0 { -1.0_f64 } else { 1.0_f64 };
    let confidence = (deviation.abs() / cfg.session_vwap_threshold).min(1.0);
    let regime_vote = if deviation > 0.0 {
        RegimeState::Bear
    } else {
        RegimeState::Bull
    };
    Some(IndicatorSignal {
        name: "session_vwap".to_string(),
        bias,
        confidence,
        regime_vote,
        metadata: json!({
            "vwap": last_vwap,
            "close": last_close,
            "deviation": deviation,
        }),
    })
}

fn clamp(v: f64, lo: f64, hi: f64) -> f64 {
    v.max(lo).min(hi)
}

fn regime_from_votes(votes: &[(&RegimeState, f64)]) -> RegimeState {
    let mut bull = 0.0;
    let mut bear = 0.0;
    let mut neutral = 0.0;
    for (vote, weight) in votes {
        match vote {
            RegimeState::Bull => bull += *weight,
            RegimeState::Bear => bear += *weight,
            RegimeState::Neutral => neutral += *weight,
        }
    }
    if bull > bear && bull >= neutral {
        RegimeState::Bull
    } else if bear > bull && bear >= neutral {
        RegimeState::Bear
    } else {
        RegimeState::Neutral
    }
}

pub fn build_decisions(candles: &[Candle], profile: &StrategyProfile) -> Vec<FusionDecision> {
    if candles.is_empty() {
        return Vec::new();
    }

    let close = closes(candles);
    let vol = volumes(candles);
    let i_cfg = &profile.indicators;

    let ma_fast = ma(&close, &vol, i_cfg.ma_fast.max(1), &i_cfg.ma_type);
    let ma_slow = ma(&close, &vol, i_cfg.ma_slow.max(2), &i_cfg.ma_type);
    let rsi_v = rsi(&close, i_cfg.rsi_len.max(2));
    let bb_basis = ma(
        &close,
        &vol,
        i_cfg.bb_len.max(2),
        &crate::types::MaType::Vwma,
    );
    let bb_std = rolling_std(&close, i_cfg.bb_len.max(2));
    let ichi = ichimoku(
        candles,
        i_cfg.ichimoku_conv,
        i_cfg.ichimoku_base,
        i_cfg.ichimoku_span_b,
    );
    let (macd_line, macd_sig, macd_hist) =
        macd(&close, i_cfg.macd_fast, i_cfg.macd_slow, i_cfg.macd_signal);
    let (adx_v, plus_di, minus_di) = adx(candles, i_cfg.adx_len.max(2));
    let atr_v = atr(candles, i_cfg.atr_len.max(2));
    let vwap_v = cumulative_vwap(candles);
    let stoch = stoch_rsi(
        &rsi_v,
        i_cfg.stoch_rsi_len.max(2),
        i_cfg.stoch_rsi_smooth.max(1),
    );
    let vol_sma = sma(&vol, i_cfg.volume_lookback.max(2));

    let mut out = Vec::with_capacity(candles.len());

    for i in 0..candles.len() {
        let px = close[i];
        let mut signals: Vec<IndicatorSignal> = Vec::new();

        if let (Some(f), Some(s)) = (ma_fast[i], ma_slow[i]) {
            let rel = if s.abs() > 1e-12 { (f - s) / s } else { 0.0 };
            let bias = clamp(rel * 30.0, -1.0, 1.0);
            let conf = clamp(rel.abs() * 40.0, 0.0, 1.0);
            let vote = if rel > 0.001 {
                RegimeState::Bull
            } else if rel < -0.001 {
                RegimeState::Bear
            } else {
                RegimeState::Neutral
            };
            signals.push(IndicatorSignal {
                name: "ma_regime".to_string(),
                bias,
                confidence: conf,
                regime_vote: vote,
                metadata: json!({"fast": f, "slow": s, "rel": rel}),
            });
        }

        if let Some(rv) = rsi_v[i] {
            let mut bias = (50.0 - rv) / 25.0;
            bias = clamp(bias, -1.0, 1.0);
            let conf = if rv <= i_cfg.rsi_oversold || rv >= i_cfg.rsi_overbought {
                0.9
            } else {
                clamp(((rv - 50.0).abs() / 50.0) * 0.8, 0.05, 0.8)
            };
            let vote = if rv > 52.0 {
                RegimeState::Bull
            } else if rv < 48.0 {
                RegimeState::Bear
            } else {
                RegimeState::Neutral
            };
            signals.push(IndicatorSignal {
                name: "rsi".to_string(),
                bias,
                confidence: conf,
                regime_vote: vote,
                metadata: json!({"rsi": rv}),
            });
        }

        if let (Some(basis), Some(std)) = (bb_basis[i], bb_std[i]) {
            let dist = if std > 0.0 {
                (px - basis) / (std * i_cfg.bb_fib_multiplier.max(0.1))
            } else {
                0.0
            };
            let bias = clamp(-dist, -1.0, 1.0);
            let conf = clamp(dist.abs() / 2.0, 0.05, 1.0);
            let vote = if px > basis {
                RegimeState::Bull
            } else if px < basis {
                RegimeState::Bear
            } else {
                RegimeState::Neutral
            };
            signals.push(IndicatorSignal {
                name: "fib_bb".to_string(),
                bias,
                confidence: conf,
                regime_vote: vote,
                metadata: json!({"basis": basis, "std": std, "dist": dist}),
            });
        }

        if i >= i_cfg.ichimoku_displacement {
            let idx = i - i_cfg.ichimoku_displacement;
            if let (Some(sa), Some(sb), Some(conv), Some(base)) = (
                ichi.span_a[idx],
                ichi.span_b[idx],
                ichi.conversion[i],
                ichi.base[i],
            ) {
                let cloud_top = sa.max(sb);
                let cloud_bot = sa.min(sb);
                let vote = if px > cloud_top {
                    RegimeState::Bull
                } else if px < cloud_bot {
                    RegimeState::Bear
                } else {
                    RegimeState::Neutral
                };
                let span = (cloud_top - cloud_bot).abs().max(1e-9);
                let z = if px > cloud_top {
                    (px - cloud_top) / span
                } else if px < cloud_bot {
                    -((cloud_bot - px) / span)
                } else {
                    0.0
                };
                let cross = if conv > base { 1.0 } else { -1.0 };
                let bias = clamp((z * 0.7) + (cross * 0.3), -1.0, 1.0);
                let conf = clamp(z.abs().min(1.0) * 0.7 + 0.3, 0.05, 1.0);
                signals.push(IndicatorSignal {
                    name: "ichimoku".to_string(),
                    bias,
                    confidence: conf,
                    regime_vote: vote,
                    metadata: json!({"span_a": sa, "span_b": sb, "conv": conv, "base": base}),
                });
            }
        }

        if let (Some(m), Some(s), Some(h)) = (macd_line[i], macd_sig[i], macd_hist[i]) {
            let bias = clamp((h / (px * 0.002).max(1e-9)) * 0.5, -1.0, 1.0);
            let conf = clamp((h.abs() / (px * 0.001).max(1e-9)).min(1.0), 0.05, 1.0);
            let vote = if m >= s {
                RegimeState::Bull
            } else {
                RegimeState::Bear
            };
            signals.push(IndicatorSignal {
                name: "macd".to_string(),
                bias,
                confidence: conf,
                regime_vote: vote,
                metadata: json!({"macd": m, "signal": s, "hist": h}),
            });
        }

        if let (Some(adx_v), Some(pdi), Some(mdi)) = (adx_v[i], plus_di[i], minus_di[i]) {
            let trend = (pdi - mdi) / (pdi + mdi + 1e-9);
            let bias = clamp(trend, -1.0, 1.0);
            let conf = clamp(adx_v / 50.0, 0.05, 1.0);
            let vote = if trend > 0.05 {
                RegimeState::Bull
            } else if trend < -0.05 {
                RegimeState::Bear
            } else {
                RegimeState::Neutral
            };
            signals.push(IndicatorSignal {
                name: "adx".to_string(),
                bias,
                confidence: conf,
                regime_vote: vote,
                metadata: json!({"adx": adx_v, "plus_di": pdi, "minus_di": mdi}),
            });
        }

        if let Some(atr_now) = atr_v[i] {
            let ret = if i > 0 {
                (px - close[i - 1]) / close[i - 1]
            } else {
                0.0
            };
            let norm = atr_now / px.max(1e-9);
            let bias = clamp(ret / norm.max(1e-6), -1.0, 1.0);
            let conf = clamp(norm * 20.0, 0.05, 1.0);
            let vote = if ret > 0.0 {
                RegimeState::Bull
            } else if ret < 0.0 {
                RegimeState::Bear
            } else {
                RegimeState::Neutral
            };
            signals.push(IndicatorSignal {
                name: "atr".to_string(),
                bias,
                confidence: conf,
                regime_vote: vote,
                metadata: json!({"atr": atr_now, "ret": ret}),
            });
        }

        if let Some(vs) = vol_sma[i] {
            let vol_ratio = if vs > 0.0 { vol[i] / vs } else { 1.0 };
            let candle_dir = if candles[i].close >= candles[i].open {
                1.0
            } else {
                -1.0
            };
            let bias = clamp(candle_dir * (vol_ratio - 1.0), -1.0, 1.0);
            let conf = clamp((vol_ratio - 1.0).abs(), 0.05, 1.0);
            let vote = if candle_dir > 0.0 {
                RegimeState::Bull
            } else {
                RegimeState::Bear
            };
            signals.push(IndicatorSignal {
                name: "volume".to_string(),
                bias,
                confidence: conf,
                regime_vote: vote,
                metadata: json!({"volume": vol[i], "volume_sma": vs, "ratio": vol_ratio}),
            });
        }

        if let Some(vwap) = vwap_v[i] {
            let dev = (px - vwap) / vwap.max(1e-9);
            let bias = clamp(-dev * 12.0, -1.0, 1.0);
            let conf = clamp(dev.abs() * 20.0, 0.05, 1.0);
            let vote = if px > vwap {
                RegimeState::Bull
            } else if px < vwap {
                RegimeState::Bear
            } else {
                RegimeState::Neutral
            };
            signals.push(IndicatorSignal {
                name: "vwap_dev".to_string(),
                bias,
                confidence: conf,
                regime_vote: vote,
                metadata: json!({"vwap": vwap, "dev": dev}),
            });
        }

        if let Some(sto) = stoch[i] {
            let bias = clamp((0.5 - sto) * 2.0, -1.0, 1.0);
            let conf = clamp((sto - 0.5).abs() * 2.0, 0.05, 1.0);
            let vote = if sto > 0.55 {
                RegimeState::Bull
            } else if sto < 0.45 {
                RegimeState::Bear
            } else {
                RegimeState::Neutral
            };
            signals.push(IndicatorSignal {
                name: "stoch_rsi".to_string(),
                bias,
                confidence: conf,
                regime_vote: vote,
                metadata: json!({"stoch_rsi": sto}),
            });
        }

        let mut numerator = 0.0;
        let mut denominator = 1e-9;
        let mut aligned = 0usize;
        let mut votes: Vec<(&RegimeState, f64)> = Vec::new();

        for s in &signals {
            let w = profile.weights.get(&s.name).copied().unwrap_or(0.0);
            numerator += w * s.bias * s.confidence;
            // abs() normalises: weight magnitude controls influence, weight sign flips bias direction.
            denominator += w.abs() * s.confidence;
            votes.push((&s.regime_vote, (w.abs() * s.confidence).max(0.01)));
        }

        let mut score = clamp(numerator / denominator, -1.0, 1.0);
        let regime = regime_from_votes(&votes);

        for s in &signals {
            if (score > 0.0 && s.bias > 0.1) || (score < 0.0 && s.bias < -0.1) {
                aligned += 1;
            }
        }

        match regime {
            RegimeState::Bull if score < 0.0 => score *= 0.5,
            RegimeState::Bear if score > 0.0 => score *= 0.5,
            RegimeState::Neutral => score /= profile.fusion.neutral_regime_multiplier.max(1.0),
            _ => {}
        }

        let action = if score >= profile.fusion.buy_threshold
            && aligned >= profile.fusion.min_confluence
        {
            TradeAction::Buy
        } else if score <= profile.fusion.sell_threshold && aligned >= profile.fusion.min_confluence
        {
            TradeAction::Sell
        } else {
            TradeAction::Hold
        };

        out.push(FusionDecision {
            ts_ms: candles[i].ts_ms,
            score,
            regime,
            action,
            confluence: aligned,
            indicators: signals,
        });
    }

    out
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Candle;

    #[test]
    fn session_vwap_signal_above_threshold_emits_bearish() {
        // Three candles spread across session boundaries.
        // VWAP for the last session ≈ 1.01 (only last candle in its session
        // given that each candle lands on a different day boundary).
        // last close 1.01 → deviation = 0 within session; to force > threshold
        // we use a single-candle "session" where close > typ (achieved via
        // high/low asymmetry).
        //
        // Simpler: put all three candles in one session (same day) so VWAP
        // accumulates, then make the last close diverge significantly.
        let candles = vec![
            Candle { ts_ms: 0,              open: 1.0, high: 1.0,  low: 1.0, close: 1.0,  volume: 100.0 },
            Candle { ts_ms: 3_600_000,      open: 1.0, high: 1.0,  low: 1.0, close: 1.0,  volume: 100.0 },
            Candle { ts_ms: 7_200_000,      open: 1.0, high: 1.02, low: 1.0, close: 1.02, volume: 100.0 },
        ];
        // VWAP after 3 candles in same session:
        //   typ0 = 1.0, typ1 = 1.0, typ2 = (1.02+1.0+1.02)/3 ≈ 1.01333
        //   pv = 100*1.0 + 100*1.0 + 100*1.01333 = 301.333, vv=300
        //   vwap ≈ 1.00444
        // deviation = (1.02 - 1.00444) / 1.00444 ≈ 0.0155 > 0.005 → signal
        let cfg = SignalConfig {
            session_vwap_threshold: 0.005,
            session_start_hour_utc: 0,
        };
        let signal = session_vwap_signal(&candles, &cfg);
        assert!(signal.is_some(), "should emit signal when deviation exceeds threshold");
        let s = signal.unwrap();
        assert!(s.bias < 0.0, "price above vwap should produce bearish (negative) bias");
    }

    #[test]
    fn session_vwap_signal_within_threshold_returns_none() {
        // Two candles in the same session, prices nearly identical → tiny deviation.
        let candles = vec![
            Candle { ts_ms: 0,         open: 1.0, high: 1.0,   low: 1.0, close: 1.0,   volume: 100.0 },
            Candle { ts_ms: 3_600_000, open: 1.0, high: 1.001, low: 1.0, close: 1.0001, volume: 100.0 },
        ];
        let cfg = SignalConfig {
            session_vwap_threshold: 0.005,
            session_start_hour_utc: 0,
        };
        let signal = session_vwap_signal(&candles, &cfg);
        assert!(signal.is_none(), "deviation within threshold should return None");
    }

    #[test]
    fn session_vwap_signal_empty_candles_returns_none() {
        let cfg = SignalConfig::default();
        assert!(session_vwap_signal(&[], &cfg).is_none());
    }

    #[test]
    fn session_vwap_signal_below_threshold_emits_bullish() {
        // Last close well below VWAP → bullish bias.
        let candles = vec![
            Candle { ts_ms: 0,         open: 1.0, high: 1.05, low: 1.0, close: 1.05, volume: 200.0 },
            Candle { ts_ms: 3_600_000, open: 1.0, high: 1.05, low: 1.0, close: 0.98, volume: 100.0 },
        ];
        let cfg = SignalConfig { session_vwap_threshold: 0.005, session_start_hour_utc: 0 };
        let signal = session_vwap_signal(&candles, &cfg);
        assert!(signal.is_some(), "should emit signal when price is below vwap by more than threshold");
        let s = signal.unwrap();
        assert!(s.bias > 0.0, "price below vwap should produce bullish (positive) bias");
    }
}
