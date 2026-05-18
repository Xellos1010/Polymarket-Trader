use crate::types::{Candle, MaType};

/// Three aligned optional indicator series (e.g. MACD line/signal/hist or ADX components).
type OptionSeriesTriple = (Vec<Option<f64>>, Vec<Option<f64>>, Vec<Option<f64>>);

pub fn closes(candles: &[Candle]) -> Vec<f64> {
    candles.iter().map(|c| c.close).collect()
}

pub fn highs(candles: &[Candle]) -> Vec<f64> {
    candles.iter().map(|c| c.high).collect()
}

pub fn lows(candles: &[Candle]) -> Vec<f64> {
    candles.iter().map(|c| c.low).collect()
}

pub fn volumes(candles: &[Candle]) -> Vec<f64> {
    candles.iter().map(|c| c.volume).collect()
}

pub fn sma(values: &[f64], len: usize) -> Vec<Option<f64>> {
    if len == 0 {
        return vec![None; values.len()];
    }
    let mut out = vec![None; values.len()];
    if values.len() < len {
        return out;
    }
    let mut sum: f64 = values[..len].iter().sum();
    out[len - 1] = Some(sum / len as f64);
    for i in len..values.len() {
        sum += values[i] - values[i - len];
        out[i] = Some(sum / len as f64);
    }
    out
}

pub fn ema(values: &[f64], len: usize) -> Vec<Option<f64>> {
    if len == 0 || values.is_empty() {
        return vec![None; values.len()];
    }
    let mut out = vec![None; values.len()];
    let k = 2.0 / (len as f64 + 1.0);
    let mut ema_val = values[0];
    out[0] = Some(ema_val);
    for (i, v) in values.iter().enumerate().skip(1) {
        ema_val = (*v * k) + (ema_val * (1.0 - k));
        out[i] = Some(ema_val);
    }
    out
}

fn ema_seeded(values: &[f64], len: usize) -> Vec<Option<f64>> {
    if len == 0 || values.is_empty() {
        return vec![None; values.len()];
    }
    let mut out = vec![None; values.len()];
    if values.len() < len {
        return out;
    }
    let k = 2.0 / (len as f64 + 1.0);
    let seed = values[..len].iter().sum::<f64>() / len as f64;
    let mut ema_val = seed;
    out[len - 1] = Some(seed);
    for i in len..values.len() {
        ema_val = (values[i] * k) + (ema_val * (1.0 - k));
        out[i] = Some(ema_val);
    }
    out
}

pub fn rma(values: &[f64], len: usize) -> Vec<Option<f64>> {
    if len == 0 || values.is_empty() {
        return vec![None; values.len()];
    }
    let mut out = vec![None; values.len()];
    let mut avg = values[0];
    out[0] = Some(avg);
    for (i, v) in values.iter().enumerate().skip(1) {
        avg = ((avg * (len as f64 - 1.0)) + *v) / len as f64;
        out[i] = Some(avg);
    }
    out
}

pub fn wma(values: &[f64], len: usize) -> Vec<Option<f64>> {
    if len == 0 {
        return vec![None; values.len()];
    }
    let mut out = vec![None; values.len()];
    let denom: f64 = (1..=len).map(|v| v as f64).sum();
    if values.len() < len {
        return out;
    }
    for (i, out_slot) in out.iter_mut().enumerate().skip(len - 1) {
        let mut acc = 0.0;
        for w in 0..len {
            let idx = i + 1 - len + w;
            let weight = (w + 1) as f64;
            acc += values[idx] * weight;
        }
        *out_slot = Some(acc / denom);
    }
    out
}

pub fn vwma(values: &[f64], volumes: &[f64], len: usize) -> Vec<Option<f64>> {
    if len == 0 || values.len() != volumes.len() {
        return vec![None; values.len()];
    }
    let mut out = vec![None; values.len()];
    if values.len() < len {
        return out;
    }
    let mut pv_sum = 0.0;
    let mut vol_sum = 0.0;
    for i in 0..len {
        pv_sum += values[i] * volumes[i];
        vol_sum += volumes[i];
    }
    if vol_sum > 0.0 {
        out[len - 1] = Some(pv_sum / vol_sum);
    }
    for i in len..values.len() {
        pv_sum += values[i] * volumes[i];
        pv_sum -= values[i - len] * volumes[i - len];
        vol_sum += volumes[i] - volumes[i - len];
        if vol_sum > 0.0 {
            out[i] = Some(pv_sum / vol_sum);
        }
    }
    out
}

pub fn zlema(values: &[f64], len: usize) -> Vec<Option<f64>> {
    if len == 0 {
        return vec![None; values.len()];
    }
    let lag = (len.saturating_sub(1)) / 2;
    let mut adjusted = vec![0.0; values.len()];
    for i in 0..values.len() {
        if i >= lag {
            adjusted[i] = values[i] + (values[i] - values[i - lag]);
        } else {
            adjusted[i] = values[i];
        }
    }
    ema(&adjusted, len)
}

pub fn dema(values: &[f64], len: usize) -> Vec<Option<f64>> {
    let ema1 = ema(values, len);
    let ema1_vals: Vec<f64> = ema1.iter().map(|v| v.unwrap_or(0.0)).collect();
    let ema2 = ema(&ema1_vals, len);
    ema1.iter()
        .zip(ema2.iter())
        .map(|(a, b)| match (a, b) {
            (Some(a), Some(b)) => Some((2.0 * a) - b),
            _ => None,
        })
        .collect()
}

pub fn tema(values: &[f64], len: usize) -> Vec<Option<f64>> {
    let ema1 = ema(values, len);
    let ema1_vals: Vec<f64> = ema1.iter().map(|v| v.unwrap_or(0.0)).collect();
    let ema2 = ema(&ema1_vals, len);
    let ema2_vals: Vec<f64> = ema2.iter().map(|v| v.unwrap_or(0.0)).collect();
    let ema3 = ema(&ema2_vals, len);
    ema1.iter()
        .zip(ema2.iter())
        .zip(ema3.iter())
        .map(|((a, b), c)| match (a, b, c) {
            (Some(a), Some(b), Some(c)) => Some((3.0 * a) - (3.0 * b) + c),
            _ => None,
        })
        .collect()
}

pub fn hma(values: &[f64], len: usize) -> Vec<Option<f64>> {
    if len == 0 {
        return vec![None; values.len()];
    }
    let half = (len / 2).max(1);
    let sqrt_len = (len as f64).sqrt().round() as usize;
    let wma_half = wma(values, half);
    let wma_full = wma(values, len);
    let mut diff = vec![0.0; values.len()];
    for i in 0..values.len() {
        let h = wma_half[i].unwrap_or(values[i]);
        let f = wma_full[i].unwrap_or(values[i]);
        diff[i] = (2.0 * h) - f;
    }
    wma(&diff, sqrt_len.max(1))
}

pub fn ma(values: &[f64], volumes: &[f64], len: usize, ma_type: &MaType) -> Vec<Option<f64>> {
    match ma_type {
        MaType::Ema => ema(values, len),
        MaType::Sma => sma(values, len),
        MaType::Wma => wma(values, len),
        MaType::Hma => hma(values, len),
        MaType::Dema => dema(values, len),
        MaType::Tema => tema(values, len),
        MaType::Vwma => vwma(values, volumes, len),
        MaType::Rma => rma(values, len),
        MaType::Zlema => zlema(values, len),
    }
}

pub fn rolling_std(values: &[f64], len: usize) -> Vec<Option<f64>> {
    if len == 0 {
        return vec![None; values.len()];
    }
    let mut out = vec![None; values.len()];
    if values.len() < len {
        return out;
    }
    for i in (len - 1)..values.len() {
        let start = i + 1 - len;
        let window = &values[start..=i];
        let mean = window.iter().sum::<f64>() / len as f64;
        let var = window
            .iter()
            .map(|v| {
                let d = *v - mean;
                d * d
            })
            .sum::<f64>()
            / len as f64;
        out[i] = Some(var.sqrt());
    }
    out
}

pub fn rsi(values: &[f64], len: usize) -> Vec<Option<f64>> {
    let mut out = vec![None; values.len()];
    if len == 0 || values.len() <= len {
        return out;
    }

    let mut gains = 0.0;
    let mut losses = 0.0;
    for i in 1..=len {
        let delta = values[i] - values[i - 1];
        gains += delta.max(0.0);
        losses += (-delta).max(0.0);
    }

    let mut avg_gain = gains / len as f64;
    let mut avg_loss = losses / len as f64;

    out[len] = Some(if avg_loss <= 1e-12 {
        100.0
    } else {
        let rs = avg_gain / avg_loss;
        100.0 - (100.0 / (1.0 + rs))
    });

    for i in (len + 1)..values.len() {
        let delta = values[i] - values[i - 1];
        let gain = delta.max(0.0);
        let loss = (-delta).max(0.0);

        avg_gain = ((avg_gain * (len as f64 - 1.0)) + gain) / len as f64;
        avg_loss = ((avg_loss * (len as f64 - 1.0)) + loss) / len as f64;

        out[i] = Some(if avg_loss <= 1e-12 {
            100.0
        } else {
            let rs = avg_gain / avg_loss;
            100.0 - (100.0 / (1.0 + rs))
        });
    }

    out
}

pub fn macd(values: &[f64], fast: usize, slow: usize, signal: usize) -> OptionSeriesTriple {
    let fast_ema = ema_seeded(values, fast.max(1));
    let slow_ema = ema_seeded(values, slow.max(1));
    let mut line = vec![None; values.len()];
    for i in 0..values.len() {
        if let (Some(f), Some(s)) = (fast_ema[i], slow_ema[i]) {
            line[i] = Some(f - s);
        }
    }
    let valid_line: Vec<f64> = line.iter().flatten().copied().collect();
    let valid_signal = ema_seeded(&valid_line, signal.max(1));
    let mut signal_line = vec![None; values.len()];
    let mut signal_iter = valid_signal.into_iter();
    for (i, value) in line.iter().enumerate() {
        if value.is_some() {
            signal_line[i] = signal_iter.next().unwrap_or(None);
        }
    }
    let hist: Vec<Option<f64>> = line
        .iter()
        .zip(signal_line.iter())
        .map(|(m, s)| match (m, s) {
            (Some(m), Some(s)) => Some(m - s),
            _ => None,
        })
        .collect();
    (line, signal_line, hist)
}

pub fn true_range(candles: &[Candle]) -> Vec<f64> {
    let mut out = vec![0.0; candles.len()];
    if candles.is_empty() {
        return out;
    }
    out[0] = candles[0].high - candles[0].low;
    for i in 1..candles.len() {
        let hl = candles[i].high - candles[i].low;
        let hc = (candles[i].high - candles[i - 1].close).abs();
        let lc = (candles[i].low - candles[i - 1].close).abs();
        out[i] = hl.max(hc).max(lc);
    }
    out
}

pub fn atr(candles: &[Candle], len: usize) -> Vec<Option<f64>> {
    let tr = true_range(candles);
    rma(&tr, len)
}

pub fn adx(candles: &[Candle], len: usize) -> OptionSeriesTriple {
    let n = candles.len();
    if n == 0 {
        return (vec![], vec![], vec![]);
    }

    let mut plus_dm = vec![0.0; n];
    let mut minus_dm = vec![0.0; n];
    for i in 1..n {
        let up_move = candles[i].high - candles[i - 1].high;
        let down_move = candles[i - 1].low - candles[i].low;
        plus_dm[i] = if up_move > down_move && up_move > 0.0 {
            up_move
        } else {
            0.0
        };
        minus_dm[i] = if down_move > up_move && down_move > 0.0 {
            down_move
        } else {
            0.0
        };
    }

    let tr = true_range(candles);
    let tr_rma = rma(&tr, len);
    let plus_rma = rma(&plus_dm, len);
    let minus_rma = rma(&minus_dm, len);

    let mut plus_di = vec![None; n];
    let mut minus_di = vec![None; n];
    let mut dx_raw = vec![0.0; n];
    for i in 0..n {
        if let (Some(trv), Some(p), Some(m)) = (tr_rma[i], plus_rma[i], minus_rma[i]) {
            if trv > 0.0 {
                let pdi = 100.0 * p / trv;
                let mdi = 100.0 * m / trv;
                plus_di[i] = Some(pdi);
                minus_di[i] = Some(mdi);
                let denom = pdi + mdi;
                if denom > 0.0 {
                    dx_raw[i] = ((pdi - mdi).abs() / denom) * 100.0;
                }
            }
        }
    }
    let adx_line = rma(&dx_raw, len);
    (adx_line, plus_di, minus_di)
}

pub fn stoch_rsi(rsi_values: &[Option<f64>], len: usize, smooth: usize) -> Vec<Option<f64>> {
    let n = rsi_values.len();
    let mut out = vec![None; n];
    if len == 0 || n < len {
        return out;
    }

    for i in (len - 1)..n {
        let mut min_rsi = f64::INFINITY;
        let mut max_rsi = f64::NEG_INFINITY;
        for rv in rsi_values.iter().take(i + 1).skip(i + 1 - len).flatten() {
            min_rsi = min_rsi.min(*rv);
            max_rsi = max_rsi.max(*rv);
        }
        if !min_rsi.is_finite() || !max_rsi.is_finite() || (max_rsi - min_rsi).abs() < 1e-12 {
            continue;
        }
        if let Some(cur) = rsi_values[i] {
            out[i] = Some((cur - min_rsi) / (max_rsi - min_rsi));
        }
    }

    if smooth > 1 {
        let raw: Vec<f64> = out.iter().map(|v| v.unwrap_or(0.0)).collect();
        return sma(&raw, smooth)
            .into_iter()
            .map(|v| v.map(|x| x.clamp(0.0, 1.0)))
            .collect();
    }

    out.into_iter()
        .map(|v| v.map(|x| x.clamp(0.0, 1.0)))
        .collect()
}

pub fn cumulative_vwap(candles: &[Candle]) -> Vec<Option<f64>> {
    let mut out = vec![None; candles.len()];
    let mut pv = 0.0;
    let mut vv = 0.0;
    for (i, c) in candles.iter().enumerate() {
        let typ = (c.high + c.low + c.close) / 3.0;
        pv += typ * c.volume;
        vv += c.volume;
        if vv > 0.0 {
            out[i] = Some(pv / vv);
        }
    }
    out
}

pub fn highest(values: &[f64], len: usize) -> Vec<Option<f64>> {
    if len == 0 {
        return vec![None; values.len()];
    }
    let mut out = vec![None; values.len()];
    if values.len() < len {
        return out;
    }
    for i in (len - 1)..values.len() {
        out[i] = Some(
            values[i + 1 - len..=i]
                .iter()
                .fold(f64::NEG_INFINITY, |a, b| a.max(*b)),
        );
    }
    out
}

pub fn lowest(values: &[f64], len: usize) -> Vec<Option<f64>> {
    if len == 0 {
        return vec![None; values.len()];
    }
    let mut out = vec![None; values.len()];
    if values.len() < len {
        return out;
    }
    for i in (len - 1)..values.len() {
        out[i] = Some(
            values[i + 1 - len..=i]
                .iter()
                .fold(f64::INFINITY, |a, b| a.min(*b)),
        );
    }
    out
}

#[derive(Debug, Clone)]
pub struct IchimokuSeries {
    pub conversion: Vec<Option<f64>>,
    pub base: Vec<Option<f64>>,
    pub span_a: Vec<Option<f64>>,
    pub span_b: Vec<Option<f64>>,
}

pub fn ichimoku(candles: &[Candle], conv: usize, base: usize, span_b_len: usize) -> IchimokuSeries {
    let highs_v = highs(candles);
    let lows_v = lows(candles);

    let conv_h = highest(&highs_v, conv.max(1));
    let conv_l = lowest(&lows_v, conv.max(1));
    let base_h = highest(&highs_v, base.max(1));
    let base_l = lowest(&lows_v, base.max(1));
    let span_h = highest(&highs_v, span_b_len.max(1));
    let span_l = lowest(&lows_v, span_b_len.max(1));

    let mut conversion = vec![None; candles.len()];
    let mut base_line = vec![None; candles.len()];
    let mut span_a = vec![None; candles.len()];
    let mut span_b = vec![None; candles.len()];

    for i in 0..candles.len() {
        if let (Some(h), Some(l)) = (conv_h[i], conv_l[i]) {
            conversion[i] = Some((h + l) / 2.0);
        }
        if let (Some(h), Some(l)) = (base_h[i], base_l[i]) {
            base_line[i] = Some((h + l) / 2.0);
        }
        if let (Some(c), Some(b)) = (conversion[i], base_line[i]) {
            span_a[i] = Some((c + b) / 2.0);
        }
        if let (Some(h), Some(l)) = (span_h[i], span_l[i]) {
            span_b[i] = Some((h + l) / 2.0);
        }
    }

    IchimokuSeries {
        conversion,
        base: base_line,
        span_a,
        span_b,
    }
}

pub fn max_drawdown(equity: &[f64]) -> f64 {
    let mut peak = f64::NEG_INFINITY;
    let mut max_dd: f64 = 0.0;
    for v in equity {
        peak = peak.max(*v);
        if peak > 0.0 {
            max_dd = max_dd.max((peak - *v) / peak);
        }
    }
    max_dd
}

#[cfg(test)]
mod tests {
    use super::macd;

    #[test]
    fn macd_preserves_warmup_gaps_in_signal_and_histogram() {
        let values: Vec<f64> = (1..=10).map(|v| v as f64).collect();
        let (line, signal, hist) = macd(&values, 3, 5, 3);

        assert!(line[..4].iter().all(Option::is_none));
        assert!(signal[..6].iter().all(Option::is_none));
        assert!(hist[..6].iter().all(Option::is_none));
        assert!(line[4].is_some());
        assert!(signal[6].is_some());
        assert!(hist[6].is_some());
    }

    #[test]
    fn macd_signal_seed_uses_valid_line_history_instead_of_zero_fill() {
        let values: Vec<f64> = (1..=10).map(|v| v as f64).collect();
        let (line, signal, hist) = macd(&values, 3, 5, 3);

        let expected_seed = [line[4], line[5], line[6]]
            .into_iter()
            .map(|value| value.expect("valid macd line"))
            .sum::<f64>()
            / 3.0;
        let actual_signal = signal[6].expect("seeded signal");
        let actual_hist = hist[6].expect("aligned histogram");

        assert!((actual_signal - expected_seed).abs() < 1e-12);
        assert!((actual_hist - (line[6].expect("macd line") - actual_signal)).abs() < 1e-12);
    }
}
