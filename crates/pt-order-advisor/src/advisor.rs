/// Configuration for spread/velocity thresholds.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub taker_threshold_bps: f64,
    pub velocity_threshold: f64,
    pub skip_threshold_bps: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            taker_threshold_bps: 5.0,
            velocity_threshold: 0.5,
            skip_threshold_bps: 50.0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MarketContext {
    pub spread_bps: f64,
    pub tick_velocity: f64,
    pub signal_direction: i8,  // +1 bullish, -1 bearish, 0 neutral
    pub position_pnl: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlacementMode { Maker, Taker, Skip }

pub struct MakerTakerAdvisor { pub cfg: Config }

impl MakerTakerAdvisor {
    pub fn new(cfg: Config) -> Self { Self { cfg } }

    pub fn advise(&self, ctx: &MarketContext) -> PlacementMode {
        if ctx.spread_bps >= self.cfg.skip_threshold_bps {
            return PlacementMode::Skip;
        }
        if ctx.spread_bps <= self.cfg.taker_threshold_bps
            && ctx.tick_velocity >= self.cfg.velocity_threshold
        {
            return PlacementMode::Taker;
        }
        PlacementMode::Maker
    }
}

pub struct ContextualAdvisor { inner: MakerTakerAdvisor }

impl ContextualAdvisor {
    pub fn new(cfg: Config) -> Self { Self { inner: MakerTakerAdvisor::new(cfg) } }

    pub fn advise(&self, ctx: &MarketContext, has_position: bool) -> PlacementMode {
        if !has_position {
            if ctx.spread_bps >= self.inner.cfg.skip_threshold_bps {
                return PlacementMode::Skip;
            }
            return PlacementMode::Maker;
        }
        self.inner.advise(ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(spread_bps: f64, velocity: f64) -> MarketContext {
        MarketContext { spread_bps, tick_velocity: velocity, ..Default::default() }
    }

    #[test]
    fn default_is_maker() {
        let a = MakerTakerAdvisor::new(Config::default());
        assert_eq!(a.advise(&ctx(10.0, 0.0)), PlacementMode::Maker);
    }

    #[test]
    fn wide_spread_skips() {
        let a = MakerTakerAdvisor::new(Config::default());
        assert_eq!(a.advise(&ctx(60.0, 1.0)), PlacementMode::Skip);
    }

    #[test]
    fn tight_spread_high_velocity_is_taker() {
        let a = MakerTakerAdvisor::new(Config::default());
        assert_eq!(a.advise(&ctx(3.0, 0.8)), PlacementMode::Taker);
    }

    #[test]
    fn tight_spread_low_velocity_is_maker() {
        let a = MakerTakerAdvisor::new(Config::default());
        assert_eq!(a.advise(&ctx(3.0, 0.2)), PlacementMode::Maker);
    }

    #[test]
    fn contextual_no_position_always_maker() {
        let a = ContextualAdvisor::new(Config::default());
        assert_eq!(a.advise(&ctx(3.0, 2.0), false), PlacementMode::Maker);
    }

    #[test]
    fn contextual_no_position_wide_spread_skips() {
        let a = ContextualAdvisor::new(Config::default());
        assert_eq!(a.advise(&ctx(60.0, 0.0), false), PlacementMode::Skip);
    }

    #[test]
    fn contextual_with_position_delegates_to_inner() {
        let a = ContextualAdvisor::new(Config::default());
        assert_eq!(a.advise(&ctx(3.0, 0.8), true), PlacementMode::Taker);
    }

    #[test]
    fn skip_threshold_exact_boundary() {
        let a = MakerTakerAdvisor::new(Config::default());
        assert_eq!(a.advise(&ctx(50.0, 0.0)), PlacementMode::Skip);
    }
}
