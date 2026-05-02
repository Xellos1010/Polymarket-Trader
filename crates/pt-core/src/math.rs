pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

pub fn round_down(price: f64, tick: f64) -> f64 {
    if tick <= 0.0 {
        return price;
    }
    (price / tick).floor() * tick
}

pub fn round_up(price: f64, tick: f64) -> f64 {
    if tick <= 0.0 {
        return price;
    }
    (price / tick).ceil() * tick
}

pub fn bps_to_decimal(bps: f64) -> f64 {
    bps / 10_000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rounding() {
        assert_eq!(round_down(0.123, 0.01), 0.12);
        assert_eq!(round_up(0.123, 0.01), 0.13);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(2.0, 0.0, 1.0), 1.0);
        assert_eq!(clamp(-1.0, 0.0, 1.0), 0.0);
    }
}
